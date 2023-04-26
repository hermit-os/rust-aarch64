use aarch64_cpu::registers::DAIF;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

/// Enable interrupts
#[inline]
pub fn enable() {
    DAIF.modify(DAIF::A::Masked + DAIF::I::Masked + DAIF::F::Masked);
}

/// Disable interrupts
#[inline]
pub fn disable() {
    DAIF.modify(DAIF::A::Unmasked + DAIF::I::Unmasked + DAIF::F::Unmasked);
}

/// Disable IRQs (nested)
///
/// Disable all IRQs when unsure if IRQs were enabled at all.
/// This function together with nested_enable can be used
/// in situations when interrupts shouldn't be activated if they
/// were not activated before calling this function.
#[inline]
pub fn nested_disable() -> u64 {
    let bits = DAIF.get();
    disable();
    bits
}

/// Enable all IRQs (nested)
///
/// Can be used in conjunction with nested_disable() to only enable
/// interrupts again if they were enabled before.
#[inline]
pub fn nested_enable(bits: u64) {
    DAIF.set(bits);
}
