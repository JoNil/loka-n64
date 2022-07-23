use std::{env, fs};

fn main() {
    env::set_current_dir(env::current_exe().unwrap().join("../../../game")).unwrap();
    fs::create_dir("out").ok();
    let out_dir = env::current_dir().unwrap().join("out");
    game_pipeline::run(&out_dir);
}
