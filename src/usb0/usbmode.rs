#[doc = "Register `USBMODE` reader"]
pub type R = crate::R<USBMODE_SPEC>;
#[doc = "Register `USBMODE` writer"]
pub type W = crate::W<USBMODE_SPEC>;
#[doc = "Field `CM` reader - CM Controller Mode - R/WO. Controller mode is defaulted to the proper mode for host only and device only implementations. For those designs that contain both host &amp; device capability, the controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. For combination host/ device controllers, this register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register. For OTG controller core, reset value is '00b'. 00 - Idle \\[Default for combination host/device\\]
01 - Reserved 10 - Device Controller \\[Default for device only controller\\]
11 - Host Controller \\[Default for host only controller\\]"]
pub type CM_R = crate::FieldReader;
#[doc = "Field `CM` writer - CM Controller Mode - R/WO. Controller mode is defaulted to the proper mode for host only and device only implementations. For those designs that contain both host &amp; device capability, the controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. For combination host/ device controllers, this register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register. For OTG controller core, reset value is '00b'. 00 - Idle \\[Default for combination host/device\\]
01 - Reserved 10 - Device Controller \\[Default for device only controller\\]
11 - Host Controller \\[Default for host only controller\\]"]
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ES` reader - ES Endian Select - Read/Write. This bit can change the byte alignment of the transfer buffers to match the host microprocessor. The bit fields in the microprocessor interface and the data structures are unaffected by the value of this bit because they are based upon the 32-bit word. Bit Meaning 0 - Little Endian \\[Default\\]
1 - Big Endian"]
pub type ES_R = crate::BitReader;
#[doc = "Field `ES` writer - ES Endian Select - Read/Write. This bit can change the byte alignment of the transfer buffers to match the host microprocessor. The bit fields in the microprocessor interface and the data structures are unaffected by the value of this bit because they are based upon the 32-bit word. Bit Meaning 0 - Little Endian \\[Default\\]
1 - Big Endian"]
pub type ES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOM` reader - SLOM Setup Lockout Mode. In device mode, this bit controls behavior of the setup lock mechanism. See Control Endpoint Operation Model . 0 - Setup Lockouts On (default); 1 - Setup Lockouts Off. DCD requires use of Setup Data Buffer Tripwire in USBCMD."]
pub type SLOM_R = crate::BitReader;
#[doc = "Field `SLOM` writer - SLOM Setup Lockout Mode. In device mode, this bit controls behavior of the setup lock mechanism. See Control Endpoint Operation Model . 0 - Setup Lockouts On (default); 1 - Setup Lockouts Off. DCD requires use of Setup Data Buffer Tripwire in USBCMD."]
pub type SLOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIS` reader - SDIS Stream Disable Mode. (0 - Inactive \\[default\\]; 1 - Active) Device Mode: Setting to a '1' disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received are responded to with a NYET handshake when stream disable is active. Host Mode: Setting to a '1' ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the TX latency is filled to capacity before the packet is launched onto the USB. NOTE: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING and TXTTFILLTUNING \\[MPH Only\\]
to characterize the adjustments needed for the scheduler when using this feature. NOTE: The use of this feature substantially limits of the overall USB performance that can be achieved."]
pub type SDIS_R = crate::BitReader;
#[doc = "Field `SDIS` writer - SDIS Stream Disable Mode. (0 - Inactive \\[default\\]; 1 - Active) Device Mode: Setting to a '1' disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received are responded to with a NYET handshake when stream disable is active. Host Mode: Setting to a '1' ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the TX latency is filled to capacity before the packet is launched onto the USB. NOTE: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING and TXTTFILLTUNING \\[MPH Only\\]
to characterize the adjustments needed for the scheduler when using this feature. NOTE: The use of this feature substantially limits of the overall USB performance that can be achieved."]
pub type SDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CM Controller Mode - R/WO. Controller mode is defaulted to the proper mode for host only and device only implementations. For those designs that contain both host &amp; device capability, the controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. For combination host/ device controllers, this register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register. For OTG controller core, reset value is '00b'. 00 - Idle \\[Default for combination host/device\\]
01 - Reserved 10 - Device Controller \\[Default for device only controller\\]
11 - Host Controller \\[Default for host only controller\\]"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ES Endian Select - Read/Write. This bit can change the byte alignment of the transfer buffers to match the host microprocessor. The bit fields in the microprocessor interface and the data structures are unaffected by the value of this bit because they are based upon the 32-bit word. Bit Meaning 0 - Little Endian \\[Default\\]
1 - Big Endian"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SLOM Setup Lockout Mode. In device mode, this bit controls behavior of the setup lock mechanism. See Control Endpoint Operation Model . 0 - Setup Lockouts On (default); 1 - Setup Lockouts Off. DCD requires use of Setup Data Buffer Tripwire in USBCMD."]
    #[inline(always)]
    pub fn slom(&self) -> SLOM_R {
        SLOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SDIS Stream Disable Mode. (0 - Inactive \\[default\\]; 1 - Active) Device Mode: Setting to a '1' disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received are responded to with a NYET handshake when stream disable is active. Host Mode: Setting to a '1' ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the TX latency is filled to capacity before the packet is launched onto the USB. NOTE: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING and TXTTFILLTUNING \\[MPH Only\\]
to characterize the adjustments needed for the scheduler when using this feature. NOTE: The use of this feature substantially limits of the overall USB performance that can be achieved."]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CM Controller Mode - R/WO. Controller mode is defaulted to the proper mode for host only and device only implementations. For those designs that contain both host &amp; device capability, the controller defaults to an idle state and needs to be initialized to the desired operating mode after reset. For combination host/ device controllers, this register can only be written once after reset. If it is necessary to switch modes, software must reset the controller by writing to the RESET bit in the USBCMD register before reprogramming this register. For OTG controller core, reset value is '00b'. 00 - Idle \\[Default for combination host/device\\]
01 - Reserved 10 - Device Controller \\[Default for device only controller\\]
11 - Host Controller \\[Default for host only controller\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<USBMODE_SPEC> {
        CM_W::new(self, 0)
    }
    #[doc = "Bit 2 - ES Endian Select - Read/Write. This bit can change the byte alignment of the transfer buffers to match the host microprocessor. The bit fields in the microprocessor interface and the data structures are unaffected by the value of this bit because they are based upon the 32-bit word. Bit Meaning 0 - Little Endian \\[Default\\]
1 - Big Endian"]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> ES_W<USBMODE_SPEC> {
        ES_W::new(self, 2)
    }
    #[doc = "Bit 3 - SLOM Setup Lockout Mode. In device mode, this bit controls behavior of the setup lock mechanism. See Control Endpoint Operation Model . 0 - Setup Lockouts On (default); 1 - Setup Lockouts Off. DCD requires use of Setup Data Buffer Tripwire in USBCMD."]
    #[inline(always)]
    #[must_use]
    pub fn slom(&mut self) -> SLOM_W<USBMODE_SPEC> {
        SLOM_W::new(self, 3)
    }
    #[doc = "Bit 4 - SDIS Stream Disable Mode. (0 - Inactive \\[default\\]; 1 - Active) Device Mode: Setting to a '1' disables double priming on both RX and TX for low bandwidth systems. This mode ensures that when the RX and TX buffers are sufficient to contain an entire packet that the standard double buffering scheme is disabled to prevent overruns/underruns in bandwidth limited systems. Note: In High Speed Mode, all packets received are responded to with a NYET handshake when stream disable is active. Host Mode: Setting to a '1' ensures that overruns/underruns of the latency FIFO are eliminated for low bandwidth systems where the RX and TX buffers are sufficient to contain the entire packet. Enabling stream disable also has the effect of ensuring the TX latency is filled to capacity before the packet is launched onto the USB. NOTE: Time duration to pre-fill the FIFO becomes significant when stream disable is active. See TXFILLTUNING and TXTTFILLTUNING \\[MPH Only\\]
to characterize the adjustments needed for the scheduler when using this feature. NOTE: The use of this feature substantially limits of the overall USB performance that can be achieved."]
    #[inline(always)]
    #[must_use]
    pub fn sdis(&mut self) -> SDIS_W<USBMODE_SPEC> {
        SDIS_W::new(self, 4)
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
#[doc = "USB Device Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBMODE_SPEC;
impl crate::RegisterSpec for USBMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbmode::R`](R) reader structure"]
impl crate::Readable for USBMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbmode::W`](W) writer structure"]
impl crate::Writable for USBMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBMODE to value 0"]
impl crate::Resettable for USBMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
