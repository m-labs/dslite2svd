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
#[doc = r" Value of the field"]
pub struct PHYHOLDR {
    bits: bool,
}
impl PHYHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `ANMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANMODER {
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    _10HD,
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    _10FD,
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    _100HD,
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    _100FD,
}
impl ANMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ANMODER::_10HD => 0,
            ANMODER::_10FD => 1,
            ANMODER::_100HD => 2,
            ANMODER::_100FD => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ANMODER {
        match value {
            0 => ANMODER::_10HD,
            1 => ANMODER::_10FD,
            2 => ANMODER::_100HD,
            3 => ANMODER::_100FD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_10HD`"]
    #[inline]
    pub fn is_10hd(&self) -> bool {
        *self == ANMODER::_10HD
    }
    #[doc = "Checks if the value of the field is `_10FD`"]
    #[inline]
    pub fn is_10fd(&self) -> bool {
        *self == ANMODER::_10FD
    }
    #[doc = "Checks if the value of the field is `_100HD`"]
    #[inline]
    pub fn is_100hd(&self) -> bool {
        *self == ANMODER::_100HD
    }
    #[doc = "Checks if the value of the field is `_100FD`"]
    #[inline]
    pub fn is_100fd(&self) -> bool {
        *self == ANMODER::_100FD
    }
}
#[doc = r" Value of the field"]
pub struct ANENR {
    bits: bool,
}
impl ANENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FASTANSELR {
    bits: u8,
}
impl FASTANSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FASTANENR {
    bits: bool,
}
impl FASTANENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct EXTFDR {
    bits: bool,
}
impl EXTFDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FASTLUPDR {
    bits: bool,
}
impl FASTLUPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FASTRXDVR {
    bits: bool,
}
impl FASTRXDVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct MDIXENR {
    bits: bool,
}
impl MDIXENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FASTMDIXR {
    bits: bool,
}
impl FASTMDIXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RBSTMDIXR {
    bits: bool,
}
impl RBSTMDIXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct MDISWAPR {
    bits: bool,
}
impl MDISWAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct POLSWAPR {
    bits: bool,
}
impl POLSWAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FASTLDMODER {
    bits: u8,
}
impl FASTLDMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TDRRUNR {
    bits: bool,
}
impl TDRRUNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct LRRR {
    bits: bool,
}
impl LRRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ISOMIILLR {
    bits: bool,
}
impl ISOMIILLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RXERIDLER {
    bits: bool,
}
impl RXERIDLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct NIBDETDISR {
    bits: bool,
}
impl NIBDETDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DIGRESTARTR {
    bits: bool,
}
impl DIGRESTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PINTFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINTFSR {
    #[doc = "MII (default) Used for internal PHY or external PHY connected via MII"]
    IMII,
    #[doc = "RMII: Used for external PHY connected via RMII"]
    RMII,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PINTFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINTFSR::IMII => 0,
            PINTFSR::RMII => 4,
            PINTFSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINTFSR {
        match value {
            0 => PINTFSR::IMII,
            4 => PINTFSR::RMII,
            i => PINTFSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMII`"]
    #[inline]
    pub fn is_imii(&self) -> bool {
        *self == PINTFSR::IMII
    }
    #[doc = "Checks if the value of the field is `RMII`"]
    #[inline]
    pub fn is_rmii(&self) -> bool {
        *self == PINTFSR::RMII
    }
}
#[doc = r" Value of the field"]
pub struct PHYEXTR {
    bits: bool,
}
impl PHYEXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _PHYHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYHOLDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ANMODE`"]
pub enum ANMODEW {
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    _10HD,
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    _10FD,
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    _100HD,
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    _100FD,
}
impl ANMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ANMODEW::_10HD => 0,
            ANMODEW::_10FD => 1,
            ANMODEW::_100HD => 2,
            ANMODEW::_100FD => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ANMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    #[inline]
    pub fn _10hd(self) -> &'a mut W {
        self.variant(ANMODEW::_10HD)
    }
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    #[inline]
    pub fn _10fd(self) -> &'a mut W {
        self.variant(ANMODEW::_10FD)
    }
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    #[inline]
    pub fn _100hd(self) -> &'a mut W {
        self.variant(ANMODEW::_100HD)
    }
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    #[inline]
    pub fn _100fd(self) -> &'a mut W {
        self.variant(ANMODEW::_100FD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ANENW<'a> {
    w: &'a mut W,
}
impl<'a> _ANENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FASTANSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTANSELW<'a> {
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
pub struct _FASTANENW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTANENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTFDW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTFDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FASTLUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTLUPDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FASTRXDVW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTRXDVW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MDIXENW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIXENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FASTMDIXW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTMDIXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RBSTMDIXW<'a> {
    w: &'a mut W,
}
impl<'a> _RBSTMDIXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MDISWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISWAPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POLSWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _POLSWAPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FASTLDMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTLDMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TDRRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _TDRRUNW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LRRW<'a> {
    w: &'a mut W,
}
impl<'a> _LRRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ISOMIILLW<'a> {
    w: &'a mut W,
}
impl<'a> _ISOMIILLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXERIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERIDLEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NIBDETDISW<'a> {
    w: &'a mut W,
}
impl<'a> _NIBDETDISW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIGRESTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIGRESTARTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINTFS`"]
pub enum PINTFSW {
    #[doc = "MII (default) Used for internal PHY or external PHY connected via MII"]
    IMII,
    #[doc = "RMII: Used for external PHY connected via RMII"]
    RMII,
}
impl PINTFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINTFSW::IMII => 0,
            PINTFSW::RMII => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINTFSW<'a> {
    w: &'a mut W,
}
impl<'a> _PINTFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINTFSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MII (default) Used for internal PHY or external PHY connected via MII"]
    #[inline]
    pub fn imii(self) -> &'a mut W {
        self.variant(PINTFSW::IMII)
    }
    #[doc = "RMII: Used for external PHY connected via RMII"]
    #[inline]
    pub fn rmii(self) -> &'a mut W {
        self.variant(PINTFSW::RMII)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHYEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYEXTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Ethernet PHY Hold"]
    #[inline]
    pub fn phyhold(&self) -> PHYHOLDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHYHOLDR { bits }
    }
    #[doc = "Bits 1:2 - Auto Negotiation Mode"]
    #[inline]
    pub fn anmode(&self) -> ANMODER {
        ANMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Auto Negotiation Enable"]
    #[inline]
    pub fn anen(&self) -> ANENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANENR { bits }
    }
    #[doc = "Bits 4:5 - Fast Auto Negotiation Select"]
    #[inline]
    pub fn fastansel(&self) -> FASTANSELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FASTANSELR { bits }
    }
    #[doc = "Bit 6 - Fast Auto Negotiation Enable"]
    #[inline]
    pub fn fastanen(&self) -> FASTANENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FASTANENR { bits }
    }
    #[doc = "Bit 7 - Extended Full Duplex Ability"]
    #[inline]
    pub fn extfd(&self) -> EXTFDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXTFDR { bits }
    }
    #[doc = "Bit 8 - FAST Link-Up in Parallel Detect"]
    #[inline]
    pub fn fastlupd(&self) -> FASTLUPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FASTLUPDR { bits }
    }
    #[doc = "Bit 9 - Fast RXDV Detection"]
    #[inline]
    pub fn fastrxdv(&self) -> FASTRXDVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FASTRXDVR { bits }
    }
    #[doc = "Bit 10 - MDIX Enable"]
    #[inline]
    pub fn mdixen(&self) -> MDIXENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MDIXENR { bits }
    }
    #[doc = "Bit 11 - Fast Auto MDI-X"]
    #[inline]
    pub fn fastmdix(&self) -> FASTMDIXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FASTMDIXR { bits }
    }
    #[doc = "Bit 12 - Robust Auto MDI-X"]
    #[inline]
    pub fn rbstmdix(&self) -> RBSTMDIXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RBSTMDIXR { bits }
    }
    #[doc = "Bit 13 - MDI Swap"]
    #[inline]
    pub fn mdiswap(&self) -> MDISWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MDISWAPR { bits }
    }
    #[doc = "Bit 14 - Polarity Swap"]
    #[inline]
    pub fn polswap(&self) -> POLSWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POLSWAPR { bits }
    }
    #[doc = "Bits 15:19 - Fast Link Down Mode"]
    #[inline]
    pub fn fastldmode(&self) -> FASTLDMODER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FASTLDMODER { bits }
    }
    #[doc = "Bit 20 - TDR Auto Run"]
    #[inline]
    pub fn tdrrun(&self) -> TDRRUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TDRRUNR { bits }
    }
    #[doc = "Bit 21 - Link Loss Recovery"]
    #[inline]
    pub fn lrr(&self) -> LRRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LRRR { bits }
    }
    #[doc = "Bit 22 - Isolate MII in Link Loss"]
    #[inline]
    pub fn isomiill(&self) -> ISOMIILLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ISOMIILLR { bits }
    }
    #[doc = "Bit 23 - RXER Detection During Idle"]
    #[inline]
    pub fn rxeridle(&self) -> RXERIDLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXERIDLER { bits }
    }
    #[doc = "Bit 24 - Odd Nibble TXER Detection Disable"]
    #[inline]
    pub fn nibdetdis(&self) -> NIBDETDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NIBDETDISR { bits }
    }
    #[doc = "Bit 25 - PHY Soft Restart"]
    #[inline]
    pub fn digrestart(&self) -> DIGRESTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIGRESTARTR { bits }
    }
    #[doc = "Bits 28:30 - Ethernet Interface Select"]
    #[inline]
    pub fn pintfs(&self) -> PINTFSR {
        PINTFSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - PHY Select"]
    #[inline]
    pub fn phyext(&self) -> PHYEXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHYEXTR { bits }
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
    #[doc = "Bit 0 - Ethernet PHY Hold"]
    #[inline]
    pub fn phyhold(&mut self) -> _PHYHOLDW {
        _PHYHOLDW { w: self }
    }
    #[doc = "Bits 1:2 - Auto Negotiation Mode"]
    #[inline]
    pub fn anmode(&mut self) -> _ANMODEW {
        _ANMODEW { w: self }
    }
    #[doc = "Bit 3 - Auto Negotiation Enable"]
    #[inline]
    pub fn anen(&mut self) -> _ANENW {
        _ANENW { w: self }
    }
    #[doc = "Bits 4:5 - Fast Auto Negotiation Select"]
    #[inline]
    pub fn fastansel(&mut self) -> _FASTANSELW {
        _FASTANSELW { w: self }
    }
    #[doc = "Bit 6 - Fast Auto Negotiation Enable"]
    #[inline]
    pub fn fastanen(&mut self) -> _FASTANENW {
        _FASTANENW { w: self }
    }
    #[doc = "Bit 7 - Extended Full Duplex Ability"]
    #[inline]
    pub fn extfd(&mut self) -> _EXTFDW {
        _EXTFDW { w: self }
    }
    #[doc = "Bit 8 - FAST Link-Up in Parallel Detect"]
    #[inline]
    pub fn fastlupd(&mut self) -> _FASTLUPDW {
        _FASTLUPDW { w: self }
    }
    #[doc = "Bit 9 - Fast RXDV Detection"]
    #[inline]
    pub fn fastrxdv(&mut self) -> _FASTRXDVW {
        _FASTRXDVW { w: self }
    }
    #[doc = "Bit 10 - MDIX Enable"]
    #[inline]
    pub fn mdixen(&mut self) -> _MDIXENW {
        _MDIXENW { w: self }
    }
    #[doc = "Bit 11 - Fast Auto MDI-X"]
    #[inline]
    pub fn fastmdix(&mut self) -> _FASTMDIXW {
        _FASTMDIXW { w: self }
    }
    #[doc = "Bit 12 - Robust Auto MDI-X"]
    #[inline]
    pub fn rbstmdix(&mut self) -> _RBSTMDIXW {
        _RBSTMDIXW { w: self }
    }
    #[doc = "Bit 13 - MDI Swap"]
    #[inline]
    pub fn mdiswap(&mut self) -> _MDISWAPW {
        _MDISWAPW { w: self }
    }
    #[doc = "Bit 14 - Polarity Swap"]
    #[inline]
    pub fn polswap(&mut self) -> _POLSWAPW {
        _POLSWAPW { w: self }
    }
    #[doc = "Bits 15:19 - Fast Link Down Mode"]
    #[inline]
    pub fn fastldmode(&mut self) -> _FASTLDMODEW {
        _FASTLDMODEW { w: self }
    }
    #[doc = "Bit 20 - TDR Auto Run"]
    #[inline]
    pub fn tdrrun(&mut self) -> _TDRRUNW {
        _TDRRUNW { w: self }
    }
    #[doc = "Bit 21 - Link Loss Recovery"]
    #[inline]
    pub fn lrr(&mut self) -> _LRRW {
        _LRRW { w: self }
    }
    #[doc = "Bit 22 - Isolate MII in Link Loss"]
    #[inline]
    pub fn isomiill(&mut self) -> _ISOMIILLW {
        _ISOMIILLW { w: self }
    }
    #[doc = "Bit 23 - RXER Detection During Idle"]
    #[inline]
    pub fn rxeridle(&mut self) -> _RXERIDLEW {
        _RXERIDLEW { w: self }
    }
    #[doc = "Bit 24 - Odd Nibble TXER Detection Disable"]
    #[inline]
    pub fn nibdetdis(&mut self) -> _NIBDETDISW {
        _NIBDETDISW { w: self }
    }
    #[doc = "Bit 25 - PHY Soft Restart"]
    #[inline]
    pub fn digrestart(&mut self) -> _DIGRESTARTW {
        _DIGRESTARTW { w: self }
    }
    #[doc = "Bits 28:30 - Ethernet Interface Select"]
    #[inline]
    pub fn pintfs(&mut self) -> _PINTFSW {
        _PINTFSW { w: self }
    }
    #[doc = "Bit 31 - PHY Select"]
    #[inline]
    pub fn phyext(&mut self) -> _PHYEXTW {
        _PHYEXTW { w: self }
    }
}
