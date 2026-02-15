use crate::range_config::LimitMode;
use leptos::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

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

/// 一排（row 或 column）的 transform 配對
/// row 使用 rotateX（水平軸翻轉），column 使用 rotateY（垂直軸翻轉）
const ROW_TRANSFORMS: [&str; 4] = ["", "rotateX(180deg)", "rotateX(360deg)", "rotateX(-180deg)"];
const COL_TRANSFORMS: [&str; 4] = ["", "rotateY(180deg)", "rotateY(360deg)", "rotateY(-180deg)"];

/// 每個 cell 的 transform 狀態：(cell transform, text counter-transform)
type CellTransforms = [(String, String); 9];

fn default_cell_transforms() -> CellTransforms {
    std::array::from_fn(|_| (String::new(), String::new()))
}

/// 隨機選一個 row 或 column，對該排的 cell 套用 transform
fn random_line_transform(current: &CellTransforms) -> CellTransforms {
    let mut rng = thread_rng();
    let mut result = current.clone();

    // 隨機選 row(0-2) 或 col(0-2)
    let is_row = rng.gen_bool(0.5);
    let line_idx = rng.gen_range(0..3);

    // 取得該排的 cell index
    let indices: [usize; 3] = if is_row {
        [line_idx * 3, line_idx * 3 + 1, line_idx * 3 + 2]
    } else {
        [line_idx, line_idx + 3, line_idx + 6]
    };

    let transforms = if is_row {
        &ROW_TRANSFORMS
    } else {
        &COL_TRANSFORMS
    };

    // 選一個與目前不同的 transform
    let current_tf = &current[indices[0]].0;
    let candidates: Vec<_> = transforms.iter().filter(|tf| *tf != current_tf).collect();
    let &chosen = candidates.choose(&mut rng).unwrap();

    // counter-transform：反向旋轉讓文字保持可讀
    let counter = if chosen.is_empty() {
        String::new()
    } else if chosen.contains("rotateX") {
        chosen.replace("rotateX(", "rotateX(-").replace("(-(-", "(")
    } else {
        chosen.replace("rotateY(", "rotateY(-").replace("(-(-", "(")
    };

    for &i in &indices {
        result[i] = (chosen.to_string(), counter.clone());
    }

    result
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
    let (cell_transforms, set_cell_transforms) = signal(default_cell_transforms());

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
        let (ref current, _) = grid_transform.get();
        set_grid_transform.set(random_transform(current));
    };

    let on_spin_line = move |_| {
        let current = cell_transforms.get();
        set_cell_transforms.set(random_line_transform(&current));
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

    /// 組合 transform 字串，加上 transition
    fn style_with_transform(transforms: &[&str]) -> String {
        let combined: Vec<_> = transforms
            .iter()
            .filter(|s| !s.is_empty())
            .copied()
            .collect();
        if combined.is_empty() {
            "transition: transform 0.6s ease".to_string()
        } else {
            format!(
                "transform: {}; transition: transform 0.6s ease",
                combined.join(" ")
            )
        }
    }

    view! {
        <div class="number-pad">
            <div class="number-pad-grid" style=pad_style>
                {move || {
                    let selected = selected_numbers.get();
                    let finished = game_finished.get();
                    let needed = digits_needed.get();
                    let current_mode = mode.get();
                    let amt_limit = max_amount.get();
                    let no_rep = no_repeat.get();
                    let (_, ref grid_counter) = grid_transform.get();
                    let ct = cell_transforms.get();
                    grid_numbers
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, num)| {
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
                            let (ref cell_tf, ref cell_counter) = ct[idx];
                            let btn_style = style_with_transform(&[grid_counter, cell_tf]);
                            let text_style = style_with_transform(&[cell_counter]);
                            view! {
                                <button
                                    class="pad-cell"
                                    class:selected=is_selected
                                    class:disabled=is_disabled
                                    class:exceeds=exceeds_limit
                                    style=btn_style
                                    on:click=move |_| handle_click(num)
                                >
                                    <span class="pad-cell-text" style=text_style>
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
                <button class="pad-action-btn" on:click=on_spin_line>
                    "轉動一排"
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
