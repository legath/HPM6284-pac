#[doc = "Register `EVENT` reader"]
pub type R = crate::R<EVENT_SPEC>;
#[doc = "Register `EVENT` writer"]
pub type W = crate::W<EVENT_SPEC>;
#[doc = "Field `BATT_ESC_SEC` reader - BATT is escalting ssecure event"]
pub type BATT_ESC_SEC_R = crate::BitReader;
#[doc = "Field `BATT_ESC_NSC` reader - BATT is escalating non-secure event"]
pub type BATT_ESC_NSC_R = crate::BitReader;
#[doc = "Field `EVENT` reader - local event statue, each bit represents one security event"]
pub type EVENT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - BATT is escalting ssecure event"]
    #[inline(always)]
    pub fn batt_esc_sec(&self) -> BATT_ESC_SEC_R {
        BATT_ESC_SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BATT is escalating non-secure event"]
    #[inline(always)]
    pub fn batt_esc_nsc(&self) -> BATT_ESC_NSC_R {
        BATT_ESC_NSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:31 - local event statue, each bit represents one security event"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "Event and escalate status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVENT_SPEC;
impl crate::RegisterSpec for EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event::R`](R) reader structure"]
impl crate::Readable for EVENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`event::W`](W) writer structure"]
impl crate::Writable for EVENT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENT to value 0"]
impl crate::Resettable for EVENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
