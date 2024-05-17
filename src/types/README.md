# Rust

**Rust**에는 정수, 부동 소수점 숫자, Boolean, 문자라는 네 가지 기본 스칼라 유형이 있다.

## Integer Types

|Length|Signed|Unsigned|
|------|------|--------|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

**정수 타입**은 부호가 있는 정수와 없는 정수로 나뉜다.

부호가 있는 정수는 -(2 <sup>n-1</sup> ) ~ 2 <sup>n-1</sup> - 1 까지의 숫자를 저장할 수 있다.

반대로 부호가 없는 정수는 음수를 다루지 않고 0 ~ 255의 정수를 저장할 수 있다.

확실히 **정해진 경우**가 아니라면 Rust의 기본 값인 i32를 사용하는 것이 일반적으로 좋은 선택일 수 있다.

## Floating-Point Types

소수점을 갖는 부동 소수점 숫자를 위한 두 가지 기본 타입은 f32와 f64가 있다.

왼쪽부터 각기 32bit, 64bit의 크기를 가지며, 기본 타입은 f64이다.

f64가 기본 값인 이유는 최신 CPU 상에서는 f64가 f32와 대략 비슷한 속도를 내면서도 더 정밀한 표현이 가능하기 때문이다.

```
fn main() {
  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
}
```

## Boolean Type

대부분의 언어들과 같이 boolean 타입은 <code>true</code> <code>false</code> 둘 중 하나의 값만 가질 수 있다.

러스트에서는 <code>bool</code>로 명시된다.

```
fn main() {
  let t = true;
  let f: bool = false;
}
```

## Character Type

Rust의 <code>char</code>는 가장 근본적인 앞파벳 타입이다.

<code>String</code> 타입은 큰 따옴표로 명시하는 반면, <code>char</code> 타입은 작은 따옴표로 명시한다.

```
fn main() {
  let c = 'c';
  let z = 'Z';
}
```

Rust의 <code>char</code> 타입은 Unicode Scalar를 표현하는 값이며 ASCII 보다 많은 표현을 할 수 있다.

한국어, 중국어, 일본어 표의 문자, 이모티콘, 넓이가 0인 공백 문자 모두가 Rust에서는 <code>char</code> 타입으로 사용할 수 있다.

하나의 문자는 각 문자를 나타내는 고유한 숫자 값을 가지고 있다.

유니코드의 스칼라는 일반적으로 16진수 표기법으로 나타내며 <code>U+</code> 접두사 다음에 16진수의 숫자가 오게 된다.

<a href="http://titus.uni-frankfurt.de/unicode/unitestx.htm#UA500" target="_blank">참고자료</a>
