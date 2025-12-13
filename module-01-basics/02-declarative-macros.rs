// Macro to print "Hello, world!"
// However, this is a declarative macro, not a function
// and this is usually useless unless for educational purposes
macro_rules! hello_world {
  () => {
    println!("Hello, world!");
  }
}

// Macro Identity Explanation:
// - $func_name:ident captures an identifier (function name)
// - stringify!() converts the identifier to a string literal
// - This macro generates function definitions at compile time
// - Each invocation creates a new function with the specified name
// - See fn currying pattern similar to on Javascript
macro_rules! create_function {
  ($func_name:ident) => {
    fn $func_name() {
      println!("You called {:?}()", stringify!($func_name));
    }
  }
}

// Macro Expressions and Usage
macro_rules! add {
  ($a: expr, $b: expr) => {
    $a + $b
  }
}

fn main() {
  hello_world!();

  println!();
  
  // Macro Identity Usage
  println!("Using identity macro to create functions:");
  create_function!(foo);
  foo(); // Above function creates foo fn and then called here

  create_function!(bar);
  bar(); // Above function creates foo fn and then called here

  println!();

  // Macro Expressions Usage
  println!("Using expression macro:");
  let sum = add!(5, 10);
  println!("The sum is: {}", sum);
}