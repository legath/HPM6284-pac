#[doc = "Register `TOGGLE` reader"]
pub type R = crate::R<TOGGLE_SPEC>;
#[doc = "Register `TOGGLE` writer"]
pub type W = crate::W<TOGGLE_SPEC>;
#[doc = "Field `LINK` reader - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before"]
pub type LINK_R = crate::FieldReader<u32>;
#[doc = "Field `LINK` writer - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before"]
pub type LINK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LINK_W<TOGGLE_SPEC> {
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
#[doc = "Group setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`toggle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`toggle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOGGLE_SPEC;
impl crate::RegisterSpec for TOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`toggle::R`](R) reader structure"]
impl crate::Readable for TOGGLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`toggle::W`](W) writer structure"]
impl crate::Writable for TOGGLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOGGLE to value 0"]
impl crate::Resettable for TOGGLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
