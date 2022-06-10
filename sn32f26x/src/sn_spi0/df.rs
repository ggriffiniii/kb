#[doc = "Register `DF` reader"]
pub struct R(crate::R<DF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DF` writer"]
pub struct W(crate::W<DF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DF_SPEC>;
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
impl From<crate::W<DF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI data fetch delay enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DF_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DF_A> for bool {
    #[inline(always)]
    fn from(variant: DF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DF` reader - SPI data fetch delay enable"]
pub type DF_R = crate::BitReader<DF_A>;
impl DF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DF_A {
        match self.bits {
            false => DF_A::DISABLE,
            true => DF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DF_A::ENABLE
    }
}
#[doc = "Field `DF` writer - SPI data fetch delay enable"]
pub type DF_W<'a> = crate::BitWriter<'a, u32, DF_SPEC, DF_A, 0>;
impl<'a> DF_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DF_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DF_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - SPI data fetch delay enable"]
    #[inline(always)]
    pub fn df(&self) -> DF_R {
        DF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI data fetch delay enable"]
    #[inline(always)]
    pub fn df(&mut self) -> DF_W {
        DF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x20 SPIn Data Fetch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [df](index.html) module"]
pub struct DF_SPEC;
impl crate::RegisterSpec for DF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [df::R](R) reader structure"]
impl crate::Readable for DF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [df::W](W) writer structure"]
impl crate::Writable for DF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DF to value 0"]
impl crate::Resettable for DF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
