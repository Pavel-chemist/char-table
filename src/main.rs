fn main() {
    println!("CHARACTER TABLE:");

    print_char_table();    

    println!("");
}

fn print_char_table () {
    let mut a: u8;
    for j in 0..16 {
        for i in 0..8 {

            a = i * 16 + j;

            if a < 10 {
                print!(" {}   : {} \t|", a, 26 as char);
            } else if a < 32 {
                print!(" {}  : {} \t|", a, 26 as char);
            } else if a < 100 {
                print!(" {}  : {} \t|", a, a as char);
            } else if a < 128 {
                print!(" {} : {} \t|", a, a as char);
            } else {
                print!(" {} : {} \t|", a, 26 as char);
            }
        }
        println!("");
    }
}