#[doc = "Register `PHYPRM` reader"]
pub struct R(crate::R<PHYPRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYPRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYPRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYPRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYPRM` writer"]
pub struct W(crate::W<PHYPRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYPRM_SPEC>;
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
impl From<crate::W<PHYPRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYPRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_PARAM` reader - USB PHY parameter"]
pub type PHY_PARAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHY_PARAM` writer - USB PHY parameter"]
pub type PHY_PARAM_W<'a> = crate::FieldWriter<'a, u32, PHYPRM_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - USB PHY parameter"]
    #[inline(always)]
    pub fn phy_param(&self) -> PHY_PARAM_R {
        PHY_PARAM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - USB PHY parameter"]
    #[inline(always)]
    pub fn phy_param(&mut self) -> PHY_PARAM_W {
        PHY_PARAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x64 USB PHY Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phyprm](index.html) module"]
pub struct PHYPRM_SPEC;
impl crate::RegisterSpec for PHYPRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phyprm::R](R) reader structure"]
impl crate::Readable for PHYPRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phyprm::W](W) writer structure"]
impl crate::Writable for PHYPRM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHYPRM to value 0"]
impl crate::Resettable for PHYPRM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}