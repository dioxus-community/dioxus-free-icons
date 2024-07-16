mod create_icon_file;

fn main() {
    const OUTPUT_BASE_PATH: &str = "../lib/src/icons";

    #[cfg(feature = "strip-fill")]
    println!("Stripping fill attribute from svg elements");

    #[cfg(feature = "strip-id")]
    println!("Stripping id attribute from svg elements");

    #[cfg(feature = "strip-class")]
    println!("Stripping class attribute from svg elements");

    #[cfg(feature = "strip-stroke")]
    println!("Stripping stroke attribute from svg elements");

    #[cfg(feature = "g-force-fill-currentcolor")]
    println!("Forceing fill to 'currentColor' on group elements");

    #[cfg(feature = "allow-fill-currentcolor")]
    println!("Allowing child elements to have fill='currentColor'");

    // create font awesome icons
    const FA_SVG_BASE_PATH: &str = "../../icon_resources/font-awesome/svgs";
    let fa_icon_types: Vec<&str> = vec![
        #[cfg(feature = "fa-brands")] "brands",
        #[cfg(feature = "fa-regular")] "regular",
        #[cfg(feature = "fa-solid")] "solid",
    ];
    for icon_type in fa_icon_types.into_iter() {
        let svg_path = format!("{}/{}", FA_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/fa_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_icon_file::create_icon_file(&svg_path, &output_path, "Fa");
    }

    // create hero icons
    const HI_SVG_BASE_PATH: &str = "../../icon_resources/heroicons/src";
    let hi_icon_types: Vec<&str> = vec![
        #[cfg(feature = "hi-outline")] "outline",
        #[cfg(feature = "hi-solid")] "solid",
    ];
    for icon_type in hi_icon_types.into_iter() {
        let svg_path = format!("{}/{}", HI_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/hi_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_icon_file::create_icon_file(&svg_path, &output_path, "Hi");
    }

    // create ionicons
    #[cfg(feature = "io")] {
        const IO_SVG_BASE_PATH: &str = "../../icon_resources/ionicons/src/svg";
        let output_path = format!("{}/io_icons.rs", OUTPUT_BASE_PATH);
        create_icon_file::create_icon_file(IO_SVG_BASE_PATH, &output_path, "Io");
    }

    // create octicons
    #[cfg(feature = "go")] {
        const GO_SVG_BASE_PATH: &str = "../../icon_resources/octicons/icons";
        let go_output_path = format!("{}/go_icons.rs", OUTPUT_BASE_PATH);
        create_icon_file::create_icon_file(GO_SVG_BASE_PATH, &go_output_path, "Go");
    }

    // create bootstrap icons
    #[cfg(feature = "bs")] {
        const BS_SVG_BASE_PATH: &str = "../../icon_resources/bootstrap/icons";
        let bs_output_path = format!("{}/bs_icons.rs", OUTPUT_BASE_PATH);
        create_icon_file::create_icon_file(BS_SVG_BASE_PATH, &bs_output_path, "Bs");
    }

    // create feather icons
    #[cfg(feature = "fi")] {
        const FI_SVG_BASE_PATH: &str = "../../icon_resources/feather/icons";
        let fi_output_path = format!("{}/fi_icons.rs", OUTPUT_BASE_PATH);
        create_icon_file::create_icon_file(FI_SVG_BASE_PATH, &fi_output_path, "Fi");
    }

    // create lucide icons
    #[cfg(feature = "ld")] {
        const LD_SVG_BASE_PATH: &str = "../../icon_resources/lucide/icons";
        let ld_output_path = format!("{}/ld_icons.rs", OUTPUT_BASE_PATH);
        create_icon_file::create_icon_file(LD_SVG_BASE_PATH, &ld_output_path, "Ld");
    }

    // create material design icons
    const MD_SVG_BASE_PATH: &str = "../../icon_resources/material-design-icons/src";
    let md_icon_type: Vec<&str> = vec![
        #[cfg(feature = "md-action")] "action",
        #[cfg(feature = "md-alert")] "alert",
        #[cfg(feature = "md-av")] "av",
        #[cfg(feature = "md-communication")] "communication",
        #[cfg(feature = "md-content")] "content",
        #[cfg(feature = "md-device")] "device",
        #[cfg(feature = "md-editor")] "editor",
        #[cfg(feature = "md-file")] "file",
        #[cfg(feature = "md-hardware")] "hardware",
        #[cfg(feature = "md-home")] "home",
        #[cfg(feature = "md-image")] "image",
        #[cfg(feature = "md-maps")] "maps",
        #[cfg(feature = "md-navigation")] "navigation",
        #[cfg(feature = "md-notification")] "notification",
        #[cfg(feature = "md-places")] "places",
        #[cfg(feature = "md-social")] "social",
        #[cfg(feature = "md-toggle")] "toggle",
    ];
    for icon_type in md_icon_type.into_iter() {
        let svg_path = format!("{}/{}", MD_SVG_BASE_PATH, icon_type);
        let output_path = format!("{}/md_{}_icons.rs", OUTPUT_BASE_PATH, icon_type);
        create_icon_file::create_icon_file(&svg_path, &output_path, "Md");
    }
}
