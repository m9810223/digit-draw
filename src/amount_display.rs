use leptos::prelude::*;

#[component]
pub fn AmountDisplay(
    selected_numbers: ReadSignal<Vec<u8>>,
    digits_needed: ReadSignal<u8>,
    hidden_mode: ReadSignal<bool>,
    game_finished: ReadSignal<bool>,
    set_game_finished: WriteSignal<bool>,
    on_reset: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="amount-display">
            <div class="amount-digits">
                {move || {
                    let needed = digits_needed.get() as usize;
                    let nums = selected_numbers.get();
                    let hidden = hidden_mode.get();
                    let finished = game_finished.get();
                    (0..needed)
                        .rev()
                        .map(|i| {
                            let digit = nums.get(i).copied();
                            let filled = digit.is_some();
                            let text = if hidden && !finished {
                                "$".to_string()
                            } else {
                                digit.map(|d| d.to_string()).unwrap_or_else(|| "$".to_string())
                            };
                            view! {
                                <span
                                    class="amount-digit"
                                    class:filled=filled
                                    class:hidden=hidden && !finished
                                    class:revealed=finished
                                >
                                    {text}
                                </span>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>

            <div class="amount-actions">
                {move || {
                    let hidden = hidden_mode.get();
                    let finished = game_finished.get();
                    let nums = selected_numbers.get();
                    let needed = digits_needed.get();
                    let all_selected = nums.len() as u8 == needed;
                    if finished || (!hidden && all_selected) {
                        view! {
                            <button class="reset-btn" on:click=move |_| on_reset.run(())>
                                "再玩一次"
                            </button>
                        }
                            .into_any()
                    } else if hidden && all_selected {
                        view! {
                            <button
                                class="confirm-btn"
                                on:click=move |_| set_game_finished.set(true)
                            >
                                "確認開獎！"
                            </button>
                        }
                            .into_any()
                    } else {
                        view! { <span class="amount-actions-placeholder"></span> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
