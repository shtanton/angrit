use std::hash;

use iced::futures::{
    io,
    prelude::*,
    stream::BoxStream,
};

use iced::Subscription;

#[derive(Debug, Clone)]
pub enum StdinMessage {
    Line(String),
}

pub fn stdin() -> Subscription<StdinMessage> {
    Subscription::from_recipe(Stdin)
}

struct Stdin;

impl<H, I> iced_native::subscription::Recipe<H, I> for Stdin
where
    H: hash::Hasher,
{
    type Output = StdinMessage;
    fn hash(&self, state: &mut H) {
        use hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }
    fn stream(self: Box<Self>, _input: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        let stdin = io::BufReader::new(io::AllowStdIo::new(std::io::stdin()));
        Box::pin(stdin.lines().map(|r| StdinMessage::Line(r.unwrap())))
    }
}
