use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        macos: { target_os = "macos" },
        linux: { target_os = "linux" },
        win64: { target_os = "windows" },
    }

    #[cfg(not(target_os = "linux"))]
    compile_error!("This program can only be compiled on Linux.");
}
