#[doc = "Register `conv_cfg1` reader"]
pub type R = crate::R<CONV_CFG1_SPEC>;
#[doc = "Register `conv_cfg1` writer"]
pub type W = crate::W<CONV_CFG1_SPEC>;
#[doc = "Field `CLOCK_DIVIDER` reader - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
pub type CLOCK_DIVIDER_R = crate::FieldReader;
#[doc = "Field `CLOCK_DIVIDER` writer - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
pub type CLOCK_DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CONVERT_CLOCK_NUMBER` reader - convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 22 adc clock cycles(based on clock after divider); user can use small value to get faster convertion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC convertion(plus sample) need 25 cycles(50MHz)."]
pub type CONVERT_CLOCK_NUMBER_R = crate::FieldReader;
#[doc = "Field `CONVERT_CLOCK_NUMBER` writer - convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 22 adc clock cycles(based on clock after divider); user can use small value to get faster convertion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC convertion(plus sample) need 25 cycles(50MHz)."]
pub type CONVERT_CLOCK_NUMBER_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
    #[inline(always)]
    pub fn clock_divider(&self) -> CLOCK_DIVIDER_R {
        CLOCK_DIVIDER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 22 adc clock cycles(based on clock after divider); user can use small value to get faster convertion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC convertion(plus sample) need 25 cycles(50MHz)."]
    #[inline(always)]
    pub fn convert_clock_number(&self) -> CONVERT_CLOCK_NUMBER_R {
        CONVERT_CLOCK_NUMBER_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divider(&mut self) -> CLOCK_DIVIDER_W<CONV_CFG1_SPEC> {
        CLOCK_DIVIDER_W::new(self, 0)
    }
    #[doc = "Bits 4:8 - convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 22 adc clock cycles(based on clock after divider); user can use small value to get faster convertion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC convertion(plus sample) need 25 cycles(50MHz)."]
    #[inline(always)]
    #[must_use]
    pub fn convert_clock_number(&mut self) -> CONVERT_CLOCK_NUMBER_W<CONV_CFG1_SPEC> {
        CONVERT_CLOCK_NUMBER_W::new(self, 4)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conv_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conv_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONV_CFG1_SPEC;
impl crate::RegisterSpec for CONV_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conv_cfg1::R`](R) reader structure"]
impl crate::Readable for CONV_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conv_cfg1::W`](W) writer structure"]
impl crate::Writable for CONV_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets conv_cfg1 to value 0"]
impl crate::Resettable for CONV_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
