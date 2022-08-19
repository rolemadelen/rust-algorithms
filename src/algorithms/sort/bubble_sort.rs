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
	let mut vec1 = vec![45, 23, -13, 93, 84, -173, 10, 17, 711];
	let mut vec2 = vec!["hello", "C++", "Spoon", "Way", "Duck", "Ginger Ale"];

	println!("Sort numbers in ascending order.");
	println!("Before: {:?}", vec1);
	bubble_sort(&mut vec1);
	println!("After: {:?}\n", vec1);

	println!("Sort strings alphabetically.");
	println!("Before: {:?}", vec2);
	bubble_sort(&mut vec2);
	println!("After: {:?}", vec2);
}