fn main(){
    let logical: bool = true;

    let a_float: f64 = 10.00; // Regular annotation
    let an_intger = 5i32;     // Suffix annotation
                              //
    let default_float = 3.0; // f64
    let default_int = 10;    // i32

    let mut inferred_type = 12;     // Type i64 is inferred from another line
    inferred_type = 4294967296i64; // Suffix annotation

    
    // Mutable variable value can be changed, but, type of variable cannot be changed.
    let mut mutable = 12;
    mutable = 21;

    // mutable = true; // Will generate error
}
