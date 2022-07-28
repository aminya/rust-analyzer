pub mod rustc_wrapper;

const STACK_SIZE: usize = 1024 * 1024 * 8;

/// Parts of rust-analyzer can use a lot of stack space, and some operating systems only give us
/// 1 MB by default (eg. Windows), so this spawns a new thread with hopefully sufficient stack
/// space.
pub fn with_extra_thread(
    thread_name: impl Into<String>,
    thread_intent: stdx::thread::ThreadIntent,
    f: impl FnOnce() -> anyhow::Result<()> + Send + 'static,
) -> anyhow::Result<()> {
    let handle = stdx::thread::Builder::new(thread_intent)
        .name(thread_name.into())
        .stack_size(STACK_SIZE)
        .spawn(f)?;

    handle.join()?;

    Ok(())
}
