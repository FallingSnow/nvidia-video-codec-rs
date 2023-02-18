use std::mem::MaybeUninit;

use crate::{cuda, npp::NppResult};

pub fn nv12_to_rgb24(
    ptr: ffi::cuvid::CUdeviceptr,
    width: u32,
    height: u32,
    pitch: i32,
    dest_ptr: *mut std::os::raw::c_void,
    dest_pitch: i32,
    stream: Option<&cuda::stream::CuStream>,
) -> Result<(), ffi::npp::NppStatus> {
    let src: [*const ffi::npp::Npp8u; 2] = unsafe {
        [
            (ptr as *const ffi::npp::Npp8u),
            (ptr as *const ffi::npp::Npp8u).offset((pitch * (height as i32)) as isize),
        ]
    };
    let size_roi = ffi::npp::NppiSize {
        width: width as _,
        height: height as _,
    };

    if let Some(stream) = stream {
        unsafe {
            if ffi::npp::nppGetStream() != (stream.stream as _) {
                ffi::npp::nppSetStream(stream.stream as _);
            }
        }
    }

    let stream_ctx = unsafe {
        let mut ctx: MaybeUninit<ffi::npp::NppStreamContext> = MaybeUninit::uninit();
        ffi::npp::nppGetStreamContext(ctx.as_mut_ptr()).err()?;
        ctx.assume_init()
    };

    unsafe {
        ffi::npp::nppiNV12ToRGB_8u_P2C3R_Ctx(
            src.as_ptr(),
            pitch,
            dest_ptr as _,
            dest_pitch,
            size_roi,
            stream_ctx,
        )
        .err()?;
    }

    Ok(())
}

pub fn nv12_to_bgr24(
    ptr: ffi::cuvid::CUdeviceptr,
    width: u32,
    height: u32,
    pitch: i32,
    dest_ptr: *mut std::os::raw::c_void,
    dest_pitch: i32,
    stream: Option<&cuda::stream::CuStream>,
) -> Result<(), ffi::npp::NppStatus> {
    let src: [*const ffi::npp::Npp8u; 2] = unsafe {
        [
            (ptr as *const ffi::npp::Npp8u),
            (ptr as *const ffi::npp::Npp8u).offset((pitch * (height as i32)) as isize),
        ]
    };
    let size_roi = ffi::npp::NppiSize {
        width: width as _,
        height: height as _,
    };

    if let Some(stream) = stream {
        unsafe {
            if ffi::npp::nppGetStream() != (stream.stream as _) {
                ffi::npp::nppSetStream(stream.stream as _);
            }
        }
    }

    let stream_ctx = unsafe {
        let mut ctx: MaybeUninit<ffi::npp::NppStreamContext> = MaybeUninit::uninit();
        ffi::npp::nppGetStreamContext(ctx.as_mut_ptr()).err()?;
        ctx.assume_init()
    };

    unsafe {
        ffi::npp::nppiNV12ToBGR_8u_P2C3R_Ctx(
            src.as_ptr(),
            pitch,
            dest_ptr as _,
            dest_pitch,
            size_roi,
            stream_ctx,
        )
        .err()?;
    }

    Ok(())
}
