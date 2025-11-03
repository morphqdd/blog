use dioxus::prelude::*;

use crate::intro::Intro;

pub mod intro;
pub mod post;

const FAVICON: Asset = asset!("/assets/favicon.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Intro {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { class: "rounded", rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Noto+Serif:ital,wght@0,100..900;1,100..900&family=Roboto:ital,wght@0,100..900;1,100..900&display=swap" }
        document::Title { "morphqdd's blog post" }
        Router::<Route> {}
    }
}
