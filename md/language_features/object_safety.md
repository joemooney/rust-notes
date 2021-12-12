## Object safety

Only an `object-safe trait` can be passed as a `trait object` parameter to a function.

All parameters to a function must have a known size a.k.a `Sized`.

Traits are `?Sized`. They have an unknown size since the sizes may vary for different concrete types.

If a `trait method` returns `Self`, then from the perspective of the trait, the method is returning a type of varying size depending on the size of the concrete type.
A function cannot return something of unknown size. 
It is fine to declare such a method for a trait because any concrete type that implements the Trait will need to be `Sized` or else compiler will know to complain.

But it is not okay to pass a reference to an object of that trait `dyn &T` as an argument to some other function since that other function may call the method returning `Self`.

Side Note: Rust does not look at the implementation to see if the method is actually being called - if a parameter is okay to be passed into a function then it should be okay to call any of its methods in the function logic.

) and that size may vary which is not allowed (we need to know how many bytes on the stack to allocate for the return value of a function before we call the function).

So, when a type 

A trait is *object safe* if all the methods defined in the trait have the following properties:
    - The return type isn’t Self.
    - There are no generic type parameters.

For a trait to be **object safe** the underlying concrete type should be referred to by a ```&self``` reference and not by ```self``` value. A reference is a known size (it is the same size, the size of a pointer, regardless of type) but a value is unknown (it varies depending on the type).

```
trait T1 {
    fn foo(self); // not object safe,
                  // size of object is unknowable at compile time.
}

trait T2 {
    fn foo(&self); // object safe
}
```