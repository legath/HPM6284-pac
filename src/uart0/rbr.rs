#[doc = "Register `RBR` reader"]
pub type R = crate::R<RBR_SPEC>;
#[doc = "Register `RBR` writer"]
pub type W = crate::W<RBR_SPEC>;
#[doc = "Field `RBR` reader - Receive data read port"]
pub type RBR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data read port"]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 0xff) as u8)
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
#[doc = "Receiver Buffer Register (when DLAB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBR_SPEC;
impl crate::RegisterSpec for RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr::R`](R) reader structure"]
impl crate::Readable for RBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rbr::W`](W) writer structure"]
impl crate::Writable for RBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBR to value 0"]
impl crate::Resettable for RBR_SPEC {
    const RESET_VALUE: u32 = 0;
}
