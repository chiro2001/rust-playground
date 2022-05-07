use std::fmt::Display;

#[warn(dead_code)]
#[warn(unused_variables)]

#[derive(Debug)]
pub struct Value<T: Display> {
  x: T
}

impl<T: Display> Value<T> {
    pub fn to_string(&self) -> String {
      format!("{}", self.x)
    }
}

impl<T: Display> Display for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

#[derive(Debug)]
pub struct Point<T: Display> {
  x: Value<T>,
  y: Value<T>
}

impl<T: Display> Point<T> {
  fn new(x: T, y: T) -> Point<T> {
    Point {
      x: Value { x },
      y: Value { x: y }
    }
  }

  pub fn to_string(&self) -> String {
    format!("Point({}, {})", self.x, self.y)
  }
}

fn main() {
  let point = Point {
    x: Value { x: 0 },
    y: Value { x: 1 }
  };
  dbg!(&point);
  println!("point = {:#?}", point);
  let point = Point::new(2, 3);
  dbg!(&point);
  println!("{}", point.to_string());
}
