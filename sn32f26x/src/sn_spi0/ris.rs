#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RX FIFO overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVFIF_A {
    #[doc = "0: No RXOVF interrupt"]
    NORXOVFINTERRUPT = 0,
    #[doc = "1: RXOVF interrupt is triggered when RXOVFIE=1"]
    METRXOVFINTERRUPTREQUIREMENTS = 1,
}
impl From<RXOVFIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVFIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOVFIF` reader - RX FIFO overflow interrupt flag"]
pub type RXOVFIF_R = crate::BitReader<RXOVFIF_A>;
impl RXOVFIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOVFIF_A {
        match self.bits {
            false => RXOVFIF_A::NORXOVFINTERRUPT,
            true => RXOVFIF_A::METRXOVFINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NORXOVFINTERRUPT`"]
    #[inline(always)]
    pub fn is_no_rxovfinterrupt(&self) -> bool {
        *self == RXOVFIF_A::NORXOVFINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METRXOVFINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_met_rxovfinterruptrequirements(&self) -> bool {
        *self == RXOVFIF_A::METRXOVFINTERRUPTREQUIREMENTS
    }
}
#[doc = "RX time-out interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOIF_A {
    #[doc = "0: No RXTO interrupt"]
    NORXTOINTERRUPT = 0,
    #[doc = "1: RXTO interrupt is triggered when RXTOIE=1"]
    METRXTOINTERRUPTREQUIREMENTS = 1,
}
impl From<RXTOIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTOIF` reader - RX time-out interrupt flag"]
pub type RXTOIF_R = crate::BitReader<RXTOIF_A>;
impl RXTOIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTOIF_A {
        match self.bits {
            false => RXTOIF_A::NORXTOINTERRUPT,
            true => RXTOIF_A::METRXTOINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NORXTOINTERRUPT`"]
    #[inline(always)]
    pub fn is_no_rxtointerrupt(&self) -> bool {
        *self == RXTOIF_A::NORXTOINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METRXTOINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_met_rxtointerruptrequirements(&self) -> bool {
        *self == RXTOIF_A::METRXTOINTERRUPTREQUIREMENTS
    }
}
#[doc = "RX FIFO threshold interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOTHIF_A {
    #[doc = "0: No RXFIFOTH interrupt"]
    NORXFIFOTHINTERRUPT = 0,
    #[doc = "1: RX FIFO threshold is triggered when RXFIFOTHIE=1"]
    METRXFIFOTHINTERRUPTREQUIREMENTS = 1,
}
impl From<RXFIFOTHIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFOTHIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFOTHIF` reader - RX FIFO threshold interrupt flag"]
pub type RXFIFOTHIF_R = crate::BitReader<RXFIFOTHIF_A>;
impl RXFIFOTHIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOTHIF_A {
        match self.bits {
            false => RXFIFOTHIF_A::NORXFIFOTHINTERRUPT,
            true => RXFIFOTHIF_A::METRXFIFOTHINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NORXFIFOTHINTERRUPT`"]
    #[inline(always)]
    pub fn is_no_rxfifothinterrupt(&self) -> bool {
        *self == RXFIFOTHIF_A::NORXFIFOTHINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METRXFIFOTHINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_met_rxfifothinterruptrequirements(&self) -> bool {
        *self == RXFIFOTHIF_A::METRXFIFOTHINTERRUPTREQUIREMENTS
    }
}
#[doc = "TX FIFO threshold interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOTHIF_A {
    #[doc = "0: No TXFIFOTH interrupt"]
    NOTXFIFOTHINTERRUPT = 0,
    #[doc = "1: TX FIFO threshold is triggered when TXFIFOTHIE=1"]
    METTXFIFOTHINTERRUPTREQUIREMENTS = 1,
}
impl From<TXFIFOTHIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIFOTHIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFOTHIF` reader - TX FIFO threshold interrupt flag"]
pub type TXFIFOTHIF_R = crate::BitReader<TXFIFOTHIF_A>;
impl TXFIFOTHIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOTHIF_A {
        match self.bits {
            false => TXFIFOTHIF_A::NOTXFIFOTHINTERRUPT,
            true => TXFIFOTHIF_A::METTXFIFOTHINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NOTXFIFOTHINTERRUPT`"]
    #[inline(always)]
    pub fn is_no_txfifothinterrupt(&self) -> bool {
        *self == TXFIFOTHIF_A::NOTXFIFOTHINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METTXFIFOTHINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_met_txfifothinterruptrequirements(&self) -> bool {
        *self == TXFIFOTHIF_A::METTXFIFOTHINTERRUPTREQUIREMENTS
    }
}
impl R {
    #[doc = "Bit 0 - RX FIFO overflow interrupt flag"]
    #[inline(always)]
    pub fn rxovfif(&self) -> RXOVFIF_R {
        RXOVFIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX time-out interrupt flag"]
    #[inline(always)]
    pub fn rxtoif(&self) -> RXTOIF_R {
        RXTOIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO threshold interrupt flag"]
    #[inline(always)]
    pub fn rxfifothif(&self) -> RXFIFOTHIF_R {
        RXFIFOTHIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO threshold interrupt flag"]
    #[inline(always)]
    pub fn txfifothif(&self) -> TXFIFOTHIF_R {
        TXFIFOTHIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Offset:0x14 SPIn Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
