Rustofi
=======

Rustofi is a library for building RUI (Rofi User Interface) applications.
It supports getting user selection, user entry and can run associated callbacks on item selection.

# Usage
Add this to your `Cargo.toml`
```toml
[dependencies]
rustofi = "0.2.2"
```

then to use in your Rust 2018 application you'll probably want these imports
```rust
use rustofi::{AppRoot, RustofiOption, RustofiOptionType};
use rustofi::window::{Window, Location};
```

# Example

## Simple
The example `simple` just displays a rofi window in a loop and returns the text selected.
```bash
git clone https://github.com/krruzic/rustofi
cd rustofi
cargo run --example simple
```

## Todo App
A more complicated example `todo_app` is a persistent Todo List that can
- create new Todos
- delete Todos
- mark Todos as finished

```bash
git clone https://github.com/krruzic/rustofi
cd rustofi
cargo run --example todo_app
```
