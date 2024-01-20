#[doc = "Register `z` reader"]
pub type R = crate::R<Z_SPEC>;
#[doc = "Register `z` writer"]
pub type W = crate::W<Z_SPEC>;
#[doc = "Field `ZCNT` reader - zcnt value"]
pub type ZCNT_R = crate::FieldReader<u32>;
#[doc = "Field `ZCNT` writer - zcnt value"]
pub type ZCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - zcnt value"]
    #[inline(always)]
    pub fn zcnt(&self) -> ZCNT_R {
        ZCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - zcnt value"]
    #[inline(always)]
    #[must_use]
    pub fn zcnt(&mut self) -> ZCNT_W<Z_SPEC> {
        ZCNT_W::new(self, 0)
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
#[doc = "Z counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`z::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Z_SPEC;
impl crate::RegisterSpec for Z_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`z::R`](R) reader structure"]
impl crate::Readable for Z_SPEC {}
#[doc = "`write(|w| ..)` method takes [`z::W`](W) writer structure"]
impl crate::Writable for Z_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets z to value 0"]
impl crate::Resettable for Z_SPEC {
    const RESET_VALUE: u32 = 0;
}
