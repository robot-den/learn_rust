use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (index, v) in vec.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", index, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    println!("Hello, world from Rust!");
    let x = 5 + 90 + 5;
    println!("The value of `x` is {}", x);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("My name is {0}, {1} {0}", "Bond", "James");
    println!("{number:0>5}", number="xx");

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:z>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    println!("{:?} months in a year.", 12);

    println!("Debug Structure: {:?}", Structure(3));
    println!("Display Structure: {}", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("{:?}", point);
    println!("{}", point);

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Pretty: {:#?}", complex);
    println!("Debug: {:?}", complex);
    println!("Display: {}", complex);

    let list = List(vec![1, 2, 3]);
    println!("Display: {}", list);
}