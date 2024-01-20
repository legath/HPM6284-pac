#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `CH0ERR` reader - Ch0 Error"]
pub type CH0ERR_R = crate::BitReader;
#[doc = "Field `CH1ERR` reader - Ch1 Error"]
pub type CH1ERR_R = crate::BitReader;
#[doc = "Field `CH2ERR` reader - Ch2 Error"]
pub type CH2ERR_R = crate::BitReader;
#[doc = "Field `CH3ERR` reader - Ch3 Error. ORed together by channel related error signals and corresponding error interrupt enable signals. De-assert this bit by write-1-clear the corresponding error status bits in the channel status registers."]
pub type CH3ERR_R = crate::BitReader;
#[doc = "Field `CH0DRY` reader - Ch0 Data Ready"]
pub type CH0DRY_R = crate::BitReader;
#[doc = "Field `CH1DRY` reader - Ch1 Data Ready"]
pub type CH1DRY_R = crate::BitReader;
#[doc = "Field `CH2DRY` reader - Ch2 Data Ready"]
pub type CH2DRY_R = crate::BitReader;
#[doc = "Field `CH3DRY` reader - Ch3 Data Ready. De-assert this bit by reading the data (or data fifo) registers."]
pub type CH3DRY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Ch0 Error"]
    #[inline(always)]
    pub fn ch0err(&self) -> CH0ERR_R {
        CH0ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ch1 Error"]
    #[inline(always)]
    pub fn ch1err(&self) -> CH1ERR_R {
        CH1ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ch2 Error"]
    #[inline(always)]
    pub fn ch2err(&self) -> CH2ERR_R {
        CH2ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ch3 Error. ORed together by channel related error signals and corresponding error interrupt enable signals. De-assert this bit by write-1-clear the corresponding error status bits in the channel status registers."]
    #[inline(always)]
    pub fn ch3err(&self) -> CH3ERR_R {
        CH3ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ch0 Data Ready"]
    #[inline(always)]
    pub fn ch0dry(&self) -> CH0DRY_R {
        CH0DRY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ch1 Data Ready"]
    #[inline(always)]
    pub fn ch1dry(&self) -> CH1DRY_R {
        CH1DRY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ch2 Data Ready"]
    #[inline(always)]
    pub fn ch2dry(&self) -> CH2DRY_R {
        CH2DRY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ch3 Data Ready. De-assert this bit by reading the data (or data fifo) registers."]
    #[inline(always)]
    pub fn ch3dry(&self) -> CH3DRY_R {
        CH3DRY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
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
#[doc = "Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
