#[doc = "Register `CAP0` reader"]
pub struct R(crate::R<CAP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAP0` reader - Timer counter capture value"]
pub type CAP0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer counter capture value"]
    #[inline(always)]
    pub fn cap0(&self) -> CAP0_R {
        CAP0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Offset:0x84 CT16Bn CAP0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap0](index.html) module"]
pub struct CAP0_SPEC;
impl crate::RegisterSpec for CAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap0::R](R) reader structure"]
impl crate::Readable for CAP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAP0 to value 0"]
impl crate::Resettable for CAP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
