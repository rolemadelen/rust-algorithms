use std::cmp::PartialEq;

pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    None
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    println!("Array: {:?}", vec);
    println!("1 at position: {:?}",linear_search(&1, &vec));
    println!("5 at position: {:?}",linear_search(&5, &vec));
    println!("10 at position: {:?}\n",linear_search(&10, &vec));

    let vec2 = vec!["Rust", "Hello", "World", "Fine", "Sing"];
    println!("Array: {:?}", vec2);
    println!("Find 'Rust': {:?}", linear_search(&"Rust", &vec2));
    println!("Find 'Apple': {:?}", linear_search(&"Apple", &vec2));
    println!("Find 'Sing': {:?}", linear_search(&"Sing", &vec2));
}