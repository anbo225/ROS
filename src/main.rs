#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ros::println;
use x86_64::VirtAddr;
use core::panic::PanicInfo;

use bootloader::BootInfo;
use ros::memory::{active_level_4_table,translate_addr};

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe {
        active_level_4_table(phys_mem_offset)
    };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
           
            println!("L4 Entry {}: {:?}", i, entry);
            
        }
    }

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }


    // ros::init();

    // // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // #[cfg(test)]
    // test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}