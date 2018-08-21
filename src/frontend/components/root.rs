//! The Root component

use frontend::{
    components::{feed::Feed, login::Login},
    routes::RouterComponent,
    services::{
        cookie::CookieService,
        protocol::ProtocolService,
        router::{Request, Route, RouterAgent},
        websocket::{Request as WsRequest, WebSocketAgent, WsResponse},
    },
    SESSION_TOKEN,
};
use yew::{format, prelude::*, services::ConsoleService};

/// Available message types to process
pub enum Msg {
    HandleRoute(Route<()>),
    LoginRequest,
    LoginResponse(Vec<u8>),
    WebSocketConnected,
    WebSocketFailure,
}

/// Data Model for the Root Component
pub struct RootComponent {
    router_agent: Box<Bridge<RouterAgent<()>>>,
    child_component: RouterComponent,
    console_service: ConsoleService,
    protocol_service: ProtocolService,
    ws_agent: Box<Bridge<WebSocketAgent>>,
    cookie_service: CookieService,
}

impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = |msg| match msg {
            WsResponse::Connected => Msg::WebSocketConnected,
            WsResponse::Failure => Msg::WebSocketFailure,
            WsResponse::Data(bytes) => Msg::LoginResponse(bytes),
        };
        Self {
            router_agent: RouterAgent::bridge(link.send_back(|route| Msg::HandleRoute(route))),
            child_component: RouterComponent::Loading,
            console_service: ConsoleService::new(),
            protocol_service: ProtocolService::new(),
            ws_agent: WebSocketAgent::bridge(link.send_back(callback)),
            cookie_service: CookieService::new(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WebSocketConnected => {
                if let Ok(token) = self.cookie_service.get(SESSION_TOKEN) {
                    match self.protocol_service.write_request_login_token(&token) {
                        Ok(data) => {
                            self.console_service
                                .info("Token found, trying to authenticate");
                            self.ws_agent.send(WsRequest(data.to_vec()));
                        }
                        Err(_) => {
                            self.cookie_service.remove(SESSION_TOKEN);
                            self.router_agent
                                .send(Request::ChangeRoute(RouterComponent::Login.into()));
                            return true;
                        }
                    }
                } else {
                    self.console_service
                        .info("No token found, routing to login");
                    self.router_agent
                        .send(Request::ChangeRoute(RouterComponent::Login.into()));
                    return true;
                }
                false
            }
            Msg::LoginResponse(response) => {
                match self.protocol_service.read_response_login(&response) {
                    Ok(Some(token)) => {
                        self.cookie_service.set(SESSION_TOKEN, &token);
                        self.router_agent
                            .send(Request::ChangeRoute(RouterComponent::Feed.into()));
                        true
                    }
                    Ok(None) => false, // Not my response
                    Err(e) => {
                        // Remote the existing cookie
                        self.console_service.info(&format!("Login failed: {}", e));
                        self.cookie_service.remove(SESSION_TOKEN);
                        self.router_agent
                            .send(Request::ChangeRoute(RouterComponent::Login.into()));
                        true
                    }
                }
            }
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
                <div class="uk-position-center", uk-spinner="",></div>
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
