use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;

const GAME_NAV_TRANSITION_DURATION: u32 = 1500;

#[derive(Clone, Copy)]
pub enum GameNavPhase {
    Idle,
    TransitioningOut,
    TransitioningIn,
}

pub trait GameNavPhaseTrait {
    fn is_transitioning(&self) -> bool;
}

impl GameNavPhaseTrait for GameNavPhase {
    fn is_transitioning(&self) -> bool {
        matches!(self, GameNavPhase::TransitioningOut | GameNavPhase::TransitioningIn)
    }
}

impl Default for GameNavPhase {
    fn default() -> Self {
        GameNavPhase::Idle
    }
}

#[derive(Clone, Copy)]
pub struct GameNavState {
    pub phase: RwSignal<GameNavPhase>,
    pub current_board_idx: RwSignal<usize>,
    pub previous_board_idx: RwSignal<usize>,
}

impl GameNavPhaseTrait for GameNavState {
    fn is_transitioning(&self) -> bool {
        self.phase.with(|phase| phase.is_transitioning())
    }
}

impl GameNavState {
    pub fn new() -> Self {
        Self { 
            phase: RwSignal::new(GameNavPhase::Idle), 
            current_board_idx: RwSignal::new(0), 
            previous_board_idx: RwSignal::new(0),
        }
    }
    pub fn transition_to(&self, new_board_idx: usize) {
        if self.is_transitioning() {
            return;
        }
        self.phase.set(GameNavPhase::TransitioningOut);
        let phase = self.phase.clone();
        let current_board_idx = self.current_board_idx.clone();
        let previous_board_idx = self.previous_board_idx.clone();
        spawn_local(async move {
            previous_board_idx.set(current_board_idx.get());
            TimeoutFuture::new(GAME_NAV_TRANSITION_DURATION).await;
            current_board_idx.set(new_board_idx);
            phase.set(GameNavPhase::TransitioningIn);
            TimeoutFuture::new(GAME_NAV_TRANSITION_DURATION).await;
            phase.set(GameNavPhase::Idle);
        });
    }
}
