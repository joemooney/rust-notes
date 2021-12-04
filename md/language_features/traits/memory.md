# Iterator

You implement an iterator for some custom collection type you define. Maybe you have a company struct which contains a vector
list of employees but the list may contain former employees 
who should not be included in any iteration of the employees. 
So company.employees may be an iterator.
Implementing the Iterator trait means implementing the `next` function.
You struct will need some index to point to the current location.
You 

```rust,editable
impl<'a> Iterator for Iter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // index or some pointer to current item
        while self.index + 1 < self.vec.len() {
            let prev = self.vec[self.index];
            self.index += 1;
            if (*self.f)(prev) {
                return Some(self.vec[self.index]);
            }
        }
        None
    }
}
```
