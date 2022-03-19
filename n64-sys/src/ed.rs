#![allow(dead_code)]

use crate::pi;

const REG_BASE: usize = 0xBF80_0000;

const REG_SYS_CFG: u16 = 0x8000;
const REG_KEY: u16 = 0x8004;

const REG_USB_CFG: u16 = 0x0004;
const REG_USB_DAT: u16 = 0x0400;

const USB_LE_CFG: u32 = 0x8000;
const USB_LE_CTR: u32 = 0x4000;

const USB_CFG_ACT: u32 = 0x0200;
const USB_CFG_RD: u32 = 0x0400;
const USB_CFG_WR: u32 = 0x0000;

const USB_STA_ACT: u32 = 0x0200;
const USB_STA_RXF: u32 = 0x0400;
const USB_STA_TXE: u32 = 0x0800;
const USB_STA_PWR: u32 = 0x1000;
const USB_STA_BSY: u32 = 0x2000;

const USB_CMD_RD_NOP: u32 = USB_LE_CFG | USB_LE_CTR | USB_CFG_RD;
const USB_CMD_RD: u32 = USB_LE_CFG | USB_LE_CTR | USB_CFG_RD | USB_CFG_ACT;
const USB_CMD_WR_NOP: u32 = USB_LE_CFG | USB_LE_CTR | USB_CFG_WR;
const USB_CMD_WR: u32 = USB_LE_CFG | USB_LE_CTR | USB_CFG_WR | USB_CFG_ACT;

fn register_write(reg: u16, val: u32) {
    pi::write(&val as *const u32 as _, 4, REG_BASE + reg as usize);
}

fn register_read(reg: u16) -> u32 {
    let mut val = 0;
    pi::read(&mut val as *mut u32 as _, 4, REG_BASE + reg as usize);
    val
}

pub fn init() {
    register_write(REG_KEY, 0xAA55);
    register_write(REG_SYS_CFG, 0);

    let mut buff = [0u8; 512];
    register_write(REG_USB_CFG, USB_CMD_RD_NOP); //turn off usb r/w activity

    // flush fifo buffer
    while can_read() {
        if !usb_read(&mut buff) {
            break;
        }
    }
}

fn can_read() -> bool {
    register_read(REG_USB_CFG) & (USB_STA_PWR | USB_STA_RXF) == USB_STA_PWR
}

fn can_write() -> bool {
    register_read(REG_USB_CFG) & (USB_STA_PWR | USB_STA_TXE) == USB_STA_PWR
}

fn usb_wait_unitil_done() {
    while (register_read(REG_USB_CFG) & USB_STA_ACT) != 0 {}
}

pub fn usb_read(dst: &mut [u8]) -> bool {
    let mut remaining = dst.len();
    let mut dst = dst.as_mut_ptr();

    while remaining > 0 {
        let buffer_len = if remaining < 512 { remaining } else { 512 };
        let buffer_addr = (512 - buffer_len) as u32;

        register_write(REG_USB_CFG, USB_CMD_RD | buffer_addr);
        usb_wait_unitil_done();

        pi::read(
            dst,
            buffer_len as u32,
            REG_BASE + REG_USB_DAT as usize + buffer_addr as usize,
        );

        dst = ((dst as usize) + buffer_len) as _;
        remaining -= buffer_len;
    }

    true
}

pub fn usb_write(src: &[u8]) -> bool {
    let mut remaining = src.len();
    let mut src = src.as_ptr();

    register_write(REG_USB_CFG, USB_CMD_WR_NOP);

    while remaining > 0 {
        let buffer_len = if remaining < 512 { remaining } else { 512 };
        let buffer_addr = (514 - buffer_len) as u32;

        pi::write(
            src,
            buffer_len as u32,
            REG_BASE + REG_USB_DAT as usize + buffer_addr as usize,
        );
        src = ((src as usize) + 512) as _;

        register_write(REG_USB_CFG, USB_CMD_WR | buffer_addr);

        usb_wait_unitil_done();

        remaining -= buffer_len;
    }

    true
}
