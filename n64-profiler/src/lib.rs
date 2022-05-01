#![cfg_attr(target_vendor = "nintendo64", no_std)]

#[cfg(target_vendor = "nintendo64")]
mod inner {

    use core::marker::PhantomData;
    use n64_sys::sys::current_time_us;
    use n64_types::{ScopeData, MESSAGE_MAGIC_PROFILER};
    use zerocopy::AsBytes;

    #[repr(C, packed)]
    #[derive(AsBytes)]
    pub struct ProfilerMessageBuffer {
        message_header_buffer: u8,
        current_index: i16,
        scopes: [ScopeData; 1024],
    }

    #[repr(C, align(16))]
    pub struct Profiler {
        b: ProfilerMessageBuffer,
        current_depth: i16,
    }

    impl Profiler {
        #[inline]
        #[must_use]
        pub fn begin_scope(&mut self, name: &'static str) -> i16 {
            let now_us = current_time_us() as i32;

            self.current_depth += 1;

            let index = self.b.current_index;
            self.b.current_index += 1;

            self.b.scopes[self.b.current_index as usize] = ScopeData {
                start: now_us,
                end: 0,
                depth: self.current_depth,
                id: 0,
            };

            index
        }

        #[inline]
        pub fn end_scope(&mut self, index: i16) {
            self.b.scopes[index as usize].end = current_time_us() as i32;
        }

        #[inline]
        pub fn frame(&mut self) {
            core::assert!(n64_sys::ed::usb_write(
                &self.b.as_bytes()
                    [..(3 + self.b.current_index as usize * core::mem::size_of::<ScopeData>())]
            ));
            self.b.current_index = 0;
            self.current_depth = 0;
        }
    }

    pub static GLOBAL_PROFILER: spin::Mutex<Profiler> = spin::Mutex::new(Profiler {
        b: ProfilerMessageBuffer {
            message_header_buffer: MESSAGE_MAGIC_PROFILER,
            current_index: 0,
            scopes: [ScopeData::default(); 1024],
        },
        current_depth: 0,
    });

    pub struct ProfilerScope {
        index: i16,
        _dont_send_me: PhantomData<*const ()>,
    }

    impl ProfilerScope {
        #[inline]
        pub fn new(name: &'static str) -> Self {
            Self {
                index: GLOBAL_PROFILER.lock().begin_scope(name),
                _dont_send_me: PhantomData,
            }
        }
    }

    impl Drop for ProfilerScope {
        #[inline]
        fn drop(&mut self) {
            GLOBAL_PROFILER.lock().end_scope(self.index);
        }
    }

    #[inline]
    pub fn init() {}

    #[macro_export]
    macro_rules! frame {
        () => {
            $crate::GLOBAL_PROFILER.lock().frame();
        };
    }

    #[macro_export]
    macro_rules! scope {
        ($id:expr) => {
            let _profiler_scope = $crate::ProfilerScope::new($id);
        };
    }
}

#[cfg(not(target_vendor = "nintendo64"))]
mod inner {

    pub use puffin;

    pub fn init() {
        Box::leak(Box::new(
            puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap(),
        ));
    }

    #[macro_export]
    macro_rules! function {
        () => {
            let _profiler_scope = $crate::puffin::ProfilerScope::new(
                $crate::puffin::current_function_name!(),
                $crate::puffin::current_file_name!(),
                "",
            );
        };
    }

    #[macro_export]
    macro_rules! scope {
        ($id:expr) => {
            let _profiler_scope =
                $crate::puffin::ProfilerScope::new($id, $crate::puffin::current_file_name!(), "");
        };
    }

    #[macro_export]
    macro_rules! frame {
        () => {
            $crate::puffin::GlobalProfiler::lock().new_frame();
        };
    }
}

#[cfg(target_vendor = "nintendo64")]
pub use inner::*;

#[cfg(not(target_vendor = "nintendo64"))]
pub use inner::*;
