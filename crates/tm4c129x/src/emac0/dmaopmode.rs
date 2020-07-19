#[doc = "Reader of register DMAOPMODE"]
pub type R = crate::R<u32, super::DMAOPMODE>;
#[doc = "Writer for register DMAOPMODE"]
pub type W = crate::W<u32, super::DMAOPMODE>;
#[doc = "Register DMAOPMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAOPMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OSF`"]
pub type OSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSF`"]
pub struct OSF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Receive Threshold Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTC_A {
    #[doc = "0: 64 bytes"]
    _64 = 0,
    #[doc = "1: 32 bytes"]
    _32 = 1,
    #[doc = "2: 96 bytes"]
    _96 = 2,
    #[doc = "3: 128 bytes"]
    _128 = 3,
}
impl From<RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<u8, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            0 => RTC_A::_64,
            1 => RTC_A::_32,
            2 => RTC_A::_96,
            3 => RTC_A::_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == RTC_A::_64
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == RTC_A::_32
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        *self == RTC_A::_96
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == RTC_A::_128
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(RTC_A::_64)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(RTC_A::_32)
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn _96(self) -> &'a mut W {
        self.variant(RTC_A::_96)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(RTC_A::_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `DGF`"]
pub type DGF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DGF`"]
pub struct DGF_W<'a> {
    w: &'a mut W,
}
impl<'a> DGF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FUF`"]
pub type FUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUF`"]
pub struct FUF_W<'a> {
    w: &'a mut W,
}
impl<'a> FUF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FEF`"]
pub type FEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEF`"]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ST`"]
pub type ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ST`"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Transmit Threshold Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TTC_A {
    #[doc = "0: 64 bytes"]
    _64 = 0,
    #[doc = "1: 128 bytes"]
    _128 = 1,
    #[doc = "2: 192 bytes"]
    _192 = 2,
    #[doc = "3: 256 bytes"]
    _256 = 3,
    #[doc = "4: 40 bytes"]
    _40 = 4,
    #[doc = "5: 32 bytes"]
    _32 = 5,
    #[doc = "6: 24 bytes"]
    _24 = 6,
    #[doc = "7: 16 bytes"]
    _16 = 7,
}
impl From<TTC_A> for u8 {
    #[inline(always)]
    fn from(variant: TTC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TTC`"]
pub type TTC_R = crate::R<u8, TTC_A>;
impl TTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTC_A {
        match self.bits {
            0 => TTC_A::_64,
            1 => TTC_A::_128,
            2 => TTC_A::_192,
            3 => TTC_A::_256,
            4 => TTC_A::_40,
            5 => TTC_A::_32,
            6 => TTC_A::_24,
            7 => TTC_A::_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == TTC_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == TTC_A::_128
    }
    #[doc = "Checks if the value of the field is `_192`"]
    #[inline(always)]
    pub fn is_192(&self) -> bool {
        *self == TTC_A::_192
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == TTC_A::_256
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == TTC_A::_40
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == TTC_A::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == TTC_A::_24
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == TTC_A::_16
    }
}
#[doc = "Write proxy for field `TTC`"]
pub struct TTC_W<'a> {
    w: &'a mut W,
}
impl<'a> TTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TTC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(TTC_A::_64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(TTC_A::_128)
    }
    #[doc = "192 bytes"]
    #[inline(always)]
    pub fn _192(self) -> &'a mut W {
        self.variant(TTC_A::_192)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(TTC_A::_256)
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn _40(self) -> &'a mut W {
        self.variant(TTC_A::_40)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(TTC_A::_32)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(TTC_A::_24)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(TTC_A::_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `FTF`"]
pub type FTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTF`"]
pub struct FTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FTF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TSF`"]
pub type TSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSF`"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DFF`"]
pub type DFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFF`"]
pub struct DFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RSF`"]
pub type RSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSF`"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT`"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Drop Giant Frame Enable"]
    #[inline(always)]
    pub fn dgf(&self) -> DGF_R {
        DGF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn fuf(&self) -> FUF_R {
        FUF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn dff(&self) -> DFF_R {
        DFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W {
        OSF_W { w: self }
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 5 - Drop Giant Frame Enable"]
    #[inline(always)]
    pub fn dgf(&mut self) -> DGF_W {
        DGF_W { w: self }
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn fuf(&mut self) -> FUF_W {
        FUF_W { w: self }
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W {
        TTC_W { w: self }
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W {
        FTF_W { w: self }
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn dff(&mut self) -> DFF_W {
        DFF_W { w: self }
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
}
