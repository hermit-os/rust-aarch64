// Copyright (c) 2020 Stefan Lankes, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(dead_code)]
#![cfg_attr(feature = "nightly", feature(asm_const))]
#![no_std]

pub mod instructions;
pub mod irq;
pub mod paging;
pub mod regs;
