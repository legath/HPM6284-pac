#[doc = "Register `phcfg` reader"]
pub type R = crate::R<PHCFG_SPEC>;
#[doc = "Register `phcfg` writer"]
pub type W = crate::W<PHCFG_SPEC>;
#[doc = "Field `DLYCNT` reader - delay clock cycles number"]
pub type DLYCNT_R = crate::FieldReader<u32>;
#[doc = "Field `DLYCNT` writer - delay clock cycles number"]
pub type DLYCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `DLYSEL` reader - This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
pub type DLYSEL_R = crate::BitReader;
#[doc = "Field `DLYSEL` writer - This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
pub type DLYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - delay clock cycles number"]
    #[inline(always)]
    pub fn dlycnt(&self) -> DLYCNT_R {
        DLYCNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
    #[inline(always)]
    pub fn dlysel(&self) -> DLYSEL_R {
        DLYSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - delay clock cycles number"]
    #[inline(always)]
    #[must_use]
    pub fn dlycnt(&mut self) -> DLYCNT_W<PHCFG_SPEC> {
        DLYCNT_W::new(self, 0)
    }
    #[doc = "Bit 31 - This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
    #[inline(always)]
    #[must_use]
    pub fn dlysel(&mut self) -> DLYSEL_W<PHCFG_SPEC> {
        DLYSEL_W::new(self, 31)
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
