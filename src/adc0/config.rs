#[doc = "Register `CONFIG[%s]` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG[%s]` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `CHAN0` reader - channel number for 1st conversion"]
pub type CHAN0_R = crate::FieldReader;
#[doc = "Field `CHAN0` writer - channel number for 1st conversion"]
pub type CHAN0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INTEN0` reader - interupt enable for 1st conversion"]
pub type INTEN0_R = crate::BitReader;
#[doc = "Field `INTEN0` writer - interupt enable for 1st conversion"]
pub type INTEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEUE_EN` reader - preemption queue enable control"]
pub type QUEUE_EN_R = crate::BitReader;
#[doc = "Field `QUEUE_EN` writer - preemption queue enable control"]
pub type QUEUE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN1` reader - channel number for 2nd conversion"]
pub type CHAN1_R = crate::FieldReader;
#[doc = "Field `CHAN1` writer - channel number for 2nd conversion"]
pub type CHAN1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INTEN1` reader - interupt enable for 2nd conversion"]
pub type INTEN1_R = crate::BitReader;
#[doc = "Field `INTEN1` writer - interupt enable for 2nd conversion"]
pub type INTEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN2` reader - channel number for 3rd conversion"]
pub type CHAN2_R = crate::FieldReader;
#[doc = "Field `CHAN2` writer - channel number for 3rd conversion"]
pub type CHAN2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INTEN2` reader - interupt enable for 3rd conversion"]
pub type INTEN2_R = crate::BitReader;
#[doc = "Field `INTEN2` writer - interupt enable for 3rd conversion"]
pub type INTEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN3` reader - channel number for 4th conversion"]
pub type CHAN3_R = crate::FieldReader;
#[doc = "Field `CHAN3` writer - channel number for 4th conversion"]
pub type CHAN3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INTEN3` reader - interupt enable for 4th conversion"]
pub type INTEN3_R = crate::BitReader;
#[doc = "Field `INTEN3` writer - interupt enable for 4th conversion"]
pub type INTEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG_LEN` writer - length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
pub type TRIG_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - channel number for 1st conversion"]
    #[inline(always)]
    pub fn chan0(&self) -> CHAN0_R {
        CHAN0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - interupt enable for 1st conversion"]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - preemption queue enable control"]
    #[inline(always)]
    pub fn queue_en(&self) -> QUEUE_EN_R {
        QUEUE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:12 - channel number for 2nd conversion"]
    #[inline(always)]
    pub fn chan1(&self) -> CHAN1_R {
        CHAN1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - interupt enable for 2nd conversion"]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - channel number for 3rd conversion"]
    #[inline(always)]
    pub fn chan2(&self) -> CHAN2_R {
        CHAN2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - interupt enable for 3rd conversion"]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:28 - channel number for 4th conversion"]
    #[inline(always)]
    pub fn chan3(&self) -> CHAN3_R {
        CHAN3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - interupt enable for 4th conversion"]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel number for 1st conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan0(&mut self) -> CHAN0_W<CONFIG_SPEC> {
        CHAN0_W::new(self, 0)
    }
    #[doc = "Bit 5 - interupt enable for 1st conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inten0(&mut self) -> INTEN0_W<CONFIG_SPEC> {
        INTEN0_W::new(self, 5)
    }
    #[doc = "Bit 6 - preemption queue enable control"]
    #[inline(always)]
    #[must_use]
    pub fn queue_en(&mut self) -> QUEUE_EN_W<CONFIG_SPEC> {
        QUEUE_EN_W::new(self, 6)
    }
    #[doc = "Bits 8:12 - channel number for 2nd conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan1(&mut self) -> CHAN1_W<CONFIG_SPEC> {
        CHAN1_W::new(self, 8)
    }
    #[doc = "Bit 13 - interupt enable for 2nd conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inten1(&mut self) -> INTEN1_W<CONFIG_SPEC> {
        INTEN1_W::new(self, 13)
    }
    #[doc = "Bits 16:20 - channel number for 3rd conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan2(&mut self) -> CHAN2_W<CONFIG_SPEC> {
        CHAN2_W::new(self, 16)
    }
    #[doc = "Bit 21 - interupt enable for 3rd conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inten2(&mut self) -> INTEN2_W<CONFIG_SPEC> {
        INTEN2_W::new(self, 21)
    }
    #[doc = "Bits 24:28 - channel number for 4th conversion"]
    #[inline(always)]
    #[must_use]
    pub fn chan3(&mut self) -> CHAN3_W<CONFIG_SPEC> {
        CHAN3_W::new(self, 24)
    }
    #[doc = "Bit 29 - interupt enable for 4th conversion"]
    #[inline(always)]
    #[must_use]
    pub fn inten3(&mut self) -> INTEN3_W<CONFIG_SPEC> {
        INTEN3_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn trig_len(&mut self) -> TRIG_LEN_W<CONFIG_SPEC> {
        TRIG_LEN_W::new(self, 30)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG[%s]
to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
