#[doc = "Register `TOCC` reader"]
pub type R = crate::R<TOCC_SPEC>;
#[doc = "Register `TOCC` writer"]
pub type W = crate::W<TOCC_SPEC>;
#[doc = "Field `RP` reader - Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled"]
pub type RP_R = crate::BitReader;
#[doc = "Field `RP` writer - Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled"]
pub type RP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS` reader - Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1"]
pub type TOS_R = crate::FieldReader;
#[doc = "Field `TOS` writer - Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1"]
pub type TOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOP` reader - Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period."]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period."]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period."]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RP_W<TOCC_SPEC> {
        RP_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<TOCC_SPEC> {
        TOS_W::new(self, 1)
    }
    #[doc = "Bits 16:31 - Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period."]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<TOCC_SPEC> {
        TOP_W::new(self, 16)
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
#[doc = "timeout counter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCC_SPEC;
impl crate::RegisterSpec for TOCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tocc::R`](R) reader structure"]
impl crate::Readable for TOCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tocc::W`](W) writer structure"]
impl crate::Writable for TOCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOCC to value 0xffff_0000"]
impl crate::Resettable for TOCC_SPEC {
    const RESET_VALUE: u32 = 0xffff_0000;
}
