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
    MoveForward(Uuid),
    MoveBackward(Uuid),
    SetHeight(Uuid, u32),
    SetWidth(Uuid, u32),
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
            Msg::MoveBackward(id) => {
                // TODO: Make this a method on Config
                let idx = self.staged.tiles.iter().position(|tile| tile.id == id);
                match idx {
                    None => false,
                    // Already at the beginning
                    Some(0) => false,
                    Some(i) => {
                        let tile = self.staged.tiles.remove(i);
                        self.staged.tiles.insert(i - 1, tile);
                        true
                    }
                }
            }
            Msg::MoveForward(id) => {
                // TODO: Make this a method on Config
                let idx = self.staged.tiles.iter().position(|tile| tile.id == id);
                match idx {
                    None => false,
                    // Already at the end
                    Some(i) if i == self.staged.tiles.len() - 1 => false,
                    Some(i) => {
                        let tile = self.staged.tiles.remove(i);
                        self.staged.tiles.insert(i + 1, tile);
                        true
                    }
                }
            }
            Msg::SetHeight(id, height) => {
                if height < 1 {
                    return false;
                }
                // TODO: Make this a method on Config
                for tile in self.staged.tiles.iter_mut() {
                    if tile.id == id {
                        tile.height = Some(height);
                    }
                }
                true
            }
            Msg::SetWidth(id, width) => {
                if width < 1 {
                    return false;
                }
                // TODO: Make this a method on Config
                for tile in self.staged.tiles.iter_mut() {
                    if tile.id == id {
                        tile.width = Some(width);
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
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-1 auto-rows-fr">
                    { tiles.iter().map(|t| self.render_tile(t.clone())).collect::<Html>() }
                    <div class="flex flex-col items-center justify-around w-full h-full">
                        <span class="font-bold">{"New Tile"}</span>
                        <AddTileForm onsubmit=add_tile />
                    </div>
                </div>
                <div class="w-4/5 mx-auto my-2 border-t border-gray-200 dark:border-gray-700"></div>
                <SecretsForm secrets=self.staged.secrets.clone() onchange=onchange />
                <div class="w-4/5 mx-auto my-2 border-t border-gray-200 dark:border-gray-700"></div>
                {errors}
                <form class="w-full text-center" onsubmit=onsubmit>
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
        let inner = match &tile.data {
            config::Data::Clock => html! { <p>{"(not configurable)"}</p> },
            config::Data::Note { text: _ } => html! { <p>{"(not configurable)"}</p> },
            config::Data::Weather { location_id } => {
                let onchange = self.link.callback(move |lid: String| Msg::ChangeSingle {
                    id,
                    data: config::Data::Weather { location_id: lid },
                });
                html! {
                    <weather::ConfigForm location_id=location_id.clone() onchange=onchange />
                }
            }
        };
        let title = match &tile.data {
            config::Data::Clock => "Clock",
            config::Data::Note { text: _ } => "Note",
            config::Data::Weather { location_id: _ } => "Weather",
        };

        let height = tile.height.unwrap_or(1);
        let width = tile.width.unwrap_or(1);

        let backward = self.link.callback(move |_| Msg::MoveBackward(id));
        let forward = self.link.callback(move |_| Msg::MoveForward(id));
        let height_minus = self.link.callback(move |_| Msg::SetHeight(id, height - 1));
        let height_plus = self.link.callback(move |_| Msg::SetHeight(id, height + 1));
        let width_minus = self.link.callback(move |_| Msg::SetWidth(id, width - 1));
        let width_plus = self.link.callback(move |_| Msg::SetWidth(id, width + 1));

        let style = format!(
            "grid-row: auto / span {}; grid-column: auto / span {}",
            height, width
        );
        html! {
            <div class="flex flex-col items-center justify-around w-full h-full" style=style>
                <div class="w-full flex justify-between">
                    <span class="font-bold self-start">{title}</span>
                    <div class="flex space-x-1">
                        <div class="flex">
                            <button type="button" onclick=backward>{"←"}</button>
                            <button type="button" onclick=forward>{"→"}</button>
                        </div>
                        <div class="flex">
                            <button type="button" onclick=height_minus>{"H-"}</button>
                            <span>{height}</span>
                            <button type="button" onclick=height_plus>{"H+"}</button>
                            <button type="button" onclick=width_minus>{"W-"}</button>
                            <span>{width}</span>
                            <button type="button" onclick=width_plus>{"W+"}</button>
                        </div>
                    </div>
                </div>
                {inner}
                <button onclick=delete_tile>{"Delete Tile"}</button>
            </div>
        }
    }
}
