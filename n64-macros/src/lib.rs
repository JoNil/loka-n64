#![cfg_attr(target_vendor = "nintendo64", no_std)]

#[cfg(target_vendor = "nintendo64")]
mod inner {

    use zerocopy::AsBytes;

    const MESSAGE_BUFFER_SIZE: usize = 17;

    #[repr(C, packed)]
    #[derive(AsBytes)]
    pub struct DebugWriteMessageBuffer {
        buffer_message_header: u8,
        buffer: [u8; MESSAGE_BUFFER_SIZE],
    }

    n64_types::static_assert!(core::mem::size_of::<DebugWriteMessageBuffer>() == 18);

    #[repr(C, align(16))]
    pub struct DebugWrite {
        b: DebugWriteMessageBuffer,
        cursor: u16,
    }

    pub static GLOBAL_DEBUG_PRINT: spin::Mutex<DebugWrite> = spin::Mutex::new(DebugWrite {
        b: DebugWriteMessageBuffer {
            buffer_message_header: n64_types::MESSAGE_MAGIC_PRINT,
            buffer: [0; MESSAGE_BUFFER_SIZE],
        },
        cursor: 0,
    });

    impl core::fmt::Write for DebugWrite {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            for byte in s.as_bytes() {
                self.b.buffer[self.cursor as usize] = *byte;
                self.cursor += 1;

                if self.cursor == MESSAGE_BUFFER_SIZE as u16 {
                    core::assert!(n64_sys::ed::usb_write(self.b.as_bytes()));
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
            core::assert!(n64_sys::ed::usb_write(lock.b.as_bytes()));
            lock.cursor = 0;
        }
    }
}

#[cfg(not(target_vendor = "nintendo64"))]
mod inner {
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

#[cfg(target_vendor = "nintendo64")]
pub use inner::*;

#[cfg(not(target_vendor = "nintendo64"))]
pub use inner::*;

#[macro_export]
macro_rules! debugln {
    ($fmt:expr) => {
        $crate::debug!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::debug!(concat!($fmt, "\r\n"), $($arg)*)
    };
}
