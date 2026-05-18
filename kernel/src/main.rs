#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Використовуємо .response() замість .get_response()
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.response() {
        
        // 2. Перевіряємо, чи є хоча б один екран у списку
        if framebuffer_response.framebuffers().len() > 0 {
            // Беремо перший доступний фреймбуфер через [0] замість .next()
            let _framebuffer = framebuffer_response.framebuffers()[0];
            
            // Екран успішно підключено!
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