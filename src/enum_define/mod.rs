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

pub enum Role {
    Admin,
    Guest,
    Read
}

pub fn enum_role(role_number: u8) -> Role {
    if role_number == 1 {
        Role::Admin
    } else if role_number == 2 {
        Role::Guest
    } else {
        Role::Read
    }
}

#[derive(Debug)]

enum Message {
    StartGame,
    WinPoint { who: String },
    ChangePlayerName (String)
}

pub fn game_message() {
    let m1 = Message::StartGame;
    let m2 = Message::WinPoint { who: String::from("sojiwoo") };
    let m3 = Message::ChangePlayerName(String::from("클릭"));
    println!("{:?}, {:?}, {:?}", m1, m2, m3);
}


pub fn option_generic() {
    let some_number = Some(2);
    let absent_number: Option<i32> = None; // Null이 없고, 빈 값으로 인한 시스템 오류가 발생하지 않게 처리할 수 있다.
    println!("{:#?}, {:#?}", some_number, absent_number);
}