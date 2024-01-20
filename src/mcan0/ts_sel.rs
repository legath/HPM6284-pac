#[doc = "Register `TS_SEL[%s]` reader"]
pub type R = crate::R<TS_SEL_SPEC>;
#[doc = "Register `TS_SEL[%s]` writer"]
pub type W = crate::W<TS_SEL_SPEC>;
#[doc = "Field `TS` reader - Timestamp Word TS default can save 16 timestamps with 32bit; if ts64_en is set, then work at 64bit mode, can save 8 timestamps with 01/23/45…."]
pub type TS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Word TS default can save 16 timestamps with 32bit; if ts64_en is set, then work at 64bit mode, can save 8 timestamps with 01/23/45…."]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(self.bits)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TS_SEL_SPEC;
impl crate::RegisterSpec for TS_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_sel::R`](R) reader structure"]
impl crate::Readable for TS_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ts_sel::W`](W) writer structure"]
impl crate::Writable for TS_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_SEL[%s]
to value 0"]
impl crate::Resettable for TS_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
