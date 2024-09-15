// enum Option<T> {
//     Some(T),
//     None
// }

enum Result<T, E> {
    Ok(T),
    Err(E)
}

pub fn option_example(x: i32) -> Option<String> {
    match x > 2 {
        true => Some(String::from("result")),
        false => None,
    }
}

// structs
struct Rectangle<T> {
    height: T,
    width: T
}

struct Cuboid<T, U, V> {
    length: T,
    depth: U,
    breadth: V
}

pub fn struct_example() {
    let rec = Rectangle{height: 1, width: 1};
    let cuboid = Cuboid{breadth: 5, depth: 3.45, length: 34};
}

// functions

// Not allowed cause * is restricted to only Numbers
// pub fn sum_of_numbers<T>(num1: T, num2: T) -> T {
//     num1 * num2
// }

// Restricting Generics
pub fn sum_of_numbers<T: std::ops::Mul<Output = T>>(num1: T, num2: T) -> T {
    num1 * num2
}