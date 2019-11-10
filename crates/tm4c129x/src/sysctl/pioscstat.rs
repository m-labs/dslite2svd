#[doc = "Reader of register PIOSCSTAT"]
pub type R = crate::R<u32, super::PIOSCSTAT>;
#[doc = "Reader of field `CT`"]
pub type CT_R = crate::R<u8, u8>;
#[doc = "Calibration Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CR_A {
    #[doc = "0: Calibration has not been attempted"]
    CRNONE,
    #[doc = "1: The last calibration operation completed to meet 1% accuracy"]
    CRPASS,
    #[doc = "2: The last calibration operation failed to meet 1% accuracy"]
    CRFAIL,
}
impl From<CR_A> for u8 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        match variant {
            CR_A::CRNONE => 0,
            CR_A::CRPASS => 1,
            CR_A::CRFAIL => 2,
        }
    }
}
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<u8, CR_A>;
impl CR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CR_A::CRNONE),
            1 => Val(CR_A::CRPASS),
            2 => Val(CR_A::CRFAIL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRNONE`"]
    #[inline(always)]
    pub fn is_crnone(&self) -> bool {
        *self == CR_A::CRNONE
    }
    #[doc = "Checks if the value of the field is `CRPASS`"]
    #[inline(always)]
    pub fn is_crpass(&self) -> bool {
        *self == CR_A::CRPASS
    }
    #[doc = "Checks if the value of the field is `CRFAIL`"]
    #[inline(always)]
    pub fn is_crfail(&self) -> bool {
        *self == CR_A::CRFAIL
    }
}
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Calibration Trim Value"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Calibration Result"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:22 - Default Trim Value"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
