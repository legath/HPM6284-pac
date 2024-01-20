#[doc = "Register `ana_cfg0` reader"]
pub type R = crate::R<ANA_CFG0_SPEC>;
#[doc = "Register `ana_cfg0` writer"]
pub type W = crate::W<ANA_CFG0_SPEC>;
#[doc = "Field `DAC12BIT_EN` reader - No description avaiable"]
pub type DAC12BIT_EN_R = crate::BitReader;
#[doc = "Field `DAC12BIT_EN` writer - No description avaiable"]
pub type DAC12BIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS_CALI_GM` reader - No description avaiable"]
pub type BYPASS_CALI_GM_R = crate::BitReader;
#[doc = "Field `BYPASS_CALI_GM` writer - No description avaiable"]
pub type BYPASS_CALI_GM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALI_DELTA_V_CFG` reader - No description avaiable"]
pub type CALI_DELTA_V_CFG_R = crate::FieldReader;
#[doc = "Field `CALI_DELTA_V_CFG` writer - No description avaiable"]
pub type CALI_DELTA_V_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAC_CONFIG` reader - No description avaiable"]
pub type DAC_CONFIG_R = crate::FieldReader;
#[doc = "Field `DAC_CONFIG` writer - No description avaiable"]
pub type DAC_CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAC12BIT_LP_MODE` reader - No description avaiable"]
pub type DAC12BIT_LP_MODE_R = crate::BitReader;
#[doc = "Field `DAC12BIT_LP_MODE` writer - No description avaiable"]
pub type DAC12BIT_LP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn dac12bit_en(&self) -> DAC12BIT_EN_R {
        DAC12BIT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn bypass_cali_gm(&self) -> BYPASS_CALI_GM_R {
        BYPASS_CALI_GM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - No description avaiable"]
    #[inline(always)]
    pub fn cali_delta_v_cfg(&self) -> CALI_DELTA_V_CFG_R {
        CALI_DELTA_V_CFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - No description avaiable"]
    #[inline(always)]
    pub fn dac_config(&self) -> DAC_CONFIG_R {
        DAC_CONFIG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - No description avaiable"]
    #[inline(always)]
    pub fn dac12bit_lp_mode(&self) -> DAC12BIT_LP_MODE_R {
        DAC12BIT_LP_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn dac12bit_en(&mut self) -> DAC12BIT_EN_W<ANA_CFG0_SPEC> {
        DAC12BIT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_cali_gm(&mut self) -> BYPASS_CALI_GM_W<ANA_CFG0_SPEC> {
        BYPASS_CALI_GM_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn cali_delta_v_cfg(&mut self) -> CALI_DELTA_V_CFG_W<ANA_CFG0_SPEC> {
        CALI_DELTA_V_CFG_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_config(&mut self) -> DAC_CONFIG_W<ANA_CFG0_SPEC> {
        DAC_CONFIG_W::new(self, 4)
    }
    #[doc = "Bit 8 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn dac12bit_lp_mode(&mut self) -> DAC12BIT_LP_MODE_W<ANA_CFG0_SPEC> {
        DAC12BIT_LP_MODE_W::new(self, 8)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_CFG0_SPEC;
impl crate::RegisterSpec for ANA_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_cfg0::R`](R) reader structure"]
impl crate::Readable for ANA_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_cfg0::W`](W) writer structure"]
impl crate::Writable for ANA_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ana_cfg0 to value 0x30"]
impl crate::Resettable for ANA_CFG0_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
