use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct LoginForm {
    link: ComponentLink<Self>,
    props: Props,
    email_ref: NodeRef,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub onsubmit: Callback<String>,
}

pub enum Msg {
    Submit(FocusEvent),
}

impl Component for LoginForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            email_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(e) => {
                e.prevent_default();
                let email = self
                    .email_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .trim()
                    .to_owned();
                self.props.onsubmit.emit(email);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(Msg::Submit);

        html! {
            <form onsubmit=onsubmit>
                <p>{"Enter your email to receive a login link."}</p>
                <div>
                    <label for="email">{"Email"}</label>
                    <input id="email" type="text" ref=self.email_ref.clone() />
                </div>
                <button type="submit">{ "Log in" }</button>
            </form>
        }
    }
}
