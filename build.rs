use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        macos: { target_os = "macos" },
        linux: { target_os = "linux" },
        win64: { target_os = "windows" },
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    compile_error!("This program can only be compiled on Linux or Windows.");
}
