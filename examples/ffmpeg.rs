//! example: ffmpeg

use flood_tide::Arg;
use flood_tide::Lex;
use flood_tide::NameVal;
use flood_tide::Opt;
use flood_tide::OptNum;
use flood_tide::OptParseError;

//----------------------------------------------------------------------
//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
Hyper fast Audio and Video encoder.
"#;

const OPTIONS_TEXT: &str = r#"Getting help:
  -h                    print basic options
  -h long               print more options
  -h full               print all options (include all format and codec specific
                        options, very long)
  -h type=name          print all options for the named decoder/encoder/demuxer/
                        muxer/filter/bsf/protocol

Print help / information / capabilities:
  -L                    show license
  -h topic              show help
  -? topic              show help
  -version              show version
  -buildconf            show build configuration
  -formats              show available formats

Global options:
  -loglevel loglevel    set logging level
  -v loglevel           set logging level
  -report               generate a report
  -max_alloc bytes      set maximum size of a single allocated block
  -y                    overwrite output files
  -n                    never overwrite output files
  -ignore_unknown       Ignore unknown stream types
  -filter_threads number
                        number of non-complex filter threads
  -filter_complex_threads number
                        number of threads for -filter_complex
  -stats                print progress report during encoding

Per-file main options:
  -f fmt                force format
  -c codec              codec name
  -codec codec          codec name
  -pre preset           preset name
  -map_metadata outfile[,metadata]:infile[,metadata]
                        set metadata information of outfile from infile
  -t duration           record or transcode \"duration\" seconds of audio/video
  -to time_stop         record or transcode stop time
  -fs limit_size        set the limit file size in bytes
  -ss time_off          set the start time offset
  -sseof time_off       set the start time offset relative to EOF
  -seek_timestamp       enable/disable seeking by timestamp with -ss
  -timestamp time       set the recording timestamp ('now' to set the current time)
  -metadata string=string
                        add metadata
  -program title=string:st=number...
                        add program with specified streams
  -target type          specify target file type (\"vcd\", \"svcd\", \"dvd\", \"dv\"
                        or \"dv50\" with optional prefixes \"pal-\", \"ntsc-\" or \"film-\")
  -apad                 audio pad
  -frames number        set the number of frames to output
  -filter filter_graph
                        set stream filtergraph
  -filter_script filename
                        read stream filtergraph description from a file
  -reinit_filter        reinit filtergraph on input parameter changes
  -discard              discard
  -disposition          disposition

Video options:
  -vframes number       set the number of video frames to output
  -r rate               set frame rate (Hz value, fraction or abbreviation)
  -s size               set frame size (WxH or abbreviation)
  -aspect aspect        set aspect ratio (4:3, 16:9 or 1.3333, 1.7777)
  -bits_per_raw_sample number
                        set the number of bits per raw sample
  -vn                   disable video
  -vcodec codec         force video codec ('copy' to copy stream)
  -timecode hh:mm:ss[:;.]ff
                        set initial TimeCode value.
  -pass n               select the pass number (1 to 3)
  -vf filter_graph      set video filters
  -ab bitrate           audio bitrate (please use -b:a)
  -b bitrate            video bitrate (please use -b:v)
  -dn                   disable data

Audio options:
  -aframes number       set the number of audio frames to output
  -aq quality           set audio quality (codec-specific)
  -ar rate              set audio sampling rate (in Hz)
  -ac channels          set number of audio channels
  -an                   disable audio
  -acodec codec         force audio codec ('copy' to copy stream)
  -vol volume           change audio volume (256=normal)
  -af filter_graph      set audio filters

Subtitle options:
  -sn                   disable subtitle
  -scodec codec         force subtitle codec ('copy' to copy stream)
  -stag fourcc/tag      force subtitle tag/fourcc
  -fix_sub_duration     fix subtitles duration
  -canvas_size size     set canvas size (WxH or abbreviation)
  -spre preset          set the subtitle options to the indicated preset

  -H, --help            display this help and exit
  -V, --version         output version information and exit
"#;

const ARGUMENTS_TEXT: &str = r#"Argument:
  <outfile>             file path to writing
"#;

const EXAMPLES_TEXT: &str = r#"Examples:
  You can also do audio and video conversions at the same time:
    ffmpeg -i /tmp/a.wav -ar 22050 /tmp/a.mp2
  You can encode to several formats at the same time and define a mapping from input stream to output streams:
    ffmpeg -i /tmp/a.wav -map 0:a -b:a 64k /tmp/a.mp2 -map 0:a -b:a 128k /tmp/b.mp2
"#;
//}}} TEXT

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOP {
    // Print help / information / capabilities
    ShowLicense = 1,
    ShowBuildconf = 2,
    ShowFormats = 3,
    // Global options
    Loglevel = 4,
    Report = 5,
    MaxAlloc = 6,
    YesOOF = 7,
    NeverOOF = 8,
    IgnoreUnknown = 9,
    FilterThreads = 10,
    FilterComplexThreads = 11,
    Stats = 12,
    // Per-file main options
    Format = 13,
    Codec = 14,
    Preset = 15,
    MapMetadata = 16,
    Duration = 17,
    TimeStop = 18,
    FileSize = 19,
    StartTime = 20,
    StartTimeRel = 21,
    SeekTimestamp = 22,
    Timestamp = 23,
    Metadata = 24,
    Program = 25,
    Target = 26,
    AudioPad = 27,
    Frames = 28,
    Filter = 29,
    FilterScript = 30,
    ReinitFilter = 31,
    Discard = 32,
    Disposition = 33,
    // Video options
    VideoFrames = 34,
    Rate = 35,
    Size = 36,
    Aspect = 37,
    BitsPerRawSample = 38,
    VideoNone = 39,
    VideoCodec = 40,
    TimeCode = 41,
    Pass = 42,
    VideoFilter = 43,
    AudioBitrate = 44,
    VideoBitrate = 45,
    DataNone = 46,
    // Audio options
    AudioFrames = 47,
    AudioQuality = 48,
    AudioRate = 49,
    AudioChannel = 50,
    AudioNone = 51,
    AudioCodec = 52,
    Volume = 53,
    AudioFilter = 54,
    // Subtitle options
    SubtiNone = 55,
    SubtiCodec = 56,
    SubtiTag = 57,
    FixSubDuration = 58,
    CanvasSize = 59,
    SubtiPreset = 60,
    // in file
    InFile = 61,
    //
    Help = 62,
    Version = 63,
}
impl std::convert::From<OptNum> for CmdOP {
    fn from(value: OptNum) -> Self {
        unsafe { std::mem::transmute_copy(&value) }
    }
}
impl CmdOP {
    pub const fn to(self) -> OptNum {
        self as OptNum
    }
}

//
#[rustfmt::skip]
const OPT_ARY: [Opt;66] = [
    Opt { sho: b'H', lon: "-help",    has: Arg::No,  num: CmdOP::Help.to(), },
    Opt { sho: b'V', lon: "-version", has: Arg::No,  num: CmdOP::Version.to(), },
    Opt { sho: 0u8, lon: "L",        has: Arg::No,  num: CmdOP::ShowLicense.to(), },
    Opt { sho: 0u8, lon: "ab",       has: Arg::Yes, num: CmdOP::AudioBitrate.to(), },
    Opt { sho: 0u8, lon: "ac",       has: Arg::Yes, num: CmdOP::AudioChannel.to(), },
    Opt { sho: 0u8, lon: "acodec",   has: Arg::Yes, num: CmdOP::AudioCodec.to(), },
    Opt { sho: 0u8, lon: "af",       has: Arg::Yes, num: CmdOP::AudioFilter.to(), },
    Opt { sho: 0u8, lon: "aframes",  has: Arg::Yes, num: CmdOP::AudioFrames.to(), },
    Opt { sho: 0u8, lon: "an",       has: Arg::No,  num: CmdOP::AudioNone.to(), },
    Opt { sho: 0u8, lon: "apad",     has: Arg::No,  num: CmdOP::AudioPad.to(), },
    Opt { sho: 0u8, lon: "aq",       has: Arg::Yes, num: CmdOP::AudioQuality.to(), },
    Opt { sho: 0u8, lon: "ar",       has: Arg::Yes, num: CmdOP::AudioRate.to(), },
    Opt { sho: 0u8, lon: "aspect",   has: Arg::Yes, num: CmdOP::Aspect.to(), },
    Opt { sho: 0u8, lon: "b",        has: Arg::Yes, num: CmdOP::VideoBitrate.to(), },
    Opt { sho: 0u8, lon: "bits_per_raw_sample", has: Arg::Yes, num: CmdOP::BitsPerRawSample.to(), },
    Opt { sho: 0u8, lon: "buildconf",has: Arg::No,  num: CmdOP::ShowBuildconf.to(), },
    Opt { sho: 0u8, lon: "c",        has: Arg::Yes, num: CmdOP::Codec.to(), },
    Opt { sho: 0u8, lon: "canvas_size", has: Arg::Yes, num: CmdOP::CanvasSize.to(), },
    Opt { sho: 0u8, lon: "codec",    has: Arg::Yes, num: CmdOP::Codec.to(), },
    Opt { sho: 0u8, lon: "discard",   has: Arg::No,  num: CmdOP::Discard.to(), },
    Opt { sho: 0u8, lon: "disposition", has: Arg::No, num: CmdOP::Disposition.to(), },
    Opt { sho: 0u8, lon: "dn",       has: Arg::Yes, num: CmdOP::DataNone.to(), },
    Opt { sho: 0u8, lon: "f",        has: Arg::Yes, num: CmdOP::Format.to(), },
    Opt { sho: 0u8, lon: "filter",   has: Arg::Yes, num: CmdOP::Filter.to(), },
    Opt { sho: 0u8, lon: "filter_complex_threads", has: Arg::Yes, num: CmdOP::FilterComplexThreads.to(), },
    Opt { sho: 0u8, lon: "filter_script", has: Arg::Yes, num: CmdOP::FilterScript.to(), },
    Opt { sho: 0u8, lon: "filter_threads", has: Arg::Yes, num: CmdOP::FilterThreads.to(), },
    Opt { sho: 0u8, lon: "fix_sub_duration", has: Arg::No, num: CmdOP::FixSubDuration.to(), },
    Opt { sho: 0u8, lon: "formats",  has: Arg::No,  num: CmdOP::ShowFormats.to(), },
    Opt { sho: 0u8, lon: "frames",   has: Arg::Yes, num: CmdOP::Frames.to(), },
    Opt { sho: 0u8, lon: "fs",       has: Arg::Yes, num: CmdOP::FileSize.to(), },
    Opt { sho: 0u8, lon: "h",        has: Arg::Maybe, num: CmdOP::Help.to(), },
    Opt { sho: 0u8, lon: "i",        has: Arg::Yes, num: CmdOP::InFile.to(), },
    Opt { sho: 0u8, lon: "ignore_unknown", has: Arg::No, num: CmdOP::IgnoreUnknown.to(), },
    Opt { sho: 0u8, lon: "loglevel", has: Arg::Yes, num: CmdOP::Loglevel.to(), },
    Opt { sho: 0u8, lon: "map_metadata", has: Arg::Yes, num: CmdOP::MapMetadata.to(), },
    Opt { sho: 0u8, lon: "max_alloc",has: Arg::Yes, num: CmdOP::MaxAlloc.to(), },
    Opt { sho: 0u8, lon: "metadata", has: Arg::Yes, num: CmdOP::Metadata.to(), },
    Opt { sho: 0u8, lon: "n",        has: Arg::No,  num: CmdOP::NeverOOF.to(), },
    Opt { sho: 0u8, lon: "pass",     has: Arg::Yes, num: CmdOP::Pass.to(), },
    Opt { sho: 0u8, lon: "pre",      has: Arg::Yes, num: CmdOP::Preset.to(), },
    Opt { sho: 0u8, lon: "program",  has: Arg::Yes, num: CmdOP::Program.to(), },
    Opt { sho: 0u8, lon: "r",        has: Arg::Yes, num: CmdOP::Rate.to(), },
    Opt { sho: 0u8, lon: "reinit_filter", has: Arg::No, num: CmdOP::ReinitFilter.to(), },
    Opt { sho: 0u8, lon: "report",   has: Arg::No,  num: CmdOP::Report.to(), },
    Opt { sho: 0u8, lon: "s",        has: Arg::Yes, num: CmdOP::Size.to(), },
    Opt { sho: 0u8, lon: "scodec",   has: Arg::Yes, num: CmdOP::SubtiCodec.to(), },
    Opt { sho: 0u8, lon: "seek_timestamp", has: Arg::No, num: CmdOP::SeekTimestamp.to(), },
    Opt { sho: 0u8, lon: "sn",       has: Arg::No,  num: CmdOP::SubtiNone.to(), },
    Opt { sho: 0u8, lon: "spre",     has: Arg::Yes, num: CmdOP::SubtiPreset.to(), },
    Opt { sho: 0u8, lon: "ss",       has: Arg::Yes, num: CmdOP::StartTime.to(), },
    Opt { sho: 0u8, lon: "sseof",    has: Arg::Yes, num: CmdOP::StartTimeRel.to(), },
    Opt { sho: 0u8, lon: "stag",     has: Arg::Yes, num: CmdOP::SubtiTag.to(), },
    Opt { sho: 0u8, lon: "stats",    has: Arg::No,  num: CmdOP::Stats.to(), },
    Opt { sho: 0u8, lon: "t",        has: Arg::Yes, num: CmdOP::Duration.to(), },
    Opt { sho: 0u8, lon: "target",   has: Arg::Yes, num: CmdOP::Target.to(), },
    Opt { sho: 0u8, lon: "timecode", has: Arg::Yes, num: CmdOP::TimeCode.to(), },
    Opt { sho: 0u8, lon: "timestamp",has: Arg::Yes, num: CmdOP::Timestamp.to(), },
    Opt { sho: 0u8, lon: "to",       has: Arg::Yes, num: CmdOP::TimeStop.to(), },
    Opt { sho: 0u8, lon: "v",        has: Arg::Yes, num: CmdOP::Loglevel.to(), },
    Opt { sho: 0u8, lon: "vcodec",   has: Arg::Yes, num: CmdOP::VideoCodec.to(), },
    Opt { sho: 0u8, lon: "vf",       has: Arg::Yes, num: CmdOP::VideoFilter.to(), },
    Opt { sho: 0u8, lon: "vframes",  has: Arg::Yes, num: CmdOP::VideoFrames.to(), },
    Opt { sho: 0u8, lon: "vn",       has: Arg::No,  num: CmdOP::VideoNone.to(), },
    Opt { sho: 0u8, lon: "vol",      has: Arg::Yes, num: CmdOP::Volume.to(), },
    Opt { sho: 0u8, lon: "y",        has: Arg::No,  num: CmdOP::YesOOF.to(), },
];
/*
#[rustfmt::skip]
const OPT_ARY: [Opt;66] = [
    // Print help / information / capabilities
    Opt { sho: 0u8, lon: "h",        has: Arg::Maybe, num: CmdOP::Help.to(), },
    Opt { sho: 0u8, lon: "L",        has: Arg::No,  num: CmdOP::ShowLicense.to(), },
    Opt { sho: 0u8, lon: "buildconf",has: Arg::No,  num: CmdOP::ShowBuildconf.to(), },
    Opt { sho: 0u8, lon: "formats",  has: Arg::No,  num: CmdOP::ShowFormats.to(), },
    // Global options
    Opt { sho: 0u8, lon: "loglevel", has: Arg::Yes, num: CmdOP::Loglevel.to(), },
    Opt { sho: 0u8, lon: "v",        has: Arg::Yes, num: CmdOP::Loglevel.to(), },
    Opt { sho: 0u8, lon: "report",   has: Arg::No,  num: CmdOP::Report.to(), },
    Opt { sho: 0u8, lon: "max_alloc",has: Arg::Yes, num: CmdOP::MaxAlloc.to(), },
    Opt { sho: 0u8, lon: "y",        has: Arg::No,  num: CmdOP::YesOOF.to(), },
    Opt { sho: 0u8, lon: "n",        has: Arg::No,  num: CmdOP::NeverOOF.to(), },
    Opt { sho: 0u8, lon: "ignore_unknown", has: Arg::No, num: CmdOP::IgnoreUnknown.to(), },
    Opt { sho: 0u8, lon: "filter_threads", has: Arg::Yes, num: CmdOP::FilterThreads.to(), },
    Opt { sho: 0u8, lon: "filter_complex_threads", has: Arg::Yes, num: CmdOP::FilterComplexThreads.to(), },
    Opt { sho: 0u8, lon: "stats",    has: Arg::No,  num: CmdOP::Stats.to(), },
    // Per-file main options
    Opt { sho: 0u8, lon: "f",        has: Arg::Yes, num: CmdOP::Format.to(), },
    Opt { sho: 0u8, lon: "c",        has: Arg::Yes, num: CmdOP::Codec.to(), },
    Opt { sho: 0u8, lon: "codec",    has: Arg::Yes, num: CmdOP::Codec.to(), },
    Opt { sho: 0u8, lon: "pre",      has: Arg::Yes, num: CmdOP::Preset.to(), },
    Opt { sho: 0u8, lon: "map_metadata", has: Arg::Yes, num: CmdOP::MapMetadata.to(), },
    Opt { sho: 0u8, lon: "t",        has: Arg::Yes, num: CmdOP::Duration.to(), },
    Opt { sho: 0u8, lon: "to",       has: Arg::Yes, num: CmdOP::TimeStop.to(), },
    Opt { sho: 0u8, lon: "fs",       has: Arg::Yes, num: CmdOP::FileSize.to(), },
    Opt { sho: 0u8, lon: "ss",       has: Arg::Yes, num: CmdOP::StartTime.to(), },
    Opt { sho: 0u8, lon: "sseof",    has: Arg::Yes, num: CmdOP::StartTimeRel.to(), },
    Opt { sho: 0u8, lon: "seek_timestamp", has: Arg::No, num: CmdOP::SeekTimestamp.to(), },
    Opt { sho: 0u8, lon: "timestamp",has: Arg::Yes, num: CmdOP::Timestamp.to(), },
    Opt { sho: 0u8, lon: "metadata", has: Arg::Yes, num: CmdOP::Metadata.to(), },
    Opt { sho: 0u8, lon: "program",  has: Arg::Yes, num: CmdOP::Program.to(), },
    Opt { sho: 0u8, lon: "target",   has: Arg::Yes, num: CmdOP::Target.to(), },
    Opt { sho: 0u8, lon: "apad",     has: Arg::No,  num: CmdOP::AudioPad.to(), },
    Opt { sho: 0u8, lon: "frames",   has: Arg::Yes, num: CmdOP::Frames.to(), },
    Opt { sho: 0u8, lon: "filter",   has: Arg::Yes, num: CmdOP::Filter.to(), },
    Opt { sho: 0u8, lon: "filter_script", has: Arg::Yes, num: CmdOP::FilterScript.to(), },
    Opt { sho: 0u8, lon: "reinit_filter", has: Arg::No, num: CmdOP::ReinitFilter.to(), },
    Opt { sho: 0u8, lon: "discard",   has: Arg::No,  num: CmdOP::Discard.to(), },
    Opt { sho: 0u8, lon: "disposition", has: Arg::No, num: CmdOP::Disposition.to(), },
    // Video options
    Opt { sho: 0u8, lon: "vframes",  has: Arg::Yes, num: CmdOP::VideoFrames.to(), },
    Opt { sho: 0u8, lon: "r",        has: Arg::Yes, num: CmdOP::Rate.to(), },
    Opt { sho: 0u8, lon: "s",        has: Arg::Yes, num: CmdOP::Size.to(), },
    Opt { sho: 0u8, lon: "aspect",   has: Arg::Yes, num: CmdOP::Aspect.to(), },
    Opt { sho: 0u8, lon: "bits_per_raw_sample", has: Arg::Yes, num: CmdOP::BitsPerRawSample.to(), },
    Opt { sho: 0u8, lon: "vn",       has: Arg::No,  num: CmdOP::VideoNone.to(), },
    Opt { sho: 0u8, lon: "vcodec",   has: Arg::Yes, num: CmdOP::VideoCodec.to(), },
    Opt { sho: 0u8, lon: "timecode", has: Arg::Yes, num: CmdOP::TimeCode.to(), },
    Opt { sho: 0u8, lon: "pass",     has: Arg::Yes, num: CmdOP::Pass.to(), },
    Opt { sho: 0u8, lon: "vf",       has: Arg::Yes, num: CmdOP::VideoFilter.to(), },
    Opt { sho: 0u8, lon: "ab",       has: Arg::Yes, num: CmdOP::AudioBitrate.to(), },
    Opt { sho: 0u8, lon: "b",        has: Arg::Yes, num: CmdOP::VideoBitrate.to(), },
    Opt { sho: 0u8, lon: "dn",       has: Arg::Yes, num: CmdOP::DataNone.to(), },
    // Audio options
    Opt { sho: 0u8, lon: "aframes",  has: Arg::Yes, num: CmdOP::AudioFrames.to(), },
    Opt { sho: 0u8, lon: "aq",       has: Arg::Yes, num: CmdOP::AudioQuality.to(), },
    Opt { sho: 0u8, lon: "ar",       has: Arg::Yes, num: CmdOP::AudioRate.to(), },
    Opt { sho: 0u8, lon: "ac",       has: Arg::Yes, num: CmdOP::AudioChannel.to(), },
    Opt { sho: 0u8, lon: "an",       has: Arg::No,  num: CmdOP::AudioNone.to(), },
    Opt { sho: 0u8, lon: "acodec",   has: Arg::Yes, num: CmdOP::AudioCodec.to(), },
    Opt { sho: 0u8, lon: "vol",      has: Arg::Yes, num: CmdOP::Volume.to(), },
    Opt { sho: 0u8, lon: "af",       has: Arg::Yes, num: CmdOP::AudioFilter.to(), },
    // Subtitle options
    Opt { sho: 0u8, lon: "sn",       has: Arg::No,  num: CmdOP::SubtiNone.to(), },
    Opt { sho: 0u8, lon: "scodec",   has: Arg::Yes, num: CmdOP::SubtiCodec.to(), },
    Opt { sho: 0u8, lon: "stag",     has: Arg::Yes, num: CmdOP::SubtiTag.to(), },
    Opt { sho: 0u8, lon: "fix_sub_duration", has: Arg::No, num: CmdOP::FixSubDuration.to(), },
    Opt { sho: 0u8, lon: "canvas_size", has: Arg::Yes, num: CmdOP::CanvasSize.to(), },
    Opt { sho: 0u8, lon: "spre",     has: Arg::Yes, num: CmdOP::SubtiPreset.to(), },
    // in file
    Opt { sho: 0u8, lon: "i",        has: Arg::Yes, num: CmdOP::InFile.to(), },
    //
    Opt { sho: b'H', lon: "-help",    has: Arg::No,  num: CmdOP::Help.to(), },
    Opt { sho: b'V', lon: "-version", has: Arg::No,  num: CmdOP::Version.to(), },
];
*/

#[rustfmt::skip]
const OPT_ARY_SHO_IDX: [(u8,usize);2] = [
    (b'H',64), (b'V',65)
];

//----------------------------------------------------------------------
#[derive(Debug, Default)]
struct CmdOptConf {
    pub opt_program: String,
    // Global options
    pub glb_loglevel: String,
    pub glb_report: bool,
    pub glb_max_alloc: u32,
    pub glb_yes_oof: bool,
    pub glb_never_oof: bool,
    pub glb_ignore_unknown: bool,
    pub glb_filter_threads: u32,
    pub glb_filter_complex_threads: u32,
    pub glb_stats: bool,
    //
    pub in_files: Vec<FileConf>,
    pub out_file: FileConf,
    //
    pub arg_params: Vec<String>,
}

#[derive(Debug, Default)]
struct PerFileOptConf {
    // Per-file main options
    pub pfl_format: String,
    pub pfl_codec: String,
    pub pfl_preset: String,
    pub pfl_map_metadata: String,
    pub pfl_duration: String,
    pub pfl_timestop: String,
    pub pfl_filesize: u64,
    pub pfl_start_time: String,
    pub pfl_start_time_rel: String,
    pub pfl_seek_timestamp: bool,
    pub pfl_timestamp: String,
    pub pfl_metadata: String,
    pub pfl_program: String,
    pub pfl_target: String,
    pub pfl_apad: bool,
    pub pfl_frames: String,
    pub pfl_filter: String,
    pub pfl_filter_script: String,
    pub pfl_reinit_filter: bool,
    pub pfl_discard: bool,
    pub pfl_disposition: bool,
    // Video options
    pub vid_video_frames: String,
    pub vid_rate: String,
    pub vid_size: u64,
    pub vid_aspect: String,
    pub vid_bits_per_raw_sample: String,
    pub vid_video_none: bool,
    pub vid_vcodec: String,
    pub vid_timecode: String,
    pub vid_pass: String,
    pub vid_video_filter: String,
    pub vid_audio_bitrate: String,
    pub vid_video_bitrate: String,
    pub vid_data_none: bool,
    // Audio options
    pub aud_audio_frames: String,
    pub aud_audio_quality: String,
    pub aud_audio_rate: String,
    pub aud_audio_channel: String,
    pub aud_audio_none: bool,
    pub aud_audio_codec: String,
    pub aud_volume: String,
    pub aud_audio_filter: String,
    // Subtitle options
    pub sbt_subtitle_none: bool,
    pub sbt_subtitle_codec: String,
    pub sbt_subtitle_tag: String,
    pub sbt_fix_sub_duration: bool,
    pub sbt_canvas_size: String,
    pub sbt_subtitle_preset: String,
}

#[derive(Debug, Default)]
struct FileConf {
    pub conf: PerFileOptConf,
    pub file: String,
}

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {} ({})",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"),
        "examples/ffmpeg" )
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}",
        program,
        "[options] [[infile options] -i infile]... {[outfile options] <outfile>}...")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message("ffmpeg");
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT,
        ARGUMENTS_TEXT, EXAMPLES_TEXT].join("\n")
}

#[inline(never)]
fn print_help_and_exit(conf: &CmdOptConf) {
    print!("{}", help_message(&conf.opt_program));
    std::process::exit(0);
}

#[inline(never)]
fn print_version_and_exit(conf: &CmdOptConf) {
    print!("{}\n", version_message(&conf.opt_program));
    std::process::exit(0);
}

#[inline(never)]
fn value_to_string(nv: &NameVal<'_>) -> Result<String, OptParseError> {
    match nv.val {
        Some(x) => Ok(x.to_string()),
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

#[inline(never)]
fn value_to_u32(nv: &NameVal<'_>) -> Result<u32, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u32>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

#[inline(never)]
fn value_to_u64(nv: &NameVal<'_>) -> Result<u64, OptParseError> {
    match nv.val {
        Some(x) => match x.parse::<u64>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                &nv.opt.lon,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(&nv.opt.lon)),
    }
}

#[inline(never)]
fn parse_match(
    conf: &mut CmdOptConf,
    perfile_conf: &mut PerFileOptConf,
    nv: &NameVal<'_>,
) -> Result<(), OptParseError> {
    match CmdOP::from(nv.opt.num) {
        CmdOP::Help => {
            print_help_and_exit(conf);
        }
        CmdOP::Version => {
            print_version_and_exit(conf);
        }
        // Print help / information / capabilities
        CmdOP::ShowLicense => {
            std::process::exit(0);
        }
        CmdOP::ShowBuildconf => {
            std::process::exit(0);
        }
        CmdOP::ShowFormats => {
            std::process::exit(0);
        }
        // Global options
        CmdOP::Loglevel => {
            conf.glb_loglevel = value_to_string(nv)?;
        }
        CmdOP::Report => {
            conf.glb_report = true;
        }
        CmdOP::MaxAlloc => {
            conf.glb_max_alloc = value_to_u32(nv)?;
        }
        CmdOP::YesOOF => {
            conf.glb_yes_oof = true;
        }
        CmdOP::NeverOOF => {
            conf.glb_never_oof = true;
        }
        CmdOP::IgnoreUnknown => {
            conf.glb_ignore_unknown = true;
        }
        CmdOP::FilterThreads => {
            conf.glb_max_alloc = value_to_u32(nv)?;
        }
        CmdOP::FilterComplexThreads => {
            conf.glb_max_alloc = value_to_u32(nv)?;
        }
        CmdOP::Stats => {
            conf.glb_stats = true;
        }
        // Per-file main options
        CmdOP::Format => {
            perfile_conf.pfl_format = value_to_string(nv)?;
        }
        CmdOP::Codec => {
            perfile_conf.pfl_codec = value_to_string(nv)?;
        }
        CmdOP::Preset => {
            perfile_conf.pfl_preset = value_to_string(nv)?;
        }
        CmdOP::MapMetadata => {
            perfile_conf.pfl_map_metadata = value_to_string(nv)?;
        }
        CmdOP::Duration => {
            perfile_conf.pfl_duration = value_to_string(nv)?;
        }
        CmdOP::TimeStop => {
            perfile_conf.pfl_timestop = value_to_string(nv)?;
        }
        CmdOP::FileSize => {
            perfile_conf.pfl_filesize = value_to_u64(nv)?;
        }
        CmdOP::StartTime => {
            perfile_conf.pfl_start_time = value_to_string(nv)?;
        }
        CmdOP::StartTimeRel => {
            perfile_conf.pfl_start_time_rel = value_to_string(nv)?;
        }
        CmdOP::SeekTimestamp => {
            perfile_conf.pfl_seek_timestamp = true;
        }
        CmdOP::Timestamp => {
            perfile_conf.pfl_timestamp = value_to_string(nv)?;
        }
        CmdOP::Metadata => {
            perfile_conf.pfl_metadata = value_to_string(nv)?;
        }
        CmdOP::Program => {
            perfile_conf.pfl_program = value_to_string(nv)?;
        }
        CmdOP::Target => {
            perfile_conf.pfl_target = value_to_string(nv)?;
        }
        CmdOP::AudioPad => {
            perfile_conf.pfl_apad = true;
        }
        CmdOP::Frames => {
            perfile_conf.pfl_frames = value_to_string(nv)?;
        }
        CmdOP::Filter => {
            perfile_conf.pfl_filter = value_to_string(nv)?;
        }
        CmdOP::FilterScript => {
            perfile_conf.pfl_filter_script = value_to_string(nv)?;
        }
        CmdOP::ReinitFilter => {
            perfile_conf.pfl_reinit_filter = true;
        }
        CmdOP::Discard => {
            perfile_conf.pfl_discard = true;
        }
        CmdOP::Disposition => {
            perfile_conf.pfl_disposition = true;
        }
        // Video options
        CmdOP::VideoFrames => {
            perfile_conf.vid_video_frames = value_to_string(nv)?;
        }
        CmdOP::Rate => {
            perfile_conf.vid_rate = value_to_string(nv)?;
        }
        CmdOP::Size => {
            perfile_conf.vid_size = value_to_u64(nv)?;
        }
        CmdOP::Aspect => {
            perfile_conf.vid_aspect = value_to_string(nv)?;
        }
        CmdOP::BitsPerRawSample => {
            perfile_conf.vid_bits_per_raw_sample = value_to_string(nv)?;
        }
        CmdOP::VideoNone => {
            perfile_conf.vid_video_none = true;
        }
        CmdOP::VideoCodec => {
            perfile_conf.vid_vcodec = value_to_string(nv)?;
        }
        CmdOP::TimeCode => {
            perfile_conf.vid_timecode = value_to_string(nv)?;
        }
        CmdOP::Pass => {
            perfile_conf.vid_pass = value_to_string(nv)?;
        }
        CmdOP::VideoFilter => {
            perfile_conf.vid_video_filter = value_to_string(nv)?;
        }
        CmdOP::AudioBitrate => {
            perfile_conf.vid_audio_bitrate = value_to_string(nv)?;
        }
        CmdOP::VideoBitrate => {
            perfile_conf.vid_video_bitrate = value_to_string(nv)?;
        }
        CmdOP::DataNone => {
            perfile_conf.vid_data_none = true;
        }
        // Audio options
        CmdOP::AudioFrames => {
            perfile_conf.aud_audio_frames = value_to_string(nv)?;
        }
        CmdOP::AudioQuality => {
            perfile_conf.aud_audio_quality = value_to_string(nv)?;
        }
        CmdOP::AudioRate => {
            perfile_conf.aud_audio_rate = value_to_string(nv)?;
        }
        CmdOP::AudioChannel => {
            perfile_conf.aud_audio_channel = value_to_string(nv)?;
        }
        CmdOP::AudioNone => {
            perfile_conf.aud_audio_none = true;
        }
        CmdOP::AudioCodec => {
            perfile_conf.aud_audio_codec = value_to_string(nv)?;
        }
        CmdOP::Volume => {
            perfile_conf.aud_volume = value_to_string(nv)?;
        }
        CmdOP::AudioFilter => {
            perfile_conf.aud_audio_filter = value_to_string(nv)?;
        }
        // Subtitle options
        CmdOP::SubtiNone => {
            perfile_conf.sbt_subtitle_none = true;
        }
        CmdOP::SubtiCodec => {
            perfile_conf.sbt_subtitle_codec = value_to_string(nv)?;
        }
        CmdOP::SubtiTag => {
            perfile_conf.sbt_subtitle_tag = value_to_string(nv)?;
        }
        CmdOP::FixSubDuration => {
            perfile_conf.sbt_fix_sub_duration = true;
        }
        CmdOP::CanvasSize => {
            perfile_conf.sbt_canvas_size = value_to_string(nv)?;
        }
        CmdOP::SubtiPreset => {
            perfile_conf.sbt_subtitle_preset = value_to_string(nv)?;
        }
        // in file
        CmdOP::InFile => {
            // nothing todo
        }
    }
    Ok(())
}

fn parse_cmdopts(program: &str, args: Vec<&str>) -> Result<CmdOptConf, OptParseError> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    //
    let lex = Lex::create_with(&OPT_ARY, &OPT_ARY_SHO_IDX);
    let tokens = match lex.tokens_from(&args) {
        Ok(t) => t,
        Err(errs) => {
            return Err(errs);
        }
    };
    //
    let mut perfile_conf = PerFileOptConf::default();
    for nv in tokens.namevals.iter() {
        match parse_match(&mut conf, &mut perfile_conf, &nv) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
        if CmdOP::from(nv.opt.num) == CmdOP::InFile {
            let in_file = value_to_string(nv)?;
            conf.in_files.push(FileConf {
                file: in_file,
                conf: perfile_conf,
            });
            perfile_conf = PerFileOptConf::default();
        }
    }
    //
    let free = tokens.free;
    if !free.is_empty() {
        conf.arg_params.extend(free.iter().map(|&s| s.to_string()));
        //
        let out_file = conf.arg_params[0].clone();
        conf.out_file = FileConf {
            file: out_file,
            conf: perfile_conf,
        };
    }
    //
    Ok(conf)
}

//----------------------------------------------------------------------
fn create_conf() -> Result<CmdOptConf, OptParseError> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}

fn run(conf: &CmdOptConf) {
    eprintln!("{:?}", conf);
}

//----------------------------------------------------------------------
fn main() {
    //
    let conf = match create_conf() {
        Ok(conf) => conf,
        Err(err) => {
            const TRY_HELP_MSG: &str = "Try --help for help.";
            eprintln!("{}\n{}", err, TRY_HELP_MSG);
            std::process::exit(1);
        }
    };
    //
    run(&conf);
    //
    std::process::exit(0);
}

#[cfg(all(feature = "single_error", feature = "long_only"))]
mod example {
    #[test]
    fn test_ffmpeg_0() {
        assert_eq!(std::mem::size_of::<super::CmdOptConf>(), 1040);
        assert_eq!(std::mem::size_of::<super::FileConf>(), 920);
        assert_eq!(std::mem::size_of::<super::PerFileOptConf>(), 896);
    }
    #[test]
    fn test_ffmpeg_1() {
        let program = "test-ffmpeg";
        #[rustfmt::skip]
        let args = vec![];
        //
        let conf = match super::parse_cmdopts(program, args) {
            Ok(conf) => conf,
            Err(err) => {
                assert_eq!(format!("{}", err), "");
                unreachable!();
            }
        };
        //
        let thing = format!("{:?}", conf);
        let expect = concat!(
            "CmdOptConf {",
            " opt_program: \"test-ffmpeg\",",
            " glb_loglevel: \"\",",
            " glb_report: false,",
            " glb_max_alloc: 0,",
            " glb_yes_oof: false,",
            " glb_never_oof: false,",
            " glb_ignore_unknown: false,",
            " glb_filter_threads: 0,",
            " glb_filter_complex_threads: 0,",
            " glb_stats: false,",
            " in_files: [",
            "],",
            " out_file: FileConf {",
            " conf: PerFileOptConf {",
            " pfl_format: \"\",",
            " pfl_codec: \"\",",
            " pfl_preset: \"\",",
            " pfl_map_metadata: \"\",",
            " pfl_duration: \"\",",
            " pfl_timestop: \"\",",
            " pfl_filesize: 0,",
            " pfl_start_time: \"\",",
            " pfl_start_time_rel: \"\",",
            " pfl_seek_timestamp: false,",
            " pfl_timestamp: \"\",",
            " pfl_metadata: \"\",",
            " pfl_program: \"\",",
            " pfl_target: \"\",",
            " pfl_apad: false,",
            " pfl_frames: \"\",",
            " pfl_filter: \"\",",
            " pfl_filter_script: \"\",",
            " pfl_reinit_filter: false,",
            " pfl_discard: false,",
            " pfl_disposition: false,",
            " vid_video_frames: \"\",",
            " vid_rate: \"\",",
            " vid_size: 0,",
            " vid_aspect: \"\",",
            " vid_bits_per_raw_sample: \"\",",
            " vid_video_none: false,",
            " vid_vcodec: \"\",",
            " vid_timecode: \"\",",
            " vid_pass: \"\",",
            " vid_video_filter: \"\",",
            " vid_audio_bitrate: \"\",",
            " vid_video_bitrate: \"\",",
            " vid_data_none: false,",
            " aud_audio_frames: \"\",",
            " aud_audio_quality: \"\",",
            " aud_audio_rate: \"\",",
            " aud_audio_channel: \"\",",
            " aud_audio_none: false,",
            " aud_audio_codec: \"\",",
            " aud_volume: \"\",",
            " aud_audio_filter: \"\",",
            " sbt_subtitle_none: false,",
            " sbt_subtitle_codec: \"\",",
            " sbt_subtitle_tag: \"\",",
            " sbt_fix_sub_duration: false,",
            " sbt_canvas_size: \"\",",
            " sbt_subtitle_preset: \"\"",
            " },",
            " file: \"\"",
            " },",
            " arg_params: []",
            " }",
        );
        assert_eq!(thing, expect);
    }
    #[test]
    fn test_ffmpeg_2() {
        let program = "test-ffmpeg";
        #[rustfmt::skip]
        let args = vec!["-f", "oss", "-i", "/dev/dsp", "-f", "video4linux2", "-i", "/dev/video0", "-vcodec", "mpeg4", "-acodec", "aac", "/tmp/out.mp4"];
        //
        let conf = match super::parse_cmdopts(program, args) {
            Ok(conf) => conf,
            Err(err) => {
                assert_eq!(format!("{}", err), "");
                unreachable!();
            }
        };
        //
        let thing = format!("{:?}", conf);
        let expect = concat!(
            "CmdOptConf {",
            " opt_program: \"test-ffmpeg\",",
            " glb_loglevel: \"\",",
            " glb_report: false,",
            " glb_max_alloc: 0,",
            " glb_yes_oof: false,",
            " glb_never_oof: false,",
            " glb_ignore_unknown: false,",
            " glb_filter_threads: 0,",
            " glb_filter_complex_threads: 0,",
            " glb_stats: false,",
            " in_files: [",
            "FileConf {",
            " conf: PerFileOptConf {",
            " pfl_format: \"oss\",",
            " pfl_codec: \"\",",
            " pfl_preset: \"\",",
            " pfl_map_metadata: \"\",",
            " pfl_duration: \"\",",
            " pfl_timestop: \"\",",
            " pfl_filesize: 0,",
            " pfl_start_time: \"\",",
            " pfl_start_time_rel: \"\",",
            " pfl_seek_timestamp: false,",
            " pfl_timestamp: \"\",",
            " pfl_metadata: \"\",",
            " pfl_program: \"\",",
            " pfl_target: \"\",",
            " pfl_apad: false,",
            " pfl_frames: \"\",",
            " pfl_filter: \"\",",
            " pfl_filter_script: \"\",",
            " pfl_reinit_filter: false,",
            " pfl_discard: false,",
            " pfl_disposition: false,",
            " vid_video_frames: \"\",",
            " vid_rate: \"\",",
            " vid_size: 0,",
            " vid_aspect: \"\",",
            " vid_bits_per_raw_sample: \"\",",
            " vid_video_none: false,",
            " vid_vcodec: \"\",",
            " vid_timecode: \"\",",
            " vid_pass: \"\",",
            " vid_video_filter: \"\",",
            " vid_audio_bitrate: \"\",",
            " vid_video_bitrate: \"\",",
            " vid_data_none: false,",
            " aud_audio_frames: \"\",",
            " aud_audio_quality: \"\",",
            " aud_audio_rate: \"\",",
            " aud_audio_channel: \"\",",
            " aud_audio_none: false,",
            " aud_audio_codec: \"\",",
            " aud_volume: \"\",",
            " aud_audio_filter: \"\",",
            " sbt_subtitle_none: false,",
            " sbt_subtitle_codec: \"\",",
            " sbt_subtitle_tag: \"\",",
            " sbt_fix_sub_duration: false,",
            " sbt_canvas_size: \"\",",
            " sbt_subtitle_preset: \"\"",
            " },",
            " file: \"/dev/dsp\"",
            " },",
            " FileConf {",
            " conf: PerFileOptConf {",
            " pfl_format: \"video4linux2\",",
            " pfl_codec: \"\",",
            " pfl_preset: \"\",",
            " pfl_map_metadata: \"\",",
            " pfl_duration: \"\",",
            " pfl_timestop: \"\",",
            " pfl_filesize: 0,",
            " pfl_start_time: \"\",",
            " pfl_start_time_rel: \"\",",
            " pfl_seek_timestamp: false,",
            " pfl_timestamp: \"\",",
            " pfl_metadata: \"\",",
            " pfl_program: \"\",",
            " pfl_target: \"\",",
            " pfl_apad: false,",
            " pfl_frames: \"\",",
            " pfl_filter: \"\",",
            " pfl_filter_script: \"\",",
            " pfl_reinit_filter: false,",
            " pfl_discard: false,",
            " pfl_disposition: false,",
            " vid_video_frames: \"\",",
            " vid_rate: \"\",",
            " vid_size: 0,",
            " vid_aspect: \"\",",
            " vid_bits_per_raw_sample: \"\",",
            " vid_video_none: false,",
            " vid_vcodec: \"\",",
            " vid_timecode: \"\",",
            " vid_pass: \"\",",
            " vid_video_filter: \"\",",
            " vid_audio_bitrate: \"\",",
            " vid_video_bitrate: \"\",",
            " vid_data_none: false,",
            " aud_audio_frames: \"\",",
            " aud_audio_quality: \"\",",
            " aud_audio_rate: \"\",",
            " aud_audio_channel: \"\",",
            " aud_audio_none: false,",
            " aud_audio_codec: \"\",",
            " aud_volume: \"\",",
            " aud_audio_filter: \"\",",
            " sbt_subtitle_none: false,",
            " sbt_subtitle_codec: \"\",",
            " sbt_subtitle_tag: \"\",",
            " sbt_fix_sub_duration: false,",
            " sbt_canvas_size: \"\",",
            " sbt_subtitle_preset: \"\"",
            " },",
            " file: \"/dev/video0\"",
            " }",
            "],",
            " out_file: FileConf {",
            " conf: PerFileOptConf {",
            " pfl_format: \"\",",
            " pfl_codec: \"\",",
            " pfl_preset: \"\",",
            " pfl_map_metadata: \"\",",
            " pfl_duration: \"\",",
            " pfl_timestop: \"\",",
            " pfl_filesize: 0,",
            " pfl_start_time: \"\",",
            " pfl_start_time_rel: \"\",",
            " pfl_seek_timestamp: false,",
            " pfl_timestamp: \"\",",
            " pfl_metadata: \"\",",
            " pfl_program: \"\",",
            " pfl_target: \"\",",
            " pfl_apad: false,",
            " pfl_frames: \"\",",
            " pfl_filter: \"\",",
            " pfl_filter_script: \"\",",
            " pfl_reinit_filter: false,",
            " pfl_discard: false,",
            " pfl_disposition: false,",
            " vid_video_frames: \"\",",
            " vid_rate: \"\",",
            " vid_size: 0,",
            " vid_aspect: \"\",",
            " vid_bits_per_raw_sample: \"\",",
            " vid_video_none: false,",
            " vid_vcodec: \"mpeg4\",",
            " vid_timecode: \"\",",
            " vid_pass: \"\",",
            " vid_video_filter: \"\",",
            " vid_audio_bitrate: \"\",",
            " vid_video_bitrate: \"\",",
            " vid_data_none: false,",
            " aud_audio_frames: \"\",",
            " aud_audio_quality: \"\",",
            " aud_audio_rate: \"\",",
            " aud_audio_channel: \"\",",
            " aud_audio_none: false,",
            " aud_audio_codec: \"aac\",",
            " aud_volume: \"\",",
            " aud_audio_filter: \"\",",
            " sbt_subtitle_none: false,",
            " sbt_subtitle_codec: \"\",",
            " sbt_subtitle_tag: \"\",",
            " sbt_fix_sub_duration: false,",
            " sbt_canvas_size: \"\",",
            " sbt_subtitle_preset: \"\"",
            " },",
            " file: \"/tmp/out.mp4\"",
            " },",
            " arg_params: [\"/tmp/out.mp4\"]",
            " }",
        );
        assert_eq!(thing, expect);
    }
    #[test]
    fn test_ffmpeg_3() {
        let program = "test-ffmpeg";
        #[rustfmt::skip]
        let args = vec!["-v", "debug"];
        //
        let conf = match super::parse_cmdopts(program, args) {
            Ok(conf) => conf,
            Err(err) => {
                assert_eq!(format!("{}", err), "");
                unreachable!();
            }
        };
        //
        let thing = format!("{:?}", conf.glb_loglevel);
        let expect = "\"debug\"";
        assert_eq!(thing, expect);
    }
    #[test]
    fn test_ffmpeg_4() {
        let program = "test-ffmpeg";
        #[rustfmt::skip]
        let args = vec!["-a"];
        //
        let _conf = match super::parse_cmdopts(program, args) {
            Ok(conf) => {
                assert_eq!(format!("{:?}", conf), "");
                unreachable!();
            }
            Err(err) => {
                let thing = format!("{}", err);
                #[cfg(feature = "abbreviate")]
                let expect = concat!(
                    "Ambiguous option: a: possibilities:",
                    " \'--ab\'",
                    " \'--ac\'",
                    " \'--acodec\'",
                    " \'--af\'",
                    " \'--aframes\'",
                    " \'--an\'",
                    " \'--apad\'",
                    " \'--aq\'",
                    " \'--ar\'",
                    " \'--aspect\'",
                );
                #[cfg(not(feature = "abbreviate"))]
                let expect = "Invalid option: a";
                assert_eq!(thing, expect);
            }
        };
    }
    #[test]
    fn test_ffmpeg_5() {
        let program = "test-ffmpeg";
        #[rustfmt::skip]
        let args = vec!["-fil"];
        //
        let _conf = match super::parse_cmdopts(program, args) {
            Ok(conf) => {
                assert_eq!(format!("{:?}", conf), "");
                unreachable!();
            }
            Err(err) => {
                let thing = format!("{}", err);
                #[cfg(feature = "abbreviate")]
                let expect = concat!(
                    "Ambiguous option: fil: possibilities:",
                    " \'--filter\'",
                    " \'--filter_complex_threads\'",
                    " \'--filter_script\'",
                    " \'--filter_threads\'",
                );
                #[cfg(not(feature = "abbreviate"))]
                let expect = "Invalid option: fil";
                assert_eq!(thing, expect);
            }
        };
    }
}
