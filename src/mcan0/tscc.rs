#[doc = "Register `TSCC` reader"]
pub type R = crate::R<TSCC_SPEC>;
#[doc = "Register `TSCC` writer"]
pub type W = crate::W<TSCC_SPEC>;
#[doc = "Field `TSS` reader - timestamp Select 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as “00”"]
pub type TSS_R = crate::FieldReader;
#[doc = "Field `TSS` writer - timestamp Select 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as “00”"]
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCP` reader - Timestamp Counter Prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1…16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type TCP_R = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp Counter Prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1…16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
pub type TCP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - timestamp Select 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as “00”"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1…16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - timestamp Select 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as “00”"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<TSCC_SPEC> {
        TSS_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1…16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TCP_W<TSCC_SPEC> {
        TCP_W::new(self, 16)
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
#[doc = "timestamp counter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCC_SPEC;
impl crate::RegisterSpec for TSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscc::R`](R) reader structure"]
impl crate::Readable for TSCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscc::W`](W) writer structure"]
impl crate::Writable for TSCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TSCC_SPEC {
    const RESET_VALUE: u32 = 0;
}
