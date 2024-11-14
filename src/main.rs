pub mod foo;

use foo::{T,get_value, print_value};

fn main() {
    unsafe{
        let t:T = get_value();
        print_value(t);
    }
    
}