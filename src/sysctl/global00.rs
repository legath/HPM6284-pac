#[doc = "Register `global00` reader"]
pub type R = crate::R<GLOBAL00_SPEC>;
#[doc = "Register `global00` writer"]
pub type W = crate::W<GLOBAL00_SPEC>;
#[doc = "Field `MUX` reader - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
pub type MUX_R = crate::FieldReader;
#[doc = "Field `MUX` writer - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
pub type MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<GLOBAL00_SPEC> {
        MUX_W::new(self, 0)
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
#[doc = "Clock senario\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`global00::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`global00::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBAL00_SPEC;
impl crate::RegisterSpec for GLOBAL00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`global00::R`](R) reader structure"]
impl crate::Readable for GLOBAL00_SPEC {}
#[doc = "`write(|w| ..)` method takes [`global00::W`](W) writer structure"]
impl crate::Writable for GLOBAL00_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets global00 to value 0"]
impl crate::Resettable for GLOBAL00_SPEC {
    const RESET_VALUE: u32 = 0;
}
