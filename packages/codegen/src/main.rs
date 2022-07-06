mod create_icon_file;

fn main() {
    const OUTPUT_BASE_PATH: &str = "../lib/src/icons";

    // create font awesome icons
    const FA_SVG_BASE_PATH: &str = "../../icon_resources/font-awesome/svgs";
    for icon_type in vec!["brands", "regular", "solid"].into_iter() {
        let svg_path = format!("{}/{}", FA_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/fa_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_icon_file::create_icon_file(&svg_path, &output_path, "Fa");
    }

    // create hero icons
    const HI_SVG_BASE_PATH: &str = "../../icon_resources/heroicons/src";
    for icon_type in vec!["outline", "solid"].into_iter() {
        let svg_path = format!("{}/{}", HI_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/hi_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_icon_file::create_icon_file(&svg_path, &output_path, "Hi");
    }

    // create ionicons
    const IO_SVG_BASE_PATH: &str = "../../icon_resources/ionicons/src/svg";
    let output_path = format!("{}/io_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(&IO_SVG_BASE_PATH, &output_path, "Io");
}
