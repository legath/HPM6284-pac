#[doc = "Register `dmaen` reader"]
pub type R = crate::R<DMAEN_SPEC>;
#[doc = "Register `dmaen` writer"]
pub type W = crate::W<DMAEN_SPEC>;
#[doc = "Field `ZPHFEN` reader - 1- generate dma request when zphf flag set"]
pub type ZPHFEN_R = crate::BitReader;
#[doc = "Field `ZPHFEN` writer - 1- generate dma request when zphf flag set"]
pub type ZPHFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSCMPFEN` reader - 1- generate dma request when poscmpf flag set"]
pub type POSCMPFEN_R = crate::BitReader;
#[doc = "Field `POSCMPFEN` writer - 1- generate dma request when poscmpf flag set"]
pub type POSCMPFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOMEFEN` reader - 1- generate dma request when homef flag set"]
pub type HOMEFEN_R = crate::BitReader;
#[doc = "Field `HOMEFEN` writer - 1- generate dma request when homef flag set"]
pub type HOMEFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGFEN` reader - 1- generate dma request when wdg flag set"]
pub type WDGFEN_R = crate::BitReader;
#[doc = "Field `WDGFEN` writer - 1- generate dma request when wdg flag set"]
pub type WDGFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - 1- generate dma request when zphf flag set"]
    #[inline(always)]
    pub fn zphfen(&self) -> ZPHFEN_R {
        ZPHFEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- generate dma request when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpfen(&self) -> POSCMPFEN_R {
        POSCMPFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- generate dma request when homef flag set"]
    #[inline(always)]
    pub fn homefen(&self) -> HOMEFEN_R {
        HOMEFEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- generate dma request when wdg flag set"]
    #[inline(always)]
    pub fn wdgfen(&self) -> WDGFEN_R {
        WDGFEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - 1- generate dma request when zphf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn zphfen(&mut self) -> ZPHFEN_W<DMAEN_SPEC> {
        ZPHFEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1- generate dma request when poscmpf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn poscmpfen(&mut self) -> POSCMPFEN_W<DMAEN_SPEC> {
        POSCMPFEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1- generate dma request when homef flag set"]
    #[inline(always)]
    #[must_use]
    pub fn homefen(&mut self) -> HOMEFEN_W<DMAEN_SPEC> {
        HOMEFEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1- generate dma request when wdg flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wdgfen(&mut self) -> WDGFEN_W<DMAEN_SPEC> {
        WDGFEN_W::new(self, 31)
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
