pub mod color;

pub trait NppResult {
    fn ok(&self) -> bool;
    fn err(&self) -> Result<(), Self>
    where
        Self: Sized;
}

impl NppResult for ffi::npp::NppStatus {
    fn ok(&self) -> bool {
        return *self == ffi::npp::NppStatus_NPP_SUCCESS;
    }

    fn err(&self) -> Result<(), Self> {
        if *self == ffi::npp::NppStatus_NPP_SUCCESS {
            Ok(())
        } else {
            Err(*self)
        }
    }
}