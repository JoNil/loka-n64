#![cfg_attr(target_vendor = "nintendo64", no_std)]

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {

        use n64_sys::sys::current_time_us;
        use n64_types::{ScopeData, PROFILER_MESSAGE_MAGIC};

        #[repr(C, align(16))]
        pub struct Profiler {
            message_header_buffer: u8,
            scopes: [ScopeData; 1024],
            current_index: i16,
            current_depth: i16,
        }

        impl Profiler {
            #[inline]
            #[must_use]
            pub fn begin_scope(&mut self, name: &'static str) -> i16 {
                let now_us = current_time_us() as i32;

                self.current_depth += 1;

                let index = self.current_index;
                self.current_index += 1;

                self.scopes[self.current_index as usize] = ScopeData {
                    start: now_us,
                    end: 0,
                    depth: self.current_depth,
                    id: 0,
                };

                index
            }

            #[inline]
            pub fn end_scope(&mut self, index: i16) {
                self.scopes[index as usize].end = current_time_us() as i32;
            }

            #[inline]
            pub fn frame(&mut self) {
                unsafe {
                    core::assert!(n64_sys::ed::usb_write(
                        core::slice::from_raw_parts(
                            self as *const Profiler as *const u8,
                            1 + (self.current_index - 1) as usize * core::mem::size_of::<ScopeData>())
                    ));
                }
                self.current_index = 0;
                self.current_depth = 0;
            }
        }

        pub static mut GLOBAL_PROFILER: Profiler = Profiler {
            message_header_buffer: PROFILER_MESSAGE_MAGIC,
            scopes: [ScopeData::default(); 1024],
            current_index: 0,
            current_depth: 0,
        };

        pub struct ProfilerScope {
            index: i16,
            _dont_send_me: core::marker::PhantomData<*const ()>,
        }

        impl ProfilerScope {
            #[inline]
            pub fn new(name: &'static str) -> Self {
                Self {
                    index: unsafe { GLOBAL_PROFILER.begin_scope(name) },
                    _dont_send_me: Default::default(),
                }
            }
        }

        impl Drop for ProfilerScope {
            #[inline]
            fn drop(&mut self) {
                unsafe { GLOBAL_PROFILER.end_scope(self.index); }
            }
        }

        #[inline]
        pub fn init() {}

        #[macro_export]
        macro_rules! frame {
            () => {
                unsafe {
                    $crate::GLOBAL_PROFILER.frame();
                }
            };
        }

        #[macro_export]
        macro_rules! scope {
            ($id:expr) => {
                let _profiler_scope =
                    $crate::ProfilerScope::new($id);
            };
        }

    } else {

        pub use puffin;

        pub fn init() {
            Box::leak(Box::new(puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap()));
        }

        #[macro_export]
        macro_rules! function {
            () => {
                let _profiler_scope =
                    $crate::puffin::ProfilerScope::new(
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
}
