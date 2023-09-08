use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Heasdsdllo, world!"
        }
    })
}

