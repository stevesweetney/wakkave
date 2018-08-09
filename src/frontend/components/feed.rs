use yew::{prelude::*, format};
use yew::services::{
    websocket::{WebSocketService, WebSocketTask},
    ConsoleService,
};

use frontend::{
    services::{
        protocol::ProtocolService,
        router::{RouterAgent, Route, Request},
        cookie::CookieService,
    },
    routes::RouterComponent,
    SESSION_TOKEN,
};

pub struct Feed {
    router_agent: Box<Bridge<RouterAgent<()>>>,
    cookie_service: CookieService,
}

pub enum Msg {
    Ignore,
}

impl Component for Feed {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Feed {
            router_agent: RouterAgent::bridge(link.send_back(|_| Msg::Ignore)),
            cookie_service: CookieService::new(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => false,
        }
    }
}

impl Renderable<Feed> for Feed {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <p>{"Your feed will appear here"}</p>
            </div>
        }
    }
}
