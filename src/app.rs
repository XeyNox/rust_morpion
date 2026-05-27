use yew::prelude::*;
use crate::tile;
use crate::tile::Tile;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>{ "Game" }</h1>
            <p> {" "}</p>
            <Grid />
            <p> {" "}</p>
        </>
    }
}