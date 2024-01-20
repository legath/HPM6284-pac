#[doc = "Register `dmaen` reader"]
pub type R = crate::R<DMAEN_SPEC>;
#[doc = "Register `dmaen` writer"]
pub type W = crate::W<DMAEN_SPEC>;
#[doc = "Field `REDGEN` reader - Output rising edge flag DMA request enable bit."]
pub type REDGEN_R = crate::BitReader;
#[doc = "Field `REDGEN` writer - Output rising edge flag DMA request enable bit."]
pub type REDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEDGEN` reader - Output falling edge flag DMA request enable bit."]
pub type FEDGEN_R = crate::BitReader;
#[doc = "Field `FEDGEN` writer - Output falling edge flag DMA request enable bit."]
pub type FEDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output rising edge flag DMA request enable bit."]
    #[inline(always)]
    pub fn redgen(&self) -> REDGEN_R {
        REDGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output falling edge flag DMA request enable bit."]
    #[inline(always)]
    pub fn fedgen(&self) -> FEDGEN_R {
        FEDGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output rising edge flag DMA request enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn redgen(&mut self) -> REDGEN_W<DMAEN_SPEC> {
        REDGEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output falling edge flag DMA request enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn fedgen(&mut self) -> FEDGEN_W<DMAEN_SPEC> {
        FEDGEN_W::new(self, 1)
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
#[doc = "DMA request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAEN_SPEC;
impl crate::RegisterSpec for DMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaen::R`](R) reader structure"]
impl crate::Readable for DMAEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaen::W`](W) writer structure"]
impl crate::Writable for DMAEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dmaen to value 0"]
impl crate::Resettable for DMAEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
