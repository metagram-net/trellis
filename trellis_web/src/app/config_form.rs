use super::add_tile_form::AddTileForm;
use super::secrets_form::SecretsForm;
use super::weather;
use trellis_core::config;
use uuid::Uuid;
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
    AddTile(config::Data),
    DeleteTile(Uuid),
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
            Msg::AddTile(data) => {
                let tile = config::Tile {
                    id: Uuid::new_v4(),
                    row: None,
                    col: None,
                    width: None,
                    height: None,
                    data,
                };
                self.staged.tiles.push(tile);
                true
            }
            Msg::DeleteTile(id) => {
                // TODO: Make this a method on Config
                let mut new_tiles: Vec<config::Tile> = Vec::new();
                for tile in self.staged.tiles.iter() {
                    if tile.id != id {
                        new_tiles.push(tile.clone());
                    }
                }
                self.staged.tiles = new_tiles;
                true
            }
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
        let add_tile = self.link.callback(Self::Message::AddTile);

        html! {
            <>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 space-x-1 space-y-1">
                    { tiles.iter().map(|t| self.render_tile(t.clone())).collect::<Html>() }
                    <AddTileForm onsubmit=add_tile />
                </div>
                <SecretsForm secrets=self.staged.secrets.clone() onchange=onchange />
                {errors}
                <form class="w-full text-center mt-4" onsubmit=onsubmit>
                    <button type="submit">{ "Done" }</button>
                </form>
            </>
        }
    }
}

impl ConfigForm {
    fn render_tile(&self, tile: config::Tile) -> Html {
        let id = tile.id.clone();
        let delete_tile = self.link.callback(move |_| Msg::DeleteTile(id));
        match &tile.data {
            config::Data::Clock => html! {
                <div class="flex flex-col items-center justify-around w-full h-full">
                    <p>{"Clock (not configurable)"}</p>
                    <button onclick=delete_tile>{"Delete Tile"}</button>
                </div>
            },
            config::Data::Note { text: _ } => html! {
                <div class="flex flex-col items-center justify-around w-full h-full">
                    <p>{"Note (not configurable)"}</p>
                    <button onclick=delete_tile>{"Delete Tile"}</button>
                </div>
            },
            config::Data::Weather { location_id } => {
                let onchange = self.link.callback(move |lid: String| Msg::ChangeSingle {
                    id,
                    data: config::Data::Weather { location_id: lid },
                });
                html! {
                    <div class="flex flex-col items-center justify-around w-full h-full">
                        <weather::ConfigForm location_id=location_id.clone() onchange=onchange />
                        <button onclick=delete_tile>{"Delete Tile"}</button>
                    </div>
                }
            }
        }
    }
}
