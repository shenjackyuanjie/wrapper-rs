fn main() {
    if cfg!(windows) {
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
            .set("ProductVersion", "1.0.0.0")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_CHINESE_SIMPLIFIED,
                winapi::um::winnt::SUBLANG_CHINESE_SIMPLIFIED,
            ));
        res.compile().unwrap();
    }
}
