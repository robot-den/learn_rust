use std::fmt;
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

struct ShMatrix(bool, i32, str);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {:?}, {:?} )\n", self.0, self.1)?;
        write!(f, "( {:?}, {:?} )", self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.1, matrix.0, matrix.3, matrix.2)
}

fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("One million is written as {}", 1_000_000);

    let pair = (10, true);
    println!("pair is {:?}", pair);
    println!("reversed pair is {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    let mut xs: [i32; 5] = [1, 2, 3, 4, 5];
    let xs2 = [1, 2 ,3 , 4];

    let ys: [i32; 500] = [0; 500];
    let ys2 = [0; 500];
    println!("xs: {}, ys2: {}", xs.len(), ys.len());

    println!("xs: {:?}", xs);
    let mut slice = &mut xs[1..3];
    println!("slice: {:?}", slice);
    slice[0] = 8;
    println!("xs: {:?}", xs);
    // println!("{}", xs[5]); // Out of bounds in compile
    // println!("{}", xs[..][5]); // Out of bound in runtime
}