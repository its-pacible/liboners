use crate::{deck::*, player::*};

pub struct OnersGame {
    pub deck: OnersDeck,
    pub players: Vec<OnersPlayer>,
    pub id: String,
}