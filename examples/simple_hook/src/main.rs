use minhook::static_hooks;
use windows::core::{IntoParam, PCSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_OK, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

static_hooks! {
    // Create a hook for user32::MessageBoxA.
    impl MESSAGE_BOX_A_HOOK for "MessageBoxA" in "user32.dll": unsafe extern "system" fn(HWND, PCSTR, PCSTR, MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT;
}

fn main() {
    // Create a detour closure. This closure can capture any Sync variables.
    let detour = |wnd, _text, caption, flags| unsafe {
        MESSAGE_BOX_A_HOOK.call_real(wnd, PCSTR("hooked text!\0".as_ptr()), caption, flags)
    };

    // Install the hook.
    unsafe {
        MESSAGE_BOX_A_HOOK.initialize(detour).unwrap();
    }

    let text = "hello";
    let caption = "MinHook testing caption";

    // Call the function.
    unsafe {
        MessageBoxA(HWND::default(), text.clone(), caption.clone(), MB_OK);
    }

    // Enable the hook.
    MESSAGE_BOX_A_HOOK.enable().unwrap();

    // Call the - now hooked - function.
    unsafe {
        MessageBoxA(HWND::default(), text, caption, MB_OK);
    }
}
