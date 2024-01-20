#[doc = "Register `LP` reader"]
pub type R = crate::R<LP_SPEC>;
#[doc = "Register `LP` writer"]
pub type W = crate::W<LP_SPEC>;
#[doc = "Field `MODE` reader - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESET_FLAG` reader - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened"]
pub type RESET_FLAG_R = crate::BitReader;
#[doc = "Field `RESET_FLAG` writer - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened"]
pub type RESET_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_FLAG` reader - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
pub type SLEEP_FLAG_R = crate::BitReader;
#[doc = "Field `SLEEP_FLAG` writer - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
pub type SLEEP_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE_FLAG` reader - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened"]
pub type WAKE_FLAG_R = crate::BitReader;
#[doc = "Field `WAKE_FLAG` writer - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened"]
pub type WAKE_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXEC` reader - CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing"]
pub type EXEC_R = crate::BitReader;
#[doc = "Field `WAKE` reader - CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted"]
pub type WAKE_R = crate::BitReader;
#[doc = "Field `HALT` reader - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
pub type HALT_R = crate::BitReader;
#[doc = "Field `HALT` writer - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
pub type HALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE_CNT` reader - CPU0 wake up counter, counter satuated at 255, write 0x00 to clear"]
pub type WAKE_CNT_R = crate::FieldReader;
#[doc = "Field `WAKE_CNT` writer - CPU0 wake up counter, counter satuated at 255, write 0x00 to clear"]
pub type WAKE_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened"]
    #[inline(always)]
    pub fn reset_flag(&self) -> RESET_FLAG_R {
        RESET_FLAG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    #[inline(always)]
    pub fn sleep_flag(&self) -> SLEEP_FLAG_R {
        SLEEP_FLAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened"]
    #[inline(always)]
    pub fn wake_flag(&self) -> WAKE_FLAG_R {
        WAKE_FLAG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing"]
    #[inline(always)]
    pub fn exec(&self) -> EXEC_R {
        EXEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - CPU0 wake up counter, counter satuated at 255, write 0x00 to clear"]
    #[inline(always)]
    pub fn wake_cnt(&self) -> WAKE_CNT_R {
        WAKE_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<LP_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 8 - CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened"]
    #[inline(always)]
    #[must_use]
    pub fn reset_flag(&mut self) -> RESET_FLAG_W<LP_SPEC> {
        RESET_FLAG_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_flag(&mut self) -> SLEEP_FLAG_W<LP_SPEC> {
        SLEEP_FLAG_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened"]
    #[inline(always)]
    #[must_use]
    pub fn wake_flag(&mut self) -> WAKE_FLAG_W<LP_SPEC> {
        WAKE_FLAG_W::new(self, 10)
    }
    #[doc = "Bit 16 - halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<LP_SPEC> {
        HALT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CPU0 wake up counter, counter satuated at 255, write 0x00 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn wake_cnt(&mut self) -> WAKE_CNT_W<LP_SPEC> {
        WAKE_CNT_W::new(self, 24)
    }
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
#[doc = "CPU0 LP control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_SPEC;
impl crate::RegisterSpec for LP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp::R`](R) reader structure"]
impl crate::Readable for LP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp::W`](W) writer structure"]
impl crate::Writable for LP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP to value 0x1000"]
impl crate::Resettable for LP_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
