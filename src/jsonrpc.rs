use serde_json::Value;
use serde::Serialize;

use iced::{Column, Command, Element, Length, Row, Subscription, Text, text_input, TextInput};

use crate::stdin::{stdin};

pub struct Statuses {
    statuses: Vec<Status>,
    next_id: u64,
}

#[derive(Debug, Clone)]
pub enum Message {
    StdinLine(String),
    SetName(usize, String),
}

impl Statuses {
    pub fn new() -> Statuses {
        Statuses {
            statuses: Vec::new(),
            next_id: 0,
        }
    }
    pub fn update(&mut self, message: Message) -> Command<()> {
        match message {
            Message::StdinLine(s) => {
                let mut res: Value = serde_json::from_str(&s).expect("Invalid JSON from stdin");
                let id = res["id"].as_u64().unwrap();
                let mut v = res["response"].take();
                if let Some(status) = self
                    .statuses
                    .iter_mut()
                    .find(|s| s.value == StatusValue::Loading(id))
                {
                    status.value = StatusValue::Loaded(v["display"].as_str().unwrap().to_string(), v["value"].take());
                }
                Command::none()
            }
            Message::SetName(index, s) => {
                self.statuses[index].name = s;
                Command::none()
            }
        }
    }
    pub fn get_status(&mut self, name: String) {
        self.statuses.push(Status {
            name,
            value: StatusValue::Loading(self.next_id),
            input_state: text_input::State::new(),
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
            StatusValue::Loaded(_, value) => Some(StatusForJson {name: &status.name, value}),
            StatusValue::Loading(_) => None,
        }).collect();
        println!("{{\"method\": \"export\", \"params\": {{\"statuses\": {}}}}}", serde_json::to_string(&statuses_json).unwrap());
    }
    pub fn subscription(&self) -> Subscription<Message> {
        stdin().map(Message::StdinLine)
    }
    pub fn view(&mut self, editable: bool) -> Element<Message> {
        self.statuses
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |column, (index, status)| {
                column.push(status.view(editable).map(move |s| Message::SetName(index, s)))
            })
            .width(Length::Shrink)
            .into()
    }
}

struct Status {
    name: String,
    value: StatusValue,
    input_state: text_input::State,
}

impl Status {
    fn view(&mut self, editable: bool) -> Element<String> {
        let name_element: Element<String> = if editable {
            TextInput::new(&mut self.input_state, "", &self.name, |m| m).into()
        } else {
            Text::new(&self.name).width(Length::Fill).into()
        };
        Row::new()
            .push(name_element)
            .push(match &self.value {
                StatusValue::Loading(_) => Text::new("Loading"),
                StatusValue::Loaded(s, _) => Text::new(s),
            })
            .width(Length::Fill)
            .spacing(20)
            .into()
    }
}

#[derive(Serialize)]
struct StatusForJson<'a> {
    name: &'a str,
    value: &'a Value,
}

#[derive(std::cmp::PartialEq)]
enum StatusValue {
    Loading(u64),
    Loaded(String, Value),
}
