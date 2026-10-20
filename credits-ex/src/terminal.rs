use std::{io, path::Path};

use crossterm::{cursor::{Hide, MoveTo, Show}, execute, terminal::{Clear, ClearType}};

#[cfg(windows)]
use std::{env, ffi::OsString, os::windows::ffi::OsStrExt};
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT},
    System::{
        Com::{COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoInitializeEx, CoUninitialize},
        Console::{CONSOLE_CURSOR_INFO, CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_C_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT, GetConsoleCursorInfo, GetStdHandle, SetConsoleCtrlHandler, SetConsoleCursorInfo, STD_OUTPUT_HANDLE},
        Threading::{CreateEventW, EVENT_MODIFY_STATE, OpenEventW, SYNCHRONIZATION_SYNCHRONIZE, SetEvent, WaitForMultipleObjects, WaitForSingleObject},
    },
    UI::{Shell::{SEE_MASK_FLAG_NO_UI, SEE_MASK_NOASYNC, SHELLEXECUTEINFOW, ShellExecuteExW}, WindowsAndMessaging::SW_SHOWNORMAL},
};

pub struct Terminal {
    #[cfg(windows)]
    events: [HANDLE; 3],
}

pub struct Cursor;

#[cfg(windows)]
impl Drop for Terminal {

    fn drop(&mut self) {
        for event in self.events {
            if !event.is_null() {
                unsafe { CloseHandle(event); }
            }
        }
    }

}

impl Terminal {

    pub fn init(executable: &Path) -> io::Result<Option<Self>> {
        #[cfg(windows)]
        {
            let args: Vec<OsString> = env::args_os().collect::<Vec<OsString>>();
            if args.get(1).is_some_and(|arg: &OsString| arg == "--terminal") {
                let name: &str = args.get(2).and_then(|arg: &OsString| arg.to_str()).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Terminal session name is missing"))?;
                return Ok(Some(Self::open(name, false)?));
            }
            match Self::launch(executable) {
                Ok(()) => return Ok(None),
                Err(error) => eprintln!("Windows Terminal unavailable; playing in the current console. {error}"),
            }
        }
        #[cfg(not(windows))]
        let _ = executable;
        Ok(Some(Self {
            #[cfg(windows)]
            events: [std::ptr::null_mut(); 3],
        }))
    }

    #[cfg(windows)]
    pub fn open(name: &str, create: bool) -> io::Result<Self> {
        let mut terminal: Self = Self { events: [std::ptr::null_mut(); 3] };
        for (index, state) in ["ready", "start", "cancel"].iter().enumerate() {
            let event: Vec<u16> = format!("{name}-{state}").encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
            terminal.events[index] = unsafe {
                if create {
                    CreateEventW(std::ptr::null(), 1, 0, event.as_ptr())
                } else {
                    OpenEventW(EVENT_MODIFY_STATE | SYNCHRONIZATION_SYNCHRONIZE, 0, event.as_ptr())
                }
            };
            if terminal.events[index].is_null() {
                return Err(io::Error::other(format!("{} Terminal {state} event: {}", if create { "Create" } else { "Open" }, io::Error::last_os_error())));
            }
        }
        Ok(terminal)
    }

    #[cfg(windows)]
    pub fn launch(executable: &Path) -> io::Result<()> {
        let time: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(io::Error::other)?.as_nanos();
        let name: String = format!("Local\\credits-ex-{}-{time}", std::process::id());
        let terminal: Self = Self::open(&name, true)?;
        let result: io::Result<()> = (|| {
            let file: Vec<u16> = "wt.exe".encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
            // Windows Terminal parses semicolons even inside quoted arguments.
            let path: Vec<u16> = executable.as_os_str().encode_wide().flat_map(|unit: u16| {
                if unit == b';' as u16 { vec![b'\\' as u16, unit] } else { vec![unit] }
            }).collect::<Vec<u16>>();
            let parameters: Vec<u16> = "-w new new-tab \"".encode_utf16().chain(path).chain(format!("\" --terminal {name}").encode_utf16()).chain(Some(0)).collect::<Vec<u16>>();
            let directory: Vec<u16> = executable.parent().ok_or_else(|| io::Error::other("EXE directory not found"))?.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<u16>>();
            let mut info: SHELLEXECUTEINFOW = SHELLEXECUTEINFOW::default();
            info.cbSize = std::mem::size_of::<SHELLEXECUTEINFOW>() as u32;
            info.fMask = SEE_MASK_NOASYNC | SEE_MASK_FLAG_NO_UI;
            info.lpFile = file.as_ptr();
            info.lpParameters = parameters.as_ptr();
            info.lpDirectory = directory.as_ptr();
            info.nShow = SW_SHOWNORMAL;

            let initialized: i32 = unsafe { CoInitializeEx(std::ptr::null(), (COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE) as u32) };
            if initialized < 0 {
                return Err(io::Error::other(format!("Initialize Terminal launcher COM: HRESULT {initialized:#010x}")));
            }
            let launched: bool = unsafe { ShellExecuteExW(&mut info) != 0 };
            let error: Option<io::Error> = if launched { None } else { Some(io::Error::last_os_error()) };
            unsafe { CoUninitialize(); }
            if let Some(error) = error {
                return Err(io::Error::new(error.kind(), format!("Launch wt.exe: {error}")));
            }

            match unsafe { WaitForSingleObject(terminal.events[0], 10_000) } {
                WAIT_OBJECT_0 => {},
                WAIT_TIMEOUT => return Err(io::Error::new(io::ErrorKind::TimedOut, "Terminal playback did not become ready within 10 seconds")),
                _ => return Err(io::Error::other(format!("Wait for Terminal playback: {}", io::Error::last_os_error()))),
            }
            if unsafe { SetEvent(terminal.events[1]) } == 0 {
                return Err(io::Error::other(format!("Start Terminal playback: {}", io::Error::last_os_error())));
            }
            Ok(())
        })();
        if result.is_err() {
            unsafe { SetEvent(terminal.events[2]); }
        }
        result
    }

    pub fn ready(&self) -> io::Result<bool> {
        #[cfg(windows)]
        if !self.events[0].is_null() {
            if unsafe { SetEvent(self.events[0]) } == 0 {
                return Err(io::Error::other(format!("Signal Terminal playback ready: {}", io::Error::last_os_error())));
            }
            // A late child may play only after the parent explicitly grants permission.
            let events: [HANDLE; 2] = [self.events[2], self.events[1]];
            return match unsafe { WaitForMultipleObjects(events.len() as u32, events.as_ptr(), 0, 10_000) } {
                WAIT_OBJECT_0 | WAIT_TIMEOUT => Ok(false),
                result if result == WAIT_OBJECT_0 + 1 => Ok(true),
                _ => Err(io::Error::other(format!("Wait for Terminal playback start: {}", io::Error::last_os_error()))),
            };
        }
        Ok(true)
    }

    pub fn clear(&self) -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
    }

}

impl Drop for Cursor {

    fn drop(&mut self) {
        let _ = execute!(io::stdout(), Show);
        #[cfg(windows)]
        unsafe {
            SetConsoleCtrlHandler(Some(Self::on_exit), 0);
        }
    }

}

impl Cursor {

    pub fn init() -> io::Result<Self> {
        #[cfg(windows)]
        if !crossterm::ansi_support::supports_ansi() {
            return Err(io::Error::other("ANSI output is not supported by this terminal"));
        }
        let cursor: Self = Self;
        #[cfg(windows)]
        unsafe {
            SetConsoleCtrlHandler(Some(Self::on_exit), 1);
        }
        execute!(io::stdout(), Hide, Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(cursor)
    }

    #[cfg(windows)]
    pub unsafe extern "system" fn on_exit(event: u32) -> i32 {
        if matches!(event, CTRL_C_EVENT | CTRL_BREAK_EVENT | CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT) {
            // The console-control thread must not wait for stdout's lock.
            unsafe {
                let output = GetStdHandle(STD_OUTPUT_HANDLE);
                let mut cursor: CONSOLE_CURSOR_INFO = CONSOLE_CURSOR_INFO::default();
                if GetConsoleCursorInfo(output, &mut cursor) != 0 {
                    cursor.bVisible = 1;
                    SetConsoleCursorInfo(output, &cursor);
                }
            }
        }
        0
    }

}

