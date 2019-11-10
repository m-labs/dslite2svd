#[doc = "Reader of register ACSTAT0"]
pub type R = crate::R<u32, super::ACSTAT0>;
#[doc = "Reader of field `OVAL`"]
pub type OVAL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Comparator Output Value"]
    #[inline(always)]
    pub fn oval(&self) -> OVAL_R {
        OVAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
