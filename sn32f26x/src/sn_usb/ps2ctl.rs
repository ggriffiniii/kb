#[doc = "Register `PS2CTL` reader"]
pub struct R(crate::R<PS2CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS2CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS2CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS2CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS2CTL` writer"]
pub struct W(crate::W<PS2CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS2CTL_SPEC>;
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
impl From<crate::W<PS2CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS2CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PS/2 SCK mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKM_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<SCKM_A> for bool {
    #[inline(always)]
    fn from(variant: SCKM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKM` reader - PS/2 SCK mode control bit"]
pub type SCKM_R = crate::BitReader<SCKM_A>;
impl SCKM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKM_A {
        match self.bits {
            false => SCKM_A::DISABLE,
            true => SCKM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCKM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCKM_A::ENABLE
    }
}
#[doc = "Field `SCKM` writer - PS/2 SCK mode control bit"]
pub type SCKM_W<'a> = crate::BitWriter<'a, u32, PS2CTL_SPEC, SCKM_A, 0>;
impl<'a> SCKM_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCKM_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCKM_A::ENABLE)
    }
}
#[doc = "PS/2 SDA mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDAM_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<SDAM_A> for bool {
    #[inline(always)]
    fn from(variant: SDAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDAM` reader - PS/2 SDA mode control bit"]
pub type SDAM_R = crate::BitReader<SDAM_A>;
impl SDAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDAM_A {
        match self.bits {
            false => SDAM_A::DISABLE,
            true => SDAM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDAM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDAM_A::ENABLE
    }
}
#[doc = "Field `SDAM` writer - PS/2 SDA mode control bit"]
pub type SDAM_W<'a> = crate::BitWriter<'a, u32, PS2CTL_SPEC, SDAM_A, 1>;
impl<'a> SDAM_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDAM_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDAM_A::ENABLE)
    }
}
#[doc = "PS/2 SCK data buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCK_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<SCK_A> for bool {
    #[inline(always)]
    fn from(variant: SCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCK` reader - PS/2 SCK data buffer"]
pub type SCK_R = crate::BitReader<SCK_A>;
impl SCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCK_A {
        match self.bits {
            false => SCK_A::DISABLE,
            true => SCK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCK_A::ENABLE
    }
}
#[doc = "Field `SCK` writer - PS/2 SCK data buffer"]
pub type SCK_W<'a> = crate::BitWriter<'a, u32, PS2CTL_SPEC, SCK_A, 2>;
impl<'a> SCK_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCK_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCK_A::ENABLE)
    }
}
#[doc = "PS/2 SDA data buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<SDA_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDA` reader - PS/2 SDA data buffer"]
pub type SDA_R = crate::BitReader<SDA_A>;
impl SDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDA_A {
        match self.bits {
            false => SDA_A::DISABLE,
            true => SDA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDA_A::ENABLE
    }
}
#[doc = "Field `SDA` writer - PS/2 SDA data buffer"]
pub type SDA_W<'a> = crate::BitWriter<'a, u32, PS2CTL_SPEC, SDA_A, 3>;
impl<'a> SDA_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDA_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDA_A::ENABLE)
    }
}
#[doc = "PS/2 internal 5kohm pull-up resistor control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS2ENB_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<PS2ENB_A> for bool {
    #[inline(always)]
    fn from(variant: PS2ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS2ENB` reader - PS/2 internal 5kohm pull-up resistor control bit"]
pub type PS2ENB_R = crate::BitReader<PS2ENB_A>;
impl PS2ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS2ENB_A {
        match self.bits {
            false => PS2ENB_A::DISABLE,
            true => PS2ENB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PS2ENB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PS2ENB_A::ENABLE
    }
}
#[doc = "Field `PS2ENB` writer - PS/2 internal 5kohm pull-up resistor control bit"]
pub type PS2ENB_W<'a> = crate::BitWriter<'a, u32, PS2CTL_SPEC, PS2ENB_A, 31>;
impl<'a> PS2ENB_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PS2ENB_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PS2ENB_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - PS/2 SCK mode control bit"]
    #[inline(always)]
    pub fn sckm(&self) -> SCKM_R {
        SCKM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PS/2 SDA mode control bit"]
    #[inline(always)]
    pub fn sdam(&self) -> SDAM_R {
        SDAM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PS/2 SCK data buffer"]
    #[inline(always)]
    pub fn sck(&self) -> SCK_R {
        SCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PS/2 SDA data buffer"]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - PS/2 internal 5kohm pull-up resistor control bit"]
    #[inline(always)]
    pub fn ps2enb(&self) -> PS2ENB_R {
        PS2ENB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PS/2 SCK mode control bit"]
    #[inline(always)]
    pub fn sckm(&mut self) -> SCKM_W {
        SCKM_W::new(self)
    }
    #[doc = "Bit 1 - PS/2 SDA mode control bit"]
    #[inline(always)]
    pub fn sdam(&mut self) -> SDAM_W {
        SDAM_W::new(self)
    }
    #[doc = "Bit 2 - PS/2 SCK data buffer"]
    #[inline(always)]
    pub fn sck(&mut self) -> SCK_W {
        SCK_W::new(self)
    }
    #[doc = "Bit 3 - PS/2 SDA data buffer"]
    #[inline(always)]
    pub fn sda(&mut self) -> SDA_W {
        SDA_W::new(self)
    }
    #[doc = "Bit 31 - PS/2 internal 5kohm pull-up resistor control bit"]
    #[inline(always)]
    pub fn ps2enb(&mut self) -> PS2ENB_W {
        PS2ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x70 PS/2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps2ctl](index.html) module"]
pub struct PS2CTL_SPEC;
impl crate::RegisterSpec for PS2CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps2ctl::R](R) reader structure"]
impl crate::Readable for PS2CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps2ctl::W](W) writer structure"]
impl crate::Writable for PS2CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PS2CTL to value 0"]
impl crate::Resettable for PS2CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
