#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSDC0 {
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
pub struct S0DCSELR {
    bits: u8,
}
impl S0DCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S1DCSELR {
    bits: u8,
}
impl S1DCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S2DCSELR {
    bits: u8,
}
impl S2DCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S3DCSELR {
    bits: u8,
}
impl S3DCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S4DCSELR {
    bits: u8,
}
impl S4DCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S5DCSELR {
    bits: u8,
}
impl S5DCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S6DCSELR {
    bits: u8,
}
impl S6DCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S7DCSELR {
    bits: u8,
}
impl S7DCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _S0DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S0DCSELW<'a> {
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
pub struct _S1DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S1DCSELW<'a> {
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
pub struct _S2DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S2DCSELW<'a> {
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
pub struct _S3DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S3DCSELW<'a> {
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
pub struct _S4DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S4DCSELW<'a> {
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
pub struct _S5DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S5DCSELW<'a> {
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
pub struct _S6DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S6DCSELW<'a> {
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
pub struct _S7DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _S7DCSELW<'a> {
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
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline]
    pub fn s0dcsel(&self) -> S0DCSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S0DCSELR { bits }
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline]
    pub fn s1dcsel(&self) -> S1DCSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S1DCSELR { bits }
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline]
    pub fn s2dcsel(&self) -> S2DCSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S2DCSELR { bits }
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline]
    pub fn s3dcsel(&self) -> S3DCSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S3DCSELR { bits }
    }
    #[doc = "Bits 16:19 - Sample 4 Digital Comparator Select"]
    #[inline]
    pub fn s4dcsel(&self) -> S4DCSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S4DCSELR { bits }
    }
    #[doc = "Bits 20:23 - Sample 5 Digital Comparator Select"]
    #[inline]
    pub fn s5dcsel(&self) -> S5DCSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S5DCSELR { bits }
    }
    #[doc = "Bits 24:27 - Sample 6 Digital Comparator Select"]
    #[inline]
    pub fn s6dcsel(&self) -> S6DCSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S6DCSELR { bits }
    }
    #[doc = "Bits 28:31 - Sample 7 Digital Comparator Select"]
    #[inline]
    pub fn s7dcsel(&self) -> S7DCSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S7DCSELR { bits }
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
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline]
    pub fn s0dcsel(&mut self) -> _S0DCSELW {
        _S0DCSELW { w: self }
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline]
    pub fn s1dcsel(&mut self) -> _S1DCSELW {
        _S1DCSELW { w: self }
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline]
    pub fn s2dcsel(&mut self) -> _S2DCSELW {
        _S2DCSELW { w: self }
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline]
    pub fn s3dcsel(&mut self) -> _S3DCSELW {
        _S3DCSELW { w: self }
    }
    #[doc = "Bits 16:19 - Sample 4 Digital Comparator Select"]
    #[inline]
    pub fn s4dcsel(&mut self) -> _S4DCSELW {
        _S4DCSELW { w: self }
    }
    #[doc = "Bits 20:23 - Sample 5 Digital Comparator Select"]
    #[inline]
    pub fn s5dcsel(&mut self) -> _S5DCSELW {
        _S5DCSELW { w: self }
    }
    #[doc = "Bits 24:27 - Sample 6 Digital Comparator Select"]
    #[inline]
    pub fn s6dcsel(&mut self) -> _S6DCSELW {
        _S6DCSELW { w: self }
    }
    #[doc = "Bits 28:31 - Sample 7 Digital Comparator Select"]
    #[inline]
    pub fn s7dcsel(&mut self) -> _S7DCSELW {
        _S7DCSELW { w: self }
    }
}
