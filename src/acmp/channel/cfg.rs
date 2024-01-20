#[doc = "Register `cfg` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `cfg` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `FLTLEN` reader - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
pub type FLTLEN_R = crate::FieldReader<u16>;
#[doc = "Field `FLTLEN` writer - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
pub type FLTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SYNCEN` reader - This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
pub type SYNCEN_R = crate::BitReader;
#[doc = "Field `SYNCEN` writer - This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
pub type SYNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTMODE` reader - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
pub type FLTMODE_R = crate::FieldReader;
#[doc = "Field `FLTMODE` writer - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
pub type FLTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OPOL` reader - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
pub type OPOL_R = crate::BitReader;
#[doc = "Field `OPOL` writer - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
pub type OPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINEN` reader - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
pub type WINEN_R = crate::BitReader;
#[doc = "Field `WINEN` writer - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
pub type WINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTBYPS` reader - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
pub type FLTBYPS_R = crate::BitReader;
#[doc = "Field `FLTBYPS` writer - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
pub type FLTBYPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOEN` reader - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
pub type CMPOEN_R = crate::BitReader;
#[doc = "Field `CMPOEN` writer - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
pub type CMPOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINSEL` reader - MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
pub type PINSEL_R = crate::FieldReader;
#[doc = "Field `PINSEL` writer - MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
pub type PINSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MINSEL` reader - PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
pub type MINSEL_R = crate::FieldReader;
#[doc = "Field `MINSEL` writer - PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
pub type MINSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMPEN` reader - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
pub type CMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPMODE` reader - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
pub type HPMODE_R = crate::BitReader;
#[doc = "Field `HPMODE` writer - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
pub type HPMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACEN` reader - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
pub type DACEN_R = crate::BitReader;
#[doc = "Field `DACEN` writer - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
pub type DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
pub type HYST_R = crate::FieldReader;
#[doc = "Field `HYST` writer - This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
    #[inline(always)]
    pub fn fltlen(&self) -> FLTLEN_R {
        FLTLEN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
    #[inline(always)]
    pub fn fltmode(&self) -> FLTMODE_R {
        FLTMODE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
    #[inline(always)]
    pub fn opol(&self) -> OPOL_R {
        OPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
    #[inline(always)]
    pub fn winen(&self) -> WINEN_R {
        WINEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
    #[inline(always)]
    pub fn fltbyps(&self) -> FLTBYPS_R {
        FLTBYPS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
    #[inline(always)]
    pub fn cmpoen(&self) -> CMPOEN_R {
        CMPOEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
    #[inline(always)]
    pub fn minsel(&self) -> MINSEL_R {
        MINSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
    #[inline(always)]
    pub fn hpmode(&self) -> HPMODE_R {
        HPMODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn fltlen(&mut self) -> FLTLEN_W<CFG_SPEC> {
        FLTLEN_W::new(self, 0)
    }
    #[doc = "Bit 12 - This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<CFG_SPEC> {
        SYNCEN_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
    #[inline(always)]
    #[must_use]
    pub fn fltmode(&mut self) -> FLTMODE_W<CFG_SPEC> {
        FLTMODE_W::new(self, 13)
    }
    #[doc = "Bit 16 - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn opol(&mut self) -> OPOL_W<CFG_SPEC> {
        OPOL_W::new(self, 16)
    }
    #[doc = "Bit 17 - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn winen(&mut self) -> WINEN_W<CFG_SPEC> {
        WINEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
    #[inline(always)]
    #[must_use]
    pub fn fltbyps(&mut self) -> FLTBYPS_W<CFG_SPEC> {
        FLTBYPS_W::new(self, 18)
    }
    #[doc = "Bit 19 - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmpoen(&mut self) -> CMPOEN_W<CFG_SPEC> {
        CMPOEN_W::new(self, 19)
    }
    #[doc = "Bits 20:22 - MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
    #[inline(always)]
    #[must_use]
    pub fn pinsel(&mut self) -> PINSEL_W<CFG_SPEC> {
        PINSEL_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
    #[inline(always)]
    #[must_use]
    pub fn minsel(&mut self) -> MINSEL_W<CFG_SPEC> {
        MINSEL_W::new(self, 24)
    }
    #[doc = "Bit 27 - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<CFG_SPEC> {
        CMPEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn hpmode(&mut self) -> HPMODE_W<CFG_SPEC> {
        HPMODE_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<CFG_SPEC> {
        DACEN_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<CFG_SPEC> {
        HYST_W::new(self, 30)
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
#[doc = "Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cfg to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
