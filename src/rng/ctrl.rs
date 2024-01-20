#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `FUFMOD` reader - FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
pub type FUFMOD_R = crate::FieldReader;
#[doc = "Field `FUFMOD` writer - FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
pub type FUFMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AUTRSD` reader - Auto Reseed"]
pub type AUTRSD_R = crate::BitReader;
#[doc = "Field `AUTRSD` writer - Auto Reseed"]
pub type AUTRSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIRQDN` reader - Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
pub type MIRQDN_R = crate::BitReader;
#[doc = "Field `MIRQDN` writer - Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
pub type MIRQDN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIRQERR` reader - Mask Interrupt Request for Error"]
pub type MIRQERR_R = crate::BitReader;
#[doc = "Field `MIRQERR` writer - Mask Interrupt Request for Error"]
pub type MIRQERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
    #[inline(always)]
    pub fn fufmod(&self) -> FUFMOD_R {
        FUFMOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Auto Reseed"]
    #[inline(always)]
    pub fn autrsd(&self) -> AUTRSD_R {
        AUTRSD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
    #[inline(always)]
    pub fn mirqdn(&self) -> MIRQDN_R {
        MIRQDN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask Interrupt Request for Error"]
    #[inline(always)]
    pub fn mirqerr(&self) -> MIRQERR_R {
        MIRQERR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
    #[inline(always)]
    #[must_use]
    pub fn fufmod(&mut self) -> FUFMOD_W<CTRL_SPEC> {
        FUFMOD_W::new(self, 0)
    }
    #[doc = "Bit 4 - Auto Reseed"]
    #[inline(always)]
    #[must_use]
    pub fn autrsd(&mut self) -> AUTRSD_W<CTRL_SPEC> {
        AUTRSD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
    #[inline(always)]
    #[must_use]
    pub fn mirqdn(&mut self) -> MIRQDN_W<CTRL_SPEC> {
        MIRQDN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mask Interrupt Request for Error"]
    #[inline(always)]
    #[must_use]
    pub fn mirqerr(&mut self) -> MIRQERR_W<CTRL_SPEC> {
        MIRQERR_W::new(self, 6)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
