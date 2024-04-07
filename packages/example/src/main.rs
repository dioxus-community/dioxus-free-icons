#![allow(non_snake_case)]
mod use_clipboard;
use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::Icon;
use use_clipboard::copy_to_clipboard;
const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

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

pub fn app() -> Element {
    rsx!(Router::<Route> {})
}

#[derive(Clone)]
pub struct SearchIcon(String);

fn MainWrapper() -> Element {
    let grey_background = true;
    let mut search_icon = use_context_provider(|| Signal::new(SearchIcon(String::new())));
    let _iconsets = use_context_provider(|| Signal::new(build_iconsets()));
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
                                search_icon.set(SearchIcon(e.value()));
                            }
                        }
                    }
                    li {
                        Link {
                            class: "mr-5 hover:text-white",
                            to: Route::Template {
                                code: "ant".to_string(),
                            },
                            "First Link"
                        }
                    }
                    li { a { class: "mr-5 hover:text-white", "Second Link" } }
                    li { a { class: "mr-5 hover:text-white", "Third Link" } }
                    li { a { class: "mr-5 hover:text-white", "Fourth Link" } }
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
    rsx! {
        div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
            h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium text-white",
                br { class: "hidden lg:inline-block" }
                "Dioxus Sneak Peek"
            }
            p { class: "mb-8 leading-relaxed",

                "Dioxus is a new UI framework that makes it easy and simple to write cross-platform apps using web
        technologies! It is functional, fast, and portable. Dioxus can run on the web, on the desktop, and
        on mobile and embedded platforms."
            }
            div { class: "flex justify-center",
                button { class: "inline-flex text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                    "Learn more"
                }
                button { class: "ml-4 inline-flex text-gray-400 bg-gray-800 border-0 py-2 px-6 focus:outline-none hover:bg-gray-700 hover:text-white rounded text-lg",
                    "Build an app"
                }
            }
        }
        div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
            img {
                class: "object-cover object-center rounded",
                src: "https://i.imgur.com/oK6BLtw.png",
                referrerpolicy: "no-referrer",
                alt: "hero"
            }
        }
    }
}

fn build_iconsets() -> Vec<IconSet> {
    vec![IconSet {
        code: "ant".to_string(),
        name: "Ant Design Icons".to_string(),
        license: "MIT".to_string(),
        url: "".to_string(),
        icons: vec![IconName {
            name: "rust".to_string(),
            icon: rsx!(Icon { icon: FaRust }),
        }],
    }]
}

#[derive(Clone, Debug, Default)]
pub struct IconSet {
    pub code: String,
    pub name: String,
    pub license: String,
    pub url: String,
    pub icons: Vec<IconName>,
}

#[derive(Clone, Debug, Default)]
pub struct IconName {
    pub name: String,
    pub icon: Element,
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
            .filter(|icon| icon.name.contains(&search_icon().0))
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
        }
        h2 { class: "text-2xl font-bold text-gray-900", "Setup" }
        h2 { class: "text-2xl font-bold text-gray-900", "Icons" }
        div { class: "flex flex-wrap",
            for icon in icons {
                div {
                    class: "w-1/8 p-2 items-center justify-center text-center",
                    onclick: move |_e| {
                        let icon_name = icon.name.clone();
                        async move {
                            copy_to_clipboard(&format!(" {{ Icon {{ icon: {} }} }}", icon_name))
                                .await
                                .unwrap()
                        };
                    },
                    {icon.icon},
                    div { {icon.name.clone()} }
                }
            }
        }
    }
}
