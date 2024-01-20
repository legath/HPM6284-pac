#[doc = "Register `ana_ctrl0` reader"]
pub type R = crate::R<ANA_CTRL0_SPEC>;
#[doc = "Register `ana_ctrl0` writer"]
pub type W = crate::W<ANA_CTRL0_SPEC>;
#[doc = "Field `STARTCAL` reader - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
pub type STARTCAL_R = crate::BitReader;
#[doc = "Field `STARTCAL` writer - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
pub type STARTCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_CLK_ON` reader - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access"]
pub type ADC_CLK_ON_R = crate::BitReader;
#[doc = "Field `ADC_CLK_ON` writer - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access"]
pub type ADC_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
    #[inline(always)]
    pub fn startcal(&self) -> STARTCAL_R {
        STARTCAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access"]
    #[inline(always)]
    pub fn adc_clk_on(&self) -> ADC_CLK_ON_R {
        ADC_CLK_ON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
    #[inline(always)]
    #[must_use]
    pub fn startcal(&mut self) -> STARTCAL_W<ANA_CTRL0_SPEC> {
        STARTCAL_W::new(self, 2)
    }
    #[doc = "Bit 12 - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_on(&mut self) -> ADC_CLK_ON_W<ANA_CTRL0_SPEC> {
        ADC_CLK_ON_W::new(self, 12)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_CTRL0_SPEC;
impl crate::RegisterSpec for ANA_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_ctrl0::R`](R) reader structure"]
impl crate::Readable for ANA_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_ctrl0::W`](W) writer structure"]
impl crate::Writable for ANA_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ana_ctrl0 to value 0"]
impl crate::Resettable for ANA_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
