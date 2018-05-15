#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIOSCSTAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct CTR {
    bits: u8,
}
impl CTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR {
    #[doc = "Calibration has not been attempted"]
    CRNONE,
    #[doc = "The last calibration operation completed to meet 1% accuracy"]
    CRPASS,
    #[doc = "The last calibration operation failed to meet 1% accuracy"]
    CRFAIL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRR::CRNONE => 0,
            CRR::CRPASS => 1,
            CRR::CRFAIL => 2,
            CRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRR {
        match value {
            0 => CRR::CRNONE,
            1 => CRR::CRPASS,
            2 => CRR::CRFAIL,
            i => CRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRNONE`"]
    #[inline]
    pub fn is_crnone(&self) -> bool {
        *self == CRR::CRNONE
    }
    #[doc = "Checks if the value of the field is `CRPASS`"]
    #[inline]
    pub fn is_crpass(&self) -> bool {
        *self == CRR::CRPASS
    }
    #[doc = "Checks if the value of the field is `CRFAIL`"]
    #[inline]
    pub fn is_crfail(&self) -> bool {
        *self == CRR::CRFAIL
    }
}
#[doc = r" Value of the field"]
pub struct DTR {
    bits: u8,
}
impl DTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Calibration Trim Value"]
    #[inline]
    pub fn ct(&self) -> CTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTR { bits }
    }
    #[doc = "Bits 8:9 - Calibration Result"]
    #[inline]
    pub fn cr(&self) -> CRR {
        CRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:22 - Default Trim Value"]
    #[inline]
    pub fn dt(&self) -> DTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTR { bits }
    }
}
