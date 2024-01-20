#[doc = "Register `cntcopy` reader"]
pub type R = crate::R<CNTCOPY_SPEC>;
#[doc = "Register `cntcopy` writer"]
pub type W = crate::W<CNTCOPY_SPEC>;
#[doc = "Field `CNT` reader - current clock counter value"]
pub type CNT_R = crate::FieldReader<u32>;
#[doc = "Field `XCNT` reader - current extended counter value"]
pub type XCNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:27 - current clock counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits >> 4) & 0x00ff_ffff)
    }
    #[doc = "Bits 28:31 - current extended counter value"]
    #[inline(always)]
    pub fn xcnt(&self) -> XCNT_R {
        XCNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
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
#[doc = "Counter copy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcopy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcopy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTCOPY_SPEC;
impl crate::RegisterSpec for CNTCOPY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntcopy::R`](R) reader structure"]
impl crate::Readable for CNTCOPY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntcopy::W`](W) writer structure"]
impl crate::Writable for CNTCOPY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cntcopy to value 0"]
impl crate::Resettable for CNTCOPY_SPEC {
    const RESET_VALUE: u32 = 0;
}
