use iced::{Column, Element, Length, Row, Text, text_input, TextInput};

use serde_json::Value;

use crate::jsonrpc::{JsonRpc, Method, ImportStatus, ExportStatus};

pub struct Statuses {
    statuses: Vec<Status>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetName(usize, String),
}

impl Statuses {
    pub fn new() -> Statuses {
        Statuses {
            statuses: Vec::new(),
        }
    }
    pub fn set_status_name(&mut self, index: usize, name: String) {
        self.statuses[index].name = name;
    }
    pub fn set_status_value(&mut self, id: u64, data: ImportStatus) {
        if let Some(placeholder) = self
            .statuses
            .iter_mut()
            .find(|s| if let StatusValue::Loading(aid) = s.value {aid == id} else {false})
        {
            placeholder.value = StatusValue::Loaded(data.display, data.value);
        }
    }
    pub fn get_status(&mut self, name: String, jsonrpc: &mut JsonRpc) {
        let id = jsonrpc.send(Method::GetStatus).unwrap();
        self.statuses.push(Status {
            name,
            value: StatusValue::Loading(id),
            input_state: text_input::State::new(),
        });
    }
    pub fn export(&mut self, jsonrpc: &mut JsonRpc) {
        let statuses_json: Vec<_> = self.statuses.iter().filter_map(|status| match &status.value {
            StatusValue::Loaded(_, value) => Some(ExportStatus {name: status.name.clone(), value: value.clone()}),
            StatusValue::Loading(_) => None,
        }).collect();
        jsonrpc.send(Method::Export(statuses_json)).unwrap();
    }
    pub fn view(&mut self) -> Element<Message> {
        self.statuses
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |column, (index, status)| {
                column.push(status.view().map(move |s| Message::SetName(index, s)))
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
    fn view(&mut self) -> Element<String> {
        let name_element: Element<String> = TextInput::new(&mut self.input_state, "", &self.name, |m| m).into();
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

enum StatusValue {
    Loading(u64),
    Loaded(String, Value),
}
