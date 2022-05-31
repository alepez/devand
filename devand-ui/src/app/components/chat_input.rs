use web_sys::KeyboardEvent;
use yew::{prelude::*, Properties};
use yewtil::NeqAssign;

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
    SendClicked,
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
                    self.try_emit_on_return();
                }
                true
            }
            Msg::SendClicked => {
                self.try_emit_on_return();
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
        <div class="devand-chat-input">
            <input class="devand-chat-input-box" type="text" value=self.state.txt.clone() onkeydown=self.link.callback(Msg::Keydown) oninput=self.link.callback(Msg::Input) />
            <button class="devand-chat-input-button pure-button pure-button-primary" onclick=self.link.callback(|_| Msg::SendClicked)>{ "Send" }</button>
        </div>
        }
    }
}

impl ChatInput {
    fn try_emit_on_return(&mut self) {
        let txt = self.state.txt.clone();
        if !self.state.txt.is_empty() {
            self.state.txt.clear();
            self.props.on_return.emit(txt);
        }
    }
}
