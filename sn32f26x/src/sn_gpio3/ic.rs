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
#[doc = "Pn.0 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC0_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.0"]
    CLEAR = 1,
}
impl From<IC0_AW> for bool {
    #[inline(always)]
    fn from(variant: IC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC0` writer - Pn.0 interrupt flag clear"]
pub type IC0_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC0_AW, 0>;
impl<'a> IC0_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC0_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC0_AW::CLEAR)
    }
}
#[doc = "Pn.1 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC1_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.1"]
    CLEAR = 1,
}
impl From<IC1_AW> for bool {
    #[inline(always)]
    fn from(variant: IC1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC1` writer - Pn.1 interrupt flag clear"]
pub type IC1_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC1_AW, 1>;
impl<'a> IC1_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC1_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC1_AW::CLEAR)
    }
}
#[doc = "Pn.2 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC2_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.2"]
    CLEAR = 1,
}
impl From<IC2_AW> for bool {
    #[inline(always)]
    fn from(variant: IC2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC2` writer - Pn.2 interrupt flag clear"]
pub type IC2_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC2_AW, 2>;
impl<'a> IC2_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC2_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.2"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC2_AW::CLEAR)
    }
}
#[doc = "Pn.3 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC3_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.3"]
    CLEAR = 1,
}
impl From<IC3_AW> for bool {
    #[inline(always)]
    fn from(variant: IC3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC3` writer - Pn.3 interrupt flag clear"]
pub type IC3_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC3_AW, 3>;
impl<'a> IC3_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC3_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.3"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC3_AW::CLEAR)
    }
}
#[doc = "Pn.4 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC4_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.4"]
    CLEAR = 1,
}
impl From<IC4_AW> for bool {
    #[inline(always)]
    fn from(variant: IC4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC4` writer - Pn.4 interrupt flag clear"]
pub type IC4_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC4_AW, 4>;
impl<'a> IC4_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC4_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.4"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC4_AW::CLEAR)
    }
}
#[doc = "Pn.5 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC5_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.5"]
    CLEAR = 1,
}
impl From<IC5_AW> for bool {
    #[inline(always)]
    fn from(variant: IC5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC5` writer - Pn.5 interrupt flag clear"]
pub type IC5_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC5_AW, 5>;
impl<'a> IC5_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC5_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.5"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC5_AW::CLEAR)
    }
}
#[doc = "Pn.6 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC6_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.6"]
    CLEAR = 1,
}
impl From<IC6_AW> for bool {
    #[inline(always)]
    fn from(variant: IC6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC6` writer - Pn.6 interrupt flag clear"]
pub type IC6_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC6_AW, 6>;
impl<'a> IC6_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC6_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.6"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC6_AW::CLEAR)
    }
}
#[doc = "Pn.7 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC7_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.7"]
    CLEAR = 1,
}
impl From<IC7_AW> for bool {
    #[inline(always)]
    fn from(variant: IC7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC7` writer - Pn.7 interrupt flag clear"]
pub type IC7_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC7_AW, 7>;
impl<'a> IC7_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC7_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.7"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC7_AW::CLEAR)
    }
}
#[doc = "Pn.8 interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC8_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear interrupt flag on Pn.8"]
    CLEAR = 1,
}
impl From<IC8_AW> for bool {
    #[inline(always)]
    fn from(variant: IC8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC8` writer - Pn.8 interrupt flag clear"]
pub type IC8_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, IC8_AW, 8>;
impl<'a> IC8_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(IC8_AW::NOEFFECT)
    }
    #[doc = "Clear interrupt flag on Pn.8"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IC8_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - Pn.0 interrupt flag clear"]
    #[inline(always)]
    pub fn ic0(&mut self) -> IC0_W {
        IC0_W::new(self)
    }
    #[doc = "Bit 1 - Pn.1 interrupt flag clear"]
    #[inline(always)]
    pub fn ic1(&mut self) -> IC1_W {
        IC1_W::new(self)
    }
    #[doc = "Bit 2 - Pn.2 interrupt flag clear"]
    #[inline(always)]
    pub fn ic2(&mut self) -> IC2_W {
        IC2_W::new(self)
    }
    #[doc = "Bit 3 - Pn.3 interrupt flag clear"]
    #[inline(always)]
    pub fn ic3(&mut self) -> IC3_W {
        IC3_W::new(self)
    }
    #[doc = "Bit 4 - Pn.4 interrupt flag clear"]
    #[inline(always)]
    pub fn ic4(&mut self) -> IC4_W {
        IC4_W::new(self)
    }
    #[doc = "Bit 5 - Pn.5 interrupt flag clear"]
    #[inline(always)]
    pub fn ic5(&mut self) -> IC5_W {
        IC5_W::new(self)
    }
    #[doc = "Bit 6 - Pn.6 interrupt flag clear"]
    #[inline(always)]
    pub fn ic6(&mut self) -> IC6_W {
        IC6_W::new(self)
    }
    #[doc = "Bit 7 - Pn.7 interrupt flag clear"]
    #[inline(always)]
    pub fn ic7(&mut self) -> IC7_W {
        IC7_W::new(self)
    }
    #[doc = "Bit 8 - Pn.8 interrupt flag clear"]
    #[inline(always)]
    pub fn ic8(&mut self) -> IC8_W {
        IC8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x20 GPIO Port n Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](index.html) module"]
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
