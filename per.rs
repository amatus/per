use std::path::Path;
use std::rt::io::io_error;
use std::task::{SingleThreaded, spawn_sched};

use oss::*;
//use mp3lame::*;
mod oss;
//mod mp3lame;

static DSP_FILES: &'static [&'static str] = &["/dev/dsp", "/dev/dsp1"];
static DSP_SPEEDS: [int, ..2] = [44100i, 48000i];

#[fixed_stack_segment]
fn main() {
  //let ctx = LameContext::new();
  let mut foo = None;
  for file_name in DSP_FILES.iter() {
    match OssDevice::new(&Path(file_name.as_slice())) {
      Some(x) => { foo = Some(x); break }
      None => {}
    }
  };
  let dsp = match foo {
    Some(x) => x,
    None => fail!("Unable to open dsp device")
  };
  dsp.reset();
  dsp.set_format();
  dsp.set_stereo();
  let mut speed: int = 0;
  for dsp_speed in DSP_SPEEDS.iter() {
    do io_error::cond.trap(|_| {speed = 0}).inside {
      dsp.set_speed(*dsp_speed);
      speed = *dsp_speed;
    }
    if speed != 0 {
      break;
    }
  }
  println(fmt!("Sample rate: %d Hz", speed));
  let (port, chan) = stream::<~[u8]>();
  do spawn_sched(SingleThreaded) {
    dsp.read_all(&chan);
  }
  loop {
    let buffer = port.recv();
    println(fmt!("Read buffer of length %u", buffer.len()));
  }
}
