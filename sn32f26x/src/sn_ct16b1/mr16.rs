#[doc = "Register `MR16` reader"]
pub struct R(crate::R<MR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR16` writer"]
pub struct W(crate::W<MR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR16_SPEC>;
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
impl From<crate::W<MR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR16_SPEC>) -> Self {
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
#[doc = "Offset:0x60 CT16Bn MR16 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr16](index.html) module"]
pub struct MR16_SPEC;
impl crate::RegisterSpec for MR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr16::R](R) reader structure"]
impl crate::Readable for MR16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr16::W](W) writer structure"]
impl crate::Writable for MR16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR16 to value 0"]
impl crate::Resettable for MR16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
