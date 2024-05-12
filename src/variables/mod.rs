pub fn mutable() {
    let mut x: u8 = 3; // mut : mutable 키워드 사용하여 변수로 사용 가능
    println!("x의 값은 {x}입니다");
    x = 7;
    println!("x의 값은 {x}입니다");
}

pub fn immutable() {
    const PI: f64 = 3.141592;
    println!("PI = {PI}");
}

pub fn shadowing() {
    let x: i8 = 5;
    println!("x = {x}");

    let x: i8 = 6;
    println!("변수 셰도잉 발생 x = {x}");

    {
        println!("{x} + 7");
        let x: i8 = x + 7;
        println!("x = {x}");
    }

    println!("스코프를 빠져나온 x의 값 {x}")
}