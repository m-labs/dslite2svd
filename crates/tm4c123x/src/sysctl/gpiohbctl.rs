#[doc = "Reader of register GPIOHBCTL"]
pub type R = crate::R<u32, super::GPIOHBCTL>;
#[doc = "Writer for register GPIOHBCTL"]
pub type W = crate::W<u32, super::GPIOHBCTL>;
#[doc = "Register GPIOHBCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOHBCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTA`"]
pub type PORTA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTA`"]
pub struct PORTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTA_W<'a> {
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
#[doc = "Reader of field `PORTB`"]
pub type PORTB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTB`"]
pub struct PORTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTB_W<'a> {
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
#[doc = "Reader of field `PORTC`"]
pub type PORTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTC`"]
pub struct PORTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTC_W<'a> {
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
#[doc = "Reader of field `PORTD`"]
pub type PORTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTD`"]
pub struct PORTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTD_W<'a> {
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
#[doc = "Reader of field `PORTE`"]
pub type PORTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTE`"]
pub struct PORTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTE_W<'a> {
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
#[doc = "Reader of field `PORTF`"]
pub type PORTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTF`"]
pub struct PORTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Port A Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn porta(&self) -> PORTA_R {
        PORTA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port B Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn portb(&self) -> PORTB_R {
        PORTB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port C Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn portc(&self) -> PORTC_R {
        PORTC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port D Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn portd(&self) -> PORTD_R {
        PORTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port E Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn porte(&self) -> PORTE_R {
        PORTE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port F Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn portf(&self) -> PORTF_R {
        PORTF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn porta(&mut self) -> PORTA_W {
        PORTA_W { w: self }
    }
    #[doc = "Bit 1 - Port B Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn portb(&mut self) -> PORTB_W {
        PORTB_W { w: self }
    }
    #[doc = "Bit 2 - Port C Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn portc(&mut self) -> PORTC_W {
        PORTC_W { w: self }
    }
    #[doc = "Bit 3 - Port D Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn portd(&mut self) -> PORTD_W {
        PORTD_W { w: self }
    }
    #[doc = "Bit 4 - Port E Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn porte(&mut self) -> PORTE_W {
        PORTE_W { w: self }
    }
    #[doc = "Bit 5 - Port F Advanced High-Performance Bus"]
    #[inline(always)]
    pub fn portf(&mut self) -> PORTF_W {
        PORTF_W { w: self }
    }
}
