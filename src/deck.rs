use crate::{card::*, player::OnersPlayerHand};
use rand::{thread_rng, Rng};
//use strum_macros::FromRepr;

#[derive(Debug)]
pub struct OnersDeck {
    deck: Vec<OnersCard>,
    discard: Vec<OnersCard>,
}

impl OnersDeck {
    /// generates a new OnersDeck with one type of each valid card
    pub fn new() -> OnersDeck {
        let mut out = Vec::new();
        for color in 0..5 as u8 {
            for value in 0..15 as u8 {
                match new(
                    CardColor::from_repr(color).unwrap(),
                    CardValue::from_repr(value).unwrap(),
                ) {
                    Ok(c) => out.push(c),
                    Err(_) => {}
                }
            }
        }
        OnersDeck{deck: out,discard: vec![]}
    }
}

pub fn draw_infinite() -> OnersCard {
    // randomize the color
    match new(
        CardColor::from_repr(thread_rng().gen_range(0 as u8..5)).unwrap(),
        CardValue::from_repr(thread_rng().gen_range(0 as u8..12)).unwrap(),
    ) {
        Ok(c) => c,
        Err(e) => draw_infinite(),
    }
}
pub fn draw_hand_infinite() -> OnersPlayerHand {
    let mut hand = (0 as usize..7)
        .map(|c| draw_infinite())
        .collect::<Vec<OnersCard>>();
    hand
}
