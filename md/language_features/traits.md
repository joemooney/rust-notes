# Traits

## Object safety
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
