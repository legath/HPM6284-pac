#[doc = "Register `Config` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `Config` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `RXFIFOSIZE` reader - Depth of RX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words"]
pub type RXFIFOSIZE_R = crate::FieldReader;
#[doc = "Field `TXFIFOSIZE` reader - Depth of TX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words"]
pub type TXFIFOSIZE_R = crate::FieldReader;
#[doc = "Field `DUALSPI` reader - Support for Dual I/O SPI"]
pub type DUALSPI_R = crate::BitReader;
#[doc = "Field `QUADSPI` reader - Support for Quad I/O SPI"]
pub type QUADSPI_R = crate::BitReader;
#[doc = "Field `SLAVE` reader - Support for SPI Slave mode"]
pub type SLAVE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Depth of RX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Depth of TX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Support for Dual I/O SPI"]
    #[inline(always)]
    pub fn dualspi(&self) -> DUALSPI_R {
        DUALSPI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Support for Quad I/O SPI"]
    #[inline(always)]
    pub fn quadspi(&self) -> QUADSPI_R {
        QUADSPI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Support for SPI Slave mode"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
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
#[doc = "Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Config to value 0x4311"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x4311;
}
