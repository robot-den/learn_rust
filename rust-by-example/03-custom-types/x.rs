#![allow(dead_code)]

struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {top_left: tp, bottom_right: br} = rectangle;
    let width = br.x - tp.x;
    let height = tp.y - br.y;

    width * height
}

fn square(point: Point, width: f32) -> Rectangle {
    Rectangle {
        top_left: Point { x: point.x, y: point.y },
        bottom_right: Point {
            x: point.x + width,
            y: point.y + width,
        }
    }
}

enum FoodEvent {
    Cake,
    Butter,
    Salt(f32),
    Omelet { eggs: i32, tomato: i32, chorizo: bool}
}

type FE = FoodEvent;

fn check_food(event: FoodEvent) {
    match event {
        FE::Cake => {
            println!("Cake!");
        }
        FoodEvent::Butter => {
            println!("Butter!");
        }
        FoodEvent::Salt(weight) => {
            println!("Salt with weight {:?}gr", weight);
        }
        FoodEvent::Omelet { eggs, tomato, chorizo } => {
            println!("Omelet with eggs {:?} tomato {:?} and chorizo ({})", eggs, tomato, chorizo);
        }
    }
}

fn main() {
    let namee = "Peter".to_string();
    let age = 27;

    let _peter = Person { name: namee, age };

    let point: Point = Point { x: 5.0, y: 3.0 };
    let another_point: Point = Point { x: 7.0, y: 1.0 };

    let bottom_right = Point { x: 10.6, ..another_point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let rect = Rectangle {
        top_left: point,
        bottom_right: another_point
    };
    println!("rect area is {}", rect_area(rect));

    let square = square(Point {x: 2.0, y: 2.0}, 10.0);
    println!("square is {:?}", square);

    check_food(FoodEvent::Cake);
    check_food(FE::Salt(14.0));
    check_food(FoodEvent::Omelet {eggs: 1, tomato: 1, chorizo: false});
}