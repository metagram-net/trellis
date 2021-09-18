use yew::prelude::*;

pub struct LoginConfirmation {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub email: String,
    pub onsubmit: Callback<()>,
}

pub enum Msg {
    Submit(MouseEvent),
}

impl Component for LoginConfirmation {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(_) => {
                self.props.onsubmit.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let msg = match self.props.email.as_str() {
            "" => "Sent you a link!".to_owned(),
            email => format!("Sent a link to {}!", email),
        };
        let onclick = self.link.callback(Msg::Submit);

        html! {
            <div>
                <p>{msg}</p>
                <p>{"Look for an email from login@stytch.com."}</p>
                <p>{"Didn't get the email? Request another one:"}</p>
                <button type="button" onclick=onclick>{"Try again"}</button>
            </div>
        }
    }
}
