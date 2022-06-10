#[doc = "Register `RWADDR` reader"]
pub struct R(crate::R<RWADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWADDR` writer"]
pub struct W(crate::W<RWADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWADDR_SPEC>;
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
impl From<crate::W<RWADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWADDR` reader - USB FIFO address to be read or written from/to USB FIFO"]
pub type RWADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWADDR` writer - USB FIFO address to be read or written from/to USB FIFO"]
pub type RWADDR_W<'a> = crate::FieldWriter<'a, u32, RWADDR_SPEC, u8, u8, 6, 2>;
impl R {
    #[doc = "Bits 2:7 - USB FIFO address to be read or written from/to USB FIFO"]
    #[inline(always)]
    pub fn rwaddr(&self) -> RWADDR_R {
        RWADDR_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - USB FIFO address to be read or written from/to USB FIFO"]
    #[inline(always)]
    pub fn rwaddr(&mut self) -> RWADDR_W {
        RWADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x78 USB Read/Write Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwaddr](index.html) module"]
pub struct RWADDR_SPEC;
impl crate::RegisterSpec for RWADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwaddr::R](R) reader structure"]
impl crate::Readable for RWADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwaddr::W](W) writer structure"]
impl crate::Writable for RWADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RWADDR to value 0"]
impl crate::Resettable for RWADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
