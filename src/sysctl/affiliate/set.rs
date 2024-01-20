#[doc = "Register `SET` reader"]
pub type R = crate::R<SET_SPEC>;
#[doc = "Register `SET` writer"]
pub type W = crate::W<SET_SPEC>;
#[doc = "Field `LINK` reader - Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0"]
pub type LINK_R = crate::FieldReader;
#[doc = "Field `LINK` writer - Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0"]
pub type LINK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LINK_W<SET_SPEC> {
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
#[doc = "Affiliate of Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_SPEC;
impl crate::RegisterSpec for SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`set::R`](R) reader structure"]
impl crate::Readable for SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`set::W`](W) writer structure"]
impl crate::Writable for SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SET to value 0"]
impl crate::Resettable for SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
