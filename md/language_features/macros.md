# Macros

References

- [Little Book of Macros](https://danielkeep.github.io/tlborm/book/index.html)

- Rust macros are good since they generally:
    - avoid unintended side-effects
    - do not result in bizarre error messages
    - are not simple text substitutions, they must be parseable
    - are subject to much more rigorous analysis during compilation
    - are an elegant way to add a lot of power to a program, with limited risk
    - are well thought out and tightly integrated into the ecosystem 
    - have few detractors within the Rust community
    - can only expand to what is expected at that position (e.g. expression or type definition)
    - can never expand to an incomplete or invalid construct (e.g. unbalanced parenthesis). In C you could `#define Foo Bar(` that is impossible in Rust.
    - can be recursive (up to 32 levels by default)
    - are actually a big selling point of the language


