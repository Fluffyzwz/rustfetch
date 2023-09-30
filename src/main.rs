mod info;
mod ascii;
fn main() {

ascii::print_art();
info::print_distro();
info::print_kernel();
info::print_ram();
info::print_cpu();
ascii::print_blocks();

}
