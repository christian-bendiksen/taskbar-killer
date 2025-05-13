use windows::{
    Win32::{Foundation::HWND, UI::WindowsAndMessaging::*},
    core::PCSTR,
};

fn main() {
    unsafe {
        let taskbar: HWND = FindWindowA(PCSTR(b"Shell_TrayWnd\0".as_ptr()), PCSTR::null());

        if taskbar.0 != 0 {
            let _ = ShowWindow(taskbar, SW_HIDE);
            println!("Taskbar killed!");
        } else {
            println!("Could not find the taskbar.");
        }
    }
}
