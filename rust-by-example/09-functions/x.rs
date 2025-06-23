#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new_eq(v: f64) -> Point {
        Point { x: v, y: v }
    }

    fn ux(&self) -> f64 {
        self.x.abs()
    }
}


fn main() {
    let point = Point { x: -3.0, y: 4.0 };
    let point2 = Point::origin();
    let point3 = Point::new_eq(5.0);
    println!("Point {:?}", point);
    println!("Point {:?}", point2);
    println!("Point {:?}", point3);

    println!("Point ux is {:?}", point.ux());

    let outer_var = 42;
    // fn function(i: i32) -> i32 { i + outer_var }
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    let one = || 1;
    println!("closure returning one: {}", one());

    {
        let color = String::from("green");
        let print = || println!("`color`: {}", color);
        print();
        print();
        let _reborrow = &color;
        print();
        let _color_moved = color;
        // print();

        let mut count = 0;
        let mut inc = || {
            count += 1;
            println!("inc `count`: {}", count);
        };

        inc();
        // let _reborrow = &count;
        inc();
        let _count_reborrowed = &mut count;
        // inc();
    }
    {
        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
    }
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}