# Axiom Technical Implementation Guide - Part 3

## Binary Generation, Linker, Runtime & Interpreter

---

# Part 8: Binary Generation & Object Files

## 8.1 Object File Format

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         OBJECT FILE STRUCTURE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        ELF HEADER (64-bit)                           │   │
│  │                                                                      │   │
│  │  Magic: 0x7F 'E' 'L' 'F'                                            │   │
│  │  Class: 64-bit ELF                                                  │   │
│  │  Endian: Little                                                     │   │
│  │  Type: ET_REL (Relocatable)                                         │   │
│  │  Machine: EM_X86_64 / EM_AARCH64                                    │   │
│  │  Entry: 0 (for object files)                                        │   │
│  │  Flags: 0                                                           │   │
│  │  Section header offset: N                                           │   │
│  │  Program header offset: 0 (not used in object files)                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        SECTIONS                                      │   │
│  │                                                                      │   │
│  │  .text      - Executable code                                       │   │
│  │  .rodata    - Read-only data (constants, strings)                   │   │
│  │  .data      - Initialized writable data                             │   │
│  │  .bss       - Uninitialized writable data (zero-initialized)        │   │
│  │  .symtab    - Symbol table                                          │   │
│  │  .strtab    - String table for symbol names                         │   │
│  │  .rela.text - Relocations for .text section                         │   │
│  │  .debug_*   - DWARF debug information                               │   │
│  │  .eh_frame  - Exception handling frames                             │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                   │                                         │
│                                   ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     SECTION HEADER TABLE                             │   │
│  │                                                                      │   │
│  │  For each section:                                                  │   │
│  │    - Name offset in .shstrtab                                       │   │
│  │    - Type (PROGBITS, SYMTAB, STRTAB, RELA, etc.)                   │   │
│  │    - Flags (ALLOC, EXECINSTR, WRITE, etc.)                         │   │
│  │    - Virtual address (0 for object files)                           │   │
│  │    - File offset                                                     │   │
│  │    - Size                                                            │   │
│  │    - Link, Info (section indices)                                   │   │
│  │    - Alignment                                                       │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 8.2 Object File Writer

```axiom
// src/compiler/codegen/object_writer.ax

/// Object file writer - produces ELF/COFF/Mach-O
pub struct ObjectWriter {
    /// Target architecture
    target: Target,
    
    /// Output buffer
    buffer: Vec<u8>,
    
    /// Sections
    sections: Vec<Section>,
    
    /// Symbols
    symbols: Vec<Symbol>,
    
    /// Relocations
    relocations: Vec<Relocation>,
}

/// Target architecture
pub enum Target {
    X86_64Linux,
    X86_64MacOS,
    X86_64Windows,
    AArch64Linux,
    AArch64MacOS,
}

/// Section in object file
pub struct Section {
    pub name: string,
    pub kind: SectionKind,
    pub data: Vec<u8>,
    pub alignment: usize,
    pub flags: SectionFlags,
}

pub enum SectionKind {
    Text,       // Executable code
    Data,       // Initialized data
    RoData,     // Read-only data
    Bss,        // Uninitialized data
    SymTab,     // Symbol table
    StrTab,     // String table
    Rela,       // Relocations
    Debug,      // Debug info
    EhFrame,    // Exception handling
}

/// Symbol in object file
pub struct Symbol {
    pub name: string,
    pub kind: SymbolKind,
    pub section: Option<usize>,
    pub value: u64,
    pub size: u64,
    pub binding: SymbolBinding,
    pub visibility: SymbolVisibility,
}

pub enum SymbolKind {
    Func,
    Object,
    Section,
    File,
    Common,
}

pub enum SymbolBinding {
    Local,
    Global,
    Weak,
}

pub enum SymbolVisibility {
    Default,
    Internal,
    Hidden,
    Protected,
}

/// Relocation entry
pub struct Relocation {
    pub offset: u64,
    pub kind: RelocationKind,
    pub symbol: usize,
    pub addend: i64,
}

pub enum RelocationKind {
    // x86_64
    X86_64_64,       // 64-bit absolute
    X86_64_PC32,     // 32-bit PC-relative
    X86_64_PLT32,    // 32-bit PLT-relative
    X86_64_GOTPCREL, // 32-bit GOT-relative
    
    // AArch64
    AARCH64_CALL26,  // 26-bit call
    AARCH64_ADR_PREL, // ADR instruction
    AARCH64_ADD_ABS,  // ADD instruction
}

impl ObjectWriter {
    /// Create new object writer
    pub fn new(target: Target) -> Self {
        Self {
            target,
            buffer: Vec::new(),
            sections: Vec::new(),
            symbols: Vec::new(),
            relocations: Vec::new(),
        }
    }
    
    /// Add a section
    pub fn add_section(&mut self, name: string, kind: SectionKind) -> usize {
        let idx = self.sections.len();
        self.sections.push(Section {
            name,
            kind,
            data: Vec::new(),
            alignment: 1,
            flags: SectionFlags::empty(),
        });
        idx
    }
    
    /// Add a symbol
    pub fn add_symbol(&mut self, symbol: Symbol) -> usize {
        let idx = self.symbols.len();
        self.symbols.push(symbol);
        idx
    }
    
    /// Add a relocation
    pub fn add_relocation(&mut self, reloc: Relocation) {
        self.relocations.push(reloc);
    }
    
    /// Write data to a section
    pub fn write_section_data(&mut self, section: usize, data: &[u8]) {
        self.sections[section].data.extend_from_slice(data);
    }
    
    /// Emit ELF object file
    pub fn emit_elf(&self) -> Vec<u8> {
        let mut writer = BinaryWriter::new();
        
        // ELF Header
        writer.write_u8(0x7F);  // Magic
        writer.write_u8(b'E');
        writer.write_u8(b'L');
        writer.write_u8(b'F');
        writer.write_u8(2);     // 64-bit
        writer.write_u8(1);     // Little endian
        writer.write_u8(1);     // ELF version
        writer.write_u8(0);     // OS/ABI
        
        // Padding
        writer.write_bytes(&[0; 8]);
        
        // Type: ET_REL
        writer.write_u16(1);
        
        // Machine
        let machine = match self.target {
            Target::X86_64Linux | Target::X86_64MacOS | Target::X86_64Windows => 0x3E,  // EM_X86_64
            Target::AArch64Linux | Target::AArch64MacOS => 0xB7,  // EM_AARCH64
        };
        writer.write_u16(machine);
        
        // Version
        writer.write_u32(1);
        
        // Entry point (0 for object files)
        writer.write_u64(0);
        
        // Program header offset (0 for object files)
        writer.write_u64(0);
        
        // Section header offset (calculated later)
        let shoff_pos = writer.position();
        writer.write_u64(0);
        
        // Flags
        writer.write_u32(0);
        
        // ELF header size
        writer.write_u16(64);
        
        // Program header size (0 for object files)
        writer.write_u16(0);
        
        // Program header count
        writer.write_u16(0);
        
        // Section header size
        writer.write_u16(64);
        
        // Section header count
        writer.write_u16(self.sections.len() as u16 + 1);  // +1 for null section
        
        // Section name string table index
        writer.write_u16(self.sections.len() as u16);  // Last section is .shstrtab
        
        // Write sections
        let section_data_offsets: Vec<u64> = self.sections.iter().map(|sec| {
            let offset = writer.position();
            writer.write_bytes(&sec.data);
            offset
        }).collect();
        
        // Write section header table
        let shoff = writer.position();
        
        // Null section header
        writer.write_bytes(&[0; 64]);
        
        // Section headers
        for (i, section) in self.sections.iter().enumerate() {
            writer.write_u32(self.get_shstrtab_offset(&section.name));
            
            let sh_type = match section.kind {
                SectionKind::Text | SectionKind::Data | SectionKind::RoData => 1,  // SHT_PROGBITS
                SectionKind::Bss => 8,  // SHT_NOBITS
                SectionKind::SymTab => 2,  // SHT_SYMTAB
                SectionKind::StrTab => 3,  // SHT_STRTAB
                SectionKind::Rela => 4,  // SHT_RELA
                SectionKind::Debug => 1,  // SHT_PROGBITS
                SectionKind::EhFrame => 1,  // SHT_PROGBITS
            };
            writer.write_u32(sh_type);
            
            let sh_flags = self.get_section_flags(section);
            writer.write_u64(sh_flags);
            
            writer.write_u64(0);  // Virtual address
            writer.write_u64(section_data_offsets[i]);  // File offset
            writer.write_u64(section.data.len() as u64);  // Size
            writer.write_u32(0);  // Link
            writer.write_u32(0);  // Info
            writer.write_u64(section.alignment as u64);  // Alignment
            writer.write_u64(0);  // Entry size
        }
        
        // Update section header offset
        writer.patch_u64(shoff_pos, shoff);
        
        writer.into_vec()
    }
    
    fn get_section_flags(&self, section: &Section) -> u64 {
        let mut flags = 0u64;
        
        match section.kind {
            SectionKind::Text => {
                flags |= 0x2;   // SHF_ALLOC
                flags |= 0x4;   // SHF_EXECINSTR
            }
            SectionKind::Data => {
                flags |= 0x2;   // SHF_ALLOC
                flags |= 0x1;   // SHF_WRITE
            }
            SectionKind::RoData => {
                flags |= 0x2;   // SHF_ALLOC
            }
            SectionKind::Bss => {
                flags |= 0x2;   // SHF_ALLOC
                flags |= 0x1;   // SHF_WRITE
            }
            _ => {}
        }
        
        flags
    }
}

/// Binary writer helper
pub struct BinaryWriter {
    buffer: Vec<u8>,
}

impl BinaryWriter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
    
    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }
    
    pub fn write_u16(&mut self, value: u16) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }
    
    pub fn write_u32(&mut self, value: u32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }
    
    pub fn write_u64(&mut self, value: u64) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }
    
    pub fn write_i64(&mut self, value: i64) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }
    
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }
    
    pub fn position(&self) -> u64 {
        self.buffer.len() as u64
    }
    
    pub fn patch_u64(&mut self, position: u64, value: u64) {
        let pos = position as usize;
        self.buffer[pos..pos + 8].copy_from_slice(&value.to_le_bytes());
    }
    
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }
}
```

---

# Part 9: Linker

## 9.1 Linker Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           LINKER PIPELINE                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │
│  │  Object 1   │  │  Object 2   │  │  Archive    │  │   Shared    │       │
│  │   (.o)      │  │   (.o)      │  │   (.a)      │  │   Lib       │       │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘       │
│         │                │                │                │               │
│         └────────────────┴────────────────┴────────────────┘               │
│                                            │                                │
│                                            ▼                                │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                    1. INPUT PROCESSING                               │  │
│  │                                                                      │  │
│  │   • Read object files                                               │  │
│  │   • Parse ELF/COFF/Mach-O headers                                   │  │
│  │   • Extract symbols and sections                                    │  │
│  │   • Build symbol table                                              │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                            │                                │
│                                            ▼                                │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                    2. SYMBOL RESOLUTION                              │  │
│  │                                                                      │  │
│  │   • Resolve undefined symbols                                       │  │
│  │   • Detect duplicate definitions                                    │  │
│  │   • Apply visibility rules                                          │  │
│  │   • Handle weak symbols                                             │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                            │                                │
│                                            ▼                                │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                    3. SECTION LAYOUT                                 │  │
│  │                                                                      │  │
│  │   • Group sections by type                                          │  │
│  │   • Assign virtual addresses                                        │  │
│  │   • Apply alignment constraints                                     │  │
│  │   • Create program headers                                          │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                            │                                │
│                                            ▼                                │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                    4. RELOCATION                                     │  │
│  │                                                                      │  │
│  │   • Apply relocations                                               │  │
│  │   • Resolve PC-relative references                                  │  │
│  │   • Fill GOT/PLT entries                                            │  │
│  │   • Process exception handling tables                               │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                            │                                │
│                                            ▼                                │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                    5. OUTPUT GENERATION                              │  │
│  │                                                                      │  │
│  │   • Write ELF/PE/Mach-O executable                                  │  │
│  │   • Generate debug info                                             │  │
│  │   • Strip symbols (if requested)                                    │  │
│  │   • Sign executable (macOS/Windows)                                 │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                            │                                │
│                                            ▼                                │
│                                   ┌────────────────┐                       │
│                                   │   EXECUTABLE   │                       │
│                                   │   (.exe / no   │                       │
│                                   │   extension)   │                       │
│                                   └────────────────┘                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 9.2 Linker Implementation

```axiom
// src/compiler/linker/linker.ax

/// Linker - combines object files into executable
pub struct Linker {
    /// Target configuration
    target: Target,
    
    /// Input object files
    inputs: Vec<InputFile>,
    
    /// Output path
    output: PathBuf,
    
    /// Libraries to link
    libraries: Vec<string>,
    
    /// Library search paths
    library_paths: Vec<PathBuf>,
    
    /// Linker options
    options: LinkerOptions,
    
    /// Symbol table
    symbols: SymbolTable,
    
    /// Sections
    sections: Vec<OutputSection>,
}

pub struct InputFile {
    pub path: PathBuf,
    pub format: FileFormat,
    pub symbols: Vec<InputSymbol>,
    pub sections: Vec<InputSection>,
}

pub enum FileFormat {
    Elf,
    Coff,
    MachO,
    Archive,
}

pub struct InputSymbol {
    pub name: string,
    pub section: Option<usize>,
    pub value: u64,
    pub size: u64,
    pub binding: SymbolBinding,
    pub kind: SymbolKind,
    pub is_defined: bool,
    pub is_weak: bool,
    pub is_common: bool,
}

pub struct InputSection {
    pub name: string,
    pub data: Vec<u8>,
    pub relocations: Vec<InputRelocation>,
    pub alignment: usize,
    pub flags: u64,
}

pub struct OutputSection {
    pub name: string,
    pub virtual_address: u64,
    pub file_offset: u64,
    pub size: u64,
    pub data: Vec<u8>,
    pub permissions: SectionPermissions,
}

pub struct SectionPermissions {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

pub struct LinkerOptions {
    pub output_type: OutputType,
    pub is_static: bool,
    pub strip_symbols: bool,
    pub gc_sections: bool,      // Garbage collect unused sections
    pub lto: bool,              // Link-time optimization
    pub entry_point: string,
    pub base_address: Option<u64>,
}

pub enum OutputType {
    Executable,
    SharedLibrary,
    StaticLibrary,
    ObjectFile,
}

impl Linker {
    /// Create new linker
    pub fn new(target: Target, output: PathBuf) -> Self {
        Self {
            target,
            inputs: Vec::new(),
            output,
            libraries: Vec::new(),
            library_paths: Vec::new(),
            options: LinkerOptions::default(),
            symbols: SymbolTable::new(),
            sections: Vec::new(),
        }
    }
    
    /// Add input file
    pub fn add_input(&mut self, path: PathBuf) {
        self.inputs.push(InputFile {
            path,
            format: FileFormat::Elf,  // Will be detected
            symbols: Vec::new(),
            sections: Vec::new(),
        });
    }
    
    /// Add library search path
    pub fn add_library_path(&mut self, path: PathBuf) {
        self.library_paths.push(path);
    }
    
    /// Add library to link
    pub fn add_library(&mut self, name: string) {
        self.libraries.push(name);
    }
    
    /// Perform linking
    pub fn link(&mut self) -> Result!PathBuf {
        // Phase 1: Read input files
        self.read_inputs()?;
        
        // Phase 2: Resolve symbols
        self.resolve_symbols()?;
        
        // Phase 3: Layout sections
        self.layout_sections()?;
        
        // Phase 4: Apply relocations
        self.apply_relocations()?;
        
        // Phase 5: Write output
        self.write_output()?;
        
        Ok(self.output.clone())
    }
    
    /// Read all input files
    fn read_inputs(&mut self) -> Result!() {
        for input in &mut self.inputs {
            self.read_input_file(input)?;
        }
        Ok(())
    }
    
    /// Read a single input file
    fn read_input_file(&self, input: &mut InputFile) -> Result!() {
        let data = std::fs::read(&input.path)?;
        
        // Detect format
        input.format = self.detect_format(&data)?;
        
        match input.format {
            FileFormat::Elf => self.read_elf(&data, input),
            FileFormat::Coff => self.read_coff(&data, input),
            FileFormat::MachO => self.read_macho(&data, input),
            FileFormat::Archive => self.read_archive(&data, input),
        }
    }
    
    /// Detect file format from magic bytes
    fn detect_format(&self, data: &[u8]) -> Result!FileFormat {
        if data.len() < 4 {
            return Err(LinkError::invalid_file("file too small"));
        }
        
        // ELF
        if &data[0..4] == b"\x7fELF" {
            return Ok(FileFormat::Elf);
        }
        
        // Mach-O
        if data[0..4] == [0xFE, 0xED, 0xFA, 0xCE] ||  // 32-bit
           data[0..4] == [0xFE, 0xED, 0xFA, 0xCF] {   // 64-bit
            return Ok(FileFormat::MachO);
        }
        
        // COFF/PE
        if data[0..2] == [0x4D, 0x5B] {  // MZ
            return Ok(FileFormat::Coff);
        }
        
        // Archive
        if &data[0..8] == b"!<arch>\n" {
            return Ok(FileFormat::Archive);
        }
        
        Err(LinkError::invalid_file("unknown file format"))
    }
    
    /// Resolve all symbols
    fn resolve_symbols(&mut self) -> Result!() {
        // Add all defined symbols to the table
        for input in &self.inputs {
            for symbol in &input.symbols {
                if symbol.is_defined {
                    self.symbols.define(symbol)?;
                }
            }
        }
        
        // Check for undefined symbols
        for input in &self.inputs {
            for symbol in &input.symbols {
                if !symbol.is_defined && !symbol.is_weak {
                    if !self.symbols.is_defined(&symbol.name) {
                        return Err(LinkError::undefined_symbol(&symbol.name));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Layout sections in memory
    fn layout_sections(&mut self) -> Result!() {
        // Collect all sections by type
        let mut text_sections: Vec<&InputSection> = Vec::new();
        let mut data_sections: Vec<&InputSection> = Vec::new();
        let mut rodata_sections: Vec<&InputSection> = Vec::new();
        let mut bss_sections: Vec<&InputSection> = Vec::new();
        
        for input in &self.inputs {
            for section in &input.sections {
                if section.name.starts_with(".text") {
                    text_sections.push(section);
                } else if section.name.starts_with(".data") {
                    data_sections.push(section);
                } else if section.name.starts_with(".rodata") {
                    rodata_sections.push(section);
                } else if section.name.starts_with(".bss") {
                    bss_sections.push(section);
                }
            }
        }
        
        // Assign addresses
        let base = self.options.base_address.unwrap_or(0x400000);
        let mut address = base;
        
        // Text section (executable)
        address = self.align_to(address, 0x1000);
        address = self.layout_section_group(&text_sections, address, 
            SectionPermissions { readable: true, writable: false, executable: true })?;
        
        // RoData section (read-only)
        address = self.align_to(address, 0x1000);
        address = self.layout_section_group(&rodata_sections, address,
            SectionPermissions { readable: true, writable: false, executable: false })?;
        
        // Data section (read-write)
        address = self.align_to(address, 0x1000);
        address = self.layout_section_group(&data_sections, address,
            SectionPermissions { readable: true, writable: true, executable: false })?;
        
        // BSS section (uninitialized)
        address = self.align_to(address, 0x1000);
        self.layout_section_group(&bss_sections, address,
            SectionPermissions { readable: true, writable: true, executable: false })?;
        
        Ok(())
    }
    
    /// Layout a group of sections
    fn layout_section_group(
        &mut self,
        sections: &[&InputSection],
        start_address: u64,
        permissions: SectionPermissions,
    ) -> Result!u64> {
        let mut address = start_address;
        
        for section in sections {
            // Apply alignment
            address = self.align_to(address, section.alignment);
            
            // Add to output sections
            self.sections.push(OutputSection {
                name: section.name.clone(),
                virtual_address: address,
                file_offset: 0,  // Will be set later
                size: section.data.len() as u64,
                data: section.data.clone(),
                permissions: permissions.clone(),
            });
            
            address += section.data.len() as u64;
        }
        
        Ok(address)
    }
    
    /// Apply relocations
    fn apply_relocations(&mut self) -> Result!() {
        for input in &self.inputs {
            for (section_idx, section) in input.sections.iter().enumerate() {
                for reloc in &section.relocations {
                    self.apply_relocation(input, section_idx, reloc)?;
                }
            }
        }
        Ok(())
    }
    
    /// Apply a single relocation
    fn apply_relocation(
        &mut self,
        input: &InputFile,
        section_idx: usize,
        reloc: &InputRelocation,
    ) -> Result!() {
        // Find target symbol
        let symbol = self.symbols.get(&reloc.symbol_name)?;
        
        // Calculate relocation value
        let symbol_address = symbol.virtual_address;
        let reloc_address = self.get_section_address(input, section_idx)? + reloc.offset;
        
        let reloc_value = match reloc.kind {
            RelocationKind::X86_64_64 => {
                // 64-bit absolute
                symbol_address.wrapping_add(reloc.addend as u64)
            }
            RelocationKind::X86_64_PC32 => {
                // 32-bit PC-relative
                let diff = (symbol_address as i64 - reloc_address as i64 + reloc.addend) as i32;
                diff as u64
            }
            RelocationKind::X86_64_PLT32 => {
                // PLT-relative (for function calls)
                let plt_address = self.get_or_create_plt_entry(&reloc.symbol_name)?;
                (plt_address as i64 - reloc_address as i64 + reloc.addend) as i32 as u64
            }
            RelocationKind::X86_64_GOTPCREL => {
                // GOT-relative
                let got_address = self.get_or_create_got_entry(&reloc.symbol_name)?;
                (got_address as i64 - reloc_address as i64 + reloc.addend) as i32 as u64
            }
            _ => return Err(LinkError::unsupported_relocation()),
        };
        
        // Apply to section data
        self.patch_relocation(section_idx, reloc.offset, reloc_value, reloc.kind)?;
        
        Ok(())
    }
    
    /// Write the output executable
    fn write_output(&self) -> Result!() {
        match self.target {
            Target::X86_64Linux | Target::AArch64Linux => self.write_elf_executable(),
            Target::X86_64MacOS | Target::AArch64MacOS => self.write_macho_executable(),
            Target::X86_64Windows => self.write_pe_executable(),
        }
    }
    
    /// Write ELF executable
    fn write_elf_executable(&self) -> Result!() {
        let mut writer = BinaryWriter::new();
        
        // ELF Header
        writer.write_u8(0x7F);
        writer.write_u8(b'E');
        writer.write_u8(b'L');
        writer.write_u8(b'F');
        writer.write_u8(2);     // 64-bit
        writer.write_u8(1);     // Little endian
        writer.write_u8(1);     // Version
        writer.write_u8(0);     // OS/ABI
        writer.write_bytes(&[0; 8]);
        
        writer.write_u16(2);    // ET_EXEC
        writer.write_u16(self.get_elf_machine());
        writer.write_u32(1);    // Version
        
        // Entry point
        writer.write_u64(self.get_entry_point()?);
        
        // Program header offset
        let phoff = 64;  // Right after ELF header
        writer.write_u64(phoff);
        
        // Section header offset
        let shoff_pos = writer.position();
        writer.write_u64(0);  // Will be patched
        
        writer.write_u32(0);  // Flags
        writer.write_u16(64); // ELF header size
        writer.write_u16(56); // Program header size
        
        // Number of program headers
        let num_phdrs = self.count_program_headers();
        writer.write_u16(num_phdrs as u16);
        
        writer.write_u16(64); // Section header size
        writer.write_u16(0);  // Number of section headers (0 for stripped)
        writer.write_u16(0);  // Section name string table index
        
        // Program headers
        for section in &self.sections {
            self.write_program_header(&mut writer, section)?;
        }
        
        // Section data
        for section in &self.sections {
            writer.write_bytes(&section.data);
            writer.align_to(0x1000);
        }
        
        // Write to file
        std::fs::write(&self.output, writer.into_vec())?;
        
        // Make executable
        self.set_executable_permission()?;
        
        Ok(())
    }
    
    fn align_to(&self, value: u64, alignment: u64) -> u64 {
        (value + alignment - 1) & !(alignment - 1)
    }
}

/// Symbol table for linking
pub struct SymbolTable {
    symbols: HashMap<string, LinkSymbol>,
}

pub struct LinkSymbol {
    pub name: string,
    pub virtual_address: u64,
    pub size: u64,
    pub input_file: usize,
    pub is_defined: bool,
    pub is_weak: bool,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { symbols: HashMap::new() }
    }
    
    pub fn define(&mut self, symbol: &InputSymbol) -> Result!() {
        if let Some(existing) = self.symbols.get(&symbol.name) {
            // Check for duplicate definitions
            if existing.is_defined && symbol.is_defined && !symbol.is_weak {
                return Err(LinkError::duplicate_symbol(&symbol.name));
            }
        }
        
        // Weak symbols don't override strong symbols
        if symbol.is_weak && self.symbols.contains_key(&symbol.name) {
            return Ok(());
        }
        
        self.symbols.insert(symbol.name.clone(), LinkSymbol {
            name: symbol.name.clone(),
            virtual_address: symbol.value,
            size: symbol.size,
            input_file: 0,
            is_defined: symbol.is_defined,
            is_weak: symbol.is_weak,
        });
        
        Ok(())
    }
    
    pub fn get(&self, name: &str) -> Result!&LinkSymbol {
        self.symbols.get(name)
            .ok_or_else(|| LinkError::undefined_symbol(name))
    }
    
    pub fn is_defined(&self, name: &str) -> bool {
        self.symbols.get(name).map(|s| s.is_defined).unwrap_or(false)
    }
}
```

---

# Part 10: Runtime System

## 10.1 Runtime Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          RUNTIME SYSTEM                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    MEMORY MANAGEMENT                                 │   │
│  │                                                                      │   │
│  │   ┌──────────────┐   ┌──────────────┐   ┌──────────────────────┐   │   │
│  │   │   ALLOCATOR  │   │    STACK     │   │   HEAP MANAGEMENT    │   │   │
│  │   │              │   │              │   │                      │   │   │
│  │   │  • malloc    │   │  • Stack     │   │  • Free lists        │   │   │
│  │   │  • free      │   │    frames    │   │  • Memory pools      │   │   │
│  │   │  • realloc   │   │  • Red zone  │   │  • GC roots (opt)    │   │   │
│  │   └──────────────┘   └──────────────┘   └──────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    EXCEPTION HANDLING                                │   │
│  │                                                                      │   │
│  │   ┌──────────────┐   ┌──────────────┐   ┌──────────────────────┐   │   │
│  │   │   UNWINDER   │   │    PERSON-   │   │   EH FRAMES          │   │   │
│  │   │              │   │    ALITY     │   │                      │   │   │
│  │   │  • Stack     │   │              │   │  • .eh_frame         │   │   │
│  │   │    walking   │   │  • CATCH     │   │  • LSDA (Language    │   │   │
│  │   │  • Register  │   │    handlers  │   │    Specific Data)    │   │   │
│  │   │    restore   │   │  • Cleanup   │   │                      │   │   │
│  │   └──────────────┘   └──────────────┘   └──────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    TYPE INFORMATION                                  │   │
│  │                                                                      │   │
│  │   ┌──────────────┐   ┌──────────────┐   ┌──────────────────────┐   │   │
│  │   │    TYPE      │   │    VTABLE    │   │   DROP GLUE          │   │   │
│  │   │  DESCRIPTOR  │   │              │   │                      │   │   │
│  │   │              │   │  • Virtual   │   │  • Destructor        │   │   │
│  │   │  • Name      │   │    calls     │   │    calls             │   │   │
│  │   │  • Size      │   │  • Trait     │   │  • Drop order        │   │   │
│  │   │  • Align     │   │    objects   │   │                      │   │   │
│  │   └──────────────┘   └──────────────┘   └──────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    ASYNC RUNTIME                                     │   │
│  │                                                                      │   │
│  │   ┌──────────────┐   ┌──────────────┐   ┌──────────────────────┐   │   │
│  │   │   EXECUTOR   │   │    TASK      │   │   WAKER              │   │   │
│  │   │              │   │              │   │                      │   │   │
│  │   │  • Work      │   │  • State     │   │  • Wakeup            │   │   │
│  │   │    stealing  │   │    machine   │   │    mechanism         │   │   │
│  │   │  • I/O       │   │  • Context   │   │  • Atomic            │   │   │
│  │   │    driver    │   │              │   │    operations        │   │   │
│  │   └──────────────┘   └──────────────┘   └──────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 10.2 Runtime Implementation

```axiom
// src/runtime/mod.ax

/// Runtime entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize runtime
    unsafe {
        // Set up stack guard
        rt::stack_guard::init();
        
        // Initialize allocator
        rt::alloc::init();
        
        // Initialize TLS
        rt::thread_local::init();
    }
    
    // Call main
    let exit_code = match main() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    };
    
    // Exit
    rt::sys::exit(exit_code)
}

// ============ MEMORY ALLOCATION ============

/// Global allocator interface
pub trait GlobalAllocator {
    fn alloc(&self, layout: Layout) -> ?*u8;
    fn dealloc(&self, ptr: *u8, layout: Layout);
    fn realloc(&self, ptr: *u8, layout: Layout, new_size: usize) -> ?*u8;
}

/// Memory layout
pub struct Layout {
    pub size: usize,
    pub align: usize,
}

impl Layout {
    pub fn new(size: usize, align: usize) -> Self {
        Self { size, align }
    }
    
    pub fn for_type<T>() -> Self {
        Self {
            size: std::mem::size_of::<T>(),
            align: std::mem::align_of::<T>(),
        }
    }
}

/// Bump allocator for small allocations
pub struct BumpAllocator {
    /// Start of memory region
    start: *u8,
    
    /// Current position
    current: AtomicPtr<u8>,
    
    /// End of memory region
    end: *u8,
}

impl BumpAllocator {
    pub fn new(start: *u8, size: usize) -> Self {
        Self {
            start,
            current: AtomicPtr::new(start),
            end: unsafe { start.add(size) },
        }
    }
}

impl GlobalAllocator for BumpAllocator {
    fn alloc(&self, layout: Layout) -> ?*u8 {
        let align = layout.align;
        let size = layout.size;
        
        loop {
            let current = self.current.load(Ordering::Relaxed);
            
            // Align current pointer
            let aligned = align_up(current as usize, align) as *u8;
            
            // Check if we have space
            let new_current = unsafe { aligned.add(size) };
            if new_current >= self.end {
                return None;
            }
            
            // Try to claim this memory
            if self.current.compare_exchange(
                current,
                new_current,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ).is_ok() {
                return Some(aligned);
            }
        }
    }
    
    fn dealloc(&self, _ptr: *u8, _layout: Layout) {
        // Bump allocator doesn't free individual allocations
    }
    
    fn realloc(&self, ptr: *u8, layout: Layout, new_size: usize) -> ?*u8 {
        // Just allocate new and copy
        let new = self.alloc(Layout::new(new_size, layout.align))?;
        unsafe {
            std::ptr::copy_nonoverlapping(ptr, new, layout.size.min(new_size));
        }
        Some(new)
    }
}

/// Slab allocator for fixed-size allocations
pub struct SlabAllocator {
    /// Free list
    free_list: AtomicPtr<FreeNode>,
    
    /// Slab size
    slab_size: usize,
    
    /// Object size
    object_size: usize,
    
    /// Slabs
    slabs: Mutex<Vec<*u8>>,
}

struct FreeNode {
    next: ?*FreeNode,
}

impl GlobalAllocator for SlabAllocator {
    fn alloc(&self, _layout: Layout) -> ?*u8 {
        // Try to pop from free list
        loop {
            let node = self.free_list.load(Ordering::Acquire);
            
            if node.is_null() {
                // Allocate new slab
                return self.alloc_new();
            }
            
            let next = unsafe { (*node).next };
            
            if self.free_list.compare_exchange(
                node,
                next.unwrap_or(std::ptr::null_mut()),
                Ordering::AcqRel,
                Ordering::Acquire,
            ).is_ok() {
                return Some(node as *u8);
            }
        }
    }
    
    fn dealloc(&self, ptr: *u8, _layout: Layout) {
        // Push to free list
        let node = ptr as *mut FreeNode;
        
        loop {
            let next = self.free_list.load(Ordering::Acquire);
            unsafe { (*node).next = Some(next) };
            
            if self.free_list.compare_exchange(
                next,
                node,
                Ordering::AcqRel,
                Ordering::Acquire,
            ).is_ok() {
                break;
            }
        }
    }
}

// ============ EXCEPTION HANDLING ============

/// Exception personality function
#[no_mangle]
pub extern "C" fn __axiom_personality(
    version: i32,
    actions: UnwindAction,
    exception_class: u64,
    exception_object: *mut UnwindException,
    context: *mut UnwindContext,
) -> UnwindReasonCode {
    // Find landing pad for this exception
    let ip = unsafe { _Unwind_GetIP(context) };
    
    // Search for exception table entry
    let lsda = unsafe { _Unwind_GetLanguageSpecificData(context) };
    
    if lsda.is_null() {
        return UnwindReasonCode::CONTINUE_UNWIND;
    }
    
    // Parse LSDA and find handler
    match unsafe { find_handler(lsda, ip, exception_object) } {
        Some(landing_pad) => {
            unsafe {
                _Unwind_SetGR(context, 0, exception_object as usize);
                _Unwind_SetGR(context, 1, 0);  // Exception type
                _Unwind_SetIP(context, landing_pad);
            }
            UnwindReasonCode::INSTALL_CONTEXT
        }
        None => UnwindReasonCode::CONTINUE_UNWIND,
    }
}

/// Find handler in LSDA
unsafe fn find_handler(
    lsda: *const u8,
    ip: usize,
    _exception: *mut UnwindException,
) -> Option<usize> {
    // Parse LSDA header
    let mut reader = LsdaReader::new(lsda);
    
    // Read landing pad base
    let lp_start = reader.read_uleb128();
    
    // Read type table
    let type_table_offset = reader.read_uleb128();
    let type_table = lsda.add(type_table_offset);
    
    // Read call site table
    let call_site_table_size = reader.read_uleb128();
    let call_site_end = reader.ptr.add(call_site_table_size);
    
    while reader.ptr < call_site_end {
        let start = reader.read_uleb128();
        let len = reader.read_uleb128();
        let landing_pad = reader.read_uleb128();
        let action = reader.read_uleb128();
        
        let call_site_start = lp_start + start;
        let call_site_end = call_site_start + len;
        
        // Check if IP is in this call site
        if ip >= call_site_start && ip < call_site_end {
            if landing_pad != 0 {
                return Some(lp_start + landing_pad);
            }
            return None;
        }
    }
    
    None
}

// ============ TYPE INFORMATION ============

/// Type descriptor
pub struct TypeDescriptor {
    /// Type name
    pub name: &'static str,
    
    /// Size in bytes
    pub size: usize,
    
    /// Alignment in bytes
    pub align: usize,
    
    /// Drop function
    pub drop: Option<unsafe fn(*mut u8)>,
    
    /// Type ID
    pub type_id: TypeId,
}

/// VTable for trait objects
pub struct VTable {
    /// Size of the type
    pub size: usize,
    
    /// Alignment of the type
    pub align: usize,
    
    /// Drop function
    pub drop: Option<unsafe fn(*mut u8)>,
    
    /// Trait method pointers
    pub methods: [*const (); 0],  // Flexible array
}

/// Fat pointer for trait objects
pub struct TraitObject {
    /// Data pointer
    pub data: *mut u8,
    
    /// VTable pointer
    pub vtable: &'static VTable,
}

// ============ PANIC HANDLING ============

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print panic message
    eprint!("panic: ");
    
    if let Some(location) = info.location() {
        eprint!("{}:{}:{}: ", location.file(), location.line(), location.column());
    }
    
    if let Some(message) = info.message() {
        eprintln!("{}", message);
    } else {
        eprintln!("unknown error");
    }
    
    // Abort
    std::process::abort()
}
```

---

# Part 11: Interpreter

## 11.1 Interpreter Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          INTERPRETER ARCHITECTURE                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         AST EVALUATOR                                │   │
│  │                                                                      │   │
│  │   Input: AST from Parser                                            │   │
│  │   Output: Execution results                                         │   │
│  │                                                                      │   │
│  │   Features:                                                          │   │
│  │   • Direct AST interpretation                                       │   │
│  │   • No compilation to bytecode                                      │   │
│  │   • Fast startup for REPL                                           │   │
│  │   • Debug-friendly (source locations preserved)                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         VALUE SYSTEM                                 │   │
│  │                                                                      │   │
│  │   ┌──────────────────┐   ┌──────────────────┐   ┌────────────────┐ │   │
│  │   │   PRIMITIVE      │   │    COMPOSITE     │   │   REFERENCE    │ │   │
│  │   │                  │   │                  │   │                │ │   │
│  │   │  • Int(i128)     │   │  • Array(Vec)    │   │  • &Value      │ │   │
│  │   │  • Float(f64)    │   │  • Tuple(Vec)    │   │  • &mut Value  │ │   │
│  │   │  • Bool(bool)    │   │  • Struct(Map)   │   │                │ │   │
│  │   │  • Char(char)    │   │  • Enum(variant) │   │                │ │   │
│  │   │  • String        │   │  • Closure       │   │                │ │   │
│  │   └──────────────────┘   └──────────────────┘   └────────────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         ENVIRONMENT                                  │   │
│  │                                                                      │   │
│  │   • Scope stack (local variables)                                   │   │
│  │   • Function definitions                                            │   │
│  │   • Type definitions                                                │   │
│  │   • Module imports                                                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                       CONTROL FLOW                                   │   │
│  │                                                                      │   │
│  │   • Break/Continue handling                                         │   │
│  │   • Return handling                                                 │   │
│  │   • Exception propagation                                           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 11.2 Interpreter Implementation

```axiom
// src/interpreter/mod.ax

/// Interpreter for Axiom (REPL and script execution)
pub struct Interpreter {
    /// Global environment
    env: Environment,
    
    /// Type definitions
    types: TypeRegistry,
    
    /// Module cache
    modules: HashMap<string, Module>,
    
    /// Current module
    current_module: Option<string>,
    
    /// Random number generator
    rng: StdRng,
}

/// Environment (scope stack)
pub struct Environment {
    /// Scope stack
    scopes: Vec<Scope>,
    
    /// Function definitions
    functions: HashMap<string, FunctionDef>,
}

pub struct Scope {
    /// Variables in this scope
    bindings: HashMap<string, Value>,
    
    /// Scope kind
    kind: ScopeKind,
}

pub enum ScopeKind {
    Global,
    Function,
    Block,
    Loop,
}

/// Runtime value
pub enum Value {
    // Primitives
    Int(i128),
    Float(f64),
    Bool(bool),
    Char(char),
    String(GcString),
    Null,
    
    // Composites
    Array(GcVec<Value>),
    Tuple(GcVec<Value>),
    Struct {
        type_name: string,
        fields: HashMap<string, Value>,
    },
    Enum {
        type_name: string,
        variant: string,
        data: Option<Box<Value>>,
    },
    
    // Functions
    Closure {
        params: Vec<string>,
        body: Expr,
        env: HashMap<string, Value>,
    },
    NativeFunc(fn(Vec<Value>) -> Result!Value),
    
    // References
    Ref(GcPtr<Value>),
    RefMut(GcPtr<Value>),
    
    // Special
    Undefined,
}

/// Garbage-collected string
pub struct GcString {
    ptr: *mut String,
    marked: bool,
}

/// Garbage-collected vector
pub struct GcVec<T> {
    ptr: *mut Vec<T>,
    marked: bool,
}

/// Control flow signals
pub enum ControlFlow {
    /// Normal completion
    Continue,
    
    /// Return from function
    Return(Value),
    
    /// Break from loop
    Break(Value),
    
    /// Continue loop
    ContinueLoop,
    
    /// Throw exception
    Throw(Value),
}

impl Interpreter {
    /// Create new interpreter
    pub fn new() -> Self {
        let mut interp = Self {
            env: Environment::new(),
            types: TypeRegistry::new(),
            modules: HashMap::new(),
            current_module: None,
            rng: StdRng::from_entropy(),
        };
        
        // Register built-in functions
        interp.register_builtins();
        
        interp
    }
    
    /// Evaluate a module
    pub fn eval_module(&mut self, module: &Module) -> Result!Value {
        // Push global scope
        self.env.push_scope(ScopeKind::Global);
        
        // Process imports
        for import in &module.imports {
            self.process_import(import)?;
        }
        
        // Evaluate declarations
        let mut result = Value::Null;
        
        for decl in &module.declarations {
            result = self.eval_declaration(decl)?;
        }
        
        self.env.pop_scope();
        
        Ok(result)
    }
    
    /// Evaluate a declaration
    fn eval_declaration(&mut self, decl: &Declaration) -> Result!Value {
        match decl {
            Declaration::Function(func) => {
                self.env.define_function(
                    func.name.name.clone(),
                    FunctionDef {
                        name: func.name.name.clone(),
                        params: func.params.iter().map(|p| p.name.name.clone()).collect(),
                        body: func.body.clone(),
                        closure: self.env.capture_closure(),
                    }
                );
                Ok(Value::Null)
            }
            
            Declaration::Const(const_decl) => {
                let value = self.eval_expr(&const_decl.value)?;
                self.env.define(const_decl.name.name.clone(), value);
                Ok(Value::Null)
            }
            
            Declaration::Static(static_decl) => {
                let value = self.eval_expr(&static_decl.value)?;
                self.env.define(static_decl.name.name.clone(), value);
                Ok(Value::Null)
            }
            
            Declaration::Struct(struct_decl) => {
                self.types.define_struct(struct_decl);
                Ok(Value::Null)
            }
            
            Declaration::Enum(enum_decl) => {
                self.types.define_enum(enum_decl);
                Ok(Value::Null)
            }
            
            _ => Ok(Value::Null),
        }
    }
    
    /// Evaluate an expression
    pub fn eval_expr(&mut self, expr: &Expr) -> Result!Value {
        match expr {
            // Literals
            Expr::Literal(lit, _) => self.eval_literal(lit),
            
            // Identifier
            Expr::Ident(ident) => {
                self.env.get(&ident.name)
                    .ok_or_else(|| Error::undefined_variable(&ident.name))
            }
            
            // Binary operations
            Expr::Binary(binary) => self.eval_binary(binary),
            
            // Unary operations
            Expr::Unary(unary) => self.eval_unary(unary),
            
            // Assignment
            Expr::Assign(assign) => self.eval_assign(assign),
            
            // Function call
            Expr::Call(call) => self.eval_call(call),
            
            // Field access
            Expr::Field(field) => self.eval_field(field),
            
            // Index access
            Expr::Index(index) => self.eval_index(index),
            
            // If expression
            Expr::If(if_expr) => self.eval_if(if_expr),
            
            // Match expression
            Expr::Match(match_expr) => self.eval_match(match_expr),
            
            // Block
            Expr::Block(block) => self.eval_block(block),
            
            // Loop
            Expr::Loop(loop_expr) => self.eval_loop(loop_expr),
            
            // While loop
            Expr::While(while_expr) => self.eval_while(while_expr),
            
            // For loop
            Expr::For(for_expr) => self.eval_for(for_expr),
            
            // Return
            Expr::Return(return_expr) => {
                let value = match &return_expr.value {
                    Some(e) => self.eval_expr(e)?,
                    None => Value::Null,
                };
                Err(Error::Return(value))
            }
            
            // Break
            Expr::Break(break_expr) => {
                let value = match &break_expr.value {
                    Some(e) => self.eval_expr(e)?,
                    None => Value::Null,
                };
                Err(Error::Break(value))
            }
            
            // Continue
            Expr::Continue(_) => {
                Err(Error::Continue)
            }
            
            // Lambda
            Expr::Lambda(lambda) => {
                Ok(Value::Closure {
                    params: lambda.params.iter().map(|p| p.name.name.clone()).collect(),
                    body: (*lambda.body).clone(),
                    env: self.env.capture_closure(),
                })
            }
            
            // Struct construction
            Expr::Struct(struct_expr) => self.eval_struct(struct_expr),
            
            // Array
            Expr::Array(arr) => {
                let mut values = Vec::new();
                for elem in &arr.elements {
                    values.push(self.eval_expr(elem)?);
                }
                Ok(Value::Array(GcVec::new(values)))
            }
            
            // Tuple
            Expr::Tuple(tuple) => {
                let mut values = Vec::new();
                for elem in &tuple.elements {
                    values.push(self.eval_expr(elem)?);
                }
                Ok(Value::Tuple(GcVec::new(values)))
            }
            
            // Reference
            Expr::Reference(ref_expr) => {
                let value = self.eval_expr(&ref_expr.operand)?;
                let ptr = GcPtr::new(value);
                match ref_expr.mutability {
                    Mutability::Immutable => Ok(Value::Ref(ptr)),
                    Mutability::Mutable => Ok(Value::RefMut(ptr)),
                }
            }
            
            // Range
            Expr::Range(range) => {
                let start = match &range.start {
                    Some(e) => Some(Box::new(self.eval_expr(e)?)),
                    None => None,
                };
                let end = match &range.end {
                    Some(e) => Some(Box::new(self.eval_expr(e)?)),
                    None => None,
                };
                // Create range iterator
                // ...
                Ok(Value::Null)  // Placeholder
            }
            
            // Error
            Expr::Error(span, msg) => {
                Err(Error::runtime_error(span, msg))
            }
        }
    }
    
    /// Evaluate a literal
    fn eval_literal(&mut self, lit: &Literal) -> Result!Value {
        match lit {
            Literal::Int(value, _) => Ok(Value::Int(*value)),
            Literal::Float(value, _) => Ok(Value::Float(*value)),
            Literal::String(s) => Ok(Value::String(GcString::new(s.clone()))),
            Literal::Char(c) => Ok(Value::Char(*c)),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Null => Ok(Value::Null),
        }
    }
    
    /// Evaluate binary operation
    fn eval_binary(&mut self, binary: &BinaryExpr) -> Result!Value {
        // Short-circuit evaluation for logical operators
        match binary.op {
            BinaryOp::And => {
                let left = self.eval_expr(&binary.left)?;
                if let Value::Bool(b) = left {
                    if !b {
                        return Ok(Value::Bool(false));
                    }
                    let right = self.eval_expr(&binary.right)?;
                    if let Value::Bool(rb) = right {
                        return Ok(Value::Bool(rb));
                    }
                }
                return Err(Error::type_error("expected boolean"));
            }
            BinaryOp::Or => {
                let left = self.eval_expr(&binary.left)?;
                if let Value::Bool(b) = left {
                    if b {
                        return Ok(Value::Bool(true));
                    }
                    let right = self.eval_expr(&binary.right)?;
                    if let Value::Bool(rb) = right {
                        return Ok(Value::Bool(rb));
                    }
                }
                return Err(Error::type_error("expected boolean"));
            }
            _ => {}
        }
        
        // Evaluate both operands
        let left = self.eval_expr(&binary.left)?;
        let right = self.eval_expr(&binary.right)?;
        
        match (&left, &right) {
            // Integer operations
            (Value::Int(a), Value::Int(b)) => {
                let result = match binary.op {
                    BinaryOp::Add => a.checked_add(*b),
                    BinaryOp::Sub => a.checked_sub(*b),
                    BinaryOp::Mul => a.checked_mul(*b),
                    BinaryOp::Div => {
                        if *b == 0 {
                            return Err(Error::division_by_zero());
                        }
                        a.checked_div(*b)
                    }
                    BinaryOp::Mod => {
                        if *b == 0 {
                            return Err(Error::division_by_zero());
                        }
                        a.checked_rem(*b)
                    }
                    BinaryOp::BitAnd => Some(*a & *b),
                    BinaryOp::BitOr => Some(*a | *b),
                    BinaryOp::BitXor => Some(*a ^ *b),
                    BinaryOp::Shl => Some(*a << (*b as u32)),
                    BinaryOp::Shr => Some(*a >> (*b as u32)),
                    _ => return Err(Error::invalid_binop()),
                };
                
                match result {
                    Some(v) => Ok(Value::Int(v)),
                    None => Err(Error::overflow()),
                }
            }
            
            // Float operations
            (Value::Float(a), Value::Float(b)) => {
                let result = match binary.op {
                    BinaryOp::Add => a + b,
                    BinaryOp::Sub => a - b,
                    BinaryOp::Mul => a * b,
                    BinaryOp::Div => a / b,
                    BinaryOp::Mod => a % b,
                    _ => return Err(Error::invalid_binop()),
                };
                Ok(Value::Float(result))
            }
            
            // Comparison operations
            (Value::Int(a), Value::Int(b)) if binary.op.is_comparison() => {
                let result = match binary.op {
                    BinaryOp::Eq => a == b,
                    BinaryOp::Ne => a != b,
                    BinaryOp::Lt => a < b,
                    BinaryOp::Le => a <= b,
                    BinaryOp::Gt => a > b,
                    BinaryOp::Ge => a >= b,
                    _ => unreachable!(),
                };
                Ok(Value::Bool(result))
            }
            
            // String concatenation
            (Value::String(a), Value::String(b)) if binary.op == BinaryOp::Add => {
                Ok(Value::String(GcString::new(format!("{}{}", a.as_str(), b.as_str()))))
            }
            
            // String comparison
            (Value::String(a), Value::String(b)) if binary.op.is_comparison() => {
                let result = match binary.op {
                    BinaryOp::Eq => a.as_str() == b.as_str(),
                    BinaryOp::Ne => a.as_str() != b.as_str(),
                    BinaryOp::Lt => a.as_str() < b.as_str(),
                    BinaryOp::Le => a.as_str() <= b.as_str(),
                    BinaryOp::Gt => a.as_str() > b.as_str(),
                    BinaryOp::Ge => a.as_str() >= b.as_str(),
                    _ => unreachable!(),
                };
                Ok(Value::Bool(result))
            }
            
            _ => Err(Error::type_mismatch(&left, &right)),
        }
    }
    
    /// Evaluate function call
    fn eval_call(&mut self, call: &CallExpr) -> Result!Value {
        // Evaluate function
        let func = self.eval_expr(&call.func)?;
        
        // Evaluate arguments
        let mut args = Vec::new();
        for arg in &call.args {
            args.push(self.eval_expr(arg)?);
        }
        
        match func {
            Value::Closure { params, body, env } => {
                // Create new scope
                self.env.push_scope(ScopeKind::Function);
                
                // Bind parameters
                for (param, arg) in params.iter().zip(args.iter()) {
                    self.env.define(param.clone(), arg.clone());
                }
                
                // Restore closure environment
                for (name, value) in env {
                    self.env.define(name, value);
                }
                
                // Evaluate body
                let result = self.eval_expr(&body);
                
                // Pop scope
                self.env.pop_scope();
                
                // Handle return
                match result {
                    Ok(value) => Ok(value),
                    Err(Error::Return(value)) => Ok(value),
                    Err(e) => Err(e),
                }
            }
            
            Value::NativeFunc(f) => {
                f(args)
            }
            
            _ => Err(Error::not_callable(&func)),
        }
    }
    
    /// Evaluate block
    fn eval_block(&mut self, block: &Block) -> Result!Value {
        self.env.push_scope(ScopeKind::Block);
        
        let mut result = Value::Null;
        
        for stmt in &block.statements {
            result = self.eval_statement(stmt)?;
        }
        
        // Final expression
        if let Some(final_expr) = &block.final_expr {
            result = self.eval_expr(final_expr)?;
        }
        
        self.env.pop_scope();
        
        Ok(result)
    }
    
    /// Evaluate if expression
    fn eval_if(&mut self, if_expr: &IfExpr) -> Result!Value {
        let condition = self.eval_expr(&if_expr.condition)?;
        
        match condition {
            Value::Bool(true) => self.eval_block(&if_expr.then_block),
            Value::Bool(false) => {
                if let Some(else_expr) = &if_expr.else_block {
                    self.eval_expr(else_expr)
                } else {
                    Ok(Value::Null)
                }
            }
            _ => Err(Error::type_error("condition must be boolean")),
        }
    }
    
    /// Evaluate match expression
    fn eval_match(&mut self, match_expr: &MatchExpr) -> Result!Value {
        let value = self.eval_expr(&match_expr.value)?;
        
        for arm in &match_expr.arms {
            if self.match_pattern(&arm.pattern, &value)? {
                // Check guard
                if let Some(guard) = &arm.guard {
                    let guard_result = self.eval_expr(guard)?;
                    if let Value::Bool(false) = guard_result {
                        continue;
                    }
                }
                
                return match &arm.body {
                    MatchBody::Expr(expr) => self.eval_expr(expr),
                    MatchBody::Block(block) => self.eval_block(block),
                };
            }
        }
        
        Err(Error::non_exhaustive_match())
    }
    
    /// Match a pattern against a value
    fn match_pattern(&mut self, pattern: &Pattern, value: &Value) -> Result!bool {
        match pattern {
            Pattern::Wildcard(_) => Ok(true),
            
            Pattern::Literal(lit, _) => {
                let lit_value = self.eval_literal(lit)?;
                Ok(lit_value == *value)
            }
            
            Pattern::Ident(ident, _mutability) => {
                self.env.define(ident.name.clone(), value.clone());
                Ok(true)
            }
            
            Pattern::Struct(struct_pattern) => {
                if let Value::Struct { type_name, fields } = value {
                    // Check type name
                    if type_name != &struct_pattern.path.to_string() {
                        return Ok(false);
                    }
                    
                    // Match fields
                    for field in &struct_pattern.fields {
                        let field_value = fields.get(&field.name.name)
                            .ok_or_else(|| Error::missing_field(&field.name.name))?;
                        
                        let pattern = field.pattern.as_ref().unwrap_or(
                            &Pattern::Wildcard(Span::default())
                        );
                        
                        if !self.match_pattern(pattern, field_value)? {
                            return Ok(false);
                        }
                    }
                    
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            
            Pattern::Tuple(tuple_pattern) => {
                if let Value::Tuple(elements) = value {
                    if elements.len() != tuple_pattern.elements.len() {
                        return Ok(false);
                    }
                    
                    for (pattern, elem) in tuple_pattern.elements.iter().zip(elements.iter()) {
                        if !self.match_pattern(pattern, elem)? {
                            return Ok(false);
                        }
                    }
                    
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            
            Pattern::Or(or_pattern) => {
                for alternative in &or_pattern.alternatives {
                    if self.match_pattern(alternative, value)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            
            _ => Ok(false),
        }
    }
    
    /// Evaluate loop
    fn eval_loop(&mut self, loop_expr: &LoopExpr) -> Result!Value {
        self.env.push_scope(ScopeKind::Loop);
        
        let mut result = Value::Null;
        
        loop {
            match self.eval_block(&loop_expr.body) {
                Ok(value) => result = value,
                Err(Error::Break(value)) => {
                    result = value;
                    break;
                }
                Err(Error::Continue) => continue,
                Err(e) => {
                    self.env.pop_scope();
                    return Err(e);
                }
            }
        }
        
        self.env.pop_scope();
        Ok(result)
    }
    
    /// Evaluate while loop
    fn eval_while(&mut self, while_expr: &WhileExpr) -> Result!Value {
        self.env.push_scope(ScopeKind::Loop);
        
        let mut result = Value::Null;
        
        loop {
            let condition = self.eval_expr(&while_expr.condition)?;
            
            match condition {
                Value::Bool(true) => {
                    match self.eval_block(&while_expr.body) {
                        Ok(value) => result = value,
                        Err(Error::Break(value)) => {
                            result = value;
                            break;
                        }
                        Err(Error::Continue) => continue,
                        Err(e) => {
                            self.env.pop_scope();
                            return Err(e);
                        }
                    }
                }
                Value::Bool(false) => break,
                _ => {
                    self.env.pop_scope();
                    return Err(Error::type_error("condition must be boolean"));
                }
            }
        }
        
        self.env.pop_scope();
        Ok(result)
    }
    
    /// Evaluate for loop
    fn eval_for(&mut self, for_expr: &ForExpr) -> Result!Value {
        let iterable = self.eval_expr(&for_expr.iterable)?;
        
        let elements = match iterable {
            Value::Array(arr) => arr.iter().cloned().collect(),
            Value::Tuple(tup) => tup.iter().cloned().collect(),
            Value::String(s) => s.chars().map(|c| Value::Char(c)).collect(),
            _ => return Err(Error::not_iterable()),
        };
        
        let mut result = Value::Null;
        
        for elem in elements {
            self.env.push_scope(ScopeKind::Loop);
            
            // Bind pattern
            self.match_pattern(&for_expr.pattern, &elem)?;
            
            match self.eval_block(&for_expr.body) {
                Ok(value) => result = value,
                Err(Error::Break(value)) => {
                    result = value;
                    self.env.pop_scope();
                    break;
                }
                Err(Error::Continue) => {
                    self.env.pop_scope();
                    continue;
                }
                Err(e) => {
                    self.env.pop_scope();
                    return Err(e);
                }
            }
            
            self.env.pop_scope();
        }
        
        Ok(result)
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(ScopeKind::Global)],
            functions: HashMap::new(),
        }
    }
    
    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope::new(kind));
    }
    
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    
    pub fn define(&mut self, name: string, value: Value) {
        self.scopes.last_mut().unwrap().bindings.insert(name, value);
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.bindings.get(name) {
                return Some(value.clone());
            }
        }
        None
    }
    
    pub fn set(&mut self, name: &str, value: Value) -> Result!() {
        for scope in self.scopes.iter_mut().rev() {
            if scope.bindings.contains_key(name) {
                scope.bindings.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(Error::undefined_variable(name))
    }
    
    pub fn define_function(&mut self, name: string, func: FunctionDef) {
        self.functions.insert(name, func);
    }
    
    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(name)
    }
    
    pub fn capture_closure(&self) -> HashMap<string, Value> {
        let mut closure = HashMap::new();
        for scope in &self.scopes {
            for (name, value) in &scope.bindings {
                closure.insert(name.clone(), value.clone());
            }
        }
        closure
    }
}
```

---

## Summary

This comprehensive technical documentation covers:

1. **Language Syntax** - Complete EBNF grammar specification
2. **Compiler Pipeline** - Lexer, Parser, Type Checker, Borrow Checker
3. **Intermediate Representation** - AIR (SSA form)
4. **Optimization** - Constant folding, DCE, inlining, loop optimizations
5. **Code Generation** - LLVM backend
6. **Binary Generation** - ELF/COFF/Mach-O object files
7. **Linker** - Symbol resolution, section layout, relocation
8. **Runtime** - Memory management, exception handling, type information
9. **Interpreter** - AST evaluation for REPL and scripting

Total lines of technical documentation: **3,500+**
