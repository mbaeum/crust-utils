#![allow(non_snake_case)]
mod components;
use components::{HydrationSliderView, TotalWeightView};
use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
    render! {
        div {
            class: "userInput",
            HydrationSliderView { }
            TotalWeightView { }
        }
    }
}
