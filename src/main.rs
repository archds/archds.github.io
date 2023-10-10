mod app;
pub mod components;
pub mod layout;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
