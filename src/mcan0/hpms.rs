#[doc = "Register `HPMS` reader"]
pub type R = crate::R<HPMS_SPEC>;
#[doc = "Register `HPMS` writer"]
pub type W = crate::W<HPMS_SPEC>;
#[doc = "Field `BIDX` reader - Buffer Index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= ‘1’."]
pub type BIDX_R = crate::FieldReader;
#[doc = "Field `MSI` reader - Message Storage Indicator 00= No FIFO selected 01= FIFO message lost 10= Message stored in FIFO 0 11= Message stored in FIFO 1"]
pub type MSI_R = crate::FieldReader;
#[doc = "Field `FIDX` reader - Filter Index Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1."]
pub type FIDX_R = crate::FieldReader;
#[doc = "Field `FLST` reader - Filter List Indicates the filter list of the matching filter element. 0= Standard Filter List 1= Extended Filter List"]
pub type FLST_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Buffer Index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= ‘1’."]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator 00= No FIFO selected 01= FIFO message lost 10= Message stored in FIFO 0 11= Message stored in FIFO 1"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Filter Index Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1."]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter List Indicates the filter list of the matching filter element. 0= Standard Filter List 1= Extended Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
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
#[doc = "high priority message status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpms::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPMS_SPEC;
impl crate::RegisterSpec for HPMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpms::R`](R) reader structure"]
impl crate::Readable for HPMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpms::W`](W) writer structure"]
impl crate::Writable for HPMS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HPMS_SPEC {
    const RESET_VALUE: u32 = 0;
}
