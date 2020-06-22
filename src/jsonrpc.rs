use serde_json::Value;
use serde::Serialize;

use iced::{Column, Command, Element, Length, Row, Subscription, Text};

use crate::stdin::{stdin, StdinMessage};

pub struct Statuses {
    statuses: Vec<Status>,
    next_id: u64,
}

impl Statuses {
    pub fn new() -> Statuses {
        Statuses {
            statuses: Vec::new(),
            next_id: 0,
        }
    }
    pub fn update(&mut self, message: StdinMessage) -> Command<()> {
        let StdinMessage::Line(s) = message;
        let mut res: Value = serde_json::from_str(&s).expect("Invalid JSON from stdin");
        let id = res["id"].as_u64().unwrap();
        let v = res["response"].take();
        if let Some(status) = self
            .statuses
            .iter_mut()
            .find(|s| s.value == StatusValue::Loading(id))
        {
            status.value = StatusValue::Loaded(v);
        }
        Command::none()
    }
    pub fn get_status(&mut self, name: String) {
        self.statuses.push(Status {
            name,
            value: StatusValue::Loading(self.next_id),
        });
        println!( "{{\"method\": \"get_status\", \"id\": {}}}", self.next_id);
        self.next_id += 1;
    }
    pub fn start(&mut self) {
        println!("{{\"method\": \"start\"}}");
    }
    pub fn stop(&mut self) {
        println!("{{\"method\": \"stop\"}}");
    }
    pub fn export(&mut self) {
        let statuses_json: Vec<_> = self.statuses.iter().filter_map(|status| match &status.value {
            StatusValue::Loaded(value) => Some(StatusForJson {name: &status.name, value}),
            StatusValue::Loading(_) => None,
        }).collect();
        println!("{{\"method\": \"export\", \"params\": {{\"statuses\": {}}}}}", serde_json::to_string(&statuses_json).unwrap());
    }
    pub fn subscription(&self) -> Subscription<StdinMessage> {
        stdin()
    }
    pub fn view(&mut self) -> Element<StdinMessage> {
        self.statuses
            .iter_mut()
            .fold(Column::new(), |column, status| {
                column.push(
                    Row::new()
                        .push(Text::new(&status.name).width(Length::Fill))
                        .push(match &status.value {
                            StatusValue::Loading(_) => Text::new("Loading"),
                            StatusValue::Loaded(v) => Text::new(&v.to_string()),
                        })
                        .width(Length::Fill)
                        .spacing(20),
                )
            })
            .width(Length::Shrink)
            .into()
    }
}

struct Status {
    name: String,
    value: StatusValue,
}

#[derive(Serialize)]
struct StatusForJson<'a> {
    name: &'a str,
    value: &'a Value,
}

#[derive(std::cmp::PartialEq)]
enum StatusValue {
    Loading(u64),
    Loaded(Value),
}
