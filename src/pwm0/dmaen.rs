#[doc = "Register `dmaen` reader"]
pub type R = crate::R<DMAEN_SPEC>;
#[doc = "Register `dmaen` writer"]
pub type W = crate::W<DMAEN_SPEC>;
#[doc = "Field `CMPENX` reader - comparator output compare or input capture flag DMA request enable"]
pub type CMPENX_R = crate::FieldReader<u32>;
#[doc = "Field `CMPENX` writer - comparator output compare or input capture flag DMA request enable"]
pub type CMPENX_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RLDEN` reader - reload flag DMA request enable"]
pub type RLDEN_R = crate::BitReader;
#[doc = "Field `RLDEN` writer - reload flag DMA request enable"]
pub type RLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALFRLDEN` reader - half reload flag DMA request enable"]
pub type HALFRLDEN_R = crate::BitReader;
#[doc = "Field `HALFRLDEN` writer - half reload flag DMA request enable"]
pub type HALFRLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XRLDEN` reader - extended reload flag DMA request enable"]
pub type XRLDEN_R = crate::BitReader;
#[doc = "Field `XRLDEN` writer - extended reload flag DMA request enable"]
pub type XRLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTEN` reader - fault condition DMA request enable"]
pub type FAULTEN_R = crate::BitReader;
#[doc = "Field `FAULTEN` writer - fault condition DMA request enable"]
pub type FAULTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - comparator output compare or input capture flag DMA request enable"]
    #[inline(always)]
    pub fn cmpenx(&self) -> CMPENX_R {
        CMPENX_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - reload flag DMA request enable"]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - half reload flag DMA request enable"]
    #[inline(always)]
    pub fn halfrlden(&self) -> HALFRLDEN_R {
        HALFRLDEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - extended reload flag DMA request enable"]
    #[inline(always)]
    pub fn xrlden(&self) -> XRLDEN_R {
        XRLDEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - fault condition DMA request enable"]
    #[inline(always)]
    pub fn faulten(&self) -> FAULTEN_R {
        FAULTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - comparator output compare or input capture flag DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpenx(&mut self) -> CMPENX_W<DMAEN_SPEC> {
        CMPENX_W::new(self, 0)
    }
    #[doc = "Bit 24 - reload flag DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn rlden(&mut self) -> RLDEN_W<DMAEN_SPEC> {
        RLDEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - half reload flag DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfrlden(&mut self) -> HALFRLDEN_W<DMAEN_SPEC> {
        HALFRLDEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - extended reload flag DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn xrlden(&mut self) -> XRLDEN_W<DMAEN_SPEC> {
        XRLDEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - fault condition DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn faulten(&mut self) -> FAULTEN_W<DMAEN_SPEC> {
        FAULTEN_W::new(self, 27)
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
