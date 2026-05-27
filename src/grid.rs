use yew::prelude::*;

use crate::{game::Game, tile::Tile};

#[derive(Clone, PartialEq, Properties)]
pub struct GridProps {
    pub game: UseStateHandle<Game>
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    let props = props.clone();

    let change_tile_cb = {
        let props = props.clone();
        Callback::from(move |pos: (usize, usize)| {
            
            let tile = if (*props.game).clone().is_player1_turn {
                Tile::Player1
            } else {
                Tile::Player2
            };
            let game = (*props.game).clone();

            if let Some(new_game) = game.set_tile(pos, tile) {
                props.game.set(new_game);
            }
        })
    };

    let buttons = (0..9usize)
        .into_iter()
        .map(|i| {
            let cb = {
                let change_tile_cb = change_tile_cb.clone();
                Callback::from(move |_| change_tile_cb.emit((i / 3, i % 3)))
            };

            html! {
                <button class="square" onclick={cb}>{ props.game.get_tile((i / 3, i % 3)).to_string() }</button>
            }
        })
        .collect::<Html>();

    html! {
        <div id="board">
            {buttons}
        </div>
    }
}