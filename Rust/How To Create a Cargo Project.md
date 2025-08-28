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

To execute :
`./target/debug/hello_cargo`
