mod app;
pub mod components;
pub mod data;
pub mod layout;
pub mod types;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
