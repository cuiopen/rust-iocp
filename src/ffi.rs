#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use libc::{HANDLE, c_ulong, DWORD, LPDWORD, LPOVERLAPPED, BOOL};

#[cfg(target_arch = "x86")]
pub type ULONG_PTR = c_ulong;
#[cfg(target_arch = "x86_64")]
pub type ULONG_PTR = u64;

pub type ULONG = c_ulong;

pub type PULONG = *mut ULONG;

pub type PULONG_PTR = *mut ULONG_PTR;

extern "system" {
	pub fn CreateIoCompletionPort(FileHandle: HANDLE, ExistingCompletionPort: HANDLE, CompletionKey: ULONG_PTR, NumberOfConcurrentThreads: DWORD) -> HANDLE;
	pub fn GetQueuedCompletionStatus(CompletionPort: HANDLE, lpNumberOfBytesTransferred: LPDWORD, lpCompletionKey: PULONG_PTR, lpOverlapped: *mut LPOVERLAPPED, dwMilliseconds: DWORD) -> BOOL;
	pub fn GetQueuedCompletionStatusEx(CompletionPort: HANDLE, lpCompletionPortEntries: LPOVERLAPPED_ENTRY, ulCount: ULONG, ulNumEntriesRemoved: PULONG, dwMilliseconds: DWORD, fAlertable: BOOL) -> BOOL;
	pub fn PostQueuedCompletionStatus(CompletionPort: HANDLE, dwNumberOfBytesTransferred: DWORD, dwCompletionKey: ULONG_PTR, lpOverlapped: LPOVERLAPPED) -> BOOL;
}

#[repr(C)]
pub struct OVERLAPPED_ENTRY {
	pub lpCompletionKey: ULONG_PTR,
	pub lpOverlapped: LPOVERLAPPED,
	pub Internal: ULONG_PTR,
	pub dwNumberOfBytesTransferred: DWORD
}

pub type LPOVERLAPPED_ENTRY = *mut OVERLAPPED_ENTRY;