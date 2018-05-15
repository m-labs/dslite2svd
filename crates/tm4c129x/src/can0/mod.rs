#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Control"]
    pub ctl: CTL,
    #[doc = "0x04 - CAN Status"]
    pub sts: STS,
    #[doc = "0x08 - CAN Error Counter"]
    pub err: ERR,
    #[doc = "0x0c - CAN Bit Timing"]
    pub bit_: BIT,
    #[doc = "0x10 - CAN Interrupt"]
    pub int: INT,
    #[doc = "0x14 - CAN Test"]
    pub tst: TST,
    #[doc = "0x18 - CAN Baud Rate Prescaler Extension"]
    pub brpe: BRPE,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - CAN IF1 Command Request"]
    pub if1crq: IF1CRQ,
    #[doc = "0x24 - CAN IF1 Command Mask"]
    pub if1cmsk: IF1CMSK,
    #[doc = "0x28 - CAN IF1 Mask 1"]
    pub if1msk1: IF1MSK1,
    #[doc = "0x2c - CAN IF1 Mask 2"]
    pub if1msk2: IF1MSK2,
    #[doc = "0x30 - CAN IF1 Arbitration 1"]
    pub if1arb1: IF1ARB1,
    #[doc = "0x34 - CAN IF1 Arbitration 2"]
    pub if1arb2: IF1ARB2,
    #[doc = "0x38 - CAN IF1 Message Control"]
    pub if1mctl: IF1MCTL,
    #[doc = "0x3c - CAN IF1 Data A1"]
    pub if1da1: IF1DA1,
    #[doc = "0x40 - CAN IF1 Data A2"]
    pub if1da2: IF1DA2,
    #[doc = "0x44 - CAN IF1 Data B1"]
    pub if1db1: IF1DB1,
    #[doc = "0x48 - CAN IF1 Data B2"]
    pub if1db2: IF1DB2,
    _reserved1: [u8; 52usize],
    #[doc = "0x80 - CAN IF2 Command Request"]
    pub if2crq: IF2CRQ,
    #[doc = "0x84 - CAN IF2 Command Mask"]
    pub if2cmsk: IF2CMSK,
    #[doc = "0x88 - CAN IF2 Mask 1"]
    pub if2msk1: IF2MSK1,
    #[doc = "0x8c - CAN IF2 Mask 2"]
    pub if2msk2: IF2MSK2,
    #[doc = "0x90 - CAN IF2 Arbitration 1"]
    pub if2arb1: IF2ARB1,
    #[doc = "0x94 - CAN IF2 Arbitration 2"]
    pub if2arb2: IF2ARB2,
    #[doc = "0x98 - CAN IF2 Message Control"]
    pub if2mctl: IF2MCTL,
    #[doc = "0x9c - CAN IF2 Data A1"]
    pub if2da1: IF2DA1,
    #[doc = "0xa0 - CAN IF2 Data A2"]
    pub if2da2: IF2DA2,
    #[doc = "0xa4 - CAN IF2 Data B1"]
    pub if2db1: IF2DB1,
    #[doc = "0xa8 - CAN IF2 Data B2"]
    pub if2db2: IF2DB2,
    _reserved2: [u8; 84usize],
    #[doc = "0x100 - CAN Transmission Request 1"]
    pub txrq1: TXRQ1,
    #[doc = "0x104 - CAN Transmission Request 2"]
    pub txrq2: TXRQ2,
    _reserved3: [u8; 24usize],
    #[doc = "0x120 - CAN New Data 1"]
    pub nwda1: NWDA1,
    #[doc = "0x124 - CAN New Data 2"]
    pub nwda2: NWDA2,
    _reserved4: [u8; 24usize],
    #[doc = "0x140 - CAN Message 1 Interrupt Pending"]
    pub msg1int: MSG1INT,
    #[doc = "0x144 - CAN Message 2 Interrupt Pending"]
    pub msg2int: MSG2INT,
    _reserved5: [u8; 24usize],
    #[doc = "0x160 - CAN Message 1 Valid"]
    pub msg1val: MSG1VAL,
    #[doc = "0x164 - CAN Message 2 Valid"]
    pub msg2val: MSG2VAL,
}
#[doc = "CAN Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Control"]
pub mod ctl;
#[doc = "CAN Status"]
pub struct STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Status"]
pub mod sts;
#[doc = "CAN Error Counter"]
pub struct ERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Error Counter"]
pub mod err;
#[doc = "CAN Bit Timing"]
pub struct BIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Bit Timing"]
pub mod bit_;
#[doc = "CAN Interrupt"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Interrupt"]
pub mod int;
#[doc = "CAN Test"]
pub struct TST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Test"]
pub mod tst;
#[doc = "CAN Baud Rate Prescaler Extension"]
pub struct BRPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Baud Rate Prescaler Extension"]
pub mod brpe;
#[doc = "CAN IF1 Command Request"]
pub struct IF1CRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Command Request"]
pub mod if1crq;
#[doc = "CAN IF1 Command Mask"]
pub struct IF1CMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Command Mask"]
pub mod if1cmsk;
#[doc = "CAN IF1 Mask 1"]
pub struct IF1MSK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Mask 1"]
pub mod if1msk1;
#[doc = "CAN IF1 Mask 2"]
pub struct IF1MSK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Mask 2"]
pub mod if1msk2;
#[doc = "CAN IF1 Arbitration 1"]
pub struct IF1ARB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Arbitration 1"]
pub mod if1arb1;
#[doc = "CAN IF1 Arbitration 2"]
pub struct IF1ARB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Arbitration 2"]
pub mod if1arb2;
#[doc = "CAN IF1 Message Control"]
pub struct IF1MCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Message Control"]
pub mod if1mctl;
#[doc = "CAN IF1 Data A1"]
pub struct IF1DA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Data A1"]
pub mod if1da1;
#[doc = "CAN IF1 Data A2"]
pub struct IF1DA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Data A2"]
pub mod if1da2;
#[doc = "CAN IF1 Data B1"]
pub struct IF1DB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Data B1"]
pub mod if1db1;
#[doc = "CAN IF1 Data B2"]
pub struct IF1DB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF1 Data B2"]
pub mod if1db2;
#[doc = "CAN IF2 Command Request"]
pub struct IF2CRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Command Request"]
pub mod if2crq;
#[doc = "CAN IF2 Command Mask"]
pub struct IF2CMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Command Mask"]
pub mod if2cmsk;
#[doc = "CAN IF2 Mask 1"]
pub struct IF2MSK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Mask 1"]
pub mod if2msk1;
#[doc = "CAN IF2 Mask 2"]
pub struct IF2MSK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Mask 2"]
pub mod if2msk2;
#[doc = "CAN IF2 Arbitration 1"]
pub struct IF2ARB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Arbitration 1"]
pub mod if2arb1;
#[doc = "CAN IF2 Arbitration 2"]
pub struct IF2ARB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Arbitration 2"]
pub mod if2arb2;
#[doc = "CAN IF2 Message Control"]
pub struct IF2MCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Message Control"]
pub mod if2mctl;
#[doc = "CAN IF2 Data A1"]
pub struct IF2DA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Data A1"]
pub mod if2da1;
#[doc = "CAN IF2 Data A2"]
pub struct IF2DA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Data A2"]
pub mod if2da2;
#[doc = "CAN IF2 Data B1"]
pub struct IF2DB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Data B1"]
pub mod if2db1;
#[doc = "CAN IF2 Data B2"]
pub struct IF2DB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN IF2 Data B2"]
pub mod if2db2;
#[doc = "CAN Transmission Request 1"]
pub struct TXRQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Transmission Request 1"]
pub mod txrq1;
#[doc = "CAN Transmission Request 2"]
pub struct TXRQ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Transmission Request 2"]
pub mod txrq2;
#[doc = "CAN New Data 1"]
pub struct NWDA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN New Data 1"]
pub mod nwda1;
#[doc = "CAN New Data 2"]
pub struct NWDA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN New Data 2"]
pub mod nwda2;
#[doc = "CAN Message 1 Interrupt Pending"]
pub struct MSG1INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Message 1 Interrupt Pending"]
pub mod msg1int;
#[doc = "CAN Message 2 Interrupt Pending"]
pub struct MSG2INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Message 2 Interrupt Pending"]
pub mod msg2int;
#[doc = "CAN Message 1 Valid"]
pub struct MSG1VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Message 1 Valid"]
pub mod msg1val;
#[doc = "CAN Message 2 Valid"]
pub struct MSG2VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Message 2 Valid"]
pub mod msg2val;
