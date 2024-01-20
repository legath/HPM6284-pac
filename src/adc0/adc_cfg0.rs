#[doc = "Register `adc_cfg0` reader"]
pub type R = crate::R<ADC_CFG0_SPEC>;
#[doc = "Register `adc_cfg0` writer"]
pub type W = crate::W<ADC_CFG0_SPEC>;
#[doc = "Field `PORT3_REALTIME` reader - set to enable trg queue stop other queues"]
pub type PORT3_REALTIME_R = crate::BitReader;
#[doc = "Field `PORT3_REALTIME` writer - set to enable trg queue stop other queues"]
pub type PORT3_REALTIME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONVERT_DURATION` reader - for trigger queue, from trg_sample_req to trg_convert_req"]
pub type CONVERT_DURATION_R = crate::FieldReader<u16>;
#[doc = "Field `CONVERT_DURATION` writer - for trigger queue, from trg_sample_req to trg_convert_req"]
pub type CONVERT_DURATION_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADC_AHB_EN` reader - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
pub type ADC_AHB_EN_R = crate::BitReader;
#[doc = "Field `ADC_AHB_EN` writer - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
pub type ADC_AHB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_SYNC_AHB` reader - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
pub type SEL_SYNC_AHB_R = crate::BitReader;
#[doc = "Field `SEL_SYNC_AHB` writer - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
pub type SEL_SYNC_AHB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set to enable trg queue stop other queues"]
    #[inline(always)]
    pub fn port3_realtime(&self) -> PORT3_REALTIME_R {
        PORT3_REALTIME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:27 - for trigger queue, from trg_sample_req to trg_convert_req"]
    #[inline(always)]
    pub fn convert_duration(&self) -> CONVERT_DURATION_R {
        CONVERT_DURATION_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
    #[inline(always)]
    pub fn adc_ahb_en(&self) -> ADC_AHB_EN_R {
        ADC_AHB_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
    #[inline(always)]
    pub fn sel_sync_ahb(&self) -> SEL_SYNC_AHB_R {
        SEL_SYNC_AHB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set to enable trg queue stop other queues"]
    #[inline(always)]
    #[must_use]
    pub fn port3_realtime(&mut self) -> PORT3_REALTIME_W<ADC_CFG0_SPEC> {
        PORT3_REALTIME_W::new(self, 0)
    }
    #[doc = "Bits 12:27 - for trigger queue, from trg_sample_req to trg_convert_req"]
    #[inline(always)]
    #[must_use]
    pub fn convert_duration(&mut self) -> CONVERT_DURATION_W<ADC_CFG0_SPEC> {
        CONVERT_DURATION_W::new(self, 12)
    }
    #[doc = "Bit 29 - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ahb_en(&mut self) -> ADC_AHB_EN_W<ADC_CFG0_SPEC> {
        ADC_AHB_EN_W::new(self, 29)
    }
    #[doc = "Bit 31 - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
    #[inline(always)]
    #[must_use]
    pub fn sel_sync_ahb(&mut self) -> SEL_SYNC_AHB_W<ADC_CFG0_SPEC> {
        SEL_SYNC_AHB_W::new(self, 31)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CFG0_SPEC;
impl crate::RegisterSpec for ADC_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cfg0::R`](R) reader structure"]
impl crate::Readable for ADC_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_cfg0::W`](W) writer structure"]
impl crate::Writable for ADC_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets adc_cfg0 to value 0"]
impl crate::Resettable for ADC_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
