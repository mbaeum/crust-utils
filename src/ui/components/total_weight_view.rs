#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub(crate) fn TotalWeightView(cx: Scope) -> Element {
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
