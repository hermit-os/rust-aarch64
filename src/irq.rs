use core::arch::asm;

pub const IRQ_FLAG_F: u64 = 1 << 6;
pub const IRQ_FLAG_I: u64 = 1 << 7;
pub const IRQ_FLAG_A: u64 = 1 << 8;

/// Enable all interrupts
#[inline]
pub fn enable() {
    unsafe {
        asm!(
            "msr daifclr, {mask}",
            mask = const 0b111,
            options(nostack, nomem),
        );
    }
}

/// Disable all interrupts
#[inline]
pub fn disable() {
    unsafe {
        asm!(
            "msr daifset, {mask}",
            mask = const 0b111,
            options(nostack, nomem),
        );
    }
}

/// Disable IRQs (nested)
///
/// Disable all IRQs when unsure if IRQs were enabled at all.
/// This function together with nested_enable can be used
/// in situations when interrupts shouldn't be activated if they
/// were not activated before calling this function.
#[inline]
pub fn nested_disable() -> bool {
    let flags: u64;
    unsafe {
        asm!(
            "mrs {}, daif",
            out(reg) flags,
            options(nostack, nomem),
        );
    }

    let mut was_enabled = true;
    if flags & (IRQ_FLAG_A | IRQ_FLAG_I | IRQ_FLAG_F) > 0 {
        was_enabled = false;
    }

    disable();
    was_enabled
}

/// Enable all IRQs (nested)
///
/// Can be used in conjunction with nested_disable() to only enable
/// interrupts again if they were enabled before.
#[inline]
pub fn nested_enable(was_enabled: bool) {
    if was_enabled {
        enable();
    }
}
