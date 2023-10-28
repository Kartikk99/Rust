use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    real: f64,
    img: f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{:1} + {:2}i", self.real, self.img)
    }
}

fn main(){
    let minMax = MinMax(0, 15);
    println!("Compare structure:");
    println!("Display: {}", minMax);
    println!("Debug: {:?}", minMax);

    let bigRange = MinMax (-300, 300);
    let smallRange = MinMax (-3,3);

    println!("BigRange : {big}, smallRange : {small}",small = smallRange, big = bigRange);

    let point = Point2D{img:3.3, real:7.2};

    println!("Display : {}", point);
    println!("Debug : {:?}", point);
}
