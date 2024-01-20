#[doc = "Register `MUXCFG[%s]` reader"]
pub type R = crate::R<MUXCFG_SPEC>;
#[doc = "Register `MUXCFG[%s]` writer"]
pub type W = crate::W<MUXCFG_SPEC>;
#[doc = "Field `SOURCE` reader - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the \"DMA MUX Mapping\""]
pub type SOURCE_R = crate::FieldReader;
#[doc = "Field `SOURCE` writer - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the \"DMA MUX Mapping\""]
pub type SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ENABLE` reader - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the \"DMA MUX Mapping\""]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA Channel Source Specifies which DMA source, if any, is routed to a particular DMA channel. See the \"DMA MUX Mapping\""]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<MUXCFG_SPEC> {
        SOURCE_W::new(self, 0)
    }
    #[doc = "Bit 31 - DMA Mux Channel Enable Enables the channel for DMA Mux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel. 0b - DMA Mux channel is disabled 1b - DMA Mux channel is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<MUXCFG_SPEC> {
        ENABLE_W::new(self, 31)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXCFG_SPEC;
impl crate::RegisterSpec for MUXCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxcfg::R`](R) reader structure"]
impl crate::Readable for MUXCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxcfg::W`](W) writer structure"]
impl crate::Writable for MUXCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXCFG[%s]
to value 0"]
impl crate::Resettable for MUXCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
