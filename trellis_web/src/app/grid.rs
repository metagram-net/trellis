use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

pub struct Grid {
    props: Props,
}

impl Component for Grid {
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
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-1 auto-rows-fr">
                { for self.props.children.iter() }
            </div>
        }
    }
}
