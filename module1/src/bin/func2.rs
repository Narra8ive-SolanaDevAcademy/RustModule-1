// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
  let n1 = 2;
  let n2 = 3;
  let s = add_func(n1,n2);
  display_func(s);
}

fn add_func(x1:i32,x2:i32)->i32{
  return x1+x2;
}

fn display_func(q:i32){
  println!("{:?}",q);
}
