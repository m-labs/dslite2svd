#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PC {
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
#[doc = "Possible values of the field `EDM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDM0R {
    #[doc = "Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    DISABLE,
    #[doc = "An additional 6 mA option is provided"]
    _6MA,
    #[doc = "A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    PLUS2MA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EDM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDM0R::DISABLE => 0,
            EDM0R::_6MA => 1,
            EDM0R::PLUS2MA => 3,
            EDM0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDM0R {
        match value {
            0 => EDM0R::DISABLE,
            1 => EDM0R::_6MA,
            3 => EDM0R::PLUS2MA,
            i => EDM0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EDM0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `_6MA`"]
    #[inline]
    pub fn is_6ma(&self) -> bool {
        *self == EDM0R::_6MA
    }
    #[doc = "Checks if the value of the field is `PLUS2MA`"]
    #[inline]
    pub fn is_plus2ma(&self) -> bool {
        *self == EDM0R::PLUS2MA
    }
}
#[doc = r" Value of the field"]
pub struct EDM1R {
    bits: u8,
}
impl EDM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EDM2R {
    bits: u8,
}
impl EDM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EDM3R {
    bits: u8,
}
impl EDM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EDM4R {
    bits: u8,
}
impl EDM4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EDM5R {
    bits: u8,
}
impl EDM5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EDM6R {
    bits: u8,
}
impl EDM6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EDM7R {
    bits: u8,
}
impl EDM7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `EDM0`"]
pub enum EDM0W {
    #[doc = "Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    DISABLE,
    #[doc = "An additional 6 mA option is provided"]
    _6MA,
    #[doc = "A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    PLUS2MA,
}
impl EDM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDM0W::DISABLE => 0,
            EDM0W::_6MA => 1,
            EDM0W::PLUS2MA => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDM0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDM0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Drive values of 2, 4 and 8 mA are maintained. GPIO n Drive Select (GPIODRnR) registers function as normal"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDM0W::DISABLE)
    }
    #[doc = "An additional 6 mA option is provided"]
    #[inline]
    pub fn _6ma(self) -> &'a mut W {
        self.variant(EDM0W::_6MA)
    }
    #[doc = "A 2 mA driver is always enabled; setting the corresponding GPIODR4R register bit adds 2 mA and setting the corresponding GPIODR8R of GPIODR12R register bit adds an additional 4 mA"]
    #[inline]
    pub fn plus2ma(self) -> &'a mut W {
        self.variant(EDM0W::PLUS2MA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDM1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDM1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDM2W<'a> {
    w: &'a mut W,
}
impl<'a> _EDM2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDM3W<'a> {
    w: &'a mut W,
}
impl<'a> _EDM3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDM4W<'a> {
    w: &'a mut W,
}
impl<'a> _EDM4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDM5W<'a> {
    w: &'a mut W,
}
impl<'a> _EDM5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDM6W<'a> {
    w: &'a mut W,
}
impl<'a> _EDM6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDM7W<'a> {
    w: &'a mut W,
}
impl<'a> _EDM7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Extended Drive Mode Bit 0"]
    #[inline]
    pub fn edm0(&self) -> EDM0R {
        EDM0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Extended Drive Mode Bit 1"]
    #[inline]
    pub fn edm1(&self) -> EDM1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EDM1R { bits }
    }
    #[doc = "Bits 4:5 - Extended Drive Mode Bit 2"]
    #[inline]
    pub fn edm2(&self) -> EDM2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EDM2R { bits }
    }
    #[doc = "Bits 6:7 - Extended Drive Mode Bit 3"]
    #[inline]
    pub fn edm3(&self) -> EDM3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EDM3R { bits }
    }
    #[doc = "Bits 8:9 - Extended Drive Mode Bit 4"]
    #[inline]
    pub fn edm4(&self) -> EDM4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EDM4R { bits }
    }
    #[doc = "Bits 10:11 - Extended Drive Mode Bit 5"]
    #[inline]
    pub fn edm5(&self) -> EDM5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EDM5R { bits }
    }
    #[doc = "Bits 12:13 - Extended Drive Mode Bit 6"]
    #[inline]
    pub fn edm6(&self) -> EDM6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EDM6R { bits }
    }
    #[doc = "Bits 14:15 - Extended Drive Mode Bit 7"]
    #[inline]
    pub fn edm7(&self) -> EDM7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EDM7R { bits }
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
    #[doc = "Bits 0:1 - Extended Drive Mode Bit 0"]
    #[inline]
    pub fn edm0(&mut self) -> _EDM0W {
        _EDM0W { w: self }
    }
    #[doc = "Bits 2:3 - Extended Drive Mode Bit 1"]
    #[inline]
    pub fn edm1(&mut self) -> _EDM1W {
        _EDM1W { w: self }
    }
    #[doc = "Bits 4:5 - Extended Drive Mode Bit 2"]
    #[inline]
    pub fn edm2(&mut self) -> _EDM2W {
        _EDM2W { w: self }
    }
    #[doc = "Bits 6:7 - Extended Drive Mode Bit 3"]
    #[inline]
    pub fn edm3(&mut self) -> _EDM3W {
        _EDM3W { w: self }
    }
    #[doc = "Bits 8:9 - Extended Drive Mode Bit 4"]
    #[inline]
    pub fn edm4(&mut self) -> _EDM4W {
        _EDM4W { w: self }
    }
    #[doc = "Bits 10:11 - Extended Drive Mode Bit 5"]
    #[inline]
    pub fn edm5(&mut self) -> _EDM5W {
        _EDM5W { w: self }
    }
    #[doc = "Bits 12:13 - Extended Drive Mode Bit 6"]
    #[inline]
    pub fn edm6(&mut self) -> _EDM6W {
        _EDM6W { w: self }
    }
    #[doc = "Bits 14:15 - Extended Drive Mode Bit 7"]
    #[inline]
    pub fn edm7(&mut self) -> _EDM7W {
        _EDM7W { w: self }
    }
}
