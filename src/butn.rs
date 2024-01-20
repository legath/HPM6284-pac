#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    btn_status: BTN_STATUS,
    btn_irq_mask: BTN_IRQ_MASK,
    led_intense: LED_INTENSE,
}
impl RegisterBlock {
    #[doc = "0x00 - Button status"]
    #[inline(always)]
    pub const fn btn_status(&self) -> &BTN_STATUS {
        &self.btn_status
    }
    #[doc = "0x04 - Button interrupt mask"]
    #[inline(always)]
    pub const fn btn_irq_mask(&self) -> &BTN_IRQ_MASK {
        &self.btn_irq_mask
    }
    #[doc = "0x08 - Debounce setting"]
    #[inline(always)]
    pub const fn led_intense(&self) -> &LED_INTENSE {
        &self.led_intense
    }
}
#[doc = "BTN_STATUS (rw) register accessor: Button status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btn_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btn_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btn_status`]
module"]
pub type BTN_STATUS = crate::Reg<btn_status::BTN_STATUS_SPEC>;
#[doc = "Button status"]
pub mod btn_status;
#[doc = "BTN_IRQ_MASK (rw) register accessor: Button interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btn_irq_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btn_irq_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btn_irq_mask`]
module"]
pub type BTN_IRQ_MASK = crate::Reg<btn_irq_mask::BTN_IRQ_MASK_SPEC>;
#[doc = "Button interrupt mask"]
pub mod btn_irq_mask;
#[doc = "LED_INTENSE (rw) register accessor: Debounce setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`led_intense::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`led_intense::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@led_intense`]
module"]
pub type LED_INTENSE = crate::Reg<led_intense::LED_INTENSE_SPEC>;
#[doc = "Debounce setting"]
pub mod led_intense;
