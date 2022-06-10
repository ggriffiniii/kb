#[doc = "Register `SLVADDR0` reader"]
pub struct R(crate::R<SLVADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVADDR0` writer"]
pub struct W(crate::W<SLVADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVADDR0_SPEC>;
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
impl From<crate::W<SLVADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - I2Cn slave address 0"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - I2Cn slave address 0"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, SLVADDR0_SPEC, u16, u16, 10, 0>;
#[doc = "General call address enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCEN_A {
    #[doc = "0: Disable general call address"]
    DISABLE = 0,
    #[doc = "1: Enable general call address (0x0)"]
    ENABLE = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - General call address enable"]
pub type GCEN_R = crate::BitReader<GCEN_A>;
impl GCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::DISABLE,
            true => GCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GCEN_A::ENABLE
    }
}
#[doc = "Field `GCEN` writer - General call address enable"]
pub type GCEN_W<'a> = crate::BitWriter<'a, u32, SLVADDR0_SPEC, GCEN_A, 30>;
impl<'a> GCEN_W<'a> {
    #[doc = "Disable general call address"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GCEN_A::DISABLE)
    }
    #[doc = "Enable general call address (0x0)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GCEN_A::ENABLE)
    }
}
#[doc = "Slave address mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD_MODE_A {
    #[doc = "0: 7-bit slave address mode"]
    _0 = 0,
    #[doc = "1: 10-bit slave address mode"]
    _1 = 1,
}
impl From<ADD_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADD_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD_MODE` reader - Slave address mode"]
pub type ADD_MODE_R = crate::BitReader<ADD_MODE_A>;
impl ADD_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADD_MODE_A {
        match self.bits {
            false => ADD_MODE_A::_0,
            true => ADD_MODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADD_MODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADD_MODE_A::_1
    }
}
#[doc = "Field `ADD_MODE` writer - Slave address mode"]
pub type ADD_MODE_W<'a> = crate::BitWriter<'a, u32, SLVADDR0_SPEC, ADD_MODE_A, 31>;
impl<'a> ADD_MODE_W<'a> {
    #[doc = "7-bit slave address mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADD_MODE_A::_0)
    }
    #[doc = "10-bit slave address mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADD_MODE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:9 - I2Cn slave address 0"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - General call address enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Slave address mode"]
    #[inline(always)]
    pub fn add_mode(&self) -> ADD_MODE_R {
        ADD_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2Cn slave address 0"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bit 30 - General call address enable"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W::new(self)
    }
    #[doc = "Bit 31 - Slave address mode"]
    #[inline(always)]
    pub fn add_mode(&mut self) -> ADD_MODE_W {
        ADD_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x10 I2Cn Slave Address 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvaddr0](index.html) module"]
pub struct SLVADDR0_SPEC;
impl crate::RegisterSpec for SLVADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvaddr0::R](R) reader structure"]
impl crate::Readable for SLVADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvaddr0::W](W) writer structure"]
impl crate::Writable for SLVADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLVADDR0 to value 0"]
impl crate::Resettable for SLVADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
