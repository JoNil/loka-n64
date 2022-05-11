use once_cell::sync::Lazy;
use std::{fs::File, io::Write, str::FromStr, sync::Mutex};

struct State {
    next_id: i32,
    out: File,
}

static STATE: Lazy<Mutex<State>> = Lazy::new(|| {
    Mutex::new(State {
        next_id: 0,
        out: File::create("scope_names.txt").unwrap(),
    })
});

#[proc_macro]
pub fn scope_name_to_id(name: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let name = name.to_string();
    let name = name.trim_matches('"');

    let id = {
        let mut lock = STATE.lock().unwrap();
        let id = lock.next_id;
        lock.next_id += 1;

        lock.out
            .write_all(format!("{id};{name}\n").as_bytes())
            .unwrap();

        id
    };

    proc_macro::TokenStream::from_str(&format!("{}", id)).unwrap()
}
