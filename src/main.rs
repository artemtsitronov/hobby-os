#![no_std]
#![no_main]

mod colors;
mod font;
mod framebuffer;
mod text_driver;

use colors::Colors;
use core::{arch::asm, panic::PanicInfo};
use framebuffer::Framebuffer;
use limine::{
    BaseRevision,
    request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker},
};
use text_driver::TextDriver;

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

            let mut text_driver: TextDriver = TextDriver::new(framebuffer);

            // Message
            text_driver.write_str("Hello dear Rustaceans.\n");

            text_driver.set_fg(Colors::RED);
            text_driver.set_bg(Colors::WHITE);
            text_driver.write_str("I WAS VICTIM ");
            text_driver.set_fg(Colors::WHITE);
            text_driver.set_bg(Colors::BLACK);
            text_driver.write_str("of the common believe, that I could write an OS.\n");
            text_driver.set_fg(Colors::BLACK);
            text_driver.set_bg(Colors::GREEN);
            text_driver.write_str("So, here we are. HobbyOS. Nothing more.\n");
            text_driver.set_fg(Colors::WHITE);
            text_driver.set_bg(Colors::BLACK);
            text_driver
                .write_str("This is built with Limine bootloader, and it's my first OS so far.\n");
            text_driver
                .write_str("Please go easy on a newbie in oSdEv, who refuses to vibe-code.\n\n");

            text_driver.write_str("If you want to ");
            text_driver.set_fg(Colors::CYAN);
            text_driver.write_str("SUPPORT THIS PROJECT");
            text_driver.set_fg(Colors::WHITE);
            text_driver.write_str(" then please like the repo in the description :)\n");
            text_driver.write_str("With love, Artem Tsitronov\n");
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
