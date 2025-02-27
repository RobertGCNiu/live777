use clap::ValueEnum;
use webrtc::{
    api::media_engine::*,
    rtp_transceiver::{rtp_codec::RTCRtpCodecCapability, RTCPFeedback},
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Codec {
    Vp8,
    Vp9,
    H264,
    AV1,
    Opus,
    G722,
    PCMU,
    PCMA,
}

impl Into<RTCRtpCodecCapability> for Codec {
    fn into(self) -> RTCRtpCodecCapability {
        let video_rtcp_feedback = vec![
            RTCPFeedback {
                typ: "goog-remb".to_owned(),
                parameter: "".to_owned(),
            },
            RTCPFeedback {
                typ: "ccm".to_owned(),
                parameter: "fir".to_owned(),
            },
            RTCPFeedback {
                typ: "nack".to_owned(),
                parameter: "".to_owned(),
            },
            RTCPFeedback {
                typ: "nack".to_owned(),
                parameter: "pli".to_owned(),
            },
        ];
        match self {
            Codec::Vp8 => RTCRtpCodecCapability {
                mime_type: MIME_TYPE_VP8.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: "".to_owned(),
                rtcp_feedback: video_rtcp_feedback,
            },
            Codec::Vp9 => RTCRtpCodecCapability {
                mime_type: MIME_TYPE_VP9.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: "profile-id=0".to_owned(),
                rtcp_feedback: video_rtcp_feedback,
            },
            Codec::H264 => RTCRtpCodecCapability {
                mime_type: MIME_TYPE_H264.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line:
                    "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42001f"
                        .to_owned(),
                rtcp_feedback: video_rtcp_feedback,
            },
            Codec::AV1 => RTCRtpCodecCapability {
                mime_type: MIME_TYPE_AV1.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: "profile-id=0".to_owned(),
                rtcp_feedback: video_rtcp_feedback,
            },
            Codec::Opus => RTCRtpCodecCapability {
                mime_type: MIME_TYPE_OPUS.to_owned(),
                clock_rate: 48000,
                channels: 2,
                sdp_fmtp_line: "minptime=10;useinbandfec=1".to_owned(),
                rtcp_feedback: vec![],
            },
            Codec::G722 => RTCRtpCodecCapability {
                mime_type: MIME_TYPE_G722.to_owned(),
                clock_rate: 8000,
                channels: 0,
                sdp_fmtp_line: "".to_owned(),
                rtcp_feedback: vec![],
            },
            Codec::PCMU => RTCRtpCodecCapability {
                mime_type: MIME_TYPE_PCMU.to_owned(),
                clock_rate: 8000,
                channels: 0,
                sdp_fmtp_line: "".to_owned(),
                rtcp_feedback: vec![],
            },
            Codec::PCMA => RTCRtpCodecCapability {
                mime_type: MIME_TYPE_PCMA.to_owned(),
                clock_rate: 8000,
                channels: 0,
                sdp_fmtp_line: "".to_owned(),
                rtcp_feedback: vec![],
            },
        }
    }
}
