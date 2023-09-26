# 추석맞이 언어탐방

# Rust

### 설치

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`source "$HOME/.cargo/env"`

`cargo init`

- 배포파일 실행
  `./target/debug/rust`
  `cargo run`

- rust 라이브러리: 크레이트 https://crates.io/
- 의존성 관리: Cargo.toml

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
- 언어 표현으로

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
