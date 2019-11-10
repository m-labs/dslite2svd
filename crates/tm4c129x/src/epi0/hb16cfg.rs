#[doc = "Reader of register HB16CFG"]
pub type R = crate::R<u32, super::HB16CFG>;
#[doc = "Writer for register HB16CFG"]
pub type W = crate::W<u32, super::HB16CFG>;
#[doc = "Register HB16CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::HB16CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Host Bus Sub-Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: ADMUX - AD\\[15:0\\]"]
    ADMUX,
    #[doc = "1: ADNONMUX - D\\[15:0\\]"]
    ADNMUX,
    #[doc = "2: Continuous Read - D\\[15:0\\]"]
    SRAM,
    #[doc = "3: XFIFO - D\\[15:0\\]"]
    XFIFO,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::ADMUX => 0,
            MODE_A::ADNMUX => 1,
            MODE_A::SRAM => 2,
            MODE_A::XFIFO => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::ADMUX,
            1 => MODE_A::ADNMUX,
            2 => MODE_A::SRAM,
            3 => MODE_A::XFIFO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADMUX`"]
    #[inline(always)]
    pub fn is_admux(&self) -> bool {
        *self == MODE_A::ADMUX
    }
    #[doc = "Checks if the value of the field is `ADNMUX`"]
    #[inline(always)]
    pub fn is_adnmux(&self) -> bool {
        *self == MODE_A::ADNMUX
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MODE_A::SRAM
    }
    #[doc = "Checks if the value of the field is `XFIFO`"]
    #[inline(always)]
    pub fn is_xfifo(&self) -> bool {
        *self == MODE_A::XFIFO
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADMUX - AD\\[15:0\\]"]
    #[inline(always)]
    pub fn admux(self) -> &'a mut W {
        self.variant(MODE_A::ADMUX)
    }
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    #[inline(always)]
    pub fn adnmux(self) -> &'a mut W {
        self.variant(MODE_A::ADNMUX)
    }
    #[doc = "Continuous Read - D\\[15:0\\]"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MODE_A::SRAM)
    }
    #[doc = "XFIFO - D\\[15:0\\]"]
    #[inline(always)]
    pub fn xfifo(self) -> &'a mut W {
        self.variant(MODE_A::XFIFO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `BSEL`"]
pub type BSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSEL`"]
pub struct BSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BSEL_W<'a> {
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
#[doc = "Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDWS_A {
    #[doc = "0: Active RDn is 2 EPI clocks"]
    _2,
    #[doc = "1: Active RDn is 4 EPI clocks"]
    _4,
    #[doc = "2: Active RDn is 6 EPI clocks"]
    _6,
    #[doc = "3: Active RDn is 8 EPI clocks"]
    _8,
}
impl From<RDWS_A> for u8 {
    #[inline(always)]
    fn from(variant: RDWS_A) -> Self {
        match variant {
            RDWS_A::_2 => 0,
            RDWS_A::_4 => 1,
            RDWS_A::_6 => 2,
            RDWS_A::_8 => 3,
        }
    }
}
#[doc = "Reader of field `RDWS`"]
pub type RDWS_R = crate::R<u8, RDWS_A>;
impl RDWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDWS_A {
        match self.bits {
            0 => RDWS_A::_2,
            1 => RDWS_A::_4,
            2 => RDWS_A::_6,
            3 => RDWS_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RDWS_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == RDWS_A::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == RDWS_A::_6
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == RDWS_A::_8
    }
}
#[doc = "Write proxy for field `RDWS`"]
pub struct RDWS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDWS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RDWS_A::_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(RDWS_A::_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(RDWS_A::_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(RDWS_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Write Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRWS_A {
    #[doc = "0: Active WRn is 2 EPI clocks"]
    _2,
    #[doc = "1: Active WRn is 4 EPI clocks"]
    _4,
    #[doc = "2: Active WRn is 6 EPI clocks"]
    _6,
    #[doc = "3: Active WRn is 8 EPI clocks"]
    _8,
}
impl From<WRWS_A> for u8 {
    #[inline(always)]
    fn from(variant: WRWS_A) -> Self {
        match variant {
            WRWS_A::_2 => 0,
            WRWS_A::_4 => 1,
            WRWS_A::_6 => 2,
            WRWS_A::_8 => 3,
        }
    }
}
#[doc = "Reader of field `WRWS`"]
pub type WRWS_R = crate::R<u8, WRWS_A>;
impl WRWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRWS_A {
        match self.bits {
            0 => WRWS_A::_2,
            1 => WRWS_A::_4,
            2 => WRWS_A::_6,
            3 => WRWS_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == WRWS_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == WRWS_A::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == WRWS_A::_6
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == WRWS_A::_8
    }
}
#[doc = "Write proxy for field `WRWS`"]
pub struct WRWS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRWS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WRWS_A::_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(WRWS_A::_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(WRWS_A::_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(WRWS_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MAXWAIT`"]
pub type MAXWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXWAIT`"]
pub struct MAXWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
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
#[doc = "Reader of field `RDCRE`"]
pub type RDCRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDCRE`"]
pub struct RDCRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCRE_W<'a> {
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
#[doc = "Reader of field `WRCRE`"]
pub type WRCRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRCRE`"]
pub struct WRCRE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRCRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `ALEHIGH`"]
pub type ALEHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALEHIGH`"]
pub struct ALEHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEHIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RDHIGH`"]
pub type RDHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDHIGH`"]
pub struct RDHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> RDHIGH_W<'a> {
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
#[doc = "Reader of field `WRHIGH`"]
pub type WRHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRHIGH`"]
pub struct WRHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> WRHIGH_W<'a> {
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
#[doc = "Reader of field `XFEEN`"]
pub type XFEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XFEEN`"]
pub struct XFEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XFEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `XFFEN`"]
pub type XFFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XFFEN`"]
pub struct XFFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XFFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `IRDYINV`"]
pub type IRDYINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRDYINV`"]
pub struct IRDYINV_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDYINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RDYEN`"]
pub type RDYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDYEN`"]
pub struct RDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDYEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `CLKINV`"]
pub type CLKINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKINV`"]
pub struct CLKINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `CLKGATEI`"]
pub type CLKGATEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGATEI`"]
pub struct CLKGATEI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATEI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CLKGATE`"]
pub type CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGATE`"]
pub struct CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_W<'a> {
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
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Byte Select Configuration"]
    #[inline(always)]
    pub fn bsel(&self) -> BSEL_R {
        BSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn rdws(&self) -> RDWS_R {
        RDWS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn wrws(&self) -> WRWS_R {
        WRWS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn maxwait(&self) -> MAXWAIT_R {
        MAXWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst Mode"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn rdcre(&self) -> RDCRE_R {
        RDCRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn wrcre(&self) -> WRCRE_R {
        WRCRE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn alehigh(&self) -> ALEHIGH_R {
        ALEHIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn rdhigh(&self) -> RDHIGH_R {
        RDHIGH_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn wrhigh(&self) -> WRHIGH_R {
        WRHIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn xfeen(&self) -> XFEEN_R {
        XFEEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn xffen(&self) -> XFFEN_R {
        XFFEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn irdyinv(&self) -> IRDYINV_R {
        IRDYINV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn rdyen(&self) -> RDYEN_R {
        RDYEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn clkinv(&self) -> CLKINV_R {
        CLKINV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Clock Gated Idle"]
    #[inline(always)]
    pub fn clkgatei(&self) -> CLKGATEI_R {
        CLKGATEI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 2 - Byte Select Configuration"]
    #[inline(always)]
    pub fn bsel(&mut self) -> BSEL_W {
        BSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn rdws(&mut self) -> RDWS_W {
        RDWS_W { w: self }
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn wrws(&mut self) -> WRWS_W {
        WRWS_W { w: self }
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn maxwait(&mut self) -> MAXWAIT_W {
        MAXWAIT_W { w: self }
    }
    #[doc = "Bit 16 - Burst Mode"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 17 - PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn rdcre(&mut self) -> RDCRE_W {
        RDCRE_W { w: self }
    }
    #[doc = "Bit 18 - PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn wrcre(&mut self) -> WRCRE_W {
        WRCRE_W { w: self }
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn alehigh(&mut self) -> ALEHIGH_W {
        ALEHIGH_W { w: self }
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn rdhigh(&mut self) -> RDHIGH_W {
        RDHIGH_W { w: self }
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn wrhigh(&mut self) -> WRHIGH_W {
        WRHIGH_W { w: self }
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn xfeen(&mut self) -> XFEEN_W {
        XFEEN_W { w: self }
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn xffen(&mut self) -> XFFEN_W {
        XFFEN_W { w: self }
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn irdyinv(&mut self) -> IRDYINV_W {
        IRDYINV_W { w: self }
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn rdyen(&mut self) -> RDYEN_W {
        RDYEN_W { w: self }
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn clkinv(&mut self) -> CLKINV_W {
        CLKINV_W { w: self }
    }
    #[doc = "Bit 30 - Clock Gated Idle"]
    #[inline(always)]
    pub fn clkgatei(&mut self) -> CLKGATEI_W {
        CLKGATEI_W { w: self }
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
}
