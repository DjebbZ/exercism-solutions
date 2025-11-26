use std::ops::Deref;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    throws: Vec<u16>,
    is_second_throw: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            throws: Vec::new(),
            is_second_throw: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.is_second_throw && self.throws.last().unwrap() + pins > 10) {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.score().is_some() {
            Err(Error::GameComplete)
        } else {
            self.throws.push(pins);
            self.is_second_throw = if pins != 10 {
                !self.is_second_throw
            } else {
                false
            };
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut total_score: u16 = 0;
        let mut frame = 0;

        for _ in 0..10 {
            if let (Some(&first_throw), Some(&second_throw)) =
                (self.throws.get(frame), self.throws.get(frame + 1))
            {
                total_score += first_throw + second_throw;
                if first_throw == 10 || (first_throw + second_throw) == 10 {
                    if let Some(&third_throw) = self.throws.get(frame + 2) {
                        total_score += third_throw;
                    } else {
                        return None;
                    }
                }
                frame += if first_throw == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }

        Some(total_score)
    }
}
