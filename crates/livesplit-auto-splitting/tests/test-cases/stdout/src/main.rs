#[no_mangle]
pub extern "C" fn configure() {
    println!("Printing from the auto splitter");
    eprintln!("Error printing from the auto splitter");
}

fn main() {}
