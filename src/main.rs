#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let global_filter: Signal<GlobalFilter> = use_signal(|| GlobalFilter::default());

    rsx! {
        button {
            onclick: move |_| {
                let filter: GlobalFilter = global_filter.read_unchecked().to_owned();
                async move {
                    get_server_data(filter).await;
                }
            },
            "Button"
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GlobalFilter {
    pub test_string: String,

    pub test_vec: Vec<String>,
    pub test_hashset: HashSet<String>,
    pub test_hashmap: HashMap<String, String>,
    // wrap them in Option to bypass the bug
    // pub test_vec: Option<Vec<String>>,
    // pub test_hashset: Option<HashSet<String>>,
    // pub test_hashmap: Option<HashMap<String, String>>,
}

#[server]
async fn get_server_data(global_filter: GlobalFilter) -> Result<(), ServerFnError> {
    Ok(())
}
