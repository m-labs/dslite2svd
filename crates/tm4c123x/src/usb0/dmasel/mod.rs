#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMASEL {
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
pub struct DMAARXR {
    bits: u8,
}
impl DMAARXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMAATXR {
    bits: u8,
}
impl DMAATXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMABRXR {
    bits: u8,
}
impl DMABRXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMABTXR {
    bits: u8,
}
impl DMABTXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMACRXR {
    bits: u8,
}
impl DMACRXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMACTXR {
    bits: u8,
}
impl DMACTXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DMAARXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAARXW<'a> {
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
pub struct _DMAATXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAATXW<'a> {
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
pub struct _DMABRXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMABRXW<'a> {
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
pub struct _DMABTXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMABTXW<'a> {
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
pub struct _DMACRXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMACRXW<'a> {
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
pub struct _DMACTXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMACTXW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - DMA A RX Select"]
    #[inline]
    pub fn dmaarx(&self) -> DMAARXR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMAARXR { bits }
    }
    #[doc = "Bits 4:7 - DMA A TX Select"]
    #[inline]
    pub fn dmaatx(&self) -> DMAATXR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMAATXR { bits }
    }
    #[doc = "Bits 8:11 - DMA B RX Select"]
    #[inline]
    pub fn dmabrx(&self) -> DMABRXR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMABRXR { bits }
    }
    #[doc = "Bits 12:15 - DMA B TX Select"]
    #[inline]
    pub fn dmabtx(&self) -> DMABTXR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMABTXR { bits }
    }
    #[doc = "Bits 16:19 - DMA C RX Select"]
    #[inline]
    pub fn dmacrx(&self) -> DMACRXR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMACRXR { bits }
    }
    #[doc = "Bits 20:23 - DMA C TX Select"]
    #[inline]
    pub fn dmactx(&self) -> DMACTXR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMACTXR { bits }
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
    #[doc = "Bits 0:3 - DMA A RX Select"]
    #[inline]
    pub fn dmaarx(&mut self) -> _DMAARXW {
        _DMAARXW { w: self }
    }
    #[doc = "Bits 4:7 - DMA A TX Select"]
    #[inline]
    pub fn dmaatx(&mut self) -> _DMAATXW {
        _DMAATXW { w: self }
    }
    #[doc = "Bits 8:11 - DMA B RX Select"]
    #[inline]
    pub fn dmabrx(&mut self) -> _DMABRXW {
        _DMABRXW { w: self }
    }
    #[doc = "Bits 12:15 - DMA B TX Select"]
    #[inline]
    pub fn dmabtx(&mut self) -> _DMABTXW {
        _DMABTXW { w: self }
    }
    #[doc = "Bits 16:19 - DMA C RX Select"]
    #[inline]
    pub fn dmacrx(&mut self) -> _DMACRXW {
        _DMACRXW { w: self }
    }
    #[doc = "Bits 20:23 - DMA C TX Select"]
    #[inline]
    pub fn dmactx(&mut self) -> _DMACTXW {
        _DMACTXW { w: self }
    }
}
