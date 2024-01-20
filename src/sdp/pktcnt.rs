#[doc = "Register `PKTCNT` reader"]
pub type R = crate::R<PKTCNT_SPEC>;
#[doc = "Register `PKTCNT` writer"]
pub type W = crate::W<PKTCNT_SPEC>;
#[doc = "Field `CNTINCR` reader - The value written to this field is added to the spacket count."]
pub type CNTINCR_R = crate::FieldReader;
#[doc = "Field `CNTINCR` writer - The value written to this field is added to the spacket count."]
pub type CNTINCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CNTVAL` reader - This read-only field shows the current (instantaneous) value of the packet counter"]
pub type CNTVAL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The value written to this field is added to the spacket count."]
    #[inline(always)]
    pub fn cntincr(&self) -> CNTINCR_R {
        CNTINCR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This read-only field shows the current (instantaneous) value of the packet counter"]
    #[inline(always)]
    pub fn cntval(&self) -> CNTVAL_R {
        CNTVAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value written to this field is added to the spacket count."]
    #[inline(always)]
    #[must_use]
    pub fn cntincr(&mut self) -> CNTINCR_W<PKTCNT_SPEC> {
        CNTINCR_W::new(self, 0)
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
#[doc = "packet counter registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKTCNT_SPEC;
impl crate::RegisterSpec for PKTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktcnt::R`](R) reader structure"]
impl crate::Readable for PKTCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pktcnt::W`](W) writer structure"]
impl crate::Writable for PKTCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTCNT to value 0"]
impl crate::Resettable for PKTCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
