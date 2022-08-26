fn main() {
    println!("CHARACTER TABLE:");

    print_char_table();    

    println!("");
}

fn print_char_table () {
    for i in 8..32 {
        println!("");

        for a in (i*4)..((i+1)*4) {
            if a < 100 {
                print!(" {}  : {} \t|", a, a as u8 as char);
            } else {
                print!(" {} : {} \t|", a, a as u8 as char);
            }
        }        
    }
}