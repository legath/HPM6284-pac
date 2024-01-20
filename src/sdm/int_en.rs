#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `CH0ERR` reader - Ch0 Error interrupt enable"]
pub type CH0ERR_R = crate::BitReader;
#[doc = "Field `CH0ERR` writer - Ch0 Error interrupt enable"]
pub type CH0ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ERR` reader - Ch1 Error interrupt enable"]
pub type CH1ERR_R = crate::BitReader;
#[doc = "Field `CH1ERR` writer - Ch1 Error interrupt enable"]
pub type CH1ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2ERR` reader - Ch2 Error interrupt enable"]
pub type CH2ERR_R = crate::BitReader;
#[doc = "Field `CH2ERR` writer - Ch2 Error interrupt enable"]
pub type CH2ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3ERR` reader - Ch3 Error interrupt enable."]
pub type CH3ERR_R = crate::BitReader;
#[doc = "Field `CH3ERR` writer - Ch3 Error interrupt enable."]
pub type CH3ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0DRY` reader - Ch0 Data Ready interrupt enable"]
pub type CH0DRY_R = crate::BitReader;
#[doc = "Field `CH0DRY` writer - Ch0 Data Ready interrupt enable"]
pub type CH0DRY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DRY` reader - Ch1 Data Ready interrupt enable"]
pub type CH1DRY_R = crate::BitReader;
#[doc = "Field `CH1DRY` writer - Ch1 Data Ready interrupt enable"]
pub type CH1DRY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2DRY` reader - Ch2 Data Ready interrupt enable"]
pub type CH2DRY_R = crate::BitReader;
#[doc = "Field `CH2DRY` writer - Ch2 Data Ready interrupt enable"]
pub type CH2DRY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3DRY` reader - Ch3 Data Ready interrupt enable."]
pub type CH3DRY_R = crate::BitReader;
#[doc = "Field `CH3DRY` writer - Ch3 Data Ready interrupt enable."]
pub type CH3DRY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ch0 Error interrupt enable"]
    #[inline(always)]
    pub fn ch0err(&self) -> CH0ERR_R {
        CH0ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ch1 Error interrupt enable"]
    #[inline(always)]
    pub fn ch1err(&self) -> CH1ERR_R {
        CH1ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ch2 Error interrupt enable"]
    #[inline(always)]
    pub fn ch2err(&self) -> CH2ERR_R {
        CH2ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ch3 Error interrupt enable."]
    #[inline(always)]
    pub fn ch3err(&self) -> CH3ERR_R {
        CH3ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ch0 Data Ready interrupt enable"]
    #[inline(always)]
    pub fn ch0dry(&self) -> CH0DRY_R {
        CH0DRY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ch1 Data Ready interrupt enable"]
    #[inline(always)]
    pub fn ch1dry(&self) -> CH1DRY_R {
        CH1DRY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ch2 Data Ready interrupt enable"]
    #[inline(always)]
    pub fn ch2dry(&self) -> CH2DRY_R {
        CH2DRY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ch3 Data Ready interrupt enable."]
    #[inline(always)]
    pub fn ch3dry(&self) -> CH3DRY_R {
        CH3DRY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ch0 Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0err(&mut self) -> CH0ERR_W<INT_EN_SPEC> {
        CH0ERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Ch1 Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1err(&mut self) -> CH1ERR_W<INT_EN_SPEC> {
        CH1ERR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Ch2 Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2err(&mut self) -> CH2ERR_W<INT_EN_SPEC> {
        CH2ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Ch3 Error interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ch3err(&mut self) -> CH3ERR_W<INT_EN_SPEC> {
        CH3ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Ch0 Data Ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0dry(&mut self) -> CH0DRY_W<INT_EN_SPEC> {
        CH0DRY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Ch1 Data Ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1dry(&mut self) -> CH1DRY_W<INT_EN_SPEC> {
        CH1DRY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Ch2 Data Ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2dry(&mut self) -> CH2DRY_W<INT_EN_SPEC> {
        CH2DRY_W::new(self, 6)
    }
    #[doc = "Bit 7 - Ch3 Data Ready interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn ch3dry(&mut self) -> CH3DRY_W<INT_EN_SPEC> {
        CH3DRY_W::new(self, 7)
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
#[doc = "Interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
