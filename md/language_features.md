# Language Features

## Traits

A trait is a collection of methods defined for an unknown type: Self.
They can access other methods declared in the same trait.
[RustByExample](https://doc.rust-lang.org/rust-by-example/trait.html#traits)

- The owner of a type can implement any traits on that type
- The owner of a trait can implement that trait on any type

- you can implement traits you defined on types you didn't define
- you can implement traits you didn't define on your types you defined
- But you cannot implement traits you didn’t define on types you didn’t define

### Traits versus Interfaces

Unlike interfaces in languages like Java, C# or Scala, new traits
can be implemented for existing types whereas interfaces can only
be implemented for your own classes.

- The owner of a class can implement any interfaces on that type
- {red}The owner of an interface *cannot* implement that
interface on a class they did not define{/red} - {green}in Rust you can{/green}.

A trait is not the same as a C# extension method.
