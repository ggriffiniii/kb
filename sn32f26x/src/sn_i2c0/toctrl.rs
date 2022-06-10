#[doc = "Register `TOCTRL` reader"]
pub struct R(crate::R<TOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCTRL` writer"]
pub struct W(crate::W<TOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCTRL_SPEC>;
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
impl From<crate::W<TOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO` reader - Timeout period time = TO*I2Cn_PCLK cycle"]
pub type TO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TO` writer - Timeout period time = TO*I2Cn_PCLK cycle"]
pub type TO_W<'a> = crate::FieldWriter<'a, u32, TOCTRL_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Timeout period time = TO*I2Cn_PCLK cycle"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout period time = TO*I2Cn_PCLK cycle"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x2C I2Cn Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [toctrl](index.html) module"]
pub struct TOCTRL_SPEC;
impl crate::RegisterSpec for TOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [toctrl::R](R) reader structure"]
impl crate::Readable for TOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [toctrl::W](W) writer structure"]
impl crate::Writable for TOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOCTRL to value 0"]
impl crate::Resettable for TOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
