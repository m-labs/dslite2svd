#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Identification 0"]
    pub did0: DID0,
    #[doc = "0x04 - Device Identification 1"]
    pub did1: DID1,
    _reserved0: [u8; 48usize],
    #[doc = "0x38 - Power-Temp Brown Out Control"]
    pub ptboctl: PTBOCTL,
    _reserved1: [u8; 20usize],
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
    _reserved2: [u8; 20usize],
    #[doc = "0x7c - Main Oscillator Control"]
    pub moscctl: MOSCCTL,
    _reserved3: [u8; 48usize],
    #[doc = "0xb0 - Run and Sleep Mode Configuration Register"]
    pub rsclkcfg: RSCLKCFG,
    _reserved4: [u8; 12usize],
    #[doc = "0xc0 - Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
    pub memtim0: MEMTIM0,
    _reserved5: [u8; 116usize],
    #[doc = "0x138 - Alternate Clock Configuration"]
    pub altclkcfg: ALTCLKCFG,
    _reserved6: [u8; 8usize],
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
    _reserved7: [u8; 8usize],
    #[doc = "0x160 - PLL Frequency 0"]
    pub pllfreq0: PLLFREQ0,
    #[doc = "0x164 - PLL Frequency 1"]
    pub pllfreq1: PLLFREQ1,
    #[doc = "0x168 - PLL Status"]
    pub pllstat: PLLSTAT,
    _reserved8: [u8; 28usize],
    #[doc = "0x188 - Sleep Power Configuration"]
    pub slppwrcfg: SLPPWRCFG,
    #[doc = "0x18c - Deep-Sleep Power Configuration"]
    pub dslppwrcfg: DSLPPWRCFG,
    _reserved9: [u8; 16usize],
    #[doc = "0x1a0 - Non-Volatile Memory Information"]
    pub nvmstat: NVMSTAT,
    _reserved10: [u8; 16usize],
    #[doc = "0x1b4 - LDO Sleep Power Control"]
    pub ldospctl: LDOSPCTL,
    _reserved11: [u8; 4usize],
    #[doc = "0x1bc - LDO Deep-Sleep Power Control"]
    pub ldodpctl: LDODPCTL,
    _reserved12: [u8; 24usize],
    #[doc = "0x1d8 - Reset Behavior Control Register"]
    pub resbehavctl: RESBEHAVCTL,
    _reserved13: [u8; 24usize],
    #[doc = "0x1f4 - Hardware System Service Request"]
    pub hssr: HSSR,
    _reserved14: [u8; 136usize],
    #[doc = "0x280 - USB Power Domain Status"]
    pub usbpds: USBPDS,
    #[doc = "0x284 - USB Memory Power Control"]
    pub usbmpc: USBMPC,
    #[doc = "0x288 - Ethernet MAC Power Domain Status"]
    pub emacpds: EMACPDS,
    #[doc = "0x28c - Ethernet MAC Memory Power Control"]
    pub emacmpc: EMACMPC,
    _reserved15: [u8; 112usize],
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
    _reserved16: [u8; 4usize],
    #[doc = "0x328 - Universal Serial Bus Peripheral Present"]
    pub ppusb: PPUSB,
    _reserved17: [u8; 4usize],
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
    _reserved18: [u8; 4usize],
    #[doc = "0x350 - Platform Environment Control Interface Peripheral Present"]
    pub pppeci: PPPECI,
    #[doc = "0x354 - Fan Control Peripheral Present"]
    pub ppfan: PPFAN,
    #[doc = "0x358 - EEPROM Peripheral Present"]
    pub ppeeprom: PPEEPROM,
    #[doc = "0x35c - 32/64-Bit Wide General-Purpose Timer Peripheral Present"]
    pub ppwtimer: PPWTIMER,
    _reserved19: [u8; 16usize],
    #[doc = "0x370 - Remote Temperature Sensor Peripheral Present"]
    pub pprts: PPRTS,
    #[doc = "0x374 - CRC and Cryptographic Modules Peripheral Present"]
    pub ppccm: PPCCM,
    _reserved20: [u8; 24usize],
    #[doc = "0x390 - LCD Peripheral Present"]
    pub pplcd: PPLCD,
    _reserved21: [u8; 4usize],
    #[doc = "0x398 - 1-Wire Peripheral Present"]
    pub ppowire: PPOWIRE,
    #[doc = "0x39c - Ethernet MAC Peripheral Present"]
    pub ppemac: PPEMAC,
    _reserved22: [u8; 4usize],
    #[doc = "0x3a4 - Human Interface Master Peripheral Present"]
    pub pphim: PPHIM,
    _reserved23: [u8; 344usize],
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
    _reserved24: [u8; 4usize],
    #[doc = "0x528 - Universal Serial Bus Software Reset"]
    pub srusb: SRUSB,
    _reserved25: [u8; 4usize],
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
    _reserved26: [u8; 16usize],
    #[doc = "0x558 - EEPROM Software Reset"]
    pub sreeprom: SREEPROM,
    _reserved27: [u8; 24usize],
    #[doc = "0x574 - CRC and Cryptographic Modules Software Reset"]
    pub srccm: SRCCM,
    _reserved28: [u8; 36usize],
    #[doc = "0x59c - Ethernet MAC Software Reset"]
    pub sremac: SREMAC,
    _reserved29: [u8; 96usize],
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
    _reserved30: [u8; 4usize],
    #[doc = "0x628 - Universal Serial Bus Run Mode Clock Gating Control"]
    pub rcgcusb: RCGCUSB,
    _reserved31: [u8; 4usize],
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
    _reserved32: [u8; 16usize],
    #[doc = "0x658 - EEPROM Run Mode Clock Gating Control"]
    pub rcgceeprom: RCGCEEPROM,
    _reserved33: [u8; 24usize],
    #[doc = "0x674 - CRC and Cryptographic Modules Run Mode Clock Gating Control"]
    pub rcgcccm: RCGCCCM,
    _reserved34: [u8; 36usize],
    #[doc = "0x69c - Ethernet MAC Run Mode Clock Gating Control"]
    pub rcgcemac: RCGCEMAC,
    _reserved35: [u8; 96usize],
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
    _reserved36: [u8; 4usize],
    #[doc = "0x728 - Universal Serial Bus Sleep Mode Clock Gating Control"]
    pub scgcusb: SCGCUSB,
    _reserved37: [u8; 4usize],
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
    _reserved38: [u8; 16usize],
    #[doc = "0x758 - EEPROM Sleep Mode Clock Gating Control"]
    pub scgceeprom: SCGCEEPROM,
    _reserved39: [u8; 24usize],
    #[doc = "0x774 - CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
    pub scgcccm: SCGCCCM,
    _reserved40: [u8; 36usize],
    #[doc = "0x79c - Ethernet MAC Sleep Mode Clock Gating Control"]
    pub scgcemac: SCGCEMAC,
    _reserved41: [u8; 96usize],
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
    _reserved42: [u8; 4usize],
    #[doc = "0x828 - Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
    pub dcgcusb: DCGCUSB,
    _reserved43: [u8; 4usize],
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
    _reserved44: [u8; 16usize],
    #[doc = "0x858 - EEPROM Deep-Sleep Mode Clock Gating Control"]
    pub dcgceeprom: DCGCEEPROM,
    _reserved45: [u8; 24usize],
    #[doc = "0x874 - CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
    pub dcgcccm: DCGCCCM,
    _reserved46: [u8; 36usize],
    #[doc = "0x89c - Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
    pub dcgcemac: DCGCEMAC,
    _reserved47: [u8; 96usize],
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
    _reserved48: [u8; 4usize],
    #[doc = "0x928 - Universal Serial Bus Power Control"]
    pub pcusb: PCUSB,
    _reserved49: [u8; 4usize],
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
    _reserved50: [u8; 16usize],
    #[doc = "0x958 - EEPROM Power Control"]
    pub pceeprom: PCEEPROM,
    _reserved51: [u8; 24usize],
    #[doc = "0x974 - CRC and Cryptographic Modules Power Control"]
    pub pcccm: PCCCM,
    _reserved52: [u8; 36usize],
    #[doc = "0x99c - Ethernet MAC Power Control"]
    pub pcemac: PCEMAC,
    _reserved53: [u8; 96usize],
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
    _reserved54: [u8; 4usize],
    #[doc = "0xa28 - Universal Serial Bus Peripheral Ready"]
    pub prusb: PRUSB,
    _reserved55: [u8; 4usize],
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
    _reserved56: [u8; 16usize],
    #[doc = "0xa58 - EEPROM Peripheral Ready"]
    pub preeprom: PREEPROM,
    _reserved57: [u8; 24usize],
    #[doc = "0xa74 - CRC and Cryptographic Modules Peripheral Ready"]
    pub prccm: PRCCM,
    _reserved58: [u8; 36usize],
    #[doc = "0xa9c - Ethernet MAC Peripheral Ready"]
    pub premac: PREMAC,
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
#[doc = "Power-Temp Brown Out Control"]
pub struct PTBOCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-Temp Brown Out Control"]
pub mod ptboctl;
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
#[doc = "Power-Temperature Cause"]
pub struct PWRTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-Temperature Cause"]
pub mod pwrtc;
#[doc = "NMI Cause Register"]
pub struct NMIC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NMI Cause Register"]
pub mod nmic;
#[doc = "Main Oscillator Control"]
pub struct MOSCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main Oscillator Control"]
pub mod moscctl;
#[doc = "Run and Sleep Mode Configuration Register"]
pub struct RSCLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run and Sleep Mode Configuration Register"]
pub mod rsclkcfg;
#[doc = "Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
pub struct MEMTIM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
pub mod memtim0;
#[doc = "Alternate Clock Configuration"]
pub struct ALTCLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate Clock Configuration"]
pub mod altclkcfg;
#[doc = "Deep Sleep Clock Configuration Register"]
pub struct DSCLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep Sleep Clock Configuration Register"]
pub mod dsclkcfg;
#[doc = "Divisor and Source Clock Configuration"]
pub struct DIVSCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Divisor and Source Clock Configuration"]
pub mod divsclk;
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
#[doc = "Reset Behavior Control Register"]
pub struct RESBEHAVCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Behavior Control Register"]
pub mod resbehavctl;
#[doc = "Hardware System Service Request"]
pub struct HSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware System Service Request"]
pub mod hssr;
#[doc = "USB Power Domain Status"]
pub struct USBPDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Power Domain Status"]
pub mod usbpds;
#[doc = "USB Memory Power Control"]
pub struct USBMPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Memory Power Control"]
pub mod usbmpc;
#[doc = "Ethernet MAC Power Domain Status"]
pub struct EMACPDS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Power Domain Status"]
pub mod emacpds;
#[doc = "Ethernet MAC Memory Power Control"]
pub struct EMACMPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Memory Power Control"]
pub mod emacmpc;
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
#[doc = "EPI Peripheral Present"]
pub struct PPEPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Peripheral Present"]
pub mod ppepi;
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
#[doc = "Ethernet PHY Peripheral Present"]
pub struct PPEPHY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Peripheral Present"]
pub mod ppephy;
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
#[doc = "Low Pin Count Interface Peripheral Present"]
pub struct PPLPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Pin Count Interface Peripheral Present"]
pub mod pplpc;
#[doc = "Platform Environment Control Interface Peripheral Present"]
pub struct PPPECI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Platform Environment Control Interface Peripheral Present"]
pub mod pppeci;
#[doc = "Fan Control Peripheral Present"]
pub struct PPFAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fan Control Peripheral Present"]
pub mod ppfan;
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
#[doc = "Remote Temperature Sensor Peripheral Present"]
pub struct PPRTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Remote Temperature Sensor Peripheral Present"]
pub mod pprts;
#[doc = "CRC and Cryptographic Modules Peripheral Present"]
pub struct PPCCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC and Cryptographic Modules Peripheral Present"]
pub mod ppccm;
#[doc = "LCD Peripheral Present"]
pub struct PPLCD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Peripheral Present"]
pub mod pplcd;
#[doc = "1-Wire Peripheral Present"]
pub struct PPOWIRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Peripheral Present"]
pub mod ppowire;
#[doc = "Ethernet MAC Peripheral Present"]
pub struct PPEMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Peripheral Present"]
pub mod ppemac;
#[doc = "Human Interface Master Peripheral Present"]
pub struct PPHIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Human Interface Master Peripheral Present"]
pub mod pphim;
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
#[doc = "EPI Software Reset"]
pub struct SREPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Software Reset"]
pub mod srepi;
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
#[doc = "Ethernet PHY Software Reset"]
pub struct SREPHY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Software Reset"]
pub mod srephy;
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
#[doc = "CRC and Cryptographic Modules Software Reset"]
pub struct SRCCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC and Cryptographic Modules Software Reset"]
pub mod srccm;
#[doc = "Ethernet MAC Software Reset"]
pub struct SREMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Software Reset"]
pub mod sremac;
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
#[doc = "EPI Run Mode Clock Gating Control"]
pub struct RCGCEPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Run Mode Clock Gating Control"]
pub mod rcgcepi;
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
#[doc = "Ethernet PHY Run Mode Clock Gating Control"]
pub struct RCGCEPHY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Run Mode Clock Gating Control"]
pub mod rcgcephy;
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
#[doc = "CRC and Cryptographic Modules Run Mode Clock Gating Control"]
pub struct RCGCCCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC and Cryptographic Modules Run Mode Clock Gating Control"]
pub mod rcgcccm;
#[doc = "Ethernet MAC Run Mode Clock Gating Control"]
pub struct RCGCEMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Run Mode Clock Gating Control"]
pub mod rcgcemac;
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
#[doc = "EPI Sleep Mode Clock Gating Control"]
pub struct SCGCEPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Sleep Mode Clock Gating Control"]
pub mod scgcepi;
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
#[doc = "Ethernet PHY Sleep Mode Clock Gating Control"]
pub struct SCGCEPHY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Sleep Mode Clock Gating Control"]
pub mod scgcephy;
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
#[doc = "CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
pub struct SCGCCCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
pub mod scgcccm;
#[doc = "Ethernet MAC Sleep Mode Clock Gating Control"]
pub struct SCGCEMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Sleep Mode Clock Gating Control"]
pub mod scgcemac;
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
#[doc = "EPI Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCEPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcepi;
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
#[doc = "Ethernet PHY Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCEPHY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcephy;
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
#[doc = "CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCCCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcccm;
#[doc = "Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
pub struct DCGCEMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcemac;
#[doc = "Watchdog Timer Power Control"]
pub struct PCWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Timer Power Control"]
pub mod pcwd;
#[doc = "16/32-Bit General-Purpose Timer Power Control"]
pub struct PCTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "16/32-Bit General-Purpose Timer Power Control"]
pub mod pctimer;
#[doc = "General-Purpose Input/Output Power Control"]
pub struct PCGPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General-Purpose Input/Output Power Control"]
pub mod pcgpio;
#[doc = "Micro Direct Memory Access Power Control"]
pub struct PCDMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Micro Direct Memory Access Power Control"]
pub mod pcdma;
#[doc = "External Peripheral Interface Power Control"]
pub struct PCEPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Peripheral Interface Power Control"]
pub mod pcepi;
#[doc = "Hibernation Power Control"]
pub struct PCHIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernation Power Control"]
pub mod pchib;
#[doc = "Universal Asynchronous Receiver/Transmitter Power Control"]
pub struct PCUART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Asynchronous Receiver/Transmitter Power Control"]
pub mod pcuart;
#[doc = "Synchronous Serial Interface Power Control"]
pub struct PCSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronous Serial Interface Power Control"]
pub mod pcssi;
#[doc = "Inter-Integrated Circuit Power Control"]
pub struct PCI2C {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inter-Integrated Circuit Power Control"]
pub mod pci2c;
#[doc = "Universal Serial Bus Power Control"]
pub struct PCUSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Universal Serial Bus Power Control"]
pub mod pcusb;
#[doc = "Ethernet PHY Power Control"]
pub struct PCEPHY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Power Control"]
pub mod pcephy;
#[doc = "Controller Area Network Power Control"]
pub struct PCCAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controller Area Network Power Control"]
pub mod pccan;
#[doc = "Analog-to-Digital Converter Power Control"]
pub struct PCADC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog-to-Digital Converter Power Control"]
pub mod pcadc;
#[doc = "Analog Comparator Power Control"]
pub struct PCACMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator Power Control"]
pub mod pcacmp;
#[doc = "Pulse Width Modulator Power Control"]
pub struct PCPWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Width Modulator Power Control"]
pub mod pcpwm;
#[doc = "Quadrature Encoder Interface Power Control"]
pub struct PCQEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Encoder Interface Power Control"]
pub mod pcqei;
#[doc = "EEPROM Power Control"]
pub struct PCEEPROM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM Power Control"]
pub mod pceeprom;
#[doc = "CRC and Cryptographic Modules Power Control"]
pub struct PCCCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC and Cryptographic Modules Power Control"]
pub mod pcccm;
#[doc = "Ethernet MAC Power Control"]
pub struct PCEMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Power Control"]
pub mod pcemac;
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
#[doc = "EPI Peripheral Ready"]
pub struct PREPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EPI Peripheral Ready"]
pub mod prepi;
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
#[doc = "Ethernet PHY Peripheral Ready"]
pub struct PREPHY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Peripheral Ready"]
pub mod prephy;
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
#[doc = "CRC and Cryptographic Modules Peripheral Ready"]
pub struct PRCCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC and Cryptographic Modules Peripheral Ready"]
pub mod prccm;
#[doc = "Ethernet MAC Peripheral Ready"]
pub struct PREMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC Peripheral Ready"]
pub mod premac;
