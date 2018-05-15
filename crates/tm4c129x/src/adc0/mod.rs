#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Active Sample Sequencer"]
    pub actss: ACTSS,
    #[doc = "0x04 - ADC Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x08 - ADC Interrupt Mask"]
    pub im: IM,
    #[doc = "0x0c - ADC Interrupt Status and Clear"]
    pub isc: ISC,
    #[doc = "0x10 - ADC Overflow Status"]
    pub ostat: OSTAT,
    #[doc = "0x14 - ADC Event Multiplexer Select"]
    pub emux: EMUX,
    #[doc = "0x18 - ADC Underflow Status"]
    pub ustat: USTAT,
    #[doc = "0x1c - ADC Trigger Source Select"]
    pub tssel: TSSEL,
    #[doc = "0x20 - ADC Sample Sequencer Priority"]
    pub sspri: SSPRI,
    #[doc = "0x24 - ADC Sample Phase Control"]
    pub spc: SPC,
    #[doc = "0x28 - ADC Processor Sample Sequence Initiate"]
    pub pssi: PSSI,
    _reserved0: [u8; 4usize],
    #[doc = "0x30 - ADC Sample Averaging Control"]
    pub sac: SAC,
    #[doc = "0x34 - ADC Digital Comparator Interrupt Status and Clear"]
    pub dcisc: DCISC,
    #[doc = "0x38 - ADC Control"]
    pub ctl: CTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x40 - ADC Sample Sequence Input Multiplexer Select 0"]
    pub ssmux0: SSMUX0,
    #[doc = "0x44 - ADC Sample Sequence Control 0"]
    pub ssctl0: SSCTL0,
    #[doc = "0x48 - ADC Sample Sequence Result FIFO 0"]
    pub ssfifo0: SSFIFO0,
    #[doc = "0x4c - ADC Sample Sequence FIFO 0 Status"]
    pub ssfstat0: SSFSTAT0,
    #[doc = "0x50 - ADC Sample Sequence 0 Operation"]
    pub ssop0: SSOP0,
    #[doc = "0x54 - ADC Sample Sequence 0 Digital Comparator Select"]
    pub ssdc0: SSDC0,
    #[doc = "0x58 - ADC Sample Sequence Extended Input Multiplexer Select 0"]
    pub ssemux0: SSEMUX0,
    #[doc = "0x5c - ADC Sample Sequence 0 Sample and Hold Time"]
    pub sstsh0: SSTSH0,
    #[doc = "0x60 - ADC Sample Sequence Input Multiplexer Select 1"]
    pub ssmux1: SSMUX1,
    #[doc = "0x64 - ADC Sample Sequence Control 1"]
    pub ssctl1: SSCTL1,
    #[doc = "0x68 - ADC Sample Sequence Result FIFO 1"]
    pub ssfifo1: SSFIFO1,
    #[doc = "0x6c - ADC Sample Sequence FIFO 1 Status"]
    pub ssfstat1: SSFSTAT1,
    #[doc = "0x70 - ADC Sample Sequence 1 Operation"]
    pub ssop1: SSOP1,
    #[doc = "0x74 - ADC Sample Sequence 1 Digital Comparator Select"]
    pub ssdc1: SSDC1,
    #[doc = "0x78 - ADC Sample Sequence Extended Input Multiplexer Select 1"]
    pub ssemux1: SSEMUX1,
    #[doc = "0x7c - ADC Sample Sequence 1 Sample and Hold Time"]
    pub sstsh1: SSTSH1,
    #[doc = "0x80 - ADC Sample Sequence Input Multiplexer Select 2"]
    pub ssmux2: SSMUX2,
    #[doc = "0x84 - ADC Sample Sequence Control 2"]
    pub ssctl2: SSCTL2,
    #[doc = "0x88 - ADC Sample Sequence Result FIFO 2"]
    pub ssfifo2: SSFIFO2,
    #[doc = "0x8c - ADC Sample Sequence FIFO 2 Status"]
    pub ssfstat2: SSFSTAT2,
    #[doc = "0x90 - ADC Sample Sequence 2 Operation"]
    pub ssop2: SSOP2,
    #[doc = "0x94 - ADC Sample Sequence 2 Digital Comparator Select"]
    pub ssdc2: SSDC2,
    #[doc = "0x98 - ADC Sample Sequence Extended Input Multiplexer Select 2"]
    pub ssemux2: SSEMUX2,
    #[doc = "0x9c - ADC Sample Sequence 2 Sample and Hold Time"]
    pub sstsh2: SSTSH2,
    #[doc = "0xa0 - ADC Sample Sequence Input Multiplexer Select 3"]
    pub ssmux3: SSMUX3,
    #[doc = "0xa4 - ADC Sample Sequence Control 3"]
    pub ssctl3: SSCTL3,
    #[doc = "0xa8 - ADC Sample Sequence Result FIFO 3"]
    pub ssfifo3: SSFIFO3,
    #[doc = "0xac - ADC Sample Sequence FIFO 3 Status"]
    pub ssfstat3: SSFSTAT3,
    #[doc = "0xb0 - ADC Sample Sequence 3 Operation"]
    pub ssop3: SSOP3,
    #[doc = "0xb4 - ADC Sample Sequence 3 Digital Comparator Select"]
    pub ssdc3: SSDC3,
    #[doc = "0xb8 - ADC Sample Sequence Extended Input Multiplexer Select 3"]
    pub ssemux3: SSEMUX3,
    #[doc = "0xbc - ADC Sample Sequence 3 Sample and Hold Time"]
    pub sstsh3: SSTSH3,
    _reserved2: [u8; 3136usize],
    #[doc = "0xd00 - ADC Digital Comparator Reset Initial Conditions"]
    pub dcric: DCRIC,
    _reserved3: [u8; 252usize],
    #[doc = "0xe00 - ADC Digital Comparator Control 0"]
    pub dcctl0: DCCTL0,
    #[doc = "0xe04 - ADC Digital Comparator Control 1"]
    pub dcctl1: DCCTL1,
    #[doc = "0xe08 - ADC Digital Comparator Control 2"]
    pub dcctl2: DCCTL2,
    #[doc = "0xe0c - ADC Digital Comparator Control 3"]
    pub dcctl3: DCCTL3,
    #[doc = "0xe10 - ADC Digital Comparator Control 4"]
    pub dcctl4: DCCTL4,
    #[doc = "0xe14 - ADC Digital Comparator Control 5"]
    pub dcctl5: DCCTL5,
    #[doc = "0xe18 - ADC Digital Comparator Control 6"]
    pub dcctl6: DCCTL6,
    #[doc = "0xe1c - ADC Digital Comparator Control 7"]
    pub dcctl7: DCCTL7,
    _reserved4: [u8; 32usize],
    #[doc = "0xe40 - ADC Digital Comparator Range 0"]
    pub dccmp0: DCCMP0,
    #[doc = "0xe44 - ADC Digital Comparator Range 1"]
    pub dccmp1: DCCMP1,
    #[doc = "0xe48 - ADC Digital Comparator Range 2"]
    pub dccmp2: DCCMP2,
    #[doc = "0xe4c - ADC Digital Comparator Range 3"]
    pub dccmp3: DCCMP3,
    #[doc = "0xe50 - ADC Digital Comparator Range 4"]
    pub dccmp4: DCCMP4,
    #[doc = "0xe54 - ADC Digital Comparator Range 5"]
    pub dccmp5: DCCMP5,
    #[doc = "0xe58 - ADC Digital Comparator Range 6"]
    pub dccmp6: DCCMP6,
    #[doc = "0xe5c - ADC Digital Comparator Range 7"]
    pub dccmp7: DCCMP7,
    _reserved5: [u8; 352usize],
    #[doc = "0xfc0 - ADC Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - ADC Peripheral Configuration"]
    pub pc: PC,
    #[doc = "0xfc8 - ADC Clock Configuration"]
    pub cc: CC,
}
#[doc = "ADC Active Sample Sequencer"]
pub struct ACTSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Active Sample Sequencer"]
pub mod actss;
#[doc = "ADC Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Raw Interrupt Status"]
pub mod ris;
#[doc = "ADC Interrupt Mask"]
pub struct IM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Interrupt Mask"]
pub mod im;
#[doc = "ADC Interrupt Status and Clear"]
pub struct ISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Interrupt Status and Clear"]
pub mod isc;
#[doc = "ADC Overflow Status"]
pub struct OSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Overflow Status"]
pub mod ostat;
#[doc = "ADC Event Multiplexer Select"]
pub struct EMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Event Multiplexer Select"]
pub mod emux;
#[doc = "ADC Underflow Status"]
pub struct USTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Underflow Status"]
pub mod ustat;
#[doc = "ADC Trigger Source Select"]
pub struct TSSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Trigger Source Select"]
pub mod tssel;
#[doc = "ADC Sample Sequencer Priority"]
pub struct SSPRI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequencer Priority"]
pub mod sspri;
#[doc = "ADC Sample Phase Control"]
pub struct SPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Phase Control"]
pub mod spc;
#[doc = "ADC Processor Sample Sequence Initiate"]
pub struct PSSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Processor Sample Sequence Initiate"]
pub mod pssi;
#[doc = "ADC Sample Averaging Control"]
pub struct SAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Averaging Control"]
pub mod sac;
#[doc = "ADC Digital Comparator Interrupt Status and Clear"]
pub struct DCISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Interrupt Status and Clear"]
pub mod dcisc;
#[doc = "ADC Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Control"]
pub mod ctl;
#[doc = "ADC Sample Sequence Input Multiplexer Select 0"]
pub struct SSMUX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Input Multiplexer Select 0"]
pub mod ssmux0;
#[doc = "ADC Sample Sequence Control 0"]
pub struct SSCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Control 0"]
pub mod ssctl0;
#[doc = "ADC Sample Sequence Result FIFO 0"]
pub struct SSFIFO0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Result FIFO 0"]
pub mod ssfifo0;
#[doc = "ADC Sample Sequence FIFO 0 Status"]
pub struct SSFSTAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence FIFO 0 Status"]
pub mod ssfstat0;
#[doc = "ADC Sample Sequence 0 Operation"]
pub struct SSOP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 0 Operation"]
pub mod ssop0;
#[doc = "ADC Sample Sequence 0 Digital Comparator Select"]
pub struct SSDC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 0 Digital Comparator Select"]
pub mod ssdc0;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 0"]
pub struct SSEMUX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 0"]
pub mod ssemux0;
#[doc = "ADC Sample Sequence 0 Sample and Hold Time"]
pub struct SSTSH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 0 Sample and Hold Time"]
pub mod sstsh0;
#[doc = "ADC Sample Sequence Input Multiplexer Select 1"]
pub struct SSMUX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Input Multiplexer Select 1"]
pub mod ssmux1;
#[doc = "ADC Sample Sequence Control 1"]
pub struct SSCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Control 1"]
pub mod ssctl1;
#[doc = "ADC Sample Sequence Result FIFO 1"]
pub struct SSFIFO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Result FIFO 1"]
pub mod ssfifo1;
#[doc = "ADC Sample Sequence FIFO 1 Status"]
pub struct SSFSTAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence FIFO 1 Status"]
pub mod ssfstat1;
#[doc = "ADC Sample Sequence 1 Operation"]
pub struct SSOP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 1 Operation"]
pub mod ssop1;
#[doc = "ADC Sample Sequence 1 Digital Comparator Select"]
pub struct SSDC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 1 Digital Comparator Select"]
pub mod ssdc1;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 1"]
pub struct SSEMUX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 1"]
pub mod ssemux1;
#[doc = "ADC Sample Sequence 1 Sample and Hold Time"]
pub struct SSTSH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 1 Sample and Hold Time"]
pub mod sstsh1;
#[doc = "ADC Sample Sequence Input Multiplexer Select 2"]
pub struct SSMUX2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Input Multiplexer Select 2"]
pub mod ssmux2;
#[doc = "ADC Sample Sequence Control 2"]
pub struct SSCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Control 2"]
pub mod ssctl2;
#[doc = "ADC Sample Sequence Result FIFO 2"]
pub struct SSFIFO2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Result FIFO 2"]
pub mod ssfifo2;
#[doc = "ADC Sample Sequence FIFO 2 Status"]
pub struct SSFSTAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence FIFO 2 Status"]
pub mod ssfstat2;
#[doc = "ADC Sample Sequence 2 Operation"]
pub struct SSOP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 2 Operation"]
pub mod ssop2;
#[doc = "ADC Sample Sequence 2 Digital Comparator Select"]
pub struct SSDC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 2 Digital Comparator Select"]
pub mod ssdc2;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 2"]
pub struct SSEMUX2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 2"]
pub mod ssemux2;
#[doc = "ADC Sample Sequence 2 Sample and Hold Time"]
pub struct SSTSH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 2 Sample and Hold Time"]
pub mod sstsh2;
#[doc = "ADC Sample Sequence Input Multiplexer Select 3"]
pub struct SSMUX3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Input Multiplexer Select 3"]
pub mod ssmux3;
#[doc = "ADC Sample Sequence Control 3"]
pub struct SSCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Control 3"]
pub mod ssctl3;
#[doc = "ADC Sample Sequence Result FIFO 3"]
pub struct SSFIFO3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Result FIFO 3"]
pub mod ssfifo3;
#[doc = "ADC Sample Sequence FIFO 3 Status"]
pub struct SSFSTAT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence FIFO 3 Status"]
pub mod ssfstat3;
#[doc = "ADC Sample Sequence 3 Operation"]
pub struct SSOP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 3 Operation"]
pub mod ssop3;
#[doc = "ADC Sample Sequence 3 Digital Comparator Select"]
pub struct SSDC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 3 Digital Comparator Select"]
pub mod ssdc3;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 3"]
pub struct SSEMUX3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 3"]
pub mod ssemux3;
#[doc = "ADC Sample Sequence 3 Sample and Hold Time"]
pub struct SSTSH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Sample Sequence 3 Sample and Hold Time"]
pub mod sstsh3;
#[doc = "ADC Digital Comparator Reset Initial Conditions"]
pub struct DCRIC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Reset Initial Conditions"]
pub mod dcric;
#[doc = "ADC Digital Comparator Control 0"]
pub struct DCCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Control 0"]
pub mod dcctl0;
#[doc = "ADC Digital Comparator Control 1"]
pub struct DCCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Control 1"]
pub mod dcctl1;
#[doc = "ADC Digital Comparator Control 2"]
pub struct DCCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Control 2"]
pub mod dcctl2;
#[doc = "ADC Digital Comparator Control 3"]
pub struct DCCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Control 3"]
pub mod dcctl3;
#[doc = "ADC Digital Comparator Control 4"]
pub struct DCCTL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Control 4"]
pub mod dcctl4;
#[doc = "ADC Digital Comparator Control 5"]
pub struct DCCTL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Control 5"]
pub mod dcctl5;
#[doc = "ADC Digital Comparator Control 6"]
pub struct DCCTL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Control 6"]
pub mod dcctl6;
#[doc = "ADC Digital Comparator Control 7"]
pub struct DCCTL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Control 7"]
pub mod dcctl7;
#[doc = "ADC Digital Comparator Range 0"]
pub struct DCCMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Range 0"]
pub mod dccmp0;
#[doc = "ADC Digital Comparator Range 1"]
pub struct DCCMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Range 1"]
pub mod dccmp1;
#[doc = "ADC Digital Comparator Range 2"]
pub struct DCCMP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Range 2"]
pub mod dccmp2;
#[doc = "ADC Digital Comparator Range 3"]
pub struct DCCMP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Range 3"]
pub mod dccmp3;
#[doc = "ADC Digital Comparator Range 4"]
pub struct DCCMP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Range 4"]
pub mod dccmp4;
#[doc = "ADC Digital Comparator Range 5"]
pub struct DCCMP5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Range 5"]
pub mod dccmp5;
#[doc = "ADC Digital Comparator Range 6"]
pub struct DCCMP6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Range 6"]
pub mod dccmp6;
#[doc = "ADC Digital Comparator Range 7"]
pub struct DCCMP7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Digital Comparator Range 7"]
pub mod dccmp7;
#[doc = "ADC Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Peripheral Properties"]
pub mod pp;
#[doc = "ADC Peripheral Configuration"]
pub struct PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Peripheral Configuration"]
pub mod pc;
#[doc = "ADC Clock Configuration"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Clock Configuration"]
pub mod cc;
