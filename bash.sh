islam-os/
├── Cargo.toml
├── Cargo.lock
├── Makefile
├── .gitignore
├── .cargo/
│   └── config.toml
├── src/
│   ├── boot/
│   │   ├── multiboot_header.asm
│   │   ├── boot.asm
│   │   └── linker.ld
│   ├── kernel/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── arch/
│   │       │   ├── x86_64/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── gdt.asm
│   │       │   │   ├── idt.asm
│   │       │   │   ├── interrupts.asm
│   │       │   │   └── cpu.rs
│   │       │   └── arm64/
│   │       │       └── mod.rs
│   │       ├── memory/
│   │       │   ├── mod.rs
│   │       │   ├── allocator.rs
│   │       │   ├── paging.rs
│   │       │   └── heap.rs
│   │       ├── drivers/
│   │       │   ├── mod.rs
│   │       │   ├── vga.rs
│   │       │   ├── keyboard.rs
│   │       │   ├── serial.rs
│   │       │   ├── ata.rs
│   │       │   ├── pci.rs
│   │       │   └── timer.rs
│   │       ├── process/
│   │       │   ├── mod.rs
│   │       │   ├── scheduler.rs
│   │       │   ├── task.rs
│   │       │   └── syscall.rs
│   │       ├── fs/
│   │       │   ├── mod.rs
│   │       │   ├── vfs.rs
│   │       │   ├── fat32.rs
│   │       │   └── ext2.rs
│   │       ├── net/
│   │       │   ├── mod.rs
│   │       │   ├── ethernet.rs
│   │       │   ├── tcpip.rs
│   │       │   └── nic.rs
│   │       └── gui/
│   │           ├── mod.rs
│   │           ├── window.rs
│   │           ├── widget.rs
│   │           └── renderer.rs
│   ├── ai/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── zaka_core.rs
│   │       ├── nlp_processor.rs
│   │       ├── knowledge_base.rs
│   │       └── learning.rs
│   ├── security/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── haris_core.rs
│   │       ├── firewall.rs
│   │       ├── encryption.rs
│   │       ├── ids.rs
│   │       └── antivirus.rs
│   ├── apps/
│   │   ├── shell/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       ├── parser.rs
│   │   │       └── commands.rs
│   │   ├── file_manager/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── main.rs
│   │   ├── browser/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       ├── renderer.rs
│   │   │       └── network.rs
│   │   ├── video_player/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── main.rs
│   │   ├── audio_player/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── main.rs
│   │   └── payment/
│   │       ├── Cargo.toml
│   │       └── src/
│   │           └── main.rs
│   └── libs/
│       ├── islam_math/
│       │   ├── Cargo.toml
│       │   └── src/
│       │       └── lib.rs
│       ├── islam_crypto/
│       │   ├── Cargo.toml
│       │   └── src/
│       │       └── lib.rs
│       └── islam_net/
│           ├── Cargo.toml
│           └── src/
│               └── lib.rs
├── assets/
│   ├── icons/
│   │   ├── islam_logo.ico
│   │   ├── islam_logo.png
│   │   ├── islam_logo.svg
│   │   ├── zaka_icon.png
│   │   ├── haris_icon.png
│   │   ├── payment_icon.png
│   │   └── browser_icon.png
│   ├── fonts/
│   │   ├── islamic.ttf
│   │   ├── arabic.ttf
│   │   └── monospace.ttf
│   ├── wallpapers/
│   │   ├── default.jpg
│   │   ├── islamic_pattern.jpg
│   │   └── gradient_purple.jpg
│   ├── themes/
│   │   ├── islamic_dark.json
│   │   ├── islamic_light.json
│   │   └── purple_dark.json
│   └── sounds/
│       ├── startup.wav
│       ├── notification.wav
│       └── error.wav
├── config/
│   ├── system.toml
│   ├── network.toml
│   ├── security.toml
│   ├── gui.toml
│   └── users.toml
├── scripts/
│   ├── build.sh
│   ├── run.sh
│   ├── test.sh
│   ├── deploy.sh
│   ├── create_iso.sh
│   └── install_usb.sh
├── tests/
│   ├── unit/
│   │   ├── kernel_tests.rs
│   │   ├── memory_tests.rs
│   │   └── driver_tests.rs
│   ├── integration/
│   │   ├── system_tests.rs
│   │   └── network_tests.rs
│   └── e2e/
│       └── full_system.rs
├── docs/
│   ├── README_AR.md
│   ├── README_EN.md
│   ├── DEVELOPER_GUIDE.md
│   ├── USER_GUIDE.md
│   ├── API_DOCS.md
│   ├── BRAND_GUIDE.md
│   └── CONTRIBUTING.md
├── tools/
│   ├── cross_compile.sh
│   ├── image_builder.sh
│   └── package_manager.sh
├── docker/
│   ├── Dockerfile
│   ├── docker-compose.yml
│   └── build_env.sh
└── contracts/
    ├── insan_token.sol
    ├── islam_payment.sol
    └── security_contract.sol