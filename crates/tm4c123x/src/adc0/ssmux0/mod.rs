#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSMUX0 {
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
pub struct MUX0R {
    bits: u8,
}
impl MUX0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MUX1R {
    bits: u8,
}
impl MUX1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MUX2R {
    bits: u8,
}
impl MUX2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MUX3R {
    bits: u8,
}
impl MUX3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MUX4R {
    bits: u8,
}
impl MUX4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MUX5R {
    bits: u8,
}
impl MUX5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MUX6R {
    bits: u8,
}
impl MUX6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MUX7R {
    bits: u8,
}
impl MUX7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MUX0W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX0W<'a> {
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
pub struct _MUX1W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX1W<'a> {
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
pub struct _MUX2W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX2W<'a> {
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
pub struct _MUX3W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX3W<'a> {
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
pub struct _MUX4W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX4W<'a> {
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
pub struct _MUX5W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX5W<'a> {
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
pub struct _MUX6W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX6W<'a> {
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
pub struct _MUX7W<'a> {
    w: &'a mut W,
}
impl<'a> _MUX7W<'a> {
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
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline]
    pub fn mux0(&self) -> MUX0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX0R { bits }
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline]
    pub fn mux1(&self) -> MUX1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX1R { bits }
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline]
    pub fn mux2(&self) -> MUX2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX2R { bits }
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline]
    pub fn mux3(&self) -> MUX3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX3R { bits }
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline]
    pub fn mux4(&self) -> MUX4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX4R { bits }
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline]
    pub fn mux5(&self) -> MUX5R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX5R { bits }
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline]
    pub fn mux6(&self) -> MUX6R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX6R { bits }
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline]
    pub fn mux7(&self) -> MUX7R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX7R { bits }
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
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline]
    pub fn mux0(&mut self) -> _MUX0W {
        _MUX0W { w: self }
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline]
    pub fn mux1(&mut self) -> _MUX1W {
        _MUX1W { w: self }
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline]
    pub fn mux2(&mut self) -> _MUX2W {
        _MUX2W { w: self }
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline]
    pub fn mux3(&mut self) -> _MUX3W {
        _MUX3W { w: self }
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline]
    pub fn mux4(&mut self) -> _MUX4W {
        _MUX4W { w: self }
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline]
    pub fn mux5(&mut self) -> _MUX5W {
        _MUX5W { w: self }
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline]
    pub fn mux6(&mut self) -> _MUX6W {
        _MUX6W { w: self }
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline]
    pub fn mux7(&mut self) -> _MUX7W {
        _MUX7W { w: self }
    }
}
