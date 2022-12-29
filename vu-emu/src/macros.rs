macro_rules! ins {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3].trim_start_matches("vu_emu::Vu::")
    }};
}

pub(crate) use ins;
