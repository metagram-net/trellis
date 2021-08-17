use trellis_core::config;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct SecretsForm {
    link: ComponentLink<Self>,
    props: Props,
    owm_api_key_ref: NodeRef,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub secrets: config::Secrets,
    pub onchange: Callback<config::Secrets>,
}

pub enum Msg {
    Input,
    Submit,
}

impl Component for SecretsForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            owm_api_key_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                self.props.onchange.emit(self.props.secrets.clone());
                false
            }
            Msg::Input => {
                let owm_api_key = self
                    .owm_api_key_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .trim()
                    .to_owned();
                let mut secrets = self.props.secrets.clone();
                secrets.owm_api_key = if owm_api_key.is_empty() {
                    None
                } else {
                    Some(owm_api_key)
                };
                self.props.onchange.emit(secrets);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });

        let oninput = self.link.callback(|_: InputData| Msg::Input);

        html! {
            <form
                class="flex flex-col items-center justify-around w-full h-full"
                onsubmit=onsubmit
            >
                <label>
                    <a href="https://home.openweathermap.org/api_keys">{"OpenWeatherMap API Key"}</a>
                    <input
                        type="text"
                        class="w-full"
                        value=self.props.secrets.owm_api_key.clone()
                        ref=self.owm_api_key_ref.clone()
                        oninput=oninput
                    />
                </label>
            </form>
        }
    }
}
