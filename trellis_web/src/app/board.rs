use super::grid::Grid;
use super::{clock, note, weather};
use trellis_core::config;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub config: config::Config,
    pub onchange: Callback<(Uuid, config::Data)>,
    pub onedit: Callback<()>,
}

pub enum Msg {
    Change { id: Uuid, data: config::Data },
    Edit,
}

pub struct Board {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Board {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change { id, data } => {
                self.props.onchange.emit((id, data));
                false
            }
            Msg::Edit => {
                self.props.onedit.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let tiles = self.props.config.tiles.clone();
        let secrets = self.props.config.secrets.clone();

        let onclick = self.link.callback(|_| Msg::Edit);

        // TODO: I feel like the switch-mode button doesn't belong inside this component 🤔
        html! {
            <Grid>
                { tiles.iter().map(|t| self.render_tile(t.clone(), secrets.clone())).collect::<Html>() }
                <div class="flex flex-col items-center justify-around w-full h-full">
                    <button type="button" onclick=onclick>{ "Edit Settings" }</button>
                </div>
            </Grid>
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
