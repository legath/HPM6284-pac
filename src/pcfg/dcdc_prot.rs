#[doc = "Register `DCDC_PROT` reader"]
pub type R = crate::R<DCDC_PROT_SPEC>;
#[doc = "Register `DCDC_PROT` writer"]
pub type W = crate::W<DCDC_PROT_SPEC>;
#[doc = "Field `SHORT_FLAG` reader - short circuit flag 0: current is within limit 1: short circuits detected"]
pub type SHORT_FLAG_R = crate::BitReader;
#[doc = "Field `SHORT_CURRENT` reader - short circuit current setting 0: 2.0A, 1: 1.3A"]
pub type SHORT_CURRENT_R = crate::BitReader;
#[doc = "Field `SHORT_CURRENT` writer - short circuit current setting 0: 2.0A, 1: 1.3A"]
pub type SHORT_CURRENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_SHORT` reader - disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled"]
pub type DISABLE_SHORT_R = crate::BitReader;
#[doc = "Field `DISABLE_SHORT` writer - disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled"]
pub type DISABLE_SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERVOLT_FLAG` reader - output over voltage flag 0: output is normal 1: output is unexpected high"]
pub type OVERVOLT_FLAG_R = crate::BitReader;
#[doc = "Field `DISABLE_OVERVOLTAGE` reader - ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage"]
pub type DISABLE_OVERVOLTAGE_R = crate::BitReader;
#[doc = "Field `DISABLE_OVERVOLTAGE` writer - ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage"]
pub type DISABLE_OVERVOLTAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWER_LOSS_FLAG` reader - power loss 0: input power is good 1: input power is too low"]
pub type POWER_LOSS_FLAG_R = crate::BitReader;
#[doc = "Field `DISABLE_POWER_LOSS` reader - disable power loss protection 0: power loss protection enabled, DCDC shuts down when power loss 1: power loss protection disabled, DCDC try working after power voltage drop"]
pub type DISABLE_POWER_LOSS_R = crate::BitReader;
#[doc = "Field `DISABLE_POWER_LOSS` writer - disable power loss protection 0: power loss protection enabled, DCDC shuts down when power loss 1: power loss protection disabled, DCDC try working after power voltage drop"]
pub type DISABLE_POWER_LOSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERLOAD_LP` reader - over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode"]
pub type OVERLOAD_LP_R = crate::BitReader;
#[doc = "Field `OVERLOAD_LP` writer - over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode"]
pub type OVERLOAD_LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIMIT_LP` reader - over current setting for low power mode 0:250mA 1:200mA"]
pub type ILIMIT_LP_R = crate::BitReader;
#[doc = "Field `ILIMIT_LP` writer - over current setting for low power mode 0:250mA 1:200mA"]
pub type ILIMIT_LP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - short circuit flag 0: current is within limit 1: short circuits detected"]
    #[inline(always)]
    pub fn short_flag(&self) -> SHORT_FLAG_R {
        SHORT_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - short circuit current setting 0: 2.0A, 1: 1.3A"]
    #[inline(always)]
    pub fn short_current(&self) -> SHORT_CURRENT_R {
        SHORT_CURRENT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled"]
    #[inline(always)]
    pub fn disable_short(&self) -> DISABLE_SHORT_R {
        DISABLE_SHORT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - output over voltage flag 0: output is normal 1: output is unexpected high"]
    #[inline(always)]
    pub fn overvolt_flag(&self) -> OVERVOLT_FLAG_R {
        OVERVOLT_FLAG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage"]
    #[inline(always)]
    pub fn disable_overvoltage(&self) -> DISABLE_OVERVOLTAGE_R {
        DISABLE_OVERVOLTAGE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - power loss 0: input power is good 1: input power is too low"]
    #[inline(always)]
    pub fn power_loss_flag(&self) -> POWER_LOSS_FLAG_R {
        POWER_LOSS_FLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - disable power loss protection 0: power loss protection enabled, DCDC shuts down when power loss 1: power loss protection disabled, DCDC try working after power voltage drop"]
    #[inline(always)]
    pub fn disable_power_loss(&self) -> DISABLE_POWER_LOSS_R {
        DISABLE_POWER_LOSS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode"]
    #[inline(always)]
    pub fn overload_lp(&self) -> OVERLOAD_LP_R {
        OVERLOAD_LP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - over current setting for low power mode 0:250mA 1:200mA"]
    #[inline(always)]
    pub fn ilimit_lp(&self) -> ILIMIT_LP_R {
        ILIMIT_LP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - short circuit current setting 0: 2.0A, 1: 1.3A"]
    #[inline(always)]
    #[must_use]
    pub fn short_current(&mut self) -> SHORT_CURRENT_W<DCDC_PROT_SPEC> {
        SHORT_CURRENT_W::new(self, 4)
    }
    #[doc = "Bit 7 - disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled"]
    #[inline(always)]
    #[must_use]
    pub fn disable_short(&mut self) -> DISABLE_SHORT_W<DCDC_PROT_SPEC> {
        DISABLE_SHORT_W::new(self, 7)
    }
    #[doc = "Bit 15 - ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage"]
    #[inline(always)]
    #[must_use]
    pub fn disable_overvoltage(&mut self) -> DISABLE_OVERVOLTAGE_W<DCDC_PROT_SPEC> {
        DISABLE_OVERVOLTAGE_W::new(self, 15)
    }
    #[doc = "Bit 23 - disable power loss protection 0: power loss protection enabled, DCDC shuts down when power loss 1: power loss protection disabled, DCDC try working after power voltage drop"]
    #[inline(always)]
    #[must_use]
    pub fn disable_power_loss(&mut self) -> DISABLE_POWER_LOSS_W<DCDC_PROT_SPEC> {
        DISABLE_POWER_LOSS_W::new(self, 23)
    }
    #[doc = "Bit 24 - over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn overload_lp(&mut self) -> OVERLOAD_LP_W<DCDC_PROT_SPEC> {
        OVERLOAD_LP_W::new(self, 24)
    }
    #[doc = "Bit 28 - over current setting for low power mode 0:250mA 1:200mA"]
    #[inline(always)]
    #[must_use]
    pub fn ilimit_lp(&mut self) -> ILIMIT_LP_W<DCDC_PROT_SPEC> {
        ILIMIT_LP_W::new(self, 28)
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
#[doc = "DCDC protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_prot::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_prot::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_PROT_SPEC;
impl crate::RegisterSpec for DCDC_PROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_prot::R`](R) reader structure"]
impl crate::Readable for DCDC_PROT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_prot::W`](W) writer structure"]
impl crate::Writable for DCDC_PROT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_PROT to value 0x10"]
impl crate::Resettable for DCDC_PROT_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
