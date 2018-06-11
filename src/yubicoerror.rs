extern crate reqwest;

use std::error;
use std::fmt;
use std::io::Error as ioError;
use std::sync::mpsc::RecvError as channelError;
use base64::DecodeError as base64Error;

#[derive(Debug)]
pub enum YubicoError {
    Network(reqwest::Error),
    HTTPStatusCode(reqwest::StatusCode),
    IOError(ioError),
    ChannelError(channelError),
    DecodeError(base64Error),
    CommandNotSupported,
    DeviceNotFound,
    OpenDeviceError,
    BadOTP,
    ReplayedOTP,
    BadSignature,
    MissingParameter,
    NoSuchClient,
    OperationNotAllowed,
    BackendError,
    NotEnoughAnswers,
    ReplayedRequest,
    UnknownStatus,
    OTPMismatch,
    NonceMismatch,
    SignatureMismatch,
}

impl fmt::Display for YubicoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            YubicoError::Network(ref err) => write!(f, "Connectivity error: {}", err),
            YubicoError::HTTPStatusCode(code) => write!(f, "Error found: {}", code),
            YubicoError::IOError(ref err) => write!(f, "IO error: {}", err),
            YubicoError::ChannelError(ref err) => write!(f, "Channel error: {}", err),
            YubicoError::DecodeError(ref err) => write!(f, "Decode  error: {}", err),            
            YubicoError::BadOTP => write!(f, "The OTP has invalid format."),
            YubicoError::ReplayedOTP => write!(f, "The OTP has already been seen by the service."),
            YubicoError::BadSignature => write!(f, "The HMAC signature verification failed."),
            YubicoError::MissingParameter => write!(f, "The request lacks a parameter."),
            YubicoError::NoSuchClient => write!(f, "The request id does not exist."),
            YubicoError::OperationNotAllowed => write!(f, "The request id is not allowed to verify OTPs."),
            YubicoError::BackendError => write!(f, "Unexpected error in our server. Please contact us if you see this error."),
            YubicoError::NotEnoughAnswers => write!(f, "Server could not get requested number of syncs during before timeout"),
            YubicoError::ReplayedRequest => write!(f, "Server has seen the OTP/Nonce combination before"),
            YubicoError::UnknownStatus => write!(f, "Unknown status sent by the OTP validation server"),
            YubicoError::OTPMismatch => write!(f, "OTP mismatch, It may be an attack attempt"),
            YubicoError::NonceMismatch => write!(f, "Nonce mismatch, It may be an attack attempt"),
            YubicoError::SignatureMismatch => write!(f, "Signature mismatch, It may be an attack attempt"),
            YubicoError::DeviceNotFound => write!(f, "Device not found"),
            YubicoError::OpenDeviceError => write!(f, "Can not open device"),
            YubicoError::CommandNotSupported => write!(f, "Command Not Supported"),
        }
    }
}

impl error::Error for YubicoError {
    fn description(&self) -> &str {
        match *self {
            YubicoError::Network(ref err) => err.description(),
            YubicoError::HTTPStatusCode(_) => "200 not received",
            YubicoError::IOError(ref err) => err.description(),
            YubicoError::ChannelError(ref err) => err.description(),
            YubicoError::DecodeError(ref err) => err.description(),
            YubicoError::BadOTP => "The OTP has invalid format.",
            YubicoError::ReplayedOTP => "The OTP has already been seen by the service.",
            YubicoError::BadSignature => "The HMAC signature verification failed.",
            YubicoError::MissingParameter => "The request lacks a parameter.",
            YubicoError::NoSuchClient => "The request id does not exist.",
            YubicoError::OperationNotAllowed => "The request id is not allowed to verify OTPs.",
            YubicoError::BackendError => "Unexpected error in our server. Please contact us if you see this error.",
            YubicoError::NotEnoughAnswers => "Server could not get requested number of syncs during before timeout",
            YubicoError::ReplayedRequest => "Server has seen the OTP/Nonce combination before",
            YubicoError::UnknownStatus => "Unknown status sent by the OTP validation server",
            YubicoError::OTPMismatch => "OTP in the response is the same as the supplied in the request. It may be an attack attempt",
            YubicoError::NonceMismatch => "NOnce in the response is the same as the supplied in the request. It may be an attack attempt",
            YubicoError::SignatureMismatch => "Signature in the response is the same as the supplied in the request. It may be an attack attempt",
            YubicoError::DeviceNotFound => "Youbikey device not found",
            YubicoError::OpenDeviceError => "Can not open device",
            YubicoError::CommandNotSupported => "Command Not Supported",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            YubicoError::Network(ref err) => Some(err),
            YubicoError::HTTPStatusCode(_) => None,
            YubicoError::IOError(ref err) => Some(err),
            YubicoError::ChannelError(ref err) => Some(err),
            YubicoError::DecodeError(ref err) => Some(err),            
            YubicoError::BadOTP => None,
            YubicoError::ReplayedOTP => None,
            YubicoError::BadSignature => None,
            YubicoError::MissingParameter => None,
            YubicoError::NoSuchClient => None,
            YubicoError::OperationNotAllowed => None,
            YubicoError::BackendError => None,
            YubicoError::NotEnoughAnswers => None,
            YubicoError::ReplayedRequest => None,
            YubicoError::UnknownStatus => None,
            YubicoError::OTPMismatch => None,
            YubicoError::NonceMismatch => None,
            YubicoError::SignatureMismatch => None,
            YubicoError::DeviceNotFound => None,
            YubicoError::OpenDeviceError => None,
            YubicoError::CommandNotSupported => None,
        }
    }
}

impl From<reqwest::Error> for YubicoError {
    fn from(err: reqwest::Error) -> YubicoError {
        YubicoError::Network(err)
    }
}

impl From<reqwest::StatusCode> for YubicoError {
    fn from(err: reqwest::StatusCode) -> YubicoError {
        YubicoError::HTTPStatusCode(err)
    }
}

impl From<ioError> for YubicoError {
    fn from(err: ioError) -> YubicoError {
        YubicoError::IOError(err)
    }
}

impl From<channelError> for YubicoError {
    fn from(err: channelError) -> YubicoError {
        YubicoError::ChannelError(err)
    }
}

impl From<base64Error> for YubicoError {
    fn from(err: base64Error) -> YubicoError {
        YubicoError::DecodeError(err)
    }
}