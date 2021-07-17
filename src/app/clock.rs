use chrono::{offset, DateTime, Local};
use std::time::Duration;
use yew::prelude::*;
use yew::services::interval::{IntervalService, IntervalTask};

pub struct Clock {
    time: DateTime<Local>,
    #[allow(dead_code)]
    ticker: IntervalTask,
}

impl Component for Clock {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            time: Local::now(),
            ticker: IntervalService::spawn(Duration::from_millis(1000), link.callback(|_| ())),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        self.time = Local::now();
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex flex-col items-center justify-around w-full h-full">
                <div class="text-3xl">{self.time.format("%A, %B %-d, %Y")}</div>
                <div class="text-7xl">{self.time.format("%H:%M:%S")}</div>
                <div class="flex items-center justify-around self-stretch text-gray-400">
                    <div data-clock-target="iso">{self.time.with_timezone(&offset::Utc).format("%Y-%m-%dT%H:%M:%S")}</div>
                    <div data-clock-target="unix">{self.time.format("%s")}</div>
                </div>
            </div>
        }
    }
}
