use std::error::Error;
use std::sync::Arc;

use serde_json::Value;

use iced::{Column, Command, Element, Length, Row, Subscription, Text};

use crate::process;

pub trait Sender {
    fn send(&mut self, data: String) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Clone)]
pub enum ProcessMessage {
    Response(String),
    InputReady(Arc<process::StdinSender>),
    Finish,
}

pub struct Statuses {
    statuses: Vec<Status>,
    next_id: u64,
    running: bool,
    sender: Option<process::StdinSender>,
}

impl Statuses {
    pub fn new() -> Statuses {
        let statuses = Statuses {
            statuses: Vec::new(),
            next_id: 0,
            running: false,
            sender: None,
        };
        statuses
    }
    pub fn update(&mut self, message: ProcessMessage) -> Command<()> {
        match message {
            ProcessMessage::Response(s) => {
                let mut res: Value = serde_json::from_str(&s).expect("Invalid JSON from process");
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
            ProcessMessage::InputReady(sender) => {
                self.sender =
                    Some(Arc::try_unwrap(sender).expect("Sender arc has strong count > 1"));
                Command::none()
            }
            ProcessMessage::Finish => {
                self.sender = None;
                self.running = false;
                Command::none()
            }
        }
    }
    pub fn get_status(&mut self, name: String) {
        self.statuses.push(Status {
            name,
            value: StatusValue::Loading(self.next_id),
        });
        self.sender
            .as_mut()
            .unwrap()
            .send(format!(
                "{{\"method\": \"get_status\", \"id\": {}}}",
                self.next_id
            ))
            .unwrap();
        self.next_id += 1;
    }
    pub fn start(&mut self) {
        self.running = true;
    }
    pub fn stop(&mut self) {
        self.sender.as_mut().unwrap().send("{\"method\": \"stop\"}".to_string()).unwrap();
    }
    pub fn running(&self) -> bool {
        self.running
    }
    pub fn subscription(&self) -> Subscription<ProcessMessage> {
        if self.running {
            process::process()
        } else {
            Subscription::none()
        }
    }
    pub fn view(&mut self) -> Element<ProcessMessage> {
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
