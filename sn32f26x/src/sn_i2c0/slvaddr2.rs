#[doc = "Register `SLVADDR2` reader"]
pub struct R(crate::R<SLVADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVADDR2` writer"]
pub struct W(crate::W<SLVADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVADDR2_SPEC>;
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
impl From<crate::W<SLVADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - I2Cn slave address 2"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - I2Cn slave address 2"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, SLVADDR2_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - I2Cn slave address 2"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2Cn slave address 2"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x18 I2Cn Slave Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvaddr2](index.html) module"]
pub struct SLVADDR2_SPEC;
impl crate::RegisterSpec for SLVADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvaddr2::R](R) reader structure"]
impl crate::Readable for SLVADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvaddr2::W](W) writer structure"]
impl crate::Writable for SLVADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLVADDR2 to value 0"]
impl crate::Resettable for SLVADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
