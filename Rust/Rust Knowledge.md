To use a library : 

use std::io;

To define variables : by default immutable 
```
let apples = 5; // immutable
let mut bananas = 5; // mutable
```
We can defined constants across all scope of the app. Constants cannot be the result of a calculation, it has to be defined at definition. 

```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

If a varaible is defined inside of { }, it can change as a value : 

```
let x = x + 1;
{
  let x = x * 2;
  println!("The value of x in the inner scope is: {x}");
}
```

To define many variables type : 

`let tup: (i32, f64, u8) = (500, 6.4, 1);`

To define an array with type and number of items : 

`let a: [i32; 5] = [1, 2, 3, 4, 5];`

To call functions : 
`io::stdin().read_line(&mut guess);`

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
