fn counting_sort(arr: &mut [u32], max_val: usize) {
	let mut occurrences: Vec<usize> = vec![0; max_val + 1];

	for &data in arr.iter() {
		occurrences[data as usize] += 1;
	}

	let mut i = 0;
	for (data, &number) in occurrences.iter().enumerate() {
		for _ in 0..number {
			arr[i] = data as u32;
			i += 1;
		}
	}
}

fn main() {
	let mut vec1 = vec![1, 2, 3, 4, 5];
	let mut vec2 = vec![10, 22, 130, 2, 55];

	counting_sort(&mut vec1, 5);
	for i in 0..vec1.len() {
		print!("{} ", vec1[i]);
	}
	println!();

	counting_sort(&mut vec2, 130);
	for i in 0..vec2.len() {
		print!("{} ", vec2[i]);
	}
	println!();
}
