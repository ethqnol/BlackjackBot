#![allow(non_snake_case, unused)]
use struct_iterable::Iterable;
use dioxus::prelude::*;
use log::LevelFilter;

mod server;
mod components;
use crate::components::{CardList, ActionConsole};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/game")]
    Game {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Game() -> Element {
    rsx! {
        CardList {}
    }
}



#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div {
            Link {
                to: Route::Game {},
                "Initialize Game"
            }
        }
    }
}


