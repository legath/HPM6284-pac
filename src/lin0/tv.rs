#[doc = "Register `tv` reader"]
pub type R = crate::R<TV_SPEC>;
#[doc = "Register `tv` writer"]
pub type W = crate::W<TV_SPEC>;
#[doc = "Field `WUP_REPEAT_TIME` reader - slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms"]
pub type WUP_REPEAT_TIME_R = crate::FieldReader;
#[doc = "Field `WUP_REPEAT_TIME` writer - slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms"]
pub type WUP_REPEAT_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BUS_INACTIVITY_TIME` reader - slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s"]
pub type BUS_INACTIVITY_TIME_R = crate::FieldReader;
#[doc = "Field `BUS_INACTIVITY_TIME` writer - slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s"]
pub type BUS_INACTIVITY_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MASTER_MODE` reader - master_mode"]
pub type MASTER_MODE_R = crate::BitReader;
#[doc = "Field `MASTER_MODE` writer - master_mode"]
pub type MASTER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITIAL_MODE` reader - initial_mode"]
pub type INITIAL_MODE_R = crate::BitReader;
#[doc = "Field `INITIAL_MODE` writer - initial_mode"]
pub type INITIAL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms"]
    #[inline(always)]
    pub fn wup_repeat_time(&self) -> WUP_REPEAT_TIME_R {
        WUP_REPEAT_TIME_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s"]
    #[inline(always)]
    pub fn bus_inactivity_time(&self) -> BUS_INACTIVITY_TIME_R {
        BUS_INACTIVITY_TIME_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - master_mode"]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - initial_mode"]
    #[inline(always)]
    pub fn initial_mode(&self) -> INITIAL_MODE_R {
        INITIAL_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms"]
    #[inline(always)]
    #[must_use]
    pub fn wup_repeat_time(&mut self) -> WUP_REPEAT_TIME_W<TV_SPEC> {
        WUP_REPEAT_TIME_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s"]
    #[inline(always)]
    #[must_use]
    pub fn bus_inactivity_time(&mut self) -> BUS_INACTIVITY_TIME_W<TV_SPEC> {
        BUS_INACTIVITY_TIME_W::new(self, 2)
    }
    #[doc = "Bit 6 - master_mode"]
    #[inline(always)]
    #[must_use]
    pub fn master_mode(&mut self) -> MASTER_MODE_W<TV_SPEC> {
        MASTER_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - initial_mode"]
    #[inline(always)]
    #[must_use]
    pub fn initial_mode(&mut self) -> INITIAL_MODE_W<TV_SPEC> {
        INITIAL_MODE_W::new(self, 7)
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
#[doc = "timeout control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_SPEC;
impl crate::RegisterSpec for TV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv::R`](R) reader structure"]
impl crate::Readable for TV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv::W`](W) writer structure"]
impl crate::Writable for TV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tv to value 0x40"]
impl crate::Resettable for TV_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
