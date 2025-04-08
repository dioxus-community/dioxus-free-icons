[![Discord Server](https://img.shields.io/discord/899851952891002890.svg?logo=discord&style=flat-square)](https://discord.gg/sKJSVNSCDJ)
[![Crates.io](https://img.shields.io/crates/v/dioxus-free-icons)](https://crates.io/crates/dioxus-free-icons)

# dioxus-free-icons 🙂

Use free svg icons in your [Dioxus](https://dioxuslabs.com/) projects easily with dioxus-free-icons.

More information about this crate can be found in the [crate documentation](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/).

## Install

To use `dioxus-free-icons`, add this to your Cargo.toml:

```toml
[dependencies]
dioxus-free-icons = { version = "0.9", features = ["font-awesome-brands"] }
```

### Support features

The following features are available. Please see [react-icons site](https://react-icons.github.io/react-icons) to check the icon name and icon design. 

- [bootstrap](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/bs_icons/index.html)
- [font-awesome-brands](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/fa_brands_icons/index.html)
- [font-awesome-regular](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/fa_regular_icons/index.html)
- [font-awesome-solid](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/fa_solid_icons/index.html)
- [feather](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/fi_icons/index.html)
- [octicons](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/go_icons/index.html)
- [hero-icons-outline](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/hi_outline_icons/index.html)
- [hero-icons-solid](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/hi_solid_icons/index.html)
- [ionicons](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/io_icons/index.html)
- [lucide](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/ld_icons/index.html)
- [material-design-icons-action](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_action_icons/index.html)
- [material-design-icons-alert](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_alert_icons/index.html)
- [material-design-icons-av](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_av_icons/index.html)
- [material-design-icons-communication](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_communication_icons/index.html)
- [material-design-icons-content](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_content_icons/index.html)
- [material-design-icons-device](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/io_icons/index.html)
- [material-design-icons-editor](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_editor_icons/index.html)
- [material-design-icons-file](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_file_icons/index.html)
- [material-design-icons-hardware](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_hardware_icons/index.html)
- [material-design-icons-home](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_home_icons/index.html)
- [material-design-icons-image](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_image_icons/index.html)
- [material-design-icons-maps](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_maps_icons/index.html)
- [material-design-icons-navigation](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_navigation_icons/index.html)
- [material-design-icons-notification](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_notification_icons/index.html)
- [material-design-icons-places](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_places_icons/index.html)
- [material-design-icons-social](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_social_icons/index.html)
- [material-design-icons-toggle](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/md_toggle_icons/index.html)
- [simple-icons](https://docs.rs/dioxus-free-icons/latest/dioxus_free_icons/icons/si_icons/index.html)

## Example

This library provides Icon component, which will generate SVG for a Font Awesome icon.

```rust
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::Icon;

fn RustIcon() -> Element {
    rsx!(
        Icon {
            width: 30,
            height: 30,
            fill: "black",
            icon: FaRust,
        }
    )
}
```

## License

This project is licensed under the MIT license.

### Icon

Icon Library|License|Version
---|---|---
[Bootstrap Icons](https://icons.getbootstrap.com/)|[MIT License](https://github.com/twbs/icons/blob/main/LICENSE.md)| [1.8.3](https://github.com/twbs/icons/tree/v1.8.3)
[Feather](https://feathericons.com/)|[MIT License](https://github.com/feathericons/feather/blob/master/LICENSE)| [4.29.0](https://github.com/feathericons/feather/tree/v4.29.0)
[Font Awesome](https://fontawesome.com/)|[CC BY 4.0 License](https://creativecommons.org/licenses/by/4.0/)| [6.1.1](https://github.com/FortAwesome/Font-Awesome/tree/6.1.1)
[Heroicons](https://heroicons.com/)|[MIT License](https://github.com/tailwindlabs/heroicons/blob/master/LICENSE)| [1.0.6](https://github.com/tailwindlabs/heroicons/tree/v1.0.6)
[Ionicons](https://ionic.io/ionicons)|[MIT License](https://github.com/ionic-team/ionicons/blob/main/LICENSE)| [6.0.2](https://github.com/ionic-team/ionicons/tree/v6.0.2)
[Lucide](https://lucide.dev)|[ISC License](https://github.com/lucide-icons/lucide/blob/main/LICENSE)| [0.265.0](https://github.com/lucide-icons/lucide/tree/v0.265.0)
[Material Design icons](https://developers.google.com/fonts/docs/material_icons)|[Apache License 2.0](https://github.com/google/material-design-icons/blob/master/LICENSE)| [4.0.0](https://github.com/google/material-design-icons/tree/4.0.0)
[Octicons](https://primer.style/octicons/)|[MIT License](https://github.com/primer/octicons/blob/main/LICENSE)| [17.3.0](https://github.com/primer/octicons/tree/v17.3.0)
[Simple Icons](https://simpleicons.org/)| [CC0 1.0](https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md)| [14.12.1](https://github.com/simple-icons/simple-icons/tree/14.12.1)

## Contribution

The project welcomes all contributions from anyone willing to work in good faith with other contributors and the community. 
In particular, contributions regarding support for other free icons such as Material Design icons or Ionicons are welcome. 
This library aims to be a react-icons-like library for dioxus projects.

### Development

```sh
// generate icon files
cd packages/codegen
cargo run
```

### Preview

```sh
cd packages/exmaple
cargo install dioxus-cli
dx serve
```

### Update icons

1. checkout a new tag in the icon resource submodule
2. create new icon files
3. Update README.md and check the LICENSE
