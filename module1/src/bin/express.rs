// Make it work with two ways
fn main() {
    let v = 1;
 
    assert_eq(v, 3);
 
    println!("Success!");
 }

fn assert_eq (x:i32,n:i32){
     let v = {
            let mut x = 1;
            x += 2
    };
}
