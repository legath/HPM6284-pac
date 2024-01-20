#[doc = "Register `ECR` reader"]
pub type R = crate::R<ECR_SPEC>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<ECR_SPEC>;
#[doc = "Field `TEC` reader - Transmit Error Counter Actual state of the Transmit Error Counter, values between 0 and 255 Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Receive Error Counter Actual state of the Receive Error Counter, values between 0 and 127"]
pub type REC_R = crate::FieldReader;
#[doc = "Field `RP` reader - Receive Error Passive 0= The Receive Error Counter is below the error passive level of 128 1= The Receive Error Counter has reached the error passive level of 128"]
pub type RP_R = crate::BitReader;
#[doc = "Field `CEL` reader - CAN Error Logging The counter is incremented each time when a CAN protocol error causes the 8-bit Transmit Error Counter TEC or the 7-bit Receive Error Counter REC to be incremented. The counter is also incremented when the Bus_Off limit is reached. It is not incremented when only RP is set without changing REC. The increment of CEL follows after the increment of REC or TEC. The counter is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO. Note: Byte access: Reading byte 2 will reset CEL to zero, reading bytes 3/1/0 has no impact."]
pub type CEL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter Actual state of the Transmit Error Counter, values between 0 and 255 Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter Actual state of the Receive Error Counter, values between 0 and 127"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive 0= The Receive Error Counter is below the error passive level of 128 1= The Receive Error Counter has reached the error passive level of 128"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CAN Error Logging The counter is incremented each time when a CAN protocol error causes the 8-bit Transmit Error Counter TEC or the 7-bit Receive Error Counter REC to be incremented. The counter is also incremented when the Bus_Off limit is reached. It is not incremented when only RP is set without changing REC. The increment of CEL follows after the increment of REC or TEC. The counter is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO. Note: Byte access: Reading byte 2 will reset CEL to zero, reading bytes 3/1/0 has no impact."]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
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
#[doc = "error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for ECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: u32 = 0;
}
