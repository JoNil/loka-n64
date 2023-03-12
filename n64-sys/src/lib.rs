#![no_std]
#![cfg_attr(target_vendor = "nintendo64", feature(asm_experimental_arch))]
#![cfg_attr(target_vendor = "nintendo64", feature(core_intrinsics))]
#![allow(clippy::missing_safety_doc)]

pub mod ai;
pub mod ed;
pub mod pi;
pub mod rsp;
pub mod si;
pub mod sys;
pub mod vi;
