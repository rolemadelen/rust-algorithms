# Vector

- [ ] Implement a vector (mutable array with automatic resizing):
    - [x] New raw data array with allocated memory (default = 16, or power of 2 - 16, 32, 64, ...)
    - [x] size() - return number of items in a vector.
    - [x] capacity() - return number of items it can hold.
    - [x] is_empty() - returns true if empty, else false.
    - [x] at(index) - returns item at given index; blows up if index-out-of-bound occurs.
    - [x] push(item) - inserts item at the end of the vector.
    - [x] insert(index, item) - inserts item at index; shifts that index's value and trailing elements to the right.
    - [x] prepend(item) - inserts item at index 0.
    - [x] pop() - remove and return the last item in the vector.
    - [x] delete(index) - removes item at index and shift all trailing elements to the left.
    - [x] remove(item) - looks for the item and removes index holding it (even if in multiple places).
    - [x] find(item) - looks for the item and returns first index with that value, -1 if not found.
    - [ ] resize(new_capacity) // private function
        - [ ] when you reach the capacity, double the container size.
        - [ ] when current size becomes 1/4th of the capacity, halve the container size.

## Time complexity
- O(1) to add/remove at end (amortized for allocations for more space), index, or update
- O(n) to insert/remove elsewhere
  
## Space Complexity
- contiguous in memory, so proximity helps performance
- space needed = (array capacity, which is >= n) * size of item, but even if 2n, still O(n)
  