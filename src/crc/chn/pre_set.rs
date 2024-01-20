#[doc = "Register `pre_set` reader"]
pub type R = crate::R<PRE_SET_SPEC>;
#[doc = "Register `pre_set` writer"]
pub type W = crate::W<PRE_SET_SPEC>;
#[doc = "Field `PRE_SET` reader - 0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb"]
pub type PRE_SET_R = crate::FieldReader;
#[doc = "Field `PRE_SET` writer - 0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb"]
pub type PRE_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb"]
    #[inline(always)]
    pub fn pre_set(&self) -> PRE_SET_R {
        PRE_SET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb"]
    #[inline(always)]
    #[must_use]
    pub fn pre_set(&mut self) -> PRE_SET_W<PRE_SET_SPEC> {
        PRE_SET_W::new(self, 0)
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
#[doc = "&amp;index0 pre set for crc setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pre_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pre_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRE_SET_SPEC;
impl crate::RegisterSpec for PRE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pre_set::R`](R) reader structure"]
impl crate::Readable for PRE_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pre_set::W`](W) writer structure"]
impl crate::Writable for PRE_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pre_set to value 0"]
impl crate::Resettable for PRE_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
