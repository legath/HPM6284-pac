#[doc = "Register `BUS_RESULT[%s]` reader"]
pub type R = crate::R<BUS_RESULT_SPEC>;
#[doc = "Register `BUS_RESULT[%s]` writer"]
pub type W = crate::W<BUS_RESULT_SPEC>;
#[doc = "Field `CHAN_RESULT` reader - read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
pub type CHAN_RESULT_R = crate::FieldReader<u16>;
#[doc = "Field `VALID` reader - set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
pub type VALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    #[inline(always)]
    pub fn chan_result(&self) -> CHAN_RESULT_R {
        CHAN_RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 16) & 1) != 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_RESULT_SPEC;
impl crate::RegisterSpec for BUS_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_result::R`](R) reader structure"]
impl crate::Readable for BUS_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_result::W`](W) writer structure"]
impl crate::Writable for BUS_RESULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUS_RESULT[%s]
to value 0"]
impl crate::Resettable for BUS_RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
