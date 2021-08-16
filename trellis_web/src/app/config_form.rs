use serde_json;
use trellis_core::config;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub config: config::Config,
    pub onsave: Callback<config::Config>,
}

pub struct ConfigForm {
    link: ComponentLink<Self>,
    props: Props,
    textarea_ref: NodeRef,
    error: Option<String>,
}

pub enum Msg {
    Save,
}

impl Component for ConfigForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            textarea_ref: NodeRef::default(),
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Save => {
                let text = self
                    .textarea_ref
                    .cast::<HtmlTextAreaElement>()
                    .unwrap()
                    .value();
                match serde_json::from_str::<config::Config>(&text) {
                    Ok(cfg) => {
                        self.props.onsave.emit(cfg);
                        self.error = None;
                    }
                    Err(err) => {
                        self.error = Some(err.to_string());
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Save
        });
        let text =
            serde_json::to_string_pretty(&self.props.config).expect("Could not serialize settings");
        let rows = format!(
            "{}",
            // Add an extra row because the last line doesn't have a newline.
            text.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1
        );

        let errors = match &self.error {
            None => html! {},
            Some(msg) => html! {
                <div class="alert">
                    <p>{msg}</p>
                </div>
            },
        };

        html! {
            <form onsubmit=onsubmit>
                <textarea class="w-full" rows=rows ref=self.textarea_ref.clone()>
                    {text}
                </textarea>
                <button type="submit" class="btn btn-gray">{ "Save" }</button>
                {errors}
            </form>
        }
    }
}
