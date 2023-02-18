//! # NVIDIA VIDEO CODEC SDK - ENCODER vNVENCODEAPI_PG-06155-001_v11 | 2
//! 
//! # Basic Encoding Flow
//! 
//! Developers can create a client application that calls NVENCODE API functions exposed
//! by nvEncodeAPI.dll for Windows or libnvidia-encode.so for Linux. These libraries are
//! installed as part of the NVIDIA display driver. The client application can either link to these
//! libraries at run-time using LoadLibrary() on Windows or dlopen() on Linux.
//! The NVENCODE API functions, structures and other parameters are exposed in nvEncodeAPI.h,
//! which is included in the SDK.
//! 
//! NVENCODE API is a C-API, and uses a design pattern like C++ interfaces, wherein the application
//! creates an instance of the API and retrieves a function pointer table to further interact with the
//! encoder. For programmers preferring more high-level API with ready-to-use code, SDK includes
//! sample C++ classes expose important API functions.
//! 
//! Rest of this document focuses on the C-API exposed in nvEncodeAPI.h. NVENCODE API is
//! designed to accept raw video frames (in YUV or RGB format) and output the H.264, HEVC or AV1
//! bitstream. Broadly, the encoding flow consists of the following steps:
//! 
//!   1. Initialize the encoder
//!   2. Set up the desired encoding parameters
//!   3. Allocate input/output buffers
//!   4. Copy frames to input buffers and read bitstream from the output buffers. This can be done synchronously (Windows & Linux) or asynchronously (Windows 7 and above only).
//!   5. Clean-up - release all allocated input/output buffers
//!   6. Close the encoding session
//! 
//! These steps are explained in the rest of the document and demonstrated in the sample
//! application included in the Video Codec SDK package.
//! 
//! -- See (Nvidia Encoder Programming Guide)[https://docs.nvidia.com/video-technologies/video-codec-sdk/nvenc-video-encoder-api-prog-guide/] for more info

extern crate nvidia_video_codec_sys as ffi;
#[cfg(test)]
extern crate tracing_test;

pub mod cuda;
// pub mod cuvid;
// pub mod npp;
pub mod encode;