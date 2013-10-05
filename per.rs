extern mod extra;
use extra::getopts::*;
use extra::time::*;
use std::os;
use std::path::Path;
use std::rt::io::{Create, io_error, Open, Write, Writer};
use std::rt::io::file::open;
use std::task::{SingleThreaded, spawn_sched};

use oss::OssDevice;
use mp3lame::LameContext;
mod oss;
mod mp3lame;

fn main() {
  let args = os::args();
  let opts = ~[
    groups::optmulti("f", "file", "OSS device file", "/dev/dsp"),
    groups::optmulti("r", "rate", "Sample rate in Hz", "44100"),
    groups::optopt("s", "split",
      "Number of minutes at which to split MP3 files", "60"),
    groups::optflag("a", "align",
      "Align splits as if the first one happened midnight Jan. 1, 1970"),
    groups::optopt("b", "bitrate", "MP3 bitrate in kbps", "128"),
    groups::optopt("q", "quality", "MP3 quality", "2"),
  ];
  let DSP_FILES = ~[~"/dev/dsp", ~"/dev/dsp1"];
  let DSP_SPEEDS = ~[44100i, 48000i];
  let FILE_FORMAT = "%F %H.%M.%S%z.mp3";
  let matches = match groups::getopts(args.tail(), opts) {
    Ok(m) => m,
    Err(f) => {
      println(f.to_err_msg());
      print(groups::usage("Usage: per [options]", opts));
      return;
    }
  };
  let dsp_files = match matches.opt_strs("f") {
    [] => DSP_FILES,
    f => f
  };
  let lame = LameContext::new();
  let mut foo = None;
  for file_name in dsp_files.iter() {
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
  let dsp_speeds = match matches.opt_strs("r") {
    [] => DSP_SPEEDS,
    r => r.map(|x| { from_str::<int>(*x).unwrap() })
  };
  let mut speed: int = 0;
  for dsp_speed in dsp_speeds.iter() {
    do io_error::cond.trap(|_| {speed = 0}).inside {
      dsp.set_speed(*dsp_speed);
      speed = *dsp_speed;
    }
    if speed != 0 {
      break;
    }
  }
  let quality = match matches.opt_str("q") {
    Some(q) => from_str::<int>(q).unwrap(),
    None => 2
  };
  let bitrate = match matches.opt_str("b") {
    Some(b) => from_str::<int>(b).unwrap(),
    None => 128
  };
  lame.set_in_samplerate(speed);
  lame.set_quality(quality);
  lame.set_bitrate(bitrate);
  lame.set_disable_reservoir(true);
  lame.init_params();
  println(fmt!("Recording sample rate: %d Hz", speed));
  println(fmt!("Encoding sample rate:  %d Hz", lame.get_out_samplerate()));
  let split = match matches.opt_str("s") {
    Some(s) => from_str::<int>(s).unwrap() * 60,
    None => 3600
  };
  let mut next_split = if matches.opt_present("a") {
    let Timespec { sec, _ } = get_time();
    Timespec::new(sec - sec % split as i64, 0)
  } else {
    get_time()
  };
  let mut out_file = open(&Path("/dev/null"), Open, Write).unwrap();
  let (port, chan) = stream::<~[u8]>();
  do spawn_sched(SingleThreaded) {
    dsp.read_all(&chan);
  }
  loop {
    let now = get_time();
    if next_split <= now {
      debug!("split!");
      out_file.write(lame.encode_flush_nogap());
      out_file.flush();
      out_file = open(&Path(at(now).strftime(FILE_FORMAT)), Create, Write)
        .unwrap();
      let Timespec { sec, nsec } = next_split;
      next_split = Timespec::new(sec + split as i64, nsec);
    }
    let buffer = port.recv();
    debug!("Read buffer of length %u", buffer.len());
    let mp3buf = lame.encode_buffer_interleaved(buffer);
    debug!("Encoded buffer of length %u", mp3buf.len());
    out_file.write(mp3buf);
  }
}
