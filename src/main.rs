// 패키지 import
use std::io;

fn main() {
    // 출력 구문
    println!("[가위, 바위, 보] 중 하나를 입력해 주세요.");

    // 타입은 String 타입 지정
    let mut decision = String::new();
    
    // 터미널로 입력받은 문자열을 decision에 담고, 예외 상황엔 expect 함수로 메세지 대체
    io::stdin().read_line(&mut decision).expect("입력에 실패하였습니다.");

    println!("당신의 선택은: {decision}");

    mutable();
    float();
    variables_shadowing();
    data_type();
    rust_calc()
}

fn mutable() {
    let mut x: u8 = 3; // mut : mutable 키워드 사용하여 변수로 사용 가능
    println!("x의 값은 {x}입니다");
    x = 7;
    println!("x의 값은 {x}입니다");
}

fn float() {
    const PI: f32 = 3.141592;
    println!("PI상수 값은 {PI}입니다");
}

fn variables_shadowing() {
    let number: u8 = 3;
    println!("Number의 값은: {number}"); // 3

    let number = number + 1;
    // 위에서 선언된 number와 다른 number
    // 앞으로는 해당 변수로만 참조됨
    println!("Number의 값은: {number}"); // 4

    {
        let number = number * 3; // 블록이 끝나면 참조 불가능
        println!("Number의 값은: {number}"); // 12
    }

    println!("Number의 값은: {number}"); // 4

    let sign_integer: i8 = -16;
    let number: i8 = sign_integer;

    println!("Number의 값은: {number}") // -16
}

fn data_type() {
    // Rust에서 모든 값은 특정 데이터 타입의 값
    // 기본 값 Scalar 또는 복합값 Compound
    // Rust는 정적 타입 언어
    // 컴파일 시점에 모든 변수의 타입을 알아야 함
    let unsign_x: u8 = 255;
    let integer_x: i8 = -128;

    let float_x: f32 = 3.14;

    println!("{unsign_x}");
    println!("{integer_x}");
    println!("{float_x}");
}

fn rust_calc() {
    let _add: i8 = 3 + 8;

    let _sub: f32 = 26.5 - 2.1;

    let _mul: u8 = 7 * 20;

    let _quotieno: f32 = 12.0 / 3.14;
    let truncated: u8 = 8 / 3;

    println!("{truncated}");
}