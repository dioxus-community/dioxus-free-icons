mod create_example_file;
mod create_icon_file;

fn main() {
    const OUTPUT_BASE_PATH: &str = "../lib/src/icons";
    const EXAMPLE_BASE_PATH: &str = "../example/src/icons";

    // // create flag icons
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
        let example_path = format!("{}/fa_{}_icons.rs", EXAMPLE_BASE_PATH, icon_type);
        let names = create_icon_file::create_icon_file(&svg_path, &output_path, "Fa");
        create_example_file::create_example_file(
            names,
            &example_path,
            "Font Awesome",
            "https://fontawesome.com/",
            "CC BY 4.0 License",
            "https://creativecommons.org/licenses/by/4.0/",
            "6.1.1",
            "https://github.com/FortAwesome/Font-Awesome/tree/6.1.1",
        );
    }

    /*
        | [Bootstrap Icons](https://icons.getbootstrap.com/)                               | [MIT License](https://github.com/twbs/icons/blob/main/LICENSE.md)                         | [1.8.3](https://github.com/twbs/icons/tree/v1.8.3)                  |
    | [Feather](https://feathericons.com/)                                             | [MIT License](https://github.com/feathericons/feather/blob/master/LICENSE)                | [4.29.1](https://github.com/feathericons/feather/tree/v4.29.0)      |
    | [Flag Icons](https://flagicons.lipis.dev/)                                       | [MIT License](https://github.com/lipis/flag-icons/blob/main/LICENSE)                      | [7.2.1](https://github.com/lipis/flag-icons/tree/v7.2.1)            |
    | [Font Awesome](https://fontawesome.com/)                                         | [CC BY 4.0 License](https://creativecommons.org/licenses/by/4.0/)                         | [6.1.1](https://github.com/FortAwesome/Font-Awesome/tree/6.1.1)     |
    | [Heroicons](https://heroicons.com/)                                              | [MIT License](https://github.com/tailwindlabs/heroicons/blob/master/LICENSE)              | [1.0.6](https://github.com/tailwindlabs/heroicons/tree/v1.0.6)      |
    | [Ionicons](https://ionic.io/ionicons)                                            | [MIT License](https://github.com/ionic-team/ionicons/blob/main/LICENSE)                   | [6.0.2](https://github.com/ionic-team/ionicons/tree/v6.0.2)         |
    | [Material Design icons](https://developers.google.com/fonts/docs/material_icons) | [Apache License 2.0](https://github.com/google/material-design-icons/blob/master/LICENSE) | [4.0.0](https://github.com/google/material-design-icons/tree/4.0.0) |
    | [Octicons](https://primer.style/octicons/)                                       | [MIT License](https://github.com/primer/octicons/blob/main/LICENSE)                       | [17.3.0](https://github.com/primer/octicons/tree/v17.3.0)           |
     */

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

    // create feather icons
    const LD_SVG_BASE_PATH: &str = "../../icon_resources/lucide/icons";
    let ld_output_path = format!("{}/ld_icons.rs", OUTPUT_BASE_PATH);
    create_icon_file::create_icon_file(LD_SVG_BASE_PATH, &ld_output_path, "Ld");

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
