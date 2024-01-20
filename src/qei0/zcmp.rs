#[doc = "Register `zcmp` reader"]
pub type R = crate::R<ZCMP_SPEC>;
#[doc = "Register `zcmp` writer"]
pub type W = crate::W<ZCMP_SPEC>;
#[doc = "Field `ZCMP` reader - zcnt postion compare value"]
pub type ZCMP_R = crate::FieldReader<u32>;
#[doc = "Field `ZCMP` writer - zcnt postion compare value"]
pub type ZCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - zcnt postion compare value"]
    #[inline(always)]
    pub fn zcmp(&self) -> ZCMP_R {
        ZCMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - zcnt postion compare value"]
    #[inline(always)]
    #[must_use]
    pub fn zcmp(&mut self) -> ZCMP_W<ZCMP_SPEC> {
        ZCMP_W::new(self, 0)
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
#[doc = "Z comparator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zcmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zcmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZCMP_SPEC;
impl crate::RegisterSpec for ZCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zcmp::R`](R) reader structure"]
impl crate::Readable for ZCMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`zcmp::W`](W) writer structure"]
impl crate::Writable for ZCMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets zcmp to value 0"]
impl crate::Resettable for ZCMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
