// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../../../spiel/build/libspeechprovider
// DO NOT EDIT

use crate::{EventType};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "SpeechProviderStreamWriter")]
    pub struct StreamWriter(Object<ffi::SpeechProviderStreamWriter, ffi::SpeechProviderStreamWriterClass>);

    match fn {
        type_ => || ffi::speech_provider_stream_writer_get_type(),
    }
}

impl StreamWriter {
    #[doc(alias = "speech_provider_stream_writer_new")]
    pub fn new(fd: i32) -> StreamWriter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::speech_provider_stream_writer_new(fd))
        }
    }

    #[doc(alias = "speech_provider_stream_writer_close")]
    pub fn close(&self) {
        unsafe {
            ffi::speech_provider_stream_writer_close(self.to_glib_none().0);
        }
    }

    #[doc(alias = "speech_provider_stream_writer_send_audio")]
    pub fn send_audio(&self, chunk: &[u8]) {
        let chunk_size = chunk.len() as _;
        unsafe {
            ffi::speech_provider_stream_writer_send_audio(self.to_glib_none().0, chunk.to_glib_none().0, chunk_size);
        }
    }

    #[doc(alias = "speech_provider_stream_writer_send_event")]
    pub fn send_event(&self, event_type: EventType, range_start: u32, range_end: u32, mark_name: &str) {
        unsafe {
            ffi::speech_provider_stream_writer_send_event(self.to_glib_none().0, event_type.into_glib(), range_start, range_end, mark_name.to_glib_none().0);
        }
    }

    #[doc(alias = "speech_provider_stream_writer_send_stream_header")]
    pub fn send_stream_header(&self) {
        unsafe {
            ffi::speech_provider_stream_writer_send_stream_header(self.to_glib_none().0);
        }
    }

    pub fn fd(&self) -> i32 {
        ObjectExt::property(self, "fd")
    }
}
