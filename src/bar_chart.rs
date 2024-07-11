use dioxus::prelude::*;
use std::sync::Arc;

pub trait BarChartExt {
    type X;
}

#[component]
fn Bars<Bar: BarChartExt<X: Clone + PartialEq + 'static> + Clone + PartialEq + 'static>(
    items: Arc<Vec<Bar>>,
) -> Element {
    rsx! {
        div {
        }
    }
}

#[derive(Clone, Default, PartialEq, Props)]
pub struct BarProps<Bar: BarChartExt<X: Clone + PartialEq + 'static> + Clone + PartialEq + 'static>
{
    items: Arc<Vec<Bar>>,
}

#[component]
fn Label(y: String, max_y: usize, image: Option<String>) -> Element {
    rsx! {
        div {
            if let Some(src) = image {
                img {
                    alt: "Image",
                    loading: "lazy",
                    width: 20,
                    height: 20,
                    src
                }
            }
        }
    }
}

#[component]
pub fn AsciiBarChart<
    Bar: BarChartExt<X: Clone + PartialEq + 'static> + Clone + PartialEq + 'static,
>(
    props: BarProps<Bar>,
) -> Element {
    let items = props.items.clone();

    rsx! {
        div {
            for (index , _item) in items.clone().iter().enumerate() {
                {
                    rsx! {
                        div {
                            key: "{index}",
                            Bars {
                                items: items.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}
