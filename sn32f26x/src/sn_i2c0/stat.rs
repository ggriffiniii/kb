#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RX done status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DN_A {
    #[doc = "0: No RX with ACK/NACK transfer"]
    NOTDONE = 0,
    #[doc = "1: 8-bit RX with ACK/NACK transfer"]
    DONE = 1,
}
impl From<RX_DN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DN` reader - RX done status"]
pub type RX_DN_R = crate::BitReader<RX_DN_A>;
impl RX_DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DN_A {
        match self.bits {
            false => RX_DN_A::NOTDONE,
            true => RX_DN_A::DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_notdone(&self) -> bool {
        *self == RX_DN_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == RX_DN_A::DONE
    }
}
#[doc = "ACK done status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_STAT_A {
    #[doc = "0: No ACK received"]
    NO = 0,
    #[doc = "1: Receive an ACK"]
    DONE = 1,
}
impl From<ACK_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK_STAT` reader - ACK done status"]
pub type ACK_STAT_R = crate::BitReader<ACK_STAT_A>;
impl ACK_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_STAT_A {
        match self.bits {
            false => ACK_STAT_A::NO,
            true => ACK_STAT_A::DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == ACK_STAT_A::NO
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == ACK_STAT_A::DONE
    }
}
#[doc = "NACK done status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_STAT_A {
    #[doc = "0: No NACK received"]
    NO = 0,
    #[doc = "1: Receive a NACK"]
    DONE = 1,
}
impl From<NACK_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK_STAT` reader - NACK done status"]
pub type NACK_STAT_R = crate::BitReader<NACK_STAT_A>;
impl NACK_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_STAT_A {
        match self.bits {
            false => NACK_STAT_A::NO,
            true => NACK_STAT_A::DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == NACK_STAT_A::NO
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == NACK_STAT_A::DONE
    }
}
#[doc = "STOP done status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_DN_A {
    #[doc = "0: No STOP condition"]
    NO = 0,
    #[doc = "1: Transmit or receive a STOP condition"]
    DONE = 1,
}
impl From<STOP_DN_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_DN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_DN` reader - STOP done status"]
pub type STOP_DN_R = crate::BitReader<STOP_DN_A>;
impl STOP_DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_DN_A {
        match self.bits {
            false => STOP_DN_A::NO,
            true => STOP_DN_A::DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == STOP_DN_A::NO
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STOP_DN_A::DONE
    }
}
#[doc = "START done status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_DN_A {
    #[doc = "0: No START condition"]
    NO = 0,
    #[doc = "1: Transmit or receive a START condition"]
    ASSERT = 1,
}
impl From<START_DN_A> for bool {
    #[inline(always)]
    fn from(variant: START_DN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START_DN` reader - START done status"]
pub type START_DN_R = crate::BitReader<START_DN_A>;
impl START_DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_DN_A {
        match self.bits {
            false => START_DN_A::NO,
            true => START_DN_A::ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == START_DN_A::NO
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == START_DN_A::ASSERT
    }
}
#[doc = "I2C master/slave status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST_A {
    #[doc = "0: Act as Slave"]
    SLAVE = 0,
    #[doc = "1: Act as Master"]
    MASTER = 1,
}
impl From<MST_A> for bool {
    #[inline(always)]
    fn from(variant: MST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST` reader - I2C master/slave status"]
pub type MST_R = crate::BitReader<MST_A>;
impl MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MST_A {
        match self.bits {
            false => MST_A::SLAVE,
            true => MST_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MST_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MST_A::MASTER
    }
}
#[doc = "Slave RX address hit flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLV_RX_HIT_A {
    #[doc = "0: No matched slave address"]
    _0 = 0,
    #[doc = "1: Slave address hit, and is called for RX"]
    _1 = 1,
}
impl From<SLV_RX_HIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLV_RX_HIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLV_RX_HIT` reader - Slave RX address hit flag"]
pub type SLV_RX_HIT_R = crate::BitReader<SLV_RX_HIT_A>;
impl SLV_RX_HIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLV_RX_HIT_A {
        match self.bits {
            false => SLV_RX_HIT_A::_0,
            true => SLV_RX_HIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLV_RX_HIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLV_RX_HIT_A::_1
    }
}
#[doc = "Slave TX address hit flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLV_TX_HIT_A {
    #[doc = "0: No matched slave address"]
    _0 = 0,
    #[doc = "1: Slave address hit, and is called for TX"]
    _1 = 1,
}
impl From<SLV_TX_HIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLV_TX_HIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLV_TX_HIT` reader - Slave TX address hit flag"]
pub type SLV_TX_HIT_R = crate::BitReader<SLV_TX_HIT_A>;
impl SLV_TX_HIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLV_TX_HIT_A {
        match self.bits {
            false => SLV_TX_HIT_A::_0,
            true => SLV_TX_HIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLV_TX_HIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLV_TX_HIT_A::_1
    }
}
#[doc = "Lost arbitration status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOST_ARB_A {
    #[doc = "0: Not lost arbitration"]
    _0 = 0,
    #[doc = "1: Lost arbitration"]
    _1 = 1,
}
impl From<LOST_ARB_A> for bool {
    #[inline(always)]
    fn from(variant: LOST_ARB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOST_ARB` reader - Lost arbitration status"]
pub type LOST_ARB_R = crate::BitReader<LOST_ARB_A>;
impl LOST_ARB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOST_ARB_A {
        match self.bits {
            false => LOST_ARB_A::_0,
            true => LOST_ARB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOST_ARB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOST_ARB_A::_1
    }
}
#[doc = "Time-out status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: No timeout"]
    _0 = 0,
    #[doc = "1: Timeout"]
    _1 = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - Time-out status"]
pub type TIMEOUT_R = crate::BitReader<TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::_0,
            true => TIMEOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIMEOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIMEOUT_A::_1
    }
}
#[doc = "I2C interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CIF_A {
    #[doc = "0: I2C status doesn't change"]
    _0 = 0,
    #[doc = "1: I2C status changes"]
    _1 = 1,
}
impl From<I2CIF_A> for bool {
    #[inline(always)]
    fn from(variant: I2CIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CIF` reader - I2C interrupt flag"]
pub type I2CIF_R = crate::BitReader<I2CIF_A>;
impl I2CIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CIF_A {
        match self.bits {
            false => I2CIF_A::_0,
            true => I2CIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2CIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2CIF_A::_1
    }
}
#[doc = "Field `I2CIF` writer - I2C interrupt flag"]
pub type I2CIF_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, I2CIF_A, 15>;
impl<'a> I2CIF_W<'a> {
    #[doc = "I2C status doesn't change"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2CIF_A::_0)
    }
    #[doc = "I2C status changes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2CIF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RX done status"]
    #[inline(always)]
    pub fn rx_dn(&self) -> RX_DN_R {
        RX_DN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACK done status"]
    #[inline(always)]
    pub fn ack_stat(&self) -> ACK_STAT_R {
        ACK_STAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NACK done status"]
    #[inline(always)]
    pub fn nack_stat(&self) -> NACK_STAT_R {
        NACK_STAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP done status"]
    #[inline(always)]
    pub fn stop_dn(&self) -> STOP_DN_R {
        STOP_DN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - START done status"]
    #[inline(always)]
    pub fn start_dn(&self) -> START_DN_R {
        START_DN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C master/slave status"]
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave RX address hit flag"]
    #[inline(always)]
    pub fn slv_rx_hit(&self) -> SLV_RX_HIT_R {
        SLV_RX_HIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave TX address hit flag"]
    #[inline(always)]
    pub fn slv_tx_hit(&self) -> SLV_TX_HIT_R {
        SLV_TX_HIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Lost arbitration status"]
    #[inline(always)]
    pub fn lost_arb(&self) -> LOST_ARB_R {
        LOST_ARB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Time-out status"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C interrupt flag"]
    #[inline(always)]
    pub fn i2cif(&self) -> I2CIF_R {
        I2CIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - I2C interrupt flag"]
    #[inline(always)]
    pub fn i2cif(&mut self) -> I2CIF_W {
        I2CIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x04 I2Cn Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
