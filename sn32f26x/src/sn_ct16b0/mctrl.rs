#[doc = "Register `MCTRL` reader"]
pub struct R(crate::R<MCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRL` writer"]
pub struct W(crate::W<MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL_SPEC>;
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
impl From<crate::W<MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable generating an interrupt when MR0 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Generating an interrupt when MR0 matches TC"]
    ENABLE = 1,
}
impl From<MR0IE_A> for bool {
    #[inline(always)]
    fn from(variant: MR0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0IE` reader - Enable generating an interrupt when MR0 matches TC"]
pub type MR0IE_R = crate::BitReader<MR0IE_A>;
impl MR0IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0IE_A {
        match self.bits {
            false => MR0IE_A::DISABLE,
            true => MR0IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR0IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR0IE_A::ENABLE
    }
}
#[doc = "Field `MR0IE` writer - Enable generating an interrupt when MR0 matches TC"]
pub type MR0IE_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR0IE_A, 0>;
impl<'a> MR0IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR0IE_A::DISABLE)
    }
    #[doc = "Generating an interrupt when MR0 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR0IE_A::ENABLE)
    }
}
#[doc = "Enable reset TC when MR0 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0RST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Reset TC when MR0 matches TC"]
    ENABLE = 1,
}
impl From<MR0RST_A> for bool {
    #[inline(always)]
    fn from(variant: MR0RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0RST` reader - Enable reset TC when MR0 matches TC"]
pub type MR0RST_R = crate::BitReader<MR0RST_A>;
impl MR0RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0RST_A {
        match self.bits {
            false => MR0RST_A::DISABLE,
            true => MR0RST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR0RST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR0RST_A::ENABLE
    }
}
#[doc = "Field `MR0RST` writer - Enable reset TC when MR0 matches TC"]
pub type MR0RST_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR0RST_A, 1>;
impl<'a> MR0RST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR0RST_A::DISABLE)
    }
    #[doc = "Reset TC when MR0 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR0RST_A::ENABLE)
    }
}
#[doc = "Stop TC and PC and clear CEN bit when MR0 matches TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0STOP_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Stop TC and PC and clear CEN bit when MR0 matches TC"]
    ENABLE = 1,
}
impl From<MR0STOP_A> for bool {
    #[inline(always)]
    fn from(variant: MR0STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0STOP` reader - Stop TC and PC and clear CEN bit when MR0 matches TC"]
pub type MR0STOP_R = crate::BitReader<MR0STOP_A>;
impl MR0STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0STOP_A {
        match self.bits {
            false => MR0STOP_A::DISABLE,
            true => MR0STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MR0STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MR0STOP_A::ENABLE
    }
}
#[doc = "Field `MR0STOP` writer - Stop TC and PC and clear CEN bit when MR0 matches TC"]
pub type MR0STOP_W<'a> = crate::BitWriter<'a, u32, MCTRL_SPEC, MR0STOP_A, 2>;
impl<'a> MR0STOP_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MR0STOP_A::DISABLE)
    }
    #[doc = "Stop TC and PC and clear CEN bit when MR0 matches TC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MR0STOP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable generating an interrupt when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0ie(&self) -> MR0IE_R {
        MR0IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable reset TC when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0rst(&self) -> MR0RST_R {
        MR0RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop TC and PC and clear CEN bit when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0stop(&self) -> MR0STOP_R {
        MR0STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable generating an interrupt when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0ie(&mut self) -> MR0IE_W {
        MR0IE_W::new(self)
    }
    #[doc = "Bit 1 - Enable reset TC when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0rst(&mut self) -> MR0RST_W {
        MR0RST_W::new(self)
    }
    #[doc = "Bit 2 - Stop TC and PC and clear CEN bit when MR0 matches TC"]
    #[inline(always)]
    pub fn mr0stop(&mut self) -> MR0STOP_W {
        MR0STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x14 CT16Bn Match Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](index.html) module"]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctrl::R](R) reader structure"]
impl crate::Readable for MCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrl::W](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
