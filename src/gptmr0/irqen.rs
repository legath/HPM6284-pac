#[doc = "Register `IRQEN` reader"]
pub type R = crate::R<IRQEN_SPEC>;
#[doc = "Register `IRQEN` writer"]
pub type W = crate::W<IRQEN_SPEC>;
#[doc = "Field `CH0RLDEN` reader - 1- generate interrupt request when ch0rldf flag is set"]
pub type CH0RLDEN_R = crate::BitReader;
#[doc = "Field `CH0RLDEN` writer - 1- generate interrupt request when ch0rldf flag is set"]
pub type CH0RLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0CAPEN` reader - 1- generate interrupt request when ch0capf flag is set"]
pub type CH0CAPEN_R = crate::BitReader;
#[doc = "Field `CH0CAPEN` writer - 1- generate interrupt request when ch0capf flag is set"]
pub type CH0CAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0CMP0EN` reader - 1- generate interrupt request when ch0cmp0f flag is set"]
pub type CH0CMP0EN_R = crate::BitReader;
#[doc = "Field `CH0CMP0EN` writer - 1- generate interrupt request when ch0cmp0f flag is set"]
pub type CH0CMP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0CMP1EN` reader - 1- generate interrupt request when ch0cmp1f flag is set"]
pub type CH0CMP1EN_R = crate::BitReader;
#[doc = "Field `CH0CMP1EN` writer - 1- generate interrupt request when ch0cmp1f flag is set"]
pub type CH0CMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1RLDEN` reader - 1- generate interrupt request when ch1rldf flag is set"]
pub type CH1RLDEN_R = crate::BitReader;
#[doc = "Field `CH1RLDEN` writer - 1- generate interrupt request when ch1rldf flag is set"]
pub type CH1RLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CAPEN` reader - 1- generate interrupt request when ch1capf flag is set"]
pub type CH1CAPEN_R = crate::BitReader;
#[doc = "Field `CH1CAPEN` writer - 1- generate interrupt request when ch1capf flag is set"]
pub type CH1CAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CMP0EN` reader - 1- generate interrupt request when ch1cmp0f flag is set"]
pub type CH1CMP0EN_R = crate::BitReader;
#[doc = "Field `CH1CMP0EN` writer - 1- generate interrupt request when ch1cmp0f flag is set"]
pub type CH1CMP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CMP1EN` reader - 1- generate interrupt request when ch1cmp1f flag is set"]
pub type CH1CMP1EN_R = crate::BitReader;
#[doc = "Field `CH1CMP1EN` writer - 1- generate interrupt request when ch1cmp1f flag is set"]
pub type CH1CMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2RLDEN` reader - 1- generate interrupt request when ch2rldf flag is set"]
pub type CH2RLDEN_R = crate::BitReader;
#[doc = "Field `CH2RLDEN` writer - 1- generate interrupt request when ch2rldf flag is set"]
pub type CH2RLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CAPEN` reader - 1- generate interrupt request when ch2capf flag is set"]
pub type CH2CAPEN_R = crate::BitReader;
#[doc = "Field `CH2CAPEN` writer - 1- generate interrupt request when ch2capf flag is set"]
pub type CH2CAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CMP0EN` reader - 1- generate interrupt request when ch2cmp0f flag is set"]
pub type CH2CMP0EN_R = crate::BitReader;
#[doc = "Field `CH2CMP0EN` writer - 1- generate interrupt request when ch2cmp0f flag is set"]
pub type CH2CMP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CMP1EN` reader - 1- generate interrupt request when ch2cmp1f flag is set"]
pub type CH2CMP1EN_R = crate::BitReader;
#[doc = "Field `CH2CMP1EN` writer - 1- generate interrupt request when ch2cmp1f flag is set"]
pub type CH2CMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3RLDEN` reader - 1- generate interrupt request when ch3rldf flag is set"]
pub type CH3RLDEN_R = crate::BitReader;
#[doc = "Field `CH3RLDEN` writer - 1- generate interrupt request when ch3rldf flag is set"]
pub type CH3RLDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CAPEN` reader - 1- generate interrupt request when ch3capf flag is set"]
pub type CH3CAPEN_R = crate::BitReader;
#[doc = "Field `CH3CAPEN` writer - 1- generate interrupt request when ch3capf flag is set"]
pub type CH3CAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CMP0EN` reader - 1- generate interrupt request when ch3cmp0f flag is set"]
pub type CH3CMP0EN_R = crate::BitReader;
#[doc = "Field `CH3CMP0EN` writer - 1- generate interrupt request when ch3cmp0f flag is set"]
pub type CH3CMP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CMP1EN` reader - 1- generate interrupt request when ch3cmp1f flag is set"]
pub type CH3CMP1EN_R = crate::BitReader;
#[doc = "Field `CH3CMP1EN` writer - 1- generate interrupt request when ch3cmp1f flag is set"]
pub type CH3CMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1- generate interrupt request when ch0rldf flag is set"]
    #[inline(always)]
    pub fn ch0rlden(&self) -> CH0RLDEN_R {
        CH0RLDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1- generate interrupt request when ch0capf flag is set"]
    #[inline(always)]
    pub fn ch0capen(&self) -> CH0CAPEN_R {
        CH0CAPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1- generate interrupt request when ch0cmp0f flag is set"]
    #[inline(always)]
    pub fn ch0cmp0en(&self) -> CH0CMP0EN_R {
        CH0CMP0EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1- generate interrupt request when ch0cmp1f flag is set"]
    #[inline(always)]
    pub fn ch0cmp1en(&self) -> CH0CMP1EN_R {
        CH0CMP1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1- generate interrupt request when ch1rldf flag is set"]
    #[inline(always)]
    pub fn ch1rlden(&self) -> CH1RLDEN_R {
        CH1RLDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1- generate interrupt request when ch1capf flag is set"]
    #[inline(always)]
    pub fn ch1capen(&self) -> CH1CAPEN_R {
        CH1CAPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1- generate interrupt request when ch1cmp0f flag is set"]
    #[inline(always)]
    pub fn ch1cmp0en(&self) -> CH1CMP0EN_R {
        CH1CMP0EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1- generate interrupt request when ch1cmp1f flag is set"]
    #[inline(always)]
    pub fn ch1cmp1en(&self) -> CH1CMP1EN_R {
        CH1CMP1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1- generate interrupt request when ch2rldf flag is set"]
    #[inline(always)]
    pub fn ch2rlden(&self) -> CH2RLDEN_R {
        CH2RLDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1- generate interrupt request when ch2capf flag is set"]
    #[inline(always)]
    pub fn ch2capen(&self) -> CH2CAPEN_R {
        CH2CAPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1- generate interrupt request when ch2cmp0f flag is set"]
    #[inline(always)]
    pub fn ch2cmp0en(&self) -> CH2CMP0EN_R {
        CH2CMP0EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1- generate interrupt request when ch2cmp1f flag is set"]
    #[inline(always)]
    pub fn ch2cmp1en(&self) -> CH2CMP1EN_R {
        CH2CMP1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1- generate interrupt request when ch3rldf flag is set"]
    #[inline(always)]
    pub fn ch3rlden(&self) -> CH3RLDEN_R {
        CH3RLDEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1- generate interrupt request when ch3capf flag is set"]
    #[inline(always)]
    pub fn ch3capen(&self) -> CH3CAPEN_R {
        CH3CAPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1- generate interrupt request when ch3cmp0f flag is set"]
    #[inline(always)]
    pub fn ch3cmp0en(&self) -> CH3CMP0EN_R {
        CH3CMP0EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1- generate interrupt request when ch3cmp1f flag is set"]
    #[inline(always)]
    pub fn ch3cmp1en(&self) -> CH3CMP1EN_R {
        CH3CMP1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1- generate interrupt request when ch0rldf flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rlden(&mut self) -> CH0RLDEN_W<IRQEN_SPEC> {
        CH0RLDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1- generate interrupt request when ch0capf flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0capen(&mut self) -> CH0CAPEN_W<IRQEN_SPEC> {
        CH0CAPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1- generate interrupt request when ch0cmp0f flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cmp0en(&mut self) -> CH0CMP0EN_W<IRQEN_SPEC> {
        CH0CMP0EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1- generate interrupt request when ch0cmp1f flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cmp1en(&mut self) -> CH0CMP1EN_W<IRQEN_SPEC> {
        CH0CMP1EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1- generate interrupt request when ch1rldf flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rlden(&mut self) -> CH1RLDEN_W<IRQEN_SPEC> {
        CH1RLDEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1- generate interrupt request when ch1capf flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1capen(&mut self) -> CH1CAPEN_W<IRQEN_SPEC> {
        CH1CAPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1- generate interrupt request when ch1cmp0f flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cmp0en(&mut self) -> CH1CMP0EN_W<IRQEN_SPEC> {
        CH1CMP0EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1- generate interrupt request when ch1cmp1f flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cmp1en(&mut self) -> CH1CMP1EN_W<IRQEN_SPEC> {
        CH1CMP1EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1- generate interrupt request when ch2rldf flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2rlden(&mut self) -> CH2RLDEN_W<IRQEN_SPEC> {
        CH2RLDEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1- generate interrupt request when ch2capf flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2capen(&mut self) -> CH2CAPEN_W<IRQEN_SPEC> {
        CH2CAPEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1- generate interrupt request when ch2cmp0f flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cmp0en(&mut self) -> CH2CMP0EN_W<IRQEN_SPEC> {
        CH2CMP0EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1- generate interrupt request when ch2cmp1f flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cmp1en(&mut self) -> CH2CMP1EN_W<IRQEN_SPEC> {
        CH2CMP1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1- generate interrupt request when ch3rldf flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3rlden(&mut self) -> CH3RLDEN_W<IRQEN_SPEC> {
        CH3RLDEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1- generate interrupt request when ch3capf flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3capen(&mut self) -> CH3CAPEN_W<IRQEN_SPEC> {
        CH3CAPEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1- generate interrupt request when ch3cmp0f flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cmp0en(&mut self) -> CH3CMP0EN_W<IRQEN_SPEC> {
        CH3CMP0EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1- generate interrupt request when ch3cmp1f flag is set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cmp1en(&mut self) -> CH3CMP1EN_W<IRQEN_SPEC> {
        CH3CMP1EN_W::new(self, 15)
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
#[doc = "Interrupt request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQEN_SPEC;
impl crate::RegisterSpec for IRQEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqen::R`](R) reader structure"]
impl crate::Readable for IRQEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irqen::W`](W) writer structure"]
impl crate::Writable for IRQEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQEN to value 0"]
impl crate::Resettable for IRQEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
