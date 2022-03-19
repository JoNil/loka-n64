#![cfg_attr(target_vendor = "nintendo64", no_std)]

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {

        #[repr(align(16))]
        pub struct DebugWrite {
            buffer: [u8; 16],
            cursor: u16,
        }

        pub static GLOBAL_DEBUG_PRINT: spin::Mutex<DebugWrite> = spin::Mutex::new(DebugWrite { buffer: [0; 16], cursor: 0 });

        impl core::fmt::Write for DebugWrite {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {

                for byte in s.as_bytes() {

                    self.buffer[self.cursor as usize] = *byte;
                    self.cursor += 1;

                    if self.cursor == 16 {
                        core::assert!(n64_sys::ed::usb_write(&self.buffer));
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
                lock.buffer[(cursor as usize)..].fill(b'x');
                core::assert!(n64_sys::ed::usb_write(&lock.buffer));
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
        $crate::debug!(concat!($fmt, "00"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::debug!(concat!($fmt, "00"), $($arg)*)
    };
}
