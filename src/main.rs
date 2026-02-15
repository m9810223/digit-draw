mod amount_display;
mod app;
mod game_options;
mod number_pad;
mod range_config;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
