#[doc = "Register `DEVICEADDR` reader"]
pub type R = crate::R<DEVICEADDR_SPEC>;
#[doc = "Register `DEVICEADDR` writer"]
pub type W = crate::W<DEVICEADDR_SPEC>;
#[doc = "Field `USBADRA` reader - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement."]
pub type USBADRA_R = crate::BitReader;
#[doc = "Field `USBADRA` writer - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement."]
pub type USBADRA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBADR` reader - USBADR Device Address. These bits correspond to the USB device address"]
pub type USBADR_R = crate::FieldReader;
#[doc = "Field `USBADR` writer - USBADR Device Address. These bits correspond to the USB device address"]
pub type USBADR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 24 - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement."]
    #[inline(always)]
    pub fn usbadra(&self) -> USBADRA_R {
        USBADRA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - USBADR Device Address. These bits correspond to the USB device address"]
    #[inline(always)]
    pub fn usbadr(&self) -> USBADR_R {
        USBADR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - USBADRA Device Address Advance. Default=0. When this bit is '0', any writes to USBADR are instantaneous. When this bit is written to a '1' at the same time or before USBADR is written, the write to the USBADR field is staged and held in a hidden register. After an IN occurs on endpoint 0 and is ACKed, USBADR will be loaded from the holding register. Hardware will automatically clear this bit on the following conditions: 1) IN is ACKed to endpoint 0. (USBADR is updated from staging register). 2) OUT/SETUP occur to endpoint 0. (USBADR is not updated). 3) Device Reset occurs (USBADR is reset to 0). NOTE: After the status phase of the SET_ADDRESS descriptor, the DCD has 2 ms to program the USBADR field. This mechanism will ensure this specification is met when the DCD can not write of the device address within 2ms from the SET_ADDRESS status phase. If the DCD writes the USBADR with USBADRA=1 after the SET_ADDRESS data phase (before the prime of the status phase), the USBADR will be programmed instantly at the correct time and meet the 2ms USB requirement."]
    #[inline(always)]
    #[must_use]
    pub fn usbadra(&mut self) -> USBADRA_W<DEVICEADDR_SPEC> {
        USBADRA_W::new(self, 24)
    }
    #[doc = "Bits 25:31 - USBADR Device Address. These bits correspond to the USB device address"]
    #[inline(always)]
    #[must_use]
    pub fn usbadr(&mut self) -> USBADR_W<DEVICEADDR_SPEC> {
        USBADR_W::new(self, 25)
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
#[doc = "Device Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deviceaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICEADDR_SPEC;
impl crate::RegisterSpec for DEVICEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deviceaddr::R`](R) reader structure"]
impl crate::Readable for DEVICEADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`deviceaddr::W`](W) writer structure"]
impl crate::Writable for DEVICEADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVICEADDR to value 0"]
impl crate::Resettable for DEVICEADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
