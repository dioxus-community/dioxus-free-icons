use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::Icon;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." },
            Icon {
                size: 60,
                icon: FaRust,
            }
        }
    ))
}
