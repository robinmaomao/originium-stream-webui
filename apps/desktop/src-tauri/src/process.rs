//! Helpers for spawning child processes without opening console windows.

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub trait NoWindow {
    fn no_window(&mut self) -> &mut Self;
}

#[cfg(windows)]
impl NoWindow for std::process::Command {
    fn no_window(&mut self) -> &mut Self {
        use std::os::windows::process::CommandExt;
        self.creation_flags(CREATE_NO_WINDOW)
    }
}

#[cfg(windows)]
impl NoWindow for tokio::process::Command {
    fn no_window(&mut self) -> &mut Self {
        self.creation_flags(CREATE_NO_WINDOW)
    }
}

#[cfg(not(windows))]
impl NoWindow for std::process::Command {
    fn no_window(&mut self) -> &mut Self {
        self
    }
}

#[cfg(not(windows))]
impl NoWindow for tokio::process::Command {
    fn no_window(&mut self) -> &mut Self {
        self
    }
}
