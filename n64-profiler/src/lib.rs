#![cfg_attr(target_vendor = "nintendo64", no_std)]

cfg_if::cfg_if! {
    if #[cfg(target_vendor = "nintendo64")] {

        extern crate alloc;

        use alloc::vec::Vec;
        use n64_sys::sys::current_time_us;

        struct ScopeData {
            start: i32,
            end: i32,
            name: &'static str,
            depth: i16,
        }

        pub struct Profiler {
            scopes: Vec<ScopeData>,
            current_depth: i16,
        }

        impl Profiler {
            #[inline]
            #[must_use]
            pub fn begin_scope(&mut self, name: &'static str) -> u32 {
                let now_us = current_time_us() as i32;

                self.current_depth += 1;

                self.scopes.push(ScopeData {
                    start: now_us,
                    end: 0,
                    name,
                    depth: self.current_depth,
                });

                (self.scopes.len() - 1) as u32
            }

            #[inline]
            pub fn end_scope(&mut self, index: u32) {
                self.scopes[index as usize].end = current_time_us() as i32;
            }

            pub fn frame(&mut self) {
                self.scopes.clear();
                self.current_depth = 0;
            }
        }

        pub static mut GLOBAL_PROFILER: Profiler = Profiler {
            scopes: Vec::new(),
            current_depth: 0,
        };

        pub struct ProfilerScope {
            index: u32,
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
            Box::new(puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap()).leak();
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
