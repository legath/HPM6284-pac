#[doc = "Register `capt_snaph` reader"]
pub type R = crate::R<CAPT_SNAPH_SPEC>;
#[doc = "Register `capt_snaph` writer"]
pub type W = crate::W<CAPT_SNAPH_SPEC>;
#[doc = "Field `CAPT_SNAP_HIGH` reader - take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8"]
pub type CAPT_SNAP_HIGH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8"]
    #[inline(always)]
    pub fn capt_snap_high(&self) -> CAPT_SNAP_HIGH_R {
        CAPT_SNAP_HIGH_R::new(self.bits)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capt_snaph::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capt_snaph::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPT_SNAPH_SPEC;
impl crate::RegisterSpec for CAPT_SNAPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capt_snaph::R`](R) reader structure"]
impl crate::Readable for CAPT_SNAPH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`capt_snaph::W`](W) writer structure"]
impl crate::Writable for CAPT_SNAPH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets capt_snaph to value 0"]
impl crate::Resettable for CAPT_SNAPH_SPEC {
    const RESET_VALUE: u32 = 0;
}
