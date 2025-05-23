use crate::get_error;
use crate::Error;
use libc::c_char;
use libc::c_void;
use std::ffi::{CStr, CString};

use crate::sys;

/// Clipboard utility functions. Access with `VideoSubsystem::clipboard()`.
///
/// These functions require the video subsystem to be initialized.
///
/// ```no_run
/// let sdl_context = sdl3::init().unwrap();
/// let video_subsystem = sdl_context.video().unwrap();
///
/// video_subsystem.clipboard().set_clipboard_text("Hello World!").unwrap();
/// ```
pub struct ClipboardUtil {
    _subsystem: crate::VideoSubsystem,
}

impl crate::VideoSubsystem {
    #[inline]
    pub fn clipboard(&self) -> ClipboardUtil {
        ClipboardUtil {
            _subsystem: self.clone(),
        }
    }
}

impl ClipboardUtil {
    #[doc(alias = "SDL_SetClipboardText")]
    pub fn set_clipboard_text(&self, text: &str) -> Result<(), Error> {
        unsafe {
            let text = CString::new(text).unwrap();
            let result = sys::clipboard::SDL_SetClipboardText(text.as_ptr() as *const c_char);

            if !result {
                Err(get_error())
            } else {
                Ok(())
            }
        }
    }

    #[doc(alias = "SDL_GetClipboardText")]
    pub fn clipboard_text(&self) -> Result<String, Error> {
        unsafe {
            let buf = sys::clipboard::SDL_GetClipboardText();

            if buf.is_null() {
                Err(get_error())
            } else {
                let s = CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned();
                sys::stdinc::SDL_free(buf as *mut c_void);
                Ok(s)
            }
        }
    }

    #[doc(alias = "SDL_HasClipboardText")]
    pub fn has_clipboard_text(&self) -> bool {
        unsafe { sys::clipboard::SDL_HasClipboardText() }
    }
}
