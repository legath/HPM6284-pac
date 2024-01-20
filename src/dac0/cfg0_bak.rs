#[doc = "Register `cfg0_bak` reader"]
pub type R = crate::R<CFG0_BAK_SPEC>;
#[doc = "Register `cfg0_bak` writer"]
pub type W = crate::W<CFG0_BAK_SPEC>;
#[doc = "Field `HBURST_CFG` reader - DAC support following fixed burst only 000-SINGLE; 011-INCR4; 101: INCR8 others are reserved"]
pub type HBURST_CFG_R = crate::FieldReader;
#[doc = "Field `HBURST_CFG` writer - DAC support following fixed burst only 000-SINGLE; 011-INCR4; 101: INCR8 others are reserved"]
pub type HBURST_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUF_DATA_MODE` reader - data structure for buffer mode, 0: each 32-bit data contains 2 points, b11:0 for first, b27:16 for second. 1: each 32-bit data contains 1 point, b11:0 for first"]
pub type BUF_DATA_MODE_R = crate::BitReader;
#[doc = "Field `BUF_DATA_MODE` writer - data structure for buffer mode, 0: each 32-bit data contains 2 points, b11:0 for first, b27:16 for second. 1: each 32-bit data contains 1 point, b11:0 for first"]
pub type BUF_DATA_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_MODE` reader - 00: direct mode, DAC output the fixed configured data(from sw_dac_data) 01: step mode, DAC output from start_point to end point, with configured step, can step up or step down 10: buffer mode, read data from buffer, then output to analog, internal DMA will load next burst if enough space in local FIFO;"]
pub type DAC_MODE_R = crate::FieldReader;
#[doc = "Field `DAC_MODE` writer - 00: direct mode, DAC output the fixed configured data(from sw_dac_data) 01: step mode, DAC output from start_point to end point, with configured step, can step up or step down 10: buffer mode, read data from buffer, then output to analog, internal DMA will load next burst if enough space in local FIFO;"]
pub type DAC_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HW_TRIG_EN` reader - set to use trigger signal from trigger_mux, user should config it to pulse in single mode, and level in continual mode"]
pub type HW_TRIG_EN_R = crate::BitReader;
#[doc = "Field `HW_TRIG_EN` writer - set to use trigger signal from trigger_mux, user should config it to pulse in single mode, and level in continual mode"]
pub type HW_TRIG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG_MODE` reader - 0: single mode, one trigger pulse will send one 12bit data to DAC analog; 1: continual mode, if trigger signal(either or HW) is set, DAC will send data if FIFO is not empty, if trigger signal is clear, DAC will stop send data."]
pub type TRIG_MODE_R = crate::BitReader;
#[doc = "Field `TRIG_MODE` writer - 0: single mode, one trigger pulse will send one 12bit data to DAC analog; 1: continual mode, if trigger signal(either or HW) is set, DAC will send data if FIFO is not empty, if trigger signal is clear, DAC will stop send data."]
pub type TRIG_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_MODE` reader - 1: sync dac clock and ahb clock. all HW trigger signals are pulse in sync mode, can get faster response; 0: async dac clock and ahb_clock all HW trigger signals should be level and should be more than one dac clock cycle, used to get accurate output frequency(which may not be divided from AHB clock)"]
pub type SYNC_MODE_R = crate::BitReader;
#[doc = "Field `SYNC_MODE` writer - 1: sync dac clock and ahb clock. all HW trigger signals are pulse in sync mode, can get faster response; 0: async dac clock and ahb_clock all HW trigger signals should be level and should be more than one dac clock cycle, used to get accurate output frequency(which may not be divided from AHB clock)"]
pub type SYNC_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_AHB_EN` reader - set to enable internal DMA, it will read one burst if enough space in FIFO. Should only be used in buffer mode."]
pub type DMA_AHB_EN_R = crate::BitReader;
#[doc = "Field `DMA_AHB_EN` writer - set to enable internal DMA, it will read one burst if enough space in FIFO. Should only be used in buffer mode."]
pub type DMA_AHB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_DAC_DATA` reader - dac data used in direct mode(dac_mode==2'b10)"]
pub type SW_DAC_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `SW_DAC_DATA` writer - dac data used in direct mode(dac_mode==2'b10)"]
pub type SW_DAC_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:2 - DAC support following fixed burst only 000-SINGLE; 011-INCR4; 101: INCR8 others are reserved"]
    #[inline(always)]
    pub fn hburst_cfg(&self) -> HBURST_CFG_R {
        HBURST_CFG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - data structure for buffer mode, 0: each 32-bit data contains 2 points, b11:0 for first, b27:16 for second. 1: each 32-bit data contains 1 point, b11:0 for first"]
    #[inline(always)]
    pub fn buf_data_mode(&self) -> BUF_DATA_MODE_R {
        BUF_DATA_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 00: direct mode, DAC output the fixed configured data(from sw_dac_data) 01: step mode, DAC output from start_point to end point, with configured step, can step up or step down 10: buffer mode, read data from buffer, then output to analog, internal DMA will load next burst if enough space in local FIFO;"]
    #[inline(always)]
    pub fn dac_mode(&self) -> DAC_MODE_R {
        DAC_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - set to use trigger signal from trigger_mux, user should config it to pulse in single mode, and level in continual mode"]
    #[inline(always)]
    pub fn hw_trig_en(&self) -> HW_TRIG_EN_R {
        HW_TRIG_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: single mode, one trigger pulse will send one 12bit data to DAC analog; 1: continual mode, if trigger signal(either or HW) is set, DAC will send data if FIFO is not empty, if trigger signal is clear, DAC will stop send data."]
    #[inline(always)]
    pub fn trig_mode(&self) -> TRIG_MODE_R {
        TRIG_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: sync dac clock and ahb clock. all HW trigger signals are pulse in sync mode, can get faster response; 0: async dac clock and ahb_clock all HW trigger signals should be level and should be more than one dac clock cycle, used to get accurate output frequency(which may not be divided from AHB clock)"]
    #[inline(always)]
    pub fn sync_mode(&self) -> SYNC_MODE_R {
        SYNC_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - set to enable internal DMA, it will read one burst if enough space in FIFO. Should only be used in buffer mode."]
    #[inline(always)]
    pub fn dma_ahb_en(&self) -> DMA_AHB_EN_R {
        DMA_AHB_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:27 - dac data used in direct mode(dac_mode==2'b10)"]
    #[inline(always)]
    pub fn sw_dac_data(&self) -> SW_DAC_DATA_R {
        SW_DAC_DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC support following fixed burst only 000-SINGLE; 011-INCR4; 101: INCR8 others are reserved"]
    #[inline(always)]
    #[must_use]
    pub fn hburst_cfg(&mut self) -> HBURST_CFG_W<CFG0_BAK_SPEC> {
        HBURST_CFG_W::new(self, 0)
    }
    #[doc = "Bit 3 - data structure for buffer mode, 0: each 32-bit data contains 2 points, b11:0 for first, b27:16 for second. 1: each 32-bit data contains 1 point, b11:0 for first"]
    #[inline(always)]
    #[must_use]
    pub fn buf_data_mode(&mut self) -> BUF_DATA_MODE_W<CFG0_BAK_SPEC> {
        BUF_DATA_MODE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - 00: direct mode, DAC output the fixed configured data(from sw_dac_data) 01: step mode, DAC output from start_point to end point, with configured step, can step up or step down 10: buffer mode, read data from buffer, then output to analog, internal DMA will load next burst if enough space in local FIFO;"]
    #[inline(always)]
    #[must_use]
    pub fn dac_mode(&mut self) -> DAC_MODE_W<CFG0_BAK_SPEC> {
        DAC_MODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - set to use trigger signal from trigger_mux, user should config it to pulse in single mode, and level in continual mode"]
    #[inline(always)]
    #[must_use]
    pub fn hw_trig_en(&mut self) -> HW_TRIG_EN_W<CFG0_BAK_SPEC> {
        HW_TRIG_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - 0: single mode, one trigger pulse will send one 12bit data to DAC analog; 1: continual mode, if trigger signal(either or HW) is set, DAC will send data if FIFO is not empty, if trigger signal is clear, DAC will stop send data."]
    #[inline(always)]
    #[must_use]
    pub fn trig_mode(&mut self) -> TRIG_MODE_W<CFG0_BAK_SPEC> {
        TRIG_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1: sync dac clock and ahb clock. all HW trigger signals are pulse in sync mode, can get faster response; 0: async dac clock and ahb_clock all HW trigger signals should be level and should be more than one dac clock cycle, used to get accurate output frequency(which may not be divided from AHB clock)"]
    #[inline(always)]
    #[must_use]
    pub fn sync_mode(&mut self) -> SYNC_MODE_W<CFG0_BAK_SPEC> {
        SYNC_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - set to enable internal DMA, it will read one burst if enough space in FIFO. Should only be used in buffer mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma_ahb_en(&mut self) -> DMA_AHB_EN_W<CFG0_BAK_SPEC> {
        DMA_AHB_EN_W::new(self, 9)
    }
    #[doc = "Bits 16:27 - dac data used in direct mode(dac_mode==2'b10)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dac_data(&mut self) -> SW_DAC_DATA_W<CFG0_BAK_SPEC> {
        SW_DAC_DATA_W::new(self, 16)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_bak::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_bak::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG0_BAK_SPEC;
impl crate::RegisterSpec for CFG0_BAK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_bak::R`](R) reader structure"]
impl crate::Readable for CFG0_BAK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg0_bak::W`](W) writer structure"]
impl crate::Writable for CFG0_BAK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cfg0_bak to value 0"]
impl crate::Resettable for CFG0_BAK_SPEC {
    const RESET_VALUE: u32 = 0;
}
