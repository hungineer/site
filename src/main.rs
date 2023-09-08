use dioxus::prelude::*;
mod app;
mod pages;

fn main() {
    
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(Root);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(Root, dioxus_desktop::Config::new().with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()));
}

#[allow(non_snake_case)]
fn Root(cx: Scope) -> Element {
    let rsx = rsx!(
        app::App {}
    );
    
    cx.render(rsx)
}
