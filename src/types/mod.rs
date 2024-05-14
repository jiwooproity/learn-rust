pub fn integer() {
    let i8_01: i8 = 8;
    let i8_02: i8 = -8;

    let u8_01: u8 = 8;

    // let u8_02: u8 = -8; 부호 불가능

    // i8, i16, i32, i64, i128, size
    // u8, u16, u32, u64, u128

    println!("i8_01 = {i8_01}");
    println!("i8_02 = {i8_02}");
    println!("u8_01 = {u8_01}");
}

pub fn char() {
    // String
    let name: String = "sojiwoo".to_string();
    println!("My name is {name}");

    // &str
    let company = "toss";
    println!("i hope to join the {company}");
    
    // char
    let character: char = 'A';
    let unicode: u32 = 0x41;
    println!("{}의 유니코드는 {:#0x}입니다.", character, unicode);

    // scalar unicode convert
    let get_char: char = char::from_u32(unicode).unwrap();
    println!("{:#0x}의 유니코드를 문자로 표현하면 {}입니다.", unicode, get_char);

    println!("[{progress:>0width$}]", progress="###", width=10);
}

pub fn array() {
    let arr: [i8; 3] = [8 ,9, 10];

    // 구조분해 할당
    let [x, y, z] = arr; // 배열 구조분해

    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");

    // 인덱스 접근
    let x = arr[0];
    let y = arr[1];
    let z = arr[2];

    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");

    // 원하는 개수만큼 배열 생성
    let create_arr: [i8; 300] = [3; 300];
    let size = create_arr.len();
    let three_hundred_value = create_arr[size - 1];

    println!("생성된 배열의 크기: {size}");
    println!("{size}번째 배열의 값: {three_hundred_value}")
}

pub fn float() {
    let a: f32 = 3.141592;
    let b: f32 = -3.141592;
    println!("f32 양수 : {a}");
    println!("f32 음수 : {b}");
}

pub fn tuple() {
    let tuple: (i8, bool, f32) = (-16, false, 3.14);

    // 구조 분해
    let (a, b, c) = tuple;

    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");

    // 접근 연산자
    let d = tuple.0;
    let e = tuple.1;
    let f = tuple.2;

    println!("d = {d}");
    println!("e = {e}");
    println!("f = {f}");
}