
/// Halt the CPU until the next interrupt occurs
#[inline]
pub fn halt() {
    unsafe {
        asm!("wfi", options(nomem, nostack))
    }
}

mod exceptions {
    /// Generate an exception targeting EL1, with the specified exception code
    #[inline]
    pub unsafe fn svc<const CODE: u16>() {
        asm!("svc #{}", const CODE, options(nomem, nostack));
    }

    /// Generate an exception targeting EL2, with the specified exception code
    #[inline]
    pub unsafe fn hvc<const CODE: u16>() {
        asm!("hvc #{}", const CODE, options(nomem, nostack))
    }

    /// Generate an exception targeting EL3, with the specified exception code
    #[inline]
    pub unsafe fn smc<const CODE: u16>() {
        asm!("smc #{}", const CODE, options(nomem, nostack))
    }

    /// Cause a breakpoint exception by invoking the `brk` instruction, with
    /// the specified code saved in `ESR_ELx.ISS`
    #[inline]
    pub fn brk<const CODE: u16>() {
        unsafe {
            asm!("brk #{}", const CODE, options(nomem, nostack))
        }
    }

    /// Trigger a halt debug event, the architecture will stop and enter a debug state.
    #[inline]
    pub fn debug_halt() {
        unsafe {
            asm!("hlt #0", options(nomem, nostack))
        }
    }
}
