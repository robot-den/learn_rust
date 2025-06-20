fn main() {
    let unm = 2;
    let mut m = 8;

    println!("m = {}", m);
    m = 12;
    println!("m = {}", m);

    {
        let x = 5;
        println!("x = {}", x);

        println!("unm original = {}", unm);
        let unm = "shadow";
        println!("unm = {}", unm);
    }
    println!("unm outside = {}", unm);

}