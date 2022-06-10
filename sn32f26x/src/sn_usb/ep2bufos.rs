#[doc = "Register `EP2BUFOS` reader"]
pub struct R(crate::R<EP2BUFOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP2BUFOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP2BUFOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP2BUFOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP2BUFOS` writer"]
pub struct W(crate::W<EP2BUFOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP2BUFOS_SPEC>;
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
impl From<crate::W<EP2BUFOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP2BUFOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - The offset address for endpoint data buffer"]
pub type OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET` writer - The offset address for endpoint data buffer"]
pub type OFFSET_W<'a> = crate::FieldWriter<'a, u32, EP2BUFOS_SPEC, u8, u8, 7, 2>;
impl R {
    #[doc = "Bits 2:8 - The offset address for endpoint data buffer"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:8 - The offset address for endpoint data buffer"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x4C USB Endpoint 2 Buffer Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2bufos](index.html) module"]
pub struct EP2BUFOS_SPEC;
impl crate::RegisterSpec for EP2BUFOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep2bufos::R](R) reader structure"]
impl crate::Readable for EP2BUFOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep2bufos::W](W) writer structure"]
impl crate::Writable for EP2BUFOS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP2BUFOS to value 0"]
impl crate::Resettable for EP2BUFOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
