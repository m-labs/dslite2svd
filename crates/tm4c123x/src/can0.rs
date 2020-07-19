#[doc = r"Register block"]
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
    _reserved7: [u8; 4usize],
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
    _reserved18: [u8; 52usize],
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
    _reserved29: [u8; 84usize],
    #[doc = "0x100 - CAN Transmission Request 1"]
    pub txrq1: TXRQ1,
    #[doc = "0x104 - CAN Transmission Request 2"]
    pub txrq2: TXRQ2,
    _reserved31: [u8; 24usize],
    #[doc = "0x120 - CAN New Data 1"]
    pub nwda1: NWDA1,
    #[doc = "0x124 - CAN New Data 2"]
    pub nwda2: NWDA2,
    _reserved33: [u8; 24usize],
    #[doc = "0x140 - CAN Message 1 Interrupt Pending"]
    pub msg1int: MSG1INT,
    #[doc = "0x144 - CAN Message 2 Interrupt Pending"]
    pub msg2int: MSG2INT,
    _reserved35: [u8; 24usize],
    #[doc = "0x160 - CAN Message 1 Valid"]
    pub msg1val: MSG1VAL,
    #[doc = "0x164 - CAN Message 2 Valid"]
    pub msg2val: MSG2VAL,
}
#[doc = "CAN Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "CAN Control"]
pub mod ctl;
#[doc = "CAN Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](sts) module"]
pub type STS = crate::Reg<u32, _STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS;
#[doc = "`read()` method returns [sts::R](sts::R) reader structure"]
impl crate::Readable for STS {}
#[doc = "`write(|w| ..)` method takes [sts::W](sts::W) writer structure"]
impl crate::Writable for STS {}
#[doc = "CAN Status"]
pub mod sts;
#[doc = "CAN Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](err) module"]
pub type ERR = crate::Reg<u32, _ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR;
#[doc = "`read()` method returns [err::R](err::R) reader structure"]
impl crate::Readable for ERR {}
#[doc = "CAN Error Counter"]
pub mod err;
#[doc = "CAN Bit Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bit_](bit_) module"]
pub type BIT = crate::Reg<u32, _BIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIT;
#[doc = "`read()` method returns [bit_::R](bit_::R) reader structure"]
impl crate::Readable for BIT {}
#[doc = "`write(|w| ..)` method takes [bit_::W](bit_::W) writer structure"]
impl crate::Writable for BIT {}
#[doc = "CAN Bit Timing"]
pub mod bit_;
#[doc = "CAN Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](int) module"]
pub type INT = crate::Reg<u32, _INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT;
#[doc = "`read()` method returns [int::R](int::R) reader structure"]
impl crate::Readable for INT {}
#[doc = "CAN Interrupt"]
pub mod int;
#[doc = "CAN Test\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst](tst) module"]
pub type TST = crate::Reg<u32, _TST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TST;
#[doc = "`read()` method returns [tst::R](tst::R) reader structure"]
impl crate::Readable for TST {}
#[doc = "`write(|w| ..)` method takes [tst::W](tst::W) writer structure"]
impl crate::Writable for TST {}
#[doc = "CAN Test"]
pub mod tst;
#[doc = "CAN Baud Rate Prescaler Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brpe](brpe) module"]
pub type BRPE = crate::Reg<u32, _BRPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRPE;
#[doc = "`read()` method returns [brpe::R](brpe::R) reader structure"]
impl crate::Readable for BRPE {}
#[doc = "`write(|w| ..)` method takes [brpe::W](brpe::W) writer structure"]
impl crate::Writable for BRPE {}
#[doc = "CAN Baud Rate Prescaler Extension"]
pub mod brpe;
#[doc = "CAN IF1 Command Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1crq](if1crq) module"]
pub type IF1CRQ = crate::Reg<u32, _IF1CRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1CRQ;
#[doc = "`read()` method returns [if1crq::R](if1crq::R) reader structure"]
impl crate::Readable for IF1CRQ {}
#[doc = "`write(|w| ..)` method takes [if1crq::W](if1crq::W) writer structure"]
impl crate::Writable for IF1CRQ {}
#[doc = "CAN IF1 Command Request"]
pub mod if1crq;
#[doc = "CAN IF1 Command Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1cmsk](if1cmsk) module"]
pub type IF1CMSK = crate::Reg<u32, _IF1CMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1CMSK;
#[doc = "`read()` method returns [if1cmsk::R](if1cmsk::R) reader structure"]
impl crate::Readable for IF1CMSK {}
#[doc = "`write(|w| ..)` method takes [if1cmsk::W](if1cmsk::W) writer structure"]
impl crate::Writable for IF1CMSK {}
#[doc = "CAN IF1 Command Mask"]
pub mod if1cmsk;
#[doc = "CAN IF1 Mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1msk1](if1msk1) module"]
pub type IF1MSK1 = crate::Reg<u32, _IF1MSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1MSK1;
#[doc = "`read()` method returns [if1msk1::R](if1msk1::R) reader structure"]
impl crate::Readable for IF1MSK1 {}
#[doc = "`write(|w| ..)` method takes [if1msk1::W](if1msk1::W) writer structure"]
impl crate::Writable for IF1MSK1 {}
#[doc = "CAN IF1 Mask 1"]
pub mod if1msk1;
#[doc = "CAN IF1 Mask 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1msk2](if1msk2) module"]
pub type IF1MSK2 = crate::Reg<u32, _IF1MSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1MSK2;
#[doc = "`read()` method returns [if1msk2::R](if1msk2::R) reader structure"]
impl crate::Readable for IF1MSK2 {}
#[doc = "`write(|w| ..)` method takes [if1msk2::W](if1msk2::W) writer structure"]
impl crate::Writable for IF1MSK2 {}
#[doc = "CAN IF1 Mask 2"]
pub mod if1msk2;
#[doc = "CAN IF1 Arbitration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1arb1](if1arb1) module"]
pub type IF1ARB1 = crate::Reg<u32, _IF1ARB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1ARB1;
#[doc = "`read()` method returns [if1arb1::R](if1arb1::R) reader structure"]
impl crate::Readable for IF1ARB1 {}
#[doc = "`write(|w| ..)` method takes [if1arb1::W](if1arb1::W) writer structure"]
impl crate::Writable for IF1ARB1 {}
#[doc = "CAN IF1 Arbitration 1"]
pub mod if1arb1;
#[doc = "CAN IF1 Arbitration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1arb2](if1arb2) module"]
pub type IF1ARB2 = crate::Reg<u32, _IF1ARB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1ARB2;
#[doc = "`read()` method returns [if1arb2::R](if1arb2::R) reader structure"]
impl crate::Readable for IF1ARB2 {}
#[doc = "`write(|w| ..)` method takes [if1arb2::W](if1arb2::W) writer structure"]
impl crate::Writable for IF1ARB2 {}
#[doc = "CAN IF1 Arbitration 2"]
pub mod if1arb2;
#[doc = "CAN IF1 Message Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1mctl](if1mctl) module"]
pub type IF1MCTL = crate::Reg<u32, _IF1MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1MCTL;
#[doc = "`read()` method returns [if1mctl::R](if1mctl::R) reader structure"]
impl crate::Readable for IF1MCTL {}
#[doc = "`write(|w| ..)` method takes [if1mctl::W](if1mctl::W) writer structure"]
impl crate::Writable for IF1MCTL {}
#[doc = "CAN IF1 Message Control"]
pub mod if1mctl;
#[doc = "CAN IF1 Data A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1da1](if1da1) module"]
pub type IF1DA1 = crate::Reg<u32, _IF1DA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1DA1;
#[doc = "`read()` method returns [if1da1::R](if1da1::R) reader structure"]
impl crate::Readable for IF1DA1 {}
#[doc = "`write(|w| ..)` method takes [if1da1::W](if1da1::W) writer structure"]
impl crate::Writable for IF1DA1 {}
#[doc = "CAN IF1 Data A1"]
pub mod if1da1;
#[doc = "CAN IF1 Data A2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1da2](if1da2) module"]
pub type IF1DA2 = crate::Reg<u32, _IF1DA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1DA2;
#[doc = "`read()` method returns [if1da2::R](if1da2::R) reader structure"]
impl crate::Readable for IF1DA2 {}
#[doc = "`write(|w| ..)` method takes [if1da2::W](if1da2::W) writer structure"]
impl crate::Writable for IF1DA2 {}
#[doc = "CAN IF1 Data A2"]
pub mod if1da2;
#[doc = "CAN IF1 Data B1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1db1](if1db1) module"]
pub type IF1DB1 = crate::Reg<u32, _IF1DB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1DB1;
#[doc = "`read()` method returns [if1db1::R](if1db1::R) reader structure"]
impl crate::Readable for IF1DB1 {}
#[doc = "`write(|w| ..)` method takes [if1db1::W](if1db1::W) writer structure"]
impl crate::Writable for IF1DB1 {}
#[doc = "CAN IF1 Data B1"]
pub mod if1db1;
#[doc = "CAN IF1 Data B2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1db2](if1db2) module"]
pub type IF1DB2 = crate::Reg<u32, _IF1DB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1DB2;
#[doc = "`read()` method returns [if1db2::R](if1db2::R) reader structure"]
impl crate::Readable for IF1DB2 {}
#[doc = "`write(|w| ..)` method takes [if1db2::W](if1db2::W) writer structure"]
impl crate::Writable for IF1DB2 {}
#[doc = "CAN IF1 Data B2"]
pub mod if1db2;
#[doc = "CAN IF2 Command Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2crq](if2crq) module"]
pub type IF2CRQ = crate::Reg<u32, _IF2CRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2CRQ;
#[doc = "`read()` method returns [if2crq::R](if2crq::R) reader structure"]
impl crate::Readable for IF2CRQ {}
#[doc = "`write(|w| ..)` method takes [if2crq::W](if2crq::W) writer structure"]
impl crate::Writable for IF2CRQ {}
#[doc = "CAN IF2 Command Request"]
pub mod if2crq;
#[doc = "CAN IF2 Command Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2cmsk](if2cmsk) module"]
pub type IF2CMSK = crate::Reg<u32, _IF2CMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2CMSK;
#[doc = "`read()` method returns [if2cmsk::R](if2cmsk::R) reader structure"]
impl crate::Readable for IF2CMSK {}
#[doc = "`write(|w| ..)` method takes [if2cmsk::W](if2cmsk::W) writer structure"]
impl crate::Writable for IF2CMSK {}
#[doc = "CAN IF2 Command Mask"]
pub mod if2cmsk;
#[doc = "CAN IF2 Mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2msk1](if2msk1) module"]
pub type IF2MSK1 = crate::Reg<u32, _IF2MSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2MSK1;
#[doc = "`read()` method returns [if2msk1::R](if2msk1::R) reader structure"]
impl crate::Readable for IF2MSK1 {}
#[doc = "`write(|w| ..)` method takes [if2msk1::W](if2msk1::W) writer structure"]
impl crate::Writable for IF2MSK1 {}
#[doc = "CAN IF2 Mask 1"]
pub mod if2msk1;
#[doc = "CAN IF2 Mask 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2msk2](if2msk2) module"]
pub type IF2MSK2 = crate::Reg<u32, _IF2MSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2MSK2;
#[doc = "`read()` method returns [if2msk2::R](if2msk2::R) reader structure"]
impl crate::Readable for IF2MSK2 {}
#[doc = "`write(|w| ..)` method takes [if2msk2::W](if2msk2::W) writer structure"]
impl crate::Writable for IF2MSK2 {}
#[doc = "CAN IF2 Mask 2"]
pub mod if2msk2;
#[doc = "CAN IF2 Arbitration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2arb1](if2arb1) module"]
pub type IF2ARB1 = crate::Reg<u32, _IF2ARB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2ARB1;
#[doc = "`read()` method returns [if2arb1::R](if2arb1::R) reader structure"]
impl crate::Readable for IF2ARB1 {}
#[doc = "`write(|w| ..)` method takes [if2arb1::W](if2arb1::W) writer structure"]
impl crate::Writable for IF2ARB1 {}
#[doc = "CAN IF2 Arbitration 1"]
pub mod if2arb1;
#[doc = "CAN IF2 Arbitration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2arb2](if2arb2) module"]
pub type IF2ARB2 = crate::Reg<u32, _IF2ARB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2ARB2;
#[doc = "`read()` method returns [if2arb2::R](if2arb2::R) reader structure"]
impl crate::Readable for IF2ARB2 {}
#[doc = "`write(|w| ..)` method takes [if2arb2::W](if2arb2::W) writer structure"]
impl crate::Writable for IF2ARB2 {}
#[doc = "CAN IF2 Arbitration 2"]
pub mod if2arb2;
#[doc = "CAN IF2 Message Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2mctl](if2mctl) module"]
pub type IF2MCTL = crate::Reg<u32, _IF2MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2MCTL;
#[doc = "`read()` method returns [if2mctl::R](if2mctl::R) reader structure"]
impl crate::Readable for IF2MCTL {}
#[doc = "`write(|w| ..)` method takes [if2mctl::W](if2mctl::W) writer structure"]
impl crate::Writable for IF2MCTL {}
#[doc = "CAN IF2 Message Control"]
pub mod if2mctl;
#[doc = "CAN IF2 Data A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2da1](if2da1) module"]
pub type IF2DA1 = crate::Reg<u32, _IF2DA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2DA1;
#[doc = "`read()` method returns [if2da1::R](if2da1::R) reader structure"]
impl crate::Readable for IF2DA1 {}
#[doc = "`write(|w| ..)` method takes [if2da1::W](if2da1::W) writer structure"]
impl crate::Writable for IF2DA1 {}
#[doc = "CAN IF2 Data A1"]
pub mod if2da1;
#[doc = "CAN IF2 Data A2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2da2](if2da2) module"]
pub type IF2DA2 = crate::Reg<u32, _IF2DA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2DA2;
#[doc = "`read()` method returns [if2da2::R](if2da2::R) reader structure"]
impl crate::Readable for IF2DA2 {}
#[doc = "`write(|w| ..)` method takes [if2da2::W](if2da2::W) writer structure"]
impl crate::Writable for IF2DA2 {}
#[doc = "CAN IF2 Data A2"]
pub mod if2da2;
#[doc = "CAN IF2 Data B1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2db1](if2db1) module"]
pub type IF2DB1 = crate::Reg<u32, _IF2DB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2DB1;
#[doc = "`read()` method returns [if2db1::R](if2db1::R) reader structure"]
impl crate::Readable for IF2DB1 {}
#[doc = "`write(|w| ..)` method takes [if2db1::W](if2db1::W) writer structure"]
impl crate::Writable for IF2DB1 {}
#[doc = "CAN IF2 Data B1"]
pub mod if2db1;
#[doc = "CAN IF2 Data B2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2db2](if2db2) module"]
pub type IF2DB2 = crate::Reg<u32, _IF2DB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF2DB2;
#[doc = "`read()` method returns [if2db2::R](if2db2::R) reader structure"]
impl crate::Readable for IF2DB2 {}
#[doc = "`write(|w| ..)` method takes [if2db2::W](if2db2::W) writer structure"]
impl crate::Writable for IF2DB2 {}
#[doc = "CAN IF2 Data B2"]
pub mod if2db2;
#[doc = "CAN Transmission Request 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txrq1](txrq1) module"]
pub type TXRQ1 = crate::Reg<u32, _TXRQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXRQ1;
#[doc = "`read()` method returns [txrq1::R](txrq1::R) reader structure"]
impl crate::Readable for TXRQ1 {}
#[doc = "CAN Transmission Request 1"]
pub mod txrq1;
#[doc = "CAN Transmission Request 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txrq2](txrq2) module"]
pub type TXRQ2 = crate::Reg<u32, _TXRQ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXRQ2;
#[doc = "`read()` method returns [txrq2::R](txrq2::R) reader structure"]
impl crate::Readable for TXRQ2 {}
#[doc = "CAN Transmission Request 2"]
pub mod txrq2;
#[doc = "CAN New Data 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwda1](nwda1) module"]
pub type NWDA1 = crate::Reg<u32, _NWDA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWDA1;
#[doc = "`read()` method returns [nwda1::R](nwda1::R) reader structure"]
impl crate::Readable for NWDA1 {}
#[doc = "CAN New Data 1"]
pub mod nwda1;
#[doc = "CAN New Data 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nwda2](nwda2) module"]
pub type NWDA2 = crate::Reg<u32, _NWDA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NWDA2;
#[doc = "`read()` method returns [nwda2::R](nwda2::R) reader structure"]
impl crate::Readable for NWDA2 {}
#[doc = "CAN New Data 2"]
pub mod nwda2;
#[doc = "CAN Message 1 Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg1int](msg1int) module"]
pub type MSG1INT = crate::Reg<u32, _MSG1INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSG1INT;
#[doc = "`read()` method returns [msg1int::R](msg1int::R) reader structure"]
impl crate::Readable for MSG1INT {}
#[doc = "CAN Message 1 Interrupt Pending"]
pub mod msg1int;
#[doc = "CAN Message 2 Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg2int](msg2int) module"]
pub type MSG2INT = crate::Reg<u32, _MSG2INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSG2INT;
#[doc = "`read()` method returns [msg2int::R](msg2int::R) reader structure"]
impl crate::Readable for MSG2INT {}
#[doc = "CAN Message 2 Interrupt Pending"]
pub mod msg2int;
#[doc = "CAN Message 1 Valid\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg1val](msg1val) module"]
pub type MSG1VAL = crate::Reg<u32, _MSG1VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSG1VAL;
#[doc = "`read()` method returns [msg1val::R](msg1val::R) reader structure"]
impl crate::Readable for MSG1VAL {}
#[doc = "CAN Message 1 Valid"]
pub mod msg1val;
#[doc = "CAN Message 2 Valid\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg2val](msg2val) module"]
pub type MSG2VAL = crate::Reg<u32, _MSG2VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSG2VAL;
#[doc = "`read()` method returns [msg2val::R](msg2val::R) reader structure"]
impl crate::Readable for MSG2VAL {}
#[doc = "CAN Message 2 Valid"]
pub mod msg2val;
