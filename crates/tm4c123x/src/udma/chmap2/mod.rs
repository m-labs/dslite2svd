#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMAP2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CH16SELR {
    bits: u8,
}
impl CH16SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH17SELR {
    bits: u8,
}
impl CH17SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH18SELR {
    bits: u8,
}
impl CH18SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH19SELR {
    bits: u8,
}
impl CH19SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH20SELR {
    bits: u8,
}
impl CH20SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH21SELR {
    bits: u8,
}
impl CH21SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH22SELR {
    bits: u8,
}
impl CH22SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH23SELR {
    bits: u8,
}
impl CH23SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CH16SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH16SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH17SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH17SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH18SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH18SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH19SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH19SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH20SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH20SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH21SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH21SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH22SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH22SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH23SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH23SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline]
    pub fn ch16sel(&self) -> CH16SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH16SELR { bits }
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline]
    pub fn ch17sel(&self) -> CH17SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH17SELR { bits }
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline]
    pub fn ch18sel(&self) -> CH18SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH18SELR { bits }
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline]
    pub fn ch19sel(&self) -> CH19SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH19SELR { bits }
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline]
    pub fn ch20sel(&self) -> CH20SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH20SELR { bits }
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline]
    pub fn ch21sel(&self) -> CH21SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH21SELR { bits }
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline]
    pub fn ch22sel(&self) -> CH22SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH22SELR { bits }
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline]
    pub fn ch23sel(&self) -> CH23SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH23SELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - uDMA Channel 16 Source Select"]
    #[inline]
    pub fn ch16sel(&mut self) -> _CH16SELW {
        _CH16SELW { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 17 Source Select"]
    #[inline]
    pub fn ch17sel(&mut self) -> _CH17SELW {
        _CH17SELW { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 18 Source Select"]
    #[inline]
    pub fn ch18sel(&mut self) -> _CH18SELW {
        _CH18SELW { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 19 Source Select"]
    #[inline]
    pub fn ch19sel(&mut self) -> _CH19SELW {
        _CH19SELW { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 20 Source Select"]
    #[inline]
    pub fn ch20sel(&mut self) -> _CH20SELW {
        _CH20SELW { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 21 Source Select"]
    #[inline]
    pub fn ch21sel(&mut self) -> _CH21SELW {
        _CH21SELW { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 22 Source Select"]
    #[inline]
    pub fn ch22sel(&mut self) -> _CH22SELW {
        _CH22SELW { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 23 Source Select"]
    #[inline]
    pub fn ch23sel(&mut self) -> _CH23SELW {
        _CH23SELW { w: self }
    }
}
