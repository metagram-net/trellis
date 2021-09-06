use super::grid::Grid;
use super::settings;
use super::{clock, note, weather};
use trellis_core::config;
use uuid::Uuid;
use yew::prelude::*;

pub struct Board {
    link: ComponentLink<Self>,
    settings: Option<config::Config>,
    settings_service: Box<dyn Bridge<settings::Settings>>,
}

pub enum Msg {
    Load(config::Config),
    Change { id: Uuid, data: config::Data },
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut settings_service = settings::Settings::bridge(link.callback(Msg::Load));
        settings_service.send(settings::Request::Load);
        Self {
            link,
            settings: None,
            settings_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Load(cfg) => {
                self.settings = Some(cfg);
                true
            }
            Msg::Change { id, data } => {
                self.settings_service
                    .send(settings::Request::SaveSingle { id, data });
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.settings {
            None => html! { <p class="text-xl">{"Loading..."}</p> },
            Some(cfg) => {
                let tiles = cfg.tiles.clone();
                let secrets = cfg.secrets.clone();
                html! {
                    <Grid>
                        { tiles.iter().map(|t| self.render_tile(t.clone(), secrets.clone())).collect::<Html>() }
                    </Grid>
                }
            }
        }
    }
}

impl Board {
    // TODO: Tile component
    fn render_tile(&self, tile: config::Tile, secrets: config::Secrets) -> Html {
        let id = tile.id.clone();
        let inner = match &tile.data {
            config::Data::Clock => html! { <clock::Clock /> },
            config::Data::Weather { location_id } => {
                html! {
                    <weather::Weather
                        location_id=location_id.clone()
                        owm_api_key=secrets.owm_api_key.clone().unwrap_or("".to_owned())
                    />
                }
            }
            config::Data::Note { text } => html! {
                <note::Note
                    text=text.clone()
                    onchange=self.link.callback(move |text| {
                        Msg::Change{id, data: config::Data::Note{ text }}
                    })
                />
            },
        };

        let height = tile.height.unwrap_or(1);
        let width = tile.width.unwrap_or(1);
        let style = format!(
            "grid-row: auto / span {}; grid-column: auto / span {}",
            height, width
        );
        html! {
            <div style=style>{inner}</div>
        }
    }
}
