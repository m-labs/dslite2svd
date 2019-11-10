#[doc = "Reader of register RXCSRH7"]
pub type R = crate::R<u8, super::RXCSRH7>;
#[doc = "Writer for register RXCSRH7"]
pub type W = crate::W<u8, super::RXCSRH7>;
#[doc = "Register RXCSRH7 `reset()`'s with value 0"]
impl crate::ResetValue for super::RXCSRH7 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DTWE`"]
pub type DTWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTWE`"]
pub struct DTWE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMAMOD`"]
pub type DMAMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMOD`"]
pub struct DMAMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PIDERR`"]
pub type PIDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIDERR`"]
pub struct PIDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DISNYET`"]
pub type DISNYET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISNYET`"]
pub struct DISNYET_W<'a> {
    w: &'a mut W,
}
impl<'a> DISNYET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ISO`"]
pub type ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISO`"]
pub struct ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `AUTORQ`"]
pub type AUTORQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTORQ`"]
pub struct AUTORQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTORQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `AUTOCL`"]
pub type AUTOCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCL`"]
pub struct AUTOCL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn dtwe(&self) -> DTWE_R {
        DTWE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn dmamod(&self) -> DMAMOD_R {
        DMAMOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn piderr(&self) -> PIDERR_R {
        PIDERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    pub fn disnyet(&self) -> DISNYET_R {
        DISNYET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn autorq(&self) -> AUTORQ_R {
        AUTORQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn autocl(&self) -> AUTOCL_R {
        AUTOCL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn dtwe(&mut self) -> DTWE_W {
        DTWE_W { w: self }
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn dmamod(&mut self) -> DMAMOD_W {
        DMAMOD_W { w: self }
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn piderr(&mut self) -> PIDERR_W {
        PIDERR_W { w: self }
    }
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    pub fn disnyet(&mut self) -> DISNYET_W {
        DISNYET_W { w: self }
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W {
        ISO_W { w: self }
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn autorq(&mut self) -> AUTORQ_W {
        AUTORQ_W { w: self }
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn autocl(&mut self) -> AUTOCL_W {
        AUTOCL_W { w: self }
    }
}
