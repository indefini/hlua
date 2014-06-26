## rust-hl-lua

This library is a high-level binding for Lua 5.2. You don't have access to the Lua stack, all you can do is read/write variables (including callbacks) and execute Lua code.

[![Build Status](https://travis-ci.org/Tomaka17/rust-hl-lua.svg?branch=master)](https://travis-ci.org/Tomaka17/rust-hl-lua)

*Important*: the library is a bit broken for the moment because of a bug in the rust compiler (see https://github.com/mozilla/rust/issues/13853 and https://github.com/mozilla/rust/issues/14889).
The library should be working but things pushed on the Lua stack are not popped. I don't really know whether this is simply a memory leak or if it can cause crashes.

### How to install it?

Add this to the `Cargo.toml` file of your project

    [dependencies.rust-hl-lua]
    git = "https://github.com/Tomaka17/rust-hl-lua"

If you don't use cargo yet, just compile with `rustc src/lib.rs`. You can also generate the docs with `rustdoc src/lib.rs`.

In the future, this library will directly include the Lua C library if cargo allows this.

### How to use it?

    extern crate rust-hl-lua;
    use rust-hl-lua::Lua;

The `Lua` struct is the main element of this library. It represents a context in which you can execute Lua code.

    let mut lua = Lua::new();     // mutable is mandatory

#### Reading and writing variables

    lua.set("x", 2);
    lua.execute("x = x + 1").unwrap();
    let x: int = lua.get("x").unwrap();  // x is equal to 3

Reading and writing global variables of the Lua context can be done with `set` and `get`.
The `get` function returns an `Option<T>` 

The base types that can be read and written are: `int`, `i8`, `i16`, `i32`, `uint`, `u8`, `u16`, `u32`, `f32`, `f64`, `bool`, `String`.

If you wish so, you can also add other types by implementing the `Pushable` and `CopyReadable`/`ConsumeReadable` traits.

#### Executing Lua

    let x: uint = lua.execute("return 6 * 2;").unwrap();    // equals 12

The `execute` function returns a `Result<Readable, ExecutionError>`.

#### Writing functions

*(note: functions with parameters are not yet supported)*

    fn add(a: int, b: int) -> int {
        a + b
    }
    
    lua.set("add", add);
    lua.execute("local c = add(2, 4)");
    lua.get("c").unwrap();  // return 6
    
In Lua, functions are exactly like regular variables.

#### Manipulating Lua tables

Manipulating a Lua table can be done by reading a `LuaTable` object.

    let mut table: rust-hl-lua::LuaTable = lua.get("a").unwrap();

You can then iterate through the table with the `.iter()` function. Note that the value returned by the iterator is an `Option<()>`.

    for elem in table.iter() {
        match elem {
            &Some(key, value) => ...,
            &None => fail!("Could not read key or value because they are of the wrong type")
        }
    }

You can also retreive individual values: *(note: this is not yet supported)*

    let x = table.get("a").unwrap();

### Roadmap

 - [ ] Reading/writing inside Lua tables
   - [x] Iterating through tables
   - [ ] Reading elements
 - [ ] Reading/writing containers
 - [ ] Writing functions
   - [x] Basic support
   - [ ] Functions with parameters
   - [ ] Static closures
 - [ ] Reading or loading a function and calling it later
 - [ ] Access to the metatables of tables and user data
 - [ ] Access to the registry
 - [ ] Better user data
