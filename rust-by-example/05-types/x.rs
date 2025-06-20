type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn print_type(e: u64) {
    match e {
        u64 => println!("type is u64"),
        NanoSecond => println!("type is Nanosecond"),
    }
}

fn main() {
    let decimal = 65.4321_f32;

    let integer = decimal as u8;

    let character = integer as char;

    println!("d: {:?}, i: {:?}, c: {:?}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);

    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));


    let mut vec = Vec::new();
    vec.push(x);
    println!("vec: {:?}", vec);

    let u: u64 = 10;
    let nanosecond: NanoSecond = 5 as u64;

    print_type(u);
    print_type(nanosecond);
}