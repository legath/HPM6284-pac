#[doc = "Register `capt_snapl` reader"]
pub type R = crate::R<CAPT_SNAPL_SPEC>;
#[doc = "Register `capt_snapl` writer"]
pub type W = crate::W<CAPT_SNAPL_SPEC>;
#[doc = "Field `CAPT_SNAP_LOW` reader - No description avaiable"]
pub type CAPT_SNAP_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `CAPT_SNAP_LOW` writer - No description avaiable"]
pub type CAPT_SNAP_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn capt_snap_low(&self) -> CAPT_SNAP_LOW_R {
        CAPT_SNAP_LOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn capt_snap_low(&mut self) -> CAPT_SNAP_LOW_W<CAPT_SNAPL_SPEC> {
        CAPT_SNAP_LOW_W::new(self, 0)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capt_snapl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capt_snapl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPT_SNAPL_SPEC;
impl crate::RegisterSpec for CAPT_SNAPL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capt_snapl::R`](R) reader structure"]
impl crate::Readable for CAPT_SNAPL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`capt_snapl::W`](W) writer structure"]
impl crate::Writable for CAPT_SNAPL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets capt_snapl to value 0"]
impl crate::Resettable for CAPT_SNAPL_SPEC {
    const RESET_VALUE: u32 = 0;
}
