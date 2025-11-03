use dioxus::{core::Element, prelude::*};
use dioxus_markdown::Markdown;

#[derive(Props, PartialEq, Clone)]
pub struct PostProps {
    title: String,
    src: String,
}

#[component]
pub fn Post(props: PostProps) -> Element {
    rsx! {
        div {
            class: "p-5",
            h2 {
                class: "text-[48px] font-bold p-4",
                "{props.title}"
            }
            Markdown { src: props.src }
        }
    }
}
