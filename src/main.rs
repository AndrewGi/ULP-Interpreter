mod ulp;
fn main() {
    let mut f = std::fs::File::open("1.o").expect("File not found!");
    let elf = ulp::ELF::from_file(&mut f);
    println!("is good: {}", elf.header.is_valid());
    println!("{}", elf.to_string());
}
