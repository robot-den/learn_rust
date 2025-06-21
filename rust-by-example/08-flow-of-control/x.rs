#![allow(unreachable_code, unused_labels)]

fn main() {
    let n = 15;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10.0 * n as f64
        } else {
            println!(", and is a big number, halve the number");

            n as f64 / 2.0
        };

    println!("{} -> {}", n, big_n);

    'outer: loop {
        println!("outer loop");

        'inner: loop {
            println!("inner loop");

            break 'outer;
        }

        println!("never reached");
    }

    let mut counter = 0;
    let _res1 = loop {
        counter += 1;
        println!("  loop count = {}", counter);

        if counter >= 3 {
            break counter;
        }
    };
    println!("counter = {}", counter);

    while counter < 5 {
        counter += 1;
        println!("  while counter = {}", counter);
    }
    println!("counter = {}", counter);
    let a = 1;
    let b = 3;

    for el in a..b {
        counter += el;
        println!("  for/in counter = {}", counter);
    }
    println!("counter = {}", counter);

    for el in a..=b {
        counter += el;
        println!("  for/=in counter = {}", counter);
    }
    println!("counter = {}", counter);

    // -- iterations

    {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        println!("names: {:?}", names);
    }

    {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        // NOTE: impossible without cloning
        // println!("names: {:?}", names);
    }

    {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
    }
    {
        let number = 55;

        match number {
            1 => println!("one"),
            2 | 3 => println!("two or three"),
            4..=5 => println!("four or five"),
            _ => println!("anything"),
        }

        let s = true;
        match s {
            true => println!("1"),
            false => println!("0"),
        }
    }

    {
        let triple = (0, 8, 8, 0);

        match triple {
            // Destructure the second and third elements
            (0, y, z, ..) if y == 8 && z == 8 => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
            (1, ..)  => println!("First is `1` and the rest doesn't matter"),
            (.., 2)  => println!("last is `2` and the rest doesn't matter"),
            (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
            // `..` can be used to ignore the rest of the tuple
            _      => println!("It doesn't matter what they are"),
            // `_` means don't bind the value to a variable
        }
    }

    {
        let reference = &4;

        match reference {
            &val => println!("Got a value via destructuring: {:?}", val),
        }

        match *reference {
            val => println!("Got a value via destructuring: {:?}", val),
        }

        let mut mut_value = 6;

        match mut_value {
            ref mut m => {
                *m += 10;
                println!("We added 10. `mut_value`: {:?}", m);
            },
        }
        println!("mut_value = {:?}", mut_value);
    }

    {
        match some_number() {
            Some(n @ 42, ..) => println!("The Answer: {}!", n),
            Some(n, ..)      => println!("Not interesting... {}", n),
            _            => (),
        }
    }
    {
        let c = Foo::Qux(100);

        // Binding also works with `if let`
        if let Foo::Qux(value@ 99..105) = c {
            println!("c matched");
        } else {
            println!("c not matched");
        }
    }
    {
        let a = Foof::Barr;

        // Variable a matches Foo::Bar
        if let Foof::Barr = a {
            // ^-- this causes a compile-time error. Use `if let` instead.
            println!("a is foobar");
        }
    }
}

// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foof::Barr == a fails below.
enum Foof {Barr}

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn some_number() -> Option<u32> {
    Some(42)
}