#[doc = "Register `ASYNCLISTADDR` reader"]
pub type R = crate::R<ASYNCLISTADDR_SPEC>;
#[doc = "Register `ASYNCLISTADDR` writer"]
pub type W = crate::W<ASYNCLISTADDR_SPEC>;
#[doc = "Field `ASYBASE` reader - ASYBASE Link Pointer Low (LPL). These bits correspond to memory address signals \\[31:5\\], respectively. This field may only reference a Queue Head (QH). Only used by the host controller."]
pub type ASYBASE_R = crate::FieldReader<u32>;
#[doc = "Field `ASYBASE` writer - ASYBASE Link Pointer Low (LPL). These bits correspond to memory address signals \\[31:5\\], respectively. This field may only reference a Queue Head (QH). Only used by the host controller."]
pub type ASYBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - ASYBASE Link Pointer Low (LPL). These bits correspond to memory address signals \\[31:5\\], respectively. This field may only reference a Queue Head (QH). Only used by the host controller."]
    #[inline(always)]
    pub fn asybase(&self) -> ASYBASE_R {
        ASYBASE_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - ASYBASE Link Pointer Low (LPL). These bits correspond to memory address signals \\[31:5\\], respectively. This field may only reference a Queue Head (QH). Only used by the host controller."]
    #[inline(always)]
    #[must_use]
    pub fn asybase(&mut self) -> ASYBASE_W<ASYNCLISTADDR_SPEC> {
        ASYBASE_W::new(self, 5)
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
#[doc = "Next Asynch. Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asynclistaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asynclistaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNCLISTADDR_SPEC;
impl crate::RegisterSpec for ASYNCLISTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asynclistaddr::R`](R) reader structure"]
impl crate::Readable for ASYNCLISTADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asynclistaddr::W`](W) writer structure"]
impl crate::Writable for ASYNCLISTADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASYNCLISTADDR to value 0"]
impl crate::Resettable for ASYNCLISTADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
