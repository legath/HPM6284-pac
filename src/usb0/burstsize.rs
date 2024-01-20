#[doc = "Register `BURSTSIZE` reader"]
pub type R = crate::R<BURSTSIZE_SPEC>;
#[doc = "Register `BURSTSIZE` writer"]
pub type W = crate::W<BURSTSIZE_SPEC>;
#[doc = "Field `RXPBURST` reader - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory."]
pub type RXPBURST_R = crate::FieldReader;
#[doc = "Field `RXPBURST` writer - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory."]
pub type RXPBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXPBURST` reader - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus."]
pub type TXPBURST_R = crate::FieldReader;
#[doc = "Field `TXPBURST` writer - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus."]
pub type TXPBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory."]
    #[inline(always)]
    pub fn rxpburst(&self) -> RXPBURST_R {
        RXPBURST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus."]
    #[inline(always)]
    pub fn txpburst(&self) -> TXPBURST_R {
        TXPBURST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory."]
    #[inline(always)]
    #[must_use]
    pub fn rxpburst(&mut self) -> RXPBURST_W<BURSTSIZE_SPEC> {
        RXPBURST_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus."]
    #[inline(always)]
    #[must_use]
    pub fn txpburst(&mut self) -> TXPBURST_W<BURSTSIZE_SPEC> {
        TXPBURST_W::new(self, 8)
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
#[doc = "Programmable Burst Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`burstsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`burstsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BURSTSIZE_SPEC;
impl crate::RegisterSpec for BURSTSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`burstsize::R`](R) reader structure"]
impl crate::Readable for BURSTSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`burstsize::W`](W) writer structure"]
impl crate::Writable for BURSTSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BURSTSIZE to value 0"]
impl crate::Resettable for BURSTSIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
