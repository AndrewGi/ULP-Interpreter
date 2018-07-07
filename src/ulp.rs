
fn u16_from_slice(bytes: &[u8]) -> u16 {
    debug_assert!(bytes.len()==2, "passed in {} bytes instead of 2 to u16_from_slice", bytes.len());
    bytes[0] as u16 | ((bytes[1] as u16) << 8)
}
fn u32_from_slice(bytes: &[u8]) -> u32 {
    debug_assert!(bytes.len()==4, "passed in {} bytes instead of 4 to u32_from_slice", bytes.len());
    u16_from_slice(&bytes[0..2]) as u32 | ((u16_from_slice(&bytes[2..4]) as u32) << 16)
}
fn u64_from_slice(bytes: &[u8]) -> u64 {
    debug_assert!(bytes.len()==8, "passed in {} bytes instead of 8 to u64_from_slice", bytes.len());
    u32_from_slice(&bytes[0..4]) as u64 | ((u32_from_slice(&bytes[4..8]) as u64) << 32)
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
    pub section_index: u16,
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
            section_index: u16_from_slice(&binary[0x32..0x34]),
        }
    }
    pub fn default() -> ELFHeader {
        ELFHeader::from_binary(&[0u8; 0x34])
    }
    
    pub fn is_valid(&self) -> bool {
        self.magic_number == 0x464c457f
    }
}
pub struct ELFProgramHeader {
    header_type: u32,
    offset: u32,
    virtual_address: u32,
    physical_address: u32,
    file_image_size: u32,
    memory_size: u32,
    flags: u32,
    alignment: u32
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
            alignment: u32_from_slice(&binary[0x1C..0x20])
        }
    }
}
pub struct ELFProgram {
    header: ELFProgramHeader
}
impl ELFProgram {
    pub fn from(program_header: ELFProgramHeader) -> ELFProgram {
        ELFProgram {
            header: program_header
        }
    }
}
pub struct ELFSectionHeader {
    name: u32, //Pointer to .shstrtab string
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
            name: u32_from_slice(&binary[0x0..0x4]),
            section_header_type: u32_from_slice(&binary[0x4..0x8]),
            flags: u32_from_slice(&binary[0x8..0xC]),
            addr: u32_from_slice(&binary[0xC..0x10]),
            offset: u32_from_slice(&binary[0x10..0x14]),
            size: u32_from_slice(&binary[0x14..0x18]),
            link: u32_from_slice(&binary[0x18..0x1C]),
            info: u32_from_slice(&binary[0x1C..0x20]),
            address_align: u32_from_slice(&binary[0x20..0x24]),
            entry_size: u32_from_slice(&binary[0x24..0x28])
        }
    }
}
pub struct ELFSection{
    header: ELFSectionHeader,
    data: ::std::vec::Vec<u8>
}
impl ELFSection {
    pub fn from(section_header: ELFSectionHeader, entire_buffer: &[u8]) -> ELFSection{
        ELFSection {
            data: ::std::vec::Vec::from(&entire_buffer[section_header.link as usize..(section_header.link + section_header.size) as usize]),
            header: section_header
        }
    }
    
}
pub struct ELF {
    pub data : ::std::vec::Vec<u8>,
    pub header: ELFHeader,
    pub programs: ::std::vec::Vec<ELFProgramHeader>,
    pub sections: ::std::vec::Vec<ELFSectionHeader>
}
pub struct ULP {
    pub elf: ELF,
}
impl ELF {
    pub fn from_file(file: &mut ::std::fs::File) -> ELF {
        use std::io::*;
        let mut elf = ELF {
            data: ::std::vec::Vec::new(),
            header: ELFHeader::default(),
            programs: ::std::vec::Vec::default(),
            sections: ::std::vec::Vec::default()
        };
        {
            let ref mut file_buffer = &mut elf.data;
        
            file.read_to_end(file_buffer).expect("unable to read file");
        }
        let ref file_buffer = &elf.data;
        let elf_header = ELFHeader::from_binary(&file_buffer[0..0x34]);

        let mut elf_programs = ::std::vec::Vec::new();
        for i in 0..elf_header.num_of_program_header_entries {
            let index = (elf_header.program_header_start + (i * elf_header.program_header_size) as u32) as usize;
            elf_programs.push(ELFProgram::from(ELFProgramHeader::from_binary(&file_buffer[index..index+elf_header.program_header_size as usize])));
        }

        let mut elf_sections = ::std::vec::Vec::new();
        for i in 0..elf_header.num_of_section_header_entries {
            let index = (elf_header.section_header_start + (i * elf_header.section_size) as u32) as usize;
            elf_sections.push(ELFSectionHeader::from_binary(&file_buffer[index..index+elf_header.section_size as usize]));
        }
        
        elf
    }
}
