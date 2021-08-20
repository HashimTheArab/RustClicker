use winapi::um::{winuser, libloaderapi};
use winapi::shared::windef::{HHOOK, HWND};
use winapi::um::winuser::{WM_KEYUP, WM_SYSKEYUP, WM_SYSKEYDOWN, WM_KEYDOWN, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, MSG};
use winapi::um::winnt::LPCSTR;

#[derive(Copy, Clone)]
pub struct KeyboardHook {
    key_up: fn(vk_code: u32),
    key_down: fn(vk_code: u32),
    hhook: HHOOK
}

static mut DATA: Option<KeyboardHook> = Option::None; // lord forgive me

pub unsafe fn new(key_up: fn(vk_code: u32), key_down: fn(vk_code: u32)) {
    DATA = Some(KeyboardHook {
        key_up,
        key_down,
        hhook: winuser::SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_proc), libloaderapi::LoadLibraryA("User32".as_ptr() as LPCSTR), 0)
    });
    let mut msg: MSG = std::mem::zeroed();
    while winuser::GetMessageA(&mut msg, 0 as HWND, 0, 0) != 0 {
        winuser::TranslateMessage(&msg);
        winuser::DispatchMessageA(&msg);
    }
}

/*
if n_code is 0, that means w_param and l_param have valid data.
w_param and l_param contain information about a keyboard message

w_param is the type of keyboard message
it can be one of the following constants: WM_KEYUP, WM_SYSKEYUP, WM_SYSKEYDOWN, WM_KEYDOWN

l_param is a pointer to a KBDLLHOOKSTRUCT
https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-kbdllhookstruct
 */
unsafe extern "system" fn hook_proc(n_code: i32, w_param: usize, l_param: isize) -> isize {
    if n_code == 0 {
        let key: winuser::KBDLLHOOKSTRUCT = *(l_param as *const KBDLLHOOKSTRUCT);
        if w_param == WM_KEYDOWN as usize || w_param == WM_SYSKEYDOWN as usize {
            (DATA.unwrap().key_down)(key.vkCode);
        } else if w_param == WM_KEYUP as usize || w_param == WM_SYSKEYUP as usize {
            (DATA.unwrap().key_up)(key.vkCode);
        }
    }
    return winuser::CallNextHookEx(DATA.unwrap().hhook, n_code, w_param, l_param);
}