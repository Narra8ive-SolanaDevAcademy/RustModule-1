fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn() //null
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!("")
}

//Second method COuld be
fn never_return_fn() -> ! {
    loop {}
}

//Third method
fn never_return_fn() -> ! {
    std::process::exit(1);
}
