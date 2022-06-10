#[doc = "Register `IC` writer"]
pub struct W(crate::W<IC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SPEC>;
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
impl From<crate::W<IC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MR0IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear MR0IF"]
    CLEAR = 1,
}
impl From<MR0IC_AW> for bool {
    #[inline(always)]
    fn from(variant: MR0IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0IC` writer - MR0IF clear bit"]
pub type MR0IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, MR0IC_AW, 0>;
impl<'a> MR0IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(MR0IC_AW::NOEFFECT)
    }
    #[doc = "Clear MR0IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MR0IC_AW::CLEAR)
    }
}
#[doc = "CAP0IF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear CAP0IF"]
    CLEAR = 1,
}
impl From<CAP0IC_AW> for bool {
    #[inline(always)]
    fn from(variant: CAP0IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0IC` writer - CAP0IF clear bit"]
pub type CAP0IC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, CAP0IC_AW, 24>;
impl<'a> CAP0IC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(CAP0IC_AW::NOEFFECT)
    }
    #[doc = "Clear CAP0IF"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAP0IC_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - MR0IF clear bit"]
    #[inline(always)]
    pub fn mr0ic(&mut self) -> MR0IC_W {
        MR0IC_W::new(self)
    }
    #[doc = "Bit 24 - CAP0IF clear bit"]
    #[inline(always)]
    pub fn cap0ic(&mut self) -> CAP0IC_W {
        CAP0IC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0xA8 CT16Bn Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](index.html) module"]
pub struct IC_SPEC;
impl crate::RegisterSpec for IC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ic::W](W) writer structure"]
impl crate::Writable for IC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
