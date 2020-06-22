use std::error::Error;
use std::hash;
use std::io::Write;
use std::process::{ChildStdin, Command, Stdio};
use std::sync::Arc;

use iced::futures::{
    io,
    prelude::*,
    stream::{self, BoxStream},
};

use iced::Subscription;

use crate::jsonrpc::{ProcessMessage, Sender};

#[derive(Debug)]
pub struct StdinSender {
    stdin: ChildStdin,
}

impl StdinSender {
    fn new(stdin: ChildStdin) -> StdinSender {
        StdinSender { stdin }
    }
}

impl Sender for StdinSender {
    fn send(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        writeln!(self.stdin, "{}", data)?;
        Ok(())
    }
}

pub fn process() -> Subscription<ProcessMessage> {
    Subscription::from_recipe(Process::new())
}

enum State<S: Stream<Item = String>> {
    Ready,
    Running(S),
    Finished,
}

struct Process;

impl Process {
    fn new() -> Process {
        Process
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for Process
where
    H: hash::Hasher,
{
    type Output = ProcessMessage;
    fn hash(&self, state: &mut H) {
        use hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }
    fn stream(self: Box<Self>, _input: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        Box::pin(stream::unfold(
            State::Ready,
            |state: State<stream::Map<_, _>>| async move {
                match state {
                    State::Ready => {
                        let mut child = Command::new(
                            "/home/charlie/programming/listen/target/release/audio-record",
                        )
                        .arg("test.wav")
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();
                        let stdin = child.stdin.take().unwrap();
                        let stdout = child.stdout.take().unwrap();
                        let stdout = io::BufReader::new(io::AllowStdIo::new(stdout));
                        let lines = stdout.lines().map(|line| line.unwrap());

                        Some((
                            ProcessMessage::InputReady(Arc::new(StdinSender::new(stdin))),
                            State::Running(lines),
                        ))
                    }
                    State::Running(mut lines) => {
                        if let Some(next_line) = lines.next().await {
                            Some((ProcessMessage::Response(next_line), State::Running(lines)))
                        } else {
                            Some((ProcessMessage::Finish, State::Finished))
                        }
                    }
                    State::Finished => {
                        None
                    }
                }
            },
        ))
    }
}
