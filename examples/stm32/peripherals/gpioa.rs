use core::sync::atomic::AtomicPtr;

use vcell::VolatileCell;

pub mod moder;

#[doc = "General-purpose I/Os"]
static mut REGISTER_BLOCK: AtomicPtr<RegisterBlock> = AtomicPtr::new(1207959552 as *mut RegisterBlock);

pub(crate) fn ptr() -> &'static RegisterBlock {
    unsafe {
        &**REGISTER_BLOCK.get_mut()
    }
}

#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    moder: VolatileCell<u32>,
}

impl RegisterBlock {
    pub fn moder(&self) -> impl moder::Moder<Value=u32> + '_ {
        moder::moder(&self.moder)
    }
}