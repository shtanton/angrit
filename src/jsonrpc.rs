use serde_json::Value;

use iced::{Column, Command, Element, Length, Row, Subscription, Text};

use crate::stdin::{stdin, StdinMessage};

pub struct Statuses {
    statuses: Vec<Status>,
    next_id: u64,
    running: bool,
}

impl Statuses {
    pub fn new() -> Statuses {
        Statuses {
            statuses: Vec::new(),
            next_id: 0,
            running: false,
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
        self.running = true;
    }
    pub fn stop(&mut self) {
        println!("{{\"method\": \"stop\"}}");
        self.running = false;
    }
    pub fn running(&self) -> bool {
        self.running
    }
    pub fn subscription(&self) -> Subscription<StdinMessage> {
        if self.running {
            stdin()
        } else {
            Subscription::none()
        }
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

#[derive(std::cmp::PartialEq)]
enum StatusValue {
    Loading(u64),
    Loaded(Value),
}
