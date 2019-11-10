#[doc = "Reader of register WDOGTO"]
pub type R = crate::R<u32, super::WDOGTO>;
#[doc = "Writer for register WDOGTO"]
pub type W = crate::W<u32, super::WDOGTO>;
#[doc = "Register WDOGTO `reset()`'s with value 0"]
impl crate::ResetValue for super::WDOGTO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTO`"]
pub type WTO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WTO`"]
pub struct WTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `PWE`"]
pub type PWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWE`"]
pub struct PWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWE_W<'a> {
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
impl R {
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W {
        WTO_W { w: self }
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&mut self) -> PWE_W {
        PWE_W { w: self }
    }
}
