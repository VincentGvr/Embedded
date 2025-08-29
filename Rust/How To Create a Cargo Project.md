Cargo handles dependencies in Rust

```
$ cargo new hello_cargo
$ cd hello_cargo
```

When creating a cargo, it creates : 
- a .toml file containing the dependencies and package info
- an src folder containg the source code 

To build a project : 

` cargo build `

It creates : 
- a .lock file
- a target folder containing a debug folder

To execute : `./target/debug/hello_cargo`

To compile and run : 

`cargo run`

To check code : 

`cargo check`

To create a release : 

`cargo build --release`

To clone repo and build : 

$ git clone example.org/someproject
$ cd someproject
$ cargo build
