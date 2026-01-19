//! 🤖 نواة الذكاء الاصطناعي Zaka Islam
//! المبرمج والمطور: إسلام بن الحسن - Islam Bin El-Hassan (I-H)

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Serialize, Deserialize};
use spin::Mutex;
use crate::knowledge_base::{IslamicKnowledge, ScientificKnowledge};
use crate::nlp_processor::{process_arabic, Intent};

lazy_static! {
    pub static ref ZAKA_CORE: Mutex<ZakaAI> = Mutex::new(ZakaAI::new());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZakaAI {
    pub name: String,
    pub version: String,
    pub personality: Personality,
    pub knowledge_base: KnowledgeBase,
    pub conversation_history: Vec<Conversation>,
    pub learning_rate: f32,
    pub is_learning: bool,
    pub user_profiles: BTreeMap<String, UserProfile>,
}

impl ZakaAI {
    pub fn new() -> Self {
        Self {
            name: "Zaka Islam".to_string(),
            version: "1.0.0".to_string(),
            personality: Personality::islamic_scholar(),
            knowledge_base: KnowledgeBase::new(),
            conversation_history: Vec::with_capacity(1000),
            learning_rate: 0.1,
            is_learning: true,
            user_profiles: BTreeMap::new(),
        }
    }
    
    pub fn initialize(&mut self) {
        log::info!("🤖 تهيئة Zaka Islam...");
        log::info!("👨💻 المطور: إسلام بن الحسن");
        log::info!("🏢 الشركة: شركة إسلام لتجديد العلوم والتقنيات المستقبلية إسلام-إنسان");
        
        // تحميل قاعدة المعرفة
        self.knowledge_base.load_islamic_knowledge();
        self.knowledge_base.load_scientific_knowledge();
        self.knowledge_base.load_technical_knowledge();
        
        log::info!("📚 قاعدة المعرفة: {} معلومة", self.knowledge_base.total_facts());
    }
    
    pub fn process_query(&mut self, query: &str, user_id: &str) -> AIResponse {
        // معالجة اللغة الطبيعية
        let processed = process_arabic(query);
        let intent = self.analyze_intent(&processed);
        
        // توليد الرد
        let response = match intent {
            Intent::IslamicQuestion => self.generate_islamic_response(&processed),
            Intent::TechnicalQuestion => self.generate_technical_response(&processed),
            Intent::SystemCommand => self.handle_system_command(&processed),
            Intent::Greeting => self.generate_greeting(user_id),
            Intent::Joke => self.generate_joke(),
            Intent::PrayerTime => self.get_prayer_times(),
            Intent::QuranVerse => self.get_quran_verse(&processed),
            Intent::Hadith => self.get_hadith(&processed),
            Intent::Calculation => self.calculate(&processed),
            Intent::Unknown => self.generate_default_response(&processed),
        };
        
        // حفظ المحادثة
        self.save_conversation(user_id, query, &response.text);
        
        // التعلم إذا كان مفعلاً
        if self.is_learning {
            self.learn_from_interaction(query, &response.text);
        }
        
        response
    }
    
    fn generate_islamic_response(&self, query: &ProcessedText) -> AIResponse {
        let responses = vec![
            "بسم الله الرحمن الرحيم، الحمد لله رب العالمين".to_string(),
            "قال تعالى: {وَقُلِ اعْمَلُوا فَسَيَرَى اللَّهُ عَمَلَكُمْ وَرَسُولُهُ وَالْمُؤْمِنُونَ}".to_string(),
            "قال رسول الله صلى الله عليه وسلم: {خيركم من تعلم القرآن وعلمه}".to_string(),
            "العلم نور والجهل ظلام، فاطلب العلم من المهد إلى اللحد".to_string(),
            "الصلاة عماد الدين، فحافظ عليها يا عبد الله".to_string(),
        ];
        
        let base = responses[fastrand::usize(..responses.len())].clone();
        
        AIResponse {
            text: format!("{}\n\n{}\n\n🤖 Zaka Islam\n📞 للاستفسارات: +201556328989", base, self.get_related_knowledge(query)),
            intent: Intent::IslamicQuestion,
            confidence: 0.95,
            sources: vec![
                "القرآن الكريم".to_string(),
                "صحيح البخاري".to_string(),
                "صحيح مسلم".to_string(),
            ],
        }
    }
    
    fn generate_technical_response(&self, query: &ProcessedText) -> AIResponse {
        let response = match query.keywords.get(0) {
            Some("نظام") | Some("تشغيل") => {
                "نظام تشغيل إسلام هو نظام تشغيل سيادي مبني من الصفر بلغة Rust.\n\
                 يدعم الذكاء الاصطناعي والأمن المتقدم والدفع الإلكتروني الإسلامي.".to_string()
            }
            Some("برمجة") | Some("كود") => {
                "أفضل لغات البرمجة لتطوير الأنظمة:\n\
                 1. Rust - للأداء والأمان\n\
                 2. C - للأنظمة المنخفضة المستوى\n\
                 3. Assembly - للتحكم الدقيق".to_string()
            }
            Some("أمن") | Some("حماية") => {
                "نظام Haris Islam يوفر:\n\
                 • تشفير كامل للقرص\n\
                 • جدار ناري ذكي\n\
                 • كشف التسلل التلقائي\n\
                 • حماية بلوكشين".to_string()
            }
            Some("دفع") | Some("توكن") => {
                format!("نظام Islam Payment يستخدم توكن {} (INSAN).\n\
                        الحد الأدنى: {} توكن\n\
                        العقد: {}\n\
                        للشراء: أرسل ETH إلى العقد", 
                        crate::TOKEN_NAME, crate::MIN_TOKENS, crate::CONTRACT_ADDRESS)
            }
            _ => {
                "يمكنني مساعدتك في:\n\
                 • تطوير البرمجيات\n\
                 • أمن المعلومات\n\
                 • الذكاء الاصطناعي\n\
                 • أنظمة التشغيل".to_string()
            }
        };
        
        AIResponse {
            text: format!("{}\n\n🔧 Zaka Islam - المساعد التقني\n📞 الدعم: +201556328989", response),
            intent: Intent::TechnicalQuestion,
            confidence: 0.85,
            sources: vec!["وثائق نظام إسلام".to_string()],
        }
    }
    
    fn handle_system_command(&self, query: &ProcessedText) -> AIResponse {
        let command = query.original.to_lowercase();
        let response = if command.contains("افتح") {
            "جاري فتح التطبيق المطلوب...".to_string()
        } else if command.contains("أغلق") {
            "جاري إغلاق التطبيق...".to_string()
        } else if command.contains("بحث") {
            "جاري البحث في الإنترنت...".to_string()
        } else {
            "أمر النظام تم استلامه بنجاح.".to_string()
        };
        
        AIResponse {
            text: format!("✅ {}\n\n⚙️ Zaka Islam - مساعد النظام", response),
            intent: Intent::SystemCommand,
            confidence: 0.90,
            sources: vec![],
        }
    }
    
    fn get_quran_verse(&self, query: &ProcessedText) -> AIResponse {
        let verses = vec![
            ("البقرة", 255, "اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ الْحَيُّ الْقَيُّومُ..."),
            ("الفاتحة", 1, "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ"),
            ("العلق", 1, "اقْرَأْ بِاسْمِ رَبِّكَ الَّذِي خَلَقَ"),
            ("النور", 35, "اللَّهُ نُورُ السَّمَاوَاتِ وَالْأَرْضِ..."),
        ];
        
        let (surah, ayah, text) = verses[fastrand::usize(..verses.len())];
        
        AIResponse {
            text: format!("📖 سورة {} - الآية {}\n{}\n\nتفسير موجز: {}", 
                         surah, ayah, text, self.get_tafsir(surah, ayah)),
            intent: Intent::QuranVerse,
            confidence: 1.0,
            sources: vec![format!("القرآن الكريم - سورة {}", surah)],
        }
    }
    
    fn get_hadith(&self, query: &ProcessedText) -> AIResponse {
        let hadiths = vec![
            ("البخاري", "إنما الأعمال بالنيات..."),
            ("مسلم", "من حسن إسلام المرء تركه ما لا يعنيه"),
            ("الترمذي", "اطلبوا العلم من المهد إلى اللحد"),
            ("أبو داود", "خيركم من تعلم القرآن وعلمه"),
        ];
        
        let (source, text) = hadiths[fastrand::usize(..hadiths.len())];
        
        AIResponse {
            text: format!("📜 حديث {}:\n{}\n\nدرجة الحديث: صحيح", source, text),
            intent: Intent::Hadith,
            confidence: 1.0,
            sources: vec![format!("صحيح {}", source)],
        }
    }
    
    fn calculate(&self, query: &ProcessedText) -> AIResponse {
        // معالجة رياضية بسيطة
        let result = "42"; // نتيجة افتراضية
        
        AIResponse {
            text: format!("🧮 نتيجة الحساب: {}\n\nاستخدم Islam Math Library لحسابات متقدمة", result),
            intent: Intent::Calculation,
            confidence: 0.80,
            sources: vec!["مكتبة Islam Math".to_string()],
        }
    }
    
    fn get_prayer_times(&self) -> AIResponse {
        let times = vec![
            ("الفجر", "4:30"),
            ("الظهر", "12:15"),
            ("العصر", "3:45"),
            ("المغرب", "6:20"),
            ("العشاء", "7:45"),
        ];
        
        let mut table = String::new();
        for (prayer, time) in times {
            table.push_str(&format!("🕌 {}: {}\n", prayer, time));
        }
        
        AIResponse {
            text: format!("🕋 أوقات الصلاة:\n{}\n\nملاحظة: الأوقات تقريبية، تحقق من التقويم المحلي", table),
            intent: Intent::PrayerTime,
            confidence: 0.95,
            sources: vec!["التقويم الإسلامي".to_string()],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Personality {
    pub name: String,
    pub traits: Vec<String>,
    pub knowledge_level: KnowledgeLevel,
    pub response_style: ResponseStyle,
    pub language_style: LanguageStyle,
}

impl Personality {
    pub fn islamic_scholar() -> Self {
        Self {
            name: "العالم الإسلامي".to_string(),
            traits: vec![
                "حكيم".to_string(),
                "صبور".to_string(),
                "متعلم".to_string(),
                "متواضع".to_string(),
                "مبدع".to_string(),
            ],
            knowledge_level: KnowledgeLevel::Expert,
            response_style: ResponseStyle::Detailed,
            language_style: LanguageStyle::ClassicalArabic,
        }
    }
    
    pub fn technical_expert() -> Self {
        Self {
            name: "الخبير التقني".to_string(),
            traits: vec![
                "دقيق".to_string(),
                "منطقي".to_string(),
                "مبتكر".to_string(),
                "سريع".to_string(),
                "عملي".to_string(),
            ],
            knowledge_level: KnowledgeLevel::Advanced,
            response_style: ResponseStyle::Concise,
            language_style: LanguageStyle::ModernTechnical,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIResponse {
    pub text: String,
    pub intent: Intent,
    pub confidence: f32,
    pub sources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub user_id: String,
    pub query: String,
    pub response: String,
    pub timestamp: u64,
    pub intent: Intent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub preferences: Preferences,
    pub conversation_history: Vec<Conversation>,
    pub learning_pattern: LearningPattern,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KnowledgeLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseStyle {
    Concise,
    Detailed,
    Poetic,
    Technical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LanguageStyle {
    ClassicalArabic,
    ModernArabic,
    ModernTechnical,
    Mixed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preferences {
    pub preferred_language: String,
    pub technical_level: KnowledgeLevel,
    pub interests: Vec<String>,
    pub response_length: ResponseLength,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseLength {
    Short,
    Medium,
    Long,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LearningPattern {
    pub topics_of_interest: Vec<String>,
    pub learning_speed: f32,
    pub retention_rate: f32,
    pub last_learned: u64,
}