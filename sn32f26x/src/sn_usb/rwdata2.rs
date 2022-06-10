#[doc = "Register `RWDATA2` reader"]
pub struct R(crate::R<RWDATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWDATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWDATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWDATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWDATA2` writer"]
pub struct W(crate::W<RWDATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWDATA2_SPEC>;
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
impl From<crate::W<RWDATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWDATA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWDATA` reader - Data to be read or written from/to USB FIFO"]
pub type RWDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RWDATA` writer - Data to be read or written from/to USB FIFO"]
pub type RWDATA_W<'a> = crate::FieldWriter<'a, u32, RWDATA2_SPEC, u32, u32, 32, 0>;
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
#[doc = "Offset:0x88 USB Read/Write Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwdata2](index.html) module"]
pub struct RWDATA2_SPEC;
impl crate::RegisterSpec for RWDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwdata2::R](R) reader structure"]
impl crate::Readable for RWDATA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwdata2::W](W) writer structure"]
impl crate::Writable for RWDATA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RWDATA2 to value 0"]
impl crate::Resettable for RWDATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
