#[doc = "Register `SlvSt` reader"]
pub type R = crate::R<SLV_ST_SPEC>;
#[doc = "Register `SlvSt` writer"]
pub type W = crate::W<SLV_ST_SPEC>;
#[doc = "Field `USR_STATUS` reader - User defined status flags"]
pub type USR_STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `USR_STATUS` writer - User defined status flags"]
pub type USR_STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `READY` reader - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
pub type READY_R = crate::BitReader;
#[doc = "Field `READY` writer - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
pub type READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - Data overrun occurs in the last transaction"]
pub type OVERRUN_R = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Data overrun occurs in the last transaction"]
pub type OVERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN` writer - Data underrun occurs in the last transaction"]
pub type UNDERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - User defined status flags"]
    #[inline(always)]
    pub fn usr_status(&self) -> USR_STATUS_R {
        USR_STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data overrun occurs in the last transaction"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User defined status flags"]
    #[inline(always)]
    #[must_use]
    pub fn usr_status(&mut self) -> USR_STATUS_W<SLV_ST_SPEC> {
        USR_STATUS_W::new(self, 0)
    }
    #[doc = "Bit 16 - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<SLV_ST_SPEC> {
        READY_W::new(self, 16)
    }
    #[doc = "Bit 17 - Data overrun occurs in the last transaction"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<SLV_ST_SPEC> {
        OVERRUN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Data underrun occurs in the last transaction"]
    #[inline(always)]
    #[must_use]
    pub fn underrun(&mut self) -> UNDERRUN_W<SLV_ST_SPEC> {
        UNDERRUN_W::new(self, 18)
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
#[doc = "Slave Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLV_ST_SPEC;
impl crate::RegisterSpec for SLV_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slv_st::R`](R) reader structure"]
impl crate::Readable for SLV_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slv_st::W`](W) writer structure"]
impl crate::Writable for SLV_ST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SlvSt to value 0"]
impl crate::Resettable for SLV_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
