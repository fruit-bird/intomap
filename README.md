# `#[derive(IntoMap)]`

An implementation of a serialization-like trait. Inspired by [Chapter 9 of "The Complete Rust Programming Reference Guide"](https://github.com/PacktPublishing/The-Complete-Rust-Programming-Reference-Guide/tree/master/Chapter09)

Check out my [blog post](https://fruit-bird.github.io/posts/derive-macros/) explaining this. It cover more concepts than the aforementioned book, like derive attributes

## Usage
```rust
#[derive(IntoMap)]
struct User {
    name: &'static str,
    #[intomap(ignore)]
    id: usize,
    #[intomap(rename = "online")]
    active: bool,
}

let user = User {
    name: "Jimothy",
    id: 0,
    active: true,
};

let user_map = user.as_map();
println!("{:#?}", user_map);
```
```
BTreeMap {
    "name": "Jimothy",
    "online": "true",
}
```

## What is This?
Started with trying to learn how derive macros work. Ended with me starting a blog
