#[doc = "Register `EM` reader"]
pub struct R(crate::R<EM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM` writer"]
pub struct W(crate::W<EM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM_SPEC>;
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
impl From<crate::W<EM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM0` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM0 output high or Low."]
pub type EM0_R = crate::BitReader<bool>;
#[doc = "Field `EM0` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM0 output high or Low."]
pub type EM0_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 0>;
#[doc = "Field `EM1` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM1 output high or Low."]
pub type EM1_R = crate::BitReader<bool>;
#[doc = "Field `EM1` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM1 output high or Low."]
pub type EM1_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 1>;
#[doc = "Field `EM2` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM2 output high or Low."]
pub type EM2_R = crate::BitReader<bool>;
#[doc = "Field `EM2` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM2 output high or Low."]
pub type EM2_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 2>;
#[doc = "Field `EM3` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM3 output high or Low."]
pub type EM3_R = crate::BitReader<bool>;
#[doc = "Field `EM3` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM3 output high or Low."]
pub type EM3_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 3>;
#[doc = "Field `EM4` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM4 output high or Low."]
pub type EM4_R = crate::BitReader<bool>;
#[doc = "Field `EM4` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM4 output high or Low."]
pub type EM4_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 4>;
#[doc = "Field `EM5` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM5 output high or Low."]
pub type EM5_R = crate::BitReader<bool>;
#[doc = "Field `EM5` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM5 output high or Low."]
pub type EM5_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 5>;
#[doc = "Field `EM6` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM6 output high or Low."]
pub type EM6_R = crate::BitReader<bool>;
#[doc = "Field `EM6` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM6 output high or Low."]
pub type EM6_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 6>;
#[doc = "Field `EM7` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM7 output high or Low."]
pub type EM7_R = crate::BitReader<bool>;
#[doc = "Field `EM7` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM7 output high or Low."]
pub type EM7_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 7>;
#[doc = "Field `EM8` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM8 output high or Low."]
pub type EM8_R = crate::BitReader<bool>;
#[doc = "Field `EM8` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM8 output high or Low."]
pub type EM8_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 8>;
#[doc = "Field `EM9` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM9 output high or Low."]
pub type EM9_R = crate::BitReader<bool>;
#[doc = "Field `EM9` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM9 output high or Low."]
pub type EM9_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 9>;
#[doc = "Field `EM10` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM10 output high or Low."]
pub type EM10_R = crate::BitReader<bool>;
#[doc = "Field `EM10` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM10 output high or Low."]
pub type EM10_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 10>;
#[doc = "Field `EM11` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM11 output high or Low."]
pub type EM11_R = crate::BitReader<bool>;
#[doc = "Field `EM11` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM11 output high or Low."]
pub type EM11_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 11>;
#[doc = "Field `EM12` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM12 output high or Low."]
pub type EM12_R = crate::BitReader<bool>;
#[doc = "Field `EM12` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM12 output high or Low."]
pub type EM12_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 12>;
#[doc = "Field `EM13` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM13 output high or Low."]
pub type EM13_R = crate::BitReader<bool>;
#[doc = "Field `EM13` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM13 output high or Low."]
pub type EM13_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 13>;
#[doc = "Field `EM14` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM14 output high or Low."]
pub type EM14_R = crate::BitReader<bool>;
#[doc = "Field `EM14` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM14 output high or Low."]
pub type EM14_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 14>;
#[doc = "Field `EM15` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM15 output high or Low."]
pub type EM15_R = crate::BitReader<bool>;
#[doc = "Field `EM15` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM15 output high or Low."]
pub type EM15_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 15>;
#[doc = "Field `EM16` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM16 output high or Low."]
pub type EM16_R = crate::BitReader<bool>;
#[doc = "Field `EM16` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM16 output high or Low."]
pub type EM16_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 16>;
#[doc = "Field `EM17` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM17 output high or Low."]
pub type EM17_R = crate::BitReader<bool>;
#[doc = "Field `EM17` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM17 output high or Low."]
pub type EM17_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 17>;
#[doc = "Field `EM18` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM18 output high or Low."]
pub type EM18_R = crate::BitReader<bool>;
#[doc = "Field `EM18` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM18 output high or Low."]
pub type EM18_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 18>;
#[doc = "Field `EM19` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM19 output high or Low."]
pub type EM19_R = crate::BitReader<bool>;
#[doc = "Field `EM19` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM19 output high or Low."]
pub type EM19_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 19>;
#[doc = "Field `EM20` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM20 output high or Low."]
pub type EM20_R = crate::BitReader<bool>;
#[doc = "Field `EM20` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM20 output high or Low."]
pub type EM20_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 20>;
#[doc = "Field `EM21` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM21 output high or Low."]
pub type EM21_R = crate::BitReader<bool>;
#[doc = "Field `EM21` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM21 output high or Low."]
pub type EM21_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 21>;
#[doc = "Field `EM22` reader - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM22 output high or Low."]
pub type EM22_R = crate::BitReader<bool>;
#[doc = "Field `EM22` writer - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM22 output high or Low."]
pub type EM22_W<'a> = crate::BitWriter<'a, u32, EM_SPEC, bool, 22>;
impl R {
    #[doc = "Bit 0 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM0 output high or Low."]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM1 output high or Low."]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM2 output high or Low."]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM3 output high or Low."]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM4 output high or Low."]
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM5 output high or Low."]
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM6 output high or Low."]
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM7 output high or Low."]
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM8 output high or Low."]
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM9 output high or Low."]
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM10 output high or Low."]
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM11 output high or Low."]
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM12 output high or Low."]
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM13 output high or Low."]
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM14 output high or Low."]
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM15 output high or Low."]
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM16 output high or Low."]
    #[inline(always)]
    pub fn em16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM17 output high or Low."]
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM18 output high or Low."]
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM19 output high or Low."]
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM20 output high or Low."]
    #[inline(always)]
    pub fn em20(&self) -> EM20_R {
        EM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM21 output high or Low."]
    #[inline(always)]
    pub fn em21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM22 output high or Low."]
    #[inline(always)]
    pub fn em22(&self) -> EM22_R {
        EM22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM0 output high or Low."]
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W {
        EM0_W::new(self)
    }
    #[doc = "Bit 1 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM1 output high or Low."]
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W {
        EM1_W::new(self)
    }
    #[doc = "Bit 2 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM2 output high or Low."]
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W {
        EM2_W::new(self)
    }
    #[doc = "Bit 3 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM3 output high or Low."]
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W {
        EM3_W::new(self)
    }
    #[doc = "Bit 4 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM4 output high or Low."]
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W {
        EM4_W::new(self)
    }
    #[doc = "Bit 5 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM5 output high or Low."]
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W {
        EM5_W::new(self)
    }
    #[doc = "Bit 6 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM6 output high or Low."]
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W {
        EM6_W::new(self)
    }
    #[doc = "Bit 7 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM7 output high or Low."]
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W {
        EM7_W::new(self)
    }
    #[doc = "Bit 8 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM8 output high or Low."]
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W {
        EM8_W::new(self)
    }
    #[doc = "Bit 9 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM9 output high or Low."]
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W {
        EM9_W::new(self)
    }
    #[doc = "Bit 10 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM10 output high or Low."]
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W {
        EM10_W::new(self)
    }
    #[doc = "Bit 11 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM11 output high or Low."]
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W {
        EM11_W::new(self)
    }
    #[doc = "Bit 12 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM12 output high or Low."]
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W {
        EM12_W::new(self)
    }
    #[doc = "Bit 13 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM13 output high or Low."]
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W {
        EM13_W::new(self)
    }
    #[doc = "Bit 14 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM14 output high or Low."]
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W {
        EM14_W::new(self)
    }
    #[doc = "Bit 15 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM15 output high or Low."]
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W {
        EM15_W::new(self)
    }
    #[doc = "Bit 16 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM16 output high or Low."]
    #[inline(always)]
    pub fn em16(&mut self) -> EM16_W {
        EM16_W::new(self)
    }
    #[doc = "Bit 17 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM17 output high or Low."]
    #[inline(always)]
    pub fn em17(&mut self) -> EM17_W {
        EM17_W::new(self)
    }
    #[doc = "Bit 18 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM18 output high or Low."]
    #[inline(always)]
    pub fn em18(&mut self) -> EM18_W {
        EM18_W::new(self)
    }
    #[doc = "Bit 19 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM19 output high or Low."]
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W {
        EM19_W::new(self)
    }
    #[doc = "Bit 20 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM20 output high or Low."]
    #[inline(always)]
    pub fn em20(&mut self) -> EM20_W {
        EM20_W::new(self)
    }
    #[doc = "Bit 21 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM21 output high or Low."]
    #[inline(always)]
    pub fn em21(&mut self) -> EM21_W {
        EM21_W::new(self)
    }
    #[doc = "Bit 22 - When FW write this bit 1/0, MCU will drive the state of CT16Bn_PWM22 output high or Low."]
    #[inline(always)]
    pub fn em22(&mut self) -> EM22_W {
        EM22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x88 CT16Bn External Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em](index.html) module"]
pub struct EM_SPEC;
impl crate::RegisterSpec for EM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em::R](R) reader structure"]
impl crate::Readable for EM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em::W](W) writer structure"]
impl crate::Writable for EM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM to value 0"]
impl crate::Resettable for EM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
