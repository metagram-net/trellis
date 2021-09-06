use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod add_tile_form;
mod board;
mod clock;
mod config_form;
mod grid;
mod note;
mod secrets_form;
mod settings;
mod settings_page;
mod weather;

pub struct App {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/settings"]
    Settings,
    #[to = "/"]
    Board,
}

type Anchor = RouterAnchor<AppRoute>;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="min-h-screen text-black bg-white dark:text-white dark:bg-black flex flex-col">
                <Router<AppRoute, ()> render=Router::render(move |route: AppRoute| {
                    let main = match route {
                        AppRoute::About => html! { <about::About /> },
                        AppRoute::Board => html! { <board::Board /> },
                        AppRoute::Settings => html! { <settings_page::SettingsPage /> },
                    };
                    html! { <>
                        <nav class="px-3 py-1 flex justify-between bg-gray-200 dark:bg-gray-900">
                            <Anchor route=AppRoute::Board>{"Trellis"}</Anchor>
                            <div class="space-x-4">
                                <Anchor route=AppRoute::About>{"About"}</Anchor>
                                <Anchor route=AppRoute::Settings>{"Settings"}</Anchor>
                                <a href="/login">{"Log in"}</a>
                            </div>
                        </nav>
                        <div class="flex-grow flex flex-col justify-between">
                            <main class="m-1">{main}</main>
                        </div>
                    </> }
                })
                />
            </div>
        }
    }
}
