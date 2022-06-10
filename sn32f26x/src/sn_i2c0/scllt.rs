#[doc = "Register `SCLLT` reader"]
pub struct R(crate::R<SCLLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCLLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCLLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCLLT` writer"]
pub struct W(crate::W<SCLLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLLT_SPEC>;
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
impl From<crate::W<SCLLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCLLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLL` reader - SCLn Low period time=(SCLLT+1)*I2Cn_PCLK cycle"]
pub type SCLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLL` writer - SCLn Low period time=(SCLLT+1)*I2Cn_PCLK cycle"]
pub type SCLL_W<'a> = crate::FieldWriter<'a, u32, SCLLT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - SCLn Low period time=(SCLLT+1)*I2Cn_PCLK cycle"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCLn Low period time=(SCLLT+1)*I2Cn_PCLK cycle"]
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W {
        SCLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x24 I2Cn SCL Low Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scllt](index.html) module"]
pub struct SCLLT_SPEC;
impl crate::RegisterSpec for SCLLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scllt::R](R) reader structure"]
impl crate::Readable for SCLLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scllt::W](W) writer structure"]
impl crate::Writable for SCLLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCLLT to value 0x04"]
impl crate::Resettable for SCLLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
