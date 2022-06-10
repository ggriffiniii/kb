#[doc = "Register `L4BYTE` reader"]
pub struct R(crate::R<L4BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L4BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L4BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L4BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Offset:0x00 UC Low 4 Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l4byte](index.html) module"]
pub struct L4BYTE_SPEC;
impl crate::RegisterSpec for L4BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l4byte::R](R) reader structure"]
impl crate::Readable for L4BYTE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L4BYTE to value 0"]
impl crate::Resettable for L4BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
