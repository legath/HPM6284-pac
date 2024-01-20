#[doc = "Register `TDCR` reader"]
pub type R = crate::R<TDCR_SPEC>;
#[doc = "Register `TDCR` writer"]
pub type W = crate::W<TDCR_SPEC>;
#[doc = "Field `TDCF` reader - Transmitter Delay Compensation Filter Window Length Defines the minimum value for the SSP position, dominant edges on m_can_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq."]
pub type TDCF_R = crate::FieldReader;
#[doc = "Field `TDCF` writer - Transmitter Delay Compensation Filter Window Length Defines the minimum value for the SSP position, dominant edges on m_can_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq."]
pub type TDCF_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TDCO` reader - Transmitter Delay Compensation SSP Offset Offset value defining the distance between the measured delay from m_can_tx to m_can_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
pub type TDCO_R = crate::FieldReader;
#[doc = "Field `TDCO` writer - Transmitter Delay Compensation SSP Offset Offset value defining the distance between the measured delay from m_can_tx to m_can_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
pub type TDCO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length Defines the minimum value for the SSP position, dominant edges on m_can_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation SSP Offset Offset value defining the distance between the measured delay from m_can_tx to m_can_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length Defines the minimum value for the SSP position, dominant edges on m_can_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    #[must_use]
    pub fn tdcf(&mut self) -> TDCF_W<TDCR_SPEC> {
        TDCF_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation SSP Offset Offset value defining the distance between the measured delay from m_can_tx to m_can_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    #[must_use]
    pub fn tdco(&mut self) -> TDCO_W<TDCR_SPEC> {
        TDCO_W::new(self, 8)
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
#[doc = "transmitter delay compensation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDCR_SPEC;
impl crate::RegisterSpec for TDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdcr::R`](R) reader structure"]
impl crate::Readable for TDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdcr::W`](W) writer structure"]
impl crate::Writable for TDCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDCR to value 0"]
impl crate::Resettable for TDCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
