Rustofi
=======

Rustofi is a library for building GUI applications using Rofi commands. 
It supports getting user selection, user entry and can run associated callbacks on item selection. 

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
