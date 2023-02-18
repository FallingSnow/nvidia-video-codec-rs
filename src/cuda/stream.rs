use ffi::cuda::*;

use super::CudaResult;

pub struct CuStream<'a> {
    pub(crate) lib: &'a ffi::cuda::cuda,
    pub(crate) inner: CUstream,
}

impl CuStream<'_> {
    // pub fn with_context(
    //     ctx: super::context::CuContext,
    //     non_blocking: bool,
    // ) -> Result<Self, ffi::cuda::CUresult> {
    //     let mut stream = CuStream {
    //         stream: std::ptr::null_mut(),
    //     };
    //     let flags = if non_blocking {
    //         ffi::cuda::CUstream_flags_enum_CU_STREAM_NON_BLOCKING
    //     } else {
    //         ffi::cuda::CUstream_flags_enum_CU_STREAM_DEFAULT
    //     };
    //     unsafe { self.lib.cuCtxPushCurrent_v2(ctx.context) }.err()?;
    //     let res = unsafe { self.lib.cuStreamCreate(&mut stream.stream, flags) };
    //     unsafe { self.lib.cuCtxPopCurrent_v2(std::ptr::null_mut()) }.err()?;

    //     res.result(stream)
    // }
}

impl Drop for CuStream<'_> {
    fn drop(&mut self) {
        unsafe {
            if !self.lib.cuStreamDestroy_v2(self.inner).ok() {
                tracing::error!("Failed to destroy cuda stream.");
            }
        }
    }
}
