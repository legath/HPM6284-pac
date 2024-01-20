#[doc = "Register `DCDC_LPMODE` reader"]
pub type R = crate::R<DCDC_LPMODE_SPEC>;
#[doc = "Register `DCDC_LPMODE` writer"]
pub type W = crate::W<DCDC_LPMODE_SPEC>;
#[doc = "Field `STBY_VOLT` reader - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
pub type STBY_VOLT_R = crate::FieldReader<u16>;
#[doc = "Field `STBY_VOLT` writer - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
pub type STBY_VOLT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    #[inline(always)]
    pub fn stby_volt(&self) -> STBY_VOLT_R {
        STBY_VOLT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    #[inline(always)]
    #[must_use]
    pub fn stby_volt(&mut self) -> STBY_VOLT_W<DCDC_LPMODE_SPEC> {
        STBY_VOLT_W::new(self, 0)
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
#[doc = "DCDC low power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_lpmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_lpmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_LPMODE_SPEC;
impl crate::RegisterSpec for DCDC_LPMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_lpmode::R`](R) reader structure"]
impl crate::Readable for DCDC_LPMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_lpmode::W`](W) writer structure"]
impl crate::Writable for DCDC_LPMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_LPMODE to value 0x0384"]
impl crate::Resettable for DCDC_LPMODE_SPEC {
    const RESET_VALUE: u32 = 0x0384;
}
