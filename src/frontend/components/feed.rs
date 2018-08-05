use yew::prelude::*;

pub struct Feed;

pub enum Msg {
    Ignore,
}

impl Component for Feed {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Feed
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
