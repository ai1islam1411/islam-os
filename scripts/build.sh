#!/bin/bash
# 🏗️ سكريبت بناء نظام تشغيل إسلام - الإصدار الاحترافي
# المبرمج والمطور: إسلام بن الحسن - Islam Bin El-Hassan (I-H)
# الشركة: شركة إسلام لتجديد العلوم والتقنيات المستقبلية إسلام-إنسان
# العنوان: الحي الإفرنجي، مدينة الإسماعيلية، محافظة الإسماعيلية، مصر
# البريد: islamrenewaltech@gmail.com
# الهواتف: +201556328989, +201508599689

set -euo pipefail

# الألوان
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'

# معلومات النظام
OS_NAME="نظام تشغيل إسلام"
OS_VERSION="0.1.0"
DEVELOPER="إسلام بن الحسن - Islam Bin El-Hassan (I-H)"
COMPANY="شركة إسلام لتجديد العلوم والتقنيات المستقبلية إسلام-إنسان"
EMAIL="islamrenewaltech@gmail.com"
PHONE="+201556328989"
CONTRACT="0xa23D57f128Df2517517CA0c195C5159d81324711"
TOKEN="INSAN"

# التوقيت
BUILD_START=$(date +%s)

print_banner() {
    clear
    echo -e "${PURPLE}"
    echo "╔══════════════════════════════════════════════════════════════════╗"
    echo "║                                                                  ║"
    echo "║               ${WHITE}🏗️  بناء نظام تشغيل إسلام 🏗️${PURPLE}                    ║"
    echo "║                   ${WHITE}الإصدار التجريبي ${OS_VERSION}${PURPLE}                      ║"
    echo "║                                                                  ║"
    echo "║  ${CYAN}👨💻 المطور: ${WHITE}${DEVELOPER}${PURPLE}                             ║"
    echo "║  ${CYAN}🏢 الشركة: ${WHITE}${COMPANY}${PURPLE}       ║"
    echo "║                                                                  ║"
    echo "║  ${CYAN}📍 العنوان: ${WHITE}الحي الإفرنجي، الإسماعيلية، مصر${PURPLE}          ║"
    echo "║  ${CYAN}📞 الهاتف: ${WHITE}${PHONE}${PURPLE}                             ║"
    echo "║  ${CYAN}📧 البريد: ${WHITE}${EMAIL}${PURPLE}     ║"
    echo "║  ${CYAN}🎫 التوكن: ${WHITE}${TOKEN}${PURPLE} - ${WHITE}${CONTRACT}${PURPLE} ║"
    echo "║                                                                  ║"
    echo "╚══════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
}

print_step() {
    echo -e "${BLUE}[→]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_info() {
    echo -e "${CYAN}[i]${NC} $1"
}

check_requirements() {
    print_step "🔍 التحقق من متطلبات البناء..."
    
    local missing=()
    local tools=(
        "rustc" "cargo" "nasm" "grub-mkrescue"
        "xorriso" "qemu-system-x86_64" "mtools"
        "git" "make" "gcc" "ld"
    )
    
    for tool in "${tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            missing+=("$tool")
        fi
    done
    
    if [ ${#missing[@]} -ne 0 ]; then
        print_error "الأدوات التالية غير مثبتة:"
        for tool in "${missing[@]}"; do
            echo "  - $tool"
        done
        
        echo ""
        print_info "🔧 تثبيت المتطلبات على Ubuntu/Debian:"
        echo "  sudo apt update && sudo apt upgrade -y"
        echo "  sudo apt install -y \\"
        echo "    build-essential \\"
        echo "    nasm \\"
        echo "    grub-pc-bin \\"
        echo "    grub-efi-amd64-bin \\"
        echo "    xorriso \\"
        echo "    mtools \\"
        echo "    qemu-system-x86 \\"
        echo "    git \\"
        echo "    curl"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # التحقق من إصدار Rust
    local rust_version=$(rustc --version | awk '{print $2}')
    if [[ "$rust_version" < "1.70.0" ]]; then
        print_error "إصدار Rust قديم ($rust_version). يلزم 1.70.0+"
        echo "  تحديث Rust: rustup update"
        exit 1
    fi
    
    print_success "جميع المتطلبات مثبتة"
    print_info "   Rust: $rust_version"
    print_info "   NASM: $(nasm --version | head -n1)"
}

setup_environment() {
    print_step "⚙️ إعداد بيئة البناء..."
    
    # إنشاء مجلدات البناء
    mkdir -p build/{iso,boot,initrd,modules}
    mkdir -p target/{release,debug}
    
    # نسخ الأصول
    cp -r assets build/iso/
    cp -r config build/iso/
    
    print_success "تم إعداد البيئة"
}

build_bootloader() {
    print_step "🔨 بناء برنامج الإقلاع..."
    
    cd src/boot
    
    # تجميع ملفات Assembly
    nasm -f elf64 multiboot_header.asm -o ../build/boot/multiboot.o
    nasm -f elf64 boot.asm -o ../build/boot/boot.o
    
    # الرابط
    ld -n -T linker.ld -o ../build/boot/boot.bin \
        ../build/boot/multiboot.o \
        ../build/boot/boot.o
    
    cd ../..
    
    # التحقق من الملف
    if [ -f "build/boot/boot.bin" ]; then
        print_success "تم بناء برنامج الإقلاع"
        print_info "   الحجم: $(stat -c%s build/boot/boot.bin) بايت"
    else
        print_error "فشل بناء برنامج الإقلاع"
        exit 1
    fi
}

build_kernel() {
    print_step "🏗️ بناء نواة النظام..."
    
    cd src/kernel
    
    # بناء النواة بإعدادات الإصدار
    export RUSTFLAGS="-C link-arg=-T../boot/linker.ld"
    
    if cargo build --release --target x86_64-unknown-none &> build.log; then
        print_success "تم بناء النواة بنجاح"
        
        # نسخ النواة
        cp target/x86_64-unknown-none/release/libkernel.a \
            ../../build/iso/boot/kernel.bin
        
        print_info "   الحجم: $(stat -c%s ../../build/iso/boot/kernel.bin) بايت"
    else
        print_error "فشل بناء النواة"
        cat build.log
        exit 1
    fi
    
    cd ../..
}

build_ai_system() {
    print_step "🤖 بناء نظام الذكاء الاصطناعي..."
    
    cd src/ai
    
    if cargo build --release &> build.log; then
        print_success "تم بناء Zaka Islam"
        cp target/release/libzaka_islam_ai.a \
            ../../build/modules/zaka.bin
    else
        print_warning "فشل بناء AI (تابع البناء)"
        cat build.log
    fi
    
    cd ../..
}

build_security_system() {
    print_step "🛡️ بناء نظام الأمن..."
    
    cd src/security
    
    if cargo build --release &> build.log; then
        print_success "تم بناء Haris Islam"
        cp target/release/libharis_islam_security.a \
            ../../build/modules/haris.bin
    else
        print_warning "فشل بناء Security (تابع البناء)"
        cat build.log
    fi
    
    cd ../..
}

build_applications() {
    print_step "📦 بناء التطبيقات..."
    
    local apps=("shell" "browser" "video_player" "audio_player" "payment")
    
    for app in "${apps[@]}"; do
        print_info "  بناء $app..."
        
        cd "src/apps/$app"
        
        if cargo build --release &> build.log; then
            print_success "    تم بناء $app"
            cp target/release/*.bin \
                ../../../build/modules/${app}.bin 2>/dev/null || true
        else
            print_warning "    فشل بناء $app"
        fi
        
        cd ../../..
    done
}

create_initrd() {
    print_step "📁 إنشاء initrd..."
    
    cd build/initrd
    
    # إنشاء هيكل initrd
    mkdir -p {bin,lib,etc,dev,proc,sys,home,root,tmp,usr/{bin,lib,share}}
    
    # ملفات النظام
    cat > etc/os-release << EOF
NAME="نظام تشغيل إسلام"
VERSION="${OS_VERSION}"
ID=islam
PRETTY_NAME="نظام تشغيل إسلام ${OS_VERSION}"
DEVELOPER="${DEVELOPER}"
COMPANY="${COMPANY}"
EMAIL="${EMAIL}"
PHONE="${PHONE}"
TOKEN="${TOKEN}"
CONTRACT="${CONTRACT}"
EOF
    
    cat > etc/motd << EOF
╔══════════════════════════════════════════════════════╗
║                                                      ║
║      🕌 مرحباً بك في نظام تشغيل إسلام 🕌           ║
║                                                      ║
║  👨💻 المطور: ${DEVELOPER}             ║
║  🏢 الشركة: ${COMPANY}   ║
║                                                      ║
║  📞 للدعم: ${PHONE}                       ║
║  📧 البريد: ${EMAIL}           ║
║                                                      ║
╚══════════════════════════════════════════════════════╝
EOF
    
    # ملفات التكوين
    cp ../../config/* etc/
    
    # تجميع initrd
    find . | cpio -o -H newc | gzip > ../iso/boot/initrd.img
    
    cd ../..
    
    print_success "تم إنشاء initrd"
    print_info "   الحجم: $(stat -c%s build/iso/boot/initrd.img) بايت"
}

create_grub_config() {
    print_step "⚙️ إنشاء تكوين GRUB..."
    
    cat > build/iso/boot/grub/grub.cfg << EOF
set timeout=10
set default=0
set menu_color_normal=white/black
set menu_color_highlight=black/light-magenta

menuentry "نظام تشغيل إسلام ${OS_VERSION}" {
    echo "جاري تحميل النواة..."
    multiboot2 /boot/kernel.bin
    module2 /boot/initrd.img
    boot
}

menuentry "نظام إسلام (وضع الاسترداد)" {
    echo "جاري تحميل وضع الاسترداد..."
    multiboot2 /boot/kernel.bin recovery=1
    module2 /boot/initrd.img
    boot
}

menuentry "اختبار الذاكرة" {
    echo "جاري اختبار الذاكرة..."
    memtest86+
}

menuentry "إعادة تشغيل" {
    reboot
}

menuentry "إيقاف التشغيل" {
    halt
}
EOF
    
    print_success "تم إنشاء تكوين GRUB"
}

create_iso() {
    print_step "💿 إنشاء صورة ISO..."
    
    # إنشاء صورة ISO
    grub-mkrescue -o islam-os-${OS_VERSION}.iso build/iso/ \
        -volid "ISLAM_OS" \
        &> iso.log
    
    if [ $? -eq 0 ]; then
        ISO_SIZE=$(du -h islam-os-${OS_VERSION}.iso | cut -f1)
        print_success "تم إنشاء islam-os-${OS_VERSION}.iso"
        print_info "   الحجم: ${ISO_SIZE}"
        print_info "   MD5: $(md5sum islam-os-${OS_VERSION}.iso | cut -d' ' -f1)"
    else
        print_error "فشل إنشاء ISO"
        cat iso.log
        exit 1
    fi
}

create_usb_image() {
    print_step "📀 إنشاء صورة USB..."
    
    local usb_img="build/islam-os-usb-${OS_VERSION}.img"
    
    # إنشاء صورة قرص
    dd if=/dev/zero of="$usb_img" bs=1M count=512 status=progress
    
    # تقسيم الصورة
    parted "$usb_img" mklabel msdos
    parted "$usb_img" mkpart primary fat32 1MiB 511MiB
    parted "$usb_img" set 1 boot on
    
    # توصيل الصورة
    local loop_dev=$(sudo losetup --find --show --partscan "$usb_img")
    
    # تهيئة النظام
    sudo mkfs.fat -F32 "${loop_dev}p1"
    
    # تركيب
    local mount_dir="/tmp/islam_usb_$(date +%s)"
    mkdir -p "$mount_dir"
    sudo mount "${loop_dev}p1" "$mount_dir"
    
    # تثبيت GRUB
    sudo grub-install \
        --target=i386-pc \
        --boot-directory="$mount_dir/boot" \
        --modules="part_msdos fat multiboot2" \
        "$loop_dev"
    
    # نسخ ملفات النظام
    sudo cp islam-os-${OS_VERSION}.iso "$mount_dir/"
    sudo cp -r build/iso/* "$mount_dir/"
    
    # إنشاء ملف التمهيد
    sudo cat > "$mount_dir/boot/grub/grub.cfg" << EOF
menuentry "نظام تشغيل إسلام ${OS_VERSION} (من ISO)" {
    loopback loop /islam-os-${OS_VERSION}.iso
    root=(loop)
    configfile /boot/grub/grub.cfg
}
EOF
    
    # فك التركيب
    sudo umount "$mount_dir"
    sudo losetup -d "$loop_dev"
    rm -rf "$mount_dir"
    
    print_success "تم إنشاء صورة USB"
    print_info "   الملف: $usb_img"
    print_info "   للحرق: sudo dd if=$usb_img of=/dev/sdX bs=4M status=progress"
}

run_tests() {
    print_step "🧪 تشغيل الاختبارات..."
    
    if cargo test --workspace --quiet &> test.log; then
        print_success "جميع الاختبارات ناجحة"
    else
        print_warning "بعض الاختبارات فشلت"
        cat test.log
    fi
}

show_summary() {
    local BUILD_END=$(date +%s)
    local BUILD_TIME=$((BUILD_END - BUILD_START))
    
    echo ""
    echo -e "${GREEN}══════════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}                    🎉 بناء ناجح! 🎉                         ${NC}"
    echo -e "${GREEN}══════════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "${CYAN}📊 ملخص البناء:${NC}"
    echo -e "  🕒 وقت البناء: ${BUILD_TIME} ثانية"
    echo -e "  📦 حجم النواة: $(stat -c%s build/iso/boot/kernel.bin 2>/dev/null || echo 0) بايت"
    echo -e "  📁 حجم initrd: $(stat -c%s build/iso/boot/initrd.img 2>/dev/null || echo 0) بايت"
    echo -e "  💿 حجم ISO: $(stat -c%s islam-os-${OS_VERSION}.iso 2>/dev/null || echo 0) بايت"
    echo ""
    echo -e "${CYAN}📁 الملفات المنشأة:${NC}"
    echo -e "  💿 islam-os-${OS_VERSION}.iso"
    echo -e "  📀 build/islam-os-usb-${OS_VERSION}.img"
    echo -e "  🏗️ build/ - ملفات البناء"
    echo ""
    echo -e "${CYAN}🚀 أوامر التشغيل:${NC}"
    echo -e "  ./scripts/run.sh              # تشغيل على QEMU"
    echo -e "  ./scripts/run.sh --debug      # تشغيل مع التصحيح"
    echo -e "  sudo ./scripts/install_usb.sh # حرق على USB"
    echo ""
    echo -e "${CYAN}📞 معلومات الدعم:${NC}"
    echo -e "  👨💻 المطور: ${DEVELOPER}"
    echo -e "  📞 الهاتف: ${PHONE}"
    echo -e "  📧 البريد: ${EMAIL}"
    echo -e "  🎫 التوكن: ${TOKEN}"
    echo -e "  🔗 العقد: ${CONTRACT}"
    echo ""
    echo -e "${GREEN}══════════════════════════════════════════════════════════════════${NC}"
    echo ""
}

main() {
    print_banner
    
    print_step "🚀 بدء بناء نظام تشغيل إسلام..."
    print_info "   التاريخ: $(date)"
    print_info "   الإصدار: ${OS_VERSION}"
    
    # التحقق من المتطلبات
    check_requirements
    
    # إعداد البيئة
    setup_environment
    
    # بناء المكونات
    build_bootloader
    build_kernel
    build_ai_system
    build_security_system
    build_applications
    
    # إنشاء initrd
    create_initrd
    
    # تكوين GRUB
    create_grub_config
    
    # إنشاء ISO
    create_iso
    
    # إنشاء صورة USB (اختياري)
    echo ""
    read -p "هل تريد إنشاء صورة USB أيضا؟ (y/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        create_usb_image
    fi
    
    # الاختبارات
    run_tests
    
    # الملخص
    show_summary
}

# التنفيذ الرئيسي
main "$@"