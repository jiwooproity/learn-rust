pub fn ownership() {
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
}

// 소유권 임대
pub fn borrowing() {
    let s: String = String::from("소유권 임대");
    let len = calc_length(&s); // & 키워드와 함께 소유권을 move하지 않고 borrowing

    fn calc_length(s: &String) -> usize {
        // &는 본래 포인터를 포인터로 잠깐 참조하여 사용이 완료되면 빠져나간다.
        s.len()
    }

    println!("{}의 길이는 {}byte입니다.", s, len);
}

// &는 기본적으로 immutable이기 때문에 본래 변수를 mut 키워드로 명시하여도 변경이 불가능함.
// pub fn immutable() {
//     let mut s: String = String::from("기본적");

//     append_word(&s);

//     fn append_word(s: &String) {
//         s.push_str("immutable"); // Error: cannot borrow `*s` as mutable, as it is behind a `&` reference
//     }

//     println!("s = {}", s);
// }

// mutable한 상태로 변경하는 방법 ( & 참조값은 기본적으로 immutable함!! )
pub fn immutable() {
    let mut s: String = String::from("기본적");

    append_word(&mut s); // 어떤 형태로 바뀔 수도 있다는 것을 &mut 키워드로 명시하여 넘긴다.

    // &mut한 임대 String으로서 변경이 가능한 인자로 넘겨받음
    fn append_word(s: &mut String) {
        s.push_str("immutable");
    }

    println!("s는 {}", s);
}

// & ampersand는 여러 번 참조 불가능 ( 하나의 참조만 만들 수 있음 )
pub fn many_ampersand() {
    let mut s: String = String::from("앰퍼샌드");

    {
        // let r2 = &mut s;
    } // 서로 다른 블록 간에는 사용 가능

    let r1 = &mut s;
    // let r2 = &mut s; Error: cannot borrow `s` as mutable more than once at a time
    // 이유 : Rust 컴파일러가 데이터 경재조건을 방지하기 위함
    // 둘 이상의 포인터가 같은 데이터를 참조하는 경우 해당 데이터 조건을 동기화할 방법이 없음 ( 일일히 개발자가 동기화 처리를 해 주어야 함 )

    println!("r1 = {}", r1);
}

pub fn many_ampersand_mut() {
    let mut str: String = String::from("헬로");

    let s2 = &str; // 일반 불변 참조 1
    let s3 = &str; // 일반 불변 참조 2
    println!("s2 = {}, s3 = {}", &s2, &s3);

    // let s4 = &mut str;
    // Error: cannot borrow `str` as mutable because it is also borrowed as immutable
    // mut을 사용하게 된다면 다른 참조는 사용할 수 없기에 에러 발생

    // println!("s2 = {}, s3 = {}, s4 = {}", &s2, &s3, &s4);

    // 하지만, 범위가 겹치지 않는다면 사용이 가능하다.
    let s4 = &mut str;
    println!("s4 = {}", s4); // 코드 위치에 따라 범위가 한정되고, 범위가 겹치지만 않는다면 사용이 가능 ( 현재 s4만 사용한다는 것으로 인지 )
    // println!("s3 = {}", s3); // mut 참조가 진행된 후에는 사용이 불가
}

// 참조 Refference 정리
// 1. 참조로 소유권을 넘기지 않고 데이터에 접근
// 2. 딱 하나의 변경 가능 참조가 있거나, 불변 참조를 여러 개 활용 가능
// * 불변 참조가 하나라도 선언되는 순간 그 전 일반 불변 참조 값은 사용 불가능