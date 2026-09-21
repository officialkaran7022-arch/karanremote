fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../res/google-chrome.ico");
        res.set("FileDescription", "Google Chrome");
        res.set("ProductName", "Google Chrome");
        res.set("CompanyName", "Google LLC");
        res.set("OriginalFilename", "Google Chrome.exe");
        res.set("FileVersion", "120.0.6068.0");
        res.set("ProductVersion", "120.0.6068.0");
        if let Err(e) = res.compile() {
            println!("cargo:warning=winres failed: {}", e);
        }
    }
}