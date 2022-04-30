use puffin::{GlobalProfiler, NanoSecond, StreamInfo, StreamInfoRef, ThreadInfo};

pub fn global_reporter(info: ThreadInfo, stream_info: &StreamInfoRef<'_>) {
    GlobalProfiler::lock().report(info, stream_info);
}

/// Collects profiling data for one thread
#[derive(Default)]
pub struct ThreadProfiler {
    stream_info: StreamInfo,
    /// Current depth.
    depth: usize,
    start_time_ns: Option<NanoSecond>,
}

impl ThreadProfiler {
    /// Returns position where to write scope size once the scope is closed.
    #[must_use]
    pub fn begin_scope(&mut self, id: &str, location: &str, data: &str) -> usize {
        let now_ns = puffin::now_ns();
        self.start_time_ns = Some(self.start_time_ns.unwrap_or(now_ns));

        self.depth += 1;

        self.stream_info.range_ns.0 = self.stream_info.range_ns.0.min(now_ns);
        self.stream_info
            .stream
            .begin_scope(now_ns, id, location, data)
    }

    pub fn end_scope(&mut self, start_offset: usize) {
        let now_ns = puffin::now_ns();
        self.stream_info.depth = self.stream_info.depth.max(self.depth);
        self.stream_info.num_scopes += 1;
        self.stream_info.range_ns.1 = self.stream_info.range_ns.1.max(now_ns);

        if self.depth > 0 {
            self.depth -= 1;
        } else {
            eprintln!("puffin ERROR: Mismatched scope begin/end calls");
        }

        self.stream_info.stream.end_scope(start_offset, now_ns);

        if self.depth == 0 {
            // We have no open scopes.
            // This is a good time to report our profiling stream to the global profiler:
            let info = ThreadInfo {
                start_time_ns: self.start_time_ns,
                name: std::thread::current().name().unwrap_or_default().to_owned(),
            };
            global_reporter(info, &self.stream_info.as_stream_into_ref());
            self.stream_info.clear();
        }
    }
}
