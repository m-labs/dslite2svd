#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHMAP3 {
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
pub struct CH24SELR {
    bits: u8,
}
impl CH24SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH25SELR {
    bits: u8,
}
impl CH25SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH26SELR {
    bits: u8,
}
impl CH26SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH27SELR {
    bits: u8,
}
impl CH27SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH28SELR {
    bits: u8,
}
impl CH28SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH29SELR {
    bits: u8,
}
impl CH29SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH30SELR {
    bits: u8,
}
impl CH30SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH31SELR {
    bits: u8,
}
impl CH31SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CH24SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH24SELW<'a> {
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
pub struct _CH25SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH25SELW<'a> {
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
pub struct _CH26SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH26SELW<'a> {
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
pub struct _CH27SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH27SELW<'a> {
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
pub struct _CH28SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH28SELW<'a> {
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
pub struct _CH29SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH29SELW<'a> {
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
pub struct _CH30SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH30SELW<'a> {
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
pub struct _CH31SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH31SELW<'a> {
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
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline]
    pub fn ch24sel(&self) -> CH24SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH24SELR { bits }
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline]
    pub fn ch25sel(&self) -> CH25SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH25SELR { bits }
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline]
    pub fn ch26sel(&self) -> CH26SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH26SELR { bits }
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline]
    pub fn ch27sel(&self) -> CH27SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH27SELR { bits }
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline]
    pub fn ch28sel(&self) -> CH28SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH28SELR { bits }
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline]
    pub fn ch29sel(&self) -> CH29SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH29SELR { bits }
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline]
    pub fn ch30sel(&self) -> CH30SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH30SELR { bits }
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline]
    pub fn ch31sel(&self) -> CH31SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CH31SELR { bits }
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
    #[doc = "Bits 0:3 - uDMA Channel 24 Source Select"]
    #[inline]
    pub fn ch24sel(&mut self) -> _CH24SELW {
        _CH24SELW { w: self }
    }
    #[doc = "Bits 4:7 - uDMA Channel 25 Source Select"]
    #[inline]
    pub fn ch25sel(&mut self) -> _CH25SELW {
        _CH25SELW { w: self }
    }
    #[doc = "Bits 8:11 - uDMA Channel 26 Source Select"]
    #[inline]
    pub fn ch26sel(&mut self) -> _CH26SELW {
        _CH26SELW { w: self }
    }
    #[doc = "Bits 12:15 - uDMA Channel 27 Source Select"]
    #[inline]
    pub fn ch27sel(&mut self) -> _CH27SELW {
        _CH27SELW { w: self }
    }
    #[doc = "Bits 16:19 - uDMA Channel 28 Source Select"]
    #[inline]
    pub fn ch28sel(&mut self) -> _CH28SELW {
        _CH28SELW { w: self }
    }
    #[doc = "Bits 20:23 - uDMA Channel 29 Source Select"]
    #[inline]
    pub fn ch29sel(&mut self) -> _CH29SELW {
        _CH29SELW { w: self }
    }
    #[doc = "Bits 24:27 - uDMA Channel 30 Source Select"]
    #[inline]
    pub fn ch30sel(&mut self) -> _CH30SELW {
        _CH30SELW { w: self }
    }
    #[doc = "Bits 28:31 - uDMA Channel 31 Source Select"]
    #[inline]
    pub fn ch31sel(&mut self) -> _CH31SELW {
        _CH31SELW { w: self }
    }
}
