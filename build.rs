#[cfg(windows)]
fn main() {
    use std::io::Write;
    // 创建一个临时的 manifest 文件
    let mut res = winres::WindowsResource::new();
    res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="asInvoker" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#);
    
    // 设置程序图标（如果有的话）
    // res.set_icon("path/to/icon.ico");
    
    if let Err(e) = res.compile() {
        write!(std::io::stderr(), "{}", e).unwrap();
        std::process::exit(1);
    }
}

#[cfg(not(windows))]
fn main() {} 