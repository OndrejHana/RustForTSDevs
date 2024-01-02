pub struct PointIter {
    pub points: Vec<(f64, f64)>,
    pub idx: usize,
}

impl Iterator for PointIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.points.len() {
            return None;
        }
        let point = self.points[self.idx];
        self.idx += 1;

        return Some(point);
    }
}

impl From<Vec<(f64,f64)>> for PointIter {
    fn from(points: Vec<(f64,f64)>) -> Self {
        return PointIter {
            points,
            idx: 0,
        };
    }
}



pub trait Points {
    fn points(&self) -> PointIter;
}

pub trait Contains {
    fn contains(&self, point: (f64,f64)) -> bool;
}



pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;
    fn collides(&self, others: &[T]) -> bool {
        for other in others {
            if self.collide(other) {
                return true;
            }
        }

        return false;
    }
}

impl<T, V> Collidable<T> for V
where T: Points,
      V: Contains,
{
    fn collide(&self, other: &T) -> bool {
        let points = other.points();
        for point in points {
            if self.contains(point) {
                return true;
            }
        }

        return false;
    }
}

