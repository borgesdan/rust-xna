use std::mem;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{AdjustWindowRectEx, CreateWindowExW, DefWindowProcW, GetSystemMetrics, GetWindowLongA, LoadCursorW, LoadIconW, MoveWindow, PostMessageA, PostQuitMessage, RegisterClassExW, CS_DBLCLKS, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, GWL_EXSTYLE, GWL_STYLE, IDC_ARROW, IDI_APPLICATION, SM_CXSCREEN, SM_CYSCREEN, WINDOW_EX_STYLE, WINDOW_STYLE, WM_DESTROY, WNDCLASSEXW, WS_EX_TOPMOST, WS_OVERLAPPED, WS_POPUP, WS_SYSMENU, WS_VISIBLE};
use crate::exception;
use crate::framework::game::{GameWindow, GameWindowStyle};
use crate::shared::{ExceptionConverter, XnaResult};
use crate::shared::Exception;
use crate::shared::string_helper::ToWide;

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct WindowsGameWindow {
    hwnd: HWND
}

impl WindowsGameWindow {
    pub fn get_hwnd(&self) -> HWND { self.hwnd }
}

impl GameWindow {
    ///Close the window.
    pub fn close(&self) -> XnaResult<()> {
        unsafe {
            PostMessageA(Some(self.platform.hwnd.clone()), WM_DESTROY, WPARAM(0), LPARAM(0))
                .unwrap_or_throw(exception!("PostMessageA() failed", None))?;
        }

        Ok(())
    }

    pub fn create(&mut self) -> XnaResult<()> {
        self.sanitize();

        unsafe {
            let class_name =  "XnaGameWindow".to_wide();
            let h_module = GetModuleHandleW(None)
                .unwrap_or_exception("GetModuleHandleW failed")?;

            let h_instance = HINSTANCE::from(h_module);

            let wnd_class = WNDCLASSEXW {
                cbSize : mem::size_of::<WNDCLASSEXW>() as u32,
                style : CS_DBLCLKS | CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc : Some(Self::wnd_proc),
                lpszClassName : PCWSTR(class_name.as_ptr()),
                hInstance : h_instance.into(),
                hIcon : LoadIconW(None, IDI_APPLICATION).unwrap_or_throw(exception!("LoadIconW failed", None))?,
                hCursor : LoadCursorW(None, IDC_ARROW).unwrap_or_throw(exception!("LoadCursorW failed", None))?,
                hbrBackground : CreateSolidBrush(COLORREF(0)),
                hIconSm : LoadIconW(None, IDI_APPLICATION).unwrap_or_throw(exception!("LoadIconW failed", None))?,
                ..Default::default()
            };

            RegisterClassExW(&wnd_class);

            let style = Self::convert_window_style_to_u32(&self.style);
            let wn_style = WINDOW_STYLE(style);

            self.platform.hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR(self.title.as_str().to_wide().as_ptr()),
                wn_style,
                0,
                0,
                self.width as i32,
                self.height as i32,
                None,
                None,
                Some(h_instance),
                None,
            ).unwrap_or_throw(exception!("CreateWindowExW failed", None))?;

            if self.style == GameWindowStyle::Windowed {
                self.apply_windowed()?;
            }

            Ok(())
        }
    }

    fn sanitize(&mut self)  {
        if self.width == 0 {
            self.width = 800;
        }

        if self.height == 0 {
            self.height = 480;
        }
    }

    fn apply_windowed(&mut self) -> Result<(), Exception> {
        unsafe {
            let mut win_rect = RECT { left: 0, top: 0, right: self.width as i32, bottom: self.height as i32 };
            let hwnd = &self.platform.hwnd;
            let win_style = GetWindowLongA(*hwnd, GWL_STYLE);
            let win_ex_style = GetWindowLongA(*hwnd, GWL_EXSTYLE);

            let win_style2 = WINDOW_STYLE(win_style as u32);
            let win_ex_style2 = WINDOW_EX_STYLE(win_ex_style as u32);

            AdjustWindowRectEx(&mut win_rect, win_style2, false, win_ex_style2)
                .unwrap_or_exception("AdjustWindowRectEx failed")?;

            let cx_screen = GetSystemMetrics(SM_CXSCREEN);
            let cy_screen = GetSystemMetrics(SM_CYSCREEN);

            self.x = (cx_screen / 2) - ((win_rect.right - win_rect.left) / 2);
            self.y = (cy_screen / 2) - ((win_rect.bottom - win_rect.top) / 2);

            MoveWindow(
                *hwnd,
                self.x,
                self.y,
                win_rect.right - win_rect.left,
                win_rect.bottom - win_rect.top,
                true,
            ).unwrap_or_exception("MoveWindow failed")?;

            Ok(())
        }
    }

    fn convert_window_style_to_u32(style: &GameWindowStyle) -> u32 {
        match style {
            GameWindowStyle::Windowed => WS_OVERLAPPED.0 | WS_SYSMENU.0 | WS_VISIBLE.0,
            GameWindowStyle::FullScreen => WS_POPUP.0 | WS_VISIBLE.0,
            GameWindowStyle::BorderlessFullScreen => WS_EX_TOPMOST.0 | WS_POPUP.0 | WS_VISIBLE.0
        }
    }

    pub fn update(&mut self) -> Result<(), Exception> {
        if self.style == GameWindowStyle::Windowed {
            self.apply_windowed()?;
        }

        Ok(())
    }

    pub extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match msg {
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }
}