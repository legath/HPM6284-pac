#[doc = "Register `FUSE[%s]` reader"]
pub type R = crate::R<FUSE_SPEC>;
#[doc = "Register `FUSE[%s]` writer"]
pub type W = crate::W<FUSE_SPEC>;
#[doc = "Field `FUSE` reader - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)"]
pub type FUSE_R = crate::FieldReader<u32>;
#[doc = "Field `FUSE` writer - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)"]
pub type FUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)"]
    #[inline(always)]
    pub fn fuse(&self) -> FUSE_R {
        FUSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)"]
    #[inline(always)]
    #[must_use]
    pub fn fuse(&mut self) -> FUSE_W<FUSE_SPEC> {
        FUSE_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUSE_SPEC;
impl crate::RegisterSpec for FUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fuse::R`](R) reader structure"]
impl crate::Readable for FUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fuse::W`](W) writer structure"]
impl crate::Writable for FUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUSE[%s]
to value 0"]
impl crate::Resettable for FUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
