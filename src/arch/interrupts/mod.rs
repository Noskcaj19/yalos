use log::{debug, trace};
use pic8259_simple::ChainedPics;

pub mod exception;
pub mod idt;
pub mod irq;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn initialize() {
    idt::IDT.load();
    trace!("Initialized IDT");
    unsafe {
        PICS.lock().initialize();
    }
    trace!("Initialized chained PICs");
    x86_64::instructions::interrupts::enable();
    trace!("Interrupts enabled");
    debug!("Interrupts ready")
}
