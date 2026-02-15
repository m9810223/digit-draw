use leptos::prelude::*;
use web_sys::UrlSearchParams;

use crate::amount_display::AmountDisplay;
use crate::game_options::GameOptions;
use crate::number_pad::NumberPad;
use crate::range_config::{LimitMode, RangeConfig};

const DEFAULT_DIGITS: u8 = 4;
const DEFAULT_MAX_AMOUNT: u64 = 999;

struct InitConfig {
    mode: LimitMode,
    digits: u8,
    max_amount: u64,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            mode: LimitMode::ByDigits,
            digits: DEFAULT_DIGITS,
            max_amount: DEFAULT_MAX_AMOUNT,
        }
    }
}

fn read_config_from_url() -> InitConfig {
    let params = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .and_then(|s| UrlSearchParams::new_with_str(&s).ok());

    let Some(p) = params else {
        return InitConfig::default();
    };

    if let Some(d) = p.get("digits").and_then(|v| v.parse::<u8>().ok()) {
        InitConfig {
            mode: LimitMode::ByDigits,
            digits: d.clamp(1, 9),
            ..Default::default()
        }
    } else if let Some(m) = p.get("max").and_then(|v| v.parse::<u64>().ok()) {
        InitConfig {
            mode: LimitMode::ByMaxAmount,
            max_amount: m.max(1),
            ..Default::default()
        }
    } else {
        InitConfig::default()
    }
}

fn sync_url_query(mode: LimitMode, digits: u8, max_amount: u64) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(pathname) = window.location().pathname() else {
        return;
    };
    let query = match mode {
        LimitMode::ByDigits => format!("?digits={}", digits),
        LimitMode::ByMaxAmount => format!("?max={}", max_amount),
    };
    let url = format!("{}{}", pathname, query);
    let _ = window
        .history()
        .and_then(|h| h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url)));
}

#[component]
pub fn App() -> impl IntoView {
    let config = read_config_from_url();

    let (mode, set_mode) = signal(config.mode);
    let (selected_digits, set_selected_digits) = signal(config.digits);
    let (max_amount, set_max_amount) = signal(config.max_amount);
    let (no_repeat, set_no_repeat) = signal(false);
    let (hidden_mode, set_hidden_mode) = signal(false);
    let (selected_numbers, set_selected_numbers) = signal(Vec::<u8>::new());
    let (game_finished, set_game_finished) = signal(false);

    let digits_needed = Memo::new(move |_| match mode.get() {
        LimitMode::ByDigits => selected_digits.get(),
        LimitMode::ByMaxAmount => {
            let amt = max_amount.get();
            if amt == 0 {
                1
            } else {
                (amt as f64).log10().floor() as u8 + 1
            }
        }
    });

    // 模式或參數變更時清空選取並同步 URL
    Effect::new(move |_| {
        let m = mode.get();
        let d = selected_digits.get();
        let a = max_amount.get();
        let _ = digits_needed.get();
        set_selected_numbers.set(Vec::new());
        set_game_finished.set(false);
        sync_url_query(m, d, a);
    });

    let on_reset = Callback::new(move |_: ()| {
        set_selected_numbers.set(Vec::new());
        set_game_finished.set(false);
    });

    let on_no_repeat_change = Callback::new(move |_: bool| {
        set_selected_numbers.set(Vec::new());
        set_game_finished.set(false);
    });

    let (digits_needed_read, set_digits_needed_read) = signal(config.digits);
    Effect::new(move |_| {
        set_digits_needed_read.set(digits_needed.get());
    });

    view! {
        <main class="container">
            <AmountDisplay
                selected_numbers=selected_numbers
                digits_needed=digits_needed_read
                hidden_mode=hidden_mode
                game_finished=game_finished
                set_game_finished=set_game_finished
                on_reset=on_reset
            />

            <NumberPad
                mode=mode
                digits_needed=digits_needed_read
                max_amount=max_amount
                selected_numbers=selected_numbers
                set_selected_numbers=set_selected_numbers
                hidden_mode=hidden_mode
                no_repeat=no_repeat
                game_finished=game_finished
            />

            <GameOptions
                no_repeat=no_repeat
                set_no_repeat=set_no_repeat
                hidden_mode=hidden_mode
                set_hidden_mode=set_hidden_mode
                on_no_repeat_change=on_no_repeat_change
            />

            <RangeConfig
                mode=mode
                set_mode=set_mode
                selected_digits=selected_digits
                set_selected_digits=set_selected_digits
                max_amount=max_amount
                set_max_amount=set_max_amount
            />
        </main>
    }
}
