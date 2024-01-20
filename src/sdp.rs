#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sdpcr: SDPCR,
    modctrl: MODCTRL,
    pktcnt: PKTCNT,
    sta: STA,
    keyaddr: KEYADDR,
    keydat: KEYDAT,
    ciphiv: [CIPHIV; 4],
    haswrd: [HASWRD; 8],
    cmdptr: CMDPTR,
    npktptr: NPKTPTR,
    pktctl: PKTCTL,
    pktsrc: PKTSRC,
    pktdst: PKTDST,
    pktbuf: PKTBUF,
}
impl RegisterBlock {
    #[doc = "0x00 - SDP control register"]
    #[inline(always)]
    pub const fn sdpcr(&self) -> &SDPCR {
        &self.sdpcr
    }
    #[doc = "0x04 - Mod control register."]
    #[inline(always)]
    pub const fn modctrl(&self) -> &MODCTRL {
        &self.modctrl
    }
    #[doc = "0x08 - packet counter registers."]
    #[inline(always)]
    pub const fn pktcnt(&self) -> &PKTCNT {
        &self.pktcnt
    }
    #[doc = "0x0c - Status Registers"]
    #[inline(always)]
    pub const fn sta(&self) -> &STA {
        &self.sta
    }
    #[doc = "0x10 - Key Address"]
    #[inline(always)]
    pub const fn keyaddr(&self) -> &KEYADDR {
        &self.keyaddr
    }
    #[doc = "0x14 - Key Data"]
    #[inline(always)]
    pub const fn keydat(&self) -> &KEYDAT {
        &self.keydat
    }
    #[doc = "0x18..0x28 - no description available"]
    #[inline(always)]
    pub const fn ciphiv(&self, n: usize) -> &CIPHIV {
        &self.ciphiv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x28 - no description available"]
    #[inline(always)]
    pub fn ciphiv_iter(&self) -> impl Iterator<Item = &CIPHIV> {
        self.ciphiv.iter()
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn ciphivciphiv0(&self) -> &CIPHIV {
        self.ciphiv(0)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn ciphivciphiv1(&self) -> &CIPHIV {
        self.ciphiv(1)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn ciphivciphiv2(&self) -> &CIPHIV {
        self.ciphiv(2)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn ciphivciphiv3(&self) -> &CIPHIV {
        self.ciphiv(3)
    }
    #[doc = "0x28..0x48 - no description available"]
    #[inline(always)]
    pub const fn haswrd(&self, n: usize) -> &HASWRD {
        &self.haswrd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x48 - no description available"]
    #[inline(always)]
    pub fn haswrd_iter(&self) -> impl Iterator<Item = &HASWRD> {
        self.haswrd.iter()
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn haswrdhaswrd0(&self) -> &HASWRD {
        self.haswrd(0)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn haswrdhaswrd1(&self) -> &HASWRD {
        self.haswrd(1)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn haswrdhaswrd2(&self) -> &HASWRD {
        self.haswrd(2)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn haswrdhaswrd3(&self) -> &HASWRD {
        self.haswrd(3)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn haswrdhaswrd4(&self) -> &HASWRD {
        self.haswrd(4)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn haswrdhaswrd5(&self) -> &HASWRD {
        self.haswrd(5)
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn haswrdhaswrd6(&self) -> &HASWRD {
        self.haswrd(6)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn haswrdhaswrd7(&self) -> &HASWRD {
        self.haswrd(7)
    }
    #[doc = "0x48 - Command Pointer"]
    #[inline(always)]
    pub const fn cmdptr(&self) -> &CMDPTR {
        &self.cmdptr
    }
    #[doc = "0x4c - Next Packet Address Pointer"]
    #[inline(always)]
    pub const fn npktptr(&self) -> &NPKTPTR {
        &self.npktptr
    }
    #[doc = "0x50 - Packet Control Registers"]
    #[inline(always)]
    pub const fn pktctl(&self) -> &PKTCTL {
        &self.pktctl
    }
    #[doc = "0x54 - Packet Memory Source Address"]
    #[inline(always)]
    pub const fn pktsrc(&self) -> &PKTSRC {
        &self.pktsrc
    }
    #[doc = "0x58 - Packet Memory Destination Address"]
    #[inline(always)]
    pub const fn pktdst(&self) -> &PKTDST {
        &self.pktdst
    }
    #[doc = "0x5c - Packet buffer size."]
    #[inline(always)]
    pub const fn pktbuf(&self) -> &PKTBUF {
        &self.pktbuf
    }
}
#[doc = "SDPCR (rw) register accessor: SDP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdpcr`]
module"]
pub type SDPCR = crate::Reg<sdpcr::SDPCR_SPEC>;
#[doc = "SDP control register"]
pub mod sdpcr;
#[doc = "MODCTRL (rw) register accessor: Mod control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modctrl`]
module"]
pub type MODCTRL = crate::Reg<modctrl::MODCTRL_SPEC>;
#[doc = "Mod control register."]
pub mod modctrl;
#[doc = "PKTCNT (rw) register accessor: packet counter registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktcnt`]
module"]
pub type PKTCNT = crate::Reg<pktcnt::PKTCNT_SPEC>;
#[doc = "packet counter registers."]
pub mod pktcnt;
#[doc = "STA (rw) register accessor: Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status Registers"]
pub mod sta;
#[doc = "KEYADDR (rw) register accessor: Key Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyaddr`]
module"]
pub type KEYADDR = crate::Reg<keyaddr::KEYADDR_SPEC>;
#[doc = "Key Address"]
pub mod keyaddr;
#[doc = "KEYDAT (rw) register accessor: Key Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keydat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keydat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keydat`]
module"]
pub type KEYDAT = crate::Reg<keydat::KEYDAT_SPEC>;
#[doc = "Key Data"]
pub mod keydat;
#[doc = "CIPHIV (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ciphiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ciphiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ciphiv`]
module"]
pub type CIPHIV = crate::Reg<ciphiv::CIPHIV_SPEC>;
#[doc = "no description available"]
pub mod ciphiv;
#[doc = "HASWRD (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haswrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haswrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haswrd`]
module"]
pub type HASWRD = crate::Reg<haswrd::HASWRD_SPEC>;
#[doc = "no description available"]
pub mod haswrd;
#[doc = "CMDPTR (rw) register accessor: Command Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdptr`]
module"]
pub type CMDPTR = crate::Reg<cmdptr::CMDPTR_SPEC>;
#[doc = "Command Pointer"]
pub mod cmdptr;
#[doc = "NPKTPTR (rw) register accessor: Next Packet Address Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npktptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npktptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npktptr`]
module"]
pub type NPKTPTR = crate::Reg<npktptr::NPKTPTR_SPEC>;
#[doc = "Next Packet Address Pointer"]
pub mod npktptr;
#[doc = "PKTCTL (rw) register accessor: Packet Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktctl`]
module"]
pub type PKTCTL = crate::Reg<pktctl::PKTCTL_SPEC>;
#[doc = "Packet Control Registers"]
pub mod pktctl;
#[doc = "PKTSRC (rw) register accessor: Packet Memory Source Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktsrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktsrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktsrc`]
module"]
pub type PKTSRC = crate::Reg<pktsrc::PKTSRC_SPEC>;
#[doc = "Packet Memory Source Address"]
pub mod pktsrc;
#[doc = "PKTDST (rw) register accessor: Packet Memory Destination Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdst`]
module"]
pub type PKTDST = crate::Reg<pktdst::PKTDST_SPEC>;
#[doc = "Packet Memory Destination Address"]
pub mod pktdst;
#[doc = "PKTBUF (rw) register accessor: Packet buffer size.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktbuf`]
module"]
pub type PKTBUF = crate::Reg<pktbuf::PKTBUF_SPEC>;
#[doc = "Packet buffer size."]
pub mod pktbuf;
