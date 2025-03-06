use crate::router::{switch_main, MainRoute};
use yew_router::Switch;
use yew_router::BrowserRouter;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}
