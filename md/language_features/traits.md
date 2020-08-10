# Traits

## Object safety
A trait is **object safe** if the concrete is reffered by ```&self``` reference tnd not by ```self``` value.

```
trait T1 {
    fn foo(self); // not object safe,
                  // size of object is unknowable at compile time.
}

trait T2 {
    fn foo(&self); // object safe
}
```
