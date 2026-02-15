use leptos::prelude::*;

#[component]
pub fn GameOptions(
    no_repeat: ReadSignal<bool>,
    set_no_repeat: WriteSignal<bool>,
    hidden_mode: ReadSignal<bool>,
    set_hidden_mode: WriteSignal<bool>,
    on_no_repeat_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <div class="game-options">
            <button
                class="option-btn"
                class:active=move || no_repeat.get()
                on:click=move |_| {
                    let new_val = !no_repeat.get();
                    set_no_repeat.set(new_val);
                    on_no_repeat_change.run(new_val);
                }
            >
                "不允許重複"
            </button>
            <button
                class="option-btn"
                class:active=move || hidden_mode.get()
                on:click=move |_| {
                    set_hidden_mode.set(!hidden_mode.get());
                }
            >
                "隱藏模式"
            </button>
        </div>
    }
}
