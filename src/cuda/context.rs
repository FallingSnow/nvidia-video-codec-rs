use cuda::CudaResult;

use super::{stream::CuStream, Cuda};

pub struct CuContext<'a> {
    pub(crate) lib: &'a Cuda,
    pub(crate) inner: ffi::cuda::CUcontext,
}

impl CuContext<'_> {
    pub fn get_api_version(&self) -> Result<u32, ffi::cuda::CUresult> {
        let mut ver = 0;
        let res = unsafe {
            self.lib
                .cuCtxGetApiVersion(self.inner, &mut ver as *mut u32)
        };

        res.result(ver)
    }
    pub fn new_stream(&self, non_blocking: bool) -> Result<CuStream, ffi::cuda::CUresult> {
        let mut stream = CuStream {
            inner: std::ptr::null_mut(),
            lib: &self.lib,
        };
        let flags = if non_blocking {
            ffi::cuda::CUstream_flags_enum_CU_STREAM_NON_BLOCKING
        } else {
            ffi::cuda::CUstream_flags_enum_CU_STREAM_DEFAULT
        };
        unsafe { self.lib.cuCtxPushCurrent_v2(self.inner) }.err()?;
        let res = unsafe { self.lib.cuStreamCreate(&mut stream.inner, flags) };
        unsafe { self.lib.cuCtxPopCurrent_v2(std::ptr::null_mut()) }.err()?;

        res.result(stream)
    }
}

impl Drop for CuContext<'_> {
    fn drop(&mut self) {
        unsafe {
            if !self.lib.cuCtxDestroy_v2(self.inner).ok() {
                tracing::error!("Failed to destroy cuda context.");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cuda::Cuda;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn create_context() {
        let cuda = Cuda::new().unwrap();
        cuda.init(0).unwrap();
        assert_ne!(cuda.device_count().unwrap(), 0);
        let dev = cuda.new_device(0).unwrap();
        let _ctx = cuda.new_context(dev, 0).unwrap();
    }
}
