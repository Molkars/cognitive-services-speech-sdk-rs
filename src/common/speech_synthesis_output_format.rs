/// SpeechSynthesisOutputFormat defines the possible speech synthesis output audio formats.
#[derive(Debug)]
pub enum SpeechSynthesisOutputFormat {
    /// Raw8Khz8BitMonoMULaw stands for raw-8khz-8bit-mono-mulaw
    Raw8Khz8BitMonoMULaw = 1,

    /// Riff16Khz16KbpsMonoSiren stands for riff-16khz-16kbps-mono-siren
    Riff16Khz16KbpsMonoSiren = 2,

    /// Audio16Khz16KbpsMonoSiren stands for audio-16khz-16kbps-mono-siren
    Audio16Khz16KbpsMonoSiren = 3,

    /// Audio16Khz32KBitRateMonoMp3 stands for audio-16khz-32kbitrate-mono-mp3
    Audio16Khz32KBitRateMonoMp3 = 4,

    /// Audio16Khz128KBitRateMonoMp3 stands for audio-16khz-128kbitrate-mono-mp3
    Audio16Khz128KBitRateMonoMp3 = 5,

    /// Audio16Khz64KBitRateMonoMp3 stands for audio-16khz-64kbitrate-mono-mp3
    Audio16Khz64KBitRateMonoMp3 = 6,

    /// Audio24Khz48KBitRateMonoMp3 stands for audio-24khz-48kbitrate-mono-mp3
    Audio24Khz48KBitRateMonoMp3 = 7,

    /// Audio24Khz96KBitRateMonoMp3 stands for audio-24khz-96kbitrate-mono-mp3
    Audio24Khz96KBitRateMonoMp3 = 8,

    /// Audio24Khz160KBitRateMonoMp3 stands for audio-24khz-160kbitrate-mono-mp3
    Audio24Khz160KBitRateMonoMp3 = 9,

    /// Raw16Khz16BitMonoTrueSilk stands for raw-16khz-16bit-mono-truesilk
    Raw16Khz16BitMonoTrueSilk = 10,

    /// Riff16Khz16BitMonoPcm stands for riff-16khz-16bit-mono-pcm
    Riff16Khz16BitMonoPcm = 11,

    /// Riff8Khz16BitMonoPcm stands for riff-8khz-16bit-mono-pcm
    Riff8Khz16BitMonoPcm = 12,

    /// Riff24Khz16BitMonoPcm stands for riff-24khz-16bit-mono-pcm
    Riff24Khz16BitMonoPcm = 13,

    /// Riff8Khz8BitMonoMULaw stands for riff-8khz-8bit-mono-mulaw
    Riff8Khz8BitMonoMULaw = 14,

    /// Raw16Khz16BitMonoPcm stands for raw-16khz-16bit-mono-pcm
    Raw16Khz16BitMonoPcm = 15,

    /// Raw24Khz16BitMonoPcm stands for raw-24khz-16bit-mono-pcm
    Raw24Khz16BitMonoPcm = 16,

    /// Raw8Khz16BitMonoPcm stands for raw-8khz-16bit-mono-pcm
    Raw8Khz16BitMonoPcm = 17,

    /// Ogg16Khz16BitMonoOpus stands for ogg-16khz-16bit-mono-opus
    Ogg16Khz16BitMonoOpus = 18,

    /// Ogg24Khz16BitMonoOpus stands for ogg-24khz-16bit-mono-opus
    Ogg24Khz16BitMonoOpus = 19,

    /// Raw48Khz16BitMonoPcm stands for raw-48khz-16bit-mono-pcm
    Raw48Khz16BitMonoPcm = 20,

    /// Riff48Khz16BitMonoPcm stands for riff-48khz-16bit-mono-pcm
    Riff48Khz16BitMonoPcm = 21,

    /// Audio48Khz96KBitRateMonoMp3 stands for audio-48khz-96kbitrate-mono-mp3
    Audio48Khz96KBitRateMonoMp3 = 22,

    /// Audio48Khz192KBitRateMonoMp3 stands for audio-48khz-192kbitrate-mono-mp3
    Audio48Khz192KBitRateMonoMp3 = 23,

    /// Ogg48Khz16BitMonoOpus stands for ogg-48khz-16bit-mono-opus
    Ogg48Khz16BitMonoOpus = 24,

    /// Webm16Khz16BitMonoOpus stands for webm-16khz-16bit-mono-opus
    Webm16Khz16BitMonoOpus = 25,

    /// Webm24Khz16BitMonoOpus stands for webm-24khz-16bit-mono-opus
    Webm24Khz16BitMonoOpus = 26,

    /// Raw24Khz16BitMonoTrueSilk stands for raw-24khz-16bit-mono-truesilk
    Raw24Khz16BitMonoTrueSilk = 27,

    /// Raw8Khz8BitMonoALaw stands for raw-8khz-8bit-mono-alaw
    Raw8Khz8BitMonoALaw = 28,

    /// Riff8Khz8BitMonoALaw stands for riff-8khz-8bit-mono-alaw
    Riff8Khz8BitMonoALaw = 29,
}
