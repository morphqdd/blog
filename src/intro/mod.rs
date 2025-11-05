use dioxus::{core::Element, prelude::*};

use crate::post::Post;

const POSTS: &[(&str, &str)] = &[
    (
        "The First One",
        include_str!("../../posts/the-first-one.md"),
    ),
    (
        "You should try Nix",
        include_str!("../../posts/you-should-try-nix.md"),
    ),
];

#[component]
pub fn Intro() -> Element {
    rsx! {
        div {
            class: "w-full h-screen flex justify-center p-5 md:p-10",
            div {
                class: "max-w-[40rem]",
                nav {
                    class: "text-[20px]",
                    ul {
                        class: "*:px-3 flex list-none",
                        li { a { href: "/", "home"} }
                    }
                }
                for (title, src) in POSTS.iter().rev() {
                    Post { title,  src }
                }
                // h1 {
                //     class: "leading-[0.9] w-fit mt-auto",
                //     span {
                //         class: "text-[48px] -tracking-[3]",
                //         "machine"
                //     },
                //     div {
                //         class: "flex justify-between",
                //         span {
                //             class: "text-[32px] self-end",
                //             "cloud"
                //         }
                //         span {
                //             class: "text-[16px] self-end leading-[1.1]",
                //             "0trust"
                //         }
                //     }
                // }
            }
        }
    }
}
