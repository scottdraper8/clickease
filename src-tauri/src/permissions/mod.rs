use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PermissionStatus {
    pub has_accessibility: bool,
    pub is_admin: bool,
}

pub fn get_status() -> PermissionStatus {
    PermissionStatus {
        has_accessibility: check_accessibility(),
        is_admin: check_admin(),
    }
}

#[cfg(target_os = "macos")]
pub fn check_accessibility() -> bool {
    unsafe { accessibility_sys::AXIsProcessTrusted() }
}

#[cfg(not(target_os = "macos"))]
pub fn check_accessibility() -> bool {
    true
}

#[cfg(target_os = "windows")]
pub fn check_admin() -> bool {
    use std::ptr;
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token: HANDLE = HANDLE(ptr::null_mut());
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let res = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut size,
        );

        if res.is_err() {
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}

#[cfg(not(target_os = "windows"))]
pub fn check_admin() -> bool {
    #[cfg(target_os = "linux")]
    {
        unsafe { libc::getuid() == 0 }
    }
    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}

#[cfg(target_os = "macos")]
pub fn request_accessibility() {
    use core_foundation::base::TCFType;
    use core_foundation::boolean::CFBoolean;
    use core_foundation::dictionary::CFDictionary;
    use core_foundation::string::CFString;

    let key = CFString::from_static_string("kAXTrustedCheckOptionPrompt");
    let value = CFBoolean::true_value();
    let options = CFDictionary::from_CFType_pairs(&[(key.as_CFType(), value.as_CFType())]);

    unsafe {
        accessibility_sys::AXIsProcessTrustedWithOptions(
            options.as_concrete_TypeRef() as *const _ as *const _
        );
    }
}

#[cfg(not(target_os = "macos"))]
pub fn request_accessibility() {
    // No-op or platform specific
}
