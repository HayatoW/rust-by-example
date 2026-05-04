use std::fmt::Display;

fn transpose(m: Matrix) -> Matrix {
    // こう書ける
    let Matrix(m0, m1, m2, m3) = m;

    Matrix(m0, m2, m1, m3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}
