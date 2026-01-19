//! 🌐 متصفح إسلام - متصفح ويب آمن وسريع
//! المبرمج والمطور: إسلام بن الحسن - Islam Bin El-Hassan (I-H)

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::Mutex;
use crate::gui::{Window, WindowManager};
use crate::security::haris_core::HARIS_SYSTEM;
use crate::net::{HttpClient, WebSocket};

lazy_static! {
    pub static ref ISLAM_BROWSER: Mutex<Browser> = Mutex::new(Browser::new());
}

pub struct Browser {
    name: String,
    version: String,
    windows: Vec<BrowserWindow>,
    active_window: Option<usize>,
    history: Vec<HistoryEntry>,
    bookmarks: Vec<Bookmark>,
    security_level: SecurityLevel,
    ad_blocker: AdBlocker,
    privacy_mode: bool,
}

impl Browser {
    pub fn new() -> Self {
        Self {
            name: "Islam Browser".to_string(),
            version: "1.0.0".to_string(),
            windows: Vec::new(),
            active_window: None,
            history: Vec::new(),
            bookmarks: Vec::new(),
            security_level: SecurityLevel::High,
            ad_blocker: AdBlocker::new(),
            privacy_mode: false,
        }
    }
    
    pub fn start(&mut self) {
        log::info!("🌐 بدء تشغيل متصفح إسلام...");
        
        // إنشاء نافذة رئيسية
        let main_window = BrowserWindow::new("متصفح إسلام", 5, 5, 70, 15);
        self.windows.push(main_window);
        self.active_window = Some(0);
        
        // تحميل الصفحة الرئيسية
        self.load_homepage();
        
        log::info!("✅ تم تشغيل متصفح إسلام");
    }
    
    pub fn navigate(&mut self, url: &str) {
        if let Some(index) = self.active_window {
            if let Some(window) = self.windows.get_mut(index) {
                // فحص الأمن قبل التصفح
                if !self.check_url_security(url) {
                    log::warn!("🚨 عنوان غير آمن: {}", url);
                    window.show_warning("عنوان غير آمن!");
                    return;
                }
                
                // حظر الإعلانات
                if self.ad_blocker.should_block(url) {
                    log::info!("🛡️ تم حظر إعلان من: {}", url);
                    return;
                }
                
                // التصفح
                window.navigate(url);
                self.add_to_history(url, &window.title);
                
                log::info!("🌍 تصفح: {}", url);
            }
        }
    }
    
    fn check_url_security(&self, url: &str) -> bool {
        let security = HARIS_SYSTEM.lock();
        
        // قائمة المواقع الإسلامية الآمنة
        let safe_sites = vec![
            "quran.com",
            "sunnah.com",
            "islamweb.net",
            "islamway.net",
            "islamhouse.com",
        ];
        
        for site in safe_sites {
            if url.contains(site) {
                return true;
            }
        }
        
        // فحص بواسطة النظام الأمني
        security.scan_url(url)
    }
    
    fn load_homepage(&mut self) {
        let homepage = r#"
        <html>
        <head>
            <title>متصفح إسلام - الصفحة الرئيسية</title>
        </head>
        <body style="background: #000; color: #8A2BE2; font-family: 'Arabic';">
            <center>
                <h1>🌐 متصفح إسلام</h1>
                <h3>متصفح ويب آمن وسريع من نظام تشغيل إسلام</h3>
                <hr>
                <p>👨💻 المطور: إسلام بن الحسن</p>
                <p>🏢 الشركة: شركة إسلام لتجديد العلوم والتقنيات المستقبلية إسلام-إنسان</p>
                <p>📞 الهاتف: +201556328989</p>
                <p>📧 البريد: islamrenewaltech@gmail.com</p>
                <hr>
                <h4>🔗 روابط إسلامية:</h4>
                <ul>
                    <li><a href="quran.com">القرآن الكريم</a></li>
                    <li><a href="sunnah.com">الحديث النبوي</a></li>
                    <li><a href="islamweb.net">إسلام ويب</a></li>
                </ul>
            </center>
        </body>
        </html>
        "#;
        
        if let Some(index) = self.active_window {
            if let Some(window) = self.windows.get_mut(index) {
                window.load_html(homepage);
                window.title = "متصفح إسلام - الصفحة الرئيسية".to_string();
            }
        }
    }
}

pub struct BrowserWindow {
    pub title: String,
    pub url: String,
    pub content: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub security_indicator: SecurityIndicator,
}

impl BrowserWindow {
    pub fn new(title: &str, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            url: String::new(),
            content: String::new(),
            x,
            y,
            width,
            height,
            tabs: Vec::new(),
            active_tab: 0,
            security_indicator: SecurityIndicator::Secure,
        }
    }
    
    pub fn navigate(&mut self, url: &str) {
        self.url = url.to_string();
        
        // في النسخة الحقيقية، هنا يتم جلب المحتوى من الإنترنت
        self.content = format!("جاري تحميل: {}", url);
        
        // محاكاة التحميل
        self.simulate_loading();
    }
    
    pub fn load_html(&mut self, html: &str) {
        self.content = Self::render_html(html);
    }
    
    fn simulate_loading(&mut self) {
        // محاكاة تحميل الصفحة
        for i in 0..10 {
            self.content = format!("جاري التحميل... {}%", i * 10);
            // تأخير بسيط
            crate::time::sleep_ms(100);
        }
        
        // تحميل محتوى افتراضي
        self.content = Self::render_html(&format!(r#"
            <html>
            <body style="background: #000; color: #8A2BE2;">
                <h1>🌍 {}</h1>
                <p>تم تحميل الصفحة بنجاح بواسطة متصفح إسلام</p>
                <p>👨💻 المطور: إسلام بن الحسن</p>
                <p>📞 للدعم: +201556328989</p>
            </body>
            </html>
        "#, self.url));
    }
    
    fn render_html(html: &str) -> String {
        // محول HTML بسيط للنصوص
        let mut text = String::new();
        let lines: Vec<&str> = html.split('\n').collect();
        
        for line in lines {
            if line.contains("<title>") {
                // استخراج العنوان
                if let Some(start) = line.find("<title>") {
                    if let Some(end) = line.find("</title>") {
                        text.push_str(&line[start+7..end]);
                        text.push('\n');
                    }
                }
            } else if line.contains("<h1>") {
                // العناوين الرئيسية
                if let Some(start) = line.find("<h1>") {
                    if let Some(end) = line.find("</h1>") {
                        text.push_str(&format!("📌 {}\n", &line[start+4..end]));
                    }
                }
            } else if line.contains("<p>") {
                // الفقرات
                if let Some(start) = line.find("<p>") {
                    if let Some(end) = line.find("</p>") {
                        text.push_str(&format!("  {}\n", &line[start+3..end]));
                    }
                }
            } else if line.contains("<li>") {
                // القوائم
                if let Some(start) = line.find("<li>") {
                    if let Some(end) = line.find("</li>") {
                        text.push_str(&format!("  • {}\n", &line[start+4..end]));
                    }
                }
            }
        }
        
        text
    }
    
    pub fn show_warning(&mut self, message: &str) {
        self.content = format!(
            "🚨 تحذير أمني!\n\n{}\n\n🔒 هذا الموقع قد يكون غير آمن.\n\n\
             👨💻 للمساعدة: islamrenewaltech@gmail.com\n📞 الهاتف: +201556328989",
            message
        );
        self.security_indicator = SecurityIndicator::Insecure;
    }
}

pub struct Tab {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub favicon: String,
    pub history: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Maximum,
}

#[derive(Debug, Clone, Copy)]
pub enum SecurityIndicator {
    Secure,
    Insecure,
    Warning,
    Unknown,
}

pub struct AdBlocker {
    blocked_domains: Vec<String>,
    filters: Vec<String>,
    is_enabled: bool,
}

impl AdBlocker {
    pub fn new() -> Self {
        let mut blocker = Self {
            blocked_domains: Vec::new(),
            filters: Vec::new(),
            is_enabled: true,
        };
        
        blocker.load_default_filters();
        blocker
    }
    
    fn load_default_filters(&mut self) {
        // قائمة النطاقات المحظورة
        let domains = vec![
            "doubleclick.net",
            "googleads.com",
            "adservice.google.com",
            "facebook.com/ads",
            "twitter.com/ads",
            "tracking.",
            "analytics.",
            "advertise.",
            "banner.",
            "popup.",
        ];
        
        self.blocked_domains.extend(domains.into_iter().map(String::from));
        
        // قوائم الفلاتر
        self.filters.push(r#"||ads.example.com^"#.to_string());
        self.filters.push(r#"||tracking.example.com^"#.to_string());
    }
    
    pub fn should_block(&self, url: &str) -> bool {
        if !self.is_enabled {
            return false;
        }
        
        for domain in &self.blocked_domains {
            if url.contains(domain) {
                return true;
            }
        }
        
        false
    }
}

pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub timestamp: u64,
    pub visit_count: u32,
}

pub struct Bookmark {
    pub title: String,
    pub url: String,
    pub folder: String,
    pub tags: Vec<String>,
}

// دالة بدء المتصفح
pub fn start_browser() {
    let mut browser = ISLAM_BROWSER.lock();
    browser.start();
    
    // إنشاء نافذة GUI للمتصفح
    let mut wm = crate::gui::WINDOW_MANAGER.lock();
    let handle = wm.create_window("🌐 متصفح إسلام", 10, 5, 60, 18);
    
    log::info!("🌍 متصفح إسلام جاهز للاستخدام");
    log::info!("📞 للدعم: +201556328989");
    log::info!("📧 البريد: islamrenewaltech@gmail.com");
}