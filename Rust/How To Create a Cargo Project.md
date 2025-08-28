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

To use a library : 

use std::io;

To define variables : by default immutable 

let apples = 5; // immutable
let mut bananas = 5; // mutable

To call functions : 
io::stdin().read_line(&mut guess);

Result of a call can be of two states : 
  - Ok
  - Err
To catch Err :   .expect("");
If forgotten : warning when compiling ;

To add a dependency : edit the Cargo.toml : 
vim Cargo.toml 
"a" to edit with cursor
Esc to escape edition 
:w to save 
:q to quit vim 
