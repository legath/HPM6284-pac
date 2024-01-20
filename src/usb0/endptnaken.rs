#[doc = "Register `ENDPTNAKEN` reader"]
pub type R = crate::R<ENDPTNAKEN_SPEC>;
#[doc = "Register `ENDPTNAKEN` writer"]
pub type W = crate::W<ENDPTNAKEN_SPEC>;
#[doc = "Field `EPRNE` reader - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPRNE_R = crate::FieldReader;
#[doc = "Field `EPRNE` writer - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPRNE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EPTNE` reader - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPTNE_R = crate::FieldReader;
#[doc = "Field `EPTNE` writer - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPTNE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eprne(&self) -> EPRNE_R {
        EPRNE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eptne(&self) -> EPTNE_R {
        EPTNE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    #[must_use]
    pub fn eprne(&mut self) -> EPRNE_W<ENDPTNAKEN_SPEC> {
        EPRNE_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    #[must_use]
    pub fn eptne(&mut self) -> EPTNE_W<ENDPTNAKEN_SPEC> {
        EPTNE_W::new(self, 16)
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
#[doc = "Endpoint NAK Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptnaken::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptnaken::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTNAKEN_SPEC;
impl crate::RegisterSpec for ENDPTNAKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptnaken::R`](R) reader structure"]
impl crate::Readable for ENDPTNAKEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptnaken::W`](W) writer structure"]
impl crate::Writable for ENDPTNAKEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTNAKEN to value 0"]
impl crate::Resettable for ENDPTNAKEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
