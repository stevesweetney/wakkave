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
    InvalidRequest, Post, Vote, WsMessage, SESSION_TOKEN,
};

pub struct Feed {
    message_value: String,
    router_agent: Box<Bridge<RouterAgent<()>>>,
    cookie_service: CookieService,
    ws_agent: Box<Bridge<WebSocketAgent>>,
    console_service: ConsoleService,
    protocol_service: ProtocolService,
    posts: Vec<Post>,
}

pub enum Msg {
    UpdateMessage(String),
    SendMessage,
    Ignore,
    LogoutRequest,
    Logout(Vec<u8>),
    FetchPostsRequest,
    FetchPosts(Vec<u8>),
    CreatePostRequest,
    CreatePost(Vec<u8>),
    UserVoteRequest(i32, Vote),
    UserVote(Vec<u8>),
    InvalidPosts(Vec<u8>),
    NewPost(Vec<u8>),
    UpdateUsers(Vec<u8>),
}

impl Component for Feed {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = |msg| match msg {
            WsResponse::Data(bytes) => {
                if let Ok(ws_message) = ProtocolService::which_message(&bytes) {
                    match ws_message {
                        WsMessage::Logout => Msg::Logout(bytes),
                        WsMessage::FetchPosts => Msg::FetchPosts(bytes),
                        WsMessage::CreatePost => Msg::CreatePost(bytes),
                        WsMessage::UserVote => Msg::UserVote(bytes),
                        WsMessage::InvalidPosts => Msg::InvalidPosts(bytes),
                        WsMessage::NewPost => Msg::NewPost(bytes),
                        WsMessage::UpdateUsers => Msg::UpdateUsers(bytes),
                        _ => Msg::Ignore,
                    }
                } else {
                    Msg::Ignore
                }
            }
            _ => Msg::Ignore,
        };

        Feed {
            message_value: String::new(),
            router_agent: RouterAgent::bridge(link.send_back(|_| Msg::Ignore)),
            cookie_service: CookieService::new(),
            ws_agent: WebSocketAgent::bridge(link.send_back(callback)),
            console_service: ConsoleService::new(),
            protocol_service: ProtocolService::new(),
            posts: Vec::new(),
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // Workaround for calling this when the component mounts
        self.update(Msg::FetchPostsRequest);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateMessage(message) => {
                self.message_value = message;
                true
            }
            Msg::LogoutRequest => self.make_request(Msg::LogoutRequest),
            Msg::Logout(data) => match self.protocol_service.read_response_logout(&data) {
                Ok(Some(())) => {
                    self.router_agent
                        .send(Request::ChangeRoute(RouterComponent::Login.into()));
                    false
                }
                Ok(None) => false,
                Err(_) => {
                    self.console_service.log("Error when attempting to log out");
                    false
                }
            },
            Msg::FetchPostsRequest => self.make_request(Msg::FetchPostsRequest),
            Msg::FetchPosts(data) => match self.protocol_service.read_response_fetch_posts(&data) {
                Ok(Some((token, posts))) => {
                    self.cookie_service.set(SESSION_TOKEN, &token);
                    self.posts = posts;
                    true
                }
                _ => false,
            },
            Msg::CreatePostRequest => self.make_request(Msg::CreatePostRequest),
            Msg::CreatePost(data) => match self.protocol_service.read_response_create_post(&data) {
                Ok(Some((token, post))) => {
                    self.cookie_service.set(SESSION_TOKEN, &token);
                    self.posts.push(post);
                    true
                }
                _ => false,
            },
            Msg::UserVoteRequest(post_id, vote) => {
                self.make_request(Msg::UserVoteRequest(post_id, vote))
            }
            Msg::UserVote(data) => match self.protocol_service.read_request_user_vote(&data) {
                Ok(Some(token)) => {
                    self.cookie_service.set(SESSION_TOKEN, &token);
                    true
                }
                _ => false,
            },
            Msg::InvalidPosts(data) => match self.protocol_service.read_update_invalid(&data) {
                Ok(Some(post_ids)) => {
                    self.posts.retain(|ref mut p| !post_ids.contains(&p.id));
                    true
                }
                _ => false,
            },
            Msg::NewPost(data) => match self.protocol_service.read_update_new_post(&data) {
                Ok(Some(post)) => {
                    self.posts.push(post);
                    true
                }
                _ => false,
            },
            Msg::UpdateUsers(data) => match self.protocol_service.read_update_users(&data) {
                Ok(Some(users)) => false,
                _ => false,
            },
            Msg::Ignore => false,
            _ => false,
        }
    }
}

impl Feed {
    fn make_request(&mut self, req: Msg) -> bool {
        if let Ok(token) = self.cookie_service.get(SESSION_TOKEN) {
            let protocol_req = match req {
                Msg::LogoutRequest => self.protocol_service.write_request_logout_token(&token),
                Msg::FetchPostsRequest => self.protocol_service.write_request_fetch_posts(&token),
                Msg::CreatePostRequest => self
                    .protocol_service
                    .write_request_create_post(&token, &self.message_value),
                Msg::UserVoteRequest(post_id, vote) => self
                    .protocol_service
                    .write_request_user_vote(&token, post_id, vote),
                _ => Err(InvalidRequest.into()),
            };
            if let Ok(data) = protocol_req {
                self.ws_agent.send(WsRequest(data.to_vec()));
                false
            } else {
                self.console_service
                    .log("Error when attempting make request");
                false
            }
        } else {
            false
        }
    }
}

impl Feed {
    fn view_form(&self) -> Html<Feed> {
        html! {
            <form>
                <div class="uk-flex uk-flex-middle uk-flex-column",>
                    <div class="uk-margin",>
                        <textarea
                            class="uk-textarea uk-form-width-medium",
                            rows="5", placeholder="Enter a message",
                            value={&self.message_value},
                            oninput=|e| Msg::UpdateMessage(e.value),>
                        </textarea>
                    </div>
                    <div class="uk-margin",>
                        <button class="uk-button uk-button-default",
                        onclick=|_| Msg::CreatePostRequest,>{"Send!"}</button>
                    </div>
                </div>
            </form>
        }
    }

    fn view_posts(&self) -> Html<Feed> {
        html! {
            <ul class="uk-list uk-list-divider",>
                {for self.posts.iter().map(|p| {
                    html! {<li>{&p.content}</li>}
                })}
            </ul>
        }
    }
}

impl Renderable<Feed> for Feed {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="uk-container",>
                <p>{"Your feed will appear here"}</p>
                {self.view_posts()}
                {self.view_form()}
            </div>
        }
    }
}
