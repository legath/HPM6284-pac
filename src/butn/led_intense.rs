#[doc = "Register `LED_INTENSE` reader"]
pub type R = crate::R<LED_INTENSE_SPEC>;
#[doc = "Register `LED_INTENSE` writer"]
pub type W = crate::W<LED_INTENSE_SPEC>;
#[doc = "Field `PLED` reader - Pbutton brightness 0"]
pub type PLED_R = crate::FieldReader;
#[doc = "Field `PLED` writer - Pbutton brightness 0"]
pub type PLED_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RLED` reader - Rbutton brightness 0"]
pub type RLED_R = crate::FieldReader;
#[doc = "Field `RLED` writer - Rbutton brightness 0"]
pub type RLED_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Pbutton brightness 0"]
    #[inline(always)]
    pub fn pled(&self) -> PLED_R {
        PLED_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Rbutton brightness 0"]
    #[inline(always)]
    pub fn rled(&self) -> RLED_R {
        RLED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pbutton brightness 0"]
    #[inline(always)]
    #[must_use]
    pub fn pled(&mut self) -> PLED_W<LED_INTENSE_SPEC> {
        PLED_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Rbutton brightness 0"]
    #[inline(always)]
    #[must_use]
    pub fn rled(&mut self) -> RLED_W<LED_INTENSE_SPEC> {
        RLED_W::new(self, 16)
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
#[doc = "Debounce setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`led_intense::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`led_intense::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LED_INTENSE_SPEC;
impl crate::RegisterSpec for LED_INTENSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`led_intense::R`](R) reader structure"]
impl crate::Readable for LED_INTENSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`led_intense::W`](W) writer structure"]
impl crate::Writable for LED_INTENSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LED_INTENSE to value 0"]
impl crate::Resettable for LED_INTENSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
