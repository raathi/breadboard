mod demo {//when crate linked, this mod in the same name of lib(libdemo.rslib) will be imported.
    pub fn public_function() {
        println!("called rary's `public_function()`");
    }

    fn private_function() {
        println!("called rary's `private_function()`");
    }

    pub fn indirect_access() {
        print!("called rary's `indirect_access()`, that\n> ");

        private_function();
    }
}
