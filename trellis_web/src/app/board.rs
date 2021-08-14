use super::{clock, note, weather};
use trellis_core::config;
use yew::prelude::*;

pub struct Board {
    props: Props,
    #[allow(dead_code)]
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub settings: config::Config,
}

impl Component for Board {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let tiles = self.props.settings.tiles.clone();
        let secrets = self.props.settings.secrets.clone();

        html! {
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                { tiles.iter().map(|t| render_tile(t, secrets.clone())).collect::<Html>() }
            </div>
        }
    }
}

fn render_tile(tile: &config::Tile, secrets: config::Secrets) -> Html {
    // TODO: Wrap all of these in tile/grid-cell boilerplate
    match &tile.data {
        config::Data::Clock => html! { <clock::Clock id=tile.id /> },
        config::Data::Weather { location_id } => {
            html! {
                <weather::Weather
                    id=tile.id
                    location_id=location_id.clone()
                    owm_api_key=secrets.owm_api_key.unwrap_or("".to_owned())
                />
            }
        }
        config::Data::Note { text } => html! {
            <note::Note id=tile.id text=text.clone() />
        },
    }
}
