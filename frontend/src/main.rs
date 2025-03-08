pub mod app;
pub mod models;
pub mod components;
pub mod config;
pub mod shared;
pub mod services;
mod hooks;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

