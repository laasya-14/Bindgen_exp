pub mod foo;

use foo::{T, print_value};

fn main() {
    unsafe{
        let t:T = T{x: 1, y: 2};
        print_value(t);
    }
    
}