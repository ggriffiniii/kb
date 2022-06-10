#[doc = "Register `RWDATA` reader"]
pub struct R(crate::R<RWDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWDATA` writer"]
pub struct W(crate::W<RWDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWDATA_SPEC>;
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
impl From<crate::W<RWDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWDATA` reader - Data to be read or written from/to USB FIFO"]
pub type RWDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RWDATA` writer - Data to be read or written from/to USB FIFO"]
pub type RWDATA_W<'a> = crate::FieldWriter<'a, u32, RWDATA_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Data to be read or written from/to USB FIFO"]
    #[inline(always)]
    pub fn rwdata(&self) -> RWDATA_R {
        RWDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to be read or written from/to USB FIFO"]
    #[inline(always)]
    pub fn rwdata(&mut self) -> RWDATA_W {
        RWDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x7C USB Read/Write Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwdata](index.html) module"]
pub struct RWDATA_SPEC;
impl crate::RegisterSpec for RWDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwdata::R](R) reader structure"]
impl crate::Readable for RWDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwdata::W](W) writer structure"]
impl crate::Writable for RWDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RWDATA to value 0"]
impl crate::Resettable for RWDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
