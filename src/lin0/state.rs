#[doc = "Register `state` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Register `state` writer"]
pub type W = crate::W<STATE_SPEC>;
#[doc = "Field `COMPLETE` reader - set after a transmission has been successful finished and it will reset at the start of a transmission."]
pub type COMPLETE_R = crate::BitReader;
#[doc = "Field `WAKEUP` reader - set when transmitting a wakeup signal or when received a wakeup signal. Clear when reset_error bit is 1"]
pub type WAKEUP_R = crate::BitReader;
#[doc = "Field `ERROR` reader - set when detecte an error, clear by reset_error"]
pub type ERROR_R = crate::BitReader;
#[doc = "Field `INT` reader - set when request an interrupt. Reset by reset_int"]
pub type INT_R = crate::BitReader;
#[doc = "Field `DATA_REQ` reader - slave only. Sets after receiving the identifier and requests an interrupt to the host controller."]
pub type DATA_REQ_R = crate::BitReader;
#[doc = "Field `ABORTED` reader - slave only. This bit is set by LIN core slave if a transmission is aborted after the bneginning of the data field due to a timeout or bit error."]
pub type ABORTED_R = crate::BitReader;
#[doc = "Field `BUS_IDLE_TV` reader - slave only. This bit is set by LIN core if bit sleep is not set and no bus activity is detected for 4s"]
pub type BUS_IDLE_TV_R = crate::BitReader;
#[doc = "Field `LIN_ACTIVE` reader - The bit indicates whether the LIN bus is active or not"]
pub type LIN_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - set after a transmission has been successful finished and it will reset at the start of a transmission."]
    #[inline(always)]
    pub fn complete(&self) -> COMPLETE_R {
        COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set when transmitting a wakeup signal or when received a wakeup signal. Clear when reset_error bit is 1"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - set when detecte an error, clear by reset_error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - set when request an interrupt. Reset by reset_int"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - slave only. Sets after receiving the identifier and requests an interrupt to the host controller."]
    #[inline(always)]
    pub fn data_req(&self) -> DATA_REQ_R {
        DATA_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - slave only. This bit is set by LIN core slave if a transmission is aborted after the bneginning of the data field due to a timeout or bit error."]
    #[inline(always)]
    pub fn aborted(&self) -> ABORTED_R {
        ABORTED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - slave only. This bit is set by LIN core if bit sleep is not set and no bus activity is detected for 4s"]
    #[inline(always)]
    pub fn bus_idle_tv(&self) -> BUS_IDLE_TV_R {
        BUS_IDLE_TV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The bit indicates whether the LIN bus is active or not"]
    #[inline(always)]
    pub fn lin_active(&self) -> LIN_ACTIVE_R {
        LIN_ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`state::W`](W) writer structure"]
impl crate::Writable for STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets state to value 0"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
