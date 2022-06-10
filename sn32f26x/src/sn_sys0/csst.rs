#[doc = "Register `CSST` reader"]
pub struct R(crate::R<CSST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "IHRC ready flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IHRCRDY_A {
    #[doc = "0: IHRC is Not Ready"]
    _0 = 0,
    #[doc = "1: IHRC is Ready"]
    _1 = 1,
}
impl From<IHRCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: IHRCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IHRCRDY` reader - IHRC ready flag"]
pub type IHRCRDY_R = crate::BitReader<IHRCRDY_A>;
impl IHRCRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IHRCRDY_A {
        match self.bits {
            false => IHRCRDY_A::_0,
            true => IHRCRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IHRCRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IHRCRDY_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - IHRC ready flag"]
    #[inline(always)]
    pub fn ihrcrdy(&self) -> IHRCRDY_R {
        IHRCRDY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Offset:0x08 Clock Source Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csst](index.html) module"]
pub struct CSST_SPEC;
impl crate::RegisterSpec for CSST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csst::R](R) reader structure"]
impl crate::Readable for CSST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSST to value 0x01"]
impl crate::Resettable for CSST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
