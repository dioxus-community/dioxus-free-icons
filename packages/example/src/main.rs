use dioxus::prelude::*;

use dioxus_free_icons::icons::fi_icons::FiDelete;
use dioxus_free_icons::icons::io_icons::IoAdd;
use dioxus_free_icons::Icon;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    launch(app);
}

fn app() -> Element {
    rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." },
            button {
                Icon { icon: IoAdd }
                "IoAdd"
              }
             button {
                Icon { icon: FiDelete }
               "Remove Project"
             }
        }
    )
}
