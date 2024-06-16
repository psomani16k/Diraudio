pub mod raw_audio_data {
    use std::{collections::HashMap, fs::File, path::Path};

    use symphonia::core::{
        audio::{AudioBufferRef, Channels},
        io::MediaSourceStream,
        meta::{Tag, VendorData, Visual},
    };

    #[derive(Clone, Eq, Hash, PartialEq)]
    pub enum AudioChannels {
        /// Front-left (left) or the Mono channel.
        FrontLeft = 0x0000_0001,
        /// Front-right (right) channel.
        FrontRight = 0x0000_0002,
        /// Front-centre (centre) channel.
        FrontCentre = 0x0000_0004,
        /// Low frequency channel 1.
        LFE1 = 0x0000_0008,
        /// Rear-left (surround rear left) channel.
        RearLeft = 0x0000_0010,
        /// Rear-right (surround rear right) channel.
        RearRight = 0x0000_0020,
        /// Front left-of-centre (left center) channel.
        FrontLeftCentre = 0x0000_0040,
        /// Front right-of-centre (right center) channel.
        FrontRightCentre = 0x0000_0080,
        /// Rear-centre (surround rear centre) channel.
        RearCentre = 0x0000_0100,
        /// Side left (surround left) channel.
        SideLeft = 0x0000_0200,
        /// Side right (surround right) channel.
        SideRight = 0x0000_0400,
        /// Top centre channel.
        TopCentre = 0x0000_0800,
        /// Top front-left channel.
        TopFrontLeft = 0x0000_1000,
        /// Top centre channel.
        TopFrontCentre = 0x0000_2000,
        /// Top front-right channel.
        TopFrontRight = 0x0000_4000,
        /// Top rear-left channel.
        TopRearLeft = 0x0000_8000,
        /// Top rear-centre channel.
        TopRearCentre = 0x0001_0000,
        /// Top rear-right channel.
        TopRearRight = 0x0002_0000,
        /// Rear left-of-centre channel.
        RearLeftCentre = 0x0004_0000,
        /// Rear right-of-centre channel.
        RearRightCentre = 0x0008_0000,
        /// Front left-wide channel.
        FrontLeftWide = 0x0010_0000,
        /// Front right-wide channel.
        FrontRightWide = 0x0020_0000,
        /// Front left-high channel.
        FrontLeftHigh = 0x0040_0000,
        /// Front centre-high channel.
        FrontCentreHigh = 0x0080_0000,
        /// Front right-high channel.
        FrontRightHigh = 0x0100_0000,
        /// Low frequency channel 2.
        LFE2 = 0x0200_0000,
    }

    #[derive(Clone)]
    pub struct RawAudioData {
        audio_data: HashMap<AudioChannels, Vec<i32>>,
        audio_sample_rate: u32,
        audio_bits_per_sample: u32,
        image_data: Option<Visual>,
        vendor_data: Option<VendorData>,
        tag_data: Vec<Tag>,
    }

    impl RawAudioData {
        pub fn new(
            audio: HashMap<AudioChannels, Vec<i32>>,
            sample_rate: u32,
            bits_per_sample: u32,
            image: Option<Visual>,
            vendor: Option<VendorData>,
            tags: Vec<Tag>,
        ) -> Self {
            RawAudioData {
                audio_data: audio,
                audio_bits_per_sample: bits_per_sample,
                audio_sample_rate: sample_rate,
                image_data: image,
                vendor_data: vendor,
                tag_data: tags,
            }
        }

        pub fn new_from_path(path: &Path) -> Result<RawAudioData, String> {
            let codecs = symphonia::default::get_codecs();
            let probe = symphonia::default::get_probe();
            let mss = MediaSourceStream::new(
                Box::new(File::open(path).expect("Setting MediaSource from file path")),
                Default::default(),
            );
            let mut reader = probe
                .format(
                    &Default::default(),
                    mss,
                    &Default::default(),
                    &Default::default(),
                )
                .unwrap()
                .format;
            let tracks = reader.tracks();

            // audio extraction part
            let mut decoder = codecs
                .make(&tracks[0].codec_params, &Default::default())
                .unwrap();
            //println!("{:?}", decoder.codec_params());
            let sample_rate = decoder.codec_params().sample_rate.unwrap();
            let bits_per_sample = decoder.codec_params().bits_per_sample.unwrap();
            let mut channel_data: HashMap<u8, Vec<i32>> = HashMap::new();
            let channel_count = tracks[0].codec_params.channels.unwrap();
            for i in 0..channel_count.count() {
                channel_data.insert(i.try_into().unwrap(), Vec::new());
            }

            loop {
                match reader.next_packet() {
                    Ok(packet) => {
                        let decoded = decoder.decode(&packet).unwrap();
                        match decoded {
                            AudioBufferRef::S32(buf) => {
                                let channels = buf.planes();
                                let channels = channels.planes();
                                let channel_count = channels.len();
                                for i in 0..channel_count {
                                    let mut plane_vec: Vec<i32> = channels[i].to_vec();
                                    channel_data
                                        .entry(i.try_into().unwrap())
                                        .or_insert(Vec::new())
                                        .append(&mut plane_vec);
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
                    Err(symphonia::core::errors::Error::IoError(_err)) => {
                        break;
                    }
                    Err(e) => panic!("Failed to decode, error: {}", e),
                };
            }

            let channel_mappings = HashMap::from([
                (
                    Channels::from_bits_truncate(0x0000_0001),
                    AudioChannels::FrontLeft,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0002),
                    AudioChannels::FrontRight,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0004),
                    AudioChannels::FrontCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0008),
                    AudioChannels::LFE1,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0010),
                    AudioChannels::RearLeft,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0020),
                    AudioChannels::RearRight,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0040),
                    AudioChannels::FrontLeftCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0080),
                    AudioChannels::FrontRightCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0100),
                    AudioChannels::RearCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0200),
                    AudioChannels::SideLeft,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0400),
                    AudioChannels::SideRight,
                ),
                (
                    Channels::from_bits_truncate(0x0000_0800),
                    AudioChannels::TopCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0000_1000),
                    AudioChannels::TopFrontLeft,
                ),
                (
                    Channels::from_bits_truncate(0x0000_2000),
                    AudioChannels::TopFrontCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0000_4000),
                    AudioChannels::TopFrontRight,
                ),
                (
                    Channels::from_bits_truncate(0x0000_8000),
                    AudioChannels::TopRearLeft,
                ),
                (
                    Channels::from_bits_truncate(0x0001_0000),
                    AudioChannels::TopRearCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0002_0000),
                    AudioChannels::TopRearRight,
                ),
                (
                    Channels::from_bits_truncate(0x0004_0000),
                    AudioChannels::RearLeftCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0008_0000),
                    AudioChannels::RearRightCentre,
                ),
                (
                    Channels::from_bits_truncate(0x0010_0000),
                    AudioChannels::FrontLeftWide,
                ),
                (
                    Channels::from_bits_truncate(0x0020_0000),
                    AudioChannels::FrontRightWide,
                ),
                (
                    Channels::from_bits_truncate(0x0040_0000),
                    AudioChannels::FrontLeftHigh,
                ),
                (
                    Channels::from_bits_truncate(0x0080_0000),
                    AudioChannels::FrontCentreHigh,
                ),
                (
                    Channels::from_bits_truncate(0x0100_0000),
                    AudioChannels::FrontRightHigh,
                ),
                (
                    Channels::from_bits_truncate(0x0200_0000),
                    AudioChannels::LFE2,
                ),
            ]);
            let named_channels = decoder.codec_params().channels.unwrap();
            let mut final_channels: HashMap<AudioChannels, Vec<i32>> = HashMap::new();
            let mut channel_data_key: u8 = 0;
            for i in named_channels.iter() {
                final_channels.insert(
                    channel_mappings.get(&i).unwrap().clone(),
                    channel_data.get(&channel_data_key).unwrap().to_vec(),
                );
                channel_data_key = channel_data_key + 1;
            }
            // working with meta data
            let latest_meta = reader.metadata().skip_to_latest().unwrap().clone();
            let visual_data = latest_meta.visuals().first().cloned();
            let vendor_data = latest_meta.vendor_data().first().cloned();
            let tag_data: Vec<Tag> = latest_meta.tags().to_vec();
            //println!("{:?}", tag_data);

            let raw_audio_data = RawAudioData::new(
                final_channels,
                sample_rate,
                bits_per_sample,
                visual_data,
                vendor_data,
                tag_data,
            );
            return Ok(raw_audio_data);
        }

        pub fn get_album_art(&self) -> &Option<Visual> {
            &self.image_data
        }

        pub fn get_approx_size(&self) -> usize {
            let mut size: usize = 0;
            let iter = self.audio_data.iter();
            for i in iter {
                size = i.1.len() * 4;
            }
            let size = match &self.image_data {
                Some(visual) => {
                    let bits_per_pixel = visual.bits_per_pixel.unwrap();
                    let height = visual.dimensions.unwrap().height;
                    let width = visual.dimensions.unwrap().width;
                    println!("{}", bits_per_pixel);
                    (u32::from(bits_per_pixel) * height * width) as usize + size
                }
                None => size,
            };
            size
        }

        pub fn get_sample_rate(&self) -> u32 {
            self.audio_sample_rate
        }

        pub fn get_tags(&self) -> &Vec<Tag> {
            &self.tag_data
        }

        pub fn get_no_of_channels(&self) -> u8 {
            self.audio_data.len().try_into().unwrap()
        }

        pub fn get_bit_depth(&self) -> u32 {
            self.audio_bits_per_sample
        }

        pub fn get_audio_data(&self) -> &HashMap<AudioChannels, Vec<i32>> {
            return &self.audio_data;
        }
    }

    impl std::fmt::Display for RawAudioData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Audio channels: {}\nBits per samples: {}\nSample rate: {}\nTag Data: {:?}",
                self.audio_data.len(),
                self.audio_bits_per_sample,
                self.audio_sample_rate,
                self.tag_data
            )
        }
    }
}
