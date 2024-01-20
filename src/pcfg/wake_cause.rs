#[doc = "Register `WAKE_CAUSE` reader"]
pub type R = crate::R<WAKE_CAUSE_SPEC>;
#[doc = "Register `WAKE_CAUSE` writer"]
pub type W = crate::W<WAKE_CAUSE_SPEC>;
#[doc = "Field `CAUSE` reader - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
pub type CAUSE_R = crate::FieldReader<u32>;
#[doc = "Field `CAUSE` writer - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
pub type CAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
    #[inline(always)]
    pub fn cause(&self) -> CAUSE_R {
        CAUSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cause(&mut self) -> CAUSE_W<WAKE_CAUSE_SPEC> {
        CAUSE_W::new(self, 0)
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
#[doc = "Wake up source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wake_cause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_cause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKE_CAUSE_SPEC;
impl crate::RegisterSpec for WAKE_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wake_cause::R`](R) reader structure"]
impl crate::Readable for WAKE_CAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wake_cause::W`](W) writer structure"]
impl crate::Writable for WAKE_CAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKE_CAUSE to value 0"]
impl crate::Resettable for WAKE_CAUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
