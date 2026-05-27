use crate::app::App;

mod game_state;
mod game;
mod tile;
mod app;
mod grid;

fn main() {
    yew::Renderer::<App>::new().render();
}
