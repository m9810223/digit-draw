use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum LimitMode {
    ByDigits,
    ByMaxAmount,
}

#[component]
pub fn RangeConfig(
    mode: ReadSignal<LimitMode>,
    set_mode: WriteSignal<LimitMode>,
    selected_digits: ReadSignal<u8>,
    set_selected_digits: WriteSignal<u8>,
    max_amount: ReadSignal<u64>,
    set_max_amount: WriteSignal<u64>,
) -> impl IntoView {
    view! {
        <div class="range-config">
            <h2>"設定範圍"</h2>

            <div class="mode-toggle">
                <button
                    class:active=move || mode.get() == LimitMode::ByDigits
                    on:click=move |_| set_mode.set(LimitMode::ByDigits)
                >
                    "選擇位數"
                </button>
                <button
                    class:active=move || mode.get() == LimitMode::ByMaxAmount
                    on:click=move |_| set_mode.set(LimitMode::ByMaxAmount)
                >
                    "選擇最大金額"
                </button>
            </div>

            {move || {
                if mode.get() == LimitMode::ByDigits {
                    {
                        let on_dec = move |_| {
                            set_selected_digits.update(|n| *n = (*n).saturating_sub(1).max(1));
                        };
                        let on_inc = move |_| {
                            set_selected_digits.update(|n| *n = (*n + 1).min(9));
                        };
                        let dec_disabled = move || selected_digits.get() <= 1;
                        let inc_disabled = move || selected_digits.get() >= 9;
                        view! {
                            <div class="digit-stepper">
                                <button class="stepper-btn" on:click=on_dec disabled=dec_disabled>
                                    "-"
                                </button>
                                <span class="stepper-value">{move || selected_digits.get()}</span>
                                <button class="stepper-btn" on:click=on_inc disabled=inc_disabled>
                                    "+"
                                </button>
                            </div>
                        }
                    }
                        .into_any()
                } else {
                    view! {
                        <div class="max-amount-input">
                            <label>"最大金額: "</label>
                            <input
                                type="number"
                                min="1"
                                max="987654321"
                                prop:value=move || max_amount.get().to_string()
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<u64>().unwrap_or(0);
                                    set_max_amount.set(val);
                                }
                            />
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
