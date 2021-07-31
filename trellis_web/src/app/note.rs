use std::time::Duration;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::storage::{Area, StorageService};
use yew::services::timeout::{TimeoutService, TimeoutTask};

pub struct Note {
    link: ComponentLink<Self>,
    form_ref: NodeRef,
    textarea_ref: NodeRef,
    debounce: Option<TimeoutTask>,
    text: String,
}

pub enum Msg {
    Edited,
    Saved,
}

impl Note {
    const KEY: &'static str = "trellis.note";
}

impl Component for Note {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let text = StorageService::new(Area::Local)
            .ok()
            .and_then(|storage| storage.restore::<anyhow::Result<String>>(Self::KEY).ok())
            .unwrap_or_default();
        Self {
            link: link.clone(),
            textarea_ref: NodeRef::default(),
            form_ref: NodeRef::default(),
            debounce: None,
            text,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Edited => {
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
                let save = StorageService::new(Area::Local)
                    .map(|mut storage| storage.store(Self::KEY, Ok(text)));
                if let Err(err) = save {
                    ConsoleService::error(err);
                };
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
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
                >{self.text.clone()}</textarea>
            </form>
        }
    }
}
