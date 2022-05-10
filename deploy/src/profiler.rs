use n64_types::ScopeData;
use puffin::{GlobalProfiler, NanoSecond, StreamInfo, StreamInfoRef, ThreadInfo};

pub fn global_reporter(info: ThreadInfo, stream_info: &StreamInfoRef<'_>) {
    GlobalProfiler::lock().report(info, stream_info);
}

/// Collects profiling data for one thread
#[derive(Default)]
pub struct N64Profiler {
    stream_info: StreamInfo,
    start_time_ns: Option<NanoSecond>,
    end_time_queue: Vec<(usize, NanoSecond)>,
    current_depth: i32,
}

impl N64Profiler {
    pub fn submit_scope(&mut self, scope: ScopeData) {
        while self.current_depth >= scope.depth as i32 {
            if let Some((start_offset, end_ns)) = self.end_time_queue.pop() {
                self.stream_info.stream.end_scope(start_offset, end_ns);
            } else {
                break;
            }
            self.current_depth = (self.current_depth - 1).max(0);
        }

        let start_ns = scope.start as i64 * 1000;
        let end_ns = scope.end as i64 * 1000;
        let id = scope.id;
        let id = format!("{}", id);

        self.start_time_ns = Some(self.start_time_ns.unwrap_or(start_ns));
        self.current_depth = scope.depth as i32;
        self.stream_info.range_ns.0 = self.stream_info.range_ns.0.min(start_ns);
        self.stream_info.range_ns.1 = self.stream_info.range_ns.1.max(end_ns);
        self.stream_info.depth = self.stream_info.depth.max(scope.depth as usize);
        self.stream_info.num_scopes += 1;

        let start_offset = self
            .stream_info
            .stream
            .begin_scope(start_ns, &id, "n64", "");

        self.end_time_queue.push((start_offset, end_ns));
    }

    pub fn flush_frame(&mut self) {
        while let Some((start_offset, end_ns)) = self.end_time_queue.pop() {
            self.stream_info.stream.end_scope(start_offset, end_ns);
        }

        let info = ThreadInfo {
            start_time_ns: self.start_time_ns,
            name: "N64".to_string(),
        };
        global_reporter(info, &self.stream_info.as_stream_into_ref());

        self.stream_info.clear();
        self.current_depth = 0;
    }
}
