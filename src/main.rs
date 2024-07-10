#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::Serialize;
use std::sync::Arc;
use tracing::{info, Level};

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

#[derive(Clone, Default, PartialEq, Props)]
pub struct BarProps<T: PartialEq + 'static> {
    items: Arc<Vec<T>>,
}

#[component]
pub fn AsciiBarChart(props: BarProps<SalesData>) -> Element {
    let items: Arc<Vec<_>> = props.items.clone();

    let max_value: f64 = items
        .iter()
        .map(|sales_data| sales_data.row_total)
        .reduce(|acc, a| f64::max(acc, a))
        .unwrap();
    let increment = max_value / 25.0;

    let longest_label_length: usize = items
        .iter()
        .max_by(|s1, s2| s1.model.chars().count().cmp(&s2.model.chars().count()))
        .map(|s| s.model.chars().count())
        .unwrap_or(0);

    let src = "image src";
    let class = "css class";

    rsx! {
        div {
            for item in items.clone().iter() {
                {
                    let x = item.row_total;
                    let y = item.model.clone();
                    let (bar_chunks, remainder) = div_rem(x * 8.0 / increment, 8.0);
                
                    let mut bar = "█".repeat(bar_chunks as usize);
                
                    if remainder > 0.0 {
                        bar.push(char::from_u32('█' as u32 + (8 - remainder as u32)).unwrap());
                    }
                
                    // If the bar is empty, add a left one-eighth block
                    if bar.chars().count() == 0 {
                        bar = "▏".to_string();
                    }
                
                    rsx! {
                        div {
                            key: "{y}",
                            class,

                            img {
                                src,
                            }

                            div {
                                style: "display: inline-block",
                                pre {
                                    { format!("{:>longest_label_length$} ▏", y) }
                                }
                            }
                            div {
                                style: "display: inline-block",
                                // referencing `item` triggers an error
                                // onmouseover: move |_| {
                                //     info!("breaks on item {:?}", item);
                                // },
                                onclick: move |_| {
                                    info!("works for y{}", y);
                                },
                                "{bar}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn div_rem<T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}
