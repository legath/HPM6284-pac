#[doc = "Register `NBTP` reader"]
pub type R = crate::R<NBTP_SPEC>;
#[doc = "Register `NBTP` writer"]
pub type W = crate::W<NBTP_SPEC>;
#[doc = "Field `NTSEG2` reader - Nominal Time segment after sample point Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type NTSEG2_R = crate::FieldReader;
#[doc = "Field `NTSEG2` writer - Nominal Time segment after sample point Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type NTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NTSEG1` reader - Nominal Time segment before sample point Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type NTSEG1_R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - Nominal Time segment before sample point Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
pub type NTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBRP` reader - Nominal Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type NBRP_R = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - Nominal Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type NBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSJW` reader - Nominal (Re)Synchronization Jump Width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type NSJW_R = crate::FieldReader;
#[doc = "Field `NSJW` writer - Nominal (Re)Synchronization Jump Width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type NSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> NTSEG2_W<NBTP_SPEC> {
        NTSEG2_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<NBTP_SPEC> {
        NTSEG1_W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<NBTP_SPEC> {
        NBRP_W::new(self, 16)
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<NBTP_SPEC> {
        NSJW_W::new(self, 25)
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
#[doc = "nominal bit timing and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NBTP_SPEC;
impl crate::RegisterSpec for NBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbtp::R`](R) reader structure"]
impl crate::Readable for NBTP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nbtp::W`](W) writer structure"]
impl crate::Writable for NBTP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NBTP to value 0x0600_0a03"]
impl crate::Resettable for NBTP_SPEC {
    const RESET_VALUE: u32 = 0x0600_0a03;
}
