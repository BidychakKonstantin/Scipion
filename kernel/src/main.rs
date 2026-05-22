#![no_std]
#![no_main]
pub mod arch;
use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

use  crate::arch::x86_64::idt;
#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    arch::x86_64::idt::IDTEntry::missing();
    arch::x86_64::gdt::init();
    
    // 1. Використовуємо .response() замість .get_response()
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.response() {
        
        if framebuffer_response.framebuffers().len() > 0 {
            let _framebuffer = framebuffer_response.framebuffers()[0];
        }
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

