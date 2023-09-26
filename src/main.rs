// mut
fn main() {
    let mut happy = Dog { age: 0 };
    add_one(&mut happy);
    
    print(&happy);
    assert!(happy.age == 1);

    take(happy); // 변수의 소유권Ownership이 take 함수로 넘어간다.
    print(&happy); // take 함수에서 happy를 가져갔기 때문에 여기서는 사용할 수 없다. 
}

fn print(target: &Dog) {
    println!("target: {}", target.age);
}

fn add_one(target: &mut Dog) {
    target.age += 1
}

fn take(target: Dog) {}

#[derive(Clone, Copy)] // TODO: 이게 의미하는게 무엇일까?
struct Dog {
    age: u8,
}
