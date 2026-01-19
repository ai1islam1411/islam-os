//! 🪟 نظام النوافذ المتقدم لنظام تشغيل إسلام
//! المبرمج والمطور: إسلام بن الحسن - Islam Bin El-Hassan (I-H)

use spin::Mutex;
use alloc::vec::Vec;
use alloc::string::String;
use crate::drivers::vga::{Color, WRITER};

lazy_static! {
    pub static ref WINDOW_MANAGER: Mutex<WindowManager> = Mutex::new(WindowManager::new());
}

pub struct WindowManager {
    windows: Vec<Window>,
    active_window: Option<usize>,
    theme: Theme,
    desktop_background: Background,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window: None,
            theme: Theme::islamic_dark(),
            desktop_background: Background::default(),
        }
    }
    
    pub fn create_window(&mut self, title: &str, x: u32, y: u32, width: u32, height: u32) -> WindowHandle {
        let window = Window::new(title, x, y, width, height);
        let handle = self.windows.len();
        self.windows.push(window);
        self.active_window = Some(handle);
        
        WindowHandle(handle)
    }
    
    pub fn render(&self) {
        let mut writer = WRITER.lock();
        
        // رسم الخلفية
        self.desktop_background.render(&mut writer);
        
        // رسم جميع النوافذ
        for (i, window) in self.windows.iter().enumerate() {
            let is_active = self.active_window == Some(i);
            window.render(&mut writer, is_active, &self.theme);
        }
        
        // رسم شريط المهام
        self.render_taskbar(&mut writer);
    }
    
    fn render_taskbar(&self, writer: &mut crate::drivers::vga::Writer) {
        let y = 24; // السطر الأخير
        writer.set_color(Color::Black, Color::LightMagenta);
        writer.print_at(" 🕌 نظام تشغيل إسلام ", y, 0);
        
        // عرض الوقت
        let time = "🕒 12:00"; // في النسخة الحقيقية: الوقت الفعلي
        let time_x = 80 - time.len() as u16 - 2;
        writer.print_at(time, y, time_x as usize);
        
        // عرض التوكنات
        let token_text = format!(" 💰 INSAN: 150 ");
        let token_x = time_x - token_text.len() as u16 - 2;
        writer.print_at(&token_text, y, token_x as usize);
    }
}

pub struct Window {
    title: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    content: Vec<WindowContent>,
    is_minimized: bool,
    is_maximized: bool,
}

impl Window {
    pub fn new(title: &str, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            x,
            y,
            width,
            height,
            content: Vec::new(),
            is_minimized: false,
            is_maximized: false,
        }
    }
    
    pub fn render(&self, writer: &mut crate::drivers::vga::Writer, is_active: bool, theme: &Theme) {
        if self.is_minimized {
            return;
        }
        
        // تحديد الألوان حسب النشاط
        let (bg_color, fg_color) = if is_active {
            (theme.window_active_bg, theme.window_active_fg)
        } else {
            (theme.window_inactive_bg, theme.window_inactive_fg)
        };
        
        writer.set_color(fg_color, bg_color);
        
        // رسم إطار النافذة
        self.draw_border(writer);
        
        // رسم شريط العنوان
        self.draw_title_bar(writer, is_active);
        
        // رسم المحتوى
        self.draw_content(writer);
    }
    
    fn draw_border(&self, writer: &mut crate::drivers::vga::Writer) {
        let x = self.x as usize;
        let y = self.y as usize;
        let width = self.width as usize;
        let height = self.height as usize;
        
        // الزوايا
        writer.print_at("┌", y, x);
        writer.print_at("┐", y, x + width - 1);
        writer.print_at("└", y + height - 1, x);
        writer.print_at("┘", y + height - 1, x + width - 1);
        
        // الحواف الأفقية
        for i in 1..width-1 {
            writer.print_at("─", y, x + i);
            writer.print_at("─", y + height - 1, x + i);
        }
        
        // الحواف الرأسية
        for i in 1..height-1 {
            writer.print_at("│", y + i, x);
            writer.print_at("│", y + i, x + width - 1);
        }
    }
    
    fn draw_title_bar(&self, writer: &mut crate::drivers::vga::Writer, is_active: bool) {
        let x = self.x as usize;
        let y = self.y as usize;
        let width = self.width as usize;
        
        // شريط العنوان
        let title = format!(" {} ", self.title);
        let title_x = x + (width - title.len()) / 2;
        
        writer.set_color(Color::White, Color::Magenta);
        writer.print_at(&title, y, title_x);
        
        // أزرار التحكم
        let buttons = "[—][□][×]";
        writer.print_at(buttons, y, x + width - buttons.len() - 1);
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub window_active_bg: Color,
    pub window_active_fg: Color,
    pub window_inactive_bg: Color,
    pub window_inactive_fg: Color,
    pub button_bg: Color,
    pub button_fg: Color,
}

impl Theme {
    pub fn islamic_dark() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::LightMagenta,
            accent: Color::Magenta,
            window_active_bg: Color::Black,
            window_active_fg: Color::LightMagenta,
            window_inactive_bg: Color::DarkGray,
            window_inactive_fg: Color::LightGray,
            button_bg: Color::Magenta,
            button_fg: Color::White,
        }
    }
    
    pub fn purple_dark() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::LightCyan,
            accent: Color::LightMagenta,
            window_active_bg: Color::from_rgb(30, 0, 50),
            window_active_fg: Color::LightMagenta,
            window_inactive_bg: Color::from_rgb(20, 0, 30),
            window_inactive_fg: Color::LightGray,
            button_bg: Color::from_rgb(100, 0, 150),
            button_fg: Color::White,
        }
    }
}

#[derive(Default)]
pub struct Background {
    pattern: BackgroundPattern,
    color: Color,
    image: Option<BackgroundImage>,
}

impl Background {
    pub fn render(&self, writer: &mut crate::drivers::vga::Writer) {
        writer.set_background_color(self.color);
        writer.clear_screen();
        
        // في النسخة المتقدمة، سيتم رسم النمط أو الصورة
        match self.pattern {
            BackgroundPattern::Solid => {},
            BackgroundPattern::Gradient => self.render_gradient(writer),
            BackgroundPattern::Islamic => self.render_islamic_pattern(writer),
        }
    }
    
    fn render_gradient(&self, writer: &mut crate::drivers::vga::Writer) {
        // رسم تدرج لوني
        for y in 0..25 {
            let intensity = (y as f32 / 25.0 * 100.0) as u8;
            let color = Color::from_rgb(intensity, 0, intensity * 2);
            writer.set_background_color(color);
            
            for x in 0..80 {
                writer.print_at(" ", y, x);
            }
        }
    }
    
    fn render_islamic_pattern(&self, writer: &mut crate::drivers::vga::Writer) {
        // نمط إسلامي بسيط
        let pattern = ["◈", "◇", "⬥", "✷"];
        for y in (0..25).step_by(2) {
            for x in (0..80).step_by(3) {
                let symbol = pattern[(x + y) % pattern.len()];
                writer.set_foreground_color(Color::from_rgb(150, 0, 200));
                writer.print_at(symbol, y, x);
            }
        }
    }
}

pub enum BackgroundPattern {
    Solid,
    Gradient,
    Islamic,
}

pub struct BackgroundImage {
    data: &'static [u8],
    width: u32,
    height: u32,
}

pub struct WindowHandle(usize);

pub enum WindowContent {
    Text(String),
    Button(Button),
    Label(Label),
    Input(InputField),
}

pub struct Button {
    text: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    onclick: Option<fn()>,
}

pub struct Label {
    text: String,
    x: u32,
    y: u32,
}

pub struct InputField {
    text: String,
    x: u32,
    y: u32,
    width: u32,
    placeholder: String,
}