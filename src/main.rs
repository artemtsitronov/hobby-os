#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};
use limine::{
    BaseRevision,
    request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker},
};

// - SO, limine searches for these statics,
// and they all have #[used] so that the compiler doesn't
// throw them away if unused.
// They also all point to a section name in the linker.

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END: RequestsEndMarker = RequestsEndMarker::new();

// -- //

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // -> ! {}  means no return type
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn start_fn() -> ! {
    assert!(BASE_REVISION.is_supported()); // is protocol supported?
    if let Some(response) = FRAMEBUFFER.get_response() {
        let screen = response.framebuffers().next();
        if let Some(framebuffer) = screen {
            let pixel_x: u64 = 10;
            let pixel_y: u64 = 10;

            let pixel_offset: u64 = pixel_y * framebuffer.pitch() + pixel_x * 4;
            unsafe {
                let pointer: *mut u32 = framebuffer.addr().add(pixel_offset as usize).cast::<u32>();
                pointer.write(0xFFFFFFFF); // white pixel
            }
        }
    } else {
        panic!("FRAMEBUFFER ISN'T ALIVE! HE ISN'T BREATHING!");
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
