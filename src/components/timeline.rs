use chrono::{Datelike, NaiveDate};
use yew::{function_component, html, Html, Properties};

use super::Uri;

#[derive(Clone, PartialEq)]
pub struct Event {
    name: String,
    description: String,
    img_src: Uri,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl Event {
    pub fn new(
        name: &str,
        description: &str,
        img_src: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            img_src: img_src.into(),
            start_date,
            end_date,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub events: Vec<Event>,
}

#[function_component(Timeline)]
pub fn timeline(TimelineProps { events }: &TimelineProps) -> Html {
    let events: Vec<Html> = events
        .iter()
        .map(|event| {
            html! {
            <>
                <h3>{ event.name.clone() }</h3>
                <img src={ event.img_src.clone() } />
                <p>{ event.description.clone() }</p>
                <p>
                     { format!("{} {} - {} {}",
                       event.start_date.year(),
                       event.start_date.month(),
                       event.end_date.year(),
                       event.end_date.month()) }
                </p>
            </>
            }
        })
        .collect();
    html! {
    <>
        { for events }
    </>
    }
}
