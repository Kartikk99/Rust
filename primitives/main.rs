use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let(int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({} {})\n",self.0, self.1);
        write!(f, "({} {})", self.2, self.3)
    }
}

impl Matrix {
    fn transpose(&self) -> Matrix {
        Matrix(self.0, self.2, self.1, self.3)
    }
}

fn main(){
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("Long Tuple first value : {}", long_tuple.0);
    println!("Long Tuple second value : {}", long_tuple.1);

    let tupleOfTuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    println!("Tuple of tuples : {:?}", tupleOfTuples);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("Reversed Pair : {:?}", reverse(pair));

    let tuple = (1, "hello", 4.5, true);

    let (a,b,c,d) = tuple;

    println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);

    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", matrix.transpose());
}
