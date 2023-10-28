#[derive(Debug)]
struct Person <'a>{
    name: &'a str,
    age: u8
}


fn main(){
    println!("{} days", 31);

    println!("My name is {0}, {1} {0}","Bond", "James");
    println!("Decimal: {}", 69420);
    println!("Binary: {:b}", 69420);
    println!("Octal: {:o}", 69420);
    println!("Hexa: {:x}", 69420);

    println!("{number:>5}", number=1); // Prints the number with 4 white spaces in front.

    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000

    // Only type with implement fmt::Display can be formatted with {}.
    
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    println!("Hello {0} is {1:.5}", "x", 0.01);
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("Hello {} is {:.*}", "x", 5, 0.01);
    println!("Hello {} is {number:.prec$}", "x", prec=2, number=3.141592);

    let name = "Kartik";
    let age = 25;
    let obj = Person{name, age};
    
    // :#? is used for pretty print
    println!("{:#?}", obj);
}
