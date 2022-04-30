#![cfg_attr(target_vendor = "nintendo64", no_std)]

pub use puffin;

pub struct Profiler {
    _server: puffin_http::Server,
}

impl Profiler {
    pub fn init() -> Self {
        let server =
            puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap();
        Self { _server: server }
    }
}

#[macro_export]
macro_rules! function {
    () => {
        let _profiler_scope = if $crate::are_scopes_on() {
            Some($crate::puffin::ProfilerScope::new(
                $crate::puffin::current_function_name!(),
                $crate::puffin::current_file_name!(),
                "",
            ))
        } else {
            None
        };
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
