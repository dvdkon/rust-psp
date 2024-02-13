use crate::sys::{sceIoDevctl, SceUid};
use core::{mem, ptr};

/// Event which has occurred in the memory stick ejection callback, passed in
/// `arg2`.
pub enum MsCbEvent {
	Inserted = 1,
	Ejected = 2,
}

/// Returns whether a memory stick is current inserted
///
/// # Return Value
///
/// 1 if memory stick inserted, 0 if not or if < 0 on error
#[inline(always)]
#[allow(non_snake_case)]
pub unsafe fn MScmIsMediumInserted() -> i32 {
	let mut status: i32 = 0;

	let ret = sceIoDevctl(
		b"mscmhc0:\0" as _,
		0x02025806,
		ptr::null_mut(),
		0,
		&mut status as *mut _ as _,
		mem::size_of::<i32>() as i32,
	);

	if ret < 0 {
		ret
	} else if status != 1 {
		0
	} else {
		1
	}
}

/// Registers a memory stick ejection callback.
///
/// See `MsCbEvent`.
///
/// # Parameters
///
/// - `cbid`: The uid of an allocated callback
///
/// # Return Value
///
/// 0 on success, < 0 on error
#[inline(always)]
#[allow(non_snake_case)]
pub unsafe fn MScmRegisterMSInsertEjectCallback(mut cbid: SceUid) -> i32 {
	sceIoDevctl(
		b"fatms0:\0" as _,
		0x02415821,
		&mut cbid as *mut _ as _,
		mem::size_of::<SceUid>() as i32,
		ptr::null_mut(),
		0,
	)
}

/// Unregister a memory stick ejection callback
///
/// # Parameters
///
/// - `cbid`: The uid of an allocated callback
///
/// # Return Value
///
/// 0 on success, < 0 on error
#[inline(always)]
#[allow(non_snake_case)]
pub unsafe fn MScmUnregisterMSInsertEjectCallback(mut cbid: SceUid) -> i32 {
	sceIoDevctl(
		b"fatms0:\0" as _,
		0x02415822,
		&mut cbid as *mut _ as _,
		mem::size_of::<SceUid>() as i32,
		ptr::null_mut(),
		0,
	)
}

