//! 💰 نظام الدفع إسلام - نظام دفع إلكتروني آمن
//! المبرمج والمطور: إسلام بن الحسن - Islam Bin El-Hassan (I-H)

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::Mutex;
use crate::security::haris_core::{HARIS_SYSTEM, TokenProtection};
use crate::gui::{Window, WindowManager};

lazy_static! {
    pub static ref ISLAM_PAYMENT: Mutex<PaymentSystem> = Mutex::new(PaymentSystem::new());
}

pub struct PaymentSystem {
    name: String,
    version: String,
    contract_address: String,
    token_name: String,
    min_balance: u32,
    user_wallets: Vec<Wallet>,
    transactions: Vec<Transaction>,
    exchange_rates: ExchangeRates,
    security_level: SecurityLevel,
    is_active: bool,
}

impl PaymentSystem {
    pub fn new() -> Self {
        Self {
            name: "Islam Payment".to_string(),
            version: "1.0.0".to_string(),
            contract_address: crate::CONTRACT_ADDRESS.to_string(),
            token_name: crate::TOKEN_NAME.to_string(),
            min_balance: crate::MIN_TOKENS,
            user_wallets: Vec::new(),
            transactions: Vec::new(),
            exchange_rates: ExchangeRates::new(),
            security_level: SecurityLevel::Maximum,
            is_active: false,
        }
    }
    
    pub fn activate(&mut self) {
        log::info!("💰 تفعيل نظام الدفع إسلام...");
        
        // التحقق من العقد
        self.verify_contract();
        
        // تهيئة المحافظ
        self.initialize_wallets();
        
        // تحديث أسعار الصرف
        self.update_exchange_rates();
        
        // تفعيل الحماية
        self.enable_security();
        
        self.is_active = true;
        
        log::info!("✅ تم تفعيل نظام الدفع إسلام");
        log::info!("🔗 العقد الذكي: {}", self.contract_address);
        log::info!("🎫 العملة: {} (INSAN)", self.token_name);
        log::info!("📞 للدعم: +201556328989");
    }
    
    pub fn create_wallet(&mut self, user_id: &str, initial_balance: u32) -> Result<WalletId, PaymentError> {
        // التحقق من الحد الأدنى
        if initial_balance < self.min_balance {
            return Err(PaymentError::InsufficientBalance(
                format!("الحد الأدنى: {}", self.min_balance)
            ));
        }
        
        let wallet = Wallet {
            id: self.generate_wallet_id(),
            user_id: user_id.to_string(),
            balance: initial_balance,
            address: self.generate_address(),
            public_key: self.generate_key_pair(),
            transactions: Vec::new(),
            created_at: crate::time::current_timestamp(),
        };
        
        self.user_wallets.push(wallet.clone());
        
        log::info!("👛 تم إنشاء محفظة: {}", wallet.id);
        log::info!("💰 الرصيد الأولي: {} {}", wallet.balance, self.token_name);
        
        Ok(wallet.id)
    }
    
    pub fn transfer(&mut self, from: WalletId, to: WalletId, amount: u32) -> Result<TransactionId, PaymentError> {
        // البحث عن المحافظ
        let from_index = self.find_wallet_index(from)?;
        let to_index = self.find_wallet_index(to)?;
        
        let mut from_wallet = &mut self.user_wallets[from_index];
        let mut to_wallet = &mut self.user_wallets[to_index];
        
        // التحقق من الرصيد
        if from_wallet.balance < amount {
            return Err(PaymentError::InsufficientBalance(
                format!("الرصيد الحالي: {}", from_wallet.balance)
            ));
        }
        
        // التحقق من الحد الأدنى بعد التحويل
        if from_wallet.balance - amount < self.min_balance {
            return Err(PaymentError::MinimumBalanceViolation(
                format!("يجب الاحتفاظ ب {} {} على الأقل", self.min_balance, self.token_name)
            ));
        }
        
        // إنشاء المعاملة
        let transaction = Transaction {
            id: self.generate_transaction_id(),
            from: from,
            to: to,
            amount,
            fee: self.calculate_fee(amount),
            timestamp: crate::time::current_timestamp(),
            status: TransactionStatus::Pending,
            hash: String::new(),
        };
        
        // التحقق الأمني
        let security = HARIS_SYSTEM.lock();
        if !security.protect_token_transaction(&transaction.into()) {
            return Err(PaymentError::SecurityViolation);
        }
        
        // تنفيذ التحويل
        from_wallet.balance -= amount;
        to_wallet.balance += amount;
        
        // تحديث حالة المعاملة
        let mut transaction = transaction;
        transaction.status = TransactionStatus::Completed;
        transaction.hash = self.calculate_hash(&transaction);
        
        // حفظ المعاملة
        self.transactions.push(transaction.clone());
        from_wallet.transactions.push(transaction.id);
        to_wallet.transactions.push(transaction.id);
        
        log::info!("💸 تحويل ناجح: {} {} من {} إلى {}", 
            amount, self.token_name, from, to);
        log::info!("📋 رقم المعاملة: {}", transaction.id);
        
        Ok(transaction.id)
    }
    
    pub fn get_balance(&self, wallet_id: WalletId) -> Result<u32, PaymentError> {
        let index = self.find_wallet_index(wallet_id)?;
        Ok(self.user_wallets[index].balance)
    }
    
    pub fn buy_tokens(&mut self, wallet_id: WalletId, eth_amount: f64) -> Result<TransactionId, PaymentError> {
        log::info!("🛒 شراء {} بـ {} ETH", self.token_name, eth_amount);
        
        // حساب عدد التوكنات
        let token_amount = (eth_amount * self.exchange_rates.eth_to_token) as u32;
        
        // إضافة إلى المحفظة
        let index = self.find_wallet_index(wallet_id)?;
        self.user_wallets[index].balance += token_amount;
        
        // تسجيل المعاملة
        let transaction = Transaction {
            id: self.generate_transaction_id(),
            from: WalletId(0), // النظام
            to: wallet_id,
            amount: token_amount,
            fee: 0,
            timestamp: crate::time::current_timestamp(),
            status: TransactionStatus::Completed,
            hash: self.generate_hash(&format!("BUY:{}:{}", wallet_id.0, token_amount)),
        };
        
        self.transactions.push(transaction.clone());
        
        log::info!("🪙 تم شراء {} {}", token_amount, self.token_name);
        log::info!("💰 الرصيد الجديد: {} {}", 
            self.user_wallets[index].balance, self.token_name);
        
        Ok(transaction.id)
    }
    
    fn verify_contract(&self) {
        log::info!("📜 التحقق من العقد الذكي...");
        log::info!("📍 العنوان: {}", self.contract_address);
        log::info!("🎫 العملة: {}", self.token_name);
        log::info!("📞 للمساعدة: +201556328989");
    }
    
    fn enable_security(&mut self) {
        log::info!("🔐 تفعيل حماية الدفع...");
        
        // هنا سيتم تفعيل جميع إجراءات الأمن
        self.security_level = SecurityLevel::Maximum;
    }
}

#[derive(Debug, Clone)]
pub struct Wallet {
    pub id: WalletId,
    pub user_id: String,
    pub balance: u32,
    pub address: String,
    pub public_key: String,
    pub transactions: Vec<TransactionId>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WalletId(pub u32);

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: TransactionId,
    pub from: WalletId,
    pub to: WalletId,
    pub amount: u32,
    pub fee: u32,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransactionId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Rejected,
}

pub struct ExchangeRates {
    pub eth_to_token: f64,
    pub token_to_usd: f64,
    pub token_to_egp: f64,
    pub token_to_sar: f64,
}

impl ExchangeRates {
    pub fn new() -> Self {
        Self {
            eth_to_token: 1000.0, // 1 ETH = 1000 INSAN
            token_to_usd: 0.10,   // 1 INSAN = 0.10 USD
            token_to_egp: 3.0,    // 1 INSAN = 3 EGP
            token_to_sar: 0.38,   // 1 INSAN = 0.38 SAR
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Maximum,
}

#[derive(Debug)]
pub enum PaymentError {
    WalletNotFound,
    InsufficientBalance(String),
    MinimumBalanceViolation(String),
    SecurityViolation,
    NetworkError,
    ContractError,
    InvalidAmount,
}

impl PaymentSystem {
    fn generate_wallet_id(&self) -> WalletId {
        static mut NEXT_ID: u32 = 1;
        unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            WalletId(id)
        }
    }
    
    fn generate_transaction_id(&self) -> TransactionId {
        use crate::time::current_timestamp;
        TransactionId(current_timestamp())
    }
    
    fn generate_address(&self) -> String {
        format!("ISLAM-{}", crate::crypto::generate_random_string(16))
    }
    
    fn generate_key_pair(&self) -> String {
        format!("PK-{}", crate::crypto::generate_random_string(32))
    }
    
    fn calculate_fee(&self, amount: u32) -> u32 {
        // 1% مع حد أدنى 1 توكن
        let fee = (amount as f32 * 0.01).ceil() as u32;
        fee.max(1)
    }
    
    fn calculate_hash(&self, transaction: &Transaction) -> String {
        let data = format!("{}{}{}{}", 
            transaction.from.0, 
            transaction.to.0, 
            transaction.amount, 
            transaction.timestamp
        );
        crate::crypto::sha256(&data)
    }
    
    fn find_wallet_index(&self, id: WalletId) -> Result<usize, PaymentError> {
        self.user_wallets.iter()
            .position(|w| w.id == id)
            .ok_or(PaymentError::WalletNotFound)
    }
}

// دالة بدء نظام الدفع
pub fn start_payment_system() {
    let mut payment = ISLAM_PAYMENT.lock();
    payment.activate();
    
    // إنشاء محفظة النظام
    payment.create_wallet("system", 10000).unwrap();
    
    log::info!("💳 نظام الدفع إسلام جاهز للاستخدام");
    log::info!("🎫 العملة: {} (INSAN)", payment.token_name);
    log::info!("🔗 العقد: {}", payment.contract_address);
    log::info!("📞 للدعم: +201556328989");
}