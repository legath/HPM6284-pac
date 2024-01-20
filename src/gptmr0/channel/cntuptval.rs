#[doc = "Register `CNTUPTVAL` reader"]
pub type R = crate::R<CNTUPTVAL_SPEC>;
#[doc = "Register `CNTUPTVAL` writer"]
pub type W = crate::W<CNTUPTVAL_SPEC>;
#[doc = "Field `CNTUPTVAL` reader - counter will be set to this value when software write cntupt bit in CR"]
pub type CNTUPTVAL_R = crate::FieldReader<u32>;
#[doc = "Field `CNTUPTVAL` writer - counter will be set to this value when software write cntupt bit in CR"]
pub type CNTUPTVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - counter will be set to this value when software write cntupt bit in CR"]
    #[inline(always)]
    pub fn cntuptval(&self) -> CNTUPTVAL_R {
        CNTUPTVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - counter will be set to this value when software write cntupt bit in CR"]
    #[inline(always)]
    #[must_use]
    pub fn cntuptval(&mut self) -> CNTUPTVAL_W<CNTUPTVAL_SPEC> {
        CNTUPTVAL_W::new(self, 0)
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
#[doc = "Counter update value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntuptval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntuptval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTUPTVAL_SPEC;
impl crate::RegisterSpec for CNTUPTVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntuptval::R`](R) reader structure"]
impl crate::Readable for CNTUPTVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntuptval::W`](W) writer structure"]
impl crate::Writable for CNTUPTVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTUPTVAL to value 0"]
impl crate::Resettable for CNTUPTVAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
