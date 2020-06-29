use serde::{Deserialize, Serialize};
use serde_json::Value;

use iced::{
    futures::{io, prelude::*, stream::BoxStream},
    Subscription,
};

use std::hash;

#[derive(Serialize)]
struct Send {
    id: u64,
    #[serde(flatten)]
    method: Method,
}

#[derive(Serialize)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub enum Method {
    GetStatus,
    Export(Vec<ExportStatus>),
}

#[derive(Serialize)]
pub struct ExportStatus {
    pub name: String,
    pub value: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Receive {
    pub id: u64,
    #[serde(flatten)]
    pub response: ResponseResult,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseResult {
    Response(Response),
    Error { code: usize, message: String },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Response {
    ImportStatus(ImportStatus),
}

#[derive(Deserialize, Debug, Clone)]
pub struct ImportStatus {
    pub display: String,
    pub value: Value,
}

pub struct JsonRpc {
    next_id: u64,
}

impl JsonRpc {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }
    pub fn send(&mut self, method: Method) -> Result<u64, serde_json::error::Error> {
        let id = self.next_id;
        self.next_id += 1;
        let message = Send { id, method };
        println!("{}", &serde_json::to_string(&message)?);
        Ok(id)
    }
    pub fn receive(&self) -> Subscription<Receive> {
        Subscription::from_recipe(StdinJson)
    }
}

struct StdinJson;

impl<H, I> iced_native::subscription::Recipe<H, I> for StdinJson
where
    H: hash::Hasher,
{
    type Output = Receive;
    fn hash(&self, state: &mut H) {
        use hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
    }
    fn stream(self: Box<Self>, _input: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        let stdin = io::BufReader::new(io::AllowStdIo::new(std::io::stdin()));
        Box::pin(stdin.lines().filter_map(|l| async {
            let v: Receive = serde_json::from_str(&l.ok()?).ok()?;
            Some(v)
        }))
    }
}
