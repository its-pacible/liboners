use deck::{draw_hand_infinite, OnersDeck};


pub mod card;
pub mod player;
pub mod game;
pub mod deck;
fn main() {
    println!("{:#?}",OnersDeck::new())
}