#[doc = "Register `DCDC_MODE` reader"]
pub type R = crate::R<DCDC_MODE_SPEC>;
#[doc = "Register `DCDC_MODE` writer"]
pub type W = crate::W<DCDC_MODE_SPEC>;
#[doc = "Field `VOLT` reader - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
pub type VOLT_R = crate::FieldReader<u16>;
#[doc = "Field `VOLT` writer - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
pub type VOLT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `MODE` reader - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `READY` reader - Ready flag 0: DCDC is applying new change 1: DCDC is ready"]
pub type READY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    #[inline(always)]
    pub fn volt(&self) -> VOLT_R {
        VOLT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 28 - Ready flag 0: DCDC is applying new change 1: DCDC is ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    #[inline(always)]
    #[must_use]
    pub fn volt(&mut self) -> VOLT_W<DCDC_MODE_SPEC> {
        VOLT_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<DCDC_MODE_SPEC> {
        MODE_W::new(self, 16)
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
#[doc = "DCDC mode select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_MODE_SPEC;
impl crate::RegisterSpec for DCDC_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_mode::R`](R) reader structure"]
impl crate::Readable for DCDC_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_mode::W`](W) writer structure"]
impl crate::Writable for DCDC_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDC_MODE to value 0x0001_047e"]
impl crate::Resettable for DCDC_MODE_SPEC {
    const RESET_VALUE: u32 = 0x0001_047e;
}
