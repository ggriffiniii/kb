#[doc = "Register `PHYPRM2` reader"]
pub struct R(crate::R<PHYPRM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYPRM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYPRM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYPRM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYPRM2` writer"]
pub struct W(crate::W<PHYPRM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYPRM2_SPEC>;
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
impl From<crate::W<PHYPRM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYPRM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_PS` reader - USB PHY parameter 2"]
pub type PHY_PS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PHY_PS` writer - USB PHY parameter 2"]
pub type PHY_PS_W<'a> = crate::FieldWriter<'a, u32, PHYPRM2_SPEC, u16, u16, 15, 0>;
impl R {
    #[doc = "Bits 0:14 - USB PHY parameter 2"]
    #[inline(always)]
    pub fn phy_ps(&self) -> PHY_PS_R {
        PHY_PS_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - USB PHY parameter 2"]
    #[inline(always)]
    pub fn phy_ps(&mut self) -> PHY_PS_W {
        PHY_PS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x6C USB PHY Parameter Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phyprm2](index.html) module"]
pub struct PHYPRM2_SPEC;
impl crate::RegisterSpec for PHYPRM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phyprm2::R](R) reader structure"]
impl crate::Readable for PHYPRM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phyprm2::W](W) writer structure"]
impl crate::Writable for PHYPRM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHYPRM2 to value 0x0400"]
impl crate::Resettable for PHYPRM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
