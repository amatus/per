use std::path::Path;
use std::rt::io::io_error;
use std::task::{SingleThreaded, spawn_sched};

use oss::*;
use mp3lame::*;
mod oss;
mod mp3lame;

static DSP_FILES: &'static [&'static str] = &["/dev/dsp", "/dev/dsp1"];
static DSP_SPEEDS: [int, ..2] = [44100i, 48000i];

#[fixed_stack_segment]
fn main() {
  let lame = LameContext::new();
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
  lame.set_num_channels(2);
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
  lame.set_in_samplerate(speed);
  lame.set_out_samplerate(speed);
  println(fmt!("Sample rate: %d Hz", speed));
  lame.set_quality(2);
  lame.set_bitrate(128);
  lame.set_disable_reservoir(true);
  lame.init_params();
  let (port, chan) = stream::<~[u8]>();
  do spawn_sched(SingleThreaded) {
    dsp.read_all(&chan);
  }
  loop {
    let buffer = port.recv();
    debug!("Read buffer of length %u", buffer.len());
    let mp3buf = lame.encode_buffer_interleaved(buffer);
    debug!("Encoded buffer of length %u", mp3buf.len());
  }
}
