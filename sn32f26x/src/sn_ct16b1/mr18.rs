#[doc = "Register `MR18` reader"]
pub struct R(crate::R<MR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR18` writer"]
pub struct W(crate::W<MR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR18_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x68 CT16Bn MR18 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr18](index.html) module"]
pub struct MR18_SPEC;
impl crate::RegisterSpec for MR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr18::R](R) reader structure"]
impl crate::Readable for MR18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr18::W](W) writer structure"]
impl crate::Writable for MR18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR18 to value 0"]
impl crate::Resettable for MR18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
