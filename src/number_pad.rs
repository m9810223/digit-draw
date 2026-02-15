use crate::range_config::LimitMode;
use leptos::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

/// 計算已選數字組成的金額（由個位數開始）
fn current_amount(selected: &[u8]) -> u64 {
    selected
        .iter()
        .enumerate()
        .map(|(i, &n)| n as u64 * 10u64.pow(i as u32))
        .sum()
}

/// 如果再選 `candidate` 放在下一個位數，金額是否會超過上限
fn would_exceed(selected: &[u8], candidate: u8, max_amount: u64) -> bool {
    let next_pos = selected.len() as u32;
    let new_amount = current_amount(selected) + candidate as u64 * 10u64.pow(next_pos);
    new_amount > max_amount
}

/// (grid transform, cell counter-transform)
/// 使用 rotateY/rotateX 取代 scaleX(-1)/scaleY(-1)，讓 transition 產生 3D 翻轉動畫
const TRANSFORM_PAIRS: [(&str, &str); 8] = [
    ("", ""),
    ("rotate(90deg)", "rotate(-90deg)"),
    ("rotate(180deg)", "rotate(-180deg)"),
    ("rotate(270deg)", "rotate(-270deg)"),
    ("rotateY(180deg)", "rotateY(-180deg)"),
    ("rotateX(180deg)", "rotateX(-180deg)"),
    (
        "rotate(90deg) rotateY(180deg)",
        "rotateY(-180deg) rotate(-90deg)",
    ),
    (
        "rotate(90deg) rotateX(180deg)",
        "rotateX(-180deg) rotate(-90deg)",
    ),
];

/// 隨機選取一個與 `current` 不同的 transform 配對
fn random_transform(current: &str) -> (String, String) {
    let candidates: Vec<_> = TRANSFORM_PAIRS
        .iter()
        .filter(|(tf, _)| *tf != current)
        .collect();
    let pair = candidates.choose(&mut thread_rng()).unwrap();
    (pair.0.to_string(), pair.1.to_string())
}

/// 轉動方向
#[derive(Clone, Copy)]
enum RotateDir {
    /// 所有 row 向右循環
    RowRight,
    /// 所有 column 向下循環
    ColDown,
}

/// 隨機選擇方向
fn random_rotate_dir() -> RotateDir {
    if thread_rng().gen_bool(0.5) {
        RotateDir::RowRight
    } else {
        RotateDir::ColDown
    }
}

/// 依方向將所有 row/column 做循環位移
fn rotate_lines(nums: &mut [u8], dir: RotateDir) {
    let lines: [[usize; 3]; 3] = match dir {
        RotateDir::RowRight => [[0, 1, 2], [3, 4, 5], [6, 7, 8]],
        RotateDir::ColDown => [[0, 3, 6], [1, 4, 7], [2, 5, 8]],
    };

    for indices in &lines {
        // [a, b, c] -> [c, a, b]
        let last = nums[indices[2]];
        nums[indices[2]] = nums[indices[1]];
        nums[indices[1]] = nums[indices[0]];
        nums[indices[0]] = last;
    }
}

#[component]
pub fn NumberPad(
    mode: ReadSignal<LimitMode>,
    digits_needed: ReadSignal<u8>,
    max_amount: ReadSignal<u64>,
    selected_numbers: ReadSignal<Vec<u8>>,
    set_selected_numbers: WriteSignal<Vec<u8>>,
    hidden_mode: ReadSignal<bool>,
    no_repeat: ReadSignal<bool>,
    game_finished: ReadSignal<bool>,
) -> impl IntoView {
    let (grid_numbers, set_grid_numbers) = signal(vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9]);
    let (pad_size, set_pad_size) = signal(100u32);
    let (grid_transform, set_grid_transform) = signal((String::new(), String::new()));
    // 轉動一排動畫：CSS class 名稱（"rotate-row" 或 "rotate-col"）
    let (rotate_anim, set_rotate_anim) = signal(String::new());
    let (animating, set_animating) = signal(false);

    let on_shuffle = move |_| {
        let mut nums = grid_numbers.get();
        let original = nums.clone();
        loop {
            nums.shuffle(&mut thread_rng());
            if nums != original {
                break;
            }
        }
        set_grid_numbers.set(nums);
    };

    let handle_click = move |num: u8| {
        if game_finished.get() {
            return;
        }
        let mut current = selected_numbers.get();
        let no_rep = no_repeat.get();

        if no_rep && current.contains(&num) {
            current.retain(|&n| n != num);
        } else if (current.len() as u8) < digits_needed.get() {
            if mode.get() == LimitMode::ByMaxAmount && would_exceed(&current, num, max_amount.get())
            {
                return;
            }
            current.push(num);
        }
        set_selected_numbers.set(current);
    };

    let on_spin = move |_| {
        if animating.get() {
            return;
        }
        let mut rng = thread_rng();
        if rng.gen_bool(0.5) {
            let (ref current, _) = grid_transform.get();
            set_grid_transform.set(random_transform(current));
        } else {
            let dir = random_rotate_dir();
            let anim_class = match dir {
                RotateDir::RowRight => "rotate-row",
                RotateDir::ColDown => "rotate-col",
            };
            // 1) 加上 CSS animation class
            set_rotate_anim.set(anim_class.to_string());
            set_animating.set(true);
            // 2) 動畫結束後：更新數字、移除 class
            let window = web_sys::window().unwrap();
            let cb = Closure::once(move || {
                let mut nums = grid_numbers.get();
                rotate_lines(&mut nums, dir);
                set_grid_numbers.set(nums);
                set_rotate_anim.set(String::new());
                set_animating.set(false);
            });
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    420,
                )
                .unwrap();
            cb.forget();
        }
    };

    let pad_style = move || {
        let size = format!("max-width: {}%", pad_size.get());
        let (ref tf, _) = grid_transform.get();
        if tf.is_empty() {
            format!("{}; transition: transform 0.6s ease", size)
        } else {
            format!(
                "{}; transform: {}; transition: transform 0.6s ease",
                size, tf
            )
        }
    };

    let text_style = move || {
        let (_, ref counter) = grid_transform.get();
        if counter.is_empty() {
            "transition: transform 0.6s ease".to_string()
        } else {
            format!("transform: {}; transition: transform 0.6s ease", counter)
        }
    };

    view! {
        <div class="number-pad">
            <div
                class=move || {
                    let anim = rotate_anim.get();
                    if anim.is_empty() {
                        "number-pad-grid".to_string()
                    } else {
                        format!("number-pad-grid {}", anim)
                    }
                }
                style=pad_style
            >
                {move || {
                    let selected = selected_numbers.get();
                    let finished = game_finished.get();
                    let needed = digits_needed.get();
                    let current_mode = mode.get();
                    let amt_limit = max_amount.get();
                    let no_rep = no_repeat.get();
                    grid_numbers
                        .get()
                        .into_iter()
                        .map(|num| {
                            let is_selected = selected.contains(&num);
                            let at_capacity = selected.len() as u8 >= needed;
                            let exceeds_limit = current_mode == LimitMode::ByMaxAmount
                                && !at_capacity && would_exceed(&selected, num, amt_limit);
                            let is_disabled = if no_rep {
                                finished || (!is_selected && at_capacity)
                                    || (!is_selected && exceeds_limit)
                            } else {
                                finished || at_capacity || exceeds_limit
                            };
                            let hidden = hidden_mode.get() && !finished;
                            let display_text = if hidden {
                                "$".to_string()
                            } else {
                                num.to_string()
                            };
                            let ts = text_style();
                            view! {
                                <button
                                    class="pad-cell"
                                    class:selected=is_selected
                                    class:disabled=is_disabled
                                    class:exceeds=exceeds_limit
                                    on:click=move |_| handle_click(num)
                                >
                                    <span class="pad-cell-text" style=ts>
                                        {display_text}
                                    </span>
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
            <div class="number-pad-actions">
                <button class="pad-action-btn" on:click=on_shuffle>
                    "隨機位置"
                </button>
                <button class="pad-action-btn" on:click=on_spin>
                    "隨機轉動"
                </button>
            </div>

            <div class="number-pad-size">
                <input
                    type="range"
                    min="50"
                    max="100"
                    prop:value=move || pad_size.get().to_string()
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<u32>().unwrap_or(100);
                        set_pad_size.set(val);
                    }
                />
            </div>
        </div>
    }
}
