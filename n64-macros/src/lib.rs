#![cfg_attr(target_vendor = "nintendo64", no_std)]

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {

        #[repr(C, packed)]
        pub struct DebugWriteMessageBuffer {
            buffer_message_header: u8,
            buffer: [u8; 18],
        }

        #[repr(C, align(16))]
        pub struct DebugWrite {
            b: DebugWriteMessageBuffer,
            cursor: u16,
        }

        pub static GLOBAL_DEBUG_PRINT: spin::Mutex<DebugWrite> = spin::Mutex::new(DebugWrite {
            b: DebugWriteMessageBuffer {
                buffer_message_header: n64_types::MESSAGE_MAGIC_PRINT,
                buffer: [b'z'; 18],
            },
            cursor: 0,
        });

        impl core::fmt::Write for DebugWrite {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {

                for byte in s.as_bytes() {

                    self.b.buffer[self.cursor as usize] = *byte;
                    self.cursor += 1;

                    if self.cursor == 18 {
                        core::assert!(n64_sys::ed::usb_write(
                            unsafe {
                                core::slice::from_raw_parts(
                                    &self.b as *const DebugWriteMessageBuffer as *const u8,
                                    core::mem::size_of::<DebugWriteMessageBuffer>())
                            }
                        ));
                        self.cursor = 0;
                    }
                }

                Ok(())
            }
        }

        #[macro_export]
        macro_rules! debug {
            ($($arg:tt)*) => {
                <$crate::DebugWrite as core::fmt::Write>::write_fmt(&mut $crate::GLOBAL_DEBUG_PRINT.lock(), format_args!($($arg)*)).ok()
            };
        }

        pub fn debugflush() {
            let mut lock = GLOBAL_DEBUG_PRINT.lock();
            let cursor = lock.cursor;
            if cursor > 0 {
                lock.b.buffer[(cursor as usize)..].fill(b'\r');
                core::assert!(n64_sys::ed::usb_write(
                    unsafe {
                        core::slice::from_raw_parts(
                            &lock.b as *const DebugWriteMessageBuffer as *const u8,
                            core::mem::size_of::<DebugWriteMessageBuffer>())
                    }
                ));
                lock.cursor = 0;
            }
        }

    } else {
        pub struct DebugWrite;

        impl core::fmt::Write for DebugWrite {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                print!("{}", s);
                Ok(())
            }
        }

        #[macro_export]
        macro_rules! debug {
            ($($arg:tt)*) => {
                <$crate::DebugWrite as core::fmt::Write>::write_fmt(&mut $crate::DebugWrite, format_args!($($arg)*)).ok()
            };
        }

        pub fn debugflush() {}
    }
}

#[macro_export]
macro_rules! debugln {
    ($fmt:expr) => {
        $crate::debug!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::debug!(concat!($fmt, "\r\n"), $($arg)*)
    };
}
