#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMAP1 {
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
pub struct CH8SELR {
    bits: u8,
}
impl CH8SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH9SELR {
    bits: u8,
}
impl CH9SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH10SELR {
    bits: u8,
}
impl CH10SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH11SELR {
    bits: u8,
}
impl CH11SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH12SELR {
    bits: u8,
}
impl CH12SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH13SELR {
    bits: u8,
}
impl CH13SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH14SELR {
    bits: u8,
}
impl CH14SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH15SELR {
    bits: u8,
}
impl CH15SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CH8SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8SELW<'a> {
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
pub struct _CH9SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9SELW<'a> {
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
pub struct _CH10SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10SELW<'a> {
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
pub struct _CH11SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11SELW<'a> {
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
pub struct _CH12SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH12SELW<'a> {
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
pub struct _CH13SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH13SELW<'a> {
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
pub struct _CH14SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH14SELW<'a> {
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
pub struct _CH15SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH15SELW<'a> {
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
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline]
    pub fn ch8sel(&self) -> CH8SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH8SELR { bits }
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline]
    pub fn ch9sel(&self) -> CH9SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH9SELR { bits }
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline]
    pub fn ch10sel(&self) -> CH10SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH10SELR { bits }
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline]
    pub fn ch11sel(&self) -> CH11SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH11SELR { bits }
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline]
    pub fn ch12sel(&self) -> CH12SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH12SELR { bits }
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline]
    pub fn ch13sel(&self) -> CH13SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH13SELR { bits }
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline]
    pub fn ch14sel(&self) -> CH14SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH14SELR { bits }
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline]
    pub fn ch15sel(&self) -> CH15SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH15SELR { bits }
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
    #[doc = "Bits 0:3 - uDMA Channel 8 Source Select"]
    #[inline]
    pub fn ch8sel(&mut self) -> _CH8SELW {
        _CH8SELW { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 9 Source Select"]
    #[inline]
    pub fn ch9sel(&mut self) -> _CH9SELW {
        _CH9SELW { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 10 Source Select"]
    #[inline]
    pub fn ch10sel(&mut self) -> _CH10SELW {
        _CH10SELW { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 11 Source Select"]
    #[inline]
    pub fn ch11sel(&mut self) -> _CH11SELW {
        _CH11SELW { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 12 Source Select"]
    #[inline]
    pub fn ch12sel(&mut self) -> _CH12SELW {
        _CH12SELW { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 13 Source Select"]
    #[inline]
    pub fn ch13sel(&mut self) -> _CH13SELW {
        _CH13SELW { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 14 Source Select"]
    #[inline]
    pub fn ch14sel(&mut self) -> _CH14SELW {
        _CH14SELW { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 15 Source Select"]
    #[inline]
    pub fn ch15sel(&mut self) -> _CH15SELW {
        _CH15SELW { w: self }
    }
}
