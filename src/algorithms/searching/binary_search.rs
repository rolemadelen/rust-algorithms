use std::cmp::Ordering::{Less, Equal, Greater};

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
	let mut is_asc = true;
	if arr.len() > 1 {
		is_asc = arr[0] < arr[arr.len() - 1];
	}

	let mut left = 0;
	let mut right = arr.len();

	while left < right {
		let mid = left + (right - left) / 2;

		if is_asc {
			match item.cmp(&arr[mid]) {
				Less => right = mid,
				Equal => return Some(mid),
				Greater => left = mid + 1,
			}
		} else {
			match item.cmp(&arr[mid]) {
				Less => left = mid + 1,
				Equal => return Some(mid),
				Greater => right = mid,
			}
		}
	}

	None
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    println!("Array: {:?}", vec);
    println!("1 at position: {:?}",binary_search(&1, &vec));
    println!("5 at position: {:?}",binary_search(&5, &vec));
    println!("10 at position: {:?}\n",binary_search(&10, &vec));

    let vec2 = vec![6, 5, 4, 3, 2, 1];
    println!("Array: {:?}", vec2);
    println!("1 at position: {:?}",binary_search(&1, &vec2));
    println!("5 at position: {:?}",binary_search(&5, &vec2));
    println!("10 at position: {:?}\n",binary_search(&10, &vec2));

    let mut vec3 = vec!["Rust", "Hello", "World", "Fine", "Sing"];
	vec3.sort();
    println!("Array: {:?}", vec3);
    println!("Find 'Rust': {:?}", binary_search(&"Rust", &vec3));
    println!("Find 'Apple': {:?}", binary_search(&"Apple", &vec3));
    println!("Find 'Sing': {:?}", binary_search(&"Sing", &vec3));
}

