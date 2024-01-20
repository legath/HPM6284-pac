#[doc = "Register `error` reader"]
pub type R = crate::R<ERROR_SPEC>;
#[doc = "Register `error` writer"]
pub type W = crate::W<ERROR_SPEC>;
#[doc = "Field `BIT_ERROR` reader - bit error"]
pub type BIT_ERROR_R = crate::BitReader;
#[doc = "Field `CHK_ERROR` reader - checksum error"]
pub type CHK_ERROR_R = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - timeout error. The master detects a timeout error if it is expecting data from the bus but no slave does respond. The slave detects a timeout error if it is requesting a data acknowledge to the host controller. The slave detects a timeout if it has transmitted a wakeup signal and it detects no sync field within 150ms"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `PARITY_ERROR` reader - slave only. identifier parity error"]
pub type PARITY_ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - bit error"]
    #[inline(always)]
    pub fn bit_error(&self) -> BIT_ERROR_R {
        BIT_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - checksum error"]
    #[inline(always)]
    pub fn chk_error(&self) -> CHK_ERROR_R {
        CHK_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - timeout error. The master detects a timeout error if it is expecting data from the bus but no slave does respond. The slave detects a timeout error if it is requesting a data acknowledge to the host controller. The slave detects a timeout if it has transmitted a wakeup signal and it detects no sync field within 150ms"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - slave only. identifier parity error"]
    #[inline(always)]
    pub fn parity_error(&self) -> PARITY_ERROR_R {
        PARITY_ERROR_R::new(((self.bits >> 3) & 1) != 0)
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
#[doc = "error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERROR_SPEC;
impl crate::RegisterSpec for ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error::R`](R) reader structure"]
impl crate::Readable for ERROR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`error::W`](W) writer structure"]
impl crate::Writable for ERROR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets error to value 0"]
impl crate::Resettable for ERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
