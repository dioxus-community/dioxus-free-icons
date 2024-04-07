mod create_icon_file;

fn main() {
    const OUTPUT_BASE_PATH: &str = "../lib/src/icons";

    // create flag icons
    println!("Processing flag Square icons");
    const FG_SQUARE_SVG_BASE_PATH: &str = "../../icon_resources/flag-icons/flags/1x1";
    let output_path = format!("{}/fg_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(FG_SQUARE_SVG_BASE_PATH, &output_path, "Fg");

    println!("Processing flag Rect icons");
    const FG_RECT_SVG_BASE_PATH: &str = "../../icon_resources/flag-icons/flags/4x3";
    let output_path = format!("{}/fg_rect_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(FG_RECT_SVG_BASE_PATH, &output_path, "FgR");

    // create font awesome icons
    println!("Processing font awesome icons");
    const FA_SVG_BASE_PATH: &str = "../../icon_resources/font-awesome/svgs";
    for icon_type in vec!["brands", "regular", "solid"].into_iter() {
        let svg_path = format!("{}/{}", FA_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/fa_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_icon_file::create_icon_file(&svg_path, &output_path, "Fa");
    }

    // create hero icons
    println!("Processing hero icons");
    const HI_SVG_BASE_PATH: &str = "../../icon_resources/heroicons/src";
    for icon_type in vec!["outline", "solid"].into_iter() {
        let svg_path = format!("{}/{}", HI_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/hi_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_icon_file::create_icon_file(&svg_path, &output_path, "Hi");
    }

    // create ionicons
    println!("Processing ionicons");
    const IO_SVG_BASE_PATH: &str = "../../icon_resources/ionicons/src/svg";
    let output_path = format!("{}/io_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(IO_SVG_BASE_PATH, &output_path, "Io");

    // create octicons
    println!("Processing octicons");
    const GO_SVG_BASE_PATH: &str = "../../icon_resources/octicons/icons";
    let go_output_path = format!("{}/go_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(GO_SVG_BASE_PATH, &go_output_path, "Go");

    // create bootstrap icons
    println!("Processing bootstrap icons");
    const BS_SVG_BASE_PATH: &str = "../../icon_resources/bootstrap/icons";
    let bs_output_path = format!("{}/bs_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(BS_SVG_BASE_PATH, &bs_output_path, "Bs");

    // create feather icons
    println!("Processing feather icons");
    const FI_SVG_BASE_PATH: &str = "../../icon_resources/feather/icons";
    let fi_output_path = format!("{}/fi_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(FI_SVG_BASE_PATH, &fi_output_path, "Fi");

    // create material design icons
    println!("Processing material design icons");
    const MI_SVG_BASE_PATH: &str = "../../icon_resources/material-design-icons/src";
    for icon_type in vec![
        "action",
        "alert",
        "av",
        "communication",
        "content",
        "device",
        "editor",
        "file",
        "hardware",
        "home",
        "image",
        "maps",
        "navigation",
        "notification",
        "places",
        "social",
        "toggle",
    ]
    .into_iter()
    {
        let svg_path = format!("{}/{}", MI_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/md_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_icon_file::create_icon_file(&svg_path, &output_path, "Md");
    }
}
