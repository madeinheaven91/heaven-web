mod app;
pub mod models;
pub mod router;
pub mod components;
pub mod views;
pub mod config;
pub mod shared;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
