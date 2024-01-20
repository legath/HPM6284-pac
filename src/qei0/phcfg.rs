#[doc = "Register `phcfg` reader"]
pub type R = crate::R<PHCFG_SPEC>;
#[doc = "Register `phcfg` writer"]
pub type W = crate::W<PHCFG_SPEC>;
#[doc = "Field `PHMAX` reader - maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
pub type PHMAX_R = crate::FieldReader<u32>;
#[doc = "Field `PHMAX` writer - maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
pub type PHMAX_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `PHCALIZ` reader - 1- phcnt will set to phidx when Z input assert"]
pub type PHCALIZ_R = crate::BitReader;
#[doc = "Field `PHCALIZ` writer - 1- phcnt will set to phidx when Z input assert"]
pub type PHCALIZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCNTCFG` reader - 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
pub type ZCNTCFG_R = crate::BitReader;
#[doc = "Field `ZCNTCFG` writer - 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
pub type ZCNTCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:20 - maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
    #[inline(always)]
    pub fn phmax(&self) -> PHMAX_R {
        PHMAX_R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bit 21 - 1- phcnt will set to phidx when Z input assert"]
    #[inline(always)]
    pub fn phcaliz(&self) -> PHCALIZ_R {
        PHCALIZ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
    #[inline(always)]
    pub fn zcntcfg(&self) -> ZCNTCFG_R {
        ZCNTCFG_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:20 - maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
    #[inline(always)]
    #[must_use]
    pub fn phmax(&mut self) -> PHMAX_W<PHCFG_SPEC> {
        PHMAX_W::new(self, 0)
    }
    #[doc = "Bit 21 - 1- phcnt will set to phidx when Z input assert"]
    #[inline(always)]
    #[must_use]
    pub fn phcaliz(&mut self) -> PHCALIZ_W<PHCFG_SPEC> {
        PHCALIZ_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
    #[inline(always)]
    #[must_use]
    pub fn zcntcfg(&mut self) -> ZCNTCFG_W<PHCFG_SPEC> {
        ZCNTCFG_W::new(self, 22)
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
#[doc = "Phase configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHCFG_SPEC;
impl crate::RegisterSpec for PHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phcfg::R`](R) reader structure"]
impl crate::Readable for PHCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phcfg::W`](W) writer structure"]
impl crate::Writable for PHCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets phcfg to value 0"]
impl crate::Resettable for PHCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
