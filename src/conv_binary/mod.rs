use std::io;

pub fn binary(number: i64) -> String {
    let binary_format = format!("{:032b}", number);
    println!("{binary_format}");
    let str: &str = &binary_format;
    let mut str1 = String::from("");

    for(i, &_item) in binary_format.as_bytes().iter().enumerate() {
        if i % 8 == 0 {
            str1.push_str(&str[i..i + 8]);
            str1.push_str(" ");
        }
    }

    str1
}

pub fn convert_binary() {
    let mut decision = String::new();

    io::stdin().read_line(&mut decision).expect("입력에 실패하였습니다.");
    let parse: i64 = decision.trim().parse().expect("숫자 형식이 올바르지 않습니다.");
    let get_binary = binary(parse);
    println!("{}", get_binary);
}

struct Binary {
    number: i64
}

impl Binary {
    fn convert(self: &Binary) -> String {
        binary(self.number)
    }
}

pub fn use_binary_struct(number: i64) {
    let binary_struct = Binary { number };
    let get_binary = binary_struct.convert();
    println!("{} to binary = {}", number, get_binary);
}