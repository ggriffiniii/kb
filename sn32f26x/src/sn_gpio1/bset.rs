#[doc = "Register `BSET` writer"]
pub struct W(crate::W<BSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSET_SPEC>;
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
impl From<crate::W<BSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set Pn.0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET0_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.0 to 1"]
    SET = 1,
}
impl From<BSET0_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET0` writer - Set Pn.0"]
pub type BSET0_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET0_AW, 0>;
impl<'a> BSET0_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET0_AW::NOEFFECT)
    }
    #[doc = "Set Pn.0 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET0_AW::SET)
    }
}
#[doc = "Set Pn.1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET1_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.1 to 1"]
    SET = 1,
}
impl From<BSET1_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET1` writer - Set Pn.1"]
pub type BSET1_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET1_AW, 1>;
impl<'a> BSET1_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET1_AW::NOEFFECT)
    }
    #[doc = "Set Pn.1 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET1_AW::SET)
    }
}
#[doc = "Set Pn.2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET2_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.2 to 1"]
    SET = 1,
}
impl From<BSET2_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET2` writer - Set Pn.2"]
pub type BSET2_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET2_AW, 2>;
impl<'a> BSET2_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET2_AW::NOEFFECT)
    }
    #[doc = "Set Pn.2 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET2_AW::SET)
    }
}
#[doc = "Set Pn.3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET3_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.3 to 1"]
    SET = 1,
}
impl From<BSET3_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET3` writer - Set Pn.3"]
pub type BSET3_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET3_AW, 3>;
impl<'a> BSET3_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET3_AW::NOEFFECT)
    }
    #[doc = "Set Pn.3 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET3_AW::SET)
    }
}
#[doc = "Set Pn.4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET4_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.4 to 1"]
    SET = 1,
}
impl From<BSET4_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET4` writer - Set Pn.4"]
pub type BSET4_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET4_AW, 4>;
impl<'a> BSET4_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET4_AW::NOEFFECT)
    }
    #[doc = "Set Pn.4 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET4_AW::SET)
    }
}
#[doc = "Set Pn.5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSET5_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Set Pn.5 to 1"]
    SET = 1,
}
impl From<BSET5_AW> for bool {
    #[inline(always)]
    fn from(variant: BSET5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSET5` writer - Set Pn.5"]
pub type BSET5_W<'a> = crate::BitWriter<'a, u32, BSET_SPEC, BSET5_AW, 5>;
impl<'a> BSET5_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BSET5_AW::NOEFFECT)
    }
    #[doc = "Set Pn.5 to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSET5_AW::SET)
    }
}
impl W {
    #[doc = "Bit 0 - Set Pn.0"]
    #[inline(always)]
    pub fn bset0(&mut self) -> BSET0_W {
        BSET0_W::new(self)
    }
    #[doc = "Bit 1 - Set Pn.1"]
    #[inline(always)]
    pub fn bset1(&mut self) -> BSET1_W {
        BSET1_W::new(self)
    }
    #[doc = "Bit 2 - Set Pn.2"]
    #[inline(always)]
    pub fn bset2(&mut self) -> BSET2_W {
        BSET2_W::new(self)
    }
    #[doc = "Bit 3 - Set Pn.3"]
    #[inline(always)]
    pub fn bset3(&mut self) -> BSET3_W {
        BSET3_W::new(self)
    }
    #[doc = "Bit 4 - Set Pn.4"]
    #[inline(always)]
    pub fn bset4(&mut self) -> BSET4_W {
        BSET4_W::new(self)
    }
    #[doc = "Bit 5 - Set Pn.5"]
    #[inline(always)]
    pub fn bset5(&mut self) -> BSET5_W {
        BSET5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x24 GPIO Port n Bits Set Operation Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bset](index.html) module"]
pub struct BSET_SPEC;
impl crate::RegisterSpec for BSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bset::W](W) writer structure"]
impl crate::Writable for BSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSET to value 0"]
impl crate::Resettable for BSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
