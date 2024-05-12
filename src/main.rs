// 패키지 import
use std::io;

mod variables;
mod calculate;
mod types;

fn main() {
    // 출력 구문
    println!("[가위, 바위, 보] 중 하나를 입력해 주세요.");

    // 타입은 String 타입 지정
    let mut decision = String::new();
    
    // 터미널로 입력받은 문자열을 decision에 담고, 예외 상황엔 expect 함수로 메세지 대체
    io::stdin().read_line(&mut decision).expect("입력에 실패하였습니다.");

    println!("당신의 선택은: {decision}");

    // 변수, 상수 관리
    variables::mutable();
    variables::immutable();

    // 변수 섀도잉 현상
    variables::shadowing();

    // 타입
    types::float();
    types::tuple();
    types::integer();
    types::array();

    let result_circle = calculate::circle_area(0.4);
    println!("0.4의 면적을 가진 원의 면적은 {result_circle}입니다.");

    let add_result = calculate::add_numbers(1, 3);
    println!("a + b = {add_result}입니다.");
}