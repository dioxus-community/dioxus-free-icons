use std::error::Error;
use std::fs;
use std::path;
use std::path::PathBuf;

use codegen::Scope;
use heck::ToUpperCamelCase;
use scraper::Html;
use scraper::Selector;

pub fn read_dir(path: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<path::PathBuf> = Vec::new();
    for item in dir.into_iter() {
        files.push(item?.path());
    }
    Ok(files)
}

#[allow(dead_code)]
#[derive(Debug)]
struct Icon {
    name: String,
    view_box: String,
    xmlns: String,
    d: String,
}

fn icon_name(path: &PathBuf) -> String {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let name = filename.split('.').next().unwrap();
    name.to_upper_camel_case()
}

fn main() {
    let target_path = "./svgs";
    let files = read_dir(target_path).unwrap();
    let mut icons = Vec::new();
    let svg_selector = Selector::parse("svg").unwrap();
    let path_selector = Selector::parse("path").unwrap();
    for file in files {
        let svg_str = fs::read_to_string(&file).unwrap();
        let fragment = Html::parse_fragment(&svg_str);
        let svg_data = fragment.select(&svg_selector).next().unwrap();
        let path_data = fragment.select(&path_selector).next().unwrap();

        icons.push(Icon {
            name: icon_name(&file),
            view_box: svg_data.value().attr("viewBox").unwrap().to_string(),
            xmlns: svg_data
                .value()
                .attr("xmlns")
                .unwrap_or("http://www.w3.org/2000/svg")
                .to_string(),
            d: path_data.value().attr("d").unwrap().to_string(),
        })
    }
    println!("{:?}", icons);

    let mut scope = Scope::new();
    scope
        .new_module("test")
        .new_struct("Icon")
        .field("new", "String");
    // let mut icon_enum = scope.new_enum("Icon");
    // for name in icons.iter().map(|i| i.name.clone()) {
    //     icon_enum.new_variant(&name);
    // }

    println!("{:?}", scope.to_string());
}
