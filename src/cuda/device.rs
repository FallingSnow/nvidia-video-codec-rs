use std::os::raw::c_char;
use std::os::raw::c_int;

use ffi::cuda::CUdevice;
use ffi::cuda::CUdevice_attribute;
use ffi::cuda::CUresult;

use super::Cuda;
use super::CudaResult;

#[derive(Copy, Clone)]
pub struct CuDevice<'a> {
    pub(crate) lib: &'a Cuda,
    pub(crate) inner: CUdevice,
}

impl CuDevice<'_> {
    /// Returns information about the device.
    pub fn get_attribute(&self, attr: CUdevice_attribute) -> Result<c_int, CUresult> {
        let mut pi = 0;
        let res = unsafe { self.lib.cuDeviceGetAttribute(&mut pi as *mut i32, attr, self.inner) };

        res.result(pi)
    }

    /// Returns an identifer string for the device.
    pub fn get_name(&self) -> Result<String, CUresult> {
        let mut name = vec![0; 256];
        let res = unsafe {
            self.lib.cuDeviceGetName(
                name.as_mut_ptr() as *mut c_char,
                name.len() as i32,
                self.inner,
            )
        };
        let val = String::from_utf8(name).unwrap();

        res.result(val)
    }

    /// Returns the total amount of memory on the device.
    pub fn get_total_mem(&self) -> Result<u64, CUresult> {
        let mut val = 0;
        let res = unsafe { self.lib.cuDeviceTotalMem_v2(&mut val as *mut u64, self.inner) };

        res.result(val)
    }
}

#[cfg(test)]
mod tests {

    use crate::cuda::Cuda;
    use tracing_test::traced_test;
    
    #[test]
    #[traced_test]
    fn device_enum() {
        let cuda = Cuda::new().unwrap();
        cuda.init(0).unwrap();
        for i in 0..cuda.device_count().unwrap() {
            let dev = cuda.new_device(i).unwrap();

            println!(
                "{} {}",
                dev.get_name().unwrap(),
                dev.get_total_mem().unwrap()
            );
        }
    }
}
