// Copyright (c) 2020 Stefan Lankes, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Determines the 64-bit physical count value.
#[inline(always)]
pub unsafe fn get_cntpct_el0() -> u64 {
    let value;
    llvm_asm!("mrs $0, cntpct_el0" : "=r" (value) :: : "volatile");
    value
}
