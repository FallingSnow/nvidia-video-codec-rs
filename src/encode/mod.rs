use std::ops::Deref;
use std::{ffi::c_void, mem};

use ffi::encode_api::GUID as NvGUID;
use ffi::encode_api::NVENCAPI_VERSION;
// use ffi::encode_api::NV_ENCODE_API_FUNCTION_LIST;
use ffi::encode_api::NVENCSTATUS;
use ffi::encode_api::NV_ENCODE_API_FUNCTION_LIST;
use ffi::encode_api::NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS;
use ffi::encode_api::_NV_ENC_DEVICE_TYPE_NV_ENC_DEVICE_TYPE_CUDA;
use ffi::constants::encode_api::NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER;

use crate::cuda::context::CuContext;

pub trait EncodeResult {
    fn ok(&self) -> bool;
    fn err(&self) -> Result<(), Self>
    where
        Self: Sized;
    fn result<T>(&self, value: T) -> Result<T, Self>
    where
        Self: Sized;
}

impl EncodeResult for NVENCSTATUS {
    fn ok(&self) -> bool {
        return *self == ffi::encode_api::_NVENCSTATUS_NV_ENC_SUCCESS;
    }

    fn err(&self) -> Result<(), Self> {
        if *self == ffi::encode_api::_NVENCSTATUS_NV_ENC_SUCCESS {
            Ok(())
        } else {
            Err(*self)
        }
    }

    fn result<T>(&self, value: T) -> Result<T, Self>
    where
        Self: Sized,
    {
        if *self == ffi::encode_api::_NVENCSTATUS_NV_ENC_SUCCESS {
            Ok(value)
        } else {
            Err(*self)
        }
    }
}

pub struct Encode {
    pub(crate) lib: ffi::encode_api::nvidia_encode,
    pub(crate) api: NV_ENCODE_API_FUNCTION_LIST,
}

impl Encode {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let library_name = libloading::library_filename("nvidia-encode");
        let lib = unsafe { ffi::encode_api::nvidia_encode::new(library_name) }?;
        let mut function_list: NV_ENCODE_API_FUNCTION_LIST = unsafe { mem::zeroed() };
        function_list.version = ffi::constants::encode_api::NV_ENCODE_API_FUNCTION_LIST_VER;
        {
            let res = unsafe { lib.NvEncodeAPICreateInstance(&mut function_list) };
            if !res.ok() {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("NvEncodeAPICreateInstance = {}", res),
                )));
            }
        }
        Ok(Self {
            lib,
            api: function_list,
        })
    }

    pub fn new_encoder(&self, ctx: CuContext) -> Result<Encoder<'_>, NVENCSTATUS> {
        Encoder::new(&self, ctx)
    }
}

impl Deref for Encode {
    type Target = ffi::encode_api::nvidia_encode;

    fn deref(&self) -> &Self::Target {
        &self.lib
    }
}

pub struct Encoder<'a> {
    lib: &'a Encode,
    params: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS,
    inner: *mut c_void,
}

impl<'a> Encoder<'a> {
    pub(crate) fn new(lib: &'a Encode, ctx: CuContext) -> Result<Self, NVENCSTATUS> {
        let mut params: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS =
            NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS {
                version: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER,
                deviceType: _NV_ENC_DEVICE_TYPE_NV_ENC_DEVICE_TYPE_CUDA,
                device: ctx.inner as *mut c_void,
                reserved: std::ptr::null_mut(),
                apiVersion: NVENCAPI_VERSION,
                reserved1: [0; 253],
                reserved2: [std::ptr::null_mut(); 64],
            };

        let mut encoder = std::ptr::null_mut();
        let open_session = lib
            .api
            .nvEncOpenEncodeSessionEx
            .expect("nvEncOpenEncodeSessionEx not supported");
        let res = unsafe { open_session(&mut params, &mut encoder) };
        tracing::trace!("Create encoder = {}", res);

        // session will be dropped if there is an error causing NvEncDestroyEncoder to be called
        let session = Self {
            lib,
            params,
            inner: encoder,
        };

        res.result(session)
    }

    fn guids(&mut self) -> Result<Vec<NvGUID>, NVENCSTATUS> {
        unsafe {
            let mut guid_count: u32 = 0;
            let get_guid_count = self
                .lib
                .api
                .nvEncGetEncodeGUIDCount
                .expect("nvEncGetEncodeGUIDCount not supported");

            let res = get_guid_count(self.inner, &mut guid_count);
            tracing::trace!("Get GUID count = {}\t GUID count = {}", res, guid_count);
            res.err()?;

            let mut guids: Vec<NvGUID> = vec![mem::zeroed(); guid_count as usize];
            let get_guids = self
                .lib
                .api
                .nvEncGetEncodeGUIDs
                .expect("nvEncGetEncodeGUIDs not supported");

            let res = get_guids(
                self.inner,
                guids.as_mut_ptr(),
                guids.len() as u32,
                &mut guid_count,
            );
            tracing::trace!("Get GUIDs = {}\t GUIDs = {:?}", res, guids);

            res.result(guids)
        }
    }
}

// impl Drop for Encoder<'_> {
//     fn drop(&mut self) {
//         unsafe {
//             let destroy_encoder = self
//                 .lib
//                 .api
//                 .nvEncDestroyEncoder
//                 .expect("nvEncDestroyEncoder not supported");
//                 tracing::trace!("Destroying encoder = {:p}", self.inner);
//                 if !destroy_encoder(self.inner).ok() {
//                 tracing::error!("Failed to destroy encoder.");
//             }
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::cuda::Cuda;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn encoder_create() {
        // let api = API::new().unwrap();
        let cuda = Cuda::new().unwrap();
        cuda.init(0).unwrap();
        let device = cuda.new_device(0).unwrap();
        let ctx = cuda.new_context(device, 0).unwrap();
        let encode = Encode::new().unwrap();
        let mut encoder = encode.new_encoder(ctx).unwrap();
        let guids = encoder.guids().unwrap();
    }
}
