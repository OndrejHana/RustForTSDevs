use anyhow::Result;
use std::{fs::read_to_string, str::FromStr, fmt::Display, };

use crate::shapes::{rect::Rect, circle::Circle, collisions::{Collidable, Points, Contains}};

enum ShapeType {
    Rect(Rect),
    Circle(Circle),
}

impl Display for ShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            ShapeType::Rect(r) => write!(f, "{}", r),
            ShapeType::Circle(c) => write!(f, "{}", c),
        }
    }
}

impl FromStr for ShapeType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        return match shape {
            "rect" => Ok(ShapeType::Rect(Rect::from_str(data)?)),
            "circle" => Ok(ShapeType::Circle(Circle::from_str(data)?)),
            _ => Err(anyhow::anyhow!("Bad shape!")), 
        };
    }
}

impl Points for ShapeType {
    fn points(&self) -> crate::shapes::collisions::PointIter {
        return match self {
            ShapeType::Rect(r) => r.points(),
            ShapeType::Circle(c) => c.points(),
        };
    }
}

impl Contains for ShapeType {
    fn contains(&self, point: (f64,f64)) -> bool {
        return match self {
            ShapeType::Rect(r) => r.contains(point),
            ShapeType::Circle(c) => c.contains(point),
        };
    }
}

impl Points for &ShapeType {
    fn points(&self) -> crate::shapes::collisions::PointIter {
        return match self {
            ShapeType::Rect(r) => r.points(),
            ShapeType::Circle(c) => c.points(),
        };
    }
}

impl Contains for &ShapeType {
    fn contains(&self, point: (f64,f64)) -> bool {
        return match self {
            ShapeType::Rect(r) => r.contains(point),
            ShapeType::Circle(c) => c.contains(point),
        };
    }
}

pub fn read_file(path: String) -> Result<()> {
    /*
    let file_content = read_to_string(path)?;
    let mut shapes: Vec<ShapeType> = Vec::new();
    for line in file_content.lines() {
        shapes.push(line.parse()?);
    }
    */

    let shapes: Vec<ShapeType> = read_to_string(path)?
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect();

    shapes
        .iter()
        .skip(1)
        .zip(shapes
            .iter()
            .take(shapes.len() - 1))
        .filter(|(a,b)| a.collide(b))
        .for_each(|x| println!("{}, {}", x.0, x.1));


    return Ok(());
}
