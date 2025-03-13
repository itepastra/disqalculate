use std::io::stdin;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("disqalculate/include/disqalc.h");

        fn init_calculator();

        fn do_calculation(input: String) -> String;
    }
}

fn main() {
    ffi::init_calculator();
    let mut buf = String::new();
    loop {
        let _ = stdin().read_line(&mut buf);
        println!(
            "the result of {} was {}",
            buf.trim(),
            ffi::do_calculation(buf.clone())
        );
        buf.clear()
    }
}
