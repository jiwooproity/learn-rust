# Rust

## Enum 열거형

Rust에서는 Enum 타입을 활용하여 데이터를 열거할 수 있다.

코드 예 )

```
enum Color {
    Red,
    Green,
    Blue
}

fn main() {
    let red: Color = Color::Red;
    let green: Color = Color::Green;
    let blue: Color = Color::Blue;
}
```

이렇게 열거된 각 데이터 타입에 대한 표현이 가능하다.

이 경우, Display 트레잇이 구현되어 있지 않기 때문에 출력을 위해서는 Rust의 디버깅 옵션이 필요하다.

```
#[derive(Debug)]

enum Color {
    Red,
    Green,
    Blue
}

fn main() {
    let red: Color = Color::Red;
    let green: Color = Color::Green;
    let blue: Color = Color::Blue;

    println!("red = {:?}", red);
}
```

#[derive(Debug)]을 추가해주면 output은 "red = Red" 라는 값으로 잘 출력되는 모습을 확인할 수 있다.

다른 방법으로 연산 작업 시, 값에 대한 비교 작업을 사용하고 싶을 수 있다.

이때는 PartialEq 지시어를 통해 비교 작업에 대한 결과를 출력할 수 있다.

```
#[derive(Debug, PartialEq)]

enum Color {
    Red,
    Green,
    Blue
}

fn main() {
    let red: Color = Color::Red;
    let green: Color = Color::Green;
    let blue: Color = Color::Blue;

    println!("red = {:?}", red);
    println!("red = {:?}, red == blue => {}", red, red == blue);
}
```
