use std::f64::consts::PI;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::shapes::area::Area;

use super::collisions::{Points, Contains};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Default for Circle {
    fn default() -> Self {
        return Circle {
            x: 0.,
            y: 0.,
            radius: 10.,
        };
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle({}, {}): {}", self.x, self.y, self.radius);
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Points for Circle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![(self.x,self.y)].into();
    }
}

impl Contains for Circle {
    fn contains(&self, (x,y): (f64,f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();

        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Bad Circle!"));
        }

        return Ok(Circle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            radius: parts[2].parse()?,
        });
    }
}
