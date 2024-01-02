use anyhow::anyhow;

use crate::shapes::area::Area;
use std::{fmt::{Display, Formatter}, str::FromStr};

use super::{collisions::{PointIter, Points, Contains}, circle::Circle};

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for Rect {
    fn default() -> Self {
        return Rect {
            x: 0.,
            y: 0.,
            width: 10.,
            height: 10.,
        };
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({}, {}): {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Points for Rect {
    fn points(&self) -> PointIter {
        return vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height)
        ].into();
    }
}

impl Contains for Rect {
    fn contains(&self, (x,y): (f64,f64)) -> bool {
        return self.x <= x && self.x + self.width >= x && self.y <= y && self.y + self.width >= y;
    }
}

impl FromStr for Rect {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();

        if parts.len() != 4 {
            return Err(anyhow!("Bad rectangle!"));
        }

        return Ok(Rect {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        });
    }
}

