#[doc = "Register `NDTSTS` reader"]
pub struct R(crate::R<NDTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NDTSTS` writer"]
pub struct W(crate::W<NDTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDTSTS_SPEC>;
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
impl From<crate::W<NDTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power noise status of NDT5V IP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDT5V_DET_A {
    #[doc = "0: No power noise is detected"]
    NO = 0,
    #[doc = "1: Power noise is detected by NDT5V IP"]
    DETECTED = 1,
}
impl From<NDT5V_DET_A> for bool {
    #[inline(always)]
    fn from(variant: NDT5V_DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDT5V_DET` reader - Power noise status of NDT5V IP"]
pub type NDT5V_DET_R = crate::BitReader<NDT5V_DET_A>;
impl NDT5V_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDT5V_DET_A {
        match self.bits {
            false => NDT5V_DET_A::NO,
            true => NDT5V_DET_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == NDT5V_DET_A::NO
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == NDT5V_DET_A::DETECTED
    }
}
#[doc = "Field `NDT5V_DET` writer - Power noise status of NDT5V IP"]
pub type NDT5V_DET_W<'a> = crate::BitWriter<'a, u32, NDTSTS_SPEC, NDT5V_DET_A, 1>;
impl<'a> NDT5V_DET_W<'a> {
    #[doc = "No power noise is detected"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(NDT5V_DET_A::NO)
    }
    #[doc = "Power noise is detected by NDT5V IP"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(NDT5V_DET_A::DETECTED)
    }
}
impl R {
    #[doc = "Bit 1 - Power noise status of NDT5V IP"]
    #[inline(always)]
    pub fn ndt5v_det(&self) -> NDT5V_DET_R {
        NDT5V_DET_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Power noise status of NDT5V IP"]
    #[inline(always)]
    pub fn ndt5v_det(&mut self) -> NDT5V_DET_W {
        NDT5V_DET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x2C Noise Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndtsts](index.html) module"]
pub struct NDTSTS_SPEC;
impl crate::RegisterSpec for NDTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ndtsts::R](R) reader structure"]
impl crate::Readable for NDTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ndtsts::W](W) writer structure"]
impl crate::Writable for NDTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NDTSTS to value 0"]
impl crate::Resettable for NDTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
