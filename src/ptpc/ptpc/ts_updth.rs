#[doc = "Register `ts_updth` reader"]
pub type R = crate::R<TS_UPDTH_SPEC>;
#[doc = "Register `ts_updth` writer"]
pub type W = crate::W<TS_UPDTH_SPEC>;
#[doc = "Field `SEC_UPDATE` reader - together with ts_updtl, used to initial or update timestamp"]
pub type SEC_UPDATE_R = crate::FieldReader<u32>;
#[doc = "Field `SEC_UPDATE` writer - together with ts_updtl, used to initial or update timestamp"]
pub type SEC_UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - together with ts_updtl, used to initial or update timestamp"]
    #[inline(always)]
    pub fn sec_update(&self) -> SEC_UPDATE_R {
        SEC_UPDATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - together with ts_updtl, used to initial or update timestamp"]
    #[inline(always)]
    #[must_use]
    pub fn sec_update(&mut self) -> SEC_UPDATE_W<TS_UPDTH_SPEC> {
        SEC_UPDATE_W::new(self, 0)
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
#[doc = "timestamp update high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts_updth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts_updth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TS_UPDTH_SPEC;
impl crate::RegisterSpec for TS_UPDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_updth::R`](R) reader structure"]
impl crate::Readable for TS_UPDTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ts_updth::W`](W) writer structure"]
impl crate::Writable for TS_UPDTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ts_updth to value 0"]
impl crate::Resettable for TS_UPDTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
