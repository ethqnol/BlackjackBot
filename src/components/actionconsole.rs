#![allow(non_snake_case, unused)]
use struct_iterable::Iterable;
use dioxus::prelude::*;
use log::LevelFilter;
use crate::server;

#[component]
pub fn ActionConsole() -> Element {
    rsx! {
        button {"Hit"}
        button {"Stand"}
        button {"Insurance"}
        button {"Double"}
        button {"Surrender"}
    }
}
