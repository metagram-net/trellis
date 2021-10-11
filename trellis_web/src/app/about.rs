use yew::prelude::*;

pub struct About {
    pub version: &'static str,
    pub source_url: &'static str,
}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            version: crate::VERSION,
            source_url: crate::SOURCE_URL,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // TODO(licenses): Credits and license attribution
        html! {
            <div class="mx-2 max-w-prose">
                <h1>{"Trellis"}</h1>
                <p class="text-xl">{"Grow your own homepage!"}</p>
                <p>{
                "Trellis is a web-based dashboard for collecting useful widgets.  If you want a browser homepage with data shared between devices, this could be for you!"
                }</p>
                <p>{"Build info: "}{self.version}</p>
                <p>{"Source code on "}<a href=self.source_url>{"GitHub"}</a></p>
            </div>
        }
    }
}
