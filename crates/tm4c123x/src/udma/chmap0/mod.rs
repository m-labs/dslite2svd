#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMAP0 {
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
pub struct CH0SELR {
    bits: u8,
}
impl CH0SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH1SELR {
    bits: u8,
}
impl CH1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH2SELR {
    bits: u8,
}
impl CH2SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH3SELR {
    bits: u8,
}
impl CH3SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH4SELR {
    bits: u8,
}
impl CH4SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH5SELR {
    bits: u8,
}
impl CH5SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH6SELR {
    bits: u8,
}
impl CH6SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH7SELR {
    bits: u8,
}
impl CH7SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CH0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0SELW<'a> {
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
pub struct _CH1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1SELW<'a> {
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
pub struct _CH2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2SELW<'a> {
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
pub struct _CH3SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3SELW<'a> {
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
pub struct _CH4SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4SELW<'a> {
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
pub struct _CH5SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5SELW<'a> {
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
pub struct _CH6SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6SELW<'a> {
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
pub struct _CH7SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7SELW<'a> {
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
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline]
    pub fn ch0sel(&self) -> CH0SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH0SELR { bits }
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline]
    pub fn ch1sel(&self) -> CH1SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH1SELR { bits }
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline]
    pub fn ch2sel(&self) -> CH2SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH2SELR { bits }
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline]
    pub fn ch3sel(&self) -> CH3SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH3SELR { bits }
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline]
    pub fn ch4sel(&self) -> CH4SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH4SELR { bits }
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline]
    pub fn ch5sel(&self) -> CH5SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH5SELR { bits }
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline]
    pub fn ch6sel(&self) -> CH6SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH6SELR { bits }
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline]
    pub fn ch7sel(&self) -> CH7SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH7SELR { bits }
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
    #[doc = "Bits 0:3 - uDMA Channel 0 Source Select"]
    #[inline]
    pub fn ch0sel(&mut self) -> _CH0SELW {
        _CH0SELW { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 1 Source Select"]
    #[inline]
    pub fn ch1sel(&mut self) -> _CH1SELW {
        _CH1SELW { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 2 Source Select"]
    #[inline]
    pub fn ch2sel(&mut self) -> _CH2SELW {
        _CH2SELW { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 3 Source Select"]
    #[inline]
    pub fn ch3sel(&mut self) -> _CH3SELW {
        _CH3SELW { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 4 Source Select"]
    #[inline]
    pub fn ch4sel(&mut self) -> _CH4SELW {
        _CH4SELW { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 5 Source Select"]
    #[inline]
    pub fn ch5sel(&mut self) -> _CH5SELW {
        _CH5SELW { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 6 Source Select"]
    #[inline]
    pub fn ch6sel(&mut self) -> _CH6SELW {
        _CH6SELW { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 7 Source Select"]
    #[inline]
    pub fn ch7sel(&mut self) -> _CH7SELW {
        _CH7SELW { w: self }
    }
}
