use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketTask};
use yew::format;

pub struct Login { 
    username: String,
    password: String,
    websocket_service: WebSocketService,
    ws: WebSocketTask
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
            Msg::LoginRequest => self.ws.send(Ok("Login".to_string())),
            Msg::LoginResponse(res) => (),
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
