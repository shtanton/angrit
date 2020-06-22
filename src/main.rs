mod jsonrpc;
mod stdin;

use iced::{
    button, executor,
    Application, Button, Column, Command, Element, Row, Settings, Subscription, Text,
};

use crate::jsonrpc::Statuses;
use crate::stdin::StdinMessage;

struct Hello {
    add_timestamp_buttons: Vec<AddTimestampButton>,
    record_button: button::State,
    has_recorded: bool,
    statuses: Statuses,
}

#[derive(Debug, Clone)]
enum Message {
    AddStatus(String),
    Nothing,
    Record,
    Stop,
    StatusesMessage(StdinMessage),
}

impl Application for Hello {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        let statuses = Statuses::new();
        (
            Hello {
                add_timestamp_buttons: vec![
                    AddTimestampButton::new("sermon".to_string()),
                    AddTimestampButton::new("reading".to_string()),
                    AddTimestampButton::new("test".to_string()),
                ],
                record_button: button::State::default(),
                has_recorded: false,
                statuses,
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
                self.statuses.get_status(name);
                Command::none()
            }
            Message::Nothing => Command::none(),
            Message::Record => {
                self.statuses.start();
                Command::none()
            }
            Message::Stop => {
                self.statuses.stop();
                Command::none()
            }
            Message::StatusesMessage(process_message) => self
                .statuses
                .update(process_message)
                .map(|()| Message::Nothing),
        }
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        self.statuses.subscription().map(Message::StatusesMessage)
    }
    fn view(&mut self) -> Element<Self::Message> {
        let row = Row::new().padding(20).spacing(20);
        let col = Column::new().spacing(20);
        let col = if !self.has_recorded {
            col.push(if !self.statuses.running() {
                Button::new(&mut self.record_button, Text::new("Record")).on_press(Message::Record)
            } else {
                Button::new(&mut self.record_button, Text::new("Stop")).on_press(Message::Stop)
            })
        } else {
            col
        };
        let col = if self.statuses.running() {
            self.add_timestamp_buttons
                .iter_mut()
                .fold(col, |column, button| {
                    column.push(button.view().map(|name| Message::AddStatus(name)))
                })
        } else {
            col
        };
        row.push(col)
            .push(self.statuses.view().map(Message::StatusesMessage))
            .into()
    }
}

struct AddTimestampButton {
    name: String,
    button: button::State,
}

impl AddTimestampButton {
    fn new(name: String) -> Self {
        AddTimestampButton {
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

fn main() {
    Hello::run(Settings::default());
}
