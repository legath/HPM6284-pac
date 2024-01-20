#[doc = "Register `DBTP` reader"]
pub type R = crate::R<DBTP_SPEC>;
#[doc = "Register `DBTP` writer"]
pub type W = crate::W<DBTP_SPEC>;
#[doc = "Field `DSJW` reader - Data (Re)Synchronization Jump Width Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type DSJW_R = crate::FieldReader;
#[doc = "Field `DSJW` writer - Data (Re)Synchronization Jump Width Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type DSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type DTSEG2_R = crate::FieldReader;
#[doc = "Field `DTSEG2` writer - Data time segment after sample point Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type DTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTSEG1` reader - Data time segment before sample point Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type DTSEG1_R = crate::FieldReader;
#[doc = "Field `DTSEG1` writer - Data time segment before sample point Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type DTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBRP` reader - Data Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. When TDC = ‘1’, the range is limited to 0,1. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type DBRP_R = crate::FieldReader;
#[doc = "Field `DBRP` writer - Data Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. When TDC = ‘1’, the range is limited to 0,1. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type DBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TDC` reader - transmitter delay compensation enable 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled"]
pub type TDC_R = crate::BitReader;
#[doc = "Field `TDC` writer - transmitter delay compensation enable 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled"]
pub type TDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data (Re)Synchronization Jump Width Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. When TDC = ‘1’, the range is limited to 0,1. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - transmitter delay compensation enable 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data (Re)Synchronization Jump Width Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    #[must_use]
    pub fn dsjw(&mut self) -> DSJW_W<DBTP_SPEC> {
        DSJW_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    #[must_use]
    pub fn dtseg2(&mut self) -> DTSEG2_W<DBTP_SPEC> {
        DTSEG2_W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    #[must_use]
    pub fn dtseg1(&mut self) -> DTSEG1_W<DBTP_SPEC> {
        DTSEG1_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. When TDC = ‘1’, the range is limited to 0,1. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    #[must_use]
    pub fn dbrp(&mut self) -> DBRP_W<DBTP_SPEC> {
        DBRP_W::new(self, 16)
    }
    #[doc = "Bit 23 - transmitter delay compensation enable 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TDC_W<DBTP_SPEC> {
        TDC_W::new(self, 23)
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
#[doc = "data bit timing and prescaler, writeable when CCCR.CCE and CCCR.INT are set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBTP_SPEC;
impl crate::RegisterSpec for DBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbtp::R`](R) reader structure"]
impl crate::Readable for DBTP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbtp::W`](W) writer structure"]
impl crate::Writable for DBTP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBTP to value 0x0a33"]
impl crate::Resettable for DBTP_SPEC {
    const RESET_VALUE: u32 = 0x0a33;
}
