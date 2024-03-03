#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub(crate) fn HydrationSliderView(cx: Scope) -> Element {
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
