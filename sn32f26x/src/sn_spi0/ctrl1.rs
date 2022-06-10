#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MSB/LSB seletion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSB_A {
    #[doc = "0: MSB transmit first"]
    MSB = 0,
    #[doc = "1: LSB transmit first"]
    LSB = 1,
}
impl From<MLSB_A> for bool {
    #[inline(always)]
    fn from(variant: MLSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MLSB` reader - MSB/LSB seletion"]
pub type MLSB_R = crate::BitReader<MLSB_A>;
impl MLSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MLSB_A {
        match self.bits {
            false => MLSB_A::MSB,
            true => MLSB_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MLSB_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MLSB_A::LSB
    }
}
#[doc = "Field `MLSB` writer - MSB/LSB seletion"]
pub type MLSB_W<'a> = crate::BitWriter<'a, u32, CTRL1_SPEC, MLSB_A, 0>;
impl<'a> MLSB_W<'a> {
    #[doc = "MSB transmit first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(MLSB_A::MSB)
    }
    #[doc = "LSB transmit first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MLSB_A::LSB)
    }
}
#[doc = "Clock priority selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: SCK idles at low level"]
    LOW = 0,
    #[doc = "1: SCK idles at high level"]
    HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock priority selection"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::LOW,
            true => CPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL_A::HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock priority selection"]
pub type CPOL_W<'a> = crate::BitWriter<'a, u32, CTRL1_SPEC, CPOL_A, 1>;
impl<'a> CPOL_W<'a> {
    #[doc = "SCK idles at low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOL_A::LOW)
    }
    #[doc = "SCK idles at high level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOL_A::HIGH)
    }
}
#[doc = "Clock phase of edge sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: The 1st bit is fixed already, and SCK 1st edge is to receive/transmit data"]
    CPHA0 = 0,
    #[doc = "1: SCK 1st edge is for data transition, and receive/transmit data at 2nd edge"]
    CPHA1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock phase of edge sampling"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::CPHA0,
            true => CPHA_A::CPHA1,
        }
    }
    #[doc = "Checks if the value of the field is `CPHA0`"]
    #[inline(always)]
    pub fn is_cpha0(&self) -> bool {
        *self == CPHA_A::CPHA0
    }
    #[doc = "Checks if the value of the field is `CPHA1`"]
    #[inline(always)]
    pub fn is_cpha1(&self) -> bool {
        *self == CPHA_A::CPHA1
    }
}
#[doc = "Field `CPHA` writer - Clock phase of edge sampling"]
pub type CPHA_W<'a> = crate::BitWriter<'a, u32, CTRL1_SPEC, CPHA_A, 2>;
impl<'a> CPHA_W<'a> {
    #[doc = "The 1st bit is fixed already, and SCK 1st edge is to receive/transmit data"]
    #[inline(always)]
    pub fn cpha0(self) -> &'a mut W {
        self.variant(CPHA_A::CPHA0)
    }
    #[doc = "SCK 1st edge is for data transition, and receive/transmit data at 2nd edge"]
    #[inline(always)]
    pub fn cpha1(self) -> &'a mut W {
        self.variant(CPHA_A::CPHA1)
    }
}
impl R {
    #[doc = "Bit 0 - MSB/LSB seletion"]
    #[inline(always)]
    pub fn mlsb(&self) -> MLSB_R {
        MLSB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock priority selection"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock phase of edge sampling"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSB/LSB seletion"]
    #[inline(always)]
    pub fn mlsb(&mut self) -> MLSB_W {
        MLSB_W::new(self)
    }
    #[doc = "Bit 1 - Clock priority selection"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W::new(self)
    }
    #[doc = "Bit 2 - Clock phase of edge sampling"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x04 SPIn Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
