#[doc = "Register `LDO2P5` reader"]
pub type R = crate::R<LDO2P5_SPEC>;
#[doc = "Register `LDO2P5` writer"]
pub type W = crate::W<LDO2P5_SPEC>;
#[doc = "Field `VOLT` reader - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
pub type VOLT_R = crate::FieldReader<u16>;
#[doc = "Field `VOLT` writer - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
pub type VOLT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ENABLE` reader - LDO enable 0: turn off LDO 1: turn on LDO"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - LDO enable 0: turn off LDO 1: turn on LDO"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY` reader - Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready"]
pub type READY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
    #[inline(always)]
    pub fn volt(&self) -> VOLT_R {
        VOLT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
    #[inline(always)]
    #[must_use]
    pub fn volt(&mut self) -> VOLT_W<LDO2P5_SPEC> {
        VOLT_W::new(self, 0)
    }
    #[doc = "Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<LDO2P5_SPEC> {
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
#[doc = "2.5V LDO config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo2p5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo2p5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDO2P5_SPEC;
impl crate::RegisterSpec for LDO2P5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo2p5::R`](R) reader structure"]
impl crate::Readable for LDO2P5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldo2p5::W`](W) writer structure"]
impl crate::Writable for LDO2P5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LDO2P5 to value 0x09c4"]
impl crate::Resettable for LDO2P5_SPEC {
    const RESET_VALUE: u32 = 0x09c4;
}
