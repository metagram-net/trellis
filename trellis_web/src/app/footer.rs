use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub version: String,
    pub source_url: String,
}

pub struct Footer {
    props: Props,
}

impl Component for Footer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer class="flex justify-center space-x-8 text-gray-400 text-sm">
                <p>{self.props.version.clone()}</p>
                <p>{"Source code on "}<a href=self.props.source_url.clone()>{"GitHub"}</a></p>
            </footer>
        }
    }
}
