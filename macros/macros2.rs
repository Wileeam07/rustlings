// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// I AM NO DONE

//fn main() {
    //my_macro!();
//}

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}