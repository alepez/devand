use web_sys::KeyboardEvent;
use yew::{prelude::*, Properties};

pub struct ChatInput {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_return: Callback<String>,
}

pub enum Msg {
    Input(InputData),
    Keydown(KeyboardEvent),
}

#[derive(Default)]
struct State {
    txt: String,
}

impl Component for ChatInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();
        Self { props, state, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(e) => {
                self.state.txt = e.value;
                true
            }
            Msg::Keydown(e) => {
                if e.key_code() == 13 {
                    let txt = self.state.txt.clone();
                    self.state.txt.clear();
                    self.props.on_return.emit(txt);
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <input type="text" value=self.state.txt onkeydown=self.link.callback(Msg::Keydown) oninput=self.link.callback(Msg::Input) />
        }
    }
}
