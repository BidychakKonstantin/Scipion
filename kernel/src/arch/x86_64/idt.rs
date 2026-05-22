use core::arch::asm;

#[repr(C,packed)]
pub struct IDTEntry{
    pub pointer_low:u16,
     pub codesegment:u16,
     pub options:u16,
     pub  pointer_mid:u16,
    pub  pointer_high:u32,
      pub  reserved:u32,
 }

impl IDTEntry
{
     pub const  fn missing() -> Self
     {
      Self { pointer_low:0, codesegment: 0, options: 0, pointer_mid: 0, pointer_high: 0, reserved: 0 }
     }
     pub fn init (handler_addrress:u64)-> Self
    { 
         Self{
        pointer_low: handler_addrress as u16,
        pointer_mid:(handler_addrress >> 16) as u16,
        pointer_high:(handler_addrress >> 32) as u32,
        options:0x8E00,
        codesegment:0x08,
        reserved:0,
            }
        }
   pub fn set_ist(&mut self,index:u8)
    { 
      let index = index & 0x7;
       self.options = (self.options & !0x7) | (index as u16);
    }
}
#[repr(C,packed)]
pub struct IDTR
{
      limit:u16,
     base:u64,  
}

static mut IDT: [IDTEntry;256] = [const {IDTEntry::missing()};256];

pub  fn load()
{
    unsafe {
   let idtr = IDTR{
    limit:(core::mem::size_of::<[IDTEntry;256]>() -1) as u16,
    base:&IDT as *const _ as u64,
   };
     asm!(
        "lidt [{}]",
        in(reg)  &idtr,
        options(nostack),
     )
    }
}
