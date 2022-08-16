struct Vector {
	size: i32,
	capacity: i32,
	arr: [i32; 16],
}

#[allow(dead_code)]
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
		self.arr[self.size as usize]
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

	vec.push(1);
	vec.push(2);
	vec.push(3);
	vec.print();
	
	println!("pop {}", vec.pop());
	println!("pop {}", vec.pop());
	println!("pop {}", vec.pop());
	
	vec.print();
}
