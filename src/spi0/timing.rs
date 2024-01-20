#[doc = "Register `Timing` reader"]
pub type R = crate::R<TIMING_SPEC>;
#[doc = "Register `Timing` writer"]
pub type W = crate::W<TIMING_SPEC>;
#[doc = "Field `SCLK_DIV` reader - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
pub type SCLK_DIV_R = crate::FieldReader;
#[doc = "Field `SCLK_DIV` writer - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
pub type SCLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSHT` reader - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2"]
pub type CSHT_R = crate::FieldReader;
#[doc = "Field `CSHT` writer - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2"]
pub type CSHT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CS2SCLK` reader - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2"]
pub type CS2SCLK_R = crate::FieldReader;
#[doc = "Field `CS2SCLK` writer - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2"]
pub type CS2SCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
    #[inline(always)]
    pub fn sclk_div(&self) -> SCLK_DIV_R {
        SCLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2"]
    #[inline(always)]
    pub fn cs2sclk(&self) -> CS2SCLK_R {
        CS2SCLK_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div(&mut self) -> SCLK_DIV_W<TIMING_SPEC> {
        SCLK_DIV_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2"]
    #[inline(always)]
    #[must_use]
    pub fn csht(&mut self) -> CSHT_W<TIMING_SPEC> {
        CSHT_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2"]
    #[inline(always)]
    #[must_use]
    pub fn cs2sclk(&mut self) -> CS2SCLK_W<TIMING_SPEC> {
        CS2SCLK_W::new(self, 12)
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
#[doc = "Interface Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMING_SPEC;
impl crate::RegisterSpec for TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing::R`](R) reader structure"]
impl crate::Readable for TIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timing::W`](W) writer structure"]
impl crate::Writable for TIMING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Timing to value 0"]
impl crate::Resettable for TIMING_SPEC {
    const RESET_VALUE: u32 = 0;
}
