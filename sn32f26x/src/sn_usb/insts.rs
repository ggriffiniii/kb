#[doc = "Register `INSTS` reader"]
pub struct R(crate::R<INSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Endpoint 1 NAK transaction flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP1_NAK_A {
    #[doc = "0: No EP1 NAK transacation"]
    _0 = 0,
    #[doc = "1: EP1 NAK transaction completes"]
    _1 = 1,
}
impl From<EP1_NAK_A> for bool {
    #[inline(always)]
    fn from(variant: EP1_NAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1_NAK` reader - Endpoint 1 NAK transaction flag"]
pub type EP1_NAK_R = crate::BitReader<EP1_NAK_A>;
impl EP1_NAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP1_NAK_A {
        match self.bits {
            false => EP1_NAK_A::_0,
            true => EP1_NAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP1_NAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP1_NAK_A::_1
    }
}
#[doc = "Endpoint 2 NAK transaction flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP2_NAK_A {
    #[doc = "0: No EP2 NAK transacation"]
    _0 = 0,
    #[doc = "1: EP2 NAK transaction completes"]
    _1 = 1,
}
impl From<EP2_NAK_A> for bool {
    #[inline(always)]
    fn from(variant: EP2_NAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2_NAK` reader - Endpoint 2 NAK transaction flag"]
pub type EP2_NAK_R = crate::BitReader<EP2_NAK_A>;
impl EP2_NAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP2_NAK_A {
        match self.bits {
            false => EP2_NAK_A::_0,
            true => EP2_NAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP2_NAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP2_NAK_A::_1
    }
}
#[doc = "Endpoint 3 NAK transaction flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP3_NAK_A {
    #[doc = "0: No EP3 NAK transacation"]
    _0 = 0,
    #[doc = "1: EP3 NAK transaction completes"]
    _1 = 1,
}
impl From<EP3_NAK_A> for bool {
    #[inline(always)]
    fn from(variant: EP3_NAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3_NAK` reader - Endpoint 3 NAK transaction flag"]
pub type EP3_NAK_R = crate::BitReader<EP3_NAK_A>;
impl EP3_NAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP3_NAK_A {
        match self.bits {
            false => EP3_NAK_A::_0,
            true => EP3_NAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP3_NAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP3_NAK_A::_1
    }
}
#[doc = "Endpoint 4 NAK transaction flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP4_NAK_A {
    #[doc = "0: No EP4 NAK transacation"]
    _0 = 0,
    #[doc = "1: EP4 NAK transaction completes"]
    _1 = 1,
}
impl From<EP4_NAK_A> for bool {
    #[inline(always)]
    fn from(variant: EP4_NAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4_NAK` reader - Endpoint 4 NAK transaction flag"]
pub type EP4_NAK_R = crate::BitReader<EP4_NAK_A>;
impl EP4_NAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP4_NAK_A {
        match self.bits {
            false => EP4_NAK_A::_0,
            true => EP4_NAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP4_NAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP4_NAK_A::_1
    }
}
#[doc = "Endpoint 1 ACK transaction flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP1_ACK_A {
    #[doc = "0: No EP1 ACK transacation"]
    _0 = 0,
    #[doc = "1: EP1 ACK transaction completes"]
    _1 = 1,
}
impl From<EP1_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: EP1_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1_ACK` reader - Endpoint 1 ACK transaction flag"]
pub type EP1_ACK_R = crate::BitReader<EP1_ACK_A>;
impl EP1_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP1_ACK_A {
        match self.bits {
            false => EP1_ACK_A::_0,
            true => EP1_ACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP1_ACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP1_ACK_A::_1
    }
}
#[doc = "Endpoint 2 ACK transaction flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP2_ACK_A {
    #[doc = "0: No EP2 ACK transacation"]
    _0 = 0,
    #[doc = "1: EP2 ACK transaction completes"]
    _1 = 1,
}
impl From<EP2_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: EP2_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2_ACK` reader - Endpoint 2 ACK transaction flag"]
pub type EP2_ACK_R = crate::BitReader<EP2_ACK_A>;
impl EP2_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP2_ACK_A {
        match self.bits {
            false => EP2_ACK_A::_0,
            true => EP2_ACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP2_ACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP2_ACK_A::_1
    }
}
#[doc = "Endpoint 3 ACK transaction flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP3_ACK_A {
    #[doc = "0: No EP3 ACK transacation"]
    _0 = 0,
    #[doc = "1: EP3 ACK transaction completes"]
    _1 = 1,
}
impl From<EP3_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: EP3_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3_ACK` reader - Endpoint 3 ACK transaction flag"]
pub type EP3_ACK_R = crate::BitReader<EP3_ACK_A>;
impl EP3_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP3_ACK_A {
        match self.bits {
            false => EP3_ACK_A::_0,
            true => EP3_ACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP3_ACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP3_ACK_A::_1
    }
}
#[doc = "Endpoint 4 ACK transaction flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP4_ACK_A {
    #[doc = "0: No EP4 ACK transacation"]
    _0 = 0,
    #[doc = "1: EP4 ACK transaction completes"]
    _1 = 1,
}
impl From<EP4_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: EP4_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4_ACK` reader - Endpoint 4 ACK transaction flag"]
pub type EP4_ACK_R = crate::BitReader<EP4_ACK_A>;
impl EP4_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP4_ACK_A {
        match self.bits {
            false => EP4_ACK_A::_0,
            true => EP4_ACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP4_ACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP4_ACK_A::_1
    }
}
#[doc = "Timeout Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_TIMEOUT_A {
    #[doc = "0: No time out"]
    _0 = 0,
    #[doc = "1: Bus no any response more than 18 bits time"]
    _1 = 1,
}
impl From<ERR_TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_TIMEOUT` reader - Timeout Status"]
pub type ERR_TIMEOUT_R = crate::BitReader<ERR_TIMEOUT_A>;
impl ERR_TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_TIMEOUT_A {
        match self.bits {
            false => ERR_TIMEOUT_A::_0,
            true => ERR_TIMEOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR_TIMEOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR_TIMEOUT_A::_1
    }
}
#[doc = "Wrong Setup data received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_SETUP_A {
    #[doc = "0: Normal 8-byte Setup DATA0 is received"]
    _0 = 0,
    #[doc = "1: Setup data is not 8-byte or is not DATA0"]
    _1 = 1,
}
impl From<ERR_SETUP_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_SETUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_SETUP` reader - Wrong Setup data received"]
pub type ERR_SETUP_R = crate::BitReader<ERR_SETUP_A>;
impl ERR_SETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_SETUP_A {
        match self.bits {
            false => ERR_SETUP_A::_0,
            true => ERR_SETUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR_SETUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR_SETUP_A::_1
    }
}
#[doc = "EP0 OUT STALL transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_OUT_STALL_A {
    #[doc = "0: No EP0 OUT STALL transaction"]
    _0 = 0,
    #[doc = "1: EP0 OUT STALL transaction is completed"]
    _1 = 1,
}
impl From<EP0_OUT_STALL_A> for bool {
    #[inline(always)]
    fn from(variant: EP0_OUT_STALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_OUT_STALL` reader - EP0 OUT STALL transaction"]
pub type EP0_OUT_STALL_R = crate::BitReader<EP0_OUT_STALL_A>;
impl EP0_OUT_STALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0_OUT_STALL_A {
        match self.bits {
            false => EP0_OUT_STALL_A::_0,
            true => EP0_OUT_STALL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP0_OUT_STALL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP0_OUT_STALL_A::_1
    }
}
#[doc = "EP0 IN STALL Transaction is completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_IN_STALL_A {
    #[doc = "0: No EP0 IN STALL transaction"]
    _0 = 0,
    #[doc = "1: EP0 IN STALL transaction is completed"]
    _1 = 1,
}
impl From<EP0_IN_STALL_A> for bool {
    #[inline(always)]
    fn from(variant: EP0_IN_STALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_IN_STALL` reader - EP0 IN STALL Transaction is completed"]
pub type EP0_IN_STALL_R = crate::BitReader<EP0_IN_STALL_A>;
impl EP0_IN_STALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0_IN_STALL_A {
        match self.bits {
            false => EP0_IN_STALL_A::_0,
            true => EP0_IN_STALL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP0_IN_STALL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP0_IN_STALL_A::_1
    }
}
#[doc = "EP0 OUT ACK Transaction Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_OUT_A {
    #[doc = "0: No EP0 OUT ACK transaction"]
    _0 = 0,
    #[doc = "1: EP0 OUT ACK transaction is completed"]
    _1 = 1,
}
impl From<EP0_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: EP0_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_OUT` reader - EP0 OUT ACK Transaction Flag"]
pub type EP0_OUT_R = crate::BitReader<EP0_OUT_A>;
impl EP0_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0_OUT_A {
        match self.bits {
            false => EP0_OUT_A::_0,
            true => EP0_OUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP0_OUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP0_OUT_A::_1
    }
}
#[doc = "EP0 IN ACK Transaction Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_IN_A {
    #[doc = "0: No EP0 IN Transaction"]
    _0 = 0,
    #[doc = "1: EP0 IN Transaction is completed"]
    _1 = 1,
}
impl From<EP0_IN_A> for bool {
    #[inline(always)]
    fn from(variant: EP0_IN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_IN` reader - EP0 IN ACK Transaction Flag"]
pub type EP0_IN_R = crate::BitReader<EP0_IN_A>;
impl EP0_IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0_IN_A {
        match self.bits {
            false => EP0_IN_A::_0,
            true => EP0_IN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP0_IN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP0_IN_A::_1
    }
}
#[doc = "EP0 Setup Transaction Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_SETUP_A {
    #[doc = "0: No EP0 Setup transaction"]
    _0 = 0,
    #[doc = "1: EP0 Setup transaction is completed"]
    _1 = 1,
}
impl From<EP0_SETUP_A> for bool {
    #[inline(always)]
    fn from(variant: EP0_SETUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_SETUP` reader - EP0 Setup Transaction Flag"]
pub type EP0_SETUP_R = crate::BitReader<EP0_SETUP_A>;
impl EP0_SETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0_SETUP_A {
        match self.bits {
            false => EP0_SETUP_A::_0,
            true => EP0_SETUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP0_SETUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP0_SETUP_A::_1
    }
}
#[doc = "EP0 Setup Token Packet Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_PRESETUP_A {
    #[doc = "0: No EP0 Setup token packet"]
    _0 = 0,
    #[doc = "1: EP0 Setup token packet is received"]
    _1 = 1,
}
impl From<EP0_PRESETUP_A> for bool {
    #[inline(always)]
    fn from(variant: EP0_PRESETUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_PRESETUP` reader - EP0 Setup Token Packet Flag"]
pub type EP0_PRESETUP_R = crate::BitReader<EP0_PRESETUP_A>;
impl EP0_PRESETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0_PRESETUP_A {
        match self.bits {
            false => EP0_PRESETUP_A::_0,
            true => EP0_PRESETUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EP0_PRESETUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EP0_PRESETUP_A::_1
    }
}
#[doc = "USB SOF packet received flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_SOF_A {
    #[doc = "0: No USB SOF packet"]
    _0 = 0,
    #[doc = "1: USB SOF packet is received"]
    _1 = 1,
}
impl From<USB_SOF_A> for bool {
    #[inline(always)]
    fn from(variant: USB_SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_SOF` reader - USB SOF packet received flag"]
pub type USB_SOF_R = crate::BitReader<USB_SOF_A>;
impl USB_SOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_SOF_A {
        match self.bits {
            false => USB_SOF_A::_0,
            true => USB_SOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USB_SOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USB_SOF_A::_1
    }
}
#[doc = "USB Bus Resume signal flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_RESUME_A {
    #[doc = "0: No bus resume signal is detected"]
    _0 = 0,
    #[doc = "1: Bus resume signal from suspend mode is detected"]
    _1 = 1,
}
impl From<BUS_RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_RESUME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_RESUME` reader - USB Bus Resume signal flag"]
pub type BUS_RESUME_R = crate::BitReader<BUS_RESUME_A>;
impl BUS_RESUME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_RESUME_A {
        match self.bits {
            false => BUS_RESUME_A::_0,
            true => BUS_RESUME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUS_RESUME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUS_RESUME_A::_1
    }
}
#[doc = "USB Bus Suspend signal flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_SUSPEND_A {
    #[doc = "0: No bus suspend is detected"]
    _0 = 0,
    #[doc = "1: Bus suspend is detected"]
    _1 = 1,
}
impl From<BUS_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_SUSPEND` reader - USB Bus Suspend signal flag"]
pub type BUS_SUSPEND_R = crate::BitReader<BUS_SUSPEND_A>;
impl BUS_SUSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_SUSPEND_A {
        match self.bits {
            false => BUS_SUSPEND_A::_0,
            true => BUS_SUSPEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUS_SUSPEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUS_SUSPEND_A::_1
    }
}
#[doc = "USB Bus Reset signal flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_RESET_A {
    #[doc = "0: No bus reset signal is detected"]
    _0 = 0,
    #[doc = "1: Bus reset signal is detected"]
    _1 = 1,
}
impl From<BUS_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_RESET` reader - USB Bus Reset signal flag"]
pub type BUS_RESET_R = crate::BitReader<BUS_RESET_A>;
impl BUS_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_RESET_A {
        match self.bits {
            false => BUS_RESET_A::_0,
            true => BUS_RESET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUS_RESET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUS_RESET_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint 1 NAK transaction flag"]
    #[inline(always)]
    pub fn ep1_nak(&self) -> EP1_NAK_R {
        EP1_NAK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 2 NAK transaction flag"]
    #[inline(always)]
    pub fn ep2_nak(&self) -> EP2_NAK_R {
        EP2_NAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 3 NAK transaction flag"]
    #[inline(always)]
    pub fn ep3_nak(&self) -> EP3_NAK_R {
        EP3_NAK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 4 NAK transaction flag"]
    #[inline(always)]
    pub fn ep4_nak(&self) -> EP4_NAK_R {
        EP4_NAK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 1 ACK transaction flag"]
    #[inline(always)]
    pub fn ep1_ack(&self) -> EP1_ACK_R {
        EP1_ACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 2 ACK transaction flag"]
    #[inline(always)]
    pub fn ep2_ack(&self) -> EP2_ACK_R {
        EP2_ACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint 3 ACK transaction flag"]
    #[inline(always)]
    pub fn ep3_ack(&self) -> EP3_ACK_R {
        EP3_ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint 4 ACK transaction flag"]
    #[inline(always)]
    pub fn ep4_ack(&self) -> EP4_ACK_R {
        EP4_ACK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Timeout Status"]
    #[inline(always)]
    pub fn err_timeout(&self) -> ERR_TIMEOUT_R {
        ERR_TIMEOUT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wrong Setup data received"]
    #[inline(always)]
    pub fn err_setup(&self) -> ERR_SETUP_R {
        ERR_SETUP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EP0 OUT STALL transaction"]
    #[inline(always)]
    pub fn ep0_out_stall(&self) -> EP0_OUT_STALL_R {
        EP0_OUT_STALL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EP0 IN STALL Transaction is completed"]
    #[inline(always)]
    pub fn ep0_in_stall(&self) -> EP0_IN_STALL_R {
        EP0_IN_STALL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EP0 OUT ACK Transaction Flag"]
    #[inline(always)]
    pub fn ep0_out(&self) -> EP0_OUT_R {
        EP0_OUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EP0 IN ACK Transaction Flag"]
    #[inline(always)]
    pub fn ep0_in(&self) -> EP0_IN_R {
        EP0_IN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EP0 Setup Transaction Flag"]
    #[inline(always)]
    pub fn ep0_setup(&self) -> EP0_SETUP_R {
        EP0_SETUP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EP0 Setup Token Packet Flag"]
    #[inline(always)]
    pub fn ep0_presetup(&self) -> EP0_PRESETUP_R {
        EP0_PRESETUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - USB SOF packet received flag"]
    #[inline(always)]
    pub fn usb_sof(&self) -> USB_SOF_R {
        USB_SOF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - USB Bus Resume signal flag"]
    #[inline(always)]
    pub fn bus_resume(&self) -> BUS_RESUME_R {
        BUS_RESUME_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB Bus Suspend signal flag"]
    #[inline(always)]
    pub fn bus_suspend(&self) -> BUS_SUSPEND_R {
        BUS_SUSPEND_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB Bus Reset signal flag"]
    #[inline(always)]
    pub fn bus_reset(&self) -> BUS_RESET_R {
        BUS_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Offset:0x04 USB Interrupt Event Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [insts](index.html) module"]
pub struct INSTS_SPEC;
impl crate::RegisterSpec for INSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [insts::R](R) reader structure"]
impl crate::Readable for INSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INSTS to value 0"]
impl crate::Resettable for INSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
