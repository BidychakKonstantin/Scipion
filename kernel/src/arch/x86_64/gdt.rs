#[derive(Copy,Clone)]
#[repr(transparent)]
pub struct GDTEntry(pub u64);

impl GDTEntry
{
    pub const fn create_empty() -> Self
    {
        GDTEntry(0)
    }
    pub const fn  new(acess:u8,flags:u8) -> Self
    {
        let mut descriptor = 0u64;
        descriptor |=0xFFFF;
        descriptor |= (acess as u64) << 40;
       descriptor |= (0xFf as u64) << 48;
       descriptor |= ((flags | 0x8)as u64) <<52;
       GDTEntry(descriptor)
        
    }
}

#[repr(C, packed)]
struct GDTR {
    limit: u16,
    base: u64,
}

#[repr(C)]
pub struct GDT
{
 pub entries:[GDTEntry;3],
}
pub static  GDT_TABLE: GDT =GDT{
 entries:[
  GDTEntry::create_empty(),
    GDTEntry::new(0x9A, 0x20),
        GDTEntry::new(0x92, 0x00),
 ],
};
pub fn init() {
    use core::arch::asm;
    use core::mem::size_of;

    let gdt_array = &GDT_TABLE.entries;

    let gdtr = GDTR {
        limit: (size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: gdt_array.as_ptr() as u64,
    };

    unsafe {
        asm!("lgdt [{}]", in(reg) &gdtr, options(readonly, nostack));

    asm!(
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        options(nostack)
    );

    asm!(
        "push 0x08",        
        "lea rax, [rip + 2f]", 
        "push rax", 
        "retfq",  
        "2:",
        out("rax") _,
        options(nostack)
    );
    }
    
}