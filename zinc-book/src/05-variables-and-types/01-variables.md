# Variables

As it was said before, Zinc is mostly about safety and security. Thus,
variables are immutable by default. If you are going to change their values,
you must explicitly mark them as mutable. It protects your data from accidental
mutating where the compiler is unable to check your intentions.

```rust,no_run,noplaypen
fn test() {
    let x = 0;    
    x = 42; // compile error: mutating an immutable variable

    let mut y = 0;
    y = 42; // ok
}
```

> If you are familiar with Rust, you will not have any trouble understanding this
> concept, since the syntax and semantics are almost identical. However, pattern
> matching and destructuring are not implemented yet.

Immutable variables are similar to constants. Like with constants, you cannot
change the immutable variable value. However, constants cannot infer their type
and you must specify it explicitly.

> In contrast to Rust, variables can only be declared in functions. If you need a
> global variable, you should declare a constant instead. This limitation is devised to
> prevent unwanted side effects, polluting the global namespace, and bad code design.

```rust,no_run,noplaypen
const VALUE: field = 0;

fn test() {
    let variable = VALUE;
}
```

Variable shadowing can be a convenient feature, but Zinc is going to enforce
warning-as-error development workflow, forbidding shadowing as a potentially
unsafe trick. You should use mutable variables and type suffixes if you want
to have several variables with similar logical meaning.

```rust,no_run,noplaypen
fn test() {
    let mut x = 5;
    {        
        let x = 25; // compile error: redeclared variable 'x'
    };    
    let x = 25; // compile error: redeclared variable 'x'

    x = 25; // ok
}
```
