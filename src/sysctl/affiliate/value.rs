#[doc = "Register `VALUE` reader"]
pub type R = crate::R<VALUE_SPEC>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<VALUE_SPEC>;
#[doc = "Field `LINK` reader - Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3"]
pub type LINK_R = crate::FieldReader;
#[doc = "Field `LINK` writer - Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3"]
pub type LINK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LINK_W<VALUE_SPEC> {
        LINK_W::new(self, 0)
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
#[doc = "Affiliate of Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VALUE_SPEC;
impl crate::RegisterSpec for VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for VALUE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
