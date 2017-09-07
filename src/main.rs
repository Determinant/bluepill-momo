#![no_std]

extern crate bluepill_usbcdc;
extern crate r0;
#[macro_use] extern crate stm32f103xx;
use bluepill_usbcdc::*;

/* setup interrupts */
exception!(NMI, nmi_handler);
exception!(HARD_FAULT, hardfault_handler);
//exception!(MEM_MANAGE, mem_manage_handler);
exception!(BUS_FAULT, bus_fault_handler);
exception!(SVCALL, svc_handler);
exception!(PENDSV, pend_sv_handler);
exception!(SYS_TICK, systick_handler);
interrupt!(CAN1_RX0, usb_lp_can1_rx0_irqhandler);

fn bss_init_bugfix() {
    extern "C" {
        // Boundaries of the .bss section
        static mut _ebss: u32;
        static mut _sdata: u32;
    }
    unsafe {
        r0::zero_bss(&mut _ebss, &mut _sdata);
    }
}

fn main() {
    bss_init_bugfix();
    hal_init();
    system_clock_config();
    //gpio_init();
    usb_init();
    const MOMO: &str = "momo ";
    let mut cdc_send_data: [u8; 64] = [0; 64];
    let mut cdc_cmd_data: [u8; 64] = [0; 64];
    let mut cdc_send_len: usize = 0;
    let mut cdc_cmd_len: usize = 5;
    cdc_cmd_data[..5].clone_from_slice(MOMO.as_bytes());
    loop {
        /* copy the received data from C buffer */
        unsafe {
            if cdc_recv_len > 0 {
                let l = cdc_recv_len as usize;
                cdc_cmd_data[cdc_cmd_len..cdc_cmd_len+l]
                    .clone_from_slice(&cdc_recv_data[..l]);
                cdc_send_data[cdc_send_len..cdc_send_len+l]
                    .clone_from_slice(&cdc_recv_data[..l]);
                cdc_cmd_len += l;
                cdc_send_len += l;
                cdc_recv_len = 0;
            }
        }

        if cdc_cmd_len > 0 && cdc_cmd_data[cdc_cmd_len - 1] == b'\r'
        {
            cdc_send_data[cdc_send_len] = '\n' as u8;
            cdc_send_len += 1;
            cdc_send_data[cdc_send_len..cdc_send_len+cdc_cmd_len]
                .clone_from_slice(&cdc_cmd_data[..cdc_cmd_len]);
            cdc_send_len += cdc_cmd_len;
            cdc_send_data[cdc_send_len] = '\n' as u8;
            cdc_send_len += 1;
            cdc_cmd_len = 5;
        }

        if cdc_send_len > 0 {
            while !cdc_send(&mut cdc_send_data, cdc_send_len) {
                hal_delay(100);
            }
            cdc_send_len = 0;
        }
    }
}
