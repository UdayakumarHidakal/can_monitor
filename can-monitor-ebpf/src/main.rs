#![no_std]
#![no_main]

use aya_bpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_log_ebpf::info;
use can_monitor_common::CanFrame;
use core::{mem, str::Bytes};
// use can::{constants, frame::Frame, identifier};
// use socketcan::CanFrame;
// use embedded_can::Id;

#[xdp]
pub fn can_monitor(ctx: XdpContext) -> u32 {
    match try_can_monitor(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)] // 
fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

fn try_can_monitor(ctx: XdpContext) -> Result<u32, ()> {
    let can_frame: *const CanFrame  = ptr_at(&ctx, 0)?;
    let id =  u32::from_be(unsafe { (*can_frame).can_id });
    let dlc =  u8::from_be(unsafe { (*can_frame).can_dlc });
    let data = (unsafe {
        (*can_frame).data
    });

    // let data = ctx.data();
    // let data_len = ctx.data_end() - ctx.data();
    // let pkt_info = unsafe { ctx.data() };

    info!(&ctx, "received a packet {:x}", id);
    info!(&ctx, "received a packet dlc {:x}", dlc);
    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
