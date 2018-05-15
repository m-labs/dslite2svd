#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSTSH0 {
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
#[doc = "Possible values of the field `TSH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSH0R {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSH0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSH0R::_4 => 0,
            TSH0R::_8 => 2,
            TSH0R::_16 => 4,
            TSH0R::_32 => 6,
            TSH0R::_64 => 8,
            TSH0R::_128 => 10,
            TSH0R::_256 => 12,
            TSH0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSH0R {
        match value {
            0 => TSH0R::_4,
            2 => TSH0R::_8,
            4 => TSH0R::_16,
            6 => TSH0R::_32,
            8 => TSH0R::_64,
            10 => TSH0R::_128,
            12 => TSH0R::_256,
            i => TSH0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TSH0R::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TSH0R::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TSH0R::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TSH0R::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TSH0R::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TSH0R::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TSH0R::_256
    }
}
#[doc = "Possible values of the field `TSH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSH1R {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSH1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSH1R::_4 => 0,
            TSH1R::_8 => 2,
            TSH1R::_16 => 4,
            TSH1R::_32 => 6,
            TSH1R::_64 => 8,
            TSH1R::_128 => 10,
            TSH1R::_256 => 12,
            TSH1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSH1R {
        match value {
            0 => TSH1R::_4,
            2 => TSH1R::_8,
            4 => TSH1R::_16,
            6 => TSH1R::_32,
            8 => TSH1R::_64,
            10 => TSH1R::_128,
            12 => TSH1R::_256,
            i => TSH1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TSH1R::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TSH1R::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TSH1R::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TSH1R::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TSH1R::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TSH1R::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TSH1R::_256
    }
}
#[doc = "Possible values of the field `TSH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSH2R {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSH2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSH2R::_4 => 0,
            TSH2R::_8 => 2,
            TSH2R::_16 => 4,
            TSH2R::_32 => 6,
            TSH2R::_64 => 8,
            TSH2R::_128 => 10,
            TSH2R::_256 => 12,
            TSH2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSH2R {
        match value {
            0 => TSH2R::_4,
            2 => TSH2R::_8,
            4 => TSH2R::_16,
            6 => TSH2R::_32,
            8 => TSH2R::_64,
            10 => TSH2R::_128,
            12 => TSH2R::_256,
            i => TSH2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TSH2R::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TSH2R::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TSH2R::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TSH2R::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TSH2R::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TSH2R::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TSH2R::_256
    }
}
#[doc = "Possible values of the field `TSH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSH3R {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSH3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSH3R::_4 => 0,
            TSH3R::_8 => 2,
            TSH3R::_16 => 4,
            TSH3R::_32 => 6,
            TSH3R::_64 => 8,
            TSH3R::_128 => 10,
            TSH3R::_256 => 12,
            TSH3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSH3R {
        match value {
            0 => TSH3R::_4,
            2 => TSH3R::_8,
            4 => TSH3R::_16,
            6 => TSH3R::_32,
            8 => TSH3R::_64,
            10 => TSH3R::_128,
            12 => TSH3R::_256,
            i => TSH3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TSH3R::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TSH3R::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TSH3R::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TSH3R::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TSH3R::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TSH3R::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TSH3R::_256
    }
}
#[doc = "Possible values of the field `TSH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSH4R {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSH4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSH4R::_4 => 0,
            TSH4R::_8 => 2,
            TSH4R::_16 => 4,
            TSH4R::_32 => 6,
            TSH4R::_64 => 8,
            TSH4R::_128 => 10,
            TSH4R::_256 => 12,
            TSH4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSH4R {
        match value {
            0 => TSH4R::_4,
            2 => TSH4R::_8,
            4 => TSH4R::_16,
            6 => TSH4R::_32,
            8 => TSH4R::_64,
            10 => TSH4R::_128,
            12 => TSH4R::_256,
            i => TSH4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TSH4R::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TSH4R::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TSH4R::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TSH4R::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TSH4R::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TSH4R::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TSH4R::_256
    }
}
#[doc = "Possible values of the field `TSH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSH5R {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSH5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSH5R::_4 => 0,
            TSH5R::_8 => 2,
            TSH5R::_16 => 4,
            TSH5R::_32 => 6,
            TSH5R::_64 => 8,
            TSH5R::_128 => 10,
            TSH5R::_256 => 12,
            TSH5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSH5R {
        match value {
            0 => TSH5R::_4,
            2 => TSH5R::_8,
            4 => TSH5R::_16,
            6 => TSH5R::_32,
            8 => TSH5R::_64,
            10 => TSH5R::_128,
            12 => TSH5R::_256,
            i => TSH5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TSH5R::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TSH5R::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TSH5R::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TSH5R::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TSH5R::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TSH5R::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TSH5R::_256
    }
}
#[doc = "Possible values of the field `TSH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSH6R {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSH6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSH6R::_4 => 0,
            TSH6R::_8 => 2,
            TSH6R::_16 => 4,
            TSH6R::_32 => 6,
            TSH6R::_64 => 8,
            TSH6R::_128 => 10,
            TSH6R::_256 => 12,
            TSH6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSH6R {
        match value {
            0 => TSH6R::_4,
            2 => TSH6R::_8,
            4 => TSH6R::_16,
            6 => TSH6R::_32,
            8 => TSH6R::_64,
            10 => TSH6R::_128,
            12 => TSH6R::_256,
            i => TSH6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TSH6R::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TSH6R::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TSH6R::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TSH6R::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TSH6R::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TSH6R::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TSH6R::_256
    }
}
#[doc = "Possible values of the field `TSH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSH7R {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSH7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSH7R::_4 => 0,
            TSH7R::_8 => 2,
            TSH7R::_16 => 4,
            TSH7R::_32 => 6,
            TSH7R::_64 => 8,
            TSH7R::_128 => 10,
            TSH7R::_256 => 12,
            TSH7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSH7R {
        match value {
            0 => TSH7R::_4,
            2 => TSH7R::_8,
            4 => TSH7R::_16,
            6 => TSH7R::_32,
            8 => TSH7R::_64,
            10 => TSH7R::_128,
            12 => TSH7R::_256,
            i => TSH7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == TSH7R::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == TSH7R::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == TSH7R::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == TSH7R::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == TSH7R::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == TSH7R::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == TSH7R::_256
    }
}
#[doc = "Values that can be written to the field `TSH0`"]
pub enum TSH0W {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
}
impl TSH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSH0W::_4 => 0,
            TSH0W::_8 => 2,
            TSH0W::_16 => 4,
            TSH0W::_32 => 6,
            TSH0W::_64 => 8,
            TSH0W::_128 => 10,
            TSH0W::_256 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSH0W<'a> {
    w: &'a mut W,
}
impl<'a> _TSH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSH0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "N_SH = 4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH0W::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH0W::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH0W::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH0W::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH0W::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH0W::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH0W::_256)
    }
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
#[doc = "Values that can be written to the field `TSH1`"]
pub enum TSH1W {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
}
impl TSH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSH1W::_4 => 0,
            TSH1W::_8 => 2,
            TSH1W::_16 => 4,
            TSH1W::_32 => 6,
            TSH1W::_64 => 8,
            TSH1W::_128 => 10,
            TSH1W::_256 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSH1W<'a> {
    w: &'a mut W,
}
impl<'a> _TSH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSH1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "N_SH = 4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH1W::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH1W::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH1W::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH1W::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH1W::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH1W::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH1W::_256)
    }
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
#[doc = "Values that can be written to the field `TSH2`"]
pub enum TSH2W {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
}
impl TSH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSH2W::_4 => 0,
            TSH2W::_8 => 2,
            TSH2W::_16 => 4,
            TSH2W::_32 => 6,
            TSH2W::_64 => 8,
            TSH2W::_128 => 10,
            TSH2W::_256 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSH2W<'a> {
    w: &'a mut W,
}
impl<'a> _TSH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSH2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "N_SH = 4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH2W::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH2W::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH2W::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH2W::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH2W::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH2W::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH2W::_256)
    }
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
#[doc = "Values that can be written to the field `TSH3`"]
pub enum TSH3W {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
}
impl TSH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSH3W::_4 => 0,
            TSH3W::_8 => 2,
            TSH3W::_16 => 4,
            TSH3W::_32 => 6,
            TSH3W::_64 => 8,
            TSH3W::_128 => 10,
            TSH3W::_256 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSH3W<'a> {
    w: &'a mut W,
}
impl<'a> _TSH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSH3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "N_SH = 4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH3W::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH3W::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH3W::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH3W::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH3W::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH3W::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH3W::_256)
    }
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
#[doc = "Values that can be written to the field `TSH4`"]
pub enum TSH4W {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
}
impl TSH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSH4W::_4 => 0,
            TSH4W::_8 => 2,
            TSH4W::_16 => 4,
            TSH4W::_32 => 6,
            TSH4W::_64 => 8,
            TSH4W::_128 => 10,
            TSH4W::_256 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSH4W<'a> {
    w: &'a mut W,
}
impl<'a> _TSH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSH4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "N_SH = 4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH4W::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH4W::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH4W::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH4W::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH4W::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH4W::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH4W::_256)
    }
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
#[doc = "Values that can be written to the field `TSH5`"]
pub enum TSH5W {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
}
impl TSH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSH5W::_4 => 0,
            TSH5W::_8 => 2,
            TSH5W::_16 => 4,
            TSH5W::_32 => 6,
            TSH5W::_64 => 8,
            TSH5W::_128 => 10,
            TSH5W::_256 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSH5W<'a> {
    w: &'a mut W,
}
impl<'a> _TSH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSH5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "N_SH = 4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH5W::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH5W::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH5W::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH5W::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH5W::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH5W::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH5W::_256)
    }
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
#[doc = "Values that can be written to the field `TSH6`"]
pub enum TSH6W {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
}
impl TSH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSH6W::_4 => 0,
            TSH6W::_8 => 2,
            TSH6W::_16 => 4,
            TSH6W::_32 => 6,
            TSH6W::_64 => 8,
            TSH6W::_128 => 10,
            TSH6W::_256 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSH6W<'a> {
    w: &'a mut W,
}
impl<'a> _TSH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSH6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "N_SH = 4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH6W::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH6W::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH6W::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH6W::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH6W::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH6W::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH6W::_256)
    }
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
#[doc = "Values that can be written to the field `TSH7`"]
pub enum TSH7W {
    #[doc = "N_SH = 4"]
    _4,
    #[doc = "N_SH = 8"]
    _8,
    #[doc = "N_SH = 16"]
    _16,
    #[doc = "N_SH = 32"]
    _32,
    #[doc = "N_SH = 64"]
    _64,
    #[doc = "N_SH = 128"]
    _128,
    #[doc = "N_SH = 256"]
    _256,
}
impl TSH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSH7W::_4 => 0,
            TSH7W::_8 => 2,
            TSH7W::_16 => 4,
            TSH7W::_32 => 6,
            TSH7W::_64 => 8,
            TSH7W::_128 => 10,
            TSH7W::_256 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSH7W<'a> {
    w: &'a mut W,
}
impl<'a> _TSH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSH7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "N_SH = 4"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(TSH7W::_4)
    }
    #[doc = "N_SH = 8"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(TSH7W::_8)
    }
    #[doc = "N_SH = 16"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(TSH7W::_16)
    }
    #[doc = "N_SH = 32"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(TSH7W::_32)
    }
    #[doc = "N_SH = 64"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(TSH7W::_64)
    }
    #[doc = "N_SH = 128"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(TSH7W::_128)
    }
    #[doc = "N_SH = 256"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(TSH7W::_256)
    }
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
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline]
    pub fn tsh0(&self) -> TSH0R {
        TSH0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline]
    pub fn tsh1(&self) -> TSH1R {
        TSH1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline]
    pub fn tsh2(&self) -> TSH2R {
        TSH2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh3(&self) -> TSH3R {
        TSH3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - 5th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh4(&self) -> TSH4R {
        TSH4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - 6th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh5(&self) -> TSH5R {
        TSH5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - 7th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh6(&self) -> TSH6R {
        TSH6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - 8th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh7(&self) -> TSH7R {
        TSH7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline]
    pub fn tsh0(&mut self) -> _TSH0W {
        _TSH0W { w: self }
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline]
    pub fn tsh1(&mut self) -> _TSH1W {
        _TSH1W { w: self }
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline]
    pub fn tsh2(&mut self) -> _TSH2W {
        _TSH2W { w: self }
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh3(&mut self) -> _TSH3W {
        _TSH3W { w: self }
    }
    #[doc = "Bits 16:19 - 5th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh4(&mut self) -> _TSH4W {
        _TSH4W { w: self }
    }
    #[doc = "Bits 20:23 - 6th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh5(&mut self) -> _TSH5W {
        _TSH5W { w: self }
    }
    #[doc = "Bits 24:27 - 7th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh6(&mut self) -> _TSH6W {
        _TSH6W { w: self }
    }
    #[doc = "Bits 28:31 - 8th Sample and Hold Period Select"]
    #[inline]
    pub fn tsh7(&mut self) -> _TSH7W {
        _TSH7W { w: self }
    }
}
