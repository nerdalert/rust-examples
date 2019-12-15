mod lib;

fn main() {
    // call the module located in this file
    kewlmod::hello_from_kewlmod();
    // call the function from lib.rs file
    lib::new_src();
}

mod kewlmod {
    pub fn hello_from_kewlmod() {
        println!("Hello from kewlmod");
        prvt_func()
    }

    fn prvt_func() {
        println!("Private functions can be called within the module too");
    }
}

/*
The output is:
-------------
Hello from kewlmod
Private functions can be called within the module too
Hello from another src file
*/
