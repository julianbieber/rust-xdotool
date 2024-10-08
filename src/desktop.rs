//! Convenience functions for the desktop functionality in xdotool.

use std::process::Output;

use crate::command::options::{SetDesktopOption, SyncOption};
use crate::command::{sub_commands, Command};
use crate::optionvec::OptionVec;
use crate::XServer;

impl XServer {
    /// Activate the window. This command is different from [`focus_window`](../window/fn.focus_window.html): if the window is on another desktop, we will switch to that desktop.
    ///
    /// # Options
    ///
    /// `SyncOption::Sync` Wait until the window is actually activated.
    ///
    /// # Examples
    ///
    /// Switch to the desktop and activate a window:
    ///  
    /// ```
    /// # use xdotool::{OptionVec, desktop};
    /// let output = desktop::activate_window("window-id", OptionVec::new());
    /// ```
    pub fn activate_window(&self, window: &str, options: OptionVec<SyncOption>) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::WindowActivate(options));
        self.run(c, window)
    }

    /// Output the current active window. This command is often more reliable than [`get_window_focus`](../window/fn.get_window_focus.html).
    ///
    /// # Examples
    ///
    /// Get the current active window and print it to the console:
    ///
    /// ```
    /// # use xdotool::desktop;
    /// let output = desktop::get_active_window();
    /// println!("{}", String::from_utf8(output.stdout)?);
    /// # Ok::<(), std::string::FromUtf8Error>(())
    /// ```
    pub fn get_active_window(&self) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::GetActiveWindow);
        self.run(c, "")
    }

    /// Changes the number of desktops or workspaces.
    pub fn set_num_desktops(&self, num: u8) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::SetNumDesktops);
        self.run(c, &num.to_string())
    }

    /// Output the current number of desktops.
    ///
    /// # Examples
    ///
    /// ```
    /// # use xdotool::desktop;
    /// let output = desktop::get_num_desktops();
    /// println!("{}", String::from_utf8(output.stdout)?);
    /// # Ok::<(), std::string::FromUtf8Error>(())
    /// ```
    pub fn get_num_desktops(&self) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::GetNumDesktops);
        self.run(c, "")
    }

    /// Move the viewport to the given position. Not all requests will be obeyed.
    pub fn set_desktop_viewport(&self, x: u16, y: u16) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::SetDesktopViewport);
        let args = format!("{} {}", x, y);
        self.run(c, &args)
    }

    /// Report the current viewport's position.
    ///
    /// Viewports are sometimes used instead of 'virtual desktops' on the some window managers.
    /// A viewport is simply a view on a very large desktop area.
    pub fn get_desktop_viewport(&self) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::GetDesktopViewport);
        self.run(c, "")
    }

    /// Switch to the specified desktop.
    ///
    /// # Options
    ///
    /// - `SetDesktopOption::Relative` Use relative movement instead of absolute. This lets you move relative to the current desktop.
    pub fn set_desktop(&self, desktop_number: u8, options: OptionVec<SetDesktopOption>) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::SetDesktop(options));
        self.run(c, &desktop_number.to_string())
    }

    /// Output the current desktop in view.
    pub fn get_desktop(&self) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::GetDesktop);
        self.run(c, "")
    }

    /// Move a window to a different desktop.
    pub fn set_desktop_for_window(&self, window: &str, desktop_number: u8) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::SetDesktopForWindow);
        let args = format!("{} {}", window, desktop_number);
        self.run(c, &args)
    }

    /// Output the desktop currently containing the given window.
    pub fn get_desktop_for_window(&self, window: &str) -> Output {
        let c = Command::Desktop(sub_commands::Desktop::GetDesktopForWindow);
        self.run(c, window)
    }
}
