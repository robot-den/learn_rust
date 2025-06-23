// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;
mod kek;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();

    kek::print_kek();
}