use std::env;

mod rustc_wrapper;

pub fn rustc_wrapper_main() -> Option<i32> {
    if env::var("RA_RUSTC_WRAPPER").is_ok() {
        let mut args = env::args_os();
        let _me = args.next().unwrap();
        let rustc = args.next().unwrap();
        let code = match rustc_wrapper::run_rustc_skipping_cargo_checking(rustc, args.collect()) {
            Ok(rustc_wrapper::ExitCode(code)) => code.unwrap_or(102),
            Err(err) => {
                eprintln!("{}", err);
                101
            }
        };
        return Some(code);
    }
    return None;
}
