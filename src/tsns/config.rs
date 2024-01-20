#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `ENABLE` reader - Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNC` reader - Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode"]
pub type ASYNC_R = crate::BitReader;
#[doc = "Field `ASYNC` writer - Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode"]
pub type ASYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTINUOUS` reader - continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode"]
pub type CONTINUOUS_R = crate::BitReader;
#[doc = "Field `CONTINUOUS` writer - continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode"]
pub type CONTINUOUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVERAGE` reader - Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average"]
pub type AVERAGE_R = crate::FieldReader;
#[doc = "Field `AVERAGE` writer - Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average"]
pub type AVERAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPEED` reader - cycles of a progressive step in 24M clock, valide from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step"]
pub type SPEED_R = crate::FieldReader;
#[doc = "Field `SPEED` writer - cycles of a progressive step in 24M clock, valide from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step"]
pub type SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COMPARE_MAX_EN` reader - Enable compare for maximum temperature"]
pub type COMPARE_MAX_EN_R = crate::BitReader;
#[doc = "Field `COMPARE_MAX_EN` writer - Enable compare for maximum temperature"]
pub type COMPARE_MAX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPARE_MIN_EN` reader - Enable compare for minimum temperature"]
pub type COMPARE_MIN_EN_R = crate::BitReader;
#[doc = "Field `COMPARE_MIN_EN` writer - Enable compare for minimum temperature"]
pub type COMPARE_MIN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN` reader - Enable reset"]
pub type RST_EN_R = crate::BitReader;
#[doc = "Field `RST_EN` writer - Enable reset"]
pub type RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_EN` reader - Enable interrupt"]
pub type IRQ_EN_R = crate::BitReader;
#[doc = "Field `IRQ_EN` writer - Enable interrupt"]
pub type IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode"]
    #[inline(always)]
    pub fn async_(&self) -> ASYNC_R {
        ASYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode"]
    #[inline(always)]
    pub fn continuous(&self) -> CONTINUOUS_R {
        CONTINUOUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average"]
    #[inline(always)]
    pub fn average(&self) -> AVERAGE_R {
        AVERAGE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:23 - cycles of a progressive step in 24M clock, valide from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable compare for maximum temperature"]
    #[inline(always)]
    pub fn compare_max_en(&self) -> COMPARE_MAX_EN_R {
        COMPARE_MAX_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable compare for minimum temperature"]
    #[inline(always)]
    pub fn compare_min_en(&self) -> COMPARE_MIN_EN_R {
        COMPARE_MIN_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable reset"]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable interrupt"]
    #[inline(always)]
    pub fn irq_en(&self) -> IRQ_EN_R {
        IRQ_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CONFIG_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode"]
    #[inline(always)]
    #[must_use]
    pub fn async_(&mut self) -> ASYNC_W<CONFIG_SPEC> {
        ASYNC_W::new(self, 1)
    }
    #[doc = "Bit 4 - continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn continuous(&mut self) -> CONTINUOUS_W<CONFIG_SPEC> {
        CONTINUOUS_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average"]
    #[inline(always)]
    #[must_use]
    pub fn average(&mut self) -> AVERAGE_W<CONFIG_SPEC> {
        AVERAGE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - cycles of a progressive step in 24M clock, valide from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<CONFIG_SPEC> {
        SPEED_W::new(self, 16)
    }
    #[doc = "Bit 24 - Enable compare for maximum temperature"]
    #[inline(always)]
    #[must_use]
    pub fn compare_max_en(&mut self) -> COMPARE_MAX_EN_W<CONFIG_SPEC> {
        COMPARE_MAX_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable compare for minimum temperature"]
    #[inline(always)]
    #[must_use]
    pub fn compare_min_en(&mut self) -> COMPARE_MIN_EN_W<CONFIG_SPEC> {
        COMPARE_MIN_EN_W::new(self, 25)
    }
    #[doc = "Bit 30 - Enable reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_en(&mut self) -> RST_EN_W<CONFIG_SPEC> {
        RST_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn irq_en(&mut self) -> IRQ_EN_W<CONFIG_SPEC> {
        IRQ_EN_W::new(self, 31)
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
#[doc = "Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CONFIG to value 0x0060_0300"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x0060_0300;
}
