#[doc = "Register `ENDPTSETUPSTAT` reader"]
pub type R = crate::R<ENDPTSETUPSTAT_SPEC>;
#[doc = "Register `ENDPTSETUPSTAT` writer"]
pub type W = crate::W<ENDPTSETUPSTAT_SPEC>;
#[doc = "Field `ENDPTSETUPSTAT` reader - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode."]
pub type ENDPTSETUPSTAT_R = crate::FieldReader;
#[doc = "Field `ENDPTSETUPSTAT` writer - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode."]
pub type ENDPTSETUPSTAT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode."]
    #[inline(always)]
    pub fn endptsetupstat(&self) -> ENDPTSETUPSTAT_R {
        ENDPTSETUPSTAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ENDPTSETUPSTAT Setup Endpoint Status. For every setup transaction that is received, a corresponding bit in this register is set to one. Software must clear or acknowledge the setup transfer by writing a one to a respective bit after it has read the setup data from Queue head. The response to a setup packet as in the order of operations and total response time is crucial to limit bus time outs while the setup lock out mechanism is engaged. This register is only used in device mode."]
    #[inline(always)]
    #[must_use]
    pub fn endptsetupstat(&mut self) -> ENDPTSETUPSTAT_W<ENDPTSETUPSTAT_SPEC> {
        ENDPTSETUPSTAT_W::new(self, 0)
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
#[doc = "Endpoint Setup Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptsetupstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptsetupstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTSETUPSTAT_SPEC;
impl crate::RegisterSpec for ENDPTSETUPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptsetupstat::R`](R) reader structure"]
impl crate::Readable for ENDPTSETUPSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptsetupstat::W`](W) writer structure"]
impl crate::Writable for ENDPTSETUPSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTSETUPSTAT to value 0"]
impl crate::Resettable for ENDPTSETUPSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
