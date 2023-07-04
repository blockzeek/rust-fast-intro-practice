pub fn print_a_to_cap_z() {
    for ch in ('Z'..='a').rev() {
        print!("{} ", ch);
    }
    println!();
}
