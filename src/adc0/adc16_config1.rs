#[doc = "Register `adc16_config1` reader"]
pub type R = crate::R<ADC16_CONFIG1_SPEC>;
#[doc = "Register `adc16_config1` writer"]
pub type W = crate::W<ADC16_CONFIG1_SPEC>;
#[doc = "Field `COV_END_CNT` reader - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1)."]
pub type COV_END_CNT_R = crate::FieldReader;
#[doc = "Field `COV_END_CNT` writer - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1)."]
pub type COV_END_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 8:12 - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1)."]
    #[inline(always)]
    pub fn cov_end_cnt(&self) -> COV_END_CNT_R {
        COV_END_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1)."]
    #[inline(always)]
    #[must_use]
    pub fn cov_end_cnt(&mut self) -> COV_END_CNT_W<ADC16_CONFIG1_SPEC> {
        COV_END_CNT_W::new(self, 8)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc16_config1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc16_config1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC16_CONFIG1_SPEC;
impl crate::RegisterSpec for ADC16_CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc16_config1::R`](R) reader structure"]
impl crate::Readable for ADC16_CONFIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc16_config1::W`](W) writer structure"]
impl crate::Writable for ADC16_CONFIG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets adc16_config1 to value 0"]
impl crate::Resettable for ADC16_CONFIG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
