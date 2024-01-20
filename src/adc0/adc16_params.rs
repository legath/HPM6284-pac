#[doc = "Register `ADC16_PARAMS[%s]` reader"]
pub type R = crate::R<ADC16_PARAMS_SPEC>;
#[doc = "Register `ADC16_PARAMS[%s]` writer"]
pub type W = crate::W<ADC16_PARAMS_SPEC>;
#[doc = "Field `PARAM_VAL` reader - No description avaiable"]
pub type PARAM_VAL_R = crate::FieldReader<u16>;
#[doc = "Field `PARAM_VAL` writer - No description avaiable"]
pub type PARAM_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - No description avaiable"]
    #[inline(always)]
    pub fn param_val(&self) -> PARAM_VAL_R {
        PARAM_VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn param_val(&mut self) -> PARAM_VAL_W<ADC16_PARAMS_SPEC> {
        PARAM_VAL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc16_params::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc16_params::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC16_PARAMS_SPEC;
impl crate::RegisterSpec for ADC16_PARAMS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc16_params::R`](R) reader structure"]
impl crate::Readable for ADC16_PARAMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc16_params::W`](W) writer structure"]
impl crate::Writable for ADC16_PARAMS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADC16_PARAMS[%s]
to value 0"]
impl crate::Resettable for ADC16_PARAMS_SPEC {
    const RESET_VALUE: u16 = 0;
}
