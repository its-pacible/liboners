use rand::{distributions::*, prelude::*};
use strum_macros::FromRepr;
#[derive(Debug,Eq,PartialEq)]
pub enum OnersError {
    IllegalCardError,
    IllegalValueError,
    IllegalColorError,
}

#[derive(FromRepr,Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum CardColor {
    Wild,
    Red,
    Green,
    Blue,
    Yellow,
}
#[derive(FromRepr,Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum CardValue {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    DrawTwo,
    Skip,
    Reverse,
    Empty,
    DrawFour,
}

#[derive(Debug)]
pub struct OnersCard {
    color: CardColor,
    value: CardValue,
}
/// create a new Oners Card with a given color and value.
pub fn new(color: CardColor, value: CardValue) -> Result<OnersCard, OnersError> {
    let out = OnersCard {
        color: color,
        value: value,
    };
    match out.value {
        CardValue::DrawFour | CardValue::Empty => {
            if !(out.color == CardColor::Wild) {
                Err(OnersError::IllegalCardError)
            } else {
                Ok(out)
            }
        }
        _ => {
            if out.color == CardColor::Wild {
                Err(OnersError::IllegalCardError)
            } else {
                Ok(out)
            }
        }
    }
}
