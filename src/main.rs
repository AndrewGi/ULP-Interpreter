mod ulp;
fn main() {
    let mut f = std::fs::File::open("1.o").expect("File not found!");
    let elf = ulp::ELF::from_file(&mut f);
    println!("good elf magic number: {}", elf.header.is_valid());
}
