fn make_lang_id(p: u32, s: u32) -> u16 {
    ((s << 10) | p) as u16
}

fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        // res.set_icon("icon.ico");
        // res.set_manifest(&manifest);
        // res.append_rc_content(&rc);
        res.set("ProductName", "Caller by shenjackyuanjie")
            .set("OriginalFilename", "caller.exe")
            .set("FileDescription", "一个简单的包装器")
            .set("LegalCopyright", "shenjack Copyright © 2024")
            .set("LegalTrademark", "TSW")
            .set("CompanyName", "TSW (shenjack's workshop)")
            .set("Comments", "一个简单的包装器")
            .set("InternalName", "caller.exe")
            .set("ProductVersion", "1.3.0.0")
            .set_language(make_lang_id(
                windows_sys::Win32::System::SystemServices::LANG_CHINESE_SIMPLIFIED,
                windows_sys::Win32::System::SystemServices::SUBLANG_CHINESE_SIMPLIFIED,
            ));
        res.compile().unwrap();
    }
}
