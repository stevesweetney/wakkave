use yew::services::ConsoleService;
use yew::{format, prelude::*};

use frontend::{
    routes::RouterComponent,
    services::{
        cookie::CookieService,
        protocol::ProtocolService,
        router::{Request, Route, RouterAgent},
        websocket::{Request as WsRequest, WebSocketAgent, WsResponse},
    },
    SESSION_TOKEN,
};

pub struct Login {
    username: String,
    password: String,
    reg_username: String,
    reg_password: String,
    reg_password_confirm: String,
    ws_agent: Box<Bridge<WebSocketAgent>>,
    protocol_service: ProtocolService,
    console_service: ConsoleService,
    router_agent: Box<Bridge<RouterAgent<()>>>,
    cookie_service: CookieService,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    UpdateRegUsername(String),
    UpdateRegPass(String),
    UpdateRegPassConfirm(String),
    LoginRequest,
    RegisterRequest,
    LoginResponse(Vec<u8>),
    HandleRoute(Route<()>),
    Ignore,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = |msg| match msg {
            WsResponse::Data(bytes) => Msg::LoginResponse(bytes),
            _ => Msg::Ignore,
        };

        Login {
            username: String::new(),
            password: String::new(),
            reg_username: String::new(),
            reg_password: String::new(),
            reg_password_confirm: String::new(),
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
            }
            Msg::UpdatePassword(password) => {
                self.password = password;
            }
            Msg::LoginRequest => match self
                .protocol_service
                .write_request_login_credentials(&self.username, &self.password)
            {
                Ok(data) => {
                    self.ws_agent.send(WsRequest(data.to_vec()));
                }
                Err(_e) => (),
            },
            Msg::LoginResponse(res) => match self.protocol_service.read_response_login(&res) {
                Ok(Some(token)) => {
                    self.cookie_service.set(SESSION_TOKEN, &token);
                    self.router_agent
                        .send(Request::ChangeRoute(RouterComponent::Feed.into()));
                }
                Ok(None) => return false,
                Err(e) => {
                    js! {
                        alert("Incorrect username or password");
                    }
                    self.console_service
                        .warn(&format!("Unable to login: {}", e));
                }
            },
            Msg::RegisterRequest => {
                if self.reg_password != self.reg_password_confirm {
                    js! {
                        alert("Passwords must match")
                    }
                    return true;
                } else {
                    match self
                        .protocol_service
                        .write_request_registration(&self.reg_username, &self.reg_password)
                    {
                        Ok(data) => self.ws_agent.send(WsRequest(data.to_vec())),
                        Err(_) => (),
                    }
                    return false;
                }
            }
            Msg::Ignore => return false,
            Msg::UpdateRegUsername(name) => {
                self.reg_username = name;
            }
            Msg::UpdateRegPass(password) => {
                self.reg_password = password;
            }
            Msg::UpdateRegPassConfirm(password) => {
                self.reg_password_confirm = password;
            }
            _ => return false,
        }
        true
    }
}

impl Login {
    fn view_tabs() -> Html<Self> {
        html! {
            <ul class="uk-subnav", uk-switcher="connect: .login-content",
            uk-switcher="animation: uk-animation-slide-left-small",>
                <li class="uk-active",><a href="#",>{"Login"}</a></li>
                <li><a href="#",>{"Register"}</a></li>
            </ul>
        }
    }

    fn view_forms(&self) -> Html<Self> {
        html! {
            <ul class="uk-switcher login-content",>
                <li>
                    <form onsubmit="return false",>
                        <div class="uk-margin",>
                            <input
                                class="uk-input",
                                type="text",
                                name="username",
                                placeholder="Username",
                                value=&self.username,
                                oninput=|e| Msg::UpdateUsername(e.value), />
                        </div>
                        <div class="uk-margin",>
                            <input
                                class="uk-input",
                                type="text",
                                name="password",
                                placeholder="Password",
                                value=&self.password,
                                oninput=|e| Msg::UpdatePassword(e.value), />
                        </div>
                        <div class="uk-margin",>
                            <button
                                class="uk-button uk-button-default",
                                type="submit",
                                onclick=|_| Msg::LoginRequest,>{"Login"}
                            </button>
                        </div>
                    </form>
                </li>
                <li>
                    <form onsubmit="return false",>
                        <div class="uk-margin",>
                            <input
                                class="uk-input",
                                type="text",
                                name="username",
                                placeholder="Username",
                                value=&self.reg_username,
                                oninput=|e| Msg::UpdateRegUsername(e.value), />
                        </div>
                        <div class="uk-margin",>
                            <input
                                class="uk-input",
                                type="text",
                                name="password",
                                placeholder="Password",
                                value=&self.reg_password,
                                oninput=|e| Msg::UpdateRegPass(e.value), />
                        </div>
                        <div class="uk-margin",>
                            <input
                                class="uk-input",
                                type="text",
                                name="password",
                                placeholder="Confirm password",
                                value=&self.reg_password_confirm,
                                oninput=|e| Msg::UpdateRegPassConfirm(e.value), />
                        </div>
                        <div class="uk-margin",>
                            <button
                                class="uk-button uk-button-default",
                                type="submit",
                                onclick=|_| Msg::RegisterRequest,>{"Register Now"}
                            </button>
                        </div>
                    </form>
                </li>
            </ul>
        }
    }
}

impl Renderable<Login> for Login {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="uk-position-center",>
                {Self::view_tabs()}
                {self.view_forms()}
            </div>
        }
    }
}
