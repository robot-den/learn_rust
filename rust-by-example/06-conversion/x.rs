use std::fmt;
use std::convert::From;
use std::convert::Into;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Cake {
    name: i32
}

// impl From<i32> for Cake {
//     fn from(s: i32) -> Self {
//         Cake {name: s}
//     }
// }

impl Into<Cake> for i32 {
    fn into(self) -> Cake {
        Cake {name: self}
    }
}

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        match s.trim().parse() {
            Ok(radius) => Ok(Circle { radius }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let cake1 = Cake { name: 1 };
    println!("cake1 = {:?}", cake1);
    // println!("one more cake = {:?}", Cake::from(2));

    let cake3: Cake = 3.into();
    println!("cake3 = {:?}", cake3);

    println!("Circle: {}", Circle{radius: 3});
    println!("{}",Circle{radius: 5}.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let radius = "    3 ";
    let circle: Circle = radius.parse().unwrap();
    println!("{}", circle);
}