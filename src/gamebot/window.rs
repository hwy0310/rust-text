#[cfg(target_os = "windows")]
pub fn select_window_handle() -> anyhow::Result<isize> {
    use std::ffi::c_void;
    use std::io::{self, Write};
    use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible};

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let list = &mut *(lparam as *mut Vec<(isize, String)>);
        if IsWindowVisible(hwnd) == 0 {
            return 1;
        }
        let len = GetWindowTextLengthW(hwnd);
        if len <= 0 {
            return 1;
        }
        let mut buf = vec![0u16; len as usize + 1];
        let written = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
        if written <= 0 {
            return 1;
        }
        let title = String::from_utf16_lossy(&buf[..written as usize]);
        if title.contains("三国志·战略版 电脑模拟器") {
            list.push((hwnd as isize, title));
        }
        1
    }

    let mut matches = Vec::<(isize, String)>::new();
    unsafe { EnumWindows(Some(enum_proc), &mut matches as *mut _ as *mut c_void as isize) };
    match matches.len() {
        0 => anyhow::bail!("未找到包含目标标题的窗口，请先打开该窗口后重试"),
        1 => Ok(matches[0].0),
        _ => {
            for (i, (_, title)) in matches.iter().enumerate() {
                println!("[{i}] {title}");
            }
            print!("请选择窗口索引: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let idx: usize = input.trim().parse()?;
            matches
                .get(idx)
                .map(|(h, _)| *h)
                .ok_or_else(|| anyhow::anyhow!("索引无效"))
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn select_window_handle() -> anyhow::Result<isize> {
    anyhow::bail!("仅支持 Windows")
}
