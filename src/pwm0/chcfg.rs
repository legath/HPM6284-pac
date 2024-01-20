#[doc = "Register `CHCFG[%s]` reader"]
pub type R = crate::R<CHCFG_SPEC>;
#[doc = "Register `CHCFG[%s]` writer"]
pub type W = crate::W<CHCFG_SPEC>;
#[doc = "Field `OUTPOL` reader - output polarity, set to 1 will invert the output"]
pub type OUTPOL_R = crate::BitReader;
#[doc = "Field `OUTPOL` writer - output polarity, set to 1 will invert the output"]
pub type OUTPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSELBEG` reader - assign the first comparator for this output channel"]
pub type CMPSELBEG_R = crate::FieldReader;
#[doc = "Field `CMPSELBEG` writer - assign the first comparator for this output channel"]
pub type CMPSELBEG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CMPSELEND` reader - assign the last comparator for this output channel"]
pub type CMPSELEND_R = crate::FieldReader;
#[doc = "Field `CMPSELEND` writer - assign the last comparator for this output channel"]
pub type CMPSELEND_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 1 - output polarity, set to 1 will invert the output"]
    #[inline(always)]
    pub fn outpol(&self) -> OUTPOL_R {
        OUTPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:20 - assign the first comparator for this output channel"]
    #[inline(always)]
    pub fn cmpselbeg(&self) -> CMPSELBEG_R {
        CMPSELBEG_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - assign the last comparator for this output channel"]
    #[inline(always)]
    pub fn cmpselend(&self) -> CMPSELEND_R {
        CMPSELEND_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - output polarity, set to 1 will invert the output"]
    #[inline(always)]
    #[must_use]
    pub fn outpol(&mut self) -> OUTPOL_W<CHCFG_SPEC> {
        OUTPOL_W::new(self, 1)
    }
    #[doc = "Bits 16:20 - assign the first comparator for this output channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpselbeg(&mut self) -> CMPSELBEG_W<CHCFG_SPEC> {
        CMPSELBEG_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - assign the last comparator for this output channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpselend(&mut self) -> CMPSELEND_W<CHCFG_SPEC> {
        CMPSELEND_W::new(self, 24)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCFG_SPEC;
impl crate::RegisterSpec for CHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chcfg::R`](R) reader structure"]
impl crate::Readable for CHCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chcfg::W`](W) writer structure"]
impl crate::Writable for CHCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHCFG[%s]
to value 0"]
impl crate::Resettable for CHCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
