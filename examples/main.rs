#![feature(box_syntax)]
#![feature(libc)]
extern crate iocp;
extern crate libc;
extern crate num_cpus;
extern crate threadpool;
extern crate rand;

use iocp::{IoCompletionPort, CompletionStatus};
use std::{ptr, mem, thread};
use threadpool::ThreadPool;

fn main() {
	let iocp = IoCompletionPort::new(0).unwrap();
	
	let threads = num_cpus::get() * 2;
	let taskpool = ThreadPool::new(threads);
	
	for i in 0..threads {
		let iocp_clone = iocp.clone();
		taskpool.execute(move || {
			loop {
				thread::sleep_ms(100 * i as u32);
				let status = iocp_clone.get_queued(libc::INFINITE).unwrap();
				println!("Dequeued: {} from {} with {} {:p}", status.completion_key, i, status.byte_count, status.overlapped);
				
				// We re-box all this stuff so it gets freed
				let overlapped: Box<libc::OVERLAPPED> = unsafe { mem::transmute(status.overlapped) };
				let internal: Box<u32> = unsafe { mem::transmute(overlapped.Internal) };
				let internal_high: Box<u32> = unsafe { mem::transmute(overlapped.InternalHigh) };
				
				println!("Overlapped: {} {} {} {} {:p}", internal, internal_high, overlapped.Offset, overlapped.OffsetHigh, overlapped.hEvent);
				
				thread::sleep_ms(500);
			}
		});
	}
	
	loop {
		let internal = box 3u32;
		let internal_high = box 4u32;
		// Transmute the boxes so they're on the heap but don't disappear
		let overlapped = box libc::OVERLAPPED {
			Internal: unsafe { mem::transmute(internal) },
			InternalHigh: unsafe { mem::transmute(internal_high) },
			Offset: 200,
			OffsetHigh: 300,
			hEvent: ptr::null_mut()
		};
		let status = CompletionStatus {
			byte_count: 100,
			completion_key: rand::random(),
			overlapped: unsafe { mem::transmute(overlapped) }
		};
		println!("Queued: {}", status.completion_key);
		iocp.post_queued(status).unwrap();
		thread::sleep_ms(100);
	}
}
