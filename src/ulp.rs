fn u16_from_slice(bytes: &[u8]) -> u16 {
    debug_assert!(
        bytes.len() == 2,
        "passed in {} bytes instead of 2 to u16_from_slice",
        bytes.len()
    );
    bytes[0] as u16 | ((bytes[1] as u16) << 8)
}
fn u32_from_slice(bytes: &[u8]) -> u32 {
    debug_assert!(
        bytes.len() == 4,
        "passed in {} bytes instead of 4 to u32_from_slice",
        bytes.len()
    );
    u16_from_slice(&bytes[0..2]) as u32 | ((u16_from_slice(&bytes[2..4]) as u32) << 16)
}
#[allow(dead_code)]
fn u64_from_slice(bytes: &[u8]) -> u64 {
    debug_assert!(
        bytes.len() == 8,
        "passed in {} bytes instead of 8 to u64_from_slice",
        bytes.len()
    );
    u32_from_slice(&bytes[0..4]) as u64 | ((u32_from_slice(&bytes[4..8]) as u64) << 32)
}
fn read_null_terminated_string(bytes: &[u8]) -> String {
    for i in 0..bytes.len() {
        if bytes[i] == 0 {
            return String::from_utf8(bytes[..i].to_vec()).expect("Unable to read null-terminated string")
        }
    }
    String::default()
}
pub struct ELFHeader {
    pub magic_number: u32, //0x7f 0x45 0x4c 0x46
    pub is_64_bit: bool,
    pub is_big_endianness: bool,
    pub version: u8,
    pub abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
    pub elf_type: u16,
    pub machine: u16,
    pub elf_version: u32,
    pub entry_pc: u32,
    pub program_header_start: u32,
    pub section_header_start: u32,
    pub flags: u32,
    pub this_size: u16,
    pub program_header_size: u16,
    pub num_of_program_header_entries: u16,
    pub section_size: u16,
    pub num_of_section_header_entries: u16,
    pub shstrtab_index: u16,
}

impl ELFHeader {
    pub fn from_binary(binary: &[u8]) -> ELFHeader {
        ELFHeader {
            magic_number: u32_from_slice(&binary[0x0..0x4]),
            is_64_bit: binary[0x4] == 2,
            is_big_endianness: binary[0x5] == 2,
            version: binary[0x6],
            abi: binary[0x7],
            abi_version: binary[0x8],
            padding: [0; 7],
            elf_type: u16_from_slice(&binary[0x10..0x12]),
            machine: u16_from_slice(&binary[0x12..0x14]),
            elf_version: u32_from_slice(&binary[0x14..0x18]),
            entry_pc: u32_from_slice(&binary[0x18..0x1C]),
            program_header_start: u32_from_slice(&binary[0x1C..0x20]),
            section_header_start: u32_from_slice(&binary[0x20..0x24]),
            flags: u32_from_slice(&binary[0x24..0x28]),
            this_size: u16_from_slice(&binary[0x28..0x2A]),
            program_header_size: u16_from_slice(&binary[0x2A..0x2C]),
            num_of_program_header_entries: u16_from_slice(&binary[0x2C..0x2E]),
            section_size: u16_from_slice(&binary[0x2E..0x30]),
            num_of_section_header_entries: u16_from_slice(&binary[0x30..0x32]),
            shstrtab_index: u16_from_slice(&binary[0x32..0x34]),
        }
    }
    pub fn default() -> ELFHeader {
        ELFHeader::from_binary(&[0u8; 0x34])
    }

    pub fn is_valid(&self) -> bool {
        self.magic_number == 0x464c457f
    }
}

#[allow(dead_code)]
pub struct ELFProgramHeader {
    header_type: u32,
    offset: u32,
    virtual_address: u32,
    physical_address: u32,
    file_image_size: u32,
    memory_size: u32,
    flags: u32,
    alignment: u32,
}
impl ELFProgramHeader {
    fn from_binary(binary: &[u8]) -> ELFProgramHeader {
        ELFProgramHeader {
            header_type: u32_from_slice(&binary[0..0x4]),
            offset: u32_from_slice(&binary[0x4..0x8]),
            virtual_address: u32_from_slice(&binary[0x08..0x0C]),
            physical_address: u32_from_slice(&binary[0x0C..0x10]),
            file_image_size: u32_from_slice(&binary[0x10..0x14]),
            memory_size: u32_from_slice(&binary[0x14..0x18]),
            flags: u32_from_slice(&binary[0x18..0x1C]),
            alignment: u32_from_slice(&binary[0x1C..0x20]),
        }
    }
}

#[allow(dead_code)]
pub struct ELFProgram {
    header: ELFProgramHeader,
}
impl ELFProgram {
    pub fn from(program_header: ELFProgramHeader) -> ELFProgram {
        ELFProgram {
            header: program_header,
        }
    }
}
#[allow(dead_code)]
pub struct ELFSectionHeader {
    name: String,
    name_offset: u32, //Pointer to .shstrtab string
    section_header_type: u32,
    flags: u32,
    addr: u32,
    offset: u32,
    size: u32,
    link: u32,
    info: u32,
    address_align: u32,
    entry_size: u32,
}
impl ELFSectionHeader {
    pub fn from_binary(binary: &[u8]) -> ELFSectionHeader {
        ELFSectionHeader {
            name: String::default(),
            name_offset: u32_from_slice(&binary[0x0..0x4]),
            section_header_type: u32_from_slice(&binary[0x4..0x8]),
            flags: u32_from_slice(&binary[0x8..0xC]),
            addr: u32_from_slice(&binary[0xC..0x10]),
            offset: u32_from_slice(&binary[0x10..0x14]),
            size: u32_from_slice(&binary[0x14..0x18]),
            link: u32_from_slice(&binary[0x18..0x1C]),
            info: u32_from_slice(&binary[0x1C..0x20]),
            address_align: u32_from_slice(&binary[0x20..0x24]),
            entry_size: u32_from_slice(&binary[0x24..0x28]),
        }
    }
    pub fn name_from_shstrtab(&mut self, shstrtab: &[u8]) -> &str {
        self.name = read_null_terminated_string(&shstrtab[self.name_offset as usize..]);
        if self.name.is_empty() {
            self.name = "NULL".to_string();
        };
        &self.name
    }
}
pub struct ELFSection {
    header: ELFSectionHeader,
    data: ::std::vec::Vec<u8>,
}
impl ELFSection {
    pub fn from_entire_elf(section_header: ELFSectionHeader, entire_buffer: &[u8]) -> ELFSection {
        let data = ::std::vec::Vec::from(
            &entire_buffer[section_header.offset as usize .. (section_header.offset + section_header.size) as usize]);
        ELFSection::from_known(
            section_header,
            data
        )
    }
    pub fn from_known(section_header: ELFSectionHeader, data: ::std::vec::Vec<u8>) -> ELFSection {
        ELFSection {
            header: section_header,
            data,
        }
    }
}
pub struct ELF {
    pub header: ELFHeader,
    pub programs: ::std::vec::Vec<ELFProgram>,
    pub sections: ::std::vec::Vec<ELFSection>,
    pub data: ::std::vec::Vec<u8>
}

pub struct ULP {
    pub elf: ELF,
}
impl ELF {
    pub fn from_file(file: &mut ::std::fs::File) -> ELF {
        use std::io::*;

        let ref mut file_buffer = ::std::vec::Vec::new();
        file.read_to_end(file_buffer).expect("unable to read file");
        ELF::from_bytes(&file_buffer)
    }
    pub fn from_bytes(bytes: &[u8]) -> ELF {
        let mut elf = ELF {
            header: ELFHeader::from_binary(&bytes[0..0x34]),
            programs: ::std::vec::Vec::default(),
            sections: ::std::vec::Vec::default(),
            data: ::std::vec::Vec::default()
        };
        //Header
        //Programs
        let mut elf_programs = ::std::vec::Vec::new();
        for i in 0..elf.header.num_of_program_header_entries {
            let index = (elf.header.program_header_start
                + (i * elf.header.program_header_size) as u32) as usize;
            elf_programs.push(ELFProgram::from(ELFProgramHeader::from_binary(
                &bytes[index..index + elf.header.program_header_size as usize],
            )));
        }
        //Sections
        for i in 0..elf.header.num_of_section_header_entries {
            let index =
                (elf.header.section_header_start + (i * elf.header.section_size) as u32) as usize;
            elf.sections.push(ELFSection::from_entire_elf(ELFSectionHeader::from_binary(
                &bytes[index..index + elf.header.section_size as usize],
            ), bytes));
        }

        //Map virtual data
        for section in &elf.sections {

        }
        elf.load_section_names();
        elf
    }
    pub fn load_section_names(&mut self) {
        let data = self.sections[self.header.shstrtab_index as usize].data.clone();
        self.load_section_names_from_table(&data[..]);
    }
    pub fn load_section_names_from_table(&mut self, table: &[u8]) {
        for mut sec in &mut self.sections {
            sec.header.name_from_shstrtab(table);
        }
    }
}
impl ToString for ELF {
    fn to_string(&self) -> String {

            self.sections
                .iter()
                .fold(format!("Programs: {}\n", self.programs.len()), |sum, iter| sum + &iter.header.name + "\n")
    }
}
