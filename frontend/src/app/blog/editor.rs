use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub slug: String
}

#[function_component]
pub fn Editor(Props { slug }: &Props) -> Html {
    html! {

    }
}
