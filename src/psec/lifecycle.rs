#[doc = "Register `LIFECYCLE` reader"]
pub type R = crate::R<LIFECYCLE_SPEC>;
#[doc = "Register `LIFECYCLE` writer"]
pub type W = crate::W<LIFECYCLE_SPEC>;
#[doc = "Field `LIFECYCLE` reader - lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow"]
pub type LIFECYCLE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow"]
    #[inline(always)]
    pub fn lifecycle(&self) -> LIFECYCLE_R {
        LIFECYCLE_R::new((self.bits & 0xff) as u8)
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
#[doc = "Lifecycle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lifecycle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lifecycle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIFECYCLE_SPEC;
impl crate::RegisterSpec for LIFECYCLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lifecycle::R`](R) reader structure"]
impl crate::Readable for LIFECYCLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lifecycle::W`](W) writer structure"]
impl crate::Writable for LIFECYCLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIFECYCLE to value 0"]
impl crate::Resettable for LIFECYCLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
