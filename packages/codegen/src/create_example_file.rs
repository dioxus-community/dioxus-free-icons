use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};

use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use regex::Regex;
use scraper::node::Element;
use scraper::ElementRef;
use scraper::Html;
use walkdir::WalkDir;

const HEADER_TEMPLATE: &str = r#"
use crate::{IconSet, IconName};
use dioxus::prelude::*;
use dioxus_free_icons::icons::{ICONSET_CODE}::*;
use dioxus_free_icons::Icon;

"#;

const ICONSET_TEMPLATE: &str = r#"
pub fn get_icon_set() -> IconSet{
    IconSet {
        code: "{CODE}".to_string(),
        name: "{NAME}".to_string(),
        url: "{URL}".to_string(),
        license: "{LICENSE}".to_string(),
        license_url: "{LICENSE_URL}".to_string(),
        version: "{VERSION}".to_string(),
        source_url: "{SOURCE_URL}".to_string(),
        icons: vec![
{ICONS}
        ],
    }
}
"#;

const ICON_TEMPLATE: &str = r#"
            IconName {
                name: "{NAME}".to_string(),
                icon: rsx!(Icon { icon: {ICON_NAME} }),
            }"#;

pub fn create_example_file(
    icon_names: Vec<String>,
    output_path: &str,
    name: &str,
    url: &str,
    license: &str,
    license_url: &str,
    version: &str,
    source: &str,
) {
    let code = output_path.split('/').last().unwrap().replace(".rs", "");

    let icons = icon_names
        .into_iter()
        .map(|icon_name| {
            ICON_TEMPLATE
                .replace("{NAME}", &icon_name)
                .replace("{ICON_NAME}", &icon_name)
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let mut content = HEADER_TEMPLATE.replace("{ICONSET_CODE}", &code);
    content.push_str(
        ICONSET_TEMPLATE
            .replace("{CODE}", &code)
            .replace("{NAME}", name)
            .replace("{URL}", url)
            .replace("{LICENSE}", license)
            .replace("{LICENSE_URL}", license_url)
            .replace("{VERSION}", version)
            .replace("{SOURCE_URL}", source)
            .replace("{ICONS}", &icons)
            .as_str(),
    );
    // we write the file
    let mut file = File::create(output_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    println!("Created example file: {}", output_path);
}
