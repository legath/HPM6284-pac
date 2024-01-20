#[doc = "Register `ENDPTNAK` reader"]
pub type R = crate::R<ENDPTNAK_SPEC>;
#[doc = "Register `ENDPTNAK` writer"]
pub type W = crate::W<ENDPTNAK_SPEC>;
#[doc = "Field `EPRN` reader - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPRN_R = crate::FieldReader;
#[doc = "Field `EPRN` writer - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPRN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EPTN` reader - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPTN_R = crate::FieldReader;
#[doc = "Field `EPTN` writer - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPTN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eprn(&self) -> EPRN_R {
        EPRN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eptn(&self) -> EPTN_R {
        EPTN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    #[must_use]
    pub fn eprn(&mut self) -> EPRN_W<ENDPTNAK_SPEC> {
        EPRN_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    #[must_use]
    pub fn eptn(&mut self) -> EPTN_W<ENDPTNAK_SPEC> {
        EPTN_W::new(self, 16)
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
#[doc = "Endpoint NAK Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptnak::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptnak::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTNAK_SPEC;
impl crate::RegisterSpec for ENDPTNAK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptnak::R`](R) reader structure"]
impl crate::Readable for ENDPTNAK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptnak::W`](W) writer structure"]
impl crate::Writable for ENDPTNAK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTNAK to value 0"]
impl crate::Resettable for ENDPTNAK_SPEC {
    const RESET_VALUE: u32 = 0;
}
