struct User {
    name: String,
    email: String,
    active: bool,
}

// build_user 함수를 하나 정의하여 name과 email String을 전달받은 후,
// User 구조체를 활용하여 새로운 유저 데이터를 만들고 User에 대한 소유권과 함께 반환
fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        active: true
    } // 자바스크립트에서의 객체 표현처럼 변수의 이름과 필드의 이름이 같다면 생략이 가능함.
}

pub fn struct_one() {
    // let mut user = User {
    //     name: String::from("jiwoo"),
    //     email: String::from("jiwoo@dummy.com"),
    //     active: true,
    // };

    // struct도 마찬가지로 프로퍼티를 변경하고자 한다면 mut 키워드를 명시해야함.
    let mut user = build_user(String::from("jiwoo"), String::from("jiwoo@dummy.com"));
    user.email = String::from("dummy@dummy.com");

    println!("이용자의 이름은 {}입니다.", user.name);
    println!("이용자의 이메일은 {}입니다.", user.email);
}

// 이미 있는 구조체를 활용하여 새로운 인스턴스 만들기
pub fn struct_new() {
    let user = User {
        name: String::from("hongildong"),
        email: String::from("gildong@dummy.com"),
        active: false,
    };

    // 기존 user의 정보를 사용할 수 있지만, 소유권 개념으로 인해 기존 user는 name과 email의 소유권을 뺏기게 된다.
    // Heap에 할당된 값에 대해 사용이 필요한 경우, clone() 메서드를 통해 복사를 해 주어야 한다.
    let user2 = User {
        name: user.name.clone(),
        email: user.email.clone(),
        active: true,
    };

    // Spread 연산자를 통해 할당도 가능
    // let user3 = User {
    //     active: false,
    //     ..user2
    // };

    println!("user2.email = {}", user2.email);
    println!("user2.active = {}", user2.active);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Tuple Struct 구조체 정의
// 기존 tuple 정의와 동일한 형태로 구조체 정의가 가능하고, 동일하게 동작한다.
pub fn tuple_struct() {
    let color = Color(1, 2, 3);
    let point = Point(1, -2, 3);
    
    println!("{}, {}, {}", color.0, color.1, color.2);
    println!("{}, {}, {}", point.0, point.1, point.2);
}

// 디버깅 목적
#[derive(Debug)] // or dbg!
struct Rectangle {
    width: u32,
    height: u32,
}

fn calc_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub fn struct_example() {
    let rectangle = Rectangle {
        width: 30,
        height: 40
    };

    let area = calc_area(&rectangle);

    println!("사각형 = {:?}", rectangle);
    // 사각형 = Rectangle { width: 30, height: 40 }
    
    dbg!(&rectangle);
    // [src/struct_define/mod.rs:90:5] rectangle = Rectangle {
    //     width: 30,
    //     height: 40,
    // }

    println!("직사각형의 면적은 {}입니다.", area);
}

// struct method 정의
impl Rectangle {
    // 자기 자신이 가진 프로퍼티를 가지고 동작을 정의할 수 있음
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn squere(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

pub fn struct_example_2() {
    let rectangle = Rectangle {
        width: 30,
        height: 40,
    };

    println!("사각형의 면적 = {}", rectangle.area());

    let other_rectangle = Rectangle::squere(20);
    println!("정사각형 = {:?}", other_rectangle);
}

// 직접 구조체 ( Struct )와 구조체 메서드 ( Struct Method ) 를 한번 더 응용
#[derive(Debug)]
struct Circle {
    radius: u32
}

impl Circle {
    // self: &Self = &self 축약 표현
    fn calc_round(&self) -> f32 {
        3.14 * 2 as f32 * self.radius as f32
    }
}

pub fn struct_method() {
    let mut circle = Circle { radius: 5 };
    println!("반지름이 5인 원의 둘레 = {}", circle.calc_round());
    dbg!(&circle);

    circle.radius = 10;
    println!("반지름이 10인 원의 둘레 = {}", circle.calc_round());
    dbg!(&circle);
}

struct Vector3 {
    x: i32,
    y: i32,
    z: i32
}

impl Vector3 {
    fn get_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    fn up(&mut self) {
        self.y = self.y + 50;
    }

    fn down(&mut self) {
        self.y = self.y - 50;
    }
}

pub fn struct_vector() {
    let mut vector3 = Vector3 {
        x: 50,
        y: 100,
        z: 51,
    };

    vector3.up();

    let (x, y, z) = vector3.get_tuple();

    println!("x = {}, y = {}, z = {}", x, y, z);

    vector3.down();

    let (x, y, z) = vector3.get_tuple();

    println!("x = {}, y = {}, z = {}", x, y, z);
}

struct ControlTwoNumber {
    a: u32,
    b: u32
}

impl ControlTwoNumber {
    fn add(self: &ControlTwoNumber) -> u32 {
        self.a + self.b
    }

    fn minus(self: &ControlTwoNumber) -> u32 {
        if self.a > self.b {
            self.a - self.b
        } else {
            self.b - self.a
        }
    }

    fn multi(&self) -> u32 {
        self.a * self.b
    }
}

pub fn control_two_number(a: u32, b: u32) {
    let two_number_struct = ControlTwoNumber { a, b };
    let add = two_number_struct.add();
    let minus = two_number_struct.minus();
    let multi = two_number_struct.multi();

    println!("add = {}, minus = {}, multi = {}", add, minus, multi);
}

struct Counter {
    number: i32
}

impl Counter {
    fn add(self: &Counter, b: i32) -> i32 {
        self.number + b
    }
}

pub fn control_counter(number: i32) {
    let counter_struct: Counter = Counter { number };
    let more_add = counter_struct.add(number); // 64?
    println!("number + a = {}", more_add);
}