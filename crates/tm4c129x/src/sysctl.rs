#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Identification 0"]
    pub did0: DID0,
    #[doc = "0x04 - Device Identification 1"]
    pub did1: DID1,
    _reserved2: [u8; 48usize],
    #[doc = "0x38 - Power-Temp Brown Out Control"]
    pub ptboctl: PTBOCTL,
    _reserved3: [u8; 20usize],
    #[doc = "0x50 - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x54 - Interrupt Mask Control"]
    pub imc: IMC,
    #[doc = "0x58 - Masked Interrupt Status and Clear"]
    pub misc: MISC,
    #[doc = "0x5c - Reset Cause"]
    pub resc: RESC,
    #[doc = "0x60 - Power-Temperature Cause"]
    pub pwrtc: PWRTC,
    #[doc = "0x64 - NMI Cause Register"]
    pub nmic: NMIC,
    _reserved9: [u8; 20usize],
    #[doc = "0x7c - Main Oscillator Control"]
    pub moscctl: MOSCCTL,
    _reserved10: [u8; 48usize],
    #[doc = "0xb0 - Run and Sleep Mode Configuration Register"]
    pub rsclkcfg: RSCLKCFG,
    _reserved11: [u8; 12usize],
    #[doc = "0xc0 - Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
    pub memtim0: MEMTIM0,
    _reserved12: [u8; 116usize],
    #[doc = "0x138 - Alternate Clock Configuration"]
    pub altclkcfg: ALTCLKCFG,
    _reserved13: [u8; 8usize],
    #[doc = "0x144 - Deep Sleep Clock Configuration Register"]
    pub dsclkcfg: DSCLKCFG,
    #[doc = "0x148 - Divisor and Source Clock Configuration"]
    pub divsclk: DIVSCLK,
    #[doc = "0x14c - System Properties"]
    pub sysprop: SYSPROP,
    #[doc = "0x150 - Precision Internal Oscillator Calibration"]
    pub piosccal: PIOSCCAL,
    #[doc = "0x154 - Precision Internal Oscillator Statistics"]
    pub pioscstat: PIOSCSTAT,
    _reserved18: [u8; 8usize],
    #[doc = "0x160 - PLL Frequency 0"]
    pub pllfreq0: PLLFREQ0,
    #[doc = "0x164 - PLL Frequency 1"]
    pub pllfreq1: PLLFREQ1,
    #[doc = "0x168 - PLL Status"]
    pub pllstat: PLLSTAT,
    _reserved21: [u8; 28usize],
    #[doc = "0x188 - Sleep Power Configuration"]
    pub slppwrcfg: SLPPWRCFG,
    #[doc = "0x18c - Deep-Sleep Power Configuration"]
    pub dslppwrcfg: DSLPPWRCFG,
    _reserved23: [u8; 16usize],
    #[doc = "0x1a0 - Non-Volatile Memory Information"]
    pub nvmstat: NVMSTAT,
    _reserved24: [u8; 16usize],
    #[doc = "0x1b4 - LDO Sleep Power Control"]
    pub ldospctl: LDOSPCTL,
    _reserved25: [u8; 4usize],
    #[doc = "0x1bc - LDO Deep-Sleep Power Control"]
    pub ldodpctl: LDODPCTL,
    _reserved26: [u8; 24usize],
    #[doc = "0x1d8 - Reset Behavior Control Register"]
    pub resbehavctl: RESBEHAVCTL,
    _reserved27: [u8; 24usize],
    #[doc = "0x1f4 - Hardware System Service Request"]
    pub hssr: HSSR,
    _reserved28: [u8; 136usize],
    #[doc = "0x280 - USB Power Domain Status"]
    pub usbpds: USBPDS,
    #[doc = "0x284 - USB Memory Power Control"]
    pub usbmpc: USBMPC,
    #[doc = "0x288 - Ethernet MAC Power Domain Status"]
    pub emacpds: EMACPDS,
    #[doc = "0x28c - Ethernet MAC Memory Power Control"]
    pub emacmpc: EMACMPC,
    _reserved32: [u8; 112usize],
    #[doc = "0x300 - Watchdog Timer Peripheral Present"]
    pub ppwd: PPWD,
    #[doc = "0x304 - 16/32-Bit General-Purpose Timer Peripheral Present"]
    pub pptimer: PPTIMER,
    #[doc = "0x308 - General-Purpose Input/Output Peripheral Present"]
    pub ppgpio: PPGPIO,
    #[doc = "0x30c - Micro Direct Memory Access Peripheral Present"]
    pub ppdma: PPDMA,
    #[doc = "0x310 - EPI Peripheral Present"]
    pub ppepi: PPEPI,
    #[doc = "0x314 - Hibernation Peripheral Present"]
    pub pphib: PPHIB,
    #[doc = "0x318 - Universal Asynchronous Receiver/Transmitter Peripheral Present"]
    pub ppuart: PPUART,
    #[doc = "0x31c - Synchronous Serial Interface Peripheral Present"]
    pub ppssi: PPSSI,
    #[doc = "0x320 - Inter-Integrated Circuit Peripheral Present"]
    pub ppi2c: PPI2C,
    _reserved41: [u8; 4usize],
    #[doc = "0x328 - Universal Serial Bus Peripheral Present"]
    pub ppusb: PPUSB,
    _reserved42: [u8; 4usize],
    #[doc = "0x330 - Ethernet PHY Peripheral Present"]
    pub ppephy: PPEPHY,
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
    #[doc = "0x348 - Low Pin Count Interface Peripheral Present"]
    pub pplpc: PPLPC,
    _reserved49: [u8; 4usize],
    #[doc = "0x350 - Platform Environment Control Interface Peripheral Present"]
    pub pppeci: PPPECI,
    #[doc = "0x354 - Fan Control Peripheral Present"]
    pub ppfan: PPFAN,
    #[doc = "0x358 - EEPROM Peripheral Present"]
    pub ppeeprom: PPEEPROM,
    #[doc = "0x35c - 32/64-Bit Wide General-Purpose Timer Peripheral Present"]
    pub ppwtimer: PPWTIMER,
    _reserved53: [u8; 16usize],
    #[doc = "0x370 - Remote Temperature Sensor Peripheral Present"]
    pub pprts: PPRTS,
    #[doc = "0x374 - CRC and Cryptographic Modules Peripheral Present"]
    pub ppccm: PPCCM,
    _reserved55: [u8; 24usize],
    #[doc = "0x390 - LCD Peripheral Present"]
    pub pplcd: PPLCD,
    _reserved56: [u8; 4usize],
    #[doc = "0x398 - 1-Wire Peripheral Present"]
    pub ppowire: PPOWIRE,
    #[doc = "0x39c - Ethernet MAC Peripheral Present"]
    pub ppemac: PPEMAC,
    _reserved58: [u8; 4usize],
    #[doc = "0x3a4 - Human Interface Master Peripheral Present"]
    pub pphim: PPHIM,
    _reserved59: [u8; 344usize],
    #[doc = "0x500 - Watchdog Timer Software Reset"]
    pub srwd: SRWD,
    #[doc = "0x504 - 16/32-Bit General-Purpose Timer Software Reset"]
    pub srtimer: SRTIMER,
    #[doc = "0x508 - General-Purpose Input/Output Software Reset"]
    pub srgpio: SRGPIO,
    #[doc = "0x50c - Micro Direct Memory Access Software Reset"]
    pub srdma: SRDMA,
    #[doc = "0x510 - EPI Software Reset"]
    pub srepi: SREPI,
    #[doc = "0x514 - Hibernation Software Reset"]
    pub srhib: SRHIB,
    #[doc = "0x518 - Universal Asynchronous Receiver/Transmitter Software Reset"]
    pub sruart: SRUART,
    #[doc = "0x51c - Synchronous Serial Interface Software Reset"]
    pub srssi: SRSSI,
    #[doc = "0x520 - Inter-Integrated Circuit Software Reset"]
    pub sri2c: SRI2C,
    _reserved68: [u8; 4usize],
    #[doc = "0x528 - Universal Serial Bus Software Reset"]
    pub srusb: SRUSB,
    _reserved69: [u8; 4usize],
    #[doc = "0x530 - Ethernet PHY Software Reset"]
    pub srephy: SREPHY,
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
    _reserved76: [u8; 24usize],
    #[doc = "0x574 - CRC and Cryptographic Modules Software Reset"]
    pub srccm: SRCCM,
    _reserved77: [u8; 36usize],
    #[doc = "0x59c - Ethernet MAC Software Reset"]
    pub sremac: SREMAC,
    _reserved78: [u8; 96usize],
    #[doc = "0x600 - Watchdog Timer Run Mode Clock Gating Control"]
    pub rcgcwd: RCGCWD,
    #[doc = "0x604 - 16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
    pub rcgctimer: RCGCTIMER,
    #[doc = "0x608 - General-Purpose Input/Output Run Mode Clock Gating Control"]
    pub rcgcgpio: RCGCGPIO,
    #[doc = "0x60c - Micro Direct Memory Access Run Mode Clock Gating Control"]
    pub rcgcdma: RCGCDMA,
    #[doc = "0x610 - EPI Run Mode Clock Gating Control"]
    pub rcgcepi: RCGCEPI,
    #[doc = "0x614 - Hibernation Run Mode Clock Gating Control"]
    pub rcgchib: RCGCHIB,
    #[doc = "0x618 - Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
    pub rcgcuart: RCGCUART,
    #[doc = "0x61c - Synchronous Serial Interface Run Mode Clock Gating Control"]
    pub rcgcssi: RCGCSSI,
    #[doc = "0x620 - Inter-Integrated Circuit Run Mode Clock Gating Control"]
    pub rcgci2c: RCGCI2C,
    _reserved87: [u8; 4usize],
    #[doc = "0x628 - Universal Serial Bus Run Mode Clock Gating Control"]
    pub rcgcusb: RCGCUSB,
    _reserved88: [u8; 4usize],
    #[doc = "0x630 - Ethernet PHY Run Mode Clock Gating Control"]
    pub rcgcephy: RCGCEPHY,
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
    _reserved94: [u8; 16usize],
    #[doc = "0x658 - EEPROM Run Mode Clock Gating Control"]
    pub rcgceeprom: RCGCEEPROM,
    _reserved95: [u8; 24usize],
    #[doc = "0x674 - CRC and Cryptographic Modules Run Mode Clock Gating Control"]
    pub rcgcccm: RCGCCCM,
    _reserved96: [u8; 36usize],
    #[doc = "0x69c - Ethernet MAC Run Mode Clock Gating Control"]
    pub rcgcemac: RCGCEMAC,
    _reserved97: [u8; 96usize],
    #[doc = "0x700 - Watchdog Timer Sleep Mode Clock Gating Control"]
    pub scgcwd: SCGCWD,
    #[doc = "0x704 - 16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
    pub scgctimer: SCGCTIMER,
    #[doc = "0x708 - General-Purpose Input/Output Sleep Mode Clock Gating Control"]
    pub scgcgpio: SCGCGPIO,
    #[doc = "0x70c - Micro Direct Memory Access Sleep Mode Clock Gating Control"]
    pub scgcdma: SCGCDMA,
    #[doc = "0x710 - EPI Sleep Mode Clock Gating Control"]
    pub scgcepi: SCGCEPI,
    #[doc = "0x714 - Hibernation Sleep Mode Clock Gating Control"]
    pub scgchib: SCGCHIB,
    #[doc = "0x718 - Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
    pub scgcuart: SCGCUART,
    #[doc = "0x71c - Synchronous Serial Interface Sleep Mode Clock Gating Control"]
    pub scgcssi: SCGCSSI,
    #[doc = "0x720 - Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
    pub scgci2c: SCGCI2C,
    _reserved106: [u8; 4usize],
    #[doc = "0x728 - Universal Serial Bus Sleep Mode Clock Gating Control"]
    pub scgcusb: SCGCUSB,
    _reserved107: [u8; 4usize],
    #[doc = "0x730 - Ethernet PHY Sleep Mode Clock Gating Control"]
    pub scgcephy: SCGCEPHY,
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
    _reserved113: [u8; 16usize],
    #[doc = "0x758 - EEPROM Sleep Mode Clock Gating Control"]
    pub scgceeprom: SCGCEEPROM,
    _reserved114: [u8; 24usize],
    #[doc = "0x774 - CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
    pub scgcccm: SCGCCCM,
    _reserved115: [u8; 36usize],
    #[doc = "0x79c - Ethernet MAC Sleep Mode Clock Gating Control"]
    pub scgcemac: SCGCEMAC,
    _reserved116: [u8; 96usize],
    #[doc = "0x800 - Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgcwd: DCGCWD,
    #[doc = "0x804 - 16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgctimer: DCGCTIMER,
    #[doc = "0x808 - General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
    pub dcgcgpio: DCGCGPIO,
    #[doc = "0x80c - Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
    pub dcgcdma: DCGCDMA,
    #[doc = "0x810 - EPI Deep-Sleep Mode Clock Gating Control"]
    pub dcgcepi: DCGCEPI,
    #[doc = "0x814 - Hibernation Deep-Sleep Mode Clock Gating Control"]
    pub dcgchib: DCGCHIB,
    #[doc = "0x818 - Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
    pub dcgcuart: DCGCUART,
    #[doc = "0x81c - Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
    pub dcgcssi: DCGCSSI,
    #[doc = "0x820 - Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
    pub dcgci2c: DCGCI2C,
    _reserved125: [u8; 4usize],
    #[doc = "0x828 - Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
    pub dcgcusb: DCGCUSB,
    _reserved126: [u8; 4usize],
    #[doc = "0x830 - Ethernet PHY Deep-Sleep Mode Clock Gating Control"]
    pub dcgcephy: DCGCEPHY,
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
    _reserved132: [u8; 16usize],
    #[doc = "0x858 - EEPROM Deep-Sleep Mode Clock Gating Control"]
    pub dcgceeprom: DCGCEEPROM,
    _reserved133: [u8; 24usize],
    #[doc = "0x874 - CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
    pub dcgcccm: DCGCCCM,
    _reserved134: [u8; 36usize],
    #[doc = "0x89c - Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
    pub dcgcemac: DCGCEMAC,
    _reserved135: [u8; 96usize],
    #[doc = "0x900 - Watchdog Timer Power Control"]
    pub pcwd: PCWD,
    #[doc = "0x904 - 16/32-Bit General-Purpose Timer Power Control"]
    pub pctimer: PCTIMER,
    #[doc = "0x908 - General-Purpose Input/Output Power Control"]
    pub pcgpio: PCGPIO,
    #[doc = "0x90c - Micro Direct Memory Access Power Control"]
    pub pcdma: PCDMA,
    #[doc = "0x910 - External Peripheral Interface Power Control"]
    pub pcepi: PCEPI,
    #[doc = "0x914 - Hibernation Power Control"]
    pub pchib: PCHIB,
    #[doc = "0x918 - Universal Asynchronous Receiver/Transmitter Power Control"]
    pub pcuart: PCUART,
    #[doc = "0x91c - Synchronous Serial Interface Power Control"]
    pub pcssi: PCSSI,
    #[doc = "0x920 - Inter-Integrated Circuit Power Control"]
    pub pci2c: PCI2C,
    _reserved144: [u8; 4usize],
    #[doc = "0x928 - Universal Serial Bus Power Control"]
    pub pcusb: PCUSB,
    _reserved145: [u8; 4usize],
    #[doc = "0x930 - Ethernet PHY Power Control"]
    pub pcephy: PCEPHY,
    #[doc = "0x934 - Controller Area Network Power Control"]
    pub pccan: PCCAN,
    #[doc = "0x938 - Analog-to-Digital Converter Power Control"]
    pub pcadc: PCADC,
    #[doc = "0x93c - Analog Comparator Power Control"]
    pub pcacmp: PCACMP,
    #[doc = "0x940 - Pulse Width Modulator Power Control"]
    pub pcpwm: PCPWM,
    #[doc = "0x944 - Quadrature Encoder Interface Power Control"]
    pub pcqei: PCQEI,
    _reserved151: [u8; 16usize],
    #[doc = "0x958 - EEPROM Power Control"]
    pub pceeprom: PCEEPROM,
    _reserved152: [u8; 24usize],
    #[doc = "0x974 - CRC and Cryptographic Modules Power Control"]
    pub pcccm: PCCCM,
    _reserved153: [u8; 36usize],
    #[doc = "0x99c - Ethernet MAC Power Control"]
    pub pcemac: PCEMAC,
    _reserved154: [u8; 96usize],
    #[doc = "0xa00 - Watchdog Timer Peripheral Ready"]
    pub prwd: PRWD,
    #[doc = "0xa04 - 16/32-Bit General-Purpose Timer Peripheral Ready"]
    pub prtimer: PRTIMER,
    #[doc = "0xa08 - General-Purpose Input/Output Peripheral Ready"]
    pub prgpio: PRGPIO,
    #[doc = "0xa0c - Micro Direct Memory Access Peripheral Ready"]
    pub prdma: PRDMA,
    #[doc = "0xa10 - EPI Peripheral Ready"]
    pub prepi: PREPI,
    #[doc = "0xa14 - Hibernation Peripheral Ready"]
    pub prhib: PRHIB,
    #[doc = "0xa18 - Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
    pub pruart: PRUART,
    #[doc = "0xa1c - Synchronous Serial Interface Peripheral Ready"]
    pub prssi: PRSSI,
    #[doc = "0xa20 - Inter-Integrated Circuit Peripheral Ready"]
    pub pri2c: PRI2C,
    _reserved163: [u8; 4usize],
    #[doc = "0xa28 - Universal Serial Bus Peripheral Ready"]
    pub prusb: PRUSB,
    _reserved164: [u8; 4usize],
    #[doc = "0xa30 - Ethernet PHY Peripheral Ready"]
    pub prephy: PREPHY,
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
    _reserved170: [u8; 16usize],
    #[doc = "0xa58 - EEPROM Peripheral Ready"]
    pub preeprom: PREEPROM,
    _reserved171: [u8; 24usize],
    #[doc = "0xa74 - CRC and Cryptographic Modules Peripheral Ready"]
    pub prccm: PRCCM,
    _reserved172: [u8; 36usize],
    #[doc = "0xa9c - Ethernet MAC Peripheral Ready"]
    pub premac: PREMAC,
}
#[doc = "Device Identification 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did0](did0) module"]
pub type DID0 = crate::Reg<u32, _DID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DID0;
#[doc = "`read()` method returns [did0::R](did0::R) reader structure"]
impl crate::Readable for DID0 {}
#[doc = "Device Identification 0"]
pub mod did0;
#[doc = "Device Identification 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did1](did1) module"]
pub type DID1 = crate::Reg<u32, _DID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DID1;
#[doc = "`read()` method returns [did1::R](did1::R) reader structure"]
impl crate::Readable for DID1 {}
#[doc = "Device Identification 1"]
pub mod did1;
#[doc = "Power-Temp Brown Out Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptboctl](ptboctl) module"]
pub type PTBOCTL = crate::Reg<u32, _PTBOCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTBOCTL;
#[doc = "`read()` method returns [ptboctl::R](ptboctl::R) reader structure"]
impl crate::Readable for PTBOCTL {}
#[doc = "`write(|w| ..)` method takes [ptboctl::W](ptboctl::W) writer structure"]
impl crate::Writable for PTBOCTL {}
#[doc = "Power-Temp Brown Out Control"]
pub mod ptboctl;
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "Interrupt Mask Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imc](imc) module"]
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
#[doc = "Masked Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](misc) module"]
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
#[doc = "Reset Cause\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resc](resc) module"]
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
#[doc = "Power-Temperature Cause\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrtc](pwrtc) module"]
pub type PWRTC = crate::Reg<u32, _PWRTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRTC;
#[doc = "`read()` method returns [pwrtc::R](pwrtc::R) reader structure"]
impl crate::Readable for PWRTC {}
#[doc = "`write(|w| ..)` method takes [pwrtc::W](pwrtc::W) writer structure"]
impl crate::Writable for PWRTC {}
#[doc = "Power-Temperature Cause"]
pub mod pwrtc;
#[doc = "NMI Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmic](nmic) module"]
pub type NMIC = crate::Reg<u32, _NMIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMIC;
#[doc = "`read()` method returns [nmic::R](nmic::R) reader structure"]
impl crate::Readable for NMIC {}
#[doc = "`write(|w| ..)` method takes [nmic::W](nmic::W) writer structure"]
impl crate::Writable for NMIC {}
#[doc = "NMI Cause Register"]
pub mod nmic;
#[doc = "Main Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moscctl](moscctl) module"]
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
#[doc = "Run and Sleep Mode Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsclkcfg](rsclkcfg) module"]
pub type RSCLKCFG = crate::Reg<u32, _RSCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSCLKCFG;
#[doc = "`read()` method returns [rsclkcfg::R](rsclkcfg::R) reader structure"]
impl crate::Readable for RSCLKCFG {}
#[doc = "`write(|w| ..)` method takes [rsclkcfg::W](rsclkcfg::W) writer structure"]
impl crate::Writable for RSCLKCFG {}
#[doc = "Run and Sleep Mode Configuration Register"]
pub mod rsclkcfg;
#[doc = "Memory Timing Parameter Register 0 for Main Flash and EEPROM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memtim0](memtim0) module"]
pub type MEMTIM0 = crate::Reg<u32, _MEMTIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMTIM0;
#[doc = "`read()` method returns [memtim0::R](memtim0::R) reader structure"]
impl crate::Readable for MEMTIM0 {}
#[doc = "`write(|w| ..)` method takes [memtim0::W](memtim0::W) writer structure"]
impl crate::Writable for MEMTIM0 {}
#[doc = "Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
pub mod memtim0;
#[doc = "Alternate Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altclkcfg](altclkcfg) module"]
pub type ALTCLKCFG = crate::Reg<u32, _ALTCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTCLKCFG;
#[doc = "`read()` method returns [altclkcfg::R](altclkcfg::R) reader structure"]
impl crate::Readable for ALTCLKCFG {}
#[doc = "`write(|w| ..)` method takes [altclkcfg::W](altclkcfg::W) writer structure"]
impl crate::Writable for ALTCLKCFG {}
#[doc = "Alternate Clock Configuration"]
pub mod altclkcfg;
#[doc = "Deep Sleep Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsclkcfg](dsclkcfg) module"]
pub type DSCLKCFG = crate::Reg<u32, _DSCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCLKCFG;
#[doc = "`read()` method returns [dsclkcfg::R](dsclkcfg::R) reader structure"]
impl crate::Readable for DSCLKCFG {}
#[doc = "`write(|w| ..)` method takes [dsclkcfg::W](dsclkcfg::W) writer structure"]
impl crate::Writable for DSCLKCFG {}
#[doc = "Deep Sleep Clock Configuration Register"]
pub mod dsclkcfg;
#[doc = "Divisor and Source Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divsclk](divsclk) module"]
pub type DIVSCLK = crate::Reg<u32, _DIVSCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVSCLK;
#[doc = "`read()` method returns [divsclk::R](divsclk::R) reader structure"]
impl crate::Readable for DIVSCLK {}
#[doc = "`write(|w| ..)` method takes [divsclk::W](divsclk::W) writer structure"]
impl crate::Writable for DIVSCLK {}
#[doc = "Divisor and Source Clock Configuration"]
pub mod divsclk;
#[doc = "System Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysprop](sysprop) module"]
pub type SYSPROP = crate::Reg<u32, _SYSPROP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPROP;
#[doc = "`read()` method returns [sysprop::R](sysprop::R) reader structure"]
impl crate::Readable for SYSPROP {}
#[doc = "System Properties"]
pub mod sysprop;
#[doc = "Precision Internal Oscillator Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [piosccal](piosccal) module"]
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
#[doc = "Precision Internal Oscillator Statistics\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pioscstat](pioscstat) module"]
pub type PIOSCSTAT = crate::Reg<u32, _PIOSCSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIOSCSTAT;
#[doc = "`read()` method returns [pioscstat::R](pioscstat::R) reader structure"]
impl crate::Readable for PIOSCSTAT {}
#[doc = "Precision Internal Oscillator Statistics"]
pub mod pioscstat;
#[doc = "PLL Frequency 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllfreq0](pllfreq0) module"]
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
#[doc = "PLL Frequency 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllfreq1](pllfreq1) module"]
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
#[doc = "PLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllstat](pllstat) module"]
pub type PLLSTAT = crate::Reg<u32, _PLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSTAT;
#[doc = "`read()` method returns [pllstat::R](pllstat::R) reader structure"]
impl crate::Readable for PLLSTAT {}
#[doc = "PLL Status"]
pub mod pllstat;
#[doc = "Sleep Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slppwrcfg](slppwrcfg) module"]
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
#[doc = "Deep-Sleep Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dslppwrcfg](dslppwrcfg) module"]
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
#[doc = "Non-Volatile Memory Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmstat](nvmstat) module"]
pub type NVMSTAT = crate::Reg<u32, _NVMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMSTAT;
#[doc = "`read()` method returns [nvmstat::R](nvmstat::R) reader structure"]
impl crate::Readable for NVMSTAT {}
#[doc = "Non-Volatile Memory Information"]
pub mod nvmstat;
#[doc = "LDO Sleep Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldospctl](ldospctl) module"]
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
#[doc = "LDO Deep-Sleep Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldodpctl](ldodpctl) module"]
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
#[doc = "Reset Behavior Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resbehavctl](resbehavctl) module"]
pub type RESBEHAVCTL = crate::Reg<u32, _RESBEHAVCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESBEHAVCTL;
#[doc = "`read()` method returns [resbehavctl::R](resbehavctl::R) reader structure"]
impl crate::Readable for RESBEHAVCTL {}
#[doc = "`write(|w| ..)` method takes [resbehavctl::W](resbehavctl::W) writer structure"]
impl crate::Writable for RESBEHAVCTL {}
#[doc = "Reset Behavior Control Register"]
pub mod resbehavctl;
#[doc = "Hardware System Service Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hssr](hssr) module"]
pub type HSSR = crate::Reg<u32, _HSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSSR;
#[doc = "`read()` method returns [hssr::R](hssr::R) reader structure"]
impl crate::Readable for HSSR {}
#[doc = "`write(|w| ..)` method takes [hssr::W](hssr::W) writer structure"]
impl crate::Writable for HSSR {}
#[doc = "Hardware System Service Request"]
pub mod hssr;
#[doc = "USB Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpds](usbpds) module"]
pub type USBPDS = crate::Reg<u32, _USBPDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPDS;
#[doc = "`read()` method returns [usbpds::R](usbpds::R) reader structure"]
impl crate::Readable for USBPDS {}
#[doc = "USB Power Domain Status"]
pub mod usbpds;
#[doc = "USB Memory Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbmpc](usbmpc) module"]
pub type USBMPC = crate::Reg<u32, _USBMPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBMPC;
#[doc = "`read()` method returns [usbmpc::R](usbmpc::R) reader structure"]
impl crate::Readable for USBMPC {}
#[doc = "`write(|w| ..)` method takes [usbmpc::W](usbmpc::W) writer structure"]
impl crate::Writable for USBMPC {}
#[doc = "USB Memory Power Control"]
pub mod usbmpc;
#[doc = "Ethernet MAC Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emacpds](emacpds) module"]
pub type EMACPDS = crate::Reg<u32, _EMACPDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMACPDS;
#[doc = "`read()` method returns [emacpds::R](emacpds::R) reader structure"]
impl crate::Readable for EMACPDS {}
#[doc = "Ethernet MAC Power Domain Status"]
pub mod emacpds;
#[doc = "Ethernet MAC Memory Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emacmpc](emacmpc) module"]
pub type EMACMPC = crate::Reg<u32, _EMACMPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMACMPC;
#[doc = "`read()` method returns [emacmpc::R](emacmpc::R) reader structure"]
impl crate::Readable for EMACMPC {}
#[doc = "`write(|w| ..)` method takes [emacmpc::W](emacmpc::W) writer structure"]
impl crate::Writable for EMACMPC {}
#[doc = "Ethernet MAC Memory Power Control"]
pub mod emacmpc;
#[doc = "Watchdog Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppwd](ppwd) module"]
pub type PPWD = crate::Reg<u32, _PPWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPWD;
#[doc = "`read()` method returns [ppwd::R](ppwd::R) reader structure"]
impl crate::Readable for PPWD {}
#[doc = "Watchdog Timer Peripheral Present"]
pub mod ppwd;
#[doc = "16/32-Bit General-Purpose Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pptimer](pptimer) module"]
pub type PPTIMER = crate::Reg<u32, _PPTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPTIMER;
#[doc = "`read()` method returns [pptimer::R](pptimer::R) reader structure"]
impl crate::Readable for PPTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Peripheral Present"]
pub mod pptimer;
#[doc = "General-Purpose Input/Output Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppgpio](ppgpio) module"]
pub type PPGPIO = crate::Reg<u32, _PPGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPGPIO;
#[doc = "`read()` method returns [ppgpio::R](ppgpio::R) reader structure"]
impl crate::Readable for PPGPIO {}
#[doc = "General-Purpose Input/Output Peripheral Present"]
pub mod ppgpio;
#[doc = "Micro Direct Memory Access Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppdma](ppdma) module"]
pub type PPDMA = crate::Reg<u32, _PPDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPDMA;
#[doc = "`read()` method returns [ppdma::R](ppdma::R) reader structure"]
impl crate::Readable for PPDMA {}
#[doc = "Micro Direct Memory Access Peripheral Present"]
pub mod ppdma;
#[doc = "EPI Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppepi](ppepi) module"]
pub type PPEPI = crate::Reg<u32, _PPEPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPEPI;
#[doc = "`read()` method returns [ppepi::R](ppepi::R) reader structure"]
impl crate::Readable for PPEPI {}
#[doc = "EPI Peripheral Present"]
pub mod ppepi;
#[doc = "Hibernation Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pphib](pphib) module"]
pub type PPHIB = crate::Reg<u32, _PPHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPHIB;
#[doc = "`read()` method returns [pphib::R](pphib::R) reader structure"]
impl crate::Readable for PPHIB {}
#[doc = "Hibernation Peripheral Present"]
pub mod pphib;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppuart](ppuart) module"]
pub type PPUART = crate::Reg<u32, _PPUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUART;
#[doc = "`read()` method returns [ppuart::R](ppuart::R) reader structure"]
impl crate::Readable for PPUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present"]
pub mod ppuart;
#[doc = "Synchronous Serial Interface Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppssi](ppssi) module"]
pub type PPSSI = crate::Reg<u32, _PPSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPSSI;
#[doc = "`read()` method returns [ppssi::R](ppssi::R) reader structure"]
impl crate::Readable for PPSSI {}
#[doc = "Synchronous Serial Interface Peripheral Present"]
pub mod ppssi;
#[doc = "Inter-Integrated Circuit Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppi2c](ppi2c) module"]
pub type PPI2C = crate::Reg<u32, _PPI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPI2C;
#[doc = "`read()` method returns [ppi2c::R](ppi2c::R) reader structure"]
impl crate::Readable for PPI2C {}
#[doc = "Inter-Integrated Circuit Peripheral Present"]
pub mod ppi2c;
#[doc = "Universal Serial Bus Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppusb](ppusb) module"]
pub type PPUSB = crate::Reg<u32, _PPUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUSB;
#[doc = "`read()` method returns [ppusb::R](ppusb::R) reader structure"]
impl crate::Readable for PPUSB {}
#[doc = "Universal Serial Bus Peripheral Present"]
pub mod ppusb;
#[doc = "Ethernet PHY Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppephy](ppephy) module"]
pub type PPEPHY = crate::Reg<u32, _PPEPHY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPEPHY;
#[doc = "`read()` method returns [ppephy::R](ppephy::R) reader structure"]
impl crate::Readable for PPEPHY {}
#[doc = "Ethernet PHY Peripheral Present"]
pub mod ppephy;
#[doc = "Controller Area Network Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppcan](ppcan) module"]
pub type PPCAN = crate::Reg<u32, _PPCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPCAN;
#[doc = "`read()` method returns [ppcan::R](ppcan::R) reader structure"]
impl crate::Readable for PPCAN {}
#[doc = "Controller Area Network Peripheral Present"]
pub mod ppcan;
#[doc = "Analog-to-Digital Converter Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppadc](ppadc) module"]
pub type PPADC = crate::Reg<u32, _PPADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPADC;
#[doc = "`read()` method returns [ppadc::R](ppadc::R) reader structure"]
impl crate::Readable for PPADC {}
#[doc = "Analog-to-Digital Converter Peripheral Present"]
pub mod ppadc;
#[doc = "Analog Comparator Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppacmp](ppacmp) module"]
pub type PPACMP = crate::Reg<u32, _PPACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPACMP;
#[doc = "`read()` method returns [ppacmp::R](ppacmp::R) reader structure"]
impl crate::Readable for PPACMP {}
#[doc = "Analog Comparator Peripheral Present"]
pub mod ppacmp;
#[doc = "Pulse Width Modulator Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pppwm](pppwm) module"]
pub type PPPWM = crate::Reg<u32, _PPPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPPWM;
#[doc = "`read()` method returns [pppwm::R](pppwm::R) reader structure"]
impl crate::Readable for PPPWM {}
#[doc = "Pulse Width Modulator Peripheral Present"]
pub mod pppwm;
#[doc = "Quadrature Encoder Interface Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppqei](ppqei) module"]
pub type PPQEI = crate::Reg<u32, _PPQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPQEI;
#[doc = "`read()` method returns [ppqei::R](ppqei::R) reader structure"]
impl crate::Readable for PPQEI {}
#[doc = "Quadrature Encoder Interface Peripheral Present"]
pub mod ppqei;
#[doc = "Low Pin Count Interface Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pplpc](pplpc) module"]
pub type PPLPC = crate::Reg<u32, _PPLPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPLPC;
#[doc = "`read()` method returns [pplpc::R](pplpc::R) reader structure"]
impl crate::Readable for PPLPC {}
#[doc = "Low Pin Count Interface Peripheral Present"]
pub mod pplpc;
#[doc = "Platform Environment Control Interface Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pppeci](pppeci) module"]
pub type PPPECI = crate::Reg<u32, _PPPECI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPPECI;
#[doc = "`read()` method returns [pppeci::R](pppeci::R) reader structure"]
impl crate::Readable for PPPECI {}
#[doc = "Platform Environment Control Interface Peripheral Present"]
pub mod pppeci;
#[doc = "Fan Control Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppfan](ppfan) module"]
pub type PPFAN = crate::Reg<u32, _PPFAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPFAN;
#[doc = "`read()` method returns [ppfan::R](ppfan::R) reader structure"]
impl crate::Readable for PPFAN {}
#[doc = "Fan Control Peripheral Present"]
pub mod ppfan;
#[doc = "EEPROM Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppeeprom](ppeeprom) module"]
pub type PPEEPROM = crate::Reg<u32, _PPEEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPEEPROM;
#[doc = "`read()` method returns [ppeeprom::R](ppeeprom::R) reader structure"]
impl crate::Readable for PPEEPROM {}
#[doc = "EEPROM Peripheral Present"]
pub mod ppeeprom;
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppwtimer](ppwtimer) module"]
pub type PPWTIMER = crate::Reg<u32, _PPWTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPWTIMER;
#[doc = "`read()` method returns [ppwtimer::R](ppwtimer::R) reader structure"]
impl crate::Readable for PPWTIMER {}
#[doc = "32/64-Bit Wide General-Purpose Timer Peripheral Present"]
pub mod ppwtimer;
#[doc = "Remote Temperature Sensor Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pprts](pprts) module"]
pub type PPRTS = crate::Reg<u32, _PPRTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPRTS;
#[doc = "`read()` method returns [pprts::R](pprts::R) reader structure"]
impl crate::Readable for PPRTS {}
#[doc = "Remote Temperature Sensor Peripheral Present"]
pub mod pprts;
#[doc = "CRC and Cryptographic Modules Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppccm](ppccm) module"]
pub type PPCCM = crate::Reg<u32, _PPCCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPCCM;
#[doc = "`read()` method returns [ppccm::R](ppccm::R) reader structure"]
impl crate::Readable for PPCCM {}
#[doc = "CRC and Cryptographic Modules Peripheral Present"]
pub mod ppccm;
#[doc = "LCD Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pplcd](pplcd) module"]
pub type PPLCD = crate::Reg<u32, _PPLCD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPLCD;
#[doc = "`read()` method returns [pplcd::R](pplcd::R) reader structure"]
impl crate::Readable for PPLCD {}
#[doc = "LCD Peripheral Present"]
pub mod pplcd;
#[doc = "1-Wire Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppowire](ppowire) module"]
pub type PPOWIRE = crate::Reg<u32, _PPOWIRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPOWIRE;
#[doc = "`read()` method returns [ppowire::R](ppowire::R) reader structure"]
impl crate::Readable for PPOWIRE {}
#[doc = "1-Wire Peripheral Present"]
pub mod ppowire;
#[doc = "Ethernet MAC Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppemac](ppemac) module"]
pub type PPEMAC = crate::Reg<u32, _PPEMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPEMAC;
#[doc = "`read()` method returns [ppemac::R](ppemac::R) reader structure"]
impl crate::Readable for PPEMAC {}
#[doc = "Ethernet MAC Peripheral Present"]
pub mod ppemac;
#[doc = "Human Interface Master Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pphim](pphim) module"]
pub type PPHIM = crate::Reg<u32, _PPHIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPHIM;
#[doc = "`read()` method returns [pphim::R](pphim::R) reader structure"]
impl crate::Readable for PPHIM {}
#[doc = "Human Interface Master Peripheral Present"]
pub mod pphim;
#[doc = "Watchdog Timer Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srwd](srwd) module"]
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
#[doc = "16/32-Bit General-Purpose Timer Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srtimer](srtimer) module"]
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
#[doc = "General-Purpose Input/Output Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srgpio](srgpio) module"]
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
#[doc = "Micro Direct Memory Access Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdma](srdma) module"]
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
#[doc = "EPI Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srepi](srepi) module"]
pub type SREPI = crate::Reg<u32, _SREPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SREPI;
#[doc = "`read()` method returns [srepi::R](srepi::R) reader structure"]
impl crate::Readable for SREPI {}
#[doc = "`write(|w| ..)` method takes [srepi::W](srepi::W) writer structure"]
impl crate::Writable for SREPI {}
#[doc = "EPI Software Reset"]
pub mod srepi;
#[doc = "Hibernation Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srhib](srhib) module"]
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
#[doc = "Universal Asynchronous Receiver/Transmitter Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sruart](sruart) module"]
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
#[doc = "Synchronous Serial Interface Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srssi](srssi) module"]
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
#[doc = "Inter-Integrated Circuit Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sri2c](sri2c) module"]
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
#[doc = "Universal Serial Bus Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srusb](srusb) module"]
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
#[doc = "Ethernet PHY Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srephy](srephy) module"]
pub type SREPHY = crate::Reg<u32, _SREPHY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SREPHY;
#[doc = "`read()` method returns [srephy::R](srephy::R) reader structure"]
impl crate::Readable for SREPHY {}
#[doc = "`write(|w| ..)` method takes [srephy::W](srephy::W) writer structure"]
impl crate::Writable for SREPHY {}
#[doc = "Ethernet PHY Software Reset"]
pub mod srephy;
#[doc = "Controller Area Network Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcan](srcan) module"]
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
#[doc = "Analog-to-Digital Converter Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sradc](sradc) module"]
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
#[doc = "Analog Comparator Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sracmp](sracmp) module"]
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
#[doc = "Pulse Width Modulator Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srpwm](srpwm) module"]
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
#[doc = "Quadrature Encoder Interface Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srqei](srqei) module"]
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
#[doc = "EEPROM Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sreeprom](sreeprom) module"]
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
#[doc = "CRC and Cryptographic Modules Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srccm](srccm) module"]
pub type SRCCM = crate::Reg<u32, _SRCCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCCM;
#[doc = "`read()` method returns [srccm::R](srccm::R) reader structure"]
impl crate::Readable for SRCCM {}
#[doc = "`write(|w| ..)` method takes [srccm::W](srccm::W) writer structure"]
impl crate::Writable for SRCCM {}
#[doc = "CRC and Cryptographic Modules Software Reset"]
pub mod srccm;
#[doc = "Ethernet MAC Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sremac](sremac) module"]
pub type SREMAC = crate::Reg<u32, _SREMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SREMAC;
#[doc = "`read()` method returns [sremac::R](sremac::R) reader structure"]
impl crate::Readable for SREMAC {}
#[doc = "`write(|w| ..)` method takes [sremac::W](sremac::W) writer structure"]
impl crate::Writable for SREMAC {}
#[doc = "Ethernet MAC Software Reset"]
pub mod sremac;
#[doc = "Watchdog Timer Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcwd](rcgcwd) module"]
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
#[doc = "16/32-Bit General-Purpose Timer Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgctimer](rcgctimer) module"]
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
#[doc = "General-Purpose Input/Output Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcgpio](rcgcgpio) module"]
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
#[doc = "Micro Direct Memory Access Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcdma](rcgcdma) module"]
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
#[doc = "EPI Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcepi](rcgcepi) module"]
pub type RCGCEPI = crate::Reg<u32, _RCGCEPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCEPI;
#[doc = "`read()` method returns [rcgcepi::R](rcgcepi::R) reader structure"]
impl crate::Readable for RCGCEPI {}
#[doc = "`write(|w| ..)` method takes [rcgcepi::W](rcgcepi::W) writer structure"]
impl crate::Writable for RCGCEPI {}
#[doc = "EPI Run Mode Clock Gating Control"]
pub mod rcgcepi;
#[doc = "Hibernation Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgchib](rcgchib) module"]
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
#[doc = "Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcuart](rcgcuart) module"]
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
#[doc = "Synchronous Serial Interface Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcssi](rcgcssi) module"]
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
#[doc = "Inter-Integrated Circuit Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgci2c](rcgci2c) module"]
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
#[doc = "Universal Serial Bus Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcusb](rcgcusb) module"]
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
#[doc = "Ethernet PHY Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcephy](rcgcephy) module"]
pub type RCGCEPHY = crate::Reg<u32, _RCGCEPHY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCEPHY;
#[doc = "`read()` method returns [rcgcephy::R](rcgcephy::R) reader structure"]
impl crate::Readable for RCGCEPHY {}
#[doc = "`write(|w| ..)` method takes [rcgcephy::W](rcgcephy::W) writer structure"]
impl crate::Writable for RCGCEPHY {}
#[doc = "Ethernet PHY Run Mode Clock Gating Control"]
pub mod rcgcephy;
#[doc = "Controller Area Network Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgccan](rcgccan) module"]
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
#[doc = "Analog-to-Digital Converter Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcadc](rcgcadc) module"]
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
#[doc = "Analog Comparator Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcacmp](rcgcacmp) module"]
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
#[doc = "Pulse Width Modulator Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcpwm](rcgcpwm) module"]
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
#[doc = "Quadrature Encoder Interface Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcqei](rcgcqei) module"]
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
#[doc = "EEPROM Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgceeprom](rcgceeprom) module"]
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
#[doc = "CRC and Cryptographic Modules Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcccm](rcgcccm) module"]
pub type RCGCCCM = crate::Reg<u32, _RCGCCCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCCCM;
#[doc = "`read()` method returns [rcgcccm::R](rcgcccm::R) reader structure"]
impl crate::Readable for RCGCCCM {}
#[doc = "`write(|w| ..)` method takes [rcgcccm::W](rcgcccm::W) writer structure"]
impl crate::Writable for RCGCCCM {}
#[doc = "CRC and Cryptographic Modules Run Mode Clock Gating Control"]
pub mod rcgcccm;
#[doc = "Ethernet MAC Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcemac](rcgcemac) module"]
pub type RCGCEMAC = crate::Reg<u32, _RCGCEMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCEMAC;
#[doc = "`read()` method returns [rcgcemac::R](rcgcemac::R) reader structure"]
impl crate::Readable for RCGCEMAC {}
#[doc = "`write(|w| ..)` method takes [rcgcemac::W](rcgcemac::W) writer structure"]
impl crate::Writable for RCGCEMAC {}
#[doc = "Ethernet MAC Run Mode Clock Gating Control"]
pub mod rcgcemac;
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcwd](scgcwd) module"]
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
#[doc = "16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgctimer](scgctimer) module"]
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
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcgpio](scgcgpio) module"]
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
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcdma](scgcdma) module"]
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
#[doc = "EPI Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcepi](scgcepi) module"]
pub type SCGCEPI = crate::Reg<u32, _SCGCEPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCEPI;
#[doc = "`read()` method returns [scgcepi::R](scgcepi::R) reader structure"]
impl crate::Readable for SCGCEPI {}
#[doc = "`write(|w| ..)` method takes [scgcepi::W](scgcepi::W) writer structure"]
impl crate::Writable for SCGCEPI {}
#[doc = "EPI Sleep Mode Clock Gating Control"]
pub mod scgcepi;
#[doc = "Hibernation Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgchib](scgchib) module"]
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
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcuart](scgcuart) module"]
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
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcssi](scgcssi) module"]
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
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgci2c](scgci2c) module"]
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
#[doc = "Universal Serial Bus Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcusb](scgcusb) module"]
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
#[doc = "Ethernet PHY Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcephy](scgcephy) module"]
pub type SCGCEPHY = crate::Reg<u32, _SCGCEPHY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCEPHY;
#[doc = "`read()` method returns [scgcephy::R](scgcephy::R) reader structure"]
impl crate::Readable for SCGCEPHY {}
#[doc = "`write(|w| ..)` method takes [scgcephy::W](scgcephy::W) writer structure"]
impl crate::Writable for SCGCEPHY {}
#[doc = "Ethernet PHY Sleep Mode Clock Gating Control"]
pub mod scgcephy;
#[doc = "Controller Area Network Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgccan](scgccan) module"]
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
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcadc](scgcadc) module"]
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
#[doc = "Analog Comparator Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcacmp](scgcacmp) module"]
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
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcpwm](scgcpwm) module"]
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
#[doc = "Quadrature Encoder Interface Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcqei](scgcqei) module"]
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
#[doc = "EEPROM Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgceeprom](scgceeprom) module"]
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
#[doc = "CRC and Cryptographic Modules Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcccm](scgcccm) module"]
pub type SCGCCCM = crate::Reg<u32, _SCGCCCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCCCM;
#[doc = "`read()` method returns [scgcccm::R](scgcccm::R) reader structure"]
impl crate::Readable for SCGCCCM {}
#[doc = "`write(|w| ..)` method takes [scgcccm::W](scgcccm::W) writer structure"]
impl crate::Writable for SCGCCCM {}
#[doc = "CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
pub mod scgcccm;
#[doc = "Ethernet MAC Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcemac](scgcemac) module"]
pub type SCGCEMAC = crate::Reg<u32, _SCGCEMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCEMAC;
#[doc = "`read()` method returns [scgcemac::R](scgcemac::R) reader structure"]
impl crate::Readable for SCGCEMAC {}
#[doc = "`write(|w| ..)` method takes [scgcemac::W](scgcemac::W) writer structure"]
impl crate::Writable for SCGCEMAC {}
#[doc = "Ethernet MAC Sleep Mode Clock Gating Control"]
pub mod scgcemac;
#[doc = "Watchdog Timer Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcwd](dcgcwd) module"]
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
#[doc = "16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgctimer](dcgctimer) module"]
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
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcgpio](dcgcgpio) module"]
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
#[doc = "Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcdma](dcgcdma) module"]
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
#[doc = "EPI Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcepi](dcgcepi) module"]
pub type DCGCEPI = crate::Reg<u32, _DCGCEPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCEPI;
#[doc = "`read()` method returns [dcgcepi::R](dcgcepi::R) reader structure"]
impl crate::Readable for DCGCEPI {}
#[doc = "`write(|w| ..)` method takes [dcgcepi::W](dcgcepi::W) writer structure"]
impl crate::Writable for DCGCEPI {}
#[doc = "EPI Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcepi;
#[doc = "Hibernation Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgchib](dcgchib) module"]
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
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcuart](dcgcuart) module"]
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
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcssi](dcgcssi) module"]
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
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgci2c](dcgci2c) module"]
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
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcusb](dcgcusb) module"]
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
#[doc = "Ethernet PHY Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcephy](dcgcephy) module"]
pub type DCGCEPHY = crate::Reg<u32, _DCGCEPHY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCEPHY;
#[doc = "`read()` method returns [dcgcephy::R](dcgcephy::R) reader structure"]
impl crate::Readable for DCGCEPHY {}
#[doc = "`write(|w| ..)` method takes [dcgcephy::W](dcgcephy::W) writer structure"]
impl crate::Writable for DCGCEPHY {}
#[doc = "Ethernet PHY Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcephy;
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgccan](dcgccan) module"]
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
#[doc = "Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcadc](dcgcadc) module"]
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
#[doc = "Analog Comparator Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcacmp](dcgcacmp) module"]
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
#[doc = "Pulse Width Modulator Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcpwm](dcgcpwm) module"]
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
#[doc = "Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcqei](dcgcqei) module"]
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
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgceeprom](dcgceeprom) module"]
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
#[doc = "CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcccm](dcgcccm) module"]
pub type DCGCCCM = crate::Reg<u32, _DCGCCCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCCCM;
#[doc = "`read()` method returns [dcgcccm::R](dcgcccm::R) reader structure"]
impl crate::Readable for DCGCCCM {}
#[doc = "`write(|w| ..)` method takes [dcgcccm::W](dcgcccm::W) writer structure"]
impl crate::Writable for DCGCCCM {}
#[doc = "CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcccm;
#[doc = "Ethernet MAC Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcemac](dcgcemac) module"]
pub type DCGCEMAC = crate::Reg<u32, _DCGCEMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCEMAC;
#[doc = "`read()` method returns [dcgcemac::R](dcgcemac::R) reader structure"]
impl crate::Readable for DCGCEMAC {}
#[doc = "`write(|w| ..)` method takes [dcgcemac::W](dcgcemac::W) writer structure"]
impl crate::Writable for DCGCEMAC {}
#[doc = "Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcemac;
#[doc = "Watchdog Timer Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcwd](pcwd) module"]
pub type PCWD = crate::Reg<u32, _PCWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCWD;
#[doc = "`read()` method returns [pcwd::R](pcwd::R) reader structure"]
impl crate::Readable for PCWD {}
#[doc = "`write(|w| ..)` method takes [pcwd::W](pcwd::W) writer structure"]
impl crate::Writable for PCWD {}
#[doc = "Watchdog Timer Power Control"]
pub mod pcwd;
#[doc = "16/32-Bit General-Purpose Timer Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pctimer](pctimer) module"]
pub type PCTIMER = crate::Reg<u32, _PCTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCTIMER;
#[doc = "`read()` method returns [pctimer::R](pctimer::R) reader structure"]
impl crate::Readable for PCTIMER {}
#[doc = "`write(|w| ..)` method takes [pctimer::W](pctimer::W) writer structure"]
impl crate::Writable for PCTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Power Control"]
pub mod pctimer;
#[doc = "General-Purpose Input/Output Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcgpio](pcgpio) module"]
pub type PCGPIO = crate::Reg<u32, _PCGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCGPIO;
#[doc = "`read()` method returns [pcgpio::R](pcgpio::R) reader structure"]
impl crate::Readable for PCGPIO {}
#[doc = "`write(|w| ..)` method takes [pcgpio::W](pcgpio::W) writer structure"]
impl crate::Writable for PCGPIO {}
#[doc = "General-Purpose Input/Output Power Control"]
pub mod pcgpio;
#[doc = "Micro Direct Memory Access Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdma](pcdma) module"]
pub type PCDMA = crate::Reg<u32, _PCDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCDMA;
#[doc = "`read()` method returns [pcdma::R](pcdma::R) reader structure"]
impl crate::Readable for PCDMA {}
#[doc = "`write(|w| ..)` method takes [pcdma::W](pcdma::W) writer structure"]
impl crate::Writable for PCDMA {}
#[doc = "Micro Direct Memory Access Power Control"]
pub mod pcdma;
#[doc = "External Peripheral Interface Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcepi](pcepi) module"]
pub type PCEPI = crate::Reg<u32, _PCEPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCEPI;
#[doc = "`read()` method returns [pcepi::R](pcepi::R) reader structure"]
impl crate::Readable for PCEPI {}
#[doc = "`write(|w| ..)` method takes [pcepi::W](pcepi::W) writer structure"]
impl crate::Writable for PCEPI {}
#[doc = "External Peripheral Interface Power Control"]
pub mod pcepi;
#[doc = "Hibernation Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pchib](pchib) module"]
pub type PCHIB = crate::Reg<u32, _PCHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCHIB;
#[doc = "`read()` method returns [pchib::R](pchib::R) reader structure"]
impl crate::Readable for PCHIB {}
#[doc = "`write(|w| ..)` method takes [pchib::W](pchib::W) writer structure"]
impl crate::Writable for PCHIB {}
#[doc = "Hibernation Power Control"]
pub mod pchib;
#[doc = "Universal Asynchronous Receiver/Transmitter Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcuart](pcuart) module"]
pub type PCUART = crate::Reg<u32, _PCUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCUART;
#[doc = "`read()` method returns [pcuart::R](pcuart::R) reader structure"]
impl crate::Readable for PCUART {}
#[doc = "`write(|w| ..)` method takes [pcuart::W](pcuart::W) writer structure"]
impl crate::Writable for PCUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Power Control"]
pub mod pcuart;
#[doc = "Synchronous Serial Interface Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcssi](pcssi) module"]
pub type PCSSI = crate::Reg<u32, _PCSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCSSI;
#[doc = "`read()` method returns [pcssi::R](pcssi::R) reader structure"]
impl crate::Readable for PCSSI {}
#[doc = "`write(|w| ..)` method takes [pcssi::W](pcssi::W) writer structure"]
impl crate::Writable for PCSSI {}
#[doc = "Synchronous Serial Interface Power Control"]
pub mod pcssi;
#[doc = "Inter-Integrated Circuit Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pci2c](pci2c) module"]
pub type PCI2C = crate::Reg<u32, _PCI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCI2C;
#[doc = "`read()` method returns [pci2c::R](pci2c::R) reader structure"]
impl crate::Readable for PCI2C {}
#[doc = "`write(|w| ..)` method takes [pci2c::W](pci2c::W) writer structure"]
impl crate::Writable for PCI2C {}
#[doc = "Inter-Integrated Circuit Power Control"]
pub mod pci2c;
#[doc = "Universal Serial Bus Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcusb](pcusb) module"]
pub type PCUSB = crate::Reg<u32, _PCUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCUSB;
#[doc = "`read()` method returns [pcusb::R](pcusb::R) reader structure"]
impl crate::Readable for PCUSB {}
#[doc = "`write(|w| ..)` method takes [pcusb::W](pcusb::W) writer structure"]
impl crate::Writable for PCUSB {}
#[doc = "Universal Serial Bus Power Control"]
pub mod pcusb;
#[doc = "Ethernet PHY Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcephy](pcephy) module"]
pub type PCEPHY = crate::Reg<u32, _PCEPHY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCEPHY;
#[doc = "`read()` method returns [pcephy::R](pcephy::R) reader structure"]
impl crate::Readable for PCEPHY {}
#[doc = "`write(|w| ..)` method takes [pcephy::W](pcephy::W) writer structure"]
impl crate::Writable for PCEPHY {}
#[doc = "Ethernet PHY Power Control"]
pub mod pcephy;
#[doc = "Controller Area Network Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccan](pccan) module"]
pub type PCCAN = crate::Reg<u32, _PCCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCAN;
#[doc = "`read()` method returns [pccan::R](pccan::R) reader structure"]
impl crate::Readable for PCCAN {}
#[doc = "`write(|w| ..)` method takes [pccan::W](pccan::W) writer structure"]
impl crate::Writable for PCCAN {}
#[doc = "Controller Area Network Power Control"]
pub mod pccan;
#[doc = "Analog-to-Digital Converter Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcadc](pcadc) module"]
pub type PCADC = crate::Reg<u32, _PCADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCADC;
#[doc = "`read()` method returns [pcadc::R](pcadc::R) reader structure"]
impl crate::Readable for PCADC {}
#[doc = "`write(|w| ..)` method takes [pcadc::W](pcadc::W) writer structure"]
impl crate::Writable for PCADC {}
#[doc = "Analog-to-Digital Converter Power Control"]
pub mod pcadc;
#[doc = "Analog Comparator Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcacmp](pcacmp) module"]
pub type PCACMP = crate::Reg<u32, _PCACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCACMP;
#[doc = "`read()` method returns [pcacmp::R](pcacmp::R) reader structure"]
impl crate::Readable for PCACMP {}
#[doc = "`write(|w| ..)` method takes [pcacmp::W](pcacmp::W) writer structure"]
impl crate::Writable for PCACMP {}
#[doc = "Analog Comparator Power Control"]
pub mod pcacmp;
#[doc = "Pulse Width Modulator Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpwm](pcpwm) module"]
pub type PCPWM = crate::Reg<u32, _PCPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCPWM;
#[doc = "`read()` method returns [pcpwm::R](pcpwm::R) reader structure"]
impl crate::Readable for PCPWM {}
#[doc = "`write(|w| ..)` method takes [pcpwm::W](pcpwm::W) writer structure"]
impl crate::Writable for PCPWM {}
#[doc = "Pulse Width Modulator Power Control"]
pub mod pcpwm;
#[doc = "Quadrature Encoder Interface Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcqei](pcqei) module"]
pub type PCQEI = crate::Reg<u32, _PCQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCQEI;
#[doc = "`read()` method returns [pcqei::R](pcqei::R) reader structure"]
impl crate::Readable for PCQEI {}
#[doc = "`write(|w| ..)` method takes [pcqei::W](pcqei::W) writer structure"]
impl crate::Writable for PCQEI {}
#[doc = "Quadrature Encoder Interface Power Control"]
pub mod pcqei;
#[doc = "EEPROM Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pceeprom](pceeprom) module"]
pub type PCEEPROM = crate::Reg<u32, _PCEEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCEEPROM;
#[doc = "`read()` method returns [pceeprom::R](pceeprom::R) reader structure"]
impl crate::Readable for PCEEPROM {}
#[doc = "`write(|w| ..)` method takes [pceeprom::W](pceeprom::W) writer structure"]
impl crate::Writable for PCEEPROM {}
#[doc = "EEPROM Power Control"]
pub mod pceeprom;
#[doc = "CRC and Cryptographic Modules Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcccm](pcccm) module"]
pub type PCCCM = crate::Reg<u32, _PCCCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCCM;
#[doc = "`read()` method returns [pcccm::R](pcccm::R) reader structure"]
impl crate::Readable for PCCCM {}
#[doc = "`write(|w| ..)` method takes [pcccm::W](pcccm::W) writer structure"]
impl crate::Writable for PCCCM {}
#[doc = "CRC and Cryptographic Modules Power Control"]
pub mod pcccm;
#[doc = "Ethernet MAC Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcemac](pcemac) module"]
pub type PCEMAC = crate::Reg<u32, _PCEMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCEMAC;
#[doc = "`read()` method returns [pcemac::R](pcemac::R) reader structure"]
impl crate::Readable for PCEMAC {}
#[doc = "`write(|w| ..)` method takes [pcemac::W](pcemac::W) writer structure"]
impl crate::Writable for PCEMAC {}
#[doc = "Ethernet MAC Power Control"]
pub mod pcemac;
#[doc = "Watchdog Timer Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prwd](prwd) module"]
pub type PRWD = crate::Reg<u32, _PRWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRWD;
#[doc = "`read()` method returns [prwd::R](prwd::R) reader structure"]
impl crate::Readable for PRWD {}
#[doc = "Watchdog Timer Peripheral Ready"]
pub mod prwd;
#[doc = "16/32-Bit General-Purpose Timer Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prtimer](prtimer) module"]
pub type PRTIMER = crate::Reg<u32, _PRTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRTIMER;
#[doc = "`read()` method returns [prtimer::R](prtimer::R) reader structure"]
impl crate::Readable for PRTIMER {}
#[doc = "16/32-Bit General-Purpose Timer Peripheral Ready"]
pub mod prtimer;
#[doc = "General-Purpose Input/Output Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prgpio](prgpio) module"]
pub type PRGPIO = crate::Reg<u32, _PRGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRGPIO;
#[doc = "`read()` method returns [prgpio::R](prgpio::R) reader structure"]
impl crate::Readable for PRGPIO {}
#[doc = "General-Purpose Input/Output Peripheral Ready"]
pub mod prgpio;
#[doc = "Micro Direct Memory Access Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prdma](prdma) module"]
pub type PRDMA = crate::Reg<u32, _PRDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRDMA;
#[doc = "`read()` method returns [prdma::R](prdma::R) reader structure"]
impl crate::Readable for PRDMA {}
#[doc = "Micro Direct Memory Access Peripheral Ready"]
pub mod prdma;
#[doc = "EPI Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prepi](prepi) module"]
pub type PREPI = crate::Reg<u32, _PREPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREPI;
#[doc = "`read()` method returns [prepi::R](prepi::R) reader structure"]
impl crate::Readable for PREPI {}
#[doc = "EPI Peripheral Ready"]
pub mod prepi;
#[doc = "Hibernation Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prhib](prhib) module"]
pub type PRHIB = crate::Reg<u32, _PRHIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRHIB;
#[doc = "`read()` method returns [prhib::R](prhib::R) reader structure"]
impl crate::Readable for PRHIB {}
#[doc = "Hibernation Peripheral Ready"]
pub mod prhib;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pruart](pruart) module"]
pub type PRUART = crate::Reg<u32, _PRUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRUART;
#[doc = "`read()` method returns [pruart::R](pruart::R) reader structure"]
impl crate::Readable for PRUART {}
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
pub mod pruart;
#[doc = "Synchronous Serial Interface Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prssi](prssi) module"]
pub type PRSSI = crate::Reg<u32, _PRSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSSI;
#[doc = "`read()` method returns [prssi::R](prssi::R) reader structure"]
impl crate::Readable for PRSSI {}
#[doc = "Synchronous Serial Interface Peripheral Ready"]
pub mod prssi;
#[doc = "Inter-Integrated Circuit Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pri2c](pri2c) module"]
pub type PRI2C = crate::Reg<u32, _PRI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRI2C;
#[doc = "`read()` method returns [pri2c::R](pri2c::R) reader structure"]
impl crate::Readable for PRI2C {}
#[doc = "Inter-Integrated Circuit Peripheral Ready"]
pub mod pri2c;
#[doc = "Universal Serial Bus Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prusb](prusb) module"]
pub type PRUSB = crate::Reg<u32, _PRUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRUSB;
#[doc = "`read()` method returns [prusb::R](prusb::R) reader structure"]
impl crate::Readable for PRUSB {}
#[doc = "Universal Serial Bus Peripheral Ready"]
pub mod prusb;
#[doc = "Ethernet PHY Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prephy](prephy) module"]
pub type PREPHY = crate::Reg<u32, _PREPHY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREPHY;
#[doc = "`read()` method returns [prephy::R](prephy::R) reader structure"]
impl crate::Readable for PREPHY {}
#[doc = "Ethernet PHY Peripheral Ready"]
pub mod prephy;
#[doc = "Controller Area Network Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prcan](prcan) module"]
pub type PRCAN = crate::Reg<u32, _PRCAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRCAN;
#[doc = "`read()` method returns [prcan::R](prcan::R) reader structure"]
impl crate::Readable for PRCAN {}
#[doc = "Controller Area Network Peripheral Ready"]
pub mod prcan;
#[doc = "Analog-to-Digital Converter Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pradc](pradc) module"]
pub type PRADC = crate::Reg<u32, _PRADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRADC;
#[doc = "`read()` method returns [pradc::R](pradc::R) reader structure"]
impl crate::Readable for PRADC {}
#[doc = "Analog-to-Digital Converter Peripheral Ready"]
pub mod pradc;
#[doc = "Analog Comparator Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pracmp](pracmp) module"]
pub type PRACMP = crate::Reg<u32, _PRACMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRACMP;
#[doc = "`read()` method returns [pracmp::R](pracmp::R) reader structure"]
impl crate::Readable for PRACMP {}
#[doc = "Analog Comparator Peripheral Ready"]
pub mod pracmp;
#[doc = "Pulse Width Modulator Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prpwm](prpwm) module"]
pub type PRPWM = crate::Reg<u32, _PRPWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRPWM;
#[doc = "`read()` method returns [prpwm::R](prpwm::R) reader structure"]
impl crate::Readable for PRPWM {}
#[doc = "Pulse Width Modulator Peripheral Ready"]
pub mod prpwm;
#[doc = "Quadrature Encoder Interface Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prqei](prqei) module"]
pub type PRQEI = crate::Reg<u32, _PRQEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRQEI;
#[doc = "`read()` method returns [prqei::R](prqei::R) reader structure"]
impl crate::Readable for PRQEI {}
#[doc = "Quadrature Encoder Interface Peripheral Ready"]
pub mod prqei;
#[doc = "EEPROM Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preeprom](preeprom) module"]
pub type PREEPROM = crate::Reg<u32, _PREEPROM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREEPROM;
#[doc = "`read()` method returns [preeprom::R](preeprom::R) reader structure"]
impl crate::Readable for PREEPROM {}
#[doc = "EEPROM Peripheral Ready"]
pub mod preeprom;
#[doc = "CRC and Cryptographic Modules Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prccm](prccm) module"]
pub type PRCCM = crate::Reg<u32, _PRCCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRCCM;
#[doc = "`read()` method returns [prccm::R](prccm::R) reader structure"]
impl crate::Readable for PRCCM {}
#[doc = "CRC and Cryptographic Modules Peripheral Ready"]
pub mod prccm;
#[doc = "Ethernet MAC Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [premac](premac) module"]
pub type PREMAC = crate::Reg<u32, _PREMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREMAC;
#[doc = "`read()` method returns [premac::R](premac::R) reader structure"]
impl crate::Readable for PREMAC {}
#[doc = "Ethernet MAC Peripheral Ready"]
pub mod premac;
