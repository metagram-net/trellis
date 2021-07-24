use serde_json;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

mod clock;
mod settings;
mod weather;

pub struct App {
    link: ComponentLink<Self>,
    settings: settings::Settings,
    editing: bool,
    textarea_ref: NodeRef,
}

pub enum Msg {
    EditSettings,
    SaveSettings,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            settings: settings::Settings::load(),
            editing: false,
            textarea_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EditSettings => {
                self.textarea_ref = NodeRef::default();
                self.editing = true;
                true
            }
            Msg::SaveSettings => {
                let text = self
                    .textarea_ref
                    .cast::<HtmlTextAreaElement>()
                    .unwrap()
                    .value();
                if let Ok(settings) = serde_json::from_str::<settings::Settings>(&text) {
                    if let Ok(_) = settings.save() {
                        self.settings = settings;
                        self.editing = false;
                        return true;
                    }
                }
                self.editing = true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.editing {
            self.view_settings()
        } else {
            self.view_board()
        }
    }
}

impl App {
    fn view_settings(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::SaveSettings);
        let text =
            serde_json::to_string_pretty(&self.settings).expect("Could not serialize settings");
        let rows = format!(
            "{}",
            // Add an extra row because the last line doesn't have a newline.
            text.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1
        );

        html! {
            <form>
                <textarea class="w-full" rows=rows ref=self.textarea_ref.clone()>
                    {text}
                </textarea>
                <button onclick=onclick class="btn btn-gray">{ "Save" }</button>
            </form>
        }
    }

    fn view_board(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::EditSettings);
        html! {
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                <clock::Clock />
                <weather::Weather with self.settings.weather.clone() />
                <button onclick=onclick class="btn btn-gray">{ "Settings" }</button>
            </div>
        }
    }
}
