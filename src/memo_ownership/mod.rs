pub fn memo_ownership() {
    let x = 3;
    let y = x;
    println!("x = {x}, y = {y}"); // stack에만 있는 값, Rust가 자동으로 처리하여 소유권이 없다고 봐도 무방

    let s1 = String::from("헬로"); // Heap
    println!("s1 = {}", s1);

    // let s2 = s1; // s2로 소유권이 넘어감
    // println!("s2 = {}", s2);
    let s2 = s1.clone(); // Heap 메모리에 하나 더 할당하여 복사
    println!("s2 = {}", s2); // s1, s2 따로 따로 사용 가능

    let str = String::from("헬로");
    string_length(str); // 함수로 str 소유권이 넘어감

    let number = 3;
    double(number); // 소유권이 넘어가지 않고 Stack 자체 값 복사가 발생

    let str2 = String::from("헬로우우우우우");
    let str2 = get_string_length(str2); // 소유권 이전 후, 다시 결과 값으로 반환
    println!("s의 소유권을 얻었는가?! {}", str2); // 얻었다!

    let new_str = String::from("헬로아");
    let (len, return_str) = string_length(new_str);
    println!("new_str {} length: {}", return_str, len);

}

fn get_string_length(s: String) -> String {
    println!("s의 값은 {}", s);
    s
}

fn double(x: i32) {
    println!("x = {}", x);
}

fn string_length(s: String) -> (usize, String) {
    println!("문자열 str의 길이는: {}", s.len());
    (s.len(), s)
}