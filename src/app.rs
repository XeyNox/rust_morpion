use yew::prelude::*;

use crate::{game::Game, grid::Grid};


#[function_component(App)]
pub fn app() -> Html {

    let game = use_state(|| Game::new());
    let restart_cb = {
        let game = game.clone();
        Callback::from(move |_| game.set(Game::new()))
    }; 

    html! {
        <>
            <div id="title">
                <h1>{ "Tic Tac Toe" }</h1>
            </div>
            <p class="message">{ if (*game).is_player1_turn { "Joueur 1" } else { "Joueur 2"} }</p>
            <Grid game={ game.clone() }/>
            <p class="message">{ (*game).get_game_state().to_string() }</p>
            <button onclick={restart_cb}>{"rejouer"}</button>
        </>
    }
}