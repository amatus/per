use std::libc::{c_float, c_int, c_short, c_uchar, c_ulong, c_void};

type GlobalFlags_ = *c_void;

enum MpegMode {
  Stereo = 0,
  JointStereo,
  DualChannel,
  Mono,
  NotSet,
  MaxIndicator
}

#[link_args = "-lmp3lame"]

extern {
  fn lame_init() -> GlobalFlags_;
  fn lame_set_num_samples(gfp: GlobalFlags_, samples: c_ulong) -> c_int;
  fn lame_get_num_samples(gfp: GlobalFlags_) -> c_ulong;
  fn lame_set_in_samplerate(gfp: GlobalFlags_, rate: c_int) -> c_int;
  fn lame_get_in_samplerate(gfp: GlobalFlags_) -> c_int;
  fn lame_set_num_channels(gfp: GlobalFlags_, channels: c_int) -> c_int;
  fn lame_get_num_channels(gfp: GlobalFlags_) -> c_int;
  fn lame_set_scale(gfp: GlobalFlags_, scale: c_float) -> c_int;
  fn lame_get_scale(gfp: GlobalFlags_) -> c_float;
  fn lame_set_scale_left(gfp: GlobalFlags_, scale: c_float) -> c_int;
  fn lame_get_scale_left(gfp: GlobalFlags_) -> c_float;
  fn lame_set_scale_right(gfp: GlobalFlags_, scale: c_float) -> c_int;
  fn lame_get_scale_right(gfp: GlobalFlags_) -> c_float;
  fn lame_set_out_samplerate(gfp: GlobalFlags_, rate: c_int) -> c_int;
  fn lame_get_out_samplerate(gfp: GlobalFlags_) -> c_int;
  fn lame_set_analysis(gfp: GlobalFlags_, analysis: c_int) -> c_int;
  fn lame_get_analysis(gfp: GlobalFlags_) -> c_int;
  fn lame_set_bWriteVbrTag(gfp: GlobalFlags_, write_tag: c_int) -> c_int;
  fn lame_get_bWriteVbrTag(gfp: GlobalFlags_) -> c_int;
  fn lame_set_decode_only(gfp: GlobalFlags_, decode: c_int) -> c_int;
  fn lame_get_decode_only(gfp: GlobalFlags_) -> c_int;
  fn lame_set_quality(gfp: GlobalFlags_, quality: c_int) -> c_int;
  fn lame_get_quality(gfp: GlobalFlags_) -> c_int;
  fn lame_set_mode(gfp: GlobalFlags_, mode: MpegMode) -> c_int;
  fn lame_get_mode(gfp: GlobalFlags_) -> MpegMode;
  fn lame_set_brate(gfp: GlobalFlags_, rate: c_int) -> c_int;
  fn lame_get_brate(gfp: GlobalFlags_) -> c_int;
  fn lame_set_disable_reservoir(gfp: GlobalFlags_, disable: c_int) -> c_int;
  fn lame_get_disable_reservoir(gfp: GlobalFlags_) -> c_int;
  fn lame_init_params(gfp: GlobalFlags_) -> c_int;
  fn lame_close(gfp: GlobalFlags_) -> c_int;
  fn lame_encode_buffer_interleaved(gfp: GlobalFlags_,
                                    pcm: *c_short,
                                    num_samples:c_int,
                                    mp3buf: *c_uchar,
                                    mp3buf_size: c_int) -> c_int;
  fn lame_encode_flush_nogap(gfp: GlobalFlags_,
                             mp3buf: *c_uchar,
                             size: c_int) -> c_int;
}

pub struct LameContext {
  gfp: GlobalFlags_
}

impl LameContext {
  #[fixed_stack_segment]
  pub fn new() -> LameContext {
    LameContext { gfp: unsafe { lame_init() }}
  }
  #[fixed_stack_segment]
  pub fn set_in_samplerate(&self, rate: int) {
    unsafe { lame_set_in_samplerate(self.gfp, rate as c_int) };
  }
  #[fixed_stack_segment]
  pub fn set_out_samplerate(&self, rate: int) {
    unsafe { lame_set_out_samplerate(self.gfp, rate as c_int) };
  }
  #[fixed_stack_segment]
  pub fn set_num_channels(&self, channels: int) {
    unsafe { lame_set_num_channels(self.gfp, channels as c_int) };
  }
  #[fixed_stack_segment]
  pub fn set_quality(&self, quality: int) {
    unsafe { lame_set_quality(self.gfp, quality as c_int) };
  }
  #[fixed_stack_segment]
  pub fn set_bitrate(&self, rate: int) {
    unsafe { lame_set_brate(self.gfp, rate as c_int) };
  }
  #[fixed_stack_segment]
  pub fn set_disable_reservoir(&self, disable: bool) {
    unsafe { lame_set_disable_reservoir(self.gfp, disable as c_int) };
  }
  #[fixed_stack_segment]
  pub fn init_params(&self) {
    unsafe { lame_init_params(self.gfp) };
  }
}

impl Drop for LameContext {
  #[fixed_stack_segment]
  fn drop(&mut self) {
    unsafe { lame_close(self.gfp) };
  }
}
