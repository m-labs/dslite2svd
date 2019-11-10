#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Identification 0"]
    pub did0: DID0,
    #[doc = "0x04 - Device Identification 1"]
    pub did1: DID1,
    #[doc = "0x08 - Device Capabilities 0"]
    pub dc0: DC0,
    _reserved3: [u8; 4usize],
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
    _reserved12: [u8; 12usize],
    #[doc = "0x40 - Software Reset Control 0"]
    pub srcr0: SRCR0,
    #[doc = "0x44 - Software Reset Control 1"]
    pub srcr1: SRCR1,
    #[doc = "0x48 - Software Reset Control 2"]
    pub srcr2: SRCR2,
    _reserved15: [u8; 4usize],
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
    _reserved20: [u8; 8usize],
    #[doc = "0x6c - GPIO High-Performance Bus Control"]
    pub gpiohbctl: GPIOHBCTL,
    #[doc = "0x70 - Run-Mode Clock Configuration 2"]
    pub rcc2: RCC2,
    _reserved22: [u8; 8usize],
    #[doc = "0x7c - Main Oscillator Control"]
    pub moscctl: MOSCCTL,
    _reserved23: [u8; 128usize],
    #[doc = "0x100 - Run Mode Clock Gating Control Register 0"]
    pub rcgc0: RCGC0,
    #[doc = "0x104 - Run Mode Clock Gating Control Register 1"]
    pub rcgc1: RCGC1,
    #[doc = "0x108 - Run Mode Clock Gating Control Register 2"]
    pub rcgc2: RCGC2,
    _reserved26: [u8; 4usize],
    #[doc = "0x110 - Sleep Mode Clock Gating Control Register 0"]
    pub scgc0: SCGC0,
    #[doc = "0x114 - Sleep Mode Clock Gating Control Register 1"]
    pub scgc1: SCGC1,
    #[doc = "0x118 - Sleep Mode Clock Gating Control Register 2"]
    pub scgc2: SCGC2,
    _reserved29: [u8; 4usize],
    #[doc = "0x120 - Deep Sleep Mode Clock Gating Control Register 0"]
    pub dcgc0: DCGC0,
    #[doc = "0x124 - Deep-Sleep Mode Clock Gating Control Register 1"]
    pub dcgc1: DCGC1,
    #[doc = "0x128 - Deep Sleep Mode Clock Gating Control Register 2"]
    pub dcgc2: DCGC2,
    _reserved32: [u8; 24usize],
    #[doc = "0x144 - Deep Sleep Clock Configuration"]
    pub dslpclkcfg: DSLPCLKCFG,
    _reserved33: [u8; 4usize],
    #[doc = "0x14c - System Properties"]
    pub sysprop: SYSPROP,
    #[doc = "0x150 - Precision Internal Oscillator Calibration"]
    pub piosccal: PIOSCCAL,
    #[doc = "0x154 - Precision Internal Oscillator Statistics"]
    pub pioscstat: PIOSCSTAT,
    _reserved36: [u8; 8usize],
    #[doc = "0x160 - PLL Frequency 0"]
    pub pllfreq0: PLLFREQ0,
    #[doc = "0x164 - PLL Frequency 1"]
    pub pllfreq1: PLLFREQ1,
    #[doc = "0x168 - PLL Status"]
    pub pllstat: PLLSTAT,
    _reserved39: [u8; 28usize],
    #[doc = "0x188 - Sleep Power Configuration"]
    pub slppwrcfg: SLPPWRCFG,
    #[doc = "0x18c - Deep-Sleep Power Configuration"]
    pub dslppwrcfg: DSLPPWRCFG,
    #[doc = "0x190 - Device Capabilities 9"]
    pub dc9: DC9,
    _reserved42: [u8; 12usize],
    #[doc = "0x1a0 - Non-Volatile Memory Information"]
    pub nvmstat: NVMSTAT,
    _reserved43: [u8; 16usize],
    #[doc = "0x1b4 - LDO Sleep Power Control"]
    pub ldospctl: LDOSPCTL,
    _reserved44: [u8; 4usize],
    #[doc = "0x1bc - LDO Deep-Sleep Power Control"]
    pub ldodpctl: LDODPCTL,
    _reserved45: [u8; 320usize],
    #[doc = "0x300 - Watchdog Timer Peripheral Present"]
    pub ppwd: PPWD,
    #[doc = "0x304 - 16/32-Bit General-Purpose Timer Peripheral Present"]
    pub pptimer: PPTIMER,
    #[doc = "0x308 - General-Purpose Input/Output Peripheral Present"]
    pub ppgpio: PPGPIO,
    #[doc = "0x30c - Micro Direct Memory Access Peripheral Present"]
    pub ppdma: PPDMA,
    _reserved49: [u8; 4usize],
    #[doc = "0x314 - Hibernation Peripheral Present"]
    pub pphib: PPHIB,
    #[doc = "0x318 - Universal Asynchronous Receiver/Transmitter Peripheral Present"]
    pub ppuart: PPUART,
    #[doc = "0x31c - Synchronous Serial Interface Peripheral Present"]
    pub ppssi: PPSSI,
    #[doc = "0x320 - Inter-Integrated Circuit Peripheral Present"]
    pub ppi2c: PPI2C,
    _reserved53: [u8; 4usize],
    #[doc = "0x328 - Universal Serial Bus Peripheral Present"]
    pub ppusb: PPUSB,
    _reserved54: [u8; 8usize],
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
    _reserved59: [u8; 16usize],
    #[doc = "0x358 - EEPROM Peripheral Present"]
    pub ppeeprom: PPEEPROM,
    #[doc = "0x35c - 32/64-Bit Wide General-Purpose Timer Peripheral Present"]
    pub ppwtimer: PPWTIMER,
    _reserved61: [u8; 416usize],
    #[doc = "0x500 - Watchdog Timer Software Reset"]
    pub srwd: SRWD,
    #[doc = "0x504 - 16/32-Bit General-Purpose Timer Software Reset"]
    pub srtimer: SRTIMER,
    #[doc = "0x508 - General-Purpose Input/Output Software Reset"]
    pub srgpio: SRGPIO,
    #[doc = "0x50c - Micro Direct Memory Access Software Reset"]
    pub srdma: SRDMA,
    _reserved65: [u8; 4usize],
    #[doc = "0x514 - Hibernation Software Reset"]
    pub srhib: SRHIB,
    #[doc = "0x518 - Universal Asynchronous Receiver/Transmitter Software Reset"]
    pub sruart: SRUART,
    #[doc = "0x51c - Synchronous Serial Interface Software Reset"]
    pub srssi: SRSSI,
    #[doc = "0x520 - Inter-Integrated Circuit Software Reset"]
    pub sri2c: SRI2C,
    _reserved69: [u8; 4usize],
    #[doc = "0x528 - Universal Serial Bus Software Reset"]
    pub srusb: SRUSB,
    _reserved70: [u8; 8usize],
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
    _reserved75: [u8; 16usize],
    #[doc = "0x558 - EEPROM Software Reset"]
    pub sreeprom: SREEPROM,
    #[doc = "0x55c - 32/64-Bit Wide General-Purpose Timer Software Reset"]
    pub srwtimer: SRWTIMER,
    _reserved77: [u8; 160usize],
    #[doc = "0x600 - Watchdog Timer Run Mode Clock Gating Control"]
    pub rcgcwd: RCGCWD,
    #[doc = "0x604 - 16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
    pub rcgctimer: RCGCTIMER,
    #[doc = "0x608 - General-Purpose Input/Output Run Mode Clock Gating Control"]
    pub rcgcgpio: RCGCGPIO,
    #[doc = "0x60c - Micro Direct Memory Access Run Mode Clock Gating Control"]
    pub rcgcdma: RCGCDMA,
    _reserved81: [u8; 4usize],
    #[doc = "0x614 - Hibernation Run Mode Clock Gating Control"]
    pub rcgchib: RCGCHIB,
    #[doc = "0x618 - Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
    pub rcgcuart: RCGCUART,
    #[doc = "0x61c - Synchronous Serial Interface Run Mode Clock Gating Control"]
    pub rcgcssi: RCGCSSI,
    #[doc = "0x620 - Inter-Integrated Circuit Run Mode Clock Gating Control"]
    pub rcgci2c: RCGCI2C,
    _reserved85: [u8; 4usize],
    #[doc = "0x628 - Universal Serial Bus Run Mode Clock Gating Control"]
    pub rcgcusb: RCGCUSB,
    _reserved86: [u8; 8usize],
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
    _reserved91: [u8; 16usize],
    #[doc = "0x658 - EEPROM Run Mode Clock Gating Control"]
    pub rcgceeprom: RCGCEEPROM,
    #[doc = "0x65c - 32/64-Bit Wide General-Purpose Timer Run Mode Clock Gating Control"]
    pub rcgcwtimer: RCGCWTIMER,
    _reserved93: [u8; 160usize],
    #[doc = "0x700 - Watchdog Timer Sleep Mode Clock Gating Control"]
    pub scgcwd: SCGCWD,
    #[doc = "0x704 - 16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
    pub scgctimer: SCGCTIMER,
    #[doc = "0x708 - General-Purpose Input/Output Sleep Mode Clock Gating Control"]
    pub scgcgpio: SCGCGPIO,
    #[doc = "0x70c - Micro Direct Memory Access Sleep Mode Clock Gating Control"]
    pub scgcdma: SCGCDMA,
    _reserved97: [u8; 4usize],
    #[doc = "0x714 - Hibernation Sleep Mode Clock Gating Control"]
    pub scgchib: SCGCHIB,
    #[doc = "0x718 - Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
    pub scgcuart: SCGCUART,
    #[doc = "0x71c - Synchronous Serial Interface Sleep Mode Clock Gating Control"]
    pub scgcssi: SCGCSSI,
    #[doc = "0x720 - Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
    pub scgci2c: SCGCI2C,
    _reserved101: [u8; 4usize],
    #[doc = "0x728 - Universal Serial Bus Sleep Mode Clock Gating Control"]
    pub scgcusb: SCGCUSB,
    _reserved102: [u8; 8usize],
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
    _reserved107: [u8; 16usize],
    #[doc = "0x758 - EEPROM Sleep Mode Clock Gating Control"]
    pub scgceeprom: SCGCEEPROM,
    #[doc = "0x75c - 32/64-Bit Wide General-Purpose Timer Sleep Mode Clock Gating Control"]
    pub scgcwtimer: SCGCWTIMER,
    _reserved109: [u8; 160usize],
    #[doc = "0x800 - Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgcwd: DCGCWD,
    #[doc = "0x804 - 16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgctimer: DCGCTIMER,
    #[doc = "0x808 - General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
    pub dcgcgpio: DCGCGPIO,
    #[doc = "0x80c - Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
    pub dcgcdma: DCGCDMA,
    _reserved113: [u8; 4usize],
    #[doc = "0x814 - Hibernation Deep-Sleep Mode Clock Gating Control"]
    pub dcgchib: DCGCHIB,
    #[doc = "0x818 - Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
    pub dcgcuart: DCGCUART,
    #[doc = "0x81c - Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
    pub dcgcssi: DCGCSSI,
    #[doc = "0x820 - Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
    pub dcgci2c: DCGCI2C,
    _reserved117: [u8; 4usize],
    #[doc = "0x828 - Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
    pub dcgcusb: DCGCUSB,
    _reserved118: [u8; 8usize],
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
    _reserved123: [u8; 16usize],
    #[doc = "0x858 - EEPROM Deep-Sleep Mode Clock Gating Control"]
    pub dcgceeprom: DCGCEEPROM,
    #[doc = "0x85c - 32/64-Bit Wide General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgcwtimer: DCGCWTIMER,
    _reserved125: [u8; 416usize],
    #[doc = "0xa00 - Watchdog Timer Peripheral Ready"]
    pub prwd: PRWD,
    #[doc = "0xa04 - 16/32-Bit General-Purpose Timer Peripheral Ready"]
    pub prtimer: PRTIMER,
    #[doc = "0xa08 - General-Purpose Input/Output Peripheral Ready"]
    pub prgpio: PRGPIO,
    #[doc = "0xa0c - Micro Direct Memory Access Peripheral Ready"]
    pub prdma: PRDMA,
    _reserved129: [u8; 4usize],
    #[doc = "0xa14 - Hibernation Peripheral Ready"]
    pub prhib: PRHIB,
    #[doc = "0xa18 - Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
    pub pruart: PRUART,
    #[doc = "0xa1c - Synchronous Serial Interface Peripheral Ready"]
    pub prssi: PRSSI,
    #[doc = "0xa20 - Inter-Integrated Circuit Peripheral Ready"]
    pub pri2c: PRI2C,
    _reserved133: [u8; 4usize],
    #[doc = "0xa28 - Universal Serial Bus Peripheral Ready"]
    pub prusb: PRUSB,
    _reserved134: [u8; 8usize],
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
    _reserved139: [u8; 16usize],
    #[doc = "0xa58 - EEPROM Peripheral Ready"]
    pub preeprom: PREEPROM,
    #[doc = "0xa5c - 32/64-Bit Wide General-Purpose Timer Peripheral Ready"]
    pub prwtimer: PRWTIMER,
}
#[doc = "Device Identification 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [did0](did0) module"]
pub type DID0 = crate::Reg<u32, _DID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DID0;
#[doc = "`read()` method returns [did0::R](did0::R) reader structure"]
impl crate::Readable for DID0 {}
#[doc = "Device Identification 0"]
pub mod did0;
#[doc = "Device Identification 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [did1](did1) module"]
pub type DID1 = crate::Reg<u32, _DID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DID1;
#[doc = "`read()` method returns [did1::R](did1::R) reader structure"]
impl crate::Readable for DID1 {}
#[doc = "Device Identification 1"]
pub mod did1;
#[doc = "Device Capabilities 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc0](dc0) module"]
pub type DC0 = crate::Reg<u32, _DC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC0;
#[doc = "`read()` method returns [dc0::R](dc0::R) reader structure"]
impl crate::Readable for DC0 {}
#[doc = "Device Capabilities 0"]
pub mod dc0;
#[doc = "Device Capabilities 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc1](dc1) module"]
pub type DC1 = crate::Reg<u32, _DC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC1;
#[doc = "`read()` method returns [dc1::R](dc1::R) reader structure"]
impl crate::Readable for DC1 {}
#[doc = "Device Capabilities 1"]
pub mod dc1;
#[doc = "Device Capabilities 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc2](dc2) module"]
pub type DC2 = crate::Reg<u32, _DC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC2;
#[doc = "`read()` method returns [dc2::R](dc2::R) reader structure"]
impl crate::Readable for DC2 {}
#[doc = "Device Capabilities 2"]
pub mod dc2;
#[doc = "Device Capabilities 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc3](dc3) module"]
pub type DC3 = crate::Reg<u32, _DC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC3;
#[doc = "`read()` method returns [dc3::R](dc3::R) reader structure"]
impl crate::Readable for DC3 {}
#[doc = "Device Capabilities 3"]
pub mod dc3;
#[doc = "Device Capabilities 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc4](dc4) module"]
pub type DC4 = crate::Reg<u32, _DC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC4;
#[doc = "`read()` method returns [dc4::R](dc4::R) reader structure"]
impl crate::Readable for DC4 {}
#[doc = "Device Capabilities 4"]
pub mod dc4;
#[doc = "Device Capabilities 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc5](dc5) module"]
pub type DC5 = crate::Reg<u32, _DC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC5;
#[doc = "`read()` method returns [dc5::R](dc5::R) reader structure"]
impl crate::Readable for DC5 {}
#[doc = "Device Capabilities 5"]
pub mod dc5;
#[doc = "Device Capabilities 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc6](dc6) module"]
pub type DC6 = crate::Reg<u32, _DC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC6;
#[doc = "`read()` method returns [dc6::R](dc6::R) reader structure"]
impl crate::Readable for DC6 {}
#[doc = "Device Capabilities 6"]
pub mod dc6;
#[doc = "Device Capabilities 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc7](dc7) module"]
pub type DC7 = crate::Reg<u32, _DC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC7;
#[doc = "`read()` method returns [dc7::R](dc7::R) reader structure"]
impl crate::Readable for DC7 {}
#[doc = "Device Capabilities 7"]
pub mod dc7;
#[doc = "Device Capabilities 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc8](dc8) module"]
pub type DC8 = crate::Reg<u32, _DC8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC8;
#[doc = "`read()` method returns [dc8::R](dc8::R) reader structure"]
impl crate::Readable for DC8 {}
#[doc = "Device Capabilities 8"]
pub mod dc8;
#[doc = "Brown-Out Reset Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pborctl](pborctl) module"]
pub type PBORCTL = crate::Reg<u32, _PBORCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBORCTL;
#[doc = "`read()` method returns [pborctl::R](pborctl::R) reader structure"]
impl crate::Readable for PBORCTL {}
#[doc = "`write(|w| ..)` method takes [pborctl::W](pborctl::W) writer structure"]
impl crate::Writable for PBORCTL {}
#[doc = "Brown-Out Reset Control"]
pub mod pborctl;
#[doc = "Software Reset Control 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcr0](srcr0) module"]
pub type SRCR0 = crate::Reg<u32, _SRCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR0;
#[doc = "`read()` method returns [srcr0::R](srcr0::R) reader structure"]
impl crate::Readable for SRCR0 {}
#[doc = "Software Reset Control 0"]
pub mod srcr0;
#[doc = "Software Reset Control 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcr1](srcr1) module"]
pub type SRCR1 = crate::Reg<u32, _SRCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR1;
#[doc = "`read()` method returns [srcr1::R](srcr1::R) reader structure"]
impl crate::Readable for SRCR1 {}
#[doc = "Software Reset Control 1"]
pub mod srcr1;
#[doc = "Software Reset Control 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcr2](srcr2) module"]
pub type SRCR2 = crate::Reg<u32, _SRCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR2;
#[doc = "`read()` method returns [srcr2::R](srcr2::R) reader structure"]
impl crate::Readable for SRCR2 {}
#[doc = "Software Reset Control 2"]
pub mod srcr2;
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "Interrupt Mask Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imc](imc) module"]
pub type IMC = crate::Reg<u32, _IMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMC;
#[doc = "`read()` method returns [imc::R](imc::R) reader structure"]
impl crate::Readable for IMC {}
#[doc = "`write(|w| ..)` method takes [imc::W](imc::W) writer structure"]
impl crate::Writable for IMC {}
#[doc = "Interrupt Mask Control"]
pub mod imc;
#[doc = "Masked Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc](misc) module"]
pub type MISC = crate::Reg<u32, _MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC;
#[doc = "`read()` method returns [misc::R](misc::R) reader structure"]
impl crate::Readable for MISC {}
#[doc = "`write(|w| ..)` method takes [misc::W](misc::W) writer structure"]
impl crate::Writable for MISC {}
#[doc = "Masked Interrupt Status and Clear"]
pub mod misc;
#[doc = "Reset Cause\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resc](resc) module"]
pub type RESC = crate::Reg<u32, _RESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESC;
#[doc = "`read()` method returns [resc::R](resc::R) reader structure"]
impl crate::Readable for RESC {}
#[doc = "`write(|w| ..)` method takes [resc::W](resc::W) writer structure"]
impl crate::Writable for RESC {}
#[doc = "Reset Cause"]
pub mod resc;
#[doc = "Run-Mode Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcc](rcc) module"]
pub type RCC = crate::Reg<u32, _RCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC;
#[doc = "`read()` method returns [rcc::R](rcc::R) reader structure"]
impl crate::Readable for RCC {}
#[doc = "`write(|w| ..)` method takes [rcc::W](rcc::W) writer structure"]
impl crate::Writable for RCC {}
#[doc = "Run-Mode Clock Configuration"]
pub mod rcc;
#[doc = "GPIO High-Performance Bus Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiohbctl](gpiohbctl) module"]
pub type GPIOHBCTL = crate::Reg<u32, _GPIOHBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOHBCTL;
#[doc = "`read()` method returns [gpiohbctl::R](gpiohbctl::R) reader structure"]
impl crate::Readable for GPIOHBCTL {}
#[doc = "`write(|w| ..)` method takes [gpiohbctl::W](gpiohbctl::W) writer structure"]
impl crate::Writable for GPIOHBCTL {}
#[doc = "GPIO High-Performance Bus Control"]
pub mod gpiohbctl;
#[doc = "Run-Mode Clock Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcc2](rcc2) module"]
pub type RCC2 = crate::Reg<u32, _RCC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC2;
#[doc = "`read()` method returns [rcc2::R](rcc2::R) reader structure"]
impl crate::Readable for RCC2 {}
#[doc = "`write(|w| ..)` method takes [rcc2::W](rcc2::W) writer structure"]
impl crate::Writable for RCC2 {}
#[doc = "Run-Mode Clock Configuration 2"]
pub mod rcc2;
#[doc = "Main Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [moscctl](moscctl) module"]
pub type MOSCCTL = crate::Reg<u32, _MOSCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOSCCTL;
#[doc = "`read()` method returns [moscctl::R](moscctl::R) reader structure"]
impl crate::Readable for MOSCCTL {}
#[doc = "`write(|w| ..)` method takes [moscctl::W](moscctl::W) writer structure"]
impl crate::Writable for MOSCCTL {}
#[doc = "Main Oscillator Control"]
pub mod moscctl;
#[doc = "Run Mode Clock Gating Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgc0](rcgc0) module"]
pub type RCGC0 = crate::Reg<u32, _RCGC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGC0;
#[doc = "`read()` method returns [rcgc0::R](rcgc0::R) reader structure"]
impl crate::Readable for RCGC0 {}
#[doc = "Run Mode Clock Gating Control Register 0"]
pub mod rcgc0;
#[doc = "Run Mode Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgc1](rcgc1) module"]
pub type RCGC1 = crate::Reg<u32, _RCGC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGC1;
#[doc = "`read()` method returns [rcgc1::R](rcgc1::R) reader structure"]
impl crate::Readable for RCGC1 {}
#[doc = "Run Mode Clock Gating Control Register 1"]
pub mod rcgc1;
#[doc = "Run Mode Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgc2](rcgc2) module"]
pub type RCGC2 = crate::Reg<u32, _RCGC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGC2;
#[doc = "`read()` method returns [rcgc2::R](rcgc2::R) reader structure"]
impl crate::Readable for RCGC2 {}
#[doc = "Run Mode Clock Gating Control Register 2"]
pub mod rcgc2;
#[doc = "Sleep Mode Clock Gating Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc0](scgc0) module"]
pub type SCGC0 = crate::Reg<u32, _SCGC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC0;
#[doc = "`read()` method returns [scgc0::R](scgc0::R) reader structure"]
impl crate::Readable for SCGC0 {}
#[doc = "Sleep Mode Clock Gating Control Register 0"]
pub mod scgc0;
#[doc = "Sleep Mode Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc1](scgc1) module"]
pub type SCGC1 = crate::Reg<u32, _SCGC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC1;
#[doc = "`read()` method returns [scgc1::R](scgc1::R) reader structure"]
impl crate::Readable for SCGC1 {}
#[doc = "Sleep Mode Clock Gating Control Register 1"]
pub mod scgc1;
#[doc = "Sleep Mode Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc2](scgc2) module"]
pub type SCGC2 = crate::Reg<u32, _SCGC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC2;
#[doc = "`read()` method returns [scgc2::R](scgc2::R) reader structure"]
impl crate::Readable for SCGC2 {}
#[doc = "Sleep Mode Clock Gating Control Register 2"]
pub mod scgc2;
#[doc = "Deep Sleep Mode Clock Gating Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgc0](dcgc0) module"]
pub type DCGC0 = crate::Reg<u32, _DCGC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGC0;
#[doc = "`read()` method returns [dcgc0::R](dcgc0::R) reader structure"]
impl crate::Readable for DCGC0 {}
#[doc = "Deep Sleep Mode Clock Gating Control Register 0"]
pub mod dcgc0;
#[doc = "Deep-Sleep Mode Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgc1](dcgc1) module"]
pub type DCGC1 = crate::Reg<u32, _DCGC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGC1;
#[doc = "`read()` method returns [dcgc1::R](dcgc1::R) reader structure"]
impl crate::Readable for DCGC1 {}
#[doc = "Deep-Sleep Mode Clock Gating Control Register 1"]
pub mod dcgc1;
#[doc = "Deep Sleep Mode Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgc2](dcgc2) module"]
pub type DCGC2 = crate::Reg<u32, _DCGC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGC2;
#[doc = "`read()` method returns [dcgc2::R](dcgc2::R) reader structure"]
impl crate::Readable for DCGC2 {}
#[doc = "Deep Sleep Mode Clock Gating Control Register 2"]
pub mod dcgc2;
#[doc = "Deep Sleep Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dslpclkcfg](dslpclkcfg) module"]
pub type DSLPCLKCFG = crate::Reg<u32, _DSLPCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSLPCLKCFG;
#[doc = "`read()` method returns [dslpclkcfg::R](dslpclkcfg::R) reader structure"]
impl crate::Readable for DSLPCLKCFG {}
#[doc = "`write(|w| ..)` method takes [dslpclkcfg::W](dslpclkcfg::W) writer structure"]
impl crate::Writable for DSLPCLKCFG {}
#[doc = "Deep Sleep Clock Configuration"]
pub mod dslpclkcfg;
#[doc = "System Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysprop](sysprop) module"]
pub type SYSPROP = crate::Reg<u32, _SYSPROP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPROP;
#[doc = "`read()` method returns [sysprop::R](sysprop::R) reader structure"]
impl crate::Readable for SYSPROP {}
#[doc = "System Properties"]
pub mod sysprop;
#[doc = "Precision Internal Oscillator Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [piosccal](piosccal) module"]
pub type PIOSCCAL = crate::Reg<u32, _PIOSCCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIOSCCAL;
#[doc = "`read()` method returns [piosccal::R](piosccal::R) reader structure"]
impl crate::Readable for PIOSCCAL {}
#[doc = "`write(|w| ..)` method takes [piosccal::W](piosccal::W) writer structure"]
impl crate::Writable for PIOSCCAL {}
#[doc = "Precision Internal Oscillator Calibration"]
pub mod piosccal;
#[doc = "Precision Internal Oscillator Statistics\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pioscstat](pioscstat) module"]
pub type PIOSCSTAT = crate::Reg<u32, _PIOSCSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIOSCSTAT;
#[doc = "`read()` method returns [pioscstat::R](pioscstat::R) reader structure"]
impl crate::Readable for PIOSCSTAT {}
#[doc = "Precision Internal Oscillator Statistics"]
pub mod pioscstat;
#[doc = "PLL Frequency 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pllfreq0](pllfreq0) module"]
pub type PLLFREQ0 = crate::Reg<u32, _PLLFREQ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLFREQ0;
#[doc = "`read()` method returns [pllfreq0::R](pllfreq0::R) reader structure"]
impl crate::Readable for PLLFREQ0 {}
#[doc = "`write(|w| ..)` method takes [pllfreq0::W](pllfreq0::W) writer structure"]
impl crate::Writable for PLLFREQ0 {}
#[doc = "PLL Frequency 0"]
pub mod pllfreq0;
#[doc = "PLL Frequency 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pllfreq1](pllfreq1) module"]
pub type PLLFREQ1 = crate::Reg<u32, _PLLFREQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLFREQ1;
#[doc = "`read()` method returns [pllfreq1::R](pllfreq1::R) reader structure"]
impl crate::Readable for PLLFREQ1 {}
#[doc = "`write(|w| ..)` method takes [pllfreq1::W](pllfreq1::W) writer structure"]
impl crate::Writable for PLLFREQ1 {}
#[doc = "PLL Frequency 1"]
pub mod pllfreq1;
#[doc = "PLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pllstat](pllstat) module"]
pub type PLLSTAT = crate::Reg<u32, _PLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSTAT;
#[doc = "`read()` method returns [pllstat::R](pllstat::R) reader structure"]
impl crate::Readable for PLLSTAT {}
#[doc = "PLL Status"]
pub mod pllstat;
#[doc = "Sleep Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slppwrcfg](slppwrcfg) module"]
pub type SLPPWRCFG = crate::Reg<u32, _SLPPWRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLPPWRCFG;
#[doc = "`read()` method returns [slppwrcfg::R](slppwrcfg::R) reader structure"]
impl crate::Readable for SLPPWRCFG {}
#[doc = "`write(|w| ..)` method takes [slppwrcfg::W](slppwrcfg::W) writer structure"]
impl crate::Writable for SLPPWRCFG {}
#[doc = "Sleep Power Configuration"]
pub mod slppwrcfg;
#[doc = "Deep-Sleep Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dslppwrcfg](dslppwrcfg) module"]
pub type DSLPPWRCFG = crate::Reg<u32, _DSLPPWRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSLPPWRCFG;
#[doc = "`read()` method returns [dslppwrcfg::R](dslppwrcfg::R) reader structure"]
impl crate::Readable for DSLPPWRCFG {}
#[doc = "`write(|w| ..)` method takes [dslppwrcfg::W](dslppwrcfg::W) writer structure"]
impl crate::Writable for DSLPPWRCFG {}
#[doc = "Deep-Sleep Power Configuration"]
pub mod dslppwrcfg;
#[doc = "Device Capabilities 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dc9](dc9) module"]
pub type DC9 = crate::Reg<u32, _DC9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC9;
#[doc = "`read()` method returns [dc9::R](dc9::R) reader structure"]
impl crate::Readable for DC9 {}
#[doc = "Device Capabilities 9"]
pub mod dc9;
#[doc = "Non-Volatile Memory Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvmstat](nvmstat) module"]
pub type NVMSTAT = crate::Reg<u32, _NVMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMSTAT;
#[doc = "`read()` method returns [nvmstat::R](nvmstat::R) reader structure"]
impl crate::Readable for NVMSTAT {}
#[doc = "Non-Volatile Memory Information"]
pub mod nvmstat;
#[doc = "LDO Sleep Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ldospctl](ldospctl) module"]
pub type LDOSPCTL = crate::Reg<u32, _LDOSPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDOSPCTL;
#[doc = "`read()` method returns [ldospctl::R](ldospctl::R) reader structure"]
impl crate::Readable for LDOSPCTL {}
#[doc = "`write(|w| ..)` method takes [ldospctl::W](ldospctl::W) writer structure"]
impl crate::Writable for LDOSPCTL {}
#[doc = "LDO Sleep Power Control"]
pub mod ldospctl;
#[doc = "LDO Deep-Sleep Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ldodpctl](ldodpctl) module"]
pub type LDODPCTL = crate::Reg<u32, _LDODPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDODPCTL;
#[doc = "`read()` method returns [ldodpctl::R](ldodpctl::R) reader structure"]
impl crate::Readable for LDODPCTL {}
#[doc = "`write(|w| ..)` method takes [ldodpctl::W](ldodpctl::W) writer structure"]
impl crate::Writable for LDODPCTL {}
#[doc = "LDO Deep-Sleep Power Control"]
pub mod ldodpctl;
#[doc = "Watchdog Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppwd](ppwd) module"]
pub type PPWD = crate::Reg<u32, _PPWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPWD;
#[doc = "`read()` method returns [ppwd::R](ppwd::R) reader structure"]
impl crate::Readable for PPWD {}
#[doc = "Watchdog Timer Peripheral Present"]
pub mod ppwd;
#[doc = "16/32-Bit General-Purpose Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pptimer](pptimer) module"]
pub type PPTIMER = crate::Reg<u32, _PPTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPTIMER;
#[doc = "`read()` method returns [pptimer::R](pptimer::R) reader structure"]
impl crate::Readable for PPTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Peripheral Present"]
pub mod pptimer;
#[doc = "General-Purpose Input/Output Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppgpio](ppgpio) module"]
pub type PPGPIO = crate::Reg<u32, _PPGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPGPIO;
#[doc = "`read()` method returns [ppgpio::R](ppgpio::R) reader structure"]
impl crate::Readable for PPGPIO {}
#[doc = "General-Purpose Input/Output Peripheral Present"]
pub mod ppgpio;
#[doc = "Micro Direct Memory Access Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppdma](ppdma) module"]
pub type PPDMA = crate::Reg<u32, _PPDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPDMA;
#[doc = "`read()` method returns [ppdma::R](ppdma::R) reader structure"]
impl crate::Readable for PPDMA {}
#[doc = "Micro Direct Memory Access Peripheral Present"]
pub mod ppdma;
#[doc = "Hibernation Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pphib](pphib) module"]
pub type PPHIB = crate::Reg<u32, _PPHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPHIB;
#[doc = "`read()` method returns [pphib::R](pphib::R) reader structure"]
impl crate::Readable for PPHIB {}
#[doc = "Hibernation Peripheral Present"]
pub mod pphib;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppuart](ppuart) module"]
pub type PPUART = crate::Reg<u32, _PPUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUART;
#[doc = "`read()` method returns [ppuart::R](ppuart::R) reader structure"]
impl crate::Readable for PPUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present"]
pub mod ppuart;
#[doc = "Synchronous Serial Interface Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppssi](ppssi) module"]
pub type PPSSI = crate::Reg<u32, _PPSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPSSI;
#[doc = "`read()` method returns [ppssi::R](ppssi::R) reader structure"]
impl crate::Readable for PPSSI {}
#[doc = "Synchronous Serial Interface Peripheral Present"]
pub mod ppssi;
#[doc = "Inter-Integrated Circuit Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppi2c](ppi2c) module"]
pub type PPI2C = crate::Reg<u32, _PPI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPI2C;
#[doc = "`read()` method returns [ppi2c::R](ppi2c::R) reader structure"]
impl crate::Readable for PPI2C {}
#[doc = "Inter-Integrated Circuit Peripheral Present"]
pub mod ppi2c;
#[doc = "Universal Serial Bus Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppusb](ppusb) module"]
pub type PPUSB = crate::Reg<u32, _PPUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUSB;
#[doc = "`read()` method returns [ppusb::R](ppusb::R) reader structure"]
impl crate::Readable for PPUSB {}
#[doc = "Universal Serial Bus Peripheral Present"]
pub mod ppusb;
#[doc = "Controller Area Network Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppcan](ppcan) module"]
pub type PPCAN = crate::Reg<u32, _PPCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPCAN;
#[doc = "`read()` method returns [ppcan::R](ppcan::R) reader structure"]
impl crate::Readable for PPCAN {}
#[doc = "Controller Area Network Peripheral Present"]
pub mod ppcan;
#[doc = "Analog-to-Digital Converter Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppadc](ppadc) module"]
pub type PPADC = crate::Reg<u32, _PPADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPADC;
#[doc = "`read()` method returns [ppadc::R](ppadc::R) reader structure"]
impl crate::Readable for PPADC {}
#[doc = "Analog-to-Digital Converter Peripheral Present"]
pub mod ppadc;
#[doc = "Analog Comparator Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppacmp](ppacmp) module"]
pub type PPACMP = crate::Reg<u32, _PPACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPACMP;
#[doc = "`read()` method returns [ppacmp::R](ppacmp::R) reader structure"]
impl crate::Readable for PPACMP {}
#[doc = "Analog Comparator Peripheral Present"]
pub mod ppacmp;
#[doc = "Pulse Width Modulator Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pppwm](pppwm) module"]
pub type PPPWM = crate::Reg<u32, _PPPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPPWM;
#[doc = "`read()` method returns [pppwm::R](pppwm::R) reader structure"]
impl crate::Readable for PPPWM {}
#[doc = "Pulse Width Modulator Peripheral Present"]
pub mod pppwm;
#[doc = "Quadrature Encoder Interface Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppqei](ppqei) module"]
pub type PPQEI = crate::Reg<u32, _PPQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPQEI;
#[doc = "`read()` method returns [ppqei::R](ppqei::R) reader structure"]
impl crate::Readable for PPQEI {}
#[doc = "Quadrature Encoder Interface Peripheral Present"]
pub mod ppqei;
#[doc = "EEPROM Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppeeprom](ppeeprom) module"]
pub type PPEEPROM = crate::Reg<u32, _PPEEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPEEPROM;
#[doc = "`read()` method returns [ppeeprom::R](ppeeprom::R) reader structure"]
impl crate::Readable for PPEEPROM {}
#[doc = "EEPROM Peripheral Present"]
pub mod ppeeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppwtimer](ppwtimer) module"]
pub type PPWTIMER = crate::Reg<u32, _PPWTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPWTIMER;
#[doc = "`read()` method returns [ppwtimer::R](ppwtimer::R) reader structure"]
impl crate::Readable for PPWTIMER {}
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Present"]
pub mod ppwtimer;
#[doc = "Watchdog Timer Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srwd](srwd) module"]
pub type SRWD = crate::Reg<u32, _SRWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRWD;
#[doc = "`read()` method returns [srwd::R](srwd::R) reader structure"]
impl crate::Readable for SRWD {}
#[doc = "`write(|w| ..)` method takes [srwd::W](srwd::W) writer structure"]
impl crate::Writable for SRWD {}
#[doc = "Watchdog Timer Software Reset"]
pub mod srwd;
#[doc = "16/32-Bit General-Purpose Timer Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srtimer](srtimer) module"]
pub type SRTIMER = crate::Reg<u32, _SRTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRTIMER;
#[doc = "`read()` method returns [srtimer::R](srtimer::R) reader structure"]
impl crate::Readable for SRTIMER {}
#[doc = "`write(|w| ..)` method takes [srtimer::W](srtimer::W) writer structure"]
impl crate::Writable for SRTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Software Reset"]
pub mod srtimer;
#[doc = "General-Purpose Input/Output Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srgpio](srgpio) module"]
pub type SRGPIO = crate::Reg<u32, _SRGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRGPIO;
#[doc = "`read()` method returns [srgpio::R](srgpio::R) reader structure"]
impl crate::Readable for SRGPIO {}
#[doc = "`write(|w| ..)` method takes [srgpio::W](srgpio::W) writer structure"]
impl crate::Writable for SRGPIO {}
#[doc = "General-Purpose Input/Output Software Reset"]
pub mod srgpio;
#[doc = "Micro Direct Memory Access Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srdma](srdma) module"]
pub type SRDMA = crate::Reg<u32, _SRDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRDMA;
#[doc = "`read()` method returns [srdma::R](srdma::R) reader structure"]
impl crate::Readable for SRDMA {}
#[doc = "`write(|w| ..)` method takes [srdma::W](srdma::W) writer structure"]
impl crate::Writable for SRDMA {}
#[doc = "Micro Direct Memory Access Software Reset"]
pub mod srdma;
#[doc = "Hibernation Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srhib](srhib) module"]
pub type SRHIB = crate::Reg<u32, _SRHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRHIB;
#[doc = "`read()` method returns [srhib::R](srhib::R) reader structure"]
impl crate::Readable for SRHIB {}
#[doc = "`write(|w| ..)` method takes [srhib::W](srhib::W) writer structure"]
impl crate::Writable for SRHIB {}
#[doc = "Hibernation Software Reset"]
pub mod srhib;
#[doc = "Universal Asynchronous Receiver/Transmitter Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sruart](sruart) module"]
pub type SRUART = crate::Reg<u32, _SRUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRUART;
#[doc = "`read()` method returns [sruart::R](sruart::R) reader structure"]
impl crate::Readable for SRUART {}
#[doc = "`write(|w| ..)` method takes [sruart::W](sruart::W) writer structure"]
impl crate::Writable for SRUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Software Reset"]
pub mod sruart;
#[doc = "Synchronous Serial Interface Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srssi](srssi) module"]
pub type SRSSI = crate::Reg<u32, _SRSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSSI;
#[doc = "`read()` method returns [srssi::R](srssi::R) reader structure"]
impl crate::Readable for SRSSI {}
#[doc = "`write(|w| ..)` method takes [srssi::W](srssi::W) writer structure"]
impl crate::Writable for SRSSI {}
#[doc = "Synchronous Serial Interface Software Reset"]
pub mod srssi;
#[doc = "Inter-Integrated Circuit Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sri2c](sri2c) module"]
pub type SRI2C = crate::Reg<u32, _SRI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRI2C;
#[doc = "`read()` method returns [sri2c::R](sri2c::R) reader structure"]
impl crate::Readable for SRI2C {}
#[doc = "`write(|w| ..)` method takes [sri2c::W](sri2c::W) writer structure"]
impl crate::Writable for SRI2C {}
#[doc = "Inter-Integrated Circuit Software Reset"]
pub mod sri2c;
#[doc = "Universal Serial Bus Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srusb](srusb) module"]
pub type SRUSB = crate::Reg<u32, _SRUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRUSB;
#[doc = "`read()` method returns [srusb::R](srusb::R) reader structure"]
impl crate::Readable for SRUSB {}
#[doc = "`write(|w| ..)` method takes [srusb::W](srusb::W) writer structure"]
impl crate::Writable for SRUSB {}
#[doc = "Universal Serial Bus Software Reset"]
pub mod srusb;
#[doc = "Controller Area Network Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcan](srcan) module"]
pub type SRCAN = crate::Reg<u32, _SRCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCAN;
#[doc = "`read()` method returns [srcan::R](srcan::R) reader structure"]
impl crate::Readable for SRCAN {}
#[doc = "`write(|w| ..)` method takes [srcan::W](srcan::W) writer structure"]
impl crate::Writable for SRCAN {}
#[doc = "Controller Area Network Software Reset"]
pub mod srcan;
#[doc = "Analog-to-Digital Converter Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sradc](sradc) module"]
pub type SRADC = crate::Reg<u32, _SRADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRADC;
#[doc = "`read()` method returns [sradc::R](sradc::R) reader structure"]
impl crate::Readable for SRADC {}
#[doc = "`write(|w| ..)` method takes [sradc::W](sradc::W) writer structure"]
impl crate::Writable for SRADC {}
#[doc = "Analog-to-Digital Converter Software Reset"]
pub mod sradc;
#[doc = "Analog Comparator Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sracmp](sracmp) module"]
pub type SRACMP = crate::Reg<u32, _SRACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRACMP;
#[doc = "`read()` method returns [sracmp::R](sracmp::R) reader structure"]
impl crate::Readable for SRACMP {}
#[doc = "`write(|w| ..)` method takes [sracmp::W](sracmp::W) writer structure"]
impl crate::Writable for SRACMP {}
#[doc = "Analog Comparator Software Reset"]
pub mod sracmp;
#[doc = "Pulse Width Modulator Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srpwm](srpwm) module"]
pub type SRPWM = crate::Reg<u32, _SRPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRPWM;
#[doc = "`read()` method returns [srpwm::R](srpwm::R) reader structure"]
impl crate::Readable for SRPWM {}
#[doc = "`write(|w| ..)` method takes [srpwm::W](srpwm::W) writer structure"]
impl crate::Writable for SRPWM {}
#[doc = "Pulse Width Modulator Software Reset"]
pub mod srpwm;
#[doc = "Quadrature Encoder Interface Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srqei](srqei) module"]
pub type SRQEI = crate::Reg<u32, _SRQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRQEI;
#[doc = "`read()` method returns [srqei::R](srqei::R) reader structure"]
impl crate::Readable for SRQEI {}
#[doc = "`write(|w| ..)` method takes [srqei::W](srqei::W) writer structure"]
impl crate::Writable for SRQEI {}
#[doc = "Quadrature Encoder Interface Software Reset"]
pub mod srqei;
#[doc = "EEPROM Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sreeprom](sreeprom) module"]
pub type SREEPROM = crate::Reg<u32, _SREEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SREEPROM;
#[doc = "`read()` method returns [sreeprom::R](sreeprom::R) reader structure"]
impl crate::Readable for SREEPROM {}
#[doc = "`write(|w| ..)` method takes [sreeprom::W](sreeprom::W) writer structure"]
impl crate::Writable for SREEPROM {}
#[doc = "EEPROM Software Reset"]
pub mod sreeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srwtimer](srwtimer) module"]
pub type SRWTIMER = crate::Reg<u32, _SRWTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRWTIMER;
#[doc = "`read()` method returns [srwtimer::R](srwtimer::R) reader structure"]
impl crate::Readable for SRWTIMER {}
#[doc = "`write(|w| ..)` method takes [srwtimer::W](srwtimer::W) writer structure"]
impl crate::Writable for SRWTIMER {}
#[doc = "32/64-Bit Wide General-Purpose Timer Software Reset"]
pub mod srwtimer;
#[doc = "Watchdog Timer Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcwd](rcgcwd) module"]
pub type RCGCWD = crate::Reg<u32, _RCGCWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCWD;
#[doc = "`read()` method returns [rcgcwd::R](rcgcwd::R) reader structure"]
impl crate::Readable for RCGCWD {}
#[doc = "`write(|w| ..)` method takes [rcgcwd::W](rcgcwd::W) writer structure"]
impl crate::Writable for RCGCWD {}
#[doc = "Watchdog Timer Run Mode Clock Gating Control"]
pub mod rcgcwd;
#[doc = "16/32-Bit General-Purpose Timer Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgctimer](rcgctimer) module"]
pub type RCGCTIMER = crate::Reg<u32, _RCGCTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCTIMER;
#[doc = "`read()` method returns [rcgctimer::R](rcgctimer::R) reader structure"]
impl crate::Readable for RCGCTIMER {}
#[doc = "`write(|w| ..)` method takes [rcgctimer::W](rcgctimer::W) writer structure"]
impl crate::Writable for RCGCTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
pub mod rcgctimer;
#[doc = "General-Purpose Input/Output Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcgpio](rcgcgpio) module"]
pub type RCGCGPIO = crate::Reg<u32, _RCGCGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCGPIO;
#[doc = "`read()` method returns [rcgcgpio::R](rcgcgpio::R) reader structure"]
impl crate::Readable for RCGCGPIO {}
#[doc = "`write(|w| ..)` method takes [rcgcgpio::W](rcgcgpio::W) writer structure"]
impl crate::Writable for RCGCGPIO {}
#[doc = "General-Purpose Input/Output Run Mode Clock Gating Control"]
pub mod rcgcgpio;
#[doc = "Micro Direct Memory Access Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcdma](rcgcdma) module"]
pub type RCGCDMA = crate::Reg<u32, _RCGCDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCDMA;
#[doc = "`read()` method returns [rcgcdma::R](rcgcdma::R) reader structure"]
impl crate::Readable for RCGCDMA {}
#[doc = "`write(|w| ..)` method takes [rcgcdma::W](rcgcdma::W) writer structure"]
impl crate::Writable for RCGCDMA {}
#[doc = "Micro Direct Memory Access Run Mode Clock Gating Control"]
pub mod rcgcdma;
#[doc = "Hibernation Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgchib](rcgchib) module"]
pub type RCGCHIB = crate::Reg<u32, _RCGCHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCHIB;
#[doc = "`read()` method returns [rcgchib::R](rcgchib::R) reader structure"]
impl crate::Readable for RCGCHIB {}
#[doc = "`write(|w| ..)` method takes [rcgchib::W](rcgchib::W) writer structure"]
impl crate::Writable for RCGCHIB {}
#[doc = "Hibernation Run Mode Clock Gating Control"]
pub mod rcgchib;
#[doc = "Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcuart](rcgcuart) module"]
pub type RCGCUART = crate::Reg<u32, _RCGCUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCUART;
#[doc = "`read()` method returns [rcgcuart::R](rcgcuart::R) reader structure"]
impl crate::Readable for RCGCUART {}
#[doc = "`write(|w| ..)` method takes [rcgcuart::W](rcgcuart::W) writer structure"]
impl crate::Writable for RCGCUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
pub mod rcgcuart;
#[doc = "Synchronous Serial Interface Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcssi](rcgcssi) module"]
pub type RCGCSSI = crate::Reg<u32, _RCGCSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCSSI;
#[doc = "`read()` method returns [rcgcssi::R](rcgcssi::R) reader structure"]
impl crate::Readable for RCGCSSI {}
#[doc = "`write(|w| ..)` method takes [rcgcssi::W](rcgcssi::W) writer structure"]
impl crate::Writable for RCGCSSI {}
#[doc = "Synchronous Serial Interface Run Mode Clock Gating Control"]
pub mod rcgcssi;
#[doc = "Inter-Integrated Circuit Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgci2c](rcgci2c) module"]
pub type RCGCI2C = crate::Reg<u32, _RCGCI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCI2C;
#[doc = "`read()` method returns [rcgci2c::R](rcgci2c::R) reader structure"]
impl crate::Readable for RCGCI2C {}
#[doc = "`write(|w| ..)` method takes [rcgci2c::W](rcgci2c::W) writer structure"]
impl crate::Writable for RCGCI2C {}
#[doc = "Inter-Integrated Circuit Run Mode Clock Gating Control"]
pub mod rcgci2c;
#[doc = "Universal Serial Bus Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcusb](rcgcusb) module"]
pub type RCGCUSB = crate::Reg<u32, _RCGCUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCUSB;
#[doc = "`read()` method returns [rcgcusb::R](rcgcusb::R) reader structure"]
impl crate::Readable for RCGCUSB {}
#[doc = "`write(|w| ..)` method takes [rcgcusb::W](rcgcusb::W) writer structure"]
impl crate::Writable for RCGCUSB {}
#[doc = "Universal Serial Bus Run Mode Clock Gating Control"]
pub mod rcgcusb;
#[doc = "Controller Area Network Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgccan](rcgccan) module"]
pub type RCGCCAN = crate::Reg<u32, _RCGCCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCCAN;
#[doc = "`read()` method returns [rcgccan::R](rcgccan::R) reader structure"]
impl crate::Readable for RCGCCAN {}
#[doc = "`write(|w| ..)` method takes [rcgccan::W](rcgccan::W) writer structure"]
impl crate::Writable for RCGCCAN {}
#[doc = "Controller Area Network Run Mode Clock Gating Control"]
pub mod rcgccan;
#[doc = "Analog-to-Digital Converter Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcadc](rcgcadc) module"]
pub type RCGCADC = crate::Reg<u32, _RCGCADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCADC;
#[doc = "`read()` method returns [rcgcadc::R](rcgcadc::R) reader structure"]
impl crate::Readable for RCGCADC {}
#[doc = "`write(|w| ..)` method takes [rcgcadc::W](rcgcadc::W) writer structure"]
impl crate::Writable for RCGCADC {}
#[doc = "Analog-to-Digital Converter Run Mode Clock Gating Control"]
pub mod rcgcadc;
#[doc = "Analog Comparator Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcacmp](rcgcacmp) module"]
pub type RCGCACMP = crate::Reg<u32, _RCGCACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCACMP;
#[doc = "`read()` method returns [rcgcacmp::R](rcgcacmp::R) reader structure"]
impl crate::Readable for RCGCACMP {}
#[doc = "`write(|w| ..)` method takes [rcgcacmp::W](rcgcacmp::W) writer structure"]
impl crate::Writable for RCGCACMP {}
#[doc = "Analog Comparator Run Mode Clock Gating Control"]
pub mod rcgcacmp;
#[doc = "Pulse Width Modulator Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcpwm](rcgcpwm) module"]
pub type RCGCPWM = crate::Reg<u32, _RCGCPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCPWM;
#[doc = "`read()` method returns [rcgcpwm::R](rcgcpwm::R) reader structure"]
impl crate::Readable for RCGCPWM {}
#[doc = "`write(|w| ..)` method takes [rcgcpwm::W](rcgcpwm::W) writer structure"]
impl crate::Writable for RCGCPWM {}
#[doc = "Pulse Width Modulator Run Mode Clock Gating Control"]
pub mod rcgcpwm;
#[doc = "Quadrature Encoder Interface Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcqei](rcgcqei) module"]
pub type RCGCQEI = crate::Reg<u32, _RCGCQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCQEI;
#[doc = "`read()` method returns [rcgcqei::R](rcgcqei::R) reader structure"]
impl crate::Readable for RCGCQEI {}
#[doc = "`write(|w| ..)` method takes [rcgcqei::W](rcgcqei::W) writer structure"]
impl crate::Writable for RCGCQEI {}
#[doc = "Quadrature Encoder Interface Run Mode Clock Gating Control"]
pub mod rcgcqei;
#[doc = "EEPROM Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgceeprom](rcgceeprom) module"]
pub type RCGCEEPROM = crate::Reg<u32, _RCGCEEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCEEPROM;
#[doc = "`read()` method returns [rcgceeprom::R](rcgceeprom::R) reader structure"]
impl crate::Readable for RCGCEEPROM {}
#[doc = "`write(|w| ..)` method takes [rcgceeprom::W](rcgceeprom::W) writer structure"]
impl crate::Writable for RCGCEEPROM {}
#[doc = "EEPROM Run Mode Clock Gating Control"]
pub mod rcgceeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcwtimer](rcgcwtimer) module"]
pub type RCGCWTIMER = crate::Reg<u32, _RCGCWTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCWTIMER;
#[doc = "`read()` method returns [rcgcwtimer::R](rcgcwtimer::R) reader structure"]
impl crate::Readable for RCGCWTIMER {}
#[doc = "`write(|w| ..)` method takes [rcgcwtimer::W](rcgcwtimer::W) writer structure"]
impl crate::Writable for RCGCWTIMER {}
#[doc = "32/64-Bit Wide General-Purpose Timer Run Mode Clock Gating Control"]
pub mod rcgcwtimer;
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcwd](scgcwd) module"]
pub type SCGCWD = crate::Reg<u32, _SCGCWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCWD;
#[doc = "`read()` method returns [scgcwd::R](scgcwd::R) reader structure"]
impl crate::Readable for SCGCWD {}
#[doc = "`write(|w| ..)` method takes [scgcwd::W](scgcwd::W) writer structure"]
impl crate::Writable for SCGCWD {}
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control"]
pub mod scgcwd;
#[doc = "16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgctimer](scgctimer) module"]
pub type SCGCTIMER = crate::Reg<u32, _SCGCTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCTIMER;
#[doc = "`read()` method returns [scgctimer::R](scgctimer::R) reader structure"]
impl crate::Readable for SCGCTIMER {}
#[doc = "`write(|w| ..)` method takes [scgctimer::W](scgctimer::W) writer structure"]
impl crate::Writable for SCGCTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
pub mod scgctimer;
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcgpio](scgcgpio) module"]
pub type SCGCGPIO = crate::Reg<u32, _SCGCGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCGPIO;
#[doc = "`read()` method returns [scgcgpio::R](scgcgpio::R) reader structure"]
impl crate::Readable for SCGCGPIO {}
#[doc = "`write(|w| ..)` method takes [scgcgpio::W](scgcgpio::W) writer structure"]
impl crate::Writable for SCGCGPIO {}
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control"]
pub mod scgcgpio;
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcdma](scgcdma) module"]
pub type SCGCDMA = crate::Reg<u32, _SCGCDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCDMA;
#[doc = "`read()` method returns [scgcdma::R](scgcdma::R) reader structure"]
impl crate::Readable for SCGCDMA {}
#[doc = "`write(|w| ..)` method takes [scgcdma::W](scgcdma::W) writer structure"]
impl crate::Writable for SCGCDMA {}
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control"]
pub mod scgcdma;
#[doc = "Hibernation Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgchib](scgchib) module"]
pub type SCGCHIB = crate::Reg<u32, _SCGCHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCHIB;
#[doc = "`read()` method returns [scgchib::R](scgchib::R) reader structure"]
impl crate::Readable for SCGCHIB {}
#[doc = "`write(|w| ..)` method takes [scgchib::W](scgchib::W) writer structure"]
impl crate::Writable for SCGCHIB {}
#[doc = "Hibernation Sleep Mode Clock Gating Control"]
pub mod scgchib;
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcuart](scgcuart) module"]
pub type SCGCUART = crate::Reg<u32, _SCGCUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCUART;
#[doc = "`read()` method returns [scgcuart::R](scgcuart::R) reader structure"]
impl crate::Readable for SCGCUART {}
#[doc = "`write(|w| ..)` method takes [scgcuart::W](scgcuart::W) writer structure"]
impl crate::Writable for SCGCUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
pub mod scgcuart;
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcssi](scgcssi) module"]
pub type SCGCSSI = crate::Reg<u32, _SCGCSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCSSI;
#[doc = "`read()` method returns [scgcssi::R](scgcssi::R) reader structure"]
impl crate::Readable for SCGCSSI {}
#[doc = "`write(|w| ..)` method takes [scgcssi::W](scgcssi::W) writer structure"]
impl crate::Writable for SCGCSSI {}
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control"]
pub mod scgcssi;
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgci2c](scgci2c) module"]
pub type SCGCI2C = crate::Reg<u32, _SCGCI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCI2C;
#[doc = "`read()` method returns [scgci2c::R](scgci2c::R) reader structure"]
impl crate::Readable for SCGCI2C {}
#[doc = "`write(|w| ..)` method takes [scgci2c::W](scgci2c::W) writer structure"]
impl crate::Writable for SCGCI2C {}
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
pub mod scgci2c;
#[doc = "Universal Serial Bus Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcusb](scgcusb) module"]
pub type SCGCUSB = crate::Reg<u32, _SCGCUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCUSB;
#[doc = "`read()` method returns [scgcusb::R](scgcusb::R) reader structure"]
impl crate::Readable for SCGCUSB {}
#[doc = "`write(|w| ..)` method takes [scgcusb::W](scgcusb::W) writer structure"]
impl crate::Writable for SCGCUSB {}
#[doc = "Universal Serial Bus Sleep Mode Clock Gating Control"]
pub mod scgcusb;
#[doc = "Controller Area Network Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgccan](scgccan) module"]
pub type SCGCCAN = crate::Reg<u32, _SCGCCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCCAN;
#[doc = "`read()` method returns [scgccan::R](scgccan::R) reader structure"]
impl crate::Readable for SCGCCAN {}
#[doc = "`write(|w| ..)` method takes [scgccan::W](scgccan::W) writer structure"]
impl crate::Writable for SCGCCAN {}
#[doc = "Controller Area Network Sleep Mode Clock Gating Control"]
pub mod scgccan;
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcadc](scgcadc) module"]
pub type SCGCADC = crate::Reg<u32, _SCGCADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCADC;
#[doc = "`read()` method returns [scgcadc::R](scgcadc::R) reader structure"]
impl crate::Readable for SCGCADC {}
#[doc = "`write(|w| ..)` method takes [scgcadc::W](scgcadc::W) writer structure"]
impl crate::Writable for SCGCADC {}
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
pub mod scgcadc;
#[doc = "Analog Comparator Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcacmp](scgcacmp) module"]
pub type SCGCACMP = crate::Reg<u32, _SCGCACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCACMP;
#[doc = "`read()` method returns [scgcacmp::R](scgcacmp::R) reader structure"]
impl crate::Readable for SCGCACMP {}
#[doc = "`write(|w| ..)` method takes [scgcacmp::W](scgcacmp::W) writer structure"]
impl crate::Writable for SCGCACMP {}
#[doc = "Analog Comparator Sleep Mode Clock Gating Control"]
pub mod scgcacmp;
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcpwm](scgcpwm) module"]
pub type SCGCPWM = crate::Reg<u32, _SCGCPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCPWM;
#[doc = "`read()` method returns [scgcpwm::R](scgcpwm::R) reader structure"]
impl crate::Readable for SCGCPWM {}
#[doc = "`write(|w| ..)` method takes [scgcpwm::W](scgcpwm::W) writer structure"]
impl crate::Writable for SCGCPWM {}
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control"]
pub mod scgcpwm;
#[doc = "Quadrature Encoder Interface Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcqei](scgcqei) module"]
pub type SCGCQEI = crate::Reg<u32, _SCGCQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCQEI;
#[doc = "`read()` method returns [scgcqei::R](scgcqei::R) reader structure"]
impl crate::Readable for SCGCQEI {}
#[doc = "`write(|w| ..)` method takes [scgcqei::W](scgcqei::W) writer structure"]
impl crate::Writable for SCGCQEI {}
#[doc = "Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
pub mod scgcqei;
#[doc = "EEPROM Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgceeprom](scgceeprom) module"]
pub type SCGCEEPROM = crate::Reg<u32, _SCGCEEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCEEPROM;
#[doc = "`read()` method returns [scgceeprom::R](scgceeprom::R) reader structure"]
impl crate::Readable for SCGCEEPROM {}
#[doc = "`write(|w| ..)` method takes [scgceeprom::W](scgceeprom::W) writer structure"]
impl crate::Writable for SCGCEEPROM {}
#[doc = "EEPROM Sleep Mode Clock Gating Control"]
pub mod scgceeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcwtimer](scgcwtimer) module"]
pub type SCGCWTIMER = crate::Reg<u32, _SCGCWTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCWTIMER;
#[doc = "`read()` method returns [scgcwtimer::R](scgcwtimer::R) reader structure"]
impl crate::Readable for SCGCWTIMER {}
#[doc = "`write(|w| ..)` method takes [scgcwtimer::W](scgcwtimer::W) writer structure"]
impl crate::Writable for SCGCWTIMER {}
#[doc = "32/64-Bit Wide General-Purpose Timer Sleep Mode Clock Gating Control"]
pub mod scgcwtimer;
#[doc = "Watchdog Timer Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcwd](dcgcwd) module"]
pub type DCGCWD = crate::Reg<u32, _DCGCWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCWD;
#[doc = "`read()` method returns [dcgcwd::R](dcgcwd::R) reader structure"]
impl crate::Readable for DCGCWD {}
#[doc = "`write(|w| ..)` method takes [dcgcwd::W](dcgcwd::W) writer structure"]
impl crate::Writable for DCGCWD {}
#[doc = "Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcwd;
#[doc = "16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgctimer](dcgctimer) module"]
pub type DCGCTIMER = crate::Reg<u32, _DCGCTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCTIMER;
#[doc = "`read()` method returns [dcgctimer::R](dcgctimer::R) reader structure"]
impl crate::Readable for DCGCTIMER {}
#[doc = "`write(|w| ..)` method takes [dcgctimer::W](dcgctimer::W) writer structure"]
impl crate::Writable for DCGCTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgctimer;
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcgpio](dcgcgpio) module"]
pub type DCGCGPIO = crate::Reg<u32, _DCGCGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCGPIO;
#[doc = "`read()` method returns [dcgcgpio::R](dcgcgpio::R) reader structure"]
impl crate::Readable for DCGCGPIO {}
#[doc = "`write(|w| ..)` method takes [dcgcgpio::W](dcgcgpio::W) writer structure"]
impl crate::Writable for DCGCGPIO {}
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcgpio;
#[doc = "Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcdma](dcgcdma) module"]
pub type DCGCDMA = crate::Reg<u32, _DCGCDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCDMA;
#[doc = "`read()` method returns [dcgcdma::R](dcgcdma::R) reader structure"]
impl crate::Readable for DCGCDMA {}
#[doc = "`write(|w| ..)` method takes [dcgcdma::W](dcgcdma::W) writer structure"]
impl crate::Writable for DCGCDMA {}
#[doc = "Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcdma;
#[doc = "Hibernation Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgchib](dcgchib) module"]
pub type DCGCHIB = crate::Reg<u32, _DCGCHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCHIB;
#[doc = "`read()` method returns [dcgchib::R](dcgchib::R) reader structure"]
impl crate::Readable for DCGCHIB {}
#[doc = "`write(|w| ..)` method takes [dcgchib::W](dcgchib::W) writer structure"]
impl crate::Writable for DCGCHIB {}
#[doc = "Hibernation Deep-Sleep Mode Clock Gating Control"]
pub mod dcgchib;
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcuart](dcgcuart) module"]
pub type DCGCUART = crate::Reg<u32, _DCGCUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCUART;
#[doc = "`read()` method returns [dcgcuart::R](dcgcuart::R) reader structure"]
impl crate::Readable for DCGCUART {}
#[doc = "`write(|w| ..)` method takes [dcgcuart::W](dcgcuart::W) writer structure"]
impl crate::Writable for DCGCUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcuart;
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcssi](dcgcssi) module"]
pub type DCGCSSI = crate::Reg<u32, _DCGCSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCSSI;
#[doc = "`read()` method returns [dcgcssi::R](dcgcssi::R) reader structure"]
impl crate::Readable for DCGCSSI {}
#[doc = "`write(|w| ..)` method takes [dcgcssi::W](dcgcssi::W) writer structure"]
impl crate::Writable for DCGCSSI {}
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcssi;
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgci2c](dcgci2c) module"]
pub type DCGCI2C = crate::Reg<u32, _DCGCI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCI2C;
#[doc = "`read()` method returns [dcgci2c::R](dcgci2c::R) reader structure"]
impl crate::Readable for DCGCI2C {}
#[doc = "`write(|w| ..)` method takes [dcgci2c::W](dcgci2c::W) writer structure"]
impl crate::Writable for DCGCI2C {}
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
pub mod dcgci2c;
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcusb](dcgcusb) module"]
pub type DCGCUSB = crate::Reg<u32, _DCGCUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCUSB;
#[doc = "`read()` method returns [dcgcusb::R](dcgcusb::R) reader structure"]
impl crate::Readable for DCGCUSB {}
#[doc = "`write(|w| ..)` method takes [dcgcusb::W](dcgcusb::W) writer structure"]
impl crate::Writable for DCGCUSB {}
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcusb;
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgccan](dcgccan) module"]
pub type DCGCCAN = crate::Reg<u32, _DCGCCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCCAN;
#[doc = "`read()` method returns [dcgccan::R](dcgccan::R) reader structure"]
impl crate::Readable for DCGCCAN {}
#[doc = "`write(|w| ..)` method takes [dcgccan::W](dcgccan::W) writer structure"]
impl crate::Writable for DCGCCAN {}
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control"]
pub mod dcgccan;
#[doc = "Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcadc](dcgcadc) module"]
pub type DCGCADC = crate::Reg<u32, _DCGCADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCADC;
#[doc = "`read()` method returns [dcgcadc::R](dcgcadc::R) reader structure"]
impl crate::Readable for DCGCADC {}
#[doc = "`write(|w| ..)` method takes [dcgcadc::W](dcgcadc::W) writer structure"]
impl crate::Writable for DCGCADC {}
#[doc = "Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcadc;
#[doc = "Analog Comparator Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcacmp](dcgcacmp) module"]
pub type DCGCACMP = crate::Reg<u32, _DCGCACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCACMP;
#[doc = "`read()` method returns [dcgcacmp::R](dcgcacmp::R) reader structure"]
impl crate::Readable for DCGCACMP {}
#[doc = "`write(|w| ..)` method takes [dcgcacmp::W](dcgcacmp::W) writer structure"]
impl crate::Writable for DCGCACMP {}
#[doc = "Analog Comparator Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcacmp;
#[doc = "Pulse Width Modulator Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcpwm](dcgcpwm) module"]
pub type DCGCPWM = crate::Reg<u32, _DCGCPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCPWM;
#[doc = "`read()` method returns [dcgcpwm::R](dcgcpwm::R) reader structure"]
impl crate::Readable for DCGCPWM {}
#[doc = "`write(|w| ..)` method takes [dcgcpwm::W](dcgcpwm::W) writer structure"]
impl crate::Writable for DCGCPWM {}
#[doc = "Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcpwm;
#[doc = "Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcqei](dcgcqei) module"]
pub type DCGCQEI = crate::Reg<u32, _DCGCQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCQEI;
#[doc = "`read()` method returns [dcgcqei::R](dcgcqei::R) reader structure"]
impl crate::Readable for DCGCQEI {}
#[doc = "`write(|w| ..)` method takes [dcgcqei::W](dcgcqei::W) writer structure"]
impl crate::Writable for DCGCQEI {}
#[doc = "Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcqei;
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgceeprom](dcgceeprom) module"]
pub type DCGCEEPROM = crate::Reg<u32, _DCGCEEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCEEPROM;
#[doc = "`read()` method returns [dcgceeprom::R](dcgceeprom::R) reader structure"]
impl crate::Readable for DCGCEEPROM {}
#[doc = "`write(|w| ..)` method takes [dcgceeprom::W](dcgceeprom::W) writer structure"]
impl crate::Writable for DCGCEEPROM {}
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control"]
pub mod dcgceeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcwtimer](dcgcwtimer) module"]
pub type DCGCWTIMER = crate::Reg<u32, _DCGCWTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCWTIMER;
#[doc = "`read()` method returns [dcgcwtimer::R](dcgcwtimer::R) reader structure"]
impl crate::Readable for DCGCWTIMER {}
#[doc = "`write(|w| ..)` method takes [dcgcwtimer::W](dcgcwtimer::W) writer structure"]
impl crate::Writable for DCGCWTIMER {}
#[doc = "32/64-Bit Wide General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcwtimer;
#[doc = "Watchdog Timer Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prwd](prwd) module"]
pub type PRWD = crate::Reg<u32, _PRWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRWD;
#[doc = "`read()` method returns [prwd::R](prwd::R) reader structure"]
impl crate::Readable for PRWD {}
#[doc = "Watchdog Timer Peripheral Ready"]
pub mod prwd;
#[doc = "16/32-Bit General-Purpose Timer Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prtimer](prtimer) module"]
pub type PRTIMER = crate::Reg<u32, _PRTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRTIMER;
#[doc = "`read()` method returns [prtimer::R](prtimer::R) reader structure"]
impl crate::Readable for PRTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Peripheral Ready"]
pub mod prtimer;
#[doc = "General-Purpose Input/Output Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prgpio](prgpio) module"]
pub type PRGPIO = crate::Reg<u32, _PRGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRGPIO;
#[doc = "`read()` method returns [prgpio::R](prgpio::R) reader structure"]
impl crate::Readable for PRGPIO {}
#[doc = "General-Purpose Input/Output Peripheral Ready"]
pub mod prgpio;
#[doc = "Micro Direct Memory Access Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prdma](prdma) module"]
pub type PRDMA = crate::Reg<u32, _PRDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRDMA;
#[doc = "`read()` method returns [prdma::R](prdma::R) reader structure"]
impl crate::Readable for PRDMA {}
#[doc = "Micro Direct Memory Access Peripheral Ready"]
pub mod prdma;
#[doc = "Hibernation Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prhib](prhib) module"]
pub type PRHIB = crate::Reg<u32, _PRHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRHIB;
#[doc = "`read()` method returns [prhib::R](prhib::R) reader structure"]
impl crate::Readable for PRHIB {}
#[doc = "Hibernation Peripheral Ready"]
pub mod prhib;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pruart](pruart) module"]
pub type PRUART = crate::Reg<u32, _PRUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRUART;
#[doc = "`read()` method returns [pruart::R](pruart::R) reader structure"]
impl crate::Readable for PRUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
pub mod pruart;
#[doc = "Synchronous Serial Interface Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prssi](prssi) module"]
pub type PRSSI = crate::Reg<u32, _PRSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSSI;
#[doc = "`read()` method returns [prssi::R](prssi::R) reader structure"]
impl crate::Readable for PRSSI {}
#[doc = "Synchronous Serial Interface Peripheral Ready"]
pub mod prssi;
#[doc = "Inter-Integrated Circuit Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pri2c](pri2c) module"]
pub type PRI2C = crate::Reg<u32, _PRI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRI2C;
#[doc = "`read()` method returns [pri2c::R](pri2c::R) reader structure"]
impl crate::Readable for PRI2C {}
#[doc = "Inter-Integrated Circuit Peripheral Ready"]
pub mod pri2c;
#[doc = "Universal Serial Bus Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prusb](prusb) module"]
pub type PRUSB = crate::Reg<u32, _PRUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRUSB;
#[doc = "`read()` method returns [prusb::R](prusb::R) reader structure"]
impl crate::Readable for PRUSB {}
#[doc = "Universal Serial Bus Peripheral Ready"]
pub mod prusb;
#[doc = "Controller Area Network Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prcan](prcan) module"]
pub type PRCAN = crate::Reg<u32, _PRCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRCAN;
#[doc = "`read()` method returns [prcan::R](prcan::R) reader structure"]
impl crate::Readable for PRCAN {}
#[doc = "Controller Area Network Peripheral Ready"]
pub mod prcan;
#[doc = "Analog-to-Digital Converter Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pradc](pradc) module"]
pub type PRADC = crate::Reg<u32, _PRADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRADC;
#[doc = "`read()` method returns [pradc::R](pradc::R) reader structure"]
impl crate::Readable for PRADC {}
#[doc = "Analog-to-Digital Converter Peripheral Ready"]
pub mod pradc;
#[doc = "Analog Comparator Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pracmp](pracmp) module"]
pub type PRACMP = crate::Reg<u32, _PRACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRACMP;
#[doc = "`read()` method returns [pracmp::R](pracmp::R) reader structure"]
impl crate::Readable for PRACMP {}
#[doc = "Analog Comparator Peripheral Ready"]
pub mod pracmp;
#[doc = "Pulse Width Modulator Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prpwm](prpwm) module"]
pub type PRPWM = crate::Reg<u32, _PRPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRPWM;
#[doc = "`read()` method returns [prpwm::R](prpwm::R) reader structure"]
impl crate::Readable for PRPWM {}
#[doc = "Pulse Width Modulator Peripheral Ready"]
pub mod prpwm;
#[doc = "Quadrature Encoder Interface Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prqei](prqei) module"]
pub type PRQEI = crate::Reg<u32, _PRQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRQEI;
#[doc = "`read()` method returns [prqei::R](prqei::R) reader structure"]
impl crate::Readable for PRQEI {}
#[doc = "Quadrature Encoder Interface Peripheral Ready"]
pub mod prqei;
#[doc = "EEPROM Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [preeprom](preeprom) module"]
pub type PREEPROM = crate::Reg<u32, _PREEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREEPROM;
#[doc = "`read()` method returns [preeprom::R](preeprom::R) reader structure"]
impl crate::Readable for PREEPROM {}
#[doc = "EEPROM Peripheral Ready"]
pub mod preeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prwtimer](prwtimer) module"]
pub type PRWTIMER = crate::Reg<u32, _PRWTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRWTIMER;
#[doc = "`read()` method returns [prwtimer::R](prwtimer::R) reader structure"]
impl crate::Readable for PRWTIMER {}
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Ready"]
pub mod prwtimer;
