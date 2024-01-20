#[doc = "Register `CFG_FF` reader"]
pub type R = crate::R<CFG_FF_SPEC>;
#[doc = "Register `CFG_FF` writer"]
pub type W = crate::W<CFG_FF_SPEC>;
#[doc = "Field `SEL_CFG_FF_TYPE` reader - cfg_ff type. 0: DFF. 1: 3rd_filter_0. 2: dual-edge DFF. 3: Trigger FF. 4: JK FF. 5. latch. 6: full adder/minus."]
pub type SEL_CFG_FF_TYPE_R = crate::FieldReader;
#[doc = "Field `SEL_CFG_FF_TYPE` writer - cfg_ff type. 0: DFF. 1: 3rd_filter_0. 2: dual-edge DFF. 3: Trigger FF. 4: JK FF. 5. latch. 6: full adder/minus."]
pub type SEL_CFG_FF_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEL_CLK_SOURCE` reader - cfg_ff clock source. 0: system clock. 1: use 3rd_filter_2 as clock."]
pub type SEL_CLK_SOURCE_R = crate::BitReader;
#[doc = "Field `SEL_CLK_SOURCE` writer - cfg_ff clock source. 0: system clock. 1: use 3rd_filter_2 as clock."]
pub type SEL_CLK_SOURCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_ADDER_MINUS` reader - 0: select adder when cfg_adder_minus active. 1: select minus when cfg_adder_minus active."]
pub type SEL_ADDER_MINUS_R = crate::BitReader;
#[doc = "Field `SEL_ADDER_MINUS` writer - 0: select adder when cfg_adder_minus active. 1: select minus when cfg_adder_minus active."]
pub type SEL_ADDER_MINUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_OSC_LOOP_CLAMP` reader - disable osc loop clamp. 0: enable osc loop clamp when osc ring active. 1: disable or clean current osc loop clamp."]
pub type DIS_OSC_LOOP_CLAMP_R = crate::BitReader;
#[doc = "Field `DIS_OSC_LOOP_CLAMP` writer - disable osc loop clamp. 0: enable osc loop clamp when osc ring active. 1: disable or clean current osc loop clamp."]
pub type DIS_OSC_LOOP_CLAMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC_LOOP_CLAMP_VALUE` reader - osc loop clamp value when osc ring active. 0: clamp 0. 1: clamp 1."]
pub type OSC_LOOP_CLAMP_VALUE_R = crate::BitReader;
#[doc = "Field `OSC_LOOP_CLAMP_VALUE` writer - osc loop clamp value when osc ring active. 0: clamp 0. 1: clamp 1."]
pub type OSC_LOOP_CLAMP_VALUE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - cfg_ff type. 0: DFF. 1: 3rd_filter_0. 2: dual-edge DFF. 3: Trigger FF. 4: JK FF. 5. latch. 6: full adder/minus."]
    #[inline(always)]
    pub fn sel_cfg_ff_type(&self) -> SEL_CFG_FF_TYPE_R {
        SEL_CFG_FF_TYPE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - cfg_ff clock source. 0: system clock. 1: use 3rd_filter_2 as clock."]
    #[inline(always)]
    pub fn sel_clk_source(&self) -> SEL_CLK_SOURCE_R {
        SEL_CLK_SOURCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: select adder when cfg_adder_minus active. 1: select minus when cfg_adder_minus active."]
    #[inline(always)]
    pub fn sel_adder_minus(&self) -> SEL_ADDER_MINUS_R {
        SEL_ADDER_MINUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - disable osc loop clamp. 0: enable osc loop clamp when osc ring active. 1: disable or clean current osc loop clamp."]
    #[inline(always)]
    pub fn dis_osc_loop_clamp(&self) -> DIS_OSC_LOOP_CLAMP_R {
        DIS_OSC_LOOP_CLAMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - osc loop clamp value when osc ring active. 0: clamp 0. 1: clamp 1."]
    #[inline(always)]
    pub fn osc_loop_clamp_value(&self) -> OSC_LOOP_CLAMP_VALUE_R {
        OSC_LOOP_CLAMP_VALUE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - cfg_ff type. 0: DFF. 1: 3rd_filter_0. 2: dual-edge DFF. 3: Trigger FF. 4: JK FF. 5. latch. 6: full adder/minus."]
    #[inline(always)]
    #[must_use]
    pub fn sel_cfg_ff_type(&mut self) -> SEL_CFG_FF_TYPE_W<CFG_FF_SPEC> {
        SEL_CFG_FF_TYPE_W::new(self, 0)
    }
    #[doc = "Bit 3 - cfg_ff clock source. 0: system clock. 1: use 3rd_filter_2 as clock."]
    #[inline(always)]
    #[must_use]
    pub fn sel_clk_source(&mut self) -> SEL_CLK_SOURCE_W<CFG_FF_SPEC> {
        SEL_CLK_SOURCE_W::new(self, 3)
    }
    #[doc = "Bit 4 - 0: select adder when cfg_adder_minus active. 1: select minus when cfg_adder_minus active."]
    #[inline(always)]
    #[must_use]
    pub fn sel_adder_minus(&mut self) -> SEL_ADDER_MINUS_W<CFG_FF_SPEC> {
        SEL_ADDER_MINUS_W::new(self, 4)
    }
    #[doc = "Bit 16 - disable osc loop clamp. 0: enable osc loop clamp when osc ring active. 1: disable or clean current osc loop clamp."]
    #[inline(always)]
    #[must_use]
    pub fn dis_osc_loop_clamp(&mut self) -> DIS_OSC_LOOP_CLAMP_W<CFG_FF_SPEC> {
        DIS_OSC_LOOP_CLAMP_W::new(self, 16)
    }
    #[doc = "Bit 17 - osc loop clamp value when osc ring active. 0: clamp 0. 1: clamp 1."]
    #[inline(always)]
    #[must_use]
    pub fn osc_loop_clamp_value(&mut self) -> OSC_LOOP_CLAMP_VALUE_W<CFG_FF_SPEC> {
        OSC_LOOP_CLAMP_VALUE_W::new(self, 17)
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
#[doc = "CHN&amp;index0 cfg ff\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_FF_SPEC;
impl crate::RegisterSpec for CFG_FF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_ff::R`](R) reader structure"]
impl crate::Readable for CFG_FF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_ff::W`](W) writer structure"]
impl crate::Writable for CFG_FF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_FF to value 0"]
impl crate::Resettable for CFG_FF_SPEC {
    const RESET_VALUE: u32 = 0;
}
