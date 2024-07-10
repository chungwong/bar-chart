#![allow(non_snake_case)]
mod bar_chart;

use dioxus::prelude::*;
use serde::Serialize;
use std::sync::Arc;
use tracing::Level;

use crate::bar_chart::{AsciiBarChart, BarChartExt};

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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SalesData {
    model: String,
    row_total: f64,
}

impl SalesData {
    pub fn new(model: String, row_total: f64) -> Self {
        Self { model, row_total }
    }
}

impl BarChartExt for SalesData {
    type X = f64;
}

#[component]
fn Home() -> Element {
    let data = Arc::new(vec![
        SalesData::new("Clare".into(), 64.0),
        SalesData::new("Donegal".into(), 48.0),
        SalesData::new("Mayo".into(), 57.0),
        SalesData::new("Meath".into(), 67.0),
        SalesData::new("Offaly".into(), 58.0),
        SalesData::new("Tipperary".into(), 59.0),
        SalesData::new("Wicklow".into(), 74.0),
    ]);

    rsx! {
        AsciiBarChart { items: data }
    }
}
