use iced::{
    button, executor, Application, Button, Column, Command, Element, Row, Subscription, Text,
};

use crate::jsonrpc::{self, JsonRpc};
use crate::statuses::{self, Statuses};

pub struct App {
    record_status_buttons: Vec<RecordStatusButton>,
    button: button::State,
    statuses: Statuses,
    jsonrpc: JsonRpc,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddStatus(String),
    Export,
    StatusesMessage(statuses::Message),
    JsonRpc(jsonrpc::Receive),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Vec<String>;

    fn new(flags: Self::Flags) -> (App, Command<Self::Message>) {
        let statuses = Statuses::new();
        (
            App {
                record_status_buttons: flags
                    .into_iter()
                    .map(|name| RecordStatusButton::new(name))
                    .collect(),
                button: button::State::default(),
                statuses,
                jsonrpc: JsonRpc::new(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Record Stuff")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::AddStatus(name) => {
                self.statuses.get_status(name, &mut self.jsonrpc);
                Command::none()
            }
            Message::Export => {
                self.statuses.export(&mut self.jsonrpc);
                Command::none()
            }
            Message::StatusesMessage(statuses::Message::SetName(index, name)) => {
                self.statuses.set_status_name(index, name);
                Command::none()
            }
            Message::JsonRpc(jsonrpc::Receive { id, response }) => {
                use jsonrpc::{Response, ResponseResult};
                match response {
                    ResponseResult::Response(Response::ImportStatus(import_status)) => {
                        self.statuses.set_status_value(id, import_status);
                    }
                    ResponseResult::Error {
                        code: 1,
                        message: _message,
                    } => {
                        self.statuses.remove_status(id);
                    }
                    _ => {}
                }
                Command::none()
            }
        }
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        self.jsonrpc.receive().map(Message::JsonRpc)
    }
    fn view(&mut self) -> Element<Self::Message> {
        let row = Row::new().padding(20).spacing(20);
        let col = Column::new()
            .spacing(20)
            .push(Button::new(&mut self.button, Text::new("Export")).on_press(Message::Export));
        let col = self
            .record_status_buttons
            .iter_mut()
            .fold(col, |column, button| {
                column.push(button.view().map(|name| Message::AddStatus(name)))
            });
        row.push(col)
            .push(self.statuses.view().map(Message::StatusesMessage))
            .into()
    }
}

struct RecordStatusButton {
    name: String,
    button: button::State,
}

impl RecordStatusButton {
    fn new(name: String) -> Self {
        RecordStatusButton {
            name,
            button: button::State::default(),
        }
    }
    fn view(&mut self) -> Element<String> {
        Button::new(&mut self.button, Text::new(&self.name))
            .on_press(self.name.clone())
            .into()
    }
}
