
/// Halt the CPU until the next interrupt occurs
pub fn hlt() {
    unsafe {
        asm!("hlt", options(nomem, nostack))
    }
}

mod exceptions {
    /// Generate an exception targeting EL1, with the specified exception code
    pub unsafe fn svc<const CODE: u16>() {
        asm!("svc #{}", const CODE, options(nomem, nostack));
    }

    /// Generate an exception targeting EL2, with the specified exception code
    pub unsafe fn hvc<const CODE: u16>() {
        asm!("hvc #{}", const CODE, options(nomem, nostack))
    }

    /// Generate an exception targeting EL3, with the specified exception code
    pub unsafe fn smc<const CODE: u16>() {
        asm!("smc #{}", const CODE, options(nomem, nostack))
    }

    /// Cause a breakpoint exception by invoking the `brk` instruction, with
    /// the specified code saved in `ESR_ELx.ISS`
    pub fn brk<const CODE: u16>() {
        unsafe {
            asm!("brk #{}", const CODE, options(nomem, nostack))
        }
    }
}
