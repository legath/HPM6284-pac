#[doc = "Register `IntStatus` reader"]
pub type R = crate::R<INT_STATUS_SPEC>;
#[doc = "Register `IntStatus` writer"]
pub type W = crate::W<INT_STATUS_SPEC>;
#[doc = "Field `ERROR` writer - The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status"]
pub type ERROR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ABORT` writer - The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status"]
pub type ABORT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TC` writer - The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status"]
pub type TC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<INT_STATUS_SPEC> {
        ERROR_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<INT_STATUS_SPEC> {
        ABORT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<INT_STATUS_SPEC> {
        TC_W::new(self, 16)
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
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_status::W`](W) writer structure"]
impl crate::Writable for INT_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IntStatus to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
