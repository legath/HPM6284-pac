#[doc = "Register `TAMP_FLAG` reader"]
pub type R = crate::R<TAMP_FLAG_SPEC>;
#[doc = "Register `TAMP_FLAG` writer"]
pub type W = crate::W<TAMP_FLAG_SPEC>;
#[doc = "Field `FLAG` reader - tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
pub type FLAG_R = crate::FieldReader<u16>;
#[doc = "Field `FLAG` writer - tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
pub type FLAG_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FLAG_W<TAMP_FLAG_SPEC> {
        FLAG_W::new(self, 0)
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
#[doc = "Tamper flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_flag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_flag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_FLAG_SPEC;
impl crate::RegisterSpec for TAMP_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_flag::R`](R) reader structure"]
impl crate::Readable for TAMP_FLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamp_flag::W`](W) writer structure"]
impl crate::Writable for TAMP_FLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_FLAG to value 0"]
impl crate::Resettable for TAMP_FLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
