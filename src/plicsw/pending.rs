#[doc = "Register `PENDING` reader"]
pub type R = crate::R<PENDING_SPEC>;
#[doc = "Register `PENDING` writer"]
pub type W = crate::W<PENDING_SPEC>;
#[doc = "Field `INTERRUPT` reader - writing 1 to trigger software interrupt"]
pub type INTERRUPT_R = crate::BitReader;
#[doc = "Field `INTERRUPT` writer - writing 1 to trigger software interrupt"]
pub type INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - writing 1 to trigger software interrupt"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - writing 1 to trigger software interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt(&mut self) -> INTERRUPT_W<PENDING_SPEC> {
        INTERRUPT_W::new(self, 1)
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
#[doc = "Pending status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PENDING_SPEC;
impl crate::RegisterSpec for PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending::R`](R) reader structure"]
impl crate::Readable for PENDING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pending::W`](W) writer structure"]
impl crate::Writable for PENDING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PENDING to value 0"]
impl crate::Resettable for PENDING_SPEC {
    const RESET_VALUE: u32 = 0;
}
