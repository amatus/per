use std::libc::{c_int, c_void, close, O_RDONLY, open, read, size_t};
use std::rt::io::{io_error, IoError, OtherIoError};
use std::vec;

macro_rules! _SIO(
  ($x: expr, $y: expr) => (($x << 8) | $y);
)

/* rust doesn't have a sizeof you can use in constant expressions >.<
macro_rules! _SIOWR(
  ($x: expr, $y: expr, $t: ty) => (
    0xc0000000 as c_int | (size_of::<$t>() as c_int << 16) | ($x << 8) | $y);
)
*/

macro_rules! _SIOWR(
  ($x: expr, $y: expr, $t: expr) => (
    0xc0000000 as c_int | ($t << 16) | ($x << 8) | $y);
)

static SNDCTL_DSP_RESET: c_int = _SIO!('P' as c_int, 0);
static SNDCTL_DSP_SYNC: c_int = _SIO!('P' as c_int, 1);
static SNDCTL_DSP_SPEED: c_int = _SIOWR!('P' as c_int, 2, 4);
static SNDCTL_DSP_STEREO: c_int = _SIOWR!('P' as c_int, 3, 4);
static SNDCTL_DSP_GETBLKSIZE: c_int = _SIOWR!('P' as c_int, 4, 4);
static SNDCTL_DSP_SETFMT: c_int = _SIOWR!('P' as c_int, 5, 4);
static AFMT_S16_LE: c_int = 0x00000010;

extern {
  fn ioctl(fd: c_int, ioctl: c_int, arg: *mut c_int) -> c_int;
}

pub struct OssDevice {
  fd: c_int,
}

impl OssDevice {
  #[fixed_stack_segment]
  pub fn new(path: &Path) -> Option<OssDevice> {
    let fd = do path.with_c_str |path| {
      unsafe { open(path, O_RDONLY, 0777) }
    };
    if fd != -1 {
      Some(OssDevice { fd: fd })
    } else {
      None
    }
  }
  #[fixed_stack_segment]
  pub fn reset(&self) {
    unsafe {
      if ioctl(self.fd, SNDCTL_DSP_RESET, 0 as *mut c_int) == -1 {
        io_error::cond.raise(IoError { kind: OtherIoError,
                                       desc: "Unable to reset dsp",
                                       detail: None });
      }
    };
  }
  #[fixed_stack_segment]
  pub fn set_format(&self) {
    unsafe {
      let mut i: c_int = AFMT_S16_LE;
      if ioctl(self.fd, SNDCTL_DSP_SETFMT, &mut i) == -1 {
        io_error::cond.raise(IoError { kind: OtherIoError,
                                       desc: "Unable to set dsp format",
                                       detail: None });
      }
    };
  }
  #[fixed_stack_segment]
  pub fn set_stereo(&self) {
    unsafe {
      let mut i: c_int = 1;
      if ioctl(self.fd, SNDCTL_DSP_STEREO, &mut i) == -1 {
        io_error::cond.raise(IoError { kind: OtherIoError,
                                       desc: "Unable to set dsp stereo",
                                       detail: None });
      }
    };
  }
  #[fixed_stack_segment]
  pub fn set_speed(&self, rate: int) {
    unsafe {
      let mut i: c_int = rate as c_int;
      if ioctl(self.fd, SNDCTL_DSP_SPEED, &mut i) == -1 {
        io_error::cond.raise(IoError { kind: OtherIoError,
                                       desc: "Unable to set dsp speed",
                                       detail: None });
      }
    };
  }
  #[fixed_stack_segment]
  pub fn get_block_size(&self) -> uint {
    unsafe {
      let mut i: c_int = 0;
      if ioctl(self.fd, SNDCTL_DSP_GETBLKSIZE, &mut i) == -1 {
        io_error::cond.raise(IoError { kind: OtherIoError,
                                       desc: "Unable to get dsp block size",
                                       detail: None });
      };
      i as uint
    }
  }
  #[fixed_stack_segment]
  pub fn sync(&self) {
    unsafe {
      if ioctl(self.fd, SNDCTL_DSP_SYNC, 0 as *mut c_int) == -1 {
        io_error::cond.raise(IoError { kind: OtherIoError,
                                       desc: "Unable to get sync dsp",
                                       detail: None });
      };
    }
  }
  #[fixed_stack_segment]
  pub fn read_all(&self, chan: &Chan<~[u8]>) {
    let block_size = self.get_block_size();
    self.sync();
    loop {
      let mut buffer: ~[u8] = vec::with_capacity(block_size);
      unsafe {
        let length = read(self.fd,
                          vec::raw::to_mut_ptr(buffer) as *mut c_void,
                          block_size as size_t);
        if length == -1 {
          break;
        }
        vec::raw::set_len(&mut buffer, length as uint);
      }
      chan.send(buffer);
    }
  }
}

impl Drop for OssDevice {
  #[fixed_stack_segment]
  fn drop(&mut self) {
    unsafe { close(self.fd) };
  }
}
