#[doc = "Register `adc16_config0` reader"]
pub type R = crate::R<ADC16_CONFIG0_SPEC>;
#[doc = "Register `adc16_config0` writer"]
pub type W = crate::W<ADC16_CONFIG0_SPEC>;
#[doc = "Field `CONV_PARAM` reader - convertion parameter"]
pub type CONV_PARAM_R = crate::FieldReader<u16>;
#[doc = "Field `CONV_PARAM` writer - convertion parameter"]
pub type CONV_PARAM_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PREEMPT_EN` reader - set to enable preemption feature"]
pub type PREEMPT_EN_R = crate::BitReader;
#[doc = "Field `PREEMPT_EN` writer - set to enable preemption feature"]
pub type PREEMPT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_AVG_CFG` reader - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved"]
pub type CAL_AVG_CFG_R = crate::FieldReader;
#[doc = "Field `CAL_AVG_CFG` writer - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved"]
pub type CAL_AVG_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BANDGAP_EN` reader - set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
pub type BANDGAP_EN_R = crate::BitReader;
#[doc = "Field `BANDGAP_EN` writer - set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
pub type BANDGAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_EN` reader - set to enable regulator"]
pub type REG_EN_R = crate::BitReader;
#[doc = "Field `REG_EN` writer - set to enable regulator"]
pub type REG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPSNS_EN` reader - set to enable temp senser"]
pub type TEMPSNS_EN_R = crate::BitReader;
#[doc = "Field `TEMPSNS_EN` writer - set to enable temp senser"]
pub type TEMPSNS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - convertion parameter"]
    #[inline(always)]
    pub fn conv_param(&self) -> CONV_PARAM_R {
        CONV_PARAM_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - set to enable preemption feature"]
    #[inline(always)]
    pub fn preempt_en(&self) -> PREEMPT_EN_R {
        PREEMPT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 20:22 - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved"]
    #[inline(always)]
    pub fn cal_avg_cfg(&self) -> CAL_AVG_CFG_R {
        CAL_AVG_CFG_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
    #[inline(always)]
    pub fn bandgap_en(&self) -> BANDGAP_EN_R {
        BANDGAP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - set to enable regulator"]
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - set to enable temp senser"]
    #[inline(always)]
    pub fn tempsns_en(&self) -> TEMPSNS_EN_R {
        TEMPSNS_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - convertion parameter"]
    #[inline(always)]
    #[must_use]
    pub fn conv_param(&mut self) -> CONV_PARAM_W<ADC16_CONFIG0_SPEC> {
        CONV_PARAM_W::new(self, 0)
    }
    #[doc = "Bit 14 - set to enable preemption feature"]
    #[inline(always)]
    #[must_use]
    pub fn preempt_en(&mut self) -> PREEMPT_EN_W<ADC16_CONFIG0_SPEC> {
        PREEMPT_EN_W::new(self, 14)
    }
    #[doc = "Bits 20:22 - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cal_avg_cfg(&mut self) -> CAL_AVG_CFG_W<ADC16_CONFIG0_SPEC> {
        CAL_AVG_CFG_W::new(self, 20)
    }
    #[doc = "Bit 23 - set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
    #[inline(always)]
    #[must_use]
    pub fn bandgap_en(&mut self) -> BANDGAP_EN_W<ADC16_CONFIG0_SPEC> {
        BANDGAP_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - set to enable regulator"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en(&mut self) -> REG_EN_W<ADC16_CONFIG0_SPEC> {
        REG_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - set to enable temp senser"]
    #[inline(always)]
    #[must_use]
    pub fn tempsns_en(&mut self) -> TEMPSNS_EN_W<ADC16_CONFIG0_SPEC> {
        TEMPSNS_EN_W::new(self, 25)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc16_config0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc16_config0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC16_CONFIG0_SPEC;
impl crate::RegisterSpec for ADC16_CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc16_config0::R`](R) reader structure"]
impl crate::Readable for ADC16_CONFIG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc16_config0::W`](W) writer structure"]
impl crate::Writable for ADC16_CONFIG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets adc16_config0 to value 0"]
impl crate::Resettable for ADC16_CONFIG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
