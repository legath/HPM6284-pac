#[doc = "Register `LLPointerH` reader"]
pub type R = crate::R<LLPOINTER_H_SPEC>;
#[doc = "Register `LLPointerH` writer"]
pub type W = crate::W<LLPOINTER_H_SPEC>;
#[doc = "Field `LLPOINTERH` reader - High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
pub type LLPOINTERH_R = crate::FieldReader<u32>;
#[doc = "Field `LLPOINTERH` writer - High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
pub type LLPOINTERH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    pub fn llpointerh(&self) -> LLPOINTERH_R {
        LLPOINTERH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn llpointerh(&mut self) -> LLPOINTERH_W<LLPOINTER_H_SPEC> {
        LLPOINTERH_W::new(self, 0)
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
#[doc = "Channel n Linked List Pointer High Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llpointer_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llpointer_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLPOINTER_H_SPEC;
impl crate::RegisterSpec for LLPOINTER_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llpointer_h::R`](R) reader structure"]
impl crate::Readable for LLPOINTER_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`llpointer_h::W`](W) writer structure"]
impl crate::Writable for LLPOINTER_H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LLPointerH to value 0"]
impl crate::Resettable for LLPOINTER_H_SPEC {
    const RESET_VALUE: u32 = 0;
}
