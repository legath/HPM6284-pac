#[doc = r"Register block"]
#[repr(C)]
pub struct CHCTRL {
    ctrl: CTRL,
    tran_size: TRAN_SIZE,
    src_addr: SRC_ADDR,
    src_addr_h: SRC_ADDR_H,
    dst_addr: DST_ADDR,
    dst_addr_h: DST_ADDR_H,
    llpointer: LLPOINTER,
    llpointer_h: LLPOINTER_H,
}
impl CHCTRL {
    #[doc = "0x00 - Channel n Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Channel n Transfer Size Register"]
    #[inline(always)]
    pub const fn tran_size(&self) -> &TRAN_SIZE {
        &self.tran_size
    }
    #[doc = "0x08 - Channel n Source Address Low Part Register"]
    #[inline(always)]
    pub const fn src_addr(&self) -> &SRC_ADDR {
        &self.src_addr
    }
    #[doc = "0x0c - Channel n Source Address High Part Register"]
    #[inline(always)]
    pub const fn src_addr_h(&self) -> &SRC_ADDR_H {
        &self.src_addr_h
    }
    #[doc = "0x10 - Channel n Destination Address Low Part Register"]
    #[inline(always)]
    pub const fn dst_addr(&self) -> &DST_ADDR {
        &self.dst_addr
    }
    #[doc = "0x14 - Channel n Destination Address High Part Register"]
    #[inline(always)]
    pub const fn dst_addr_h(&self) -> &DST_ADDR_H {
        &self.dst_addr_h
    }
    #[doc = "0x18 - Channel n Linked List Pointer Low Part Register"]
    #[inline(always)]
    pub const fn llpointer(&self) -> &LLPOINTER {
        &self.llpointer
    }
    #[doc = "0x1c - Channel n Linked List Pointer High Part Register"]
    #[inline(always)]
    pub const fn llpointer_h(&self) -> &LLPOINTER_H {
        &self.llpointer_h
    }
}
#[doc = "Ctrl (rw) register accessor: Channel n Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod ctrl;
#[doc = "TranSize (rw) register accessor: Channel n Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tran_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tran_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tran_size`]
module"]
pub type TRAN_SIZE = crate::Reg<tran_size::TRAN_SIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod tran_size;
#[doc = "SrcAddr (rw) register accessor: Channel n Source Address Low Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr`]
module"]
pub type SRC_ADDR = crate::Reg<src_addr::SRC_ADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod src_addr;
#[doc = "SrcAddrH (rw) register accessor: Channel n Source Address High Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_h`]
module"]
pub type SRC_ADDR_H = crate::Reg<src_addr_h::SRC_ADDR_H_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod src_addr_h;
#[doc = "DstAddr (rw) register accessor: Channel n Destination Address Low Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr`]
module"]
pub type DST_ADDR = crate::Reg<dst_addr::DST_ADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod dst_addr;
#[doc = "DstAddrH (rw) register accessor: Channel n Destination Address High Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_h`]
module"]
pub type DST_ADDR_H = crate::Reg<dst_addr_h::DST_ADDR_H_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod dst_addr_h;
#[doc = "LLPointer (rw) register accessor: Channel n Linked List Pointer Low Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llpointer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llpointer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llpointer`]
module"]
pub type LLPOINTER = crate::Reg<llpointer::LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod llpointer;
#[doc = "LLPointerH (rw) register accessor: Channel n Linked List Pointer High Part Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llpointer_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llpointer_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llpointer_h`]
module"]
pub type LLPOINTER_H = crate::Reg<llpointer_h::LLPOINTER_H_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod llpointer_h;
