#[doc = "Register `SCLCT` reader"]
pub struct R(crate::R<SCLCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCLCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCLCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCLCT` writer"]
pub struct W(crate::W<SCLCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLCT_SPEC>;
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
impl From<crate::W<SCLCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCLCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Count for checking SCL arbitration in Master mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCLCT_A {
    #[doc = "0: No effect"]
    DISABLE = 0,
}
impl From<SCLCT_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLCT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCLCT` reader - Count for checking SCL arbitration in Master mode."]
pub type SCLCT_R = crate::FieldReader<u8, SCLCT_A>;
impl SCLCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCLCT_A> {
        match self.bits {
            0 => Some(SCLCT_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCLCT_A::DISABLE
    }
}
#[doc = "Field `SCLCT` writer - Count for checking SCL arbitration in Master mode."]
pub type SCLCT_W<'a> = crate::FieldWriter<'a, u32, SCLCT_SPEC, u8, SCLCT_A, 4, 0>;
impl<'a> SCLCT_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCLCT_A::DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Count for checking SCL arbitration in Master mode."]
    #[inline(always)]
    pub fn sclct(&self) -> SCLCT_R {
        SCLCT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Count for checking SCL arbitration in Master mode."]
    #[inline(always)]
    pub fn sclct(&mut self) -> SCLCT_W {
        SCLCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x28 I2C SCL Check Time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sclct](index.html) module"]
pub struct SCLCT_SPEC;
impl crate::RegisterSpec for SCLCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sclct::R](R) reader structure"]
impl crate::Readable for SCLCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sclct::W](W) writer structure"]
impl crate::Writable for SCLCT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCLCT to value 0x01"]
impl crate::Resettable for SCLCT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
