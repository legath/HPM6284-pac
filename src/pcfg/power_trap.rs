#[doc = "Register `POWER_TRAP` reader"]
pub type R = crate::R<POWER_TRAP_SPEC>;
#[doc = "Register `POWER_TRAP` writer"]
pub type W = crate::W<POWER_TRAP_SPEC>;
#[doc = "Field `TRAP` reader - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
pub type TRAP_R = crate::BitReader;
#[doc = "Field `TRAP` writer - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
pub type TRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETENTION` reader - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
pub type RETENTION_R = crate::BitReader;
#[doc = "Field `RETENTION` writer - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
pub type RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGGERED` reader - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
pub type TRIGGERED_R = crate::BitReader;
#[doc = "Field `TRIGGERED` writer - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
pub type TRIGGERED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
    #[inline(always)]
    pub fn trap(&self) -> TRAP_R {
        TRAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
    #[inline(always)]
    pub fn retention(&self) -> RETENTION_R {
        RETENTION_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
    #[inline(always)]
    pub fn triggered(&self) -> TRIGGERED_R {
        TRIGGERED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
    #[inline(always)]
    #[must_use]
    pub fn trap(&mut self) -> TRAP_W<POWER_TRAP_SPEC> {
        TRAP_W::new(self, 0)
    }
    #[doc = "Bit 16 - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
    #[inline(always)]
    #[must_use]
    pub fn retention(&mut self) -> RETENTION_W<POWER_TRAP_SPEC> {
        RETENTION_W::new(self, 16)
    }
    #[doc = "Bit 31 - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
    #[inline(always)]
    #[must_use]
    pub fn triggered(&mut self) -> TRIGGERED_W<POWER_TRAP_SPEC> {
        TRIGGERED_W::new(self, 31)
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
#[doc = "SOC power trap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_trap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_trap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_TRAP_SPEC;
impl crate::RegisterSpec for POWER_TRAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_trap::R`](R) reader structure"]
impl crate::Readable for POWER_TRAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_trap::W`](W) writer structure"]
impl crate::Writable for POWER_TRAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_TRAP to value 0"]
impl crate::Resettable for POWER_TRAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
