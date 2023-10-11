use std::cell::RefCell;
use std::mem::transmute;
use std::ptr::null_mut;
use std::rc::Weak;

use winapi::ctypes::c_int;
use winapi::shared::minwindef::{WPARAM, LPARAM, LRESULT, UINT};
use winapi::shared::windef::{HHOOK, HWND};
use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winuser::{CallNextHookEx, SetWindowsHookExW, MSG, WH_GETMESSAGE, HC_ACTION, PM_REMOVE, GetFocus, WM_KEYDOWN};

use crate::Event;

use super::WindowState;

thread_local! {
    static MESSAGE_HOOK: RefCell<Option<MessageHook>> = RefCell::new(None);
}

pub struct MessageHook {
    handle: HHOOK,
    window_state: Vec<Weak<WindowState>>,
}

impl MessageHook {
    pub(super) fn install(window_state: Weak<WindowState>) {
        MESSAGE_HOOK.with(|thread_hook| {
            let mut thread_hook = thread_hook.borrow_mut();
            if thread_hook.is_none() {
                let handle = unsafe { SetWindowsHookExW(WH_GETMESSAGE, Some(get_msg_proc), null_mut(), GetCurrentThreadId()) };
                let hook = Self {
                    handle,
                    window_state: Default::default(),
                };

                *thread_hook = Some(hook);
            }

            thread_hook.as_mut().unwrap().window_state.push(window_state);
        });
    }

    fn process_message(&mut self, hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) {
        self.window_state.retain_mut(|window_state| {
            let Some(window_state) = window_state.upgrade() else {
                return false;
            };

            // Only process messages meant for our parent (which is the DAW window capturing our inputs)
            if hwnd != window_state.parent_hwnd {
                return true;
            }

            let mut keyboard_state = window_state.keyboard_state_mut();
    
            let opt_event = unsafe { keyboard_state.process_message(window_state.hwnd, msg, wparam, lparam) };
    
            if let Some(event) = opt_event {
                let mut window = window_state.create_window();
                let mut window = crate::Window::new(&mut window);
    
                window_state
                    .handler_mut()
                    .as_mut()
                    .unwrap()
                    .on_event(&mut window, Event::Keyboard(event));
            }    

            true
        });
    }
}

unsafe extern "system" fn get_msg_proc(code: c_int, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let msg: &MSG = unsafe {
        let ptr: *const MSG = transmute(lparam);
        &*ptr
    };

    if code == HC_ACTION && wparam == PM_REMOVE as usize {
        MESSAGE_HOOK.with(|h| {
            let mut h = h.borrow_mut();
            let hook = h.as_mut().unwrap();
            hook.process_message(msg.hwnd, msg.message, msg.wParam, msg.lParam)
        });
    }

    // TODO: Let event handlers consume the message, and in that case don't call the next hook
    let hook_handle = MESSAGE_HOOK.with(|h| h.borrow().as_ref().unwrap().handle);
    CallNextHookEx(hook_handle, code, wparam, lparam)
}
