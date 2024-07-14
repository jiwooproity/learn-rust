// 패키지 import
use std::io;

mod variables;
mod calculate;
mod types;
mod control_flow;
mod ownership;
mod struct_define;
mod conv_binary;
mod enum_define;

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
    types::integer();
    types::char();
    types::float();
    types::tuple();
    types::array();

    // 계산 함수
    calculate::integer_calc();
    calculate::multiple(-127, 126);

    let result_circle = calculate::circle_area(0.4);
    println!("0.4의 면적을 가진 원의 면적은 {result_circle}입니다.");

    let add_result = calculate::add_numbers(1, 3);
    println!("a + b = {add_result}입니다.");

    // 제어문 if, if else
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("입력에 실패하였습니다.");
    let parse_input: u32 = input_string.trim().parse().expect("숫자 형식이 올바르지 않습니다. u32");
    control_flow::if_and_else_if(parse_input);

    let check_condition = control_flow::check_condition(parse_input > 16);
    println!("{}", check_condition);

    control_flow::use_loop();
    control_flow::use_while();
    control_flow::use_for();
    control_flow::use_limit_for();

    // 메모리 관리 규칙 | 소유권
    ownership::ownership();
    ownership::borrowing();
    ownership::immutable();
    ownership::many_ampersand();
    ownership::many_ampersand_mut();
    ownership::use_slice();
    ownership::other_slice();

    // Struct: 구조체 정의
    struct_define::struct_one();
    struct_define::struct_new();
    struct_define::tuple_struct();
    struct_define::struct_example();
    struct_define::struct_example_2();
    struct_define::struct_method();
    struct_define::struct_vector();
    struct_define::control_two_number(30, 12);
    struct_define::control_two_number(12, 5);
    struct_define::control_counter(32);
    
    conv_binary::convert_binary();
    conv_binary::use_binary_struct(2);

    enum_define::enum_print();
}