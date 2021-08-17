use super::weather;
use trellis_core::config;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub config: config::Config,
    pub onsubmit: Callback<config::Config>,
}

pub struct ConfigForm {
    link: ComponentLink<Self>,
    props: Props,
    staged: config::Config,
    error: Option<String>,
}

pub enum Msg {
    ChangeSingle { id: Uuid, data: config::Data },
    ChangeSecrets(config::Secrets),
    Save,
}

impl Component for ConfigForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: props.clone(),
            staged: props.config.clone(),
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeSingle { id, data } => {
                // TODO: Make this a method on Config
                let mut new_tiles: Vec<config::Tile> = Vec::new();
                for tile in self.staged.tiles.iter() {
                    if tile.id == id {
                        new_tiles.push(config::Tile {
                            id: tile.id,
                            row: tile.row,
                            col: tile.col,
                            width: tile.width,
                            height: tile.height,
                            data: data.clone(),
                        })
                    } else {
                        new_tiles.push(tile.clone());
                    }
                }
                self.staged.tiles = new_tiles;
                true
            }
            Msg::ChangeSecrets(secrets) => {
                self.staged.secrets = secrets;
                true
            }
            Msg::Save => {
                self.props.onsubmit.emit(self.staged.clone());
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let tiles = self.staged.tiles.clone();
        let errors = match &self.error {
            None => html! {},
            Some(msg) => html! {
                <div class="alert">
                    <p>{msg}</p>
                </div>
            },
        };

        let onchange = self.link.callback(Self::Message::ChangeSecrets);
        let onsubmit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Self::Message::Save
        });

        html! {
            <>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 space-x-1 space-y-1">
                    { tiles.iter().map(|t| self.render_tile(t.clone())).collect::<Html>() }
                    <SecretsForm secrets=self.staged.secrets.clone() onchange=onchange />
                </div>
                {errors}
                <form class="w-full text-center mt-4" onsubmit=onsubmit>
                    <button type="submit" class="btn btn-gray">{ "Done" }</button>
                </form>
            </>
        }
    }
}

impl ConfigForm {
    fn render_tile(&self, tile: config::Tile) -> Html {
        let id = tile.id.clone();
        match &tile.data {
            config::Data::Clock => html! {
                <div class="flex flex-col items-center justify-around w-full h-full">
                    <p>{"Clock (not configurable)"}</p>
                </div>
            },
            config::Data::Note { text: _ } => html! {
                <div class="flex flex-col items-center justify-around w-full h-full">
                    <p>{"Note (not configurable)"}</p>
                </div>
            },
            config::Data::Weather { location_id } => {
                let onchange = self.link.callback(move |lid: String| Msg::ChangeSingle {
                    id,
                    data: config::Data::Weather { location_id: lid },
                });
                html! { <weather::ConfigForm location_id=location_id.clone() onchange=onchange /> }
            }
        }
    }
}

struct SecretsForm {
    link: ComponentLink<Self>,
    props: SecretsFormProps,
    owm_api_key_ref: NodeRef,
}

#[derive(Properties, Clone, Debug)]
pub struct SecretsFormProps {
    pub secrets: config::Secrets,
    pub onchange: Callback<config::Secrets>,
}

enum SecretsFormMsg {
    Input,
    Submit,
}

impl Component for SecretsForm {
    type Message = SecretsFormMsg;
    type Properties = SecretsFormProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            owm_api_key_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Submit => {
                self.props.onchange.emit(self.props.secrets.clone());
                false
            }
            Self::Message::Input => {
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
            SecretsFormMsg::Submit
        });

        let oninput = self.link.callback(|_: InputData| Self::Message::Input);

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
