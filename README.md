# Rust Programming Language

**Rust**는 모질라 리서치에서 개발한 다중 패러타임, 범용 프로그래밍 언어이다.

## Rust 공식 홈페이지

https://www.rust-lang.org/

## Rust, 시작하기

https://www.rust-lang.org/learn/get-started

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Rust, 패키지 매니저 ( Cargo )

**Cargo**는 **Rust**의 빌드 시스템이자 패키지 매니저이다.

Npm, Yarn의 패키지 매니저처럼 라이브러리 의존성을 관리하고 Rust 코드의 빌드를 담당한다.

Cargo는 Getting started 에서 설명한 내용처럼 Rust를 설치했다면 같이 설치가 되어있다.

```
rustc --version // Rust 설치 및 버전 확인
cargo --version // Cargo 설치 및 버전 확인
```

## Rust, 컴파일

**Rust**를 설치했다면 main.rs 러스트 코드를 작성한 후, "rustc" 커맨드로 컴파일이 가능하다.

```
// main.rs
fn main() {
  let a: i8 = 1;
  println!("a의 값 : {}", a);
}

// CLI
rustc ./main.rs
... compiling
./main // 컴파일된 러스트 코드 실행
```

## Rust, 프로젝트 시작

Cargo가 설치되어 있다면 Rust 프로젝트를 생성할 수 있다. 생성 방법은 아래와 같다.

```
cargo new rust_project
```

생성된 프로젝트를 살펴보면 Cargo.toml, .gitignore, .git, ./src/main.rs 등 파일이 생성된 것을 확인할 수 있다.

여기서 Cargo.toml은 package.json 과 같은 라이브러리 의존성을 관리하는 파일과 같다. 아래는 .toml 파일의 모습이다.

```
[package]
name = "rust_project"
version = "0.1.0"
authors = ["SongKJ00 <gowithdsm@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
// 디펜던시!
```

## Rust, 프로젝트 빌드

Rust 프로젝트에서 Cargo를 통해 main.rs 파일을 빌드할 수 있다.

```
cargo build // 컴파일 진행
cargo run // 컴파일된 파일 실행
```

## Rust, VSCode Extenstion 설치

Rust는 코드 작성에 도움을 주는 개발 도구를 지원한다.

아래 사진의 Rust Analyzer를 설치하면 컴파일을 매번 실행할 필요 없이 코드 작성 단계에서 실행해 볼 수 있다.

또한, 코드의 자동완성과 추론을 도와주며 코드 색상을 구분하여 보다 작성하기 편한 환경을 제공해 준다.

![스크린샷 2024-05-13 오후 2 57 22](https://github.com/jiwooproity/learn-rust/assets/58384366/ee8e2992-349b-45aa-aafb-0214f7062a2c)

![스크린샷 2024-05-13 오후 2 59 05](https://github.com/jiwooproity/learn-rust/assets/58384366/c1937abf-9934-4c9e-b5e8-497fa3c79739)
