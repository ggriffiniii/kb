#[doc = "Register `SGCTL` reader"]
pub struct R(crate::R<SGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SGCTL` writer"]
pub struct W(crate::W<SGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SGCTL_SPEC>;
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
impl From<crate::W<SGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB D- state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_DN_A {
    #[doc = "0: D- state is low"]
    LOW = 0,
    #[doc = "1: D- state is high"]
    HIGH = 1,
}
impl From<BUS_DN_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_DN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_DN` reader - USB D- state"]
pub type BUS_DN_R = crate::BitReader<BUS_DN_A>;
impl BUS_DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_DN_A {
        match self.bits {
            false => BUS_DN_A::LOW,
            true => BUS_DN_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == BUS_DN_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == BUS_DN_A::HIGH
    }
}
#[doc = "Field `BUS_DN` writer - USB D- state"]
pub type BUS_DN_W<'a> = crate::BitWriter<'a, u32, SGCTL_SPEC, BUS_DN_A, 0>;
impl<'a> BUS_DN_W<'a> {
    #[doc = "D- state is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(BUS_DN_A::LOW)
    }
    #[doc = "D- state is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(BUS_DN_A::HIGH)
    }
}
#[doc = "USB DP state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_DP_A {
    #[doc = "0: D+ state is low"]
    LOW = 0,
    #[doc = "1: D+ state is high"]
    HIGH = 1,
}
impl From<BUS_DP_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_DP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_DP` reader - USB DP state"]
pub type BUS_DP_R = crate::BitReader<BUS_DP_A>;
impl BUS_DP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_DP_A {
        match self.bits {
            false => BUS_DP_A::LOW,
            true => BUS_DP_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == BUS_DP_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == BUS_DP_A::HIGH
    }
}
#[doc = "Field `BUS_DP` writer - USB DP state"]
pub type BUS_DP_W<'a> = crate::BitWriter<'a, u32, SGCTL_SPEC, BUS_DP_A, 1>;
impl<'a> BUS_DP_W<'a> {
    #[doc = "D+ state is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(BUS_DP_A::LOW)
    }
    #[doc = "D+ state is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(BUS_DP_A::HIGH)
    }
}
#[doc = "Enable to drive USB bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_DRVEN_A {
    #[doc = "0: Not to drive USB bus"]
    DISABLE = 0,
    #[doc = "1: Drive USB bus"]
    ENABLE = 1,
}
impl From<BUS_DRVEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_DRVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_DRVEN` reader - Enable to drive USB bus"]
pub type BUS_DRVEN_R = crate::BitReader<BUS_DRVEN_A>;
impl BUS_DRVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_DRVEN_A {
        match self.bits {
            false => BUS_DRVEN_A::DISABLE,
            true => BUS_DRVEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUS_DRVEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUS_DRVEN_A::ENABLE
    }
}
#[doc = "Field `BUS_DRVEN` writer - Enable to drive USB bus"]
pub type BUS_DRVEN_W<'a> = crate::BitWriter<'a, u32, SGCTL_SPEC, BUS_DRVEN_A, 2>;
impl<'a> BUS_DRVEN_W<'a> {
    #[doc = "Not to drive USB bus"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BUS_DRVEN_A::DISABLE)
    }
    #[doc = "Drive USB bus"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BUS_DRVEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - USB D- state"]
    #[inline(always)]
    pub fn bus_dn(&self) -> BUS_DN_R {
        BUS_DN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB DP state"]
    #[inline(always)]
    pub fn bus_dp(&self) -> BUS_DP_R {
        BUS_DP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable to drive USB bus"]
    #[inline(always)]
    pub fn bus_drven(&self) -> BUS_DRVEN_R {
        BUS_DRVEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB D- state"]
    #[inline(always)]
    pub fn bus_dn(&mut self) -> BUS_DN_W {
        BUS_DN_W::new(self)
    }
    #[doc = "Bit 1 - USB DP state"]
    #[inline(always)]
    pub fn bus_dp(&mut self) -> BUS_DP_W {
        BUS_DP_W::new(self)
    }
    #[doc = "Bit 2 - Enable to drive USB bus"]
    #[inline(always)]
    pub fn bus_drven(&mut self) -> BUS_DRVEN_W {
        BUS_DRVEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x14 USB Signal Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sgctl](index.html) module"]
pub struct SGCTL_SPEC;
impl crate::RegisterSpec for SGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sgctl::R](R) reader structure"]
impl crate::Readable for SGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sgctl::W](W) writer structure"]
impl crate::Writable for SGCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SGCTL to value 0"]
impl crate::Resettable for SGCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
