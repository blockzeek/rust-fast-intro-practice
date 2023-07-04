mod module1;
mod module2;

pub fn run() {
    module1::print_a_to_cap_z();
    module2::module2::print_cap_a_to_z();
}
