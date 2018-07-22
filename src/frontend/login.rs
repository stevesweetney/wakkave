use yew::prelude::*;

pub struct Login { 
    username: String,
    password: String,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    LoginRequest,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login { username: String::new(), password: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateUsername(name) => {
                self.username = name;
            },
            Msg::UpdatePassword(password) => {
                self.password = password;
            },
            Msg::LoginRequest => (),
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
