use x86_64::structures::idt::Idt;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        // TODO: Add exceptions

        idt.interrupts[1].set_handler_fn(::drivers::keyboard::handler);
        idt
    };
}

pub fn initalize() {
    IDT.load();
    println!("Initalized IDT")
}
