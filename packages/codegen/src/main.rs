mod create_fa_icon_file;

fn main() {
    // create font awesome icons
    const FA_SVG_BASE_PATH: &str = "../../font-awesome/svgs";
    const OUTPUT_BASE_PATH: &str = "../lib/src/icons";

    for icon_type in vec!["brands", "regular", "solid"].into_iter() {
        let svg_path = format!("{}/{}", FA_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/fa_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_fa_icon_file::create_fa_icon_file(&svg_path, &output_path);
    }
}
