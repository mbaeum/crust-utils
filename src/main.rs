#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
fn HydrationSliderView(cx: Scope) -> Element {
    let hydration = use_state(cx, || 75);

    cx.render(rsx!(
        div {
            class: "inputHeader",
            "Hydration: {hydration}"
        }
        input {
            class: "slider",
            min: 0,
            max: 100,
            value: "{hydration}",
            r#type: "range",
            oninput: move |event| hydration.set(event.value.parse().unwrap())
         }
    ))
}

#[component]
fn TotalWeightView(cx: Scope) -> Element {
    let total = use_state(cx, || 1000);

    cx.render(rsx!(
        div {
            class: "inputHeader",
            "Total Weight: {total}"
        }
        input {
            class: "textInput",
            value: "change",
            oninput: move |event| total.set(event.value.parse().unwrap())
         }
    ))
}

fn App(cx: Scope) -> Element {
    render! {
        div {
            class: "userInput",
            HydrationSliderView { }
            TotalWeightView { }
        }
    }
}

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="style/input.css">"#.to_string()),
    );

    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}
