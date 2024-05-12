pub fn if_and_else_if(a: u32) {
    if a > 16 {
        println!("a가 16보다 큽니다!");
    } else if a < 16 {
        println!("a가 16보다 작습니다!");
    } else {
        println!("a가 16입니다.")
    }
}

pub fn check_condition(condition: bool) -> &'static str {
    let checking = if condition { "16 위" } else { "16 아래" };
    checking
}

pub fn use_loop() {
    let mut counter: i8 = 0;

    let x = loop {
        counter += 1;

        if counter >= 5 {
            break counter;
        }

        println!("반복! {counter}");
    };

    println!("반복의 마지막 값: {x}");
}

pub fn use_while() {
    let array: [i8; 5] = [1, 2, 5, 3, 2];
    let mut index = 0;

    while index < array.len() {
        println!("{index}: {}", array[index]);
        index += 1;
    }
}

pub fn use_for() {
    let array: [i8; 5] = [1, 2, 5, 3, 2];

    for x in array {
        println!("{x}");
    }
}

pub fn use_limit_for() {
    // 0 ~ 5까지 반복
    for i in 0..5 {
        println!("i = {}", i);
    }

    let array: [i8; 5] = [5, 3, 1, 2, 4];

    // 0 ~ array 개수 만큼 순차적으로 반복
    for i in 0..array.len() {
        println!("i = {}", array[i]);
    }

    // 0 ~ array 개수 만큼 반대로 반복
    for ri in (0..array.len()).rev() {
        println!("i = {}", array[ri]);
    }
}