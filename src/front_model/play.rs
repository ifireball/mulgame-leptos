use leptos::prelude::*;

use crate::model::Game;

pub struct PlayState {
    pub game: RwSignal<Game>,
    pub guesses: [RwSignal<[Option<u8>; 4]>; 10],
}


impl PlayState {
    pub fn new(game: Game) -> Self {
        Self { 
            game: RwSignal::new(game), 
            guesses: (0..10).map(|_| RwSignal::new([None; 4])).collect::<Vec<_>>().try_into().unwrap() 
        }
    }
}
