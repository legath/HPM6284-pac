#[doc = "Register `LLPointer` reader"]
pub type R = crate::R<LLPOINTER_SPEC>;
#[doc = "Register `LLPointer` writer"]
pub type W = crate::W<LLPOINTER_SPEC>;
#[doc = "Field `LLDBUSINFIDX` reader - Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
pub type LLDBUSINFIDX_R = crate::BitReader;
#[doc = "Field `LLDBUSINFIDX` writer - Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
pub type LLDBUSINFIDX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLPOINTERL` reader - Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
pub type LLPOINTERL_R = crate::FieldReader<u32>;
#[doc = "Field `LLPOINTERL` writer - Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
pub type LLPOINTERL_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    #[inline(always)]
    pub fn lldbusinfidx(&self) -> LLDBUSINFIDX_R {
        LLDBUSINFIDX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:31 - Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    #[inline(always)]
    pub fn llpointerl(&self) -> LLPOINTERL_R {
        LLPOINTERL_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    #[inline(always)]
    #[must_use]
    pub fn lldbusinfidx(&mut self) -> LLDBUSINFIDX_W<LLPOINTER_SPEC> {
        LLDBUSINFIDX_W::new(self, 0)
    }
    #[doc = "Bits 3:31 - Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    #[inline(always)]
    #[must_use]
    pub fn llpointerl(&mut self) -> LLPOINTERL_W<LLPOINTER_SPEC> {
        LLPOINTERL_W::new(self, 3)
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
#[doc = "Channel n Linked List Pointer Low Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llpointer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llpointer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLPOINTER_SPEC;
impl crate::RegisterSpec for LLPOINTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llpointer::R`](R) reader structure"]
impl crate::Readable for LLPOINTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`llpointer::W`](W) writer structure"]
impl crate::Writable for LLPOINTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LLPointer to value 0"]
impl crate::Resettable for LLPOINTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
