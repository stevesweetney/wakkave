use yew::prelude::*;
use yew::services::{
    websocket::{WebSocketService, WebSocketTask},
    ConsoleService,
};
use yew::format;

use frontend::services::protocol::ProtocolService;

pub struct Login { 
    username: String,
    password: String,
    websocket_service: WebSocketService,
    ws: WebSocketTask,
    protocol_service: ProtocolService,
    console_service: ConsoleService,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    LoginRequest,
    LoginResponse(WsResponse),
    Ignore,
}

pub enum WsResponse {
    Text(format::Text),
    Binary(format::Binary),
}

impl From<format::Text> for WsResponse {
    fn from(text: format::Text) -> Self {
        WsResponse::Text(text)
    }
}

impl From<format::Binary> for WsResponse {
    fn from(binary: format::Binary) -> Self {
        WsResponse::Binary(binary)
    }
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_service = WebSocketService::new();
        let callback = link.send_back(|data| Msg::LoginResponse(data));
        let notification = link.send_back(|_| Msg::Ignore);

        let ws_task = ws_service.connect("ws://127.0.0.1:8088", callback, notification);
        Login { 
            username: String::new(), 
            password: String::new(),
            websocket_service: ws_service,
            ws: ws_task,
            protocol_service: ProtocolService::new(),
            console_service: ConsoleService::new(),  
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateUsername(name) => {
                self.username = name;
            },
            Msg::UpdatePassword(password) => {
                self.password = password;
            },
            Msg::LoginRequest => match self
                .protocol_service
                .write_request_login_credentials(&self.username, &self.password) {
                    Ok(data) => {
                        self.ws.send_binary(Ok(data.to_owned()));
                    }
                    Err(e) => (),
                }
            Msg::LoginResponse(res) => {
                if let WsResponse::Binary(bin) = res {
                    if let Ok(bytes) = bin {
                        if let Ok(resp) = self.protocol_service.read_response_login(bytes) {
                            if let Some(token) = resp {
                                self.console_service.log(&token)
                            }
                        }
                    }
                }
            },
            Msg::Ignore => return false,
        }
        true
    }
}

impl Renderable<Login> for Login {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <form onsubmit="return false",>
                    <br/>
                    <input type="text", 
                        name="username", 
                        placeholder="Username",
                        value=&self.username, 
                        oninput=|e| Msg::UpdateUsername(e.value), />
                    <br/>
                    <br/>
                    <input type="text", 
                        name="password", 
                        placeholder="Password",
                        value=&self.password,
                        oninput=|e| Msg::UpdatePassword(e.value), />
                    <br/>
                    <button type="submit",
                        onclick=|_| Msg::LoginRequest,>{"Login"}</button>
                </form> 
            </div>
        }
    }
}
