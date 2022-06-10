#[doc = "Register `FRMNO` reader"]
pub struct R(crate::R<FRMNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAME_NO` reader - The 11-bit frame number of the SOF packet"]
pub type FRAME_NO_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - The 11-bit frame number of the SOF packet"]
    #[inline(always)]
    pub fn frame_no(&self) -> FRAME_NO_R {
        FRAME_NO_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Offset:0x60 USB Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmno](index.html) module"]
pub struct FRMNO_SPEC;
impl crate::RegisterSpec for FRMNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frmno::R](R) reader structure"]
impl crate::Readable for FRMNO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRMNO to value 0"]
impl crate::Resettable for FRMNO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
