use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod add_tile_form;
mod auth;
mod board;
mod clock;
mod config_form;
mod csrf;
mod grid;
mod login;
mod login_confirmation;
mod login_form;
mod logout_form;
mod note;
mod secrets_form;
mod settings;
mod settings_page;
mod weather;

pub struct App {
    user: Option<auth::User>,
    #[allow(dead_code)]
    auth_service: Box<dyn Bridge<auth::Auth>>,
}

pub enum Msg {
    Whoami(Option<auth::User>),
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/settings"]
    Settings,
    #[to = "/login"]
    Login,
    #[to = "/"]
    Board,
}

type Anchor = RouterAnchor<AppRoute>;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut auth_service = auth::Auth::bridge(link.callback(Msg::Whoami));
        auth_service.send(auth::Request::Whoami);

        Self {
            user: None,
            auth_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Whoami(user) => {
                self.user = user;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let logged_in = self.user.is_some();
        let log_in_out = move || {
            if logged_in {
                html! { <logout_form::LogoutForm /> }
            } else {
                html! { <Anchor route=AppRoute::Login>{"Log in"}</Anchor> }
            }
        };
        html! {
            <div class="min-h-screen text-black bg-white dark:text-white dark:bg-black flex flex-col">
                <Router<AppRoute, ()> render=Router::render(move |route: AppRoute| {
                    let main = match route {
                        AppRoute::About => html! { <about::About /> },
                        AppRoute::Board => html! { <board::Board /> },
                        AppRoute::Login => html! { <login::Login /> },
                        AppRoute::Settings => html! { <settings_page::SettingsPage /> },
                    };
                    html! { <>
                        <nav class="px-3 py-1 flex justify-between bg-gray-200 dark:bg-gray-900">
                            <Anchor route=AppRoute::Board>{"Trellis"}</Anchor>
                            <div class="space-x-4">
                                <Anchor route=AppRoute::About>{"About"}</Anchor>
                                <Anchor route=AppRoute::Settings>{"Settings"}</Anchor>
                                { log_in_out() }
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
