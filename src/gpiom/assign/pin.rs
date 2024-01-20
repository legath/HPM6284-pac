#[doc = "Register `PIN[%s]` reader"]
pub type R = crate::R<PIN_SPEC>;
#[doc = "Register `PIN[%s]` writer"]
pub type W = crate::W<PIN_SPEC>;
#[doc = "Field `SELECT` reader - select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio"]
pub type SELECT_R = crate::FieldReader;
#[doc = "Field `SELECT` writer - select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio"]
pub type SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HIDE` reader - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio"]
pub type HIDE_R = crate::FieldReader;
#[doc = "Field `HIDE` writer - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio"]
pub type HIDE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCK` reader - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio"]
    #[inline(always)]
    pub fn hide(&self) -> HIDE_R {
        HIDE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<PIN_SPEC> {
        SELECT_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio"]
    #[inline(always)]
    #[must_use]
    pub fn hide(&mut self) -> HIDE_W<PIN_SPEC> {
        HIDE_W::new(self, 8)
    }
    #[doc = "Bit 31 - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<PIN_SPEC> {
        LOCK_W::new(self, 31)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin::R`](R) reader structure"]
impl crate::Readable for PIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin::W`](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN[%s]
to value 0"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
