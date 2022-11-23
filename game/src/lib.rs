#![cfg_attr(target_vendor = "nintendo64", feature(alloc_error_handler))]
#![cfg_attr(target_vendor = "nintendo64", feature(asm_experimental_arch))]
#![cfg_attr(target_vendor = "nintendo64", feature(lang_items))]
#![cfg_attr(target_vendor = "nintendo64", feature(panic_info_message))]
#![cfg_attr(target_vendor = "nintendo64", feature(start))]
#![cfg_attr(target_vendor = "nintendo64", no_std)]
#![allow(clippy::inconsistent_digit_grouping)]
#![allow(clippy::too_many_arguments)]

extern crate alloc;

pub mod camera;
pub mod components;
pub mod ecs;
pub mod font;
pub mod map;
pub mod maps;
pub mod model;
pub mod models;
pub mod sound;
pub mod sound_mixer;
pub mod sounds;
pub mod textures;
