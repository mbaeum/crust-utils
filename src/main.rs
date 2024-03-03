#![allow(non_snake_case)]
mod ui;
use crate::ui::App;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="style/input.css">"#.to_string()),
    );

    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}
