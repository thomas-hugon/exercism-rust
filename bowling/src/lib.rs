use crate::Error::{GameComplete, NotEnoughPinsLeft};
use crate::Frame::{OpenFrame, Spare, Strike};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
#[derive(Debug, PartialEq)]
pub enum Frame {
    OpenFrame(u16, u16),
    Spare(u16),
    Strike,
}

#[derive(Default)]
pub struct BowlingGame {
    current_frame: (Option<u16>, Option<u16>),
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match (self.frames.len(), self.frames.last(), self.current_frame) {
            (10, Some(OpenFrame(_, _)), _) => return Err(GameComplete),
            (10, Some(Spare(_)), (Some(_), _)) => return Err(GameComplete),
            (10, Some(Strike), (Some(_), Some(_))) => return Err(GameComplete),
            (_, _, _) if pins > 10 => return Err(NotEnoughPinsLeft),
            (10, Some(Strike), (Some(10), None)) => self.current_frame = (Some(10), Some(pins)),
            (_, _, (Some(a), None)) if a + pins > 10 => return Err(NotEnoughPinsLeft),
            (_, _, (Some(a), None)) => self.current_frame = (Some(a), Some(pins)),
            (_, _, (None, None)) => self.current_frame = (Some(pins), None),
            _ => panic!(),
        }

        if self.frames.len() < 10 {
            match self.current_frame {
                (Some(10), _) => self.push(Strike),
                (Some(a), Some(b)) if a + b == 10 => self.push(Spare(a)),
                (Some(a), Some(b)) => self.push(OpenFrame(a, b)),
                _ => (),
            }
        }
        Ok(())
    }

    fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
        self.current_frame = (None, None);
    }

    pub fn score(&self) -> Option<u16> {
        match (self.frames.len(), self.frames.last(), self.current_frame) {
            (0..=9, _, _) => None,
            (_, Some(Strike), (_, None)) => None,
            (_, Some(Spare(_)), (None, _)) => None,
            _ => Some(self.score_frames() + self.score_fill()),
        }
    }

    fn score_fill(&self) -> u16 {
        let fill = match self.current_frame {
            (Some(a), None) => a,
            (Some(a), Some(b)) => a + b,
            _ => 0,
        };
        if let Some(Strike) = self.frames.iter().rev().nth(1) {
            self.current_frame.0.map_or(fill, |x| x + fill)
        } else {
            fill
        }
    }

    fn score_frames(&self) -> u16 {
        let tmp: Vec<&Frame> = self
            .frames
            .iter()
            .chain([OpenFrame(0, 0), OpenFrame(0, 0)].iter())
            .collect();

        tmp.windows(3).fold(0, |score, w| {
            score
                + match (&w[0], &w[1], &w[2]) {
                    (Strike, Strike, Strike) => 30,
                    (Strike, Strike, Spare(a)) => 20 + a,
                    (Strike, Strike, OpenFrame(a, _)) => 20 + a,
                    (Strike, Spare(_), _) => 20,
                    (Strike, OpenFrame(a, b), _) => 10 + a + b,
                    (Spare(_), Strike, _) => 20,
                    (Spare(_), Spare(a), _) => 10 + a,
                    (Spare(_), OpenFrame(a, _), _) => 10 + a,
                    (OpenFrame(a, b), _, _) => a + b,
                }
        })
    }
}
