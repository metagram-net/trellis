use trellis_core::config;
use yew::prelude::*;

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub onsubmit: Callback<config::Data>,
}

pub struct AddTileForm {
    link: ComponentLink<Self>,
    props: Props,
    value: String,
}

pub enum Msg {
    Change(ChangeData),
    Submit(FocusEvent),
}

impl Component for AddTileForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            value: "".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(data) => {
                match data {
                    ChangeData::Select(elt) => {
                        self.value = elt.value();
                    }
                    _ => unreachable!(),
                }
                false
            }
            Msg::Submit(e) => {
                e.prevent_default();
                let res = match self.value.as_str() {
                    "clock" => Some(config::Data::Clock),
                    "note" => Some(config::Data::Note {
                        text: "".to_owned(),
                    }),
                    "weather" => Some(config::Data::Weather {
                        location_id: "".to_owned(),
                    }),
                    _ => None,
                };
                if let Some(data) = res {
                    self.props.onsubmit.emit(data);
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onchange = self.link.callback(Msg::Change);
        let onsubmit = self.link.callback(Msg::Submit);

        html! {
            <form class="flex flex-col items-center justify-around w-full h-full" onsubmit=onsubmit>
                <label>{"Tile type"}
                    <select onchange=onchange>
                        <option value="" disabled=true>{""}</option>
                        <option value="clock">{"Clock"}</option>
                        <option value="weather">{"Weather"}</option>
                        <option value="note">{"Note"}</option>
                    </select>
                </label>
                <button type="submit">{ "Add Tile" }</button>
            </form>
        }
    }
}
