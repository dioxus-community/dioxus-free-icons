#![allow(non_snake_case)]
mod icons;
mod use_clipboard;
use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::Icon;
use icons::{get_icon_sets, IconSet};
use use_clipboard::copy_to_clipboard;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    // The home page is at the / route
    #[layout(MainWrapper)]
        #[route("/")]
        Index {},
        #[route("/iconset/:code")]
        Template {
            code: String
        },
}

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    launch(app);
}

const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");
// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

pub fn app() -> Element {
    rsx! {
        // Global app resources
        // document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[derive(Clone)]
pub struct SearchIcon(String);

fn MainWrapper() -> Element {
    let grey_background = true;
    let mut search_icon = use_context_provider(|| Signal::new(SearchIcon(String::new())));
    let iconsets = use_context_provider(|| Signal::new(get_icon_sets(48, 48)));
    rsx!(
        div {
            header {
                class: "text-gray-400 body-font",
                // you can use optional attributes to optionally apply a tailwind class
                class: if grey_background { "bg-gray-900" },
                div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                    a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                        Icon { width: 30, height: 30, icon: FaRust }
                        span { class: "ml-3 text-xl", "Dioxus Free Icons" }
                    }
                }
            }
            div { class: "grid grid-cols-4 gap-4",
                ul { class: "text-gray-400 bg-gray-800 px-5 py-5 items-center text-base justify-center",
                    li {
                        input {
                            class: "rounded-full w-full bg-gray-200 rounded border border-gray-700 focus:outline-none focus:border-indigo-500 text-base px-4 py-2",
                            placeholder: "Search Icons",
                            oninput: move |e| {
                                search_icon.set(SearchIcon(e.value().to_ascii_lowercase()));
                            },
                        }
                    }
                    for icon_set in iconsets() {
                        li {
                            Link {
                                class: "mr-5 hover:text-white",
                                to: Route::Template {
                                    code: icon_set.code.clone(),
                                },
                                {icon_set.name.clone()}
                                "("
                                {icon_set.icons.len().to_string()}
                                " icons"
                                ")"
                            }
                        }
                    }
                }

                div { class: "col-span-3 grow text-gray-400 bg-gray-900 body-font lg:flex-grow px-5 py-5 space-y-5",
                    Outlet::<Route> {}
                }
            }
        }
    )
}

#[component]
fn Index() -> Element {
    let search_icon = consume_context::<Signal<SearchIcon>>();
    let icon_sets = consume_context::<Signal<Vec<IconSet>>>();

    rsx! {
        if (search_icon().0.is_empty()) {
            div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
                h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium text-white",
                    br { class: "hidden lg:inline-block" }
                    "Dioxus free icons"
                }
                p { class: "mb-8 leading-relaxed",
                    "Just select an icon set from the left sidebar to navigate the available icon families. Use the search box to filter them."
                }
                p { class: "mb-8 leading-relaxed",
                    "Click on an icon to copy the icon code to use it in your project."
                }
            }
        } else {
            div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
                for icon_set in icon_sets() {
                    h2 { class: "text-2xl font-bold text-gray-900",
                        {icon_set.name.clone()}
                        " - "
                        {icon_set.code.clone()}
                    }
                    div { class: "flex flex-wrap",
                        for icon in icon_set.filter(&search_icon().0).icons {
                            IconPlaceholder { name: icon.0, icon: icon.1 }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Template(code: String) -> Element {
    let mut search_icon = consume_context::<Signal<SearchIcon>>();
    let mut icon_sets = consume_context::<Signal<Vec<IconSet>>>();
    let icon_set = icon_sets()
        .into_iter()
        .find(|iset| iset.code == code)
        .unwrap_or(IconSet::default());

    let mut icons = icon_set.icons.clone();
    if !search_icon().0.is_empty() {
        icons = icons
            .into_iter()
            .filter(|icon| icon.0.to_ascii_lowercase().contains(&search_icon().0))
            .collect();
    }

    rsx! {
        h1 { class: "text-3xl font-bold text-gray-900", {icon_set.name} }
        table { class: "table-auto w-full",
            tr {
                td { "License" }
                td { {icon_set.license} }
            }
            tr {
                td { "Project" }
                td {
                    a { href: icon_set.url.clone(), {icon_set.url.clone()} }
                }
            }
            tr {
                td { "Version" }
                td { {icon_set.version} }
            }
            tr {
                td { "Crate Feature" }
                td { {icon_set.code} }
            }
        }
        h2 { class: "text-2xl font-bold text-gray-900", "Icons" }
        div { class: "flex flex-wrap",
            for icon in icons {
                IconPlaceholder { name: icon.0, icon: icon.1 }
            }
        }
    }
}

#[component]
fn IconPlaceholder(name: String, icon: Element) -> Element {
    rsx! {
        div {
            class: "w-1/8 p-2 items-center justify-center text-center",
            onclick: move |_e| {
                let icon_name = name.clone();
                copy_to_clipboard(&format!(" {{ Icon {{ icon: {} }} }}", icon_name));
            },
            {icon}
            div { {name.clone()} }
        }
    }
}
