//! Nvidia cuda encoder/decoder/vpp

use std::{ffi::c_int, ops::Deref, sync::Arc};

use self::{context::CuContext, device::CuDevice};
use ffi::cuda::CUresult;

pub mod context;
pub mod device;
pub mod stream;

pub trait CudaResult {
    fn ok(&self) -> bool;
    fn err(&self) -> Result<(), Self>
    where
        Self: Sized;
    fn result<T>(&self, value: T) -> Result<T, Self>
    where
        Self: Sized;
}

impl CudaResult for ffi::cuda::CUresult {
    fn ok(&self) -> bool {
        return *self == ffi::cuda::cudaError_enum_CUDA_SUCCESS;
    }

    fn err(&self) -> Result<(), Self> {
        if *self == ffi::cuda::cudaError_enum_CUDA_SUCCESS {
            Ok(())
        } else {
            Err(*self)
        }
    }

    fn result<T>(&self, value: T) -> Result<T, Self>
    where
        Self: Sized,
    {
        if *self == ffi::cuda::cudaError_enum_CUDA_SUCCESS {
            Ok(value)
        } else {
            Err(*self)
        }
    }
}

pub struct Cuda {
    pub(crate) lib: Arc<ffi::cuda::cuda>,
}

impl Cuda {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let library_name = libloading::library_filename("cuda");
        let lib = unsafe { ffi::cuda::cuda::new(library_name) }?;
        let lib = Arc::new(lib);
        Ok(Self { lib })
    }

    pub fn init(&self, flags: u32) -> Result<(), CUresult> {
        unsafe { (*self).cuInit(flags).err() }
    }

    pub fn new_context(&self, dev: CuDevice, flags: u32) -> Result<CuContext, CUresult> {
        let mut ctx = CuContext {
            lib: &self,
            inner: std::ptr::null_mut(),
        };
        let res = unsafe { (*self).cuCtxCreate_v2(&mut ctx.inner, flags, dev.inner) };
        assert!(!ctx.inner.is_null());

        res.result(ctx)
    }

    /// Returns a handle to a compute device.
    pub fn new_device(&self, ordinal: i32) -> Result<CuDevice, CUresult> {
        let mut d = CuDevice { inner: 0, lib: &self };
        let res = unsafe { (*self).cuDeviceGet(&mut d.inner as *mut i32, ordinal) };

        res.result(d)
    }

    pub fn device_count(&self) -> Result<c_int, CUresult> {
        let mut val = 0;
        let res = unsafe { (*self).cuDeviceGetCount(&mut val as *mut i32) };

        res.result(val)
    }
}

impl Deref for Cuda {
    type Target = ffi::cuda::cuda;

    fn deref(&self) -> &Self::Target {
        &self.lib
    }
}

#[cfg(test)]
mod test {
    
}
