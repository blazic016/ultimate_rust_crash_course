fn main() {
    let mut age = 10;
    println!("age={}", age);
    age = 15;
    println!("age={}", age);

    let var = "hehe";
    println!("var={}", var);
    let var = "hehehe";
    println!("var={}", var);

    let signed_var: i8 = 127; // 0-127
    println!("signed_var={}", signed_var) ;

    let unsigned_var: u8 = 255; // 0-255
    println!("unsigned_var={}", unsigned_var);
}
