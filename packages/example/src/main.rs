use dioxus::prelude::*;

use dioxus_free_icons::icons::io_icons::{IoClose, IoFolder};
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
                Icon { icon: IoFolder }
                "IoAdd"
              }
             button {
                Icon { icon: IoClose }
               "Remove Project"
             }
        }
    )
}
