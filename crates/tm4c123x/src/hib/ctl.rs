#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCEN`"]
pub type RTCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCEN`"]
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `HIBREQ`"]
pub type HIBREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIBREQ`"]
pub struct HIBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBREQ_W<'a> {
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
#[doc = "Reader of field `RTCWEN`"]
pub type RTCWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCWEN`"]
pub struct RTCWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCWEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PINWEN`"]
pub type PINWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINWEN`"]
pub struct PINWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PINWEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CLK32EN`"]
pub type CLK32EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK32EN`"]
pub struct CLK32EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK32EN_W<'a> {
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
#[doc = "Reader of field `VABORT`"]
pub type VABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VABORT`"]
pub struct VABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> VABORT_W<'a> {
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
#[doc = "Reader of field `VDD3ON`"]
pub type VDD3ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDD3ON`"]
pub struct VDD3ON_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD3ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `BATWKEN`"]
pub type BATWKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BATWKEN`"]
pub struct BATWKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BATWKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `BATCHK`"]
pub type BATCHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BATCHK`"]
pub struct BATCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> BATCHK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Select for Low-Battery Comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATSEL_A {
    #[doc = "0: 1.9 Volts"]
    _1_9V,
    #[doc = "1: 2.1 Volts (default)"]
    _2_1V,
    #[doc = "2: 2.3 Volts"]
    _2_3V,
    #[doc = "3: 2.5 Volts"]
    _2_5V,
}
impl From<VBATSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VBATSEL_A) -> Self {
        match variant {
            VBATSEL_A::_1_9V => 0,
            VBATSEL_A::_2_1V => 1,
            VBATSEL_A::_2_3V => 2,
            VBATSEL_A::_2_5V => 3,
        }
    }
}
#[doc = "Reader of field `VBATSEL`"]
pub type VBATSEL_R = crate::R<u8, VBATSEL_A>;
impl VBATSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATSEL_A {
        match self.bits {
            0 => VBATSEL_A::_1_9V,
            1 => VBATSEL_A::_2_1V,
            2 => VBATSEL_A::_2_3V,
            3 => VBATSEL_A::_2_5V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_9V`"]
    #[inline(always)]
    pub fn is_1_9v(&self) -> bool {
        *self == VBATSEL_A::_1_9V
    }
    #[doc = "Checks if the value of the field is `_2_1V`"]
    #[inline(always)]
    pub fn is_2_1v(&self) -> bool {
        *self == VBATSEL_A::_2_1V
    }
    #[doc = "Checks if the value of the field is `_2_3V`"]
    #[inline(always)]
    pub fn is_2_3v(&self) -> bool {
        *self == VBATSEL_A::_2_3V
    }
    #[doc = "Checks if the value of the field is `_2_5V`"]
    #[inline(always)]
    pub fn is_2_5v(&self) -> bool {
        *self == VBATSEL_A::_2_5V
    }
}
#[doc = "Write proxy for field `VBATSEL`"]
pub struct VBATSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1.9 Volts"]
    #[inline(always)]
    pub fn _1_9v(self) -> &'a mut W {
        self.variant(VBATSEL_A::_1_9V)
    }
    #[doc = "2.1 Volts (default)"]
    #[inline(always)]
    pub fn _2_1v(self) -> &'a mut W {
        self.variant(VBATSEL_A::_2_1V)
    }
    #[doc = "2.3 Volts"]
    #[inline(always)]
    pub fn _2_3v(self) -> &'a mut W {
        self.variant(VBATSEL_A::_2_3V)
    }
    #[doc = "2.5 Volts"]
    #[inline(always)]
    pub fn _2_5v(self) -> &'a mut W {
        self.variant(VBATSEL_A::_2_5V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `OSCBYP`"]
pub type OSCBYP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSCBYP`"]
pub struct OSCBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `OSCDRV`"]
pub type OSCDRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSCDRV`"]
pub struct OSCDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCDRV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `WRC`"]
pub type WRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRC`"]
pub struct WRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline(always)]
    pub fn hibreq(&self) -> HIBREQ_R {
        HIBREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline(always)]
    pub fn rtcwen(&self) -> RTCWEN_R {
        RTCWEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Wake Pin Enable"]
    #[inline(always)]
    pub fn pinwen(&self) -> PINWEN_R {
        PINWEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline(always)]
    pub fn clk32en(&self) -> CLK32EN_R {
        CLK32EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline(always)]
    pub fn vabort(&self) -> VABORT_R {
        VABORT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VDD Powered"]
    #[inline(always)]
    pub fn vdd3on(&self) -> VDD3ON_R {
        VDD3ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wake on Low Battery"]
    #[inline(always)]
    pub fn batwken(&self) -> BATWKEN_R {
        BATWKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Check Battery Status"]
    #[inline(always)]
    pub fn batchk(&self) -> BATCHK_R {
        BATCHK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Select for Low-Battery Comparator"]
    #[inline(always)]
    pub fn vbatsel(&self) -> VBATSEL_R {
        VBATSEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbyp(&self) -> OSCBYP_R {
        OSCBYP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Oscillator Drive Capability"]
    #[inline(always)]
    pub fn oscdrv(&self) -> OSCDRV_R {
        OSCDRV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write Complete/Capable"]
    #[inline(always)]
    pub fn wrc(&self) -> WRC_R {
        WRC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline(always)]
    pub fn hibreq(&mut self) -> HIBREQ_W {
        HIBREQ_W { w: self }
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline(always)]
    pub fn rtcwen(&mut self) -> RTCWEN_W {
        RTCWEN_W { w: self }
    }
    #[doc = "Bit 4 - External Wake Pin Enable"]
    #[inline(always)]
    pub fn pinwen(&mut self) -> PINWEN_W {
        PINWEN_W { w: self }
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline(always)]
    pub fn clk32en(&mut self) -> CLK32EN_W {
        CLK32EN_W { w: self }
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline(always)]
    pub fn vabort(&mut self) -> VABORT_W {
        VABORT_W { w: self }
    }
    #[doc = "Bit 8 - VDD Powered"]
    #[inline(always)]
    pub fn vdd3on(&mut self) -> VDD3ON_W {
        VDD3ON_W { w: self }
    }
    #[doc = "Bit 9 - Wake on Low Battery"]
    #[inline(always)]
    pub fn batwken(&mut self) -> BATWKEN_W {
        BATWKEN_W { w: self }
    }
    #[doc = "Bit 10 - Check Battery Status"]
    #[inline(always)]
    pub fn batchk(&mut self) -> BATCHK_W {
        BATCHK_W { w: self }
    }
    #[doc = "Bits 13:14 - Select for Low-Battery Comparator"]
    #[inline(always)]
    pub fn vbatsel(&mut self) -> VBATSEL_W {
        VBATSEL_W { w: self }
    }
    #[doc = "Bit 16 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbyp(&mut self) -> OSCBYP_W {
        OSCBYP_W { w: self }
    }
    #[doc = "Bit 17 - Oscillator Drive Capability"]
    #[inline(always)]
    pub fn oscdrv(&mut self) -> OSCDRV_W {
        OSCDRV_W { w: self }
    }
    #[doc = "Bit 31 - Write Complete/Capable"]
    #[inline(always)]
    pub fn wrc(&mut self) -> WRC_W {
        WRC_W { w: self }
    }
}
