mod jsonrpc;
mod stdin;

use iced::{
    button, executor,
    Application, Button, Column, Command, Element, Row, Settings, Subscription, Text,
};

use crate::jsonrpc::Statuses;

#[derive(PartialEq)]
enum RecordingStatus {
    Ready,
    Started,
    Stopped,
    Exported,
}

struct Hello {
    record_status_buttons: Vec<RecordStatusButton>,
    button: button::State,
    statuses: Statuses,
    recording_status: RecordingStatus,
}

#[derive(Debug, Clone)]
enum Message {
    AddStatus(String),
    Nothing,
    Start,
    Stop,
    Export,
    StatusesMessage(jsonrpc::Message),
}

impl Application for Hello {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        let statuses = Statuses::new();
        (
            Hello {
                record_status_buttons: vec![
                    RecordStatusButton::new("sermon".to_string()),
                    RecordStatusButton::new("reading".to_string()),
                    RecordStatusButton::new("test".to_string()),
                ],
                button: button::State::default(),
                statuses,
                recording_status: RecordingStatus::Ready,
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
            Message::Start => {
                self.recording_status = RecordingStatus::Started;
                self.statuses.start();
                Command::none()
            }
            Message::Stop => {
                self.recording_status = RecordingStatus::Stopped;
                self.statuses.stop();
                Command::none()
            }
            Message::Export => {
                self.recording_status = RecordingStatus::Exported;
                self.statuses.export();
                Command::none()
            }
            Message::StatusesMessage(process_message) => self
                .statuses
                .update(process_message)
                .map(|()| Message::Nothing),
        }
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        if self.recording_status == RecordingStatus::Started {
            self.statuses.subscription().map(Message::StatusesMessage)
        } else {
            Subscription::none()
        }
    }
    fn view(&mut self) -> Element<Self::Message> {
        let row = Row::new().padding(20).spacing(20);
        let col = Column::new().spacing(20);
        let col = match self.recording_status {
            RecordingStatus::Ready => col.push(Button::new(&mut self.button, Text::new("Start Recording")).on_press(Message::Start)),
            RecordingStatus::Started => {
                let col_with_button = col.push(Button::new(&mut self.button, Text::new("Stop Recording")).on_press(Message::Stop));
                self.record_status_buttons
                    .iter_mut()
                    .fold(col_with_button, |column, button| {
                        column.push(button.view().map(|name| Message::AddStatus(name)))
                    })
            },
            RecordingStatus::Stopped => col.push(Button::new(&mut self.button, Text::new("Export")).on_press(Message::Export)),
            RecordingStatus::Exported => col.push(Text::new("Exported")),
        };
        row.push(col)
            .push(self.statuses.view(self.recording_status != RecordingStatus::Exported).map(Message::StatusesMessage))
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

fn main() {
    Hello::run(Settings::default());
}
