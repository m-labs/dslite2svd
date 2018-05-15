#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CRCCTRL {
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
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "Polynomial 0x8005"]
    P8055,
    #[doc = "Polynomial 0x1021"]
    P1021,
    #[doc = "Polynomial 0x4C11DB7"]
    P4C11DB7,
    #[doc = "Polynomial 0x1EDC6F41"]
    P1EDC6F41,
    #[doc = "TCP checksum"]
    TCPCHKSUM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPER::P8055 => 0,
            TYPER::P1021 => 1,
            TYPER::P4C11DB7 => 2,
            TYPER::P1EDC6F41 => 3,
            TYPER::TCPCHKSUM => 8,
            TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPER {
        match value {
            0 => TYPER::P8055,
            1 => TYPER::P1021,
            2 => TYPER::P4C11DB7,
            3 => TYPER::P1EDC6F41,
            8 => TYPER::TCPCHKSUM,
            i => TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P8055`"]
    #[inline]
    pub fn is_p8055(&self) -> bool {
        *self == TYPER::P8055
    }
    #[doc = "Checks if the value of the field is `P1021`"]
    #[inline]
    pub fn is_p1021(&self) -> bool {
        *self == TYPER::P1021
    }
    #[doc = "Checks if the value of the field is `P4C11DB7`"]
    #[inline]
    pub fn is_p4c11db7(&self) -> bool {
        *self == TYPER::P4C11DB7
    }
    #[doc = "Checks if the value of the field is `P1EDC6F41`"]
    #[inline]
    pub fn is_p1edc6f41(&self) -> bool {
        *self == TYPER::P1EDC6F41
    }
    #[doc = "Checks if the value of the field is `TCPCHKSUM`"]
    #[inline]
    pub fn is_tcpchksum(&self) -> bool {
        *self == TYPER::TCPCHKSUM
    }
}
#[doc = "Possible values of the field `ENDIAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANR {
    #[doc = "Configuration unchanged. (B3, B2, B1, B0)"]
    SBHW,
    #[doc = "Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    SHW,
    #[doc = "Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    SHWNB,
    #[doc = "Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    SBSW,
}
impl ENDIANR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENDIANR::SBHW => 0,
            ENDIANR::SHW => 1,
            ENDIANR::SHWNB => 2,
            ENDIANR::SBSW => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENDIANR {
        match value {
            0 => ENDIANR::SBHW,
            1 => ENDIANR::SHW,
            2 => ENDIANR::SHWNB,
            3 => ENDIANR::SBSW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SBHW`"]
    #[inline]
    pub fn is_sbhw(&self) -> bool {
        *self == ENDIANR::SBHW
    }
    #[doc = "Checks if the value of the field is `SHW`"]
    #[inline]
    pub fn is_shw(&self) -> bool {
        *self == ENDIANR::SHW
    }
    #[doc = "Checks if the value of the field is `SHWNB`"]
    #[inline]
    pub fn is_shwnb(&self) -> bool {
        *self == ENDIANR::SHWNB
    }
    #[doc = "Checks if the value of the field is `SBSW`"]
    #[inline]
    pub fn is_sbsw(&self) -> bool {
        *self == ENDIANR::SBSW
    }
}
#[doc = r" Value of the field"]
pub struct BRR {
    bits: bool,
}
impl BRR {
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
pub struct OBRR {
    bits: bool,
}
impl OBRR {
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
pub struct RESINVR {
    bits: bool,
}
impl RESINVR {
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
pub struct SIZER {
    bits: bool,
}
impl SIZER {
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
#[doc = "Possible values of the field `INIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITR {
    #[doc = "Use the CRCSEED register context as the starting value"]
    SEED,
    #[doc = "Initialize to all '0s'"]
    _0,
    #[doc = "Initialize to all '1s'"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INITR::SEED => 0,
            INITR::_0 => 2,
            INITR::_1 => 3,
            INITR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INITR {
        match value {
            0 => INITR::SEED,
            2 => INITR::_0,
            3 => INITR::_1,
            i => INITR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SEED`"]
    #[inline]
    pub fn is_seed(&self) -> bool {
        *self == INITR::SEED
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INITR::_1
    }
}
#[doc = "Values that can be written to the field `TYPE`"]
pub enum TYPEW {
    #[doc = "Polynomial 0x8005"]
    P8055,
    #[doc = "Polynomial 0x1021"]
    P1021,
    #[doc = "Polynomial 0x4C11DB7"]
    P4C11DB7,
    #[doc = "Polynomial 0x1EDC6F41"]
    P1EDC6F41,
    #[doc = "TCP checksum"]
    TCPCHKSUM,
}
impl TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TYPEW::P8055 => 0,
            TYPEW::P1021 => 1,
            TYPEW::P4C11DB7 => 2,
            TYPEW::P1EDC6F41 => 3,
            TYPEW::TCPCHKSUM => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Polynomial 0x8005"]
    #[inline]
    pub fn p8055(self) -> &'a mut W {
        self.variant(TYPEW::P8055)
    }
    #[doc = "Polynomial 0x1021"]
    #[inline]
    pub fn p1021(self) -> &'a mut W {
        self.variant(TYPEW::P1021)
    }
    #[doc = "Polynomial 0x4C11DB7"]
    #[inline]
    pub fn p4c11db7(self) -> &'a mut W {
        self.variant(TYPEW::P4C11DB7)
    }
    #[doc = "Polynomial 0x1EDC6F41"]
    #[inline]
    pub fn p1edc6f41(self) -> &'a mut W {
        self.variant(TYPEW::P1EDC6F41)
    }
    #[doc = "TCP checksum"]
    #[inline]
    pub fn tcpchksum(self) -> &'a mut W {
        self.variant(TYPEW::TCPCHKSUM)
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
#[doc = "Values that can be written to the field `ENDIAN`"]
pub enum ENDIANW {
    #[doc = "Configuration unchanged. (B3, B2, B1, B0)"]
    SBHW,
    #[doc = "Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    SHW,
    #[doc = "Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    SHWNB,
    #[doc = "Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    SBSW,
}
impl ENDIANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENDIANW::SBHW => 0,
            ENDIANW::SHW => 1,
            ENDIANW::SHWNB => 2,
            ENDIANW::SBSW => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDIANW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDIANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDIANW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Configuration unchanged. (B3, B2, B1, B0)"]
    #[inline]
    pub fn sbhw(self) -> &'a mut W {
        self.variant(ENDIANW::SBHW)
    }
    #[doc = "Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    #[inline]
    pub fn shw(self) -> &'a mut W {
        self.variant(ENDIANW::SHW)
    }
    #[doc = "Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    #[inline]
    pub fn shwnb(self) -> &'a mut W {
        self.variant(ENDIANW::SHWNB)
    }
    #[doc = "Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    #[inline]
    pub fn sbsw(self) -> &'a mut W {
        self.variant(ENDIANW::SBSW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BRW<'a> {
    w: &'a mut W,
}
impl<'a> _BRW<'a> {
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
pub struct _OBRW<'a> {
    w: &'a mut W,
}
impl<'a> _OBRW<'a> {
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
pub struct _RESINVW<'a> {
    w: &'a mut W,
}
impl<'a> _RESINVW<'a> {
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
pub struct _SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZEW<'a> {
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
#[doc = "Values that can be written to the field `INIT`"]
pub enum INITW {
    #[doc = "Use the CRCSEED register context as the starting value"]
    SEED,
    #[doc = "Initialize to all '0s'"]
    _0,
    #[doc = "Initialize to all '1s'"]
    _1,
}
impl INITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INITW::SEED => 0,
            INITW::_0 => 2,
            INITW::_1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITW<'a> {
    w: &'a mut W,
}
impl<'a> _INITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use the CRCSEED register context as the starting value"]
    #[inline]
    pub fn seed(self) -> &'a mut W {
        self.variant(INITW::SEED)
    }
    #[doc = "Initialize to all '0s'"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITW::_0)
    }
    #[doc = "Initialize to all '1s'"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:3 - Operation Type"]
    #[inline]
    pub fn type_(&self) -> TYPER {
        TYPER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Endian Control"]
    #[inline]
    pub fn endian(&self) -> ENDIANR {
        ENDIANR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Bit reverse enable"]
    #[inline]
    pub fn br(&self) -> BRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BRR { bits }
    }
    #[doc = "Bit 8 - Output Reverse Enable"]
    #[inline]
    pub fn obr(&self) -> OBRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OBRR { bits }
    }
    #[doc = "Bit 9 - Result Inverse Enable"]
    #[inline]
    pub fn resinv(&self) -> RESINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESINVR { bits }
    }
    #[doc = "Bit 12 - Input Data Size"]
    #[inline]
    pub fn size(&self) -> SIZER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SIZER { bits }
    }
    #[doc = "Bits 13:14 - CRC Initialization"]
    #[inline]
    pub fn init(&self) -> INITR {
        INITR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:3 - Operation Type"]
    #[inline]
    pub fn type_(&mut self) -> _TYPEW {
        _TYPEW { w: self }
    }
    #[doc = "Bits 4:5 - Endian Control"]
    #[inline]
    pub fn endian(&mut self) -> _ENDIANW {
        _ENDIANW { w: self }
    }
    #[doc = "Bit 7 - Bit reverse enable"]
    #[inline]
    pub fn br(&mut self) -> _BRW {
        _BRW { w: self }
    }
    #[doc = "Bit 8 - Output Reverse Enable"]
    #[inline]
    pub fn obr(&mut self) -> _OBRW {
        _OBRW { w: self }
    }
    #[doc = "Bit 9 - Result Inverse Enable"]
    #[inline]
    pub fn resinv(&mut self) -> _RESINVW {
        _RESINVW { w: self }
    }
    #[doc = "Bit 12 - Input Data Size"]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
    #[doc = "Bits 13:14 - CRC Initialization"]
    #[inline]
    pub fn init(&mut self) -> _INITW {
        _INITW { w: self }
    }
}
