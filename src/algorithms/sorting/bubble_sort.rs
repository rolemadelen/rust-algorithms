fn bubble_sort<T: Ord>(arr: &mut [T]) {
	if arr.is_empty() { return; }

	let mut swapped: bool;
	let n = arr.len();

	for i in 0..n {
		swapped = false;

		for j in 0..n-(i+1) {
			if arr[j] > arr[j+1] {
				arr.swap(j, j+1);
				swapped = true;
			}
		}

		if !swapped { return; }
	}
}

fn main() {
	let mut vec1 = vec![4, 5, 3, 2, 1];
	let mut vec2 = vec![1, 2, 3, 4, 5];
	let mut vec3 = vec![5, 4, 3, 2, 1];

	bubble_sort(&mut vec1);
	for i in 0..vec1.len() {
		print!("{} ", vec1[i]);
	}
	println!();

	bubble_sort(&mut vec2);
	for i in 0..vec2.len() {
		print!("{} ", vec2[i]);
	}
	println!();

	bubble_sort(&mut vec3);
	for i in 0..vec3.len() {
		print!("{} ", vec3[i]);
	}
	println!();
}
