#[doc = "Register `GFC` reader"]
pub type R = crate::R<GFC_SPEC>;
#[doc = "Register `GFC` writer"]
pub type W = crate::W<GFC_SPEC>;
#[doc = "Field `RRFE` reader - Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs"]
pub type RRFE_R = crate::BitReader;
#[doc = "Field `RRFE` writer - Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs"]
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs"]
pub type RRFS_R = crate::BitReader;
#[doc = "Field `RRFS` writer - Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs"]
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` reader - Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
pub type ANFE_R = crate::FieldReader;
#[doc = "Field `ANFE` writer - Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` reader - Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
pub type ANFS_R = crate::FieldReader;
#[doc = "Field `ANFS` writer - Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<GFC_SPEC> {
        RRFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<GFC_SPEC> {
        RRFS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<GFC_SPEC> {
        ANFE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<GFC_SPEC> {
        ANFS_W::new(self, 4)
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
#[doc = "global filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GFC_SPEC;
impl crate::RegisterSpec for GFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfc::R`](R) reader structure"]
impl crate::Readable for GFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gfc::W`](W) writer structure"]
impl crate::Writable for GFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
