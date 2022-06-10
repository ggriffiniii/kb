#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "NACK assert flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    #[doc = "0: No action"]
    NOACTION = 0,
    #[doc = "1: Assert NACK during the acknowledge clock pulse on SCLn"]
    ASSERT = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - NACK assert flag"]
pub type NACK_R = crate::BitReader<NACK_A>;
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::NOACTION,
            true => NACK_A::ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_noaction(&self) -> bool {
        *self == NACK_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == NACK_A::ASSERT
    }
}
#[doc = "Field `NACK` writer - NACK assert flag"]
pub type NACK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, NACK_A, 1>;
impl<'a> NACK_W<'a> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn noaction(self) -> &'a mut W {
        self.variant(NACK_A::NOACTION)
    }
    #[doc = "Assert NACK during the acknowledge clock pulse on SCLn"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(NACK_A::ASSERT)
    }
}
#[doc = "ACK assert flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_A {
    #[doc = "0: Master: No action/Slave: Assert NACK after receiving"]
    NO = 0,
    #[doc = "1: Assert ACK during the acknowledge clock pulse on SCLn"]
    ASSERT = 1,
}
impl From<ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK` reader - ACK assert flag"]
pub type ACK_R = crate::BitReader<ACK_A>;
impl ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            false => ACK_A::NO,
            true => ACK_A::ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACK_A::NO
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == ACK_A::ASSERT
    }
}
#[doc = "Field `ACK` writer - ACK assert flag"]
pub type ACK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, ACK_A, 2>;
impl<'a> ACK_W<'a> {
    #[doc = "Master: No action/Slave: Assert NACK after receiving"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(ACK_A::NO)
    }
    #[doc = "Assert ACK during the acknowledge clock pulse on SCLn"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(ACK_A::ASSERT)
    }
}
#[doc = "STOP assert flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STO_A {
    #[doc = "0: STOP condition idle"]
    IDLE = 0,
    #[doc = "1: Transmit a STOP condition in master mode, or recover from an error condition in slave mode"]
    ASSERT = 1,
}
impl From<STO_A> for bool {
    #[inline(always)]
    fn from(variant: STO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STO` reader - STOP assert flag"]
pub type STO_R = crate::BitReader<STO_A>;
impl STO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STO_A {
        match self.bits {
            false => STO_A::IDLE,
            true => STO_A::ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STO_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == STO_A::ASSERT
    }
}
#[doc = "Field `STO` writer - STOP assert flag"]
pub type STO_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, STO_A, 4>;
impl<'a> STO_W<'a> {
    #[doc = "STOP condition idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(STO_A::IDLE)
    }
    #[doc = "Transmit a STOP condition in master mode, or recover from an error condition in slave mode"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(STO_A::ASSERT)
    }
}
#[doc = "START assert flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STA_A {
    #[doc = "0: No START condition or Repeated START condition will be generated"]
    NOACTION = 0,
    #[doc = "1: Enter master mode and transmit a START or Repeated START condition"]
    ASSERT = 1,
}
impl From<STA_A> for bool {
    #[inline(always)]
    fn from(variant: STA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STA` reader - START assert flag"]
pub type STA_R = crate::BitReader<STA_A>;
impl STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STA_A {
        match self.bits {
            false => STA_A::NOACTION,
            true => STA_A::ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOACTION`"]
    #[inline(always)]
    pub fn is_noaction(&self) -> bool {
        *self == STA_A::NOACTION
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == STA_A::ASSERT
    }
}
#[doc = "Field `STA` writer - START assert flag"]
pub type STA_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, STA_A, 5>;
impl<'a> STA_W<'a> {
    #[doc = "No START condition or Repeated START condition will be generated"]
    #[inline(always)]
    pub fn noaction(self) -> &'a mut W {
        self.variant(STA_A::NOACTION)
    }
    #[doc = "Enter master mode and transmit a START or Repeated START condition"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(STA_A::ASSERT)
    }
}
#[doc = "I2C mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Standard/Fast mode"]
    STANDARDFASTMODE = 0,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - I2C mode"]
pub type MODE_R = crate::BitReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            false => Some(MODE_A::STANDARDFASTMODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARDFASTMODE`"]
    #[inline(always)]
    pub fn is_standard_fastmode(&self) -> bool {
        *self == MODE_A::STANDARDFASTMODE
    }
}
#[doc = "Field `MODE` writer - I2C mode"]
pub type MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, MODE_A, 7>;
impl<'a> MODE_W<'a> {
    #[doc = "Standard/Fast mode"]
    #[inline(always)]
    pub fn standard_fastmode(self) -> &'a mut W {
        self.variant(MODE_A::STANDARDFASTMODE)
    }
}
#[doc = "I2Cn interface enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CEN_A {
    #[doc = "0: Disable I2C"]
    DISABLE = 0,
    #[doc = "1: Enable I2C"]
    ENABLE = 1,
}
impl From<I2CEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CEN` reader - I2Cn interface enable"]
pub type I2CEN_R = crate::BitReader<I2CEN_A>;
impl I2CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CEN_A {
        match self.bits {
            false => I2CEN_A::DISABLE,
            true => I2CEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2CEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2CEN_A::ENABLE
    }
}
#[doc = "Field `I2CEN` writer - I2Cn interface enable"]
pub type I2CEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, I2CEN_A, 8>;
impl<'a> I2CEN_W<'a> {
    #[doc = "Disable I2C"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2CEN_A::DISABLE)
    }
    #[doc = "Enable I2C"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2CEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - NACK assert flag"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACK assert flag"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - STOP assert flag"]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - START assert flag"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2Cn interface enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - NACK assert flag"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W::new(self)
    }
    #[doc = "Bit 2 - ACK assert flag"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W::new(self)
    }
    #[doc = "Bit 4 - STOP assert flag"]
    #[inline(always)]
    pub fn sto(&mut self) -> STO_W {
        STO_W::new(self)
    }
    #[doc = "Bit 5 - START assert flag"]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W {
        STA_W::new(self)
    }
    #[doc = "Bit 7 - I2C mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 8 - I2Cn interface enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W {
        I2CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 I2Cn Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
