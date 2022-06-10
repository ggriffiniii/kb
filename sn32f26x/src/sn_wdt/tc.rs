#[doc = "Register `TC` reader"]
pub struct R(crate::R<TC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TC` writer"]
pub struct W(crate::W<TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_SPEC>;
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
impl From<crate::W<TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TC` reader - Watchdog timer constant reload value"]
pub type TC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TC` writer - Watchdog timer constant reload value"]
pub type TC_W<'a> = crate::FieldWriter<'a, u32, TC_SPEC, u8, u8, 8, 0>;
#[doc = "Field `WDKEY` writer - WDT register key"]
pub type WDKEY_W<'a> = crate::FieldWriter<'a, u32, TC_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:7 - Watchdog timer constant reload value"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog timer constant reload value"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W::new(self)
    }
    #[doc = "Bits 16:31 - WDT register key"]
    #[inline(always)]
    pub fn wdkey(&mut self) -> WDKEY_W {
        WDKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x08 WDT Timer Constant Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc](index.html) module"]
pub struct TC_SPEC;
impl crate::RegisterSpec for TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc::R](R) reader structure"]
impl crate::Readable for TC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tc::W](W) writer structure"]
impl crate::Writable for TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC to value 0xff"]
impl crate::Resettable for TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
