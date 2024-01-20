#[doc = "Register `RLD` reader"]
pub type R = crate::R<RLD_SPEC>;
#[doc = "Register `RLD` writer"]
pub type W = crate::W<RLD_SPEC>;
#[doc = "Field `RLD` reader - reload value"]
pub type RLD_R = crate::FieldReader<u32>;
#[doc = "Field `RLD` writer - reload value"]
pub type RLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reload value"]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reload value"]
    #[inline(always)]
    #[must_use]
    pub fn rld(&mut self) -> RLD_W<RLD_SPEC> {
        RLD_W::new(self, 0)
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
#[doc = "Reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLD_SPEC;
impl crate::RegisterSpec for RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rld::R`](R) reader structure"]
impl crate::Readable for RLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rld::W`](W) writer structure"]
impl crate::Writable for RLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLD to value 0xffff_ffff"]
impl crate::Resettable for RLD_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
