use std::time::Duration;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::services::timeout::{TimeoutService, TimeoutTask};

pub struct Note {
    props: Props,
    link: ComponentLink<Self>,
    form_ref: NodeRef,
    textarea_ref: NodeRef,
    debounce: Option<TimeoutTask>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub text: String,
    pub onchange: Callback<String>,
}

pub enum Msg {
    Edited,
    Saved,
}

impl Component for Note {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link: link.clone(),
            textarea_ref: NodeRef::default(),
            form_ref: NodeRef::default(),
            debounce: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Edited => {
                // TODO: Prevent navigation while waiting for save
                self.debounce = Some(TimeoutService::spawn(
                    Duration::from_millis(1000),
                    self.link.callback(|_| Msg::Saved),
                ));
                true
            }
            Msg::Saved => {
                self.debounce = None;
                let text = self
                    .textarea_ref
                    .cast::<HtmlTextAreaElement>()
                    .unwrap()
                    .value();
                self.props.onchange.emit(text);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // If a save is waiting, let it win.
        //
        // TODO: Maybe merge the text somehow?
        match self.debounce {
            Some(_) => false,
            None => {
                self.props = props;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let mut classes = classes!("w-full", "h-full", "resize-none", "border");
        if self.debounce.is_some() {
            classes.extend(classes!(
                "border-yellow-500",
                "focus:border-yellow-500",
                "focus:ring-yellow-500",
            ));
        }
        let input_callback = self.link.callback(|_| Msg::Edited);
        html! {
            <form class="w-full h-full p-2" ref=self.form_ref.clone()>
                <label for="note_text" class="sr-only">{"Note text"}</label>
                <textarea
                    id="note_text"
                    class=classes
                    ref=self.textarea_ref.clone()
                    oninput=input_callback
                >
                    {self.props.text.clone()}
                </textarea>
            </form>
        }
    }
}
