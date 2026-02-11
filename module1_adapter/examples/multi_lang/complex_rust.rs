macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

trait Addable {
    fn add(&self, other: &Self) -> Self;
}

impl Addable for i32 {
    fn add(&self, other: &i32) -> i32 {
        self + other
    }
}

fn generic_add<T: Addable>(a: &T, b: &T) -> T {
    a.add(b)
}

fn main() {
    say_hello!();
    say_hello!("World");
    
    let x = 5;
    let y = 3;
    let result = generic_add(&x, &y);
    println!("{}", result);
    
    let numbers = vec![1, 2, 3];
    for n in numbers.iter() {
        println!("{}", n);
    }
}
