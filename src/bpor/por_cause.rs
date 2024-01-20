#[doc = "Register `POR_CAUSE` reader"]
pub type R = crate::R<POR_CAUSE_SPEC>;
#[doc = "Register `POR_CAUSE` writer"]
pub type W = crate::W<POR_CAUSE_SPEC>;
#[doc = "Field `CAUSE` reader - Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
pub type CAUSE_R = crate::FieldReader;
#[doc = "Field `CAUSE` writer - Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
pub type CAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
    #[inline(always)]
    pub fn cause(&self) -> CAUSE_R {
        CAUSE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn cause(&mut self) -> CAUSE_W<POR_CAUSE_SPEC> {
        CAUSE_W::new(self, 0)
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
#[doc = "Power on cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`por_cause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`por_cause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POR_CAUSE_SPEC;
impl crate::RegisterSpec for POR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`por_cause::R`](R) reader structure"]
impl crate::Readable for POR_CAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`por_cause::W`](W) writer structure"]
impl crate::Writable for POR_CAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POR_CAUSE to value 0"]
impl crate::Resettable for POR_CAUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
