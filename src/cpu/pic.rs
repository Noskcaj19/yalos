use drivers::ports::{inb, outb};

const PIC1_CMD_IO_PORT: u16 = 0x0020;
const PIC2_CMD_IO_PORT: u16 = 0x00A0;
const PIC1_DATA_IO_PORT: u16 = 0x0021;
const PIC2_DATA_IO_PORT: u16 = 0x00A1;

// PIC commands
const ICW1: u8 = 0x11;
const ICW4: u8 = 0x1;

// new interrupt vector offsets for remapped PICs
const PIC1_VECTOR_OFFSET: u8 = 0x20;
const PIC2_VECTOR_OFFSET: u8 = 0x28;

/// Remap to PIC so that IRQs 0-7 do not conflict
pub fn remap() {
	let pic1_mask = inb(PIC1_DATA_IO_PORT);
	let pic2_mask = inb(PIC2_DATA_IO_PORT);

	// initialize both PICs
	outb(PIC1_CMD_IO_PORT, ICW1);
	outb(PIC2_CMD_IO_PORT, ICW1);

	// set vector offset of pic1 to 0x20
	outb(PIC1_DATA_IO_PORT, PIC1_VECTOR_OFFSET);
	// set vector offset of pic2 to 0x28
	outb(PIC2_DATA_IO_PORT, PIC2_VECTOR_OFFSET);

	// tell PIC1 about PIC2 at IRQ2 (0000 0100)
	outb(PIC1_DATA_IO_PORT, 4);

	// tell PIC2 its cascade identity (0000 0010)
	outb(PIC2_DATA_IO_PORT, 2);

	// set both PICs to 8086 mode
	outb(PIC1_DATA_IO_PORT, ICW4);
	outb(PIC2_DATA_IO_PORT, ICW4);

	// restore masks
	outb(PIC1_DATA_IO_PORT, pic1_mask);
	outb(PIC2_DATA_IO_PORT, pic2_mask);
}
