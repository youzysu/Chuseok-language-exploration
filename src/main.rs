// mut
fn main() {
    let mut happy = Dog { age: 0 };
    add_one(&mut happy);
    
    print(&happy);
    assert!(happy.age == 1);

    take(happy);
    print(&happy); // take 함수에서 happy를 가져갔기 때문에 여기서는 사용할 수 없다.
}

fn print(target: &Dog) {
    println!("target: {}", target.age);
}

fn add_one(target: &mut Dog) {
    target.age += 1
}

fn take(target: Dog) {}

// 재미난 struct 만들기
#[derive(Clone, Copy)] // Copy trait를 구현하면, 해당 타입은 stack에 저장된다.
struct Dog {
    age: u8,
}

// 1. 언어 표현으로 로우한 레벨의 메모리를 다루는 것이 가능하다.
// 2. 빠르다: 
// GC가 필요 없다.
// & 타입: reference를 직접 다룬다. (포인터와 비슷?)

// unwrap은 무엇인가
// 세미콜론

// rust는 success, failure 로직 처리할 때 사용하는 Result 라는 enum이 있다. (try catch 대신)
// pattern matching (switch문과 비슷)
// Vec 타입: 1byte를 저장하는 배열