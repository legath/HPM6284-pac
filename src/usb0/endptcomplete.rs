#[doc = "Register `ENDPTCOMPLETE` reader"]
pub type R = crate::R<ENDPTCOMPLETE_SPEC>;
#[doc = "Register `ENDPTCOMPLETE` writer"]
pub type W = crate::W<ENDPTCOMPLETE_SPEC>;
#[doc = "Field `ERCE` reader - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ERCE_R = crate::FieldReader;
#[doc = "Field `ERCE` writer - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ERCE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ETCE` reader - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ETCE_R = crate::FieldReader;
#[doc = "Field `ETCE` writer - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ETCE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn erce(&self) -> ERCE_R {
        ERCE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn etce(&self) -> ETCE_R {
        ETCE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    #[must_use]
    pub fn erce(&mut self) -> ERCE_W<ENDPTCOMPLETE_SPEC> {
        ERCE_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    #[must_use]
    pub fn etce(&mut self) -> ETCE_W<ENDPTCOMPLETE_SPEC> {
        ETCE_W::new(self, 16)
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
#[doc = "Endpoint Complete Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptcomplete::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptcomplete::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTCOMPLETE_SPEC;
impl crate::RegisterSpec for ENDPTCOMPLETE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptcomplete::R`](R) reader structure"]
impl crate::Readable for ENDPTCOMPLETE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptcomplete::W`](W) writer structure"]
impl crate::Writable for ENDPTCOMPLETE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTCOMPLETE to value 0"]
impl crate::Resettable for ENDPTCOMPLETE_SPEC {
    const RESET_VALUE: u32 = 0;
}
