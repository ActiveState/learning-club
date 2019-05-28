// Shea's take on Chapter 1's exercises 1 and 2 in Rust.

// Identity
fn id<T>(x: T) -> T {
    x
}

// Composition
fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

// Dummy data to demo compisition
struct Foo;
struct Bar;
struct Baz;

fn first(_: Foo) -> Bar {
    Bar {}
}

fn second(_: Bar) -> Baz {
    Baz {}
}

fn main() {
    // Print result of identity function
    println!("{}", id("Hello, world!"));
    println!("{}", id(123_456));

    // Compose first and second, get Baz from Foo
    let c = compose(first, second);
    let _: Baz = c(Foo {});
}
