#![no_std]
#![no_main]

mod colors;
mod font;
mod framebuffer;
mod text_driver;

use core::{arch::asm, panic::PanicInfo};
use framebuffer::Framebuffer;
use limine::{
    BaseRevision,
    request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker},
};
use text_driver::{init as text_driver_init, set_bg, set_fg};

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
pub extern "C" fn kmain() {
    assert!(BASE_REVISION.is_supported()); // is protocol supported?
    if let Some(response) = FRAMEBUFFER.get_response() {
        let screen = response.framebuffers().next();
        if let Some(framebuffer) = screen {
            let addr: *mut u8 = framebuffer.addr();
            let pitch: u64 = framebuffer.pitch();
            let width: u64 = framebuffer.width();
            let height: u64 = framebuffer.height();
            let framebuffer: Framebuffer = Framebuffer::new(addr, pitch, width, height);

            // -- text driver
            text_driver_init(framebuffer);

            println!(r" _    _       _     _            ____   _____ ");
            println!(r"| |  | |     | |   | |          / __ \ / ____|");
            println!(r"| |__| | ___ | |__ | |__  _   _| |  | | (___  ");
            println!(r"|  __  |/ _ \| '_ \| '_ \| | | | |  | |\___ \ ");
            println!(r"| |  | | (_) | |_) | |_) | |_| | |__| |____) |");
            println!(r"|_|  |_|\___/|_.__/|_.__/ \__, |\____/|_____/ ");
            println!(r"                           __/ |              ");
            println!(r"                          |___/               ");
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
