use anyhow;
use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::interval::{IntervalService, IntervalTask};

lazy_static! {
    static ref OWM_URL: Url =
        Url::parse("https://api.openweathermap.org").expect("could not parse URL");
}

pub struct Weather {
    props: Props,
    link: ComponentLink<Self>,
    location: Location,
    #[allow(dead_code)]
    ticker: IntervalTask,
    fetch_task: Option<FetchTask>,
    last_updated_at: Option<DateTime<Local>>,
    error: Option<anyhow::Error>,
}

#[derive(Serialize, Deserialize, Properties, Clone, Debug)]
pub struct Props {
    pub location_id: String,
    pub owm_api_key: String,
}

pub enum Msg {
    Fetch,
    Receive(Result<FetchResponse, anyhow::Error>),
}

#[derive(Clone)]
struct Location {
    id: String,
    name: String,
    icon: String,
    description: String,
    temperature: String,
}

impl Location {
    fn empty() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            icon: String::new(),
            description: String::new(),
            temperature: String::new(),
        }
    }
}

impl Component for Weather {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Immediately trigger the first update
        link.send_message(Msg::Fetch);

        Self {
            props,
            link: link.clone(),
            location: Location::empty(),
            ticker: IntervalService::spawn(Duration::from_secs(60), link.callback(|_| Msg::Fetch)),
            fetch_task: None,
            last_updated_at: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Fetch => {
                let mut url = OWM_URL.clone();
                url.set_path("data/2.5/weather");
                url.query_pairs_mut()
                    .append_pair("id", &self.props.location_id)
                    .append_pair("appid", &self.props.owm_api_key)
                    .append_pair("units", "imperial");
                let request = Request::get(url.as_str())
                    .body(Nothing)
                    .expect("could not build request");

                let callback = self.link.callback(
                    |response: Response<Json<Result<FetchResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Receive(data)
                    },
                );

                let task = FetchService::fetch(request, callback).expect("could not start request");
                self.fetch_task = Some(task);
                false
            }
            Msg::Receive(res) => {
                match res {
                    Ok(data) => {
                        // The weather comes back as an array.  This is probably one item per
                        // weather station, so taking the first one should be fine for now.
                        //
                        // TODO: Figure out why this is a list.
                        let w = &data.weather[0];
                        let mut icon_url = OWM_URL.clone();
                        icon_url.set_path(&format!("img/w/{}.png", w.icon));

                        self.location = Location {
                            id: format!("{}", data.id),
                            name: data.name,
                            temperature: format!("{:.0}℉", data.main.temp),
                            description: w.description.clone(),
                            icon: icon_url.to_string(),
                        };
                        self.error = None;
                    }
                    Err(error) => {
                        self.error = Some(error);
                        self.location = Location::empty();
                    }
                }
                self.last_updated_at = Some(Local::now());
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.last_updated_at {
            None => self.view_loading(),
            Some(_) => self.view_weather(),
        }
    }
}

impl Weather {
    fn view_loading(&self) -> Html {
        html! {
            <div class="flex flex-col items-center justify-around w-full h-full">
                <p>{"Loading..."}</p>
            </div>
        }
    }

    fn view_weather(&self) -> Html {
        let last_updated = self
            .last_updated_at
            .map_or(String::from("never"), |t| t.format("%H:%M:%S").to_string());

        let owm_url = {
            let mut url = OWM_URL.clone();
            url.path_segments_mut()
                .expect("cannot be base")
                .extend(&["city", &self.props.location_id]);
            url.to_string()
        };

        let location = self.location.clone();
        let name = location.name;
        let icon = location.icon;
        let description = location.description;
        let temperature = location.temperature;

        html! {
            <div class="flex flex-col items-center justify-around w-full h-full">
                <span class="text-3xl">{name}</span>
                <img src=icon alt=description.clone() />
                <span class="text-2xl">{description}</span>
                <div class="text-2xl">{temperature}</div>
                <div class="text-xl text-red-500">
                    {self.error.as_ref().map_or(String::new(), |e| e.to_string())}
                </div>
                <div class="flex items-center justify-around self-stretch text-gray-400">
                    <a href=owm_url>{"OpenWeatherMap"}</a>
                    <div>
                        {format!("Last updated at: {}", last_updated)}
                    </div>
                </div>
                <a href="#">{"Edit"}</a>
            </div>
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct FetchResponse {
    weather: Vec<WeatherResponse>,
    main: MainResponse,
    name: String,
    id: u64,
}

#[derive(Deserialize, Clone)]
struct WeatherResponse {
    description: String,
    icon: String,
}

#[derive(Deserialize, Clone)]
struct MainResponse {
    temp: f64,
}
