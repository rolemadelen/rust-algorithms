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
	let mut vec1 = vec![45, 23, -13, 93, 84, -173, 10, 17, 711];
	let mut vec2 = vec!["hello", "C++", "Spoon", "Way", "Duck", "Ginger Ale"];

	println!("Sort numbers in ascending order.");
	println!("Before: {:?}", vec1);
	selection_sort(&mut vec1);
	println!("After: {:?}\n", vec1);

	println!("Sort strings alphabetically.");
	println!("Before: {:?}", vec2);
	selection_sort(&mut vec2);
	println!("After: {:?}", vec2);
}
