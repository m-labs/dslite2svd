#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCTL {
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
pub struct PMC0R {
    bits: u8,
}
impl PMC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PMC1R {
    bits: u8,
}
impl PMC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PMC2R {
    bits: u8,
}
impl PMC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PMC3R {
    bits: u8,
}
impl PMC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PMC4R {
    bits: u8,
}
impl PMC4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PMC5R {
    bits: u8,
}
impl PMC5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PMC6R {
    bits: u8,
}
impl PMC6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PMC7R {
    bits: u8,
}
impl PMC7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PMC0W<'a> {
    w: &'a mut W,
}
impl<'a> _PMC0W<'a> {
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
pub struct _PMC1W<'a> {
    w: &'a mut W,
}
impl<'a> _PMC1W<'a> {
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
pub struct _PMC2W<'a> {
    w: &'a mut W,
}
impl<'a> _PMC2W<'a> {
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
pub struct _PMC3W<'a> {
    w: &'a mut W,
}
impl<'a> _PMC3W<'a> {
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
pub struct _PMC4W<'a> {
    w: &'a mut W,
}
impl<'a> _PMC4W<'a> {
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
pub struct _PMC5W<'a> {
    w: &'a mut W,
}
impl<'a> _PMC5W<'a> {
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
pub struct _PMC6W<'a> {
    w: &'a mut W,
}
impl<'a> _PMC6W<'a> {
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
pub struct _PMC7W<'a> {
    w: &'a mut W,
}
impl<'a> _PMC7W<'a> {
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
    #[doc = "Bits 0:3 - Port Mux Control 0"]
    #[inline]
    pub fn pmc0(&self) -> PMC0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMC0R { bits }
    }
    #[doc = "Bits 4:7 - Port Mux Control 1"]
    #[inline]
    pub fn pmc1(&self) -> PMC1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMC1R { bits }
    }
    #[doc = "Bits 8:11 - Port Mux Control 2"]
    #[inline]
    pub fn pmc2(&self) -> PMC2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMC2R { bits }
    }
    #[doc = "Bits 12:15 - Port Mux Control 3"]
    #[inline]
    pub fn pmc3(&self) -> PMC3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMC3R { bits }
    }
    #[doc = "Bits 16:19 - Port Mux Control 4"]
    #[inline]
    pub fn pmc4(&self) -> PMC4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMC4R { bits }
    }
    #[doc = "Bits 20:23 - Port Mux Control 5"]
    #[inline]
    pub fn pmc5(&self) -> PMC5R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMC5R { bits }
    }
    #[doc = "Bits 24:27 - Port Mux Control 6"]
    #[inline]
    pub fn pmc6(&self) -> PMC6R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMC6R { bits }
    }
    #[doc = "Bits 28:31 - Port Mux Control 7"]
    #[inline]
    pub fn pmc7(&self) -> PMC7R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMC7R { bits }
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
    #[doc = "Bits 0:3 - Port Mux Control 0"]
    #[inline]
    pub fn pmc0(&mut self) -> _PMC0W {
        _PMC0W { w: self }
    }
    #[doc = "Bits 4:7 - Port Mux Control 1"]
    #[inline]
    pub fn pmc1(&mut self) -> _PMC1W {
        _PMC1W { w: self }
    }
    #[doc = "Bits 8:11 - Port Mux Control 2"]
    #[inline]
    pub fn pmc2(&mut self) -> _PMC2W {
        _PMC2W { w: self }
    }
    #[doc = "Bits 12:15 - Port Mux Control 3"]
    #[inline]
    pub fn pmc3(&mut self) -> _PMC3W {
        _PMC3W { w: self }
    }
    #[doc = "Bits 16:19 - Port Mux Control 4"]
    #[inline]
    pub fn pmc4(&mut self) -> _PMC4W {
        _PMC4W { w: self }
    }
    #[doc = "Bits 20:23 - Port Mux Control 5"]
    #[inline]
    pub fn pmc5(&mut self) -> _PMC5W {
        _PMC5W { w: self }
    }
    #[doc = "Bits 24:27 - Port Mux Control 6"]
    #[inline]
    pub fn pmc6(&mut self) -> _PMC6W {
        _PMC6W { w: self }
    }
    #[doc = "Bits 28:31 - Port Mux Control 7"]
    #[inline]
    pub fn pmc7(&mut self) -> _PMC7W {
        _PMC7W { w: self }
    }
}
