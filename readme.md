# 추석맞이 언어탐방

# Rust

## 왜 rust를 쓸까?

1. 언어 표현으로 로우한 레벨의 메모리를 다루는 것이 가능하다.
   - `&`: reference를 직접 다룬다. (포인터와 비슷?)
2. 빠르다
3. 변수의 생명주기를 Tracking 한다.
   - GC가 있는 언어: 변수 관리하기 어려운데 막 사용하고 나중에 알아서 한번에 정리해주면 되잖아?
   - C++: 메모리 할당 해제를 개발자가 직접 해주기, 근데 이제 잘. 메모리 누수 없게. 개발자가 알아서.
   - rust: 컴파일 단계에서 변수의 생명주기를 추적한다. 컴파일러가 알려준다. (메모리 누수 예방 가능)

### 설치

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`source "$HOME/.cargo/env"`

`cargo init`

- 배포파일 실행
  `./target/debug/rust`
  `cargo run`

- rust 라이브러리: 크레이트 https://crates.io/
- 의존성 관리: Cargo.toml
- rust-analyzer: rust IDE (vscode extension)

## 변수 선언하기

```rust

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
```

### `&'static`

- &'static은 static data 영역에 저장한다는 의미
- 프로세스 종료 전까지 영구적으로 해당 주소에 할당
- 러스트는 개발자가 메모리를 직접 관리할 수 있음

### `;`

### struct와 enum

```rs
// 1. struct
struct A {
    a: i32,
    b: i32,
}

// 2. enum
enum B {
    A,
    B,
    C {
        a: i32,
        b: i32,
    }
}
```

### match

- pattern matching (switch문과 비슷)
- rust는 success, failure 로직 처리할 때 사용하는 Result 라는 enum이 있다. (try catch 대신)

```rs
match a {
    Ok(buffer) => {
        println!("buffer: {:?}", buffer);
        let string = String::from_utf8(buffer);

        match string {
            Ok(string) => {
                println!("string: {:?}", string);
            },
            Err(error) => {
                println!("error: {:?}", error);
            },
        }
    },
    Err(error) => {
        println!("error: {:?}", error);
    },
}
```

### 예시 코드

- Vec 타입: 1byte를 저장하는 배열
- String 타입: Vec<u8> 타입을 가지고 있는 구조체
- u8 타입: 1byte 크기의 부호없는 정수 unsigned integer 8bit (0 ~ 255)

```rs
fn main() {
    let a = std::fs::read("./a.txt");

    let buffer = {
        match a {
            Ok(buffer) => buffer,
            Err(error) => {
                println!("error: {:?}", error);
                return;
            },
        }
    };

    println!("buffer: {:?}", buffer);

    let string = {
        match String::from_utf8(buffer) {
            Ok(string) => string,
            Err(error) => {
                println!("error: {:?}", error);
                return;
            },
        }
    };

    println!("string: {:?}", string);
}
```

### !

```rs
fn main() {
    let a = world!();
    println!("{}", a);

    let b = hello!(5);
    println!("{}", b);
}

macro_rules! world {
    () => {
        "world"
    };
}

macro_rules! hello {
    () => {
        "hello".to_string()
    };
    ($e:expr) => {{
        let mut s = String::new();
        for _ in 0..$e {
            s.push_str("hello");
        }
        s
    }}
}

use hello;
use world;
```

## 참고

- 공식문서 https://doc.rust-lang.org/cargo/index.html
