use crate::IconSet;

mod fa_brands_icons;
mod fa_regular_icons;
mod fa_solid_icons;

pub fn get_icon_sets() -> Vec<IconSet> {
    vec![
        fa_brands_icons::get_icon_set(),
        fa_regular_icons::get_icon_set(),
        fa_solid_icons::get_icon_set(),
    ]
}
