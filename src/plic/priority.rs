#[doc = "Register `PRIORITY[%s]` reader"]
pub type R = crate::R<PRIORITY_SPEC>;
#[doc = "Register `PRIORITY[%s]` writer"]
pub type W = crate::W<PRIORITY_SPEC>;
#[doc = "Field `PRIORITY` reader - Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
pub type PRIORITY_R = crate::FieldReader<u32>;
#[doc = "Field `PRIORITY` writer - Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<PRIORITY_SPEC> {
        PRIORITY_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIORITY_SPEC;
impl crate::RegisterSpec for PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority::R`](R) reader structure"]
impl crate::Readable for PRIORITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`priority::W`](W) writer structure"]
impl crate::Writable for PRIORITY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY[%s]
to value 0x01"]
impl crate::Resettable for PRIORITY_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
