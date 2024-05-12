pub fn if_and_else_if(a: u32) {
    if a > 16 {
        println!("a가 16보다 큽니다!");
    } else if a < 16 {
        println!("a가 16보다 작습니다!");
    } else {
        println!("a가 16입니다.")
    }
}