# Vector

- [ ] Implement a vector (mutable array with automatic resizing):
    - [x] size() - number of items
    - [x] New raw data array with allocated memory (default = 16, or power of 2 - 16, 32, 64, ...)
    - [x] capacity() - number of items it can hold
    - [x] is_empty()
    - [x] at(index) - returns item at given index, blows up if index out of bounds
    - [x] push(item)
    - [x] insert(index, item) - inserts item at index, shifts that index's value and trailing elements to the right
    - [x] prepend(item) - inserts item at index 0
    - [x] pop() - remove from end, return value<
    - [x] delete(index) - delete item at index, shifting all trailing elements left
    - [x] remove(item) - looks for value and removes index holding it (even if in multiple places)
    - [x] find(item) - looks for value and returns first index with that value, -1 if not found
    - [ ] resize(new_capacity) // private function
        - [ ] when you reach capacity, resize to double the size
        - [ ] when popping an item, if size is 1/4 of capacity, resize to half

## Time complexity
- O(1) to add/remove at end (amortized for allocations for more space), index, or update
- O(n) to insert/remove elsewhere
  
## Space Complexity
- contiguous in memory, so proximity helps performance
- space needed = (array capacity, which is >= n) * size of item, but even if 2n, still O(n)
  