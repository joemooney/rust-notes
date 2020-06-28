# Memory

## References
https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait

## Passing Arguments and Returning Values
- Functions must return a known size of memory.
- Functions parameters must be of a known size of memory.
- Local variables must be of a known size.
- Functions must return a concrete *sized* type - **unlike other languages**.
- A concrete type may be sized or unsized.
- Traits are not concrete - their size is unknown.
- {red}You cannot return Traits{/red}.
- {blue}Place Traits in a Box{/blue}.
- A Box is a reference to heap memory.
- A reference has a known size - it is a pointer.
- Rust prefers code to be explicit if memory is heap or stack.
- `dyn` indicates that memory is in the heap.
- For a generic function we need to ensure that the arguments are sized.
- Rust defaults all generic type parameters to be `Sized`. So `fn generic_fn<T>(x: T) -> T { ... }` is the equivalent of `fn generic_fn<T: Sized>(x: T) -> T { ... }`. But {yellow}you may not want that{/yellow}. So `fn generic_fn<T: ?Sized>(x: &T) -> u32 { ... }` allows you to call generic_fn("abc") where `T == str` which is an unsized type `str` but the argument is `&T` which is sized so all is good.
- `trait` has an implicit `?Sized` bound on `Self`.
- `trait`s can be implemented for sized __and__ unsized types. For example, any trait which only contains methods which only take and return Self by reference can be implemented for unsized types.
- If you want to return `Self` by value or accept `Self` as an argument by value, you need to either bind the trait with `Sized` or bind the function with `Sized`: `trait A: Sized { ... }` or 
```rust
trait WithConstructor {
    fn new_with_param(param: usize) -> Self;

    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new_with_param(0)
    }
}
```