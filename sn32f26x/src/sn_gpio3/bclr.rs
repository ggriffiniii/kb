#[doc = "Register `BCLR` writer"]
pub struct W(crate::W<BCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCLR_SPEC>;
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
impl From<crate::W<BCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Pn.0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR0_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.0"]
    CLEAR = 1,
}
impl From<BCLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR0` writer - Clear Pn.0"]
pub type BCLR0_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR0_AW, 0>;
impl<'a> BCLR0_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR0_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR0_AW::CLEAR)
    }
}
#[doc = "Clear Pn.1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR1_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.1"]
    CLEAR = 1,
}
impl From<BCLR1_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR1` writer - Clear Pn.1"]
pub type BCLR1_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR1_AW, 1>;
impl<'a> BCLR1_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR1_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR1_AW::CLEAR)
    }
}
#[doc = "Clear Pn.2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR2_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.2"]
    CLEAR = 1,
}
impl From<BCLR2_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR2` writer - Clear Pn.2"]
pub type BCLR2_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR2_AW, 2>;
impl<'a> BCLR2_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR2_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.2"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR2_AW::CLEAR)
    }
}
#[doc = "Clear Pn.3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR3_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.3"]
    CLEAR = 1,
}
impl From<BCLR3_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR3` writer - Clear Pn.3"]
pub type BCLR3_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR3_AW, 3>;
impl<'a> BCLR3_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR3_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.3"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR3_AW::CLEAR)
    }
}
#[doc = "Clear Pn.4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR4_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.4"]
    CLEAR = 1,
}
impl From<BCLR4_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR4` writer - Clear Pn.4"]
pub type BCLR4_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR4_AW, 4>;
impl<'a> BCLR4_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR4_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.4"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR4_AW::CLEAR)
    }
}
#[doc = "Clear Pn.5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR5_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.5"]
    CLEAR = 1,
}
impl From<BCLR5_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR5` writer - Clear Pn.5"]
pub type BCLR5_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR5_AW, 5>;
impl<'a> BCLR5_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR5_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.5"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR5_AW::CLEAR)
    }
}
#[doc = "Clear Pn.6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR6_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.6"]
    CLEAR = 1,
}
impl From<BCLR6_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR6` writer - Clear Pn.6"]
pub type BCLR6_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR6_AW, 6>;
impl<'a> BCLR6_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR6_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.6"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR6_AW::CLEAR)
    }
}
#[doc = "Clear Pn.7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR7_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.7"]
    CLEAR = 1,
}
impl From<BCLR7_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR7` writer - Clear Pn.7"]
pub type BCLR7_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR7_AW, 7>;
impl<'a> BCLR7_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR7_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.7"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR7_AW::CLEAR)
    }
}
#[doc = "Clear Pn.8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLR8_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear Pn.8"]
    CLEAR = 1,
}
impl From<BCLR8_AW> for bool {
    #[inline(always)]
    fn from(variant: BCLR8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLR8` writer - Clear Pn.8"]
pub type BCLR8_W<'a> = crate::BitWriter<'a, u32, BCLR_SPEC, BCLR8_AW, 8>;
impl<'a> BCLR8_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BCLR8_AW::NOEFFECT)
    }
    #[doc = "Clear Pn.8"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCLR8_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Pn.0"]
    #[inline(always)]
    pub fn bclr0(&mut self) -> BCLR0_W {
        BCLR0_W::new(self)
    }
    #[doc = "Bit 1 - Clear Pn.1"]
    #[inline(always)]
    pub fn bclr1(&mut self) -> BCLR1_W {
        BCLR1_W::new(self)
    }
    #[doc = "Bit 2 - Clear Pn.2"]
    #[inline(always)]
    pub fn bclr2(&mut self) -> BCLR2_W {
        BCLR2_W::new(self)
    }
    #[doc = "Bit 3 - Clear Pn.3"]
    #[inline(always)]
    pub fn bclr3(&mut self) -> BCLR3_W {
        BCLR3_W::new(self)
    }
    #[doc = "Bit 4 - Clear Pn.4"]
    #[inline(always)]
    pub fn bclr4(&mut self) -> BCLR4_W {
        BCLR4_W::new(self)
    }
    #[doc = "Bit 5 - Clear Pn.5"]
    #[inline(always)]
    pub fn bclr5(&mut self) -> BCLR5_W {
        BCLR5_W::new(self)
    }
    #[doc = "Bit 6 - Clear Pn.6"]
    #[inline(always)]
    pub fn bclr6(&mut self) -> BCLR6_W {
        BCLR6_W::new(self)
    }
    #[doc = "Bit 7 - Clear Pn.7"]
    #[inline(always)]
    pub fn bclr7(&mut self) -> BCLR7_W {
        BCLR7_W::new(self)
    }
    #[doc = "Bit 8 - Clear Pn.8"]
    #[inline(always)]
    pub fn bclr8(&mut self) -> BCLR8_W {
        BCLR8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x28 GPIO Port n Bits Clear Operation Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bclr](index.html) module"]
pub struct BCLR_SPEC;
impl crate::RegisterSpec for BCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bclr::W](W) writer structure"]
impl crate::Writable for BCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCLR to value 0"]
impl crate::Resettable for BCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
