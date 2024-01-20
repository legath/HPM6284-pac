#[doc = "Register `TOCV` reader"]
pub type R = crate::R<TOCV_SPEC>;
#[doc = "Register `TOCV` writer"]
pub type W = crate::W<TOCV_SPEC>;
#[doc = "Field `TOC` reader - Timeout Counter The Timeout Counter is decremented in multiples of CAN bit times \\[1…16\\]
depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS. Note: Byte access: when TOCC.TOS = “00，writing one of the register bytes 3/2/1/0 will preset the Timeout Counter."]
pub type TOC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout Counter The Timeout Counter is decremented in multiples of CAN bit times \\[1…16\\]
depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS. Note: Byte access: when TOCC.TOS = “00，writing one of the register bytes 3/2/1/0 will preset the Timeout Counter."]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
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
#[doc = "timeout counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCV_SPEC;
impl crate::RegisterSpec for TOCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocv::R`](R) reader structure"]
impl crate::Readable for TOCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tocv::W`](W) writer structure"]
impl crate::Writable for TOCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCV to value 0xffff"]
impl crate::Resettable for TOCV_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
