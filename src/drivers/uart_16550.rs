use core::fmt;

use io::Io;
use io::Port;

bitflags! {
    /// Interrupt enable flags
    struct InterruptEnableFlags: u8 {
        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGE = 1 << 3;
        // 4 to 7 are unused
    }
}

bitflags! {
    /// Line status flags
    struct LineStatus: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}

bitflags! {
    /// Line control configuration bits
    struct LineControlFlags: u8 {
        /// 5 bit char
        const FIVE_BITS  = 0b00;
        /// 6 bit char
        const SIX_BITS   = 0b01;
        /// 7 bit char
        const SEVEN_BITS = 0b10;
        /// 8 bit char
        const EIGHT_BITS = 0b11;


        /// 2 (or 1.5) bit stop
        const TWO_BIT_STOP = 0b1 << 2;

        const ODD_PARITY   = 0b001 << 3;
        const EVEN_PARITY  = 0b011 << 3;
        const MARK_PARITY  = 0b101 << 3;
        const SPACE_PARITY = 0b111 << 3;
    }
}

#[allow(dead_code)]
pub struct SerialPort {
    data: Port<u8>,
    int_ctrl: Port<u8>,
    fifo_ctrl: Port<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_status: Port<u8>,
    modem_status: Port<u8>,
    scratch: Port<u8>,
}

impl SerialPort {
    /// Create a new `SerialPort` with a base COM IO address
    pub const fn new(base: u16) -> SerialPort {
        SerialPort {
            data: Port::new(base),
            int_ctrl: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_status: Port::new(base + 5),
            modem_status: Port::new(base + 6),
            scratch: Port::new(base + 7),
        }
    }

    /// Intialize the hardware
    pub fn init(&self) {
        // Disable interrupts
        self.int_ctrl.write(0x00);
        // Set DLAB (Divisor Latch Access Bit is the MSB in the LCR), this remaps the int_ctrl register
        self.line_ctrl.write(1 << 7);
        // Set baud divisor (int_ctrl is remapped above)
        self.data.write(0x03);
        self.int_ctrl.write(0x00);
        // 8 bits, no parity, one stop bit, unmaps the int_ctrl
        self.line_ctrl.write(LineControlFlags::EIGHT_BITS.bits());
        // TODO: Research these two
        // Enable FIFO, clear them, with 14-byte threshold
        self.fifo_ctrl.write(0xC7);
        // Enable IRQs, set RTS/DSR
        self.modem_ctrl.write(0x0B);
        // Enable interrupts
        self.int_ctrl.write(0x01);
    }

    fn line_status(&mut self) -> LineStatus {
        LineStatus::from_bits_truncate(self.line_status.read())
    }

    /// Receive a value
    pub fn receive(&mut self) -> Option<u8> {
        if self.line_status().contains(LineStatus::INPUT_FULL) {
            Some(self.data.read())
        } else {
            None
        }
    }

    /// Read avalible data to a buffer
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut c = 0;
        while self.line_status().contains(LineStatus::INPUT_FULL) && c < buf.len() {
            buf[c] = self.data.read();
            c += 1;
        }
        c
    }

    /// Send a value
    pub fn send(&mut self, data: u8) {
        while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {}

        self.data.write(data);
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.bytes() {
            self.send(ch)
        }

        Ok(())
    }
}

// impl io::Read for SerialPort {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//         let c = 0;
//         while self.line_status().contains(LineStatus::INPUT_FULL) && c < buf.len() {
//             buf[c] = self.data.read();
//         }
//         c
//     }
// }
