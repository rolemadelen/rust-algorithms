fn selection_sort<T: Ord>(arr: &mut [T]) {
	if arr.is_empty() { return; }

	let len = arr.len();
	for left in 0..len {
		let mut min = left;

		for right in (left+1)..len {
			if arr[right] < arr[min] {
				min = right;
			}
		}

		arr.swap(min, left);
	}
}

fn main() {
	let mut vec1 = vec![4, 5, 3, 2, 1];
	let mut vec2 = vec![1, 2, 3, 4, 5];
	let mut vec3 = vec![5, 4, 3, 2, 1];

	selection_sort(&mut vec1);
	for i in 0..vec1.len() {
		print!("{} ", vec1[i]);
	}
	println!();

	selection_sort(&mut vec2);
	for i in 0..vec2.len() {
		print!("{} ", vec2[i]);
	}
	println!();

	selection_sort(&mut vec3);
	for i in 0..vec3.len() {
		print!("{} ", vec3[i]);
	}
	println!();
}
