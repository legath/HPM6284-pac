#[doc = "Register `wdgcfg` reader"]
pub type R = crate::R<WDGCFG_SPEC>;
#[doc = "Register `wdgcfg` writer"]
pub type W = crate::W<WDGCFG_SPEC>;
#[doc = "Field `WDGTO` reader - watch dog timeout value"]
pub type WDGTO_R = crate::FieldReader<u32>;
#[doc = "Field `WDGTO` writer - watch dog timeout value"]
pub type WDGTO_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `WDGEN` reader - 1- enable wdog counter"]
pub type WDGEN_R = crate::BitReader;
#[doc = "Field `WDGEN` writer - 1- enable wdog counter"]
pub type WDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - watch dog timeout value"]
    #[inline(always)]
    pub fn wdgto(&self) -> WDGTO_R {
        WDGTO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - 1- enable wdog counter"]
    #[inline(always)]
    pub fn wdgen(&self) -> WDGEN_R {
        WDGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - watch dog timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn wdgto(&mut self) -> WDGTO_W<WDGCFG_SPEC> {
        WDGTO_W::new(self, 0)
    }
    #[doc = "Bit 31 - 1- enable wdog counter"]
    #[inline(always)]
    #[must_use]
    pub fn wdgen(&mut self) -> WDGEN_W<WDGCFG_SPEC> {
        WDGEN_W::new(self, 31)
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
#[doc = "Watchdog configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdgcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdgcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDGCFG_SPEC;
impl crate::RegisterSpec for WDGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdgcfg::R`](R) reader structure"]
impl crate::Readable for WDGCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdgcfg::W`](W) writer structure"]
impl crate::Writable for WDGCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wdgcfg to value 0"]
impl crate::Resettable for WDGCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
