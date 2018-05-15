#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Identification 0"]
    pub did0: DID0,
    #[doc = "0x04 - Device Identification 1"]
    pub did1: DID1,
    #[doc = "0x08 - Device Capabilities 0"]
    pub dc0: DC0,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Device Capabilities 1"]
    pub dc1: DC1,
    #[doc = "0x14 - Device Capabilities 2"]
    pub dc2: DC2,
    #[doc = "0x18 - Device Capabilities 3"]
    pub dc3: DC3,
    #[doc = "0x1c - Device Capabilities 4"]
    pub dc4: DC4,
    #[doc = "0x20 - Device Capabilities 5"]
    pub dc5: DC5,
    #[doc = "0x24 - Device Capabilities 6"]
    pub dc6: DC6,
    #[doc = "0x28 - Device Capabilities 7"]
    pub dc7: DC7,
    #[doc = "0x2c - Device Capabilities 8"]
    pub dc8: DC8,
    #[doc = "0x30 - Brown-Out Reset Control"]
    pub pborctl: PBORCTL,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - Software Reset Control 0"]
    pub srcr0: SRCR0,
    #[doc = "0x44 - Software Reset Control 1"]
    pub srcr1: SRCR1,
    #[doc = "0x48 - Software Reset Control 2"]
    pub srcr2: SRCR2,
    _reserved2: [u8; 4usize],
    #[doc = "0x50 - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x54 - Interrupt Mask Control"]
    pub imc: IMC,
    #[doc = "0x58 - Masked Interrupt Status and Clear"]
    pub misc: MISC,
    #[doc = "0x5c - Reset Cause"]
    pub resc: RESC,
    #[doc = "0x60 - Run-Mode Clock Configuration"]
    pub rcc: RCC,
    _reserved3: [u8; 8usize],
    #[doc = "0x6c - GPIO High-Performance Bus Control"]
    pub gpiohbctl: GPIOHBCTL,
    #[doc = "0x70 - Run-Mode Clock Configuration 2"]
    pub rcc2: RCC2,
    _reserved4: [u8; 8usize],
    #[doc = "0x7c - Main Oscillator Control"]
    pub moscctl: MOSCCTL,
    _reserved5: [u8; 128usize],
    #[doc = "0x100 - Run Mode Clock Gating Control Register 0"]
    pub rcgc0: RCGC0,
    #[doc = "0x104 - Run Mode Clock Gating Control Register 1"]
    pub rcgc1: RCGC1,
    #[doc = "0x108 - Run Mode Clock Gating Control Register 2"]
    pub rcgc2: RCGC2,
    _reserved6: [u8; 4usize],
    #[doc = "0x110 - Sleep Mode Clock Gating Control Register 0"]
    pub scgc0: SCGC0,
    #[doc = "0x114 - Sleep Mode Clock Gating Control Register 1"]
    pub scgc1: SCGC1,
    #[doc = "0x118 - Sleep Mode Clock Gating Control Register 2"]
    pub scgc2: SCGC2,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - Deep Sleep Mode Clock Gating Control Register 0"]
    pub dcgc0: DCGC0,
    #[doc = "0x124 - Deep-Sleep Mode Clock Gating Control Register 1"]
    pub dcgc1: DCGC1,
    #[doc = "0x128 - Deep Sleep Mode Clock Gating Control Register 2"]
    pub dcgc2: DCGC2,
    _reserved8: [u8; 24usize],
    #[doc = "0x144 - Deep Sleep Clock Configuration"]
    pub dslpclkcfg: DSLPCLKCFG,
    _reserved9: [u8; 4usize],
    #[doc = "0x14c - System Properties"]
    pub sysprop: SYSPROP,
    #[doc = "0x150 - Precision Internal Oscillator Calibration"]
    pub piosccal: PIOSCCAL,
    #[doc = "0x154 - Precision Internal Oscillator Statistics"]
    pub pioscstat: PIOSCSTAT,
    _reserved10: [u8; 8usize],
    #[doc = "0x160 - PLL Frequency 0"]
    pub pllfreq0: PLLFREQ0,
    #[doc = "0x164 - PLL Frequency 1"]
    pub pllfreq1: PLLFREQ1,
    #[doc = "0x168 - PLL Status"]
    pub pllstat: PLLSTAT,
    _reserved11: [u8; 28usize],
    #[doc = "0x188 - Sleep Power Configuration"]
    pub slppwrcfg: SLPPWRCFG,
    #[doc = "0x18c - Deep-Sleep Power Configuration"]
    pub dslppwrcfg: DSLPPWRCFG,
    #[doc = "0x190 - Device Capabilities 9"]
    pub dc9: DC9,
    _reserved12: [u8; 12usize],
    #[doc = "0x1a0 - Non-Volatile Memory Information"]
    pub nvmstat: NVMSTAT,
    _reserved13: [u8; 16usize],
    #[doc = "0x1b4 - LDO Sleep Power Control"]
    pub ldospctl: LDOSPCTL,
    _reserved14: [u8; 4usize],
    #[doc = "0x1bc - LDO Deep-Sleep Power Control"]
    pub ldodpctl: LDODPCTL,
    _reserved15: [u8; 320usize],
    #[doc = "0x300 - Watchdog Timer Peripheral Present"]
    pub ppwd: PPWD,
    #[doc = "0x304 - 16/32-Bit General-Purpose Timer Peripheral Present"]
    pub pptimer: PPTIMER,
    #[doc = "0x308 - General-Purpose Input/Output Peripheral Present"]
    pub ppgpio: PPGPIO,
    #[doc = "0x30c - Micro Direct Memory Access Peripheral Present"]
    pub ppdma: PPDMA,
    _reserved16: [u8; 4usize],
    #[doc = "0x314 - Hibernation Peripheral Present"]
    pub pphib: PPHIB,
    #[doc = "0x318 - Universal Asynchronous Receiver/Transmitter Peripheral Present"]
    pub ppuart: PPUART,
    #[doc = "0x31c - Synchronous Serial Interface Peripheral Present"]
    pub ppssi: PPSSI,
    #[doc = "0x320 - Inter-Integrated Circuit Peripheral Present"]
    pub ppi2c: PPI2C,
    _reserved17: [u8; 4usize],
    #[doc = "0x328 - Universal Serial Bus Peripheral Present"]
    pub ppusb: PPUSB,
    _reserved18: [u8; 8usize],
    #[doc = "0x334 - Controller Area Network Peripheral Present"]
    pub ppcan: PPCAN,
    #[doc = "0x338 - Analog-to-Digital Converter Peripheral Present"]
    pub ppadc: PPADC,
    #[doc = "0x33c - Analog Comparator Peripheral Present"]
    pub ppacmp: PPACMP,
    #[doc = "0x340 - Pulse Width Modulator Peripheral Present"]
    pub pppwm: PPPWM,
    #[doc = "0x344 - Quadrature Encoder Interface Peripheral Present"]
    pub ppqei: PPQEI,
    _reserved19: [u8; 16usize],
    #[doc = "0x358 - EEPROM Peripheral Present"]
    pub ppeeprom: PPEEPROM,
    #[doc = "0x35c - 32/64-Bit Wide General-Purpose Timer Peripheral Present"]
    pub ppwtimer: PPWTIMER,
    _reserved20: [u8; 416usize],
    #[doc = "0x500 - Watchdog Timer Software Reset"]
    pub srwd: SRWD,
    #[doc = "0x504 - 16/32-Bit General-Purpose Timer Software Reset"]
    pub srtimer: SRTIMER,
    #[doc = "0x508 - General-Purpose Input/Output Software Reset"]
    pub srgpio: SRGPIO,
    #[doc = "0x50c - Micro Direct Memory Access Software Reset"]
    pub srdma: SRDMA,
    _reserved21: [u8; 4usize],
    #[doc = "0x514 - Hibernation Software Reset"]
    pub srhib: SRHIB,
    #[doc = "0x518 - Universal Asynchronous Receiver/Transmitter Software Reset"]
    pub sruart: SRUART,
    #[doc = "0x51c - Synchronous Serial Interface Software Reset"]
    pub srssi: SRSSI,
    #[doc = "0x520 - Inter-Integrated Circuit Software Reset"]
    pub sri2c: SRI2C,
    _reserved22: [u8; 4usize],
    #[doc = "0x528 - Universal Serial Bus Software Reset"]
    pub srusb: SRUSB,
    _reserved23: [u8; 8usize],
    #[doc = "0x534 - Controller Area Network Software Reset"]
    pub srcan: SRCAN,
    #[doc = "0x538 - Analog-to-Digital Converter Software Reset"]
    pub sradc: SRADC,
    #[doc = "0x53c - Analog Comparator Software Reset"]
    pub sracmp: SRACMP,
    #[doc = "0x540 - Pulse Width Modulator Software Reset"]
    pub srpwm: SRPWM,
    #[doc = "0x544 - Quadrature Encoder Interface Software Reset"]
    pub srqei: SRQEI,
    _reserved24: [u8; 16usize],
    #[doc = "0x558 - EEPROM Software Reset"]
    pub sreeprom: SREEPROM,
    #[doc = "0x55c - 32/64-Bit Wide General-Purpose Timer Software Reset"]
    pub srwtimer: SRWTIMER,
    _reserved25: [u8; 160usize],
    #[doc = "0x600 - Watchdog Timer Run Mode Clock Gating Control"]
    pub rcgcwd: RCGCWD,
    #[doc = "0x604 - 16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
    pub rcgctimer: RCGCTIMER,
    #[doc = "0x608 - General-Purpose Input/Output Run Mode Clock Gating Control"]
    pub rcgcgpio: RCGCGPIO,
    #[doc = "0x60c - Micro Direct Memory Access Run Mode Clock Gating Control"]
    pub rcgcdma: RCGCDMA,
    _reserved26: [u8; 4usize],
    #[doc = "0x614 - Hibernation Run Mode Clock Gating Control"]
    pub rcgchib: RCGCHIB,
    #[doc = "0x618 - Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
    pub rcgcuart: RCGCUART,
    #[doc = "0x61c - Synchronous Serial Interface Run Mode Clock Gating Control"]
    pub rcgcssi: RCGCSSI,
    #[doc = "0x620 - Inter-Integrated Circuit Run Mode Clock Gating Control"]
    pub rcgci2c: RCGCI2C,
    _reserved27: [u8; 4usize],
    #[doc = "0x628 - Universal Serial Bus Run Mode Clock Gating Control"]
    pub rcgcusb: RCGCUSB,
    _reserved28: [u8; 8usize],
    #[doc = "0x634 - Controller Area Network Run Mode Clock Gating Control"]
    pub rcgccan: RCGCCAN,
    #[doc = "0x638 - Analog-to-Digital Converter Run Mode Clock Gating Control"]
    pub rcgcadc: RCGCADC,
    #[doc = "0x63c - Analog Comparator Run Mode Clock Gating Control"]
    pub rcgcacmp: RCGCACMP,
    #[doc = "0x640 - Pulse Width Modulator Run Mode Clock Gating Control"]
    pub rcgcpwm: RCGCPWM,
    #[doc = "0x644 - Quadrature Encoder Interface Run Mode Clock Gating Control"]
    pub rcgcqei: RCGCQEI,
    _reserved29: [u8; 16usize],
    #[doc = "0x658 - EEPROM Run Mode Clock Gating Control"]
    pub rcgceeprom: RCGCEEPROM,
    #[doc = "0x65c - 32/64-Bit Wide General-Purpose Timer Run Mode Clock Gating Control"]
    pub rcgcwtimer: RCGCWTIMER,
    _reserved30: [u8; 160usize],
    #[doc = "0x700 - Watchdog Timer Sleep Mode Clock Gating Control"]
    pub scgcwd: SCGCWD,
    #[doc = "0x704 - 16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
    pub scgctimer: SCGCTIMER,
    #[doc = "0x708 - General-Purpose Input/Output Sleep Mode Clock Gating Control"]
    pub scgcgpio: SCGCGPIO,
    #[doc = "0x70c - Micro Direct Memory Access Sleep Mode Clock Gating Control"]
    pub scgcdma: SCGCDMA,
    _reserved31: [u8; 4usize],
    #[doc = "0x714 - Hibernation Sleep Mode Clock Gating Control"]
    pub scgchib: SCGCHIB,
    #[doc = "0x718 - Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
    pub scgcuart: SCGCUART,
    #[doc = "0x71c - Synchronous Serial Interface Sleep Mode Clock Gating Control"]
    pub scgcssi: SCGCSSI,
    #[doc = "0x720 - Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
    pub scgci2c: SCGCI2C,
    _reserved32: [u8; 4usize],
    #[doc = "0x728 - Universal Serial Bus Sleep Mode Clock Gating Control"]
    pub scgcusb: SCGCUSB,
    _reserved33: [u8; 8usize],
    #[doc = "0x734 - Controller Area Network Sleep Mode Clock Gating Control"]
    pub scgccan: SCGCCAN,
    #[doc = "0x738 - Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
    pub scgcadc: SCGCADC,
    #[doc = "0x73c - Analog Comparator Sleep Mode Clock Gating Control"]
    pub scgcacmp: SCGCACMP,
    #[doc = "0x740 - Pulse Width Modulator Sleep Mode Clock Gating Control"]
    pub scgcpwm: SCGCPWM,
    #[doc = "0x744 - Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
    pub scgcqei: SCGCQEI,
    _reserved34: [u8; 16usize],
    #[doc = "0x758 - EEPROM Sleep Mode Clock Gating Control"]
    pub scgceeprom: SCGCEEPROM,
    #[doc = "0x75c - 32/64-Bit Wide General-Purpose Timer Sleep Mode Clock Gating Control"]
    pub scgcwtimer: SCGCWTIMER,
    _reserved35: [u8; 160usize],
    #[doc = "0x800 - Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgcwd: DCGCWD,
    #[doc = "0x804 - 16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgctimer: DCGCTIMER,
    #[doc = "0x808 - General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
    pub dcgcgpio: DCGCGPIO,
    #[doc = "0x80c - Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
    pub dcgcdma: DCGCDMA,
    _reserved36: [u8; 4usize],
    #[doc = "0x814 - Hibernation Deep-Sleep Mode Clock Gating Control"]
    pub dcgchib: DCGCHIB,
    #[doc = "0x818 - Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
    pub dcgcuart: DCGCUART,
    #[doc = "0x81c - Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
    pub dcgcssi: DCGCSSI,
    #[doc = "0x820 - Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
    pub dcgci2c: DCGCI2C,
    _reserved37: [u8; 4usize],
    #[doc = "0x828 - Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
    pub dcgcusb: DCGCUSB,
    _reserved38: [u8; 8usize],
    #[doc = "0x834 - Controller Area Network Deep-Sleep Mode Clock Gating Control"]
    pub dcgccan: DCGCCAN,
    #[doc = "0x838 - Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
    pub dcgcadc: DCGCADC,
    #[doc = "0x83c - Analog Comparator Deep-Sleep Mode Clock Gating Control"]
    pub dcgcacmp: DCGCACMP,
    #[doc = "0x840 - Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
    pub dcgcpwm: DCGCPWM,
    #[doc = "0x844 - Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
    pub dcgcqei: DCGCQEI,
    _reserved39: [u8; 16usize],
    #[doc = "0x858 - EEPROM Deep-Sleep Mode Clock Gating Control"]
    pub dcgceeprom: DCGCEEPROM,
    #[doc = "0x85c - 32/64-Bit Wide General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgcwtimer: DCGCWTIMER,
    _reserved40: [u8; 416usize],
    #[doc = "0xa00 - Watchdog Timer Peripheral Ready"]
    pub prwd: PRWD,
    #[doc = "0xa04 - 16/32-Bit General-Purpose Timer Peripheral Ready"]
    pub prtimer: PRTIMER,
    #[doc = "0xa08 - General-Purpose Input/Output Peripheral Ready"]
    pub prgpio: PRGPIO,
    #[doc = "0xa0c - Micro Direct Memory Access Peripheral Ready"]
    pub prdma: PRDMA,
    _reserved41: [u8; 4usize],
    #[doc = "0xa14 - Hibernation Peripheral Ready"]
    pub prhib: PRHIB,
    #[doc = "0xa18 - Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
    pub pruart: PRUART,
    #[doc = "0xa1c - Synchronous Serial Interface Peripheral Ready"]
    pub prssi: PRSSI,
    #[doc = "0xa20 - Inter-Integrated Circuit Peripheral Ready"]
    pub pri2c: PRI2C,
    _reserved42: [u8; 4usize],
    #[doc = "0xa28 - Universal Serial Bus Peripheral Ready"]
    pub prusb: PRUSB,
    _reserved43: [u8; 8usize],
    #[doc = "0xa34 - Controller Area Network Peripheral Ready"]
    pub prcan: PRCAN,
    #[doc = "0xa38 - Analog-to-Digital Converter Peripheral Ready"]
    pub pradc: PRADC,
    #[doc = "0xa3c - Analog Comparator Peripheral Ready"]
    pub pracmp: PRACMP,
    #[doc = "0xa40 - Pulse Width Modulator Peripheral Ready"]
    pub prpwm: PRPWM,
    #[doc = "0xa44 - Quadrature Encoder Interface Peripheral Ready"]
    pub prqei: PRQEI,
    _reserved44: [u8; 16usize],
    #[doc = "0xa58 - EEPROM Peripheral Ready"]
    pub preeprom: PREEPROM,
    #[doc = "0xa5c - 32/64-Bit Wide General-Purpose Timer Peripheral Ready"]
    pub prwtimer: PRWTIMER,
}
#[doc = "Device Identification 0"]
pub struct DID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Identification 0"]
pub mod did0;
#[doc = "Device Identification 1"]
pub struct DID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Identification 1"]
pub mod did1;
#[doc = "Device Capabilities 0"]
pub struct DC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 0"]
pub mod dc0;
#[doc = "Device Capabilities 1"]
pub struct DC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 1"]
pub mod dc1;
#[doc = "Device Capabilities 2"]
pub struct DC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 2"]
pub mod dc2;
#[doc = "Device Capabilities 3"]
pub struct DC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 3"]
pub mod dc3;
#[doc = "Device Capabilities 4"]
pub struct DC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 4"]
pub mod dc4;
#[doc = "Device Capabilities 5"]
pub struct DC5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 5"]
pub mod dc5;
#[doc = "Device Capabilities 6"]
pub struct DC6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 6"]
pub mod dc6;
#[doc = "Device Capabilities 7"]
pub struct DC7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 7"]
pub mod dc7;
#[doc = "Device Capabilities 8"]
pub struct DC8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 8"]
pub mod dc8;
#[doc = "Brown-Out Reset Control"]
pub struct PBORCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Brown-Out Reset Control"]
pub mod pborctl;
#[doc = "Software Reset Control 0"]
pub struct SRCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Reset Control 0"]
pub mod srcr0;
#[doc = "Software Reset Control 1"]
pub struct SRCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Reset Control 1"]
pub mod srcr1;
#[doc = "Software Reset Control 2"]
pub struct SRCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Reset Control 2"]
pub mod srcr2;
#[doc = "Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "Interrupt Mask Control"]
pub struct IMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Control"]
pub mod imc;
#[doc = "Masked Interrupt Status and Clear"]
pub struct MISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked Interrupt Status and Clear"]
pub mod misc;
#[doc = "Reset Cause"]
pub struct RESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Cause"]
pub mod resc;
#[doc = "Run-Mode Clock Configuration"]
pub struct RCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run-Mode Clock Configuration"]
pub mod rcc;
#[doc = "GPIO High-Performance Bus Control"]
pub struct GPIOHBCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO High-Performance Bus Control"]
pub mod gpiohbctl;
#[doc = "Run-Mode Clock Configuration 2"]
pub struct RCC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run-Mode Clock Configuration 2"]
pub mod rcc2;
#[doc = "Main Oscillator Control"]
pub struct MOSCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main Oscillator Control"]
pub mod moscctl;
#[doc = "Run Mode Clock Gating Control Register 0"]
pub struct RCGC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run Mode Clock Gating Control Register 0"]
pub mod rcgc0;
#[doc = "Run Mode Clock Gating Control Register 1"]
pub struct RCGC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run Mode Clock Gating Control Register 1"]
pub mod rcgc1;
#[doc = "Run Mode Clock Gating Control Register 2"]
pub struct RCGC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run Mode Clock Gating Control Register 2"]
pub mod rcgc2;
#[doc = "Sleep Mode Clock Gating Control Register 0"]
pub struct SCGC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Mode Clock Gating Control Register 0"]
pub mod scgc0;
#[doc = "Sleep Mode Clock Gating Control Register 1"]
pub struct SCGC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Mode Clock Gating Control Register 1"]
pub mod scgc1;
#[doc = "Sleep Mode Clock Gating Control Register 2"]
pub struct SCGC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Mode Clock Gating Control Register 2"]
pub mod scgc2;
#[doc = "Deep Sleep Mode Clock Gating Control Register 0"]
pub struct DCGC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep Sleep Mode Clock Gating Control Register 0"]
pub mod dcgc0;
#[doc = "Deep-Sleep Mode Clock Gating Control Register 1"]
pub struct DCGC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep-Sleep Mode Clock Gating Control Register 1"]
pub mod dcgc1;
#[doc = "Deep Sleep Mode Clock Gating Control Register 2"]
pub struct DCGC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep Sleep Mode Clock Gating Control Register 2"]
pub mod dcgc2;
#[doc = "Deep Sleep Clock Configuration"]
pub struct DSLPCLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep Sleep Clock Configuration"]
pub mod dslpclkcfg;
#[doc = "System Properties"]
pub struct SYSPROP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Properties"]
pub mod sysprop;
#[doc = "Precision Internal Oscillator Calibration"]
pub struct PIOSCCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Precision Internal Oscillator Calibration"]
pub mod piosccal;
#[doc = "Precision Internal Oscillator Statistics"]
pub struct PIOSCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Precision Internal Oscillator Statistics"]
pub mod pioscstat;
#[doc = "PLL Frequency 0"]
pub struct PLLFREQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Frequency 0"]
pub mod pllfreq0;
#[doc = "PLL Frequency 1"]
pub struct PLLFREQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Frequency 1"]
pub mod pllfreq1;
#[doc = "PLL Status"]
pub struct PLLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Status"]
pub mod pllstat;
#[doc = "Sleep Power Configuration"]
pub struct SLPPWRCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Power Configuration"]
pub mod slppwrcfg;
#[doc = "Deep-Sleep Power Configuration"]
pub struct DSLPPWRCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep-Sleep Power Configuration"]
pub mod dslppwrcfg;
#[doc = "Device Capabilities 9"]
pub struct DC9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Capabilities 9"]
pub mod dc9;
#[doc = "Non-Volatile Memory Information"]
pub struct NVMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-Volatile Memory Information"]
pub mod nvmstat;
#[doc = "LDO Sleep Power Control"]
pub struct LDOSPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LDO Sleep Power Control"]
pub mod ldospctl;
#[doc = "LDO Deep-Sleep Power Control"]
pub struct LDODPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LDO Deep-Sleep Power Control"]
pub mod ldodpctl;
#[doc = "Watchdog Timer Peripheral Present"]
pub struct PPWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timer Peripheral Present"]
pub mod ppwd;
#[doc = "16/32-Bit General-Purpose Timer Peripheral Present"]
pub struct PPTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "16/32-Bit General-Purpose Timer Peripheral Present"]
pub mod pptimer;
#[doc = "General-Purpose Input/Output Peripheral Present"]
pub struct PPGPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General-Purpose Input/Output Peripheral Present"]
pub mod ppgpio;
#[doc = "Micro Direct Memory Access Peripheral Present"]
pub struct PPDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Micro Direct Memory Access Peripheral Present"]
pub mod ppdma;
#[doc = "Hibernation Peripheral Present"]
pub struct PPHIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Peripheral Present"]
pub mod pphib;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present"]
pub struct PPUART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present"]
pub mod ppuart;
#[doc = "Synchronous Serial Interface Peripheral Present"]
pub struct PPSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronous Serial Interface Peripheral Present"]
pub mod ppssi;
#[doc = "Inter-Integrated Circuit Peripheral Present"]
pub struct PPI2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter-Integrated Circuit Peripheral Present"]
pub mod ppi2c;
#[doc = "Universal Serial Bus Peripheral Present"]
pub struct PPUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Serial Bus Peripheral Present"]
pub mod ppusb;
#[doc = "Controller Area Network Peripheral Present"]
pub struct PPCAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controller Area Network Peripheral Present"]
pub mod ppcan;
#[doc = "Analog-to-Digital Converter Peripheral Present"]
pub struct PPADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog-to-Digital Converter Peripheral Present"]
pub mod ppadc;
#[doc = "Analog Comparator Peripheral Present"]
pub struct PPACMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Peripheral Present"]
pub mod ppacmp;
#[doc = "Pulse Width Modulator Peripheral Present"]
pub struct PPPWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Width Modulator Peripheral Present"]
pub mod pppwm;
#[doc = "Quadrature Encoder Interface Peripheral Present"]
pub struct PPQEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Encoder Interface Peripheral Present"]
pub mod ppqei;
#[doc = "EEPROM Peripheral Present"]
pub struct PPEEPROM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Peripheral Present"]
pub mod ppeeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Present"]
pub struct PPWTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Present"]
pub mod ppwtimer;
#[doc = "Watchdog Timer Software Reset"]
pub struct SRWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timer Software Reset"]
pub mod srwd;
#[doc = "16/32-Bit General-Purpose Timer Software Reset"]
pub struct SRTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "16/32-Bit General-Purpose Timer Software Reset"]
pub mod srtimer;
#[doc = "General-Purpose Input/Output Software Reset"]
pub struct SRGPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General-Purpose Input/Output Software Reset"]
pub mod srgpio;
#[doc = "Micro Direct Memory Access Software Reset"]
pub struct SRDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Micro Direct Memory Access Software Reset"]
pub mod srdma;
#[doc = "Hibernation Software Reset"]
pub struct SRHIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Software Reset"]
pub mod srhib;
#[doc = "Universal Asynchronous Receiver/Transmitter Software Reset"]
pub struct SRUART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Asynchronous Receiver/Transmitter Software Reset"]
pub mod sruart;
#[doc = "Synchronous Serial Interface Software Reset"]
pub struct SRSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronous Serial Interface Software Reset"]
pub mod srssi;
#[doc = "Inter-Integrated Circuit Software Reset"]
pub struct SRI2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter-Integrated Circuit Software Reset"]
pub mod sri2c;
#[doc = "Universal Serial Bus Software Reset"]
pub struct SRUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Serial Bus Software Reset"]
pub mod srusb;
#[doc = "Controller Area Network Software Reset"]
pub struct SRCAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controller Area Network Software Reset"]
pub mod srcan;
#[doc = "Analog-to-Digital Converter Software Reset"]
pub struct SRADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog-to-Digital Converter Software Reset"]
pub mod sradc;
#[doc = "Analog Comparator Software Reset"]
pub struct SRACMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Software Reset"]
pub mod sracmp;
#[doc = "Pulse Width Modulator Software Reset"]
pub struct SRPWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Width Modulator Software Reset"]
pub mod srpwm;
#[doc = "Quadrature Encoder Interface Software Reset"]
pub struct SRQEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Encoder Interface Software Reset"]
pub mod srqei;
#[doc = "EEPROM Software Reset"]
pub struct SREEPROM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Software Reset"]
pub mod sreeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Software Reset"]
pub struct SRWTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32/64-Bit Wide General-Purpose Timer Software Reset"]
pub mod srwtimer;
#[doc = "Watchdog Timer Run Mode Clock Gating Control"]
pub struct RCGCWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timer Run Mode Clock Gating Control"]
pub mod rcgcwd;
#[doc = "16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
pub struct RCGCTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
pub mod rcgctimer;
#[doc = "General-Purpose Input/Output Run Mode Clock Gating Control"]
pub struct RCGCGPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General-Purpose Input/Output Run Mode Clock Gating Control"]
pub mod rcgcgpio;
#[doc = "Micro Direct Memory Access Run Mode Clock Gating Control"]
pub struct RCGCDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Micro Direct Memory Access Run Mode Clock Gating Control"]
pub mod rcgcdma;
#[doc = "Hibernation Run Mode Clock Gating Control"]
pub struct RCGCHIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Run Mode Clock Gating Control"]
pub mod rcgchib;
#[doc = "Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
pub struct RCGCUART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
pub mod rcgcuart;
#[doc = "Synchronous Serial Interface Run Mode Clock Gating Control"]
pub struct RCGCSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronous Serial Interface Run Mode Clock Gating Control"]
pub mod rcgcssi;
#[doc = "Inter-Integrated Circuit Run Mode Clock Gating Control"]
pub struct RCGCI2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter-Integrated Circuit Run Mode Clock Gating Control"]
pub mod rcgci2c;
#[doc = "Universal Serial Bus Run Mode Clock Gating Control"]
pub struct RCGCUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Serial Bus Run Mode Clock Gating Control"]
pub mod rcgcusb;
#[doc = "Controller Area Network Run Mode Clock Gating Control"]
pub struct RCGCCAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controller Area Network Run Mode Clock Gating Control"]
pub mod rcgccan;
#[doc = "Analog-to-Digital Converter Run Mode Clock Gating Control"]
pub struct RCGCADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog-to-Digital Converter Run Mode Clock Gating Control"]
pub mod rcgcadc;
#[doc = "Analog Comparator Run Mode Clock Gating Control"]
pub struct RCGCACMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Run Mode Clock Gating Control"]
pub mod rcgcacmp;
#[doc = "Pulse Width Modulator Run Mode Clock Gating Control"]
pub struct RCGCPWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Width Modulator Run Mode Clock Gating Control"]
pub mod rcgcpwm;
#[doc = "Quadrature Encoder Interface Run Mode Clock Gating Control"]
pub struct RCGCQEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Encoder Interface Run Mode Clock Gating Control"]
pub mod rcgcqei;
#[doc = "EEPROM Run Mode Clock Gating Control"]
pub struct RCGCEEPROM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Run Mode Clock Gating Control"]
pub mod rcgceeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Run Mode Clock Gating Control"]
pub struct RCGCWTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32/64-Bit Wide General-Purpose Timer Run Mode Clock Gating Control"]
pub mod rcgcwtimer;
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control"]
pub struct SCGCWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control"]
pub mod scgcwd;
#[doc = "16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
pub struct SCGCTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
pub mod scgctimer;
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control"]
pub struct SCGCGPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control"]
pub mod scgcgpio;
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control"]
pub struct SCGCDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control"]
pub mod scgcdma;
#[doc = "Hibernation Sleep Mode Clock Gating Control"]
pub struct SCGCHIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Sleep Mode Clock Gating Control"]
pub mod scgchib;
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
pub struct SCGCUART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
pub mod scgcuart;
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control"]
pub struct SCGCSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control"]
pub mod scgcssi;
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
pub struct SCGCI2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
pub mod scgci2c;
#[doc = "Universal Serial Bus Sleep Mode Clock Gating Control"]
pub struct SCGCUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Serial Bus Sleep Mode Clock Gating Control"]
pub mod scgcusb;
#[doc = "Controller Area Network Sleep Mode Clock Gating Control"]
pub struct SCGCCAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controller Area Network Sleep Mode Clock Gating Control"]
pub mod scgccan;
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
pub struct SCGCADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
pub mod scgcadc;
#[doc = "Analog Comparator Sleep Mode Clock Gating Control"]
pub struct SCGCACMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Sleep Mode Clock Gating Control"]
pub mod scgcacmp;
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control"]
pub struct SCGCPWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control"]
pub mod scgcpwm;
#[doc = "Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
pub struct SCGCQEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
pub mod scgcqei;
#[doc = "EEPROM Sleep Mode Clock Gating Control"]
pub struct SCGCEEPROM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Sleep Mode Clock Gating Control"]
pub mod scgceeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Sleep Mode Clock Gating Control"]
pub struct SCGCWTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32/64-Bit Wide General-Purpose Timer Sleep Mode Clock Gating Control"]
pub mod scgcwtimer;
#[doc = "Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcwd;
#[doc = "16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgctimer;
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCGPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcgpio;
#[doc = "Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcdma;
#[doc = "Hibernation Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCHIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Deep-Sleep Mode Clock Gating Control"]
pub mod dcgchib;
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCUART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcuart;
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcssi;
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCI2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
pub mod dcgci2c;
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcusb;
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCCAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control"]
pub mod dcgccan;
#[doc = "Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcadc;
#[doc = "Analog Comparator Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCACMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcacmp;
#[doc = "Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCPWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcpwm;
#[doc = "Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCQEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcqei;
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCEEPROM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control"]
pub mod dcgceeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCWTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32/64-Bit Wide General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcwtimer;
#[doc = "Watchdog Timer Peripheral Ready"]
pub struct PRWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timer Peripheral Ready"]
pub mod prwd;
#[doc = "16/32-Bit General-Purpose Timer Peripheral Ready"]
pub struct PRTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "16/32-Bit General-Purpose Timer Peripheral Ready"]
pub mod prtimer;
#[doc = "General-Purpose Input/Output Peripheral Ready"]
pub struct PRGPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General-Purpose Input/Output Peripheral Ready"]
pub mod prgpio;
#[doc = "Micro Direct Memory Access Peripheral Ready"]
pub struct PRDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Micro Direct Memory Access Peripheral Ready"]
pub mod prdma;
#[doc = "Hibernation Peripheral Ready"]
pub struct PRHIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Peripheral Ready"]
pub mod prhib;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
pub struct PRUART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
pub mod pruart;
#[doc = "Synchronous Serial Interface Peripheral Ready"]
pub struct PRSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronous Serial Interface Peripheral Ready"]
pub mod prssi;
#[doc = "Inter-Integrated Circuit Peripheral Ready"]
pub struct PRI2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter-Integrated Circuit Peripheral Ready"]
pub mod pri2c;
#[doc = "Universal Serial Bus Peripheral Ready"]
pub struct PRUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Serial Bus Peripheral Ready"]
pub mod prusb;
#[doc = "Controller Area Network Peripheral Ready"]
pub struct PRCAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controller Area Network Peripheral Ready"]
pub mod prcan;
#[doc = "Analog-to-Digital Converter Peripheral Ready"]
pub struct PRADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog-to-Digital Converter Peripheral Ready"]
pub mod pradc;
#[doc = "Analog Comparator Peripheral Ready"]
pub struct PRACMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Peripheral Ready"]
pub mod pracmp;
#[doc = "Pulse Width Modulator Peripheral Ready"]
pub struct PRPWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Width Modulator Peripheral Ready"]
pub mod prpwm;
#[doc = "Quadrature Encoder Interface Peripheral Ready"]
pub struct PRQEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Encoder Interface Peripheral Ready"]
pub mod prqei;
#[doc = "EEPROM Peripheral Ready"]
pub struct PREEPROM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Peripheral Ready"]
pub mod preeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Ready"]
pub struct PRWTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Ready"]
pub mod prwtimer;
