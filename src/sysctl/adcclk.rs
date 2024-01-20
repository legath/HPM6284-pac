#[doc = "Register `ADCCLK[%s]` reader"]
pub type R = crate::R<ADCCLK_SPEC>;
#[doc = "Register `ADCCLK[%s]` writer"]
pub type W = crate::W<ADCCLK_SPEC>;
#[doc = "Field `MUX` reader - current mux 0: ana clock 1: ahb clock"]
pub type MUX_R = crate::BitReader;
#[doc = "Field `MUX` writer - current mux 0: ana clock 1: ahb clock"]
pub type MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESERVE` reader - preserve function against global select 0: select global clock setting 1: not select global clock setting"]
pub type PRESERVE_R = crate::BitReader;
#[doc = "Field `PRESERVE` writer - preserve function against global select 0: select global clock setting 1: not select global clock setting"]
pub type PRESERVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOC_BUSY` reader - local busy 0: a change is pending for current node 1: current node is changing status"]
pub type LOC_BUSY_R = crate::BitReader;
#[doc = "Field `GLB_BUSY` reader - global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
pub type GLB_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - current mux 0: ana clock 1: ahb clock"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 28 - preserve function against global select 0: select global clock setting 1: not select global clock setting"]
    #[inline(always)]
    pub fn preserve(&self) -> PRESERVE_R {
        PRESERVE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - local busy 0: a change is pending for current node 1: current node is changing status"]
    #[inline(always)]
    pub fn loc_busy(&self) -> LOC_BUSY_R {
        LOC_BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    #[inline(always)]
    pub fn glb_busy(&self) -> GLB_BUSY_R {
        GLB_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - current mux 0: ana clock 1: ahb clock"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<ADCCLK_SPEC> {
        MUX_W::new(self, 8)
    }
    #[doc = "Bit 28 - preserve function against global select 0: select global clock setting 1: not select global clock setting"]
    #[inline(always)]
    #[must_use]
    pub fn preserve(&mut self) -> PRESERVE_W<ADCCLK_SPEC> {
        PRESERVE_W::new(self, 28)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCCLK_SPEC;
impl crate::RegisterSpec for ADCCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcclk::R`](R) reader structure"]
impl crate::Readable for ADCCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcclk::W`](W) writer structure"]
impl crate::Writable for ADCCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCLK[%s]
to value 0"]
impl crate::Resettable for ADCCLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
