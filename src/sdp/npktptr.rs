#[doc = "Register `NPKTPTR` reader"]
pub type R = crate::R<NPKTPTR_SPEC>;
#[doc = "Register `NPKTPTR` writer"]
pub type W = crate::W<NPKTPTR_SPEC>;
#[doc = "Field `NPKTPTR` reader - Next Packet Address Pointer"]
pub type NPKTPTR_R = crate::FieldReader<u32>;
#[doc = "Field `NPKTPTR` writer - Next Packet Address Pointer"]
pub type NPKTPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Next Packet Address Pointer"]
    #[inline(always)]
    pub fn npktptr(&self) -> NPKTPTR_R {
        NPKTPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Packet Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn npktptr(&mut self) -> NPKTPTR_W<NPKTPTR_SPEC> {
        NPKTPTR_W::new(self, 0)
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
#[doc = "Next Packet Address Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npktptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npktptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NPKTPTR_SPEC;
impl crate::RegisterSpec for NPKTPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npktptr::R`](R) reader structure"]
impl crate::Readable for NPKTPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`npktptr::W`](W) writer structure"]
impl crate::Writable for NPKTPTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NPKTPTR to value 0"]
impl crate::Resettable for NPKTPTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
