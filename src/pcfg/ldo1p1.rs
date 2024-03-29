#[doc = "Register `LDO1P1` reader"]
pub type R = crate::R<LDO1P1_SPEC>;
#[doc = "Register `LDO1P1` writer"]
pub type W = crate::W<LDO1P1_SPEC>;
#[doc = "Field `VOLT` reader - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
pub type VOLT_R = crate::FieldReader<u16>;
#[doc = "Field `VOLT` writer - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
pub type VOLT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ENABLE` reader - LDO enable 0: turn off LDO 1: turn on LDO"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - LDO enable 0: turn off LDO 1: turn on LDO"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
    #[inline(always)]
    pub fn volt(&self) -> VOLT_R {
        VOLT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
    #[inline(always)]
    #[must_use]
    pub fn volt(&mut self) -> VOLT_W<LDO1P1_SPEC> {
        VOLT_W::new(self, 0)
    }
    #[doc = "Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<LDO1P1_SPEC> {
        ENABLE_W::new(self, 16)
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
#[doc = "1V LDO config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo1p1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo1p1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDO1P1_SPEC;
impl crate::RegisterSpec for LDO1P1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo1p1::R`](R) reader structure"]
impl crate::Readable for LDO1P1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldo1p1::W`](W) writer structure"]
impl crate::Writable for LDO1P1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDO1P1 to value 0x0001_044c"]
impl crate::Resettable for LDO1P1_SPEC {
    const RESET_VALUE: u32 = 0x0001_044c;
}
