const PI: f64 = 3.141592;

pub fn circle_area(radius: f64) -> f64 {
    let r2 = radius * radius;
    PI * r2
}

pub fn add_numbers(a: i8, b: i8) -> i8 {
    a + b
}

pub fn integer_calc() {
    let a: i8 = 5 + 9;

    let b: i8 = 5 - 9;

    let c: u8 = 5 + 9;

    let d: i8 = a * b;

    // let b: u8 = 5 - 9; 음수 할당 불가능
    println!("a: {a}");
    println!("b: {b}");
    println!("c: {c}");
    println!("d: {d}");
}

pub fn multiple(a: i32, b: i32) -> i32 {
    a * b
}