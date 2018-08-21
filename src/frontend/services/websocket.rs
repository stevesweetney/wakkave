use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::{format, prelude::worker::*};

// Messages from websocket service
pub enum Msg {
    Data(WsData),
    Connected,
    Failure,
}

// Send messages of this type so subscribers
#[derive(Serialize, Deserialize, Debug)]
pub enum WsResponse {
    Data(Vec<u8>),
    Connected,
    Failure,
}

impl Transferable for WsResponse {}

// Messages that come from other agents
#[derive(Serialize, Deserialize, Debug)]
pub struct Request(pub Vec<u8>);

impl Transferable for Request {}

pub enum WsData {
    Text(format::Text),
    Binary(format::Binary),
}

impl From<format::Text> for WsData {
    fn from(text: format::Text) -> Self {
        WsData::Text(text)
    }
}

impl From<format::Binary> for WsData {
    fn from(binary: format::Binary) -> Self {
        WsData::Binary(binary)
    }
}

pub struct WebSocketAgent {
    link: AgentLink<WebSocketAgent>,
    ws_task: WebSocketTask,
    subscribers: Vec<HandlerId>,
}

impl Agent for WebSocketAgent {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = WsResponse;

    fn create(link: AgentLink<Self>) -> Self {
        let mut ws_service = WebSocketService::new();
        let ws_task = ws_service.connect(
            "ws://127.0.0.1:8088",
            link.send_back(|data| Msg::Data(data)),
            link.send_back(|data| match data {
                WebSocketStatus::Opened => Msg::Connected,
                _ => Msg::Failure,
            }),
        );
        Self {
            link,
            ws_task,
            subscribers: Vec::with_capacity(4),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Data(ws_data) => {
                if let WsData::Binary(bin) = ws_data {
                    if let Ok(mut bytes) = bin {
                        for sub in &self.subscribers {
                            self.link.response(*sub, WsResponse::Data(bytes.clone()))
                        }
                    }
                }
            }
            Msg::Connected => {
                for sub in &self.subscribers {
                    self.link.response(*sub, WsResponse::Connected)
                }
            }
            Msg::Failure => {
                for sub in &self.subscribers {
                    self.link.response(*sub, WsResponse::Failure)
                }
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, _who: HandlerId) {
        self.ws_task.send_binary(Ok(msg.0));
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.push(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        let idx = self.subscribers.iter().position(|x| *x == id);
        if let Some(i) = idx {
            let len = self.subscribers.len();
            self.subscribers.swap(i, len - 1);
            self.subscribers.pop();
        }
    }
}
