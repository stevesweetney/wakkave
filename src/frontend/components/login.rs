use yew::{prelude::*, format};
use yew::services::{
    ConsoleService,
};

use frontend::{
    services::{
        protocol::ProtocolService,
        router::{RouterAgent, Route, Request},
        cookie::CookieService,
        websocket::{WebSocketAgent, WsResponse, Request as WsRequest},
    },
    routes::RouterComponent,
    SESSION_TOKEN,
};

pub struct Login { 
    username: String,
    password: String,
    ws_agent: Box<Bridge<WebSocketAgent>>,
    protocol_service: ProtocolService,
    console_service: ConsoleService,
    router_agent: Box<Bridge<RouterAgent<()>>>,
    cookie_service: CookieService,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    LoginRequest,
    LoginResponse(Vec<u8>),
    HandleRoute(Route<()>),
    Ignore,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = |msg| {
            match msg {
                WsResponse::Data(bytes) => Msg::LoginResponse(bytes),
                _ => Msg::Ignore,
            }
        };

        Login { 
            username: String::new(), 
            password: String::new(),
            ws_agent: WebSocketAgent::bridge(link.send_back(callback)),
            protocol_service: ProtocolService::new(),
            console_service: ConsoleService::new(),
            router_agent: RouterAgent::bridge(link.send_back(|route| Msg::HandleRoute(route))),
            cookie_service: CookieService::new(),  
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
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
                        self.ws_agent.send(WsRequest(data.to_vec()));
                    }
                    Err(_e) => (),
                }
            Msg::LoginResponse(res) => {
                match self.protocol_service.read_response_login(&res) {
                    Ok(Some(token)) =>  {
                        self.cookie_service.set(SESSION_TOKEN, &token);
                        self.router_agent.send(Request::ChangeRoute(RouterComponent::Feed.into()));
                    },
                    Ok(None) => return false,
                    Err(e) => {
                        self.console_service.warn(&format!("Unable to login: {}", e));
                    }
                }
            },
            Msg::Ignore => return false,
            _ => {}
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
