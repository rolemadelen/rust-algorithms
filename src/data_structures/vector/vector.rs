struct Vector {
	size: i32,
	capacity: i32,
	arr: [i32; 16],
}

impl Vector {
	pub fn new() -> Vector {
		Vector {
			size: 0,
			capacity: 16,
			arr: [0; 16],
		}
	}

	pub fn size(&mut self) -> i32 {
		return self.size;
	}

	pub fn capacity(&mut self) -> i32 {
		return self.capacity;
	}

	pub fn is_empty(&mut self) -> bool {
		return self.size == 0;
	}

	pub fn push(&mut self, item: i32) { 
		if self.size == self.capacity {
			println!("vector is full.");
			return;
		}

		self.arr[self.size as usize] = item;
		self.size += 1;
	}

	pub fn pop(&mut self) -> i32 {
		if self.size == 0 { 
			println!("vector is empty.");
			return -1; 
		}

		self.size -= 1;
		let val = self.arr[self.size as usize];

		return val;
	}

	pub fn prepend(&mut self, item: i32) {
		let mut i = self.size as usize;
		while i > 0 {
			self.arr[i] = self.arr[(i-1) as usize];
			i -= 1;
		}

		self.arr[0] = item;
		self.size += 1;
	}

	pub fn insert(&mut self, index: usize, item: i32) {
		if self.size == self.capacity {
			println!("vector is full.");
			return;
		}

		if index == self.size as usize {
			self.push(item);
			return;
		}

		if index == 0 {
			self.prepend(item);
			return;
		}

		let mut i = self.size as usize;
		while i > index {
			self.arr[i] = self.arr[(i-1) as usize];
			i -= 1;
		}

		self.arr[index] = item;
		self.size += 1;
	}

	pub fn delete(&mut self, index: usize) {
		if self.size == 0 { 
			println!("vector is empty.");
			return; 
		}

		if index >= self.size as usize {
			println!("wrong index.");
			return;
		}

		for i in index..(self.size as usize) {
			self.arr[i as usize] = self.arr[(i+1) as usize];
		}

		self.size -= 1;
	}

	pub fn remove(&mut self, item: i32) {
		let mut index = 0;
		while index != -1 {
			index = self.find(item);
			if index == -1 { break; }
			self.delete(index as usize);
		}
	}

	// linear search
	pub fn find(&mut self, item: i32) -> i32 {
		let mut index = -1;
	
		for i in 0..self.size {
			if self.arr[i as usize] == item {
				index = i; 
				break;
			}
		}
		
		return index;
	}
	
	// blows up if index out of bounds
	pub fn at(&mut self, index: i32) -> i32 {
		return self.arr[index as usize];
	}

	pub fn print(&mut self) {
		for i in 0..(self.size) {
			print!("{} ", self.arr[i as usize]);
		}
		println!();
	}
}



fn main() {
	let mut vec = Vector::new();
	println!("{}", vec.size());
	println!("{}", vec.capacity());
	println!("{}", vec.is_empty());

	// for i in 0..5 {
	// 	vec.push(i);
	// }

	// vec.print();

	// vec.insert(1, 7);
	// vec.print();

	// vec.insert(6, 10);
	// vec.print();

	// vec.insert(3, 17);
	// vec.print();

	// vec.prepend(99);
	// vec.print();

	// vec.insert(0, 123);
	// vec.print();
	
	// println!();
	// println!();
	
	// println!("Delete at index 0");
	// vec.delete(0);
	// vec.print();

	// println!("Delete until there's only 1 element left in the vector");
	// while vec.size() > 1 {
	// 	vec.delete(0);
	// 	vec.print();
	// }
	// vec.delete(10);
	// vec.print();

	// vec.delete(0);
	// vec.print();

	vec.push(1);
	vec.push(3);
	vec.push(1);
	vec.push(4);
	vec.push(5);
	vec.push(6);
	vec.push(3);
	vec.print();

	vec.remove(1);
	vec.print();

	vec.remove(3);
	vec.print();

	println!("{}", vec.size());
}
