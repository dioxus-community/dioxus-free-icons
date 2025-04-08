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
    create_icon_file::create_icon_file(IO_SVG_BASE_PATH, &output_path, "Io");

    // create octicons
    const GO_SVG_BASE_PATH: &str = "../../icon_resources/octicons/icons";
    let go_output_path = format!("{}/go_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(GO_SVG_BASE_PATH, &go_output_path, "Go");

    // create bootstrap icons
    const BS_SVG_BASE_PATH: &str = "../../icon_resources/bootstrap/icons";
    let bs_output_path = format!("{}/bs_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(BS_SVG_BASE_PATH, &bs_output_path, "Bs");

    // create feather icons
    const FI_SVG_BASE_PATH: &str = "../../icon_resources/feather/icons";
    let fi_output_path = format!("{}/fi_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(FI_SVG_BASE_PATH, &fi_output_path, "Fi");

    // create feather icons
    const LD_SVG_BASE_PATH: &str = "../../icon_resources/lucide/icons";
    let ld_output_path = format!("{}/ld_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(LD_SVG_BASE_PATH, &ld_output_path, "Ld");

    // create material design icons
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

    const SI_SVG_BASE_PATH: &str = "../../icon_resources/simple-icons/icons";
    let ld_output_path = format!("{}/si_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(SI_SVG_BASE_PATH, &ld_output_path, "Si");
}
