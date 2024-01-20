#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    txreg: TXREG,
    rxreg: RXREG,
    txwrd: [TXWRD; 4],
    rxwrd: [RXWRD; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Command Registers"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Status Registers"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - Transmit word message to other core."]
    #[inline(always)]
    pub const fn txreg(&self) -> &TXREG {
        &self.txreg
    }
    #[doc = "0x0c - Receive word message from other core."]
    #[inline(always)]
    pub const fn rxreg(&self) -> &RXREG {
        &self.rxreg
    }
    #[doc = "0x10..0x20 - no description available"]
    #[inline(always)]
    pub const fn txwrd(&self, n: usize) -> &TXWRD {
        &self.txwrd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - no description available"]
    #[inline(always)]
    pub fn txwrd_iter(&self) -> impl Iterator<Item = &TXWRD> {
        self.txwrd.iter()
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn txwrdtxfifo0(&self) -> &TXWRD {
        self.txwrd(0)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn txwrdtxfifo1(&self) -> &TXWRD {
        self.txwrd(1)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn txwrdtxfifo2(&self) -> &TXWRD {
        self.txwrd(2)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn txwrdtxfifo3(&self) -> &TXWRD {
        self.txwrd(3)
    }
    #[doc = "0x20..0x30 - no description available"]
    #[inline(always)]
    pub const fn rxwrd(&self, n: usize) -> &RXWRD {
        &self.rxwrd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - no description available"]
    #[inline(always)]
    pub fn rxwrd_iter(&self) -> impl Iterator<Item = &RXWRD> {
        self.rxwrd.iter()
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn rxwrdrxfifo0(&self) -> &RXWRD {
        self.rxwrd(0)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn rxwrdrxfifo1(&self) -> &RXWRD {
        self.rxwrd(1)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn rxwrdrxfifo2(&self) -> &RXWRD {
        self.rxwrd(2)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn rxwrdrxfifo3(&self) -> &RXWRD {
        self.rxwrd(3)
    }
}
#[doc = "CR (rw) register accessor: Command Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Command Registers"]
pub mod cr;
#[doc = "SR (rw) register accessor: Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Registers"]
pub mod sr;
#[doc = "TXREG (rw) register accessor: Transmit word message to other core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txreg`]
module"]
pub type TXREG = crate::Reg<txreg::TXREG_SPEC>;
#[doc = "Transmit word message to other core."]
pub mod txreg;
#[doc = "RXREG (rw) register accessor: Receive word message from other core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxreg`]
module"]
pub type RXREG = crate::Reg<rxreg::RXREG_SPEC>;
#[doc = "Receive word message from other core."]
pub mod rxreg;
#[doc = "TXWRD (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txwrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txwrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txwrd`]
module"]
pub type TXWRD = crate::Reg<txwrd::TXWRD_SPEC>;
#[doc = "no description available"]
pub mod txwrd;
#[doc = "RXWRD (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxwrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxwrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxwrd`]
module"]
pub type RXWRD = crate::Reg<rxwrd::RXWRD_SPEC>;
#[doc = "no description available"]
pub mod rxwrd;
