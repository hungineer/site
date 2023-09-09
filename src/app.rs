use super::pages::about::About;
use super::pages::home::Home;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Wrapper)]
        #[nest("/")]
        #[redirect("/about", || Route::About  {})]
        #[redirect("/", || Route::Home {})]
        #[end_nest]
        #[route("/site")]
        Home { },
        #[route("/site/about")]
        About {},
}

#[inline_props]
#[allow(non_snake_case)]
fn Wrapper(cx: Scope) -> Element {
    render! {
        header {
            class: "bg-gray-400",
            "Header"
         }
         body {
            class: "bg-slate-400",
            NavBar {},
            Outlet::<Route> { }
         }
        footer { 
            class: "bg-slate-400",
            "footer" }
    }
}

#[allow(non_snake_case)]
fn NavBar(cx: Scope) -> Element {
    render! {
    nav {
        class: "bg-slate-400 border-b-4 border-b-blue-400 ",
        ul {
            class: "flex flex-col sm:flex-row items-center justify-center",
            div {
                class: "flex flex-col sm:flex-row items-center justify-center sm:w-1/3 md:w-1/6",
                NavItem {
                    Link { to: Route::Home {}, "Home" }
                }
                NavItem {
                    Link { to: Route::About {}, "About" }
                }
            }
        }
        }
    }
}

#[inline_props]
#[allow(non_snake_case)]
fn NavItem<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx!(
            li {
                class: "flex-grop text-center text-xs text-black font-semibold bg-gray-500 px-3 py-1 rounded-2xl w-full",
                children
            }
    ))
}
