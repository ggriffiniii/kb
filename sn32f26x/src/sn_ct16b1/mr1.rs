#[doc = "Register `MR1` reader"]
pub struct R(crate::R<MR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR1` writer"]
pub struct W(crate::W<MR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR1_SPEC>;
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
impl From<crate::W<MR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR1_SPEC>) -> Self {
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
#[doc = "Offset:0x24 CT16Bn MR1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr1](index.html) module"]
pub struct MR1_SPEC;
impl crate::RegisterSpec for MR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr1::R](R) reader structure"]
impl crate::Readable for MR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr1::W](W) writer structure"]
impl crate::Writable for MR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR1 to value 0"]
impl crate::Resettable for MR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
