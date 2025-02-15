use crate::IconSet;

use dioxus_free_icons::icons::*;

pub fn get_icon_sets(width: u32, height: u32) -> Vec<IconSet> {
    vec![
        IconSet {
            code: "fa_brands_icons".to_string(),
            name: "Font Awesome Brands Icons".to_string(),
            url: "https://fontawesome.com/".to_string(),
            license: "CC BY 4.0 License".to_string(),
            license_url: "https://creativecommons.org/licenses/by/4.0/".to_string(),
            version: "6.1.1".to_string(),
            source_url: "https://github.com/FortAwesome/Font-Awesome/tree/6.1.1".to_string(),
            icons: fa_brands_icons::names(32, 32),
        },
        IconSet {
            code: "fa_regular_icons".to_string(),
            name: "Font Awesome Regular Icons".to_string(),
            url: "https://fontawesome.com/".to_string(),
            license: "CC BY 4.0 License".to_string(),
            license_url: "https://creativecommons.org/licenses/by/4.0/".to_string(),
            version: "6.1.1".to_string(),
            source_url: "https://github.com/FortAwesome/Font-Awesome/tree/6.1.1".to_string(),
            icons: fa_regular_icons::names(width, height),
        },
        IconSet {
            code: "fa_solid_icons".to_string(),
            name: "Font Awesome Solid Icons ".to_string(),
            url: "https://fontawesome.com/".to_string(),
            license: "CC BY 4.0 License".to_string(),
            license_url: "https://creativecommons.org/licenses/by/4.0/".to_string(),
            version: "6.1.1".to_string(),
            source_url: "https://github.com/FortAwesome/Font-Awesome/tree/6.1.1".to_string(),
            icons: fa_solid_icons::names(width, height),
        },
    ]
}
