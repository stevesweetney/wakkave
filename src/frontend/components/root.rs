//! The Root component

use frontend::{
    components::{login::Login, feed::Feed},
    routes::RouterComponent,
    services::{
        protocol::ProtocolService,
        cookie::CookieService,
        router::{Request, Route, RouterAgent},
    },
    SESSION_TOKEN,
};
use yew::{prelude::*, format, services::{
    websocket::{WebSocketService, WebSocketTask, WebSocketStatus},
    ConsoleService,
}};

/// Available message types to process
pub enum Msg {
    HandleRoute(Route<()>),
    LoginRequest,
    LoginResponse(WsResponse),
    WebSocketConnected,
    WebSocketFailure,
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

/// Data Model for the Root Component
pub struct RootComponent {
    router_agent: Box<Bridge<RouterAgent<()>>>,
    child_component: RouterComponent,
    console_service: ConsoleService,
    protocol_service: ProtocolService,
    websocket_service: WebSocketService,
    ws: WebSocketTask,
    cookie_service: CookieService,
}

impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_service = WebSocketService::new();
        let ws_task = ws_service.connect(
                "ws://127.0.0.1:8088",
                link.send_back(|data| Msg::LoginResponse(data)),
                link.send_back(|data| match data {
                    WebSocketStatus::Opened => Msg::WebSocketConnected,
                    _ => Msg::WebSocketFailure,
                })
            );
        Self {
            router_agent: RouterAgent::bridge(link.send_back(|route| Msg::HandleRoute(route))),
            child_component: RouterComponent::Loading,
            console_service: ConsoleService::new(),
            protocol_service: ProtocolService::new(),
            websocket_service: ws_service,
            ws: ws_task,
            cookie_service: CookieService::new(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WebSocketConnected => {
                self.router_agent
                    .send(Request::ChangeRoute(RouterComponent::Login.into()));
                true
            },
            Msg::LoginResponse(response) => {
                if let WsResponse::Binary(bin) = response {
                    if let Ok(mut bytes) = bin {
                        match self.protocol_service.read_response_login(&mut bytes) {
                            Ok(Some(token)) => {
                                self.cookie_service.set(SESSION_TOKEN, &token);
                                self.router_agent
                                    .send(Request::ChangeRoute(RouterComponent::Feed.into()));
                                return true;
                            }
                            Ok(None) => {return false;}, // Not my response
                            Err(e) => {
                                // Remote the existing cookie
                                self.console_service.info(&format!("Login failed: {}", e));
                                self.cookie_service.remove(SESSION_TOKEN);
                                self.router_agent
                                    .send(Request::ChangeRoute(RouterComponent::Login.into()));
                                return true;
                            }
                        }
                    }
                }
                false  
            },
            Msg::HandleRoute(route) => {
                self.child_component = route.into();
                true
            }
            _ => {
                self.router_agent
                    .send(Request::ChangeRoute(RouterComponent::Error.into()));
                true
            }
        }
    }
}

impl Renderable<RootComponent> for RootComponent {
    fn view(&self) -> Html<Self> {
        self.child_component.view()
    }
}

impl Renderable<RootComponent> for RouterComponent {
    fn view(&self) -> Html<RootComponent> {
        match *self {
            RouterComponent::Loading => html! {
                <div class="uk-position-center", uk-spinner="",>{"Loading..."}</div>
            },
            RouterComponent::Error => html! {
                <div class="uk-position-center",>
                    {"Error loading application."}
                </div>
            },
            RouterComponent::Login => html! {
               <Login:/>
            },
            RouterComponent::Feed => html! {
               <Feed:/>
            },
        }
    }
}
