use crate::app::App;

mod game_state;
mod game;
mod tile;
mod app;

fn main() {
    yew::Renderer::<App>::new().render();
}
