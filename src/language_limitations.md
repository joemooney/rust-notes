# Language Limitations

## Optional with default Parameters

There are pros and cons to supporting python equivalent of:
```python
def foo(bar=20)
```

If we call `foo(bar)` and bar is None then the default does not
apply and 
Although in this case there is no way not to provide the argument,
you can call foo(None). This has arguable benefit of allowing foo
to be called with an argument with value None and getting the
default, whereas in python you would not know whether the argument
was supplied and if it had None you would need to check for it.

```rust
fn foo(bar: Option<usize>) {
    let x = bar.unwrap_or(20); //Default value is 20
}
```


## Different signatures
C# lets you do this:
```C#
public static int Foo(int x = 0) => 20 * x;
public static int Foo() => 10;
```
