#[doc = "Register `SCLHT` reader"]
pub struct R(crate::R<SCLHT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLHT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCLHT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCLHT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCLHT` writer"]
pub struct W(crate::W<SCLHT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLHT_SPEC>;
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
impl From<crate::W<SCLHT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCLHT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLH` reader - SCLn High period time=(SCLHT+1)*I2Cn_PCLK cycle"]
pub type SCLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLH` writer - SCLn High period time=(SCLHT+1)*I2Cn_PCLK cycle"]
pub type SCLH_W<'a> = crate::FieldWriter<'a, u32, SCLHT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - SCLn High period time=(SCLHT+1)*I2Cn_PCLK cycle"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCLn High period time=(SCLHT+1)*I2Cn_PCLK cycle"]
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W {
        SCLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x20 I2Cn SCL High Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sclht](index.html) module"]
pub struct SCLHT_SPEC;
impl crate::RegisterSpec for SCLHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sclht::R](R) reader structure"]
impl crate::Readable for SCLHT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sclht::W](W) writer structure"]
impl crate::Writable for SCLHT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCLHT to value 0x04"]
impl crate::Resettable for SCLHT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
