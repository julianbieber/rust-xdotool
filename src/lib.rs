//! A simple wrapper for the `xdotool` command line tool.
//!
//! While I've tried my best documenting everything as detailed as possible, please look at the man page of `xdotool` for detailed information.
//!
//! # Examples
//!
//! ```
//!
//! ```

// TODO: Add examples

use std::process::Command;
use std::process::Output;

pub mod command;
pub mod desktop;
pub mod keyboard;
pub mod misc;
pub mod mouse;
pub mod optionvec;
pub mod window;

pub use optionvec::OptionVec;
pub struct XServer {
    pub display: u32,
    pub auth: String,
}

impl XServer {
    /// Execute a xdotool command.
    /// This is the only function you actually need. Every other function is just for convenience.
    /// However using the convenience functions is much more straight forward and therefore more desirable.
    /// You should only use this function if there is no convenience function available.
    ///
    /// # Examples
    ///
    /// Search a window
    ///
    /// ```
    /// use std::io::Write;
    ///
    /// use xdotool::{self, option_vec, OptionVec, window};
    /// use xdotool::command::{self, options, sub_commands, Command};
    ///
    /// let options = option_vec![options::SearchOption::Name];
    /// let cmd = Command::Window(sub_commands::Window::Search(options));
    /// let output = xdotool::run(cmd, "firefox");
    /// std::io::stdout().write_all(&output.stdout).unwrap();
    /// ```
    pub fn run(&self, command: command::Command, args: &str) -> Output {
        let cmd = format!("xdotool {} {}", command, args);
        let display = self.display;

        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .env("DISPLAY", format!(":{display}"))
            .env("XAUTHORITY", self.auth.as_str())
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute 'xdotool key {}", args))
    }

    pub fn screenshot(&self) -> Output {
        let display = self.display;
        Command::new("xwd")
            .env("DISPLAY", format!(":{display}"))
            .env("XAUTHORITY", self.auth.as_str())
            .arg("-root")
            .output()
            .unwrap()
    }
}
