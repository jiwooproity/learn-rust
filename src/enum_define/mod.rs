#[derive(Debug, PartialEq)] // PartialEq : 해당 지시자를 넘겨주어 비교 연산자 구현이 가능하도록 설정
enum Color {
    Red,
    Green,
    Blue
}

pub fn enum_print() {
    let red = Color::Red;
    let green = Color::Green;
    let _blue = Color::Blue;

    println!("red = {:?}", red);
    println!("red == green => {}", red == green);
    println!("red == Color::Red => {}", red == Color::Red);
}
