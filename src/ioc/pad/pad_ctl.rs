#[doc = "Register `PAD_CTL` reader"]
pub type R = crate::R<PAD_CTL_SPEC>;
#[doc = "Register `PAD_CTL` writer"]
pub type W = crate::W<PAD_CTL_SPEC>;
#[doc = "Field `DS` reader - drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm"]
pub type DS_R = crate::FieldReader;
#[doc = "Field `DS` writer - drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm"]
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPD` reader - additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)"]
pub type SPD_R = crate::FieldReader;
#[doc = "Field `SPD` writer - additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)"]
pub type SPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SR` reader - slew rate 0: Slow slew rate 1: Fast slew rate"]
pub type SR_R = crate::BitReader;
#[doc = "Field `SR` writer - slew rate 0: Slow slew rate 1: Fast slew rate"]
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD` reader - open drain 0: open drain disable 1: open drain enable"]
pub type OD_R = crate::BitReader;
#[doc = "Field `OD` writer - open drain 0: open drain disable 1: open drain enable"]
pub type OD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KE` reader - keeper capability enable 0: keeper disable 1: keeper enable"]
pub type KE_R = crate::BitReader;
#[doc = "Field `KE` writer - keeper capability enable 0: keeper disable 1: keeper enable"]
pub type KE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - pull enable 0: pull disable 1: pull enable"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - pull enable 0: pull disable 1: pull enable"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - pull select 0: pull down 1: pull up"]
pub type PS_R = crate::BitReader;
#[doc = "Field `PS` writer - pull select 0: pull down 1: pull up"]
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm"]
pub type PRS_R = crate::FieldReader;
#[doc = "Field `PRS` writer - select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm"]
pub type PRS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HYS` reader - schmitt trigger enable 0: disable 1: enable"]
pub type HYS_R = crate::BitReader;
#[doc = "Field `HYS` writer - schmitt trigger enable 0: disable 1: enable"]
pub type HYS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - slew rate 0: Slow slew rate 1: Fast slew rate"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - open drain 0: open drain disable 1: open drain enable"]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - keeper capability enable 0: keeper disable 1: keeper enable"]
    #[inline(always)]
    pub fn ke(&self) -> KE_R {
        KE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - pull enable 0: pull disable 1: pull enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - pull select 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - schmitt trigger enable 0: disable 1: enable"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<PAD_CTL_SPEC> {
        DS_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SPD_W<PAD_CTL_SPEC> {
        SPD_W::new(self, 4)
    }
    #[doc = "Bit 6 - slew rate 0: Slow slew rate 1: Fast slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<PAD_CTL_SPEC> {
        SR_W::new(self, 6)
    }
    #[doc = "Bit 8 - open drain 0: open drain disable 1: open drain enable"]
    #[inline(always)]
    #[must_use]
    pub fn od(&mut self) -> OD_W<PAD_CTL_SPEC> {
        OD_W::new(self, 8)
    }
    #[doc = "Bit 16 - keeper capability enable 0: keeper disable 1: keeper enable"]
    #[inline(always)]
    #[must_use]
    pub fn ke(&mut self) -> KE_W<PAD_CTL_SPEC> {
        KE_W::new(self, 16)
    }
    #[doc = "Bit 17 - pull enable 0: pull disable 1: pull enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<PAD_CTL_SPEC> {
        PE_W::new(self, 17)
    }
    #[doc = "Bit 18 - pull select 0: pull down 1: pull up"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<PAD_CTL_SPEC> {
        PS_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<PAD_CTL_SPEC> {
        PRS_W::new(self, 20)
    }
    #[doc = "Bit 24 - schmitt trigger enable 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn hys(&mut self) -> HYS_W<PAD_CTL_SPEC> {
        HYS_W::new(self, 24)
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
#[doc = "PAD SETTINGS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_CTL_SPEC;
impl crate::RegisterSpec for PAD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_ctl::R`](R) reader structure"]
impl crate::Readable for PAD_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_ctl::W`](W) writer structure"]
impl crate::Writable for PAD_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_CTL to value 0x0101_0056"]
impl crate::Resettable for PAD_CTL_SPEC {
    const RESET_VALUE: u32 = 0x0101_0056;
}
