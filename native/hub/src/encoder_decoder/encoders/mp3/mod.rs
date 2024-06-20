use crate::messages::dart_signal::{Mp3Bitrate, Mp3Quality};

pub mod mp3 {
    use std::collections::HashMap;

    use mp3lame_encoder::{Builder, DualPcm, Encoder, FlushNoGap, Id3Tag, MonoPcm};

    use crate::{
        encoder_decoder::raw_audio_data::raw_audio_data::{AudioChannels, RawAudioData},
        messages::dart_signal::{Mp3Bitrate, Mp3Quality},
    };

    use super::{get_correct_bitrate, get_correct_quality};

    pub trait Mp3Encoder {
        fn encode_to_mp3(
            &self,
            quality: Mp3Quality,
            bitrate: Mp3Bitrate,
        ) -> Result<Vec<u8>, String>;
    }

    impl Mp3Encoder for RawAudioData {
        fn encode_to_mp3(
            &self,
            quality: Mp3Quality,
            bitrate: Mp3Bitrate,
        ) -> Result<Vec<u8>, String> {
            // TODO
            // 1. find a way to set the album art in the output mp3 file                                        -- done
            // 2. manage and map the channels in the raw audio data to the mp3 file                             -- actually done now, managing 1 or 2 channels
            // 3. try and make options to add more tags which are not atcually exposed by mp3lame_encoder

            // readying the encoder
            let quality = get_correct_quality(quality);
            let bitrate = get_correct_bitrate(bitrate);

            let mut mp3_encoder = Builder::new().expect("Create LAME builder");
            mp3_encoder
                .set_num_channels(self.get_no_of_channels())
                .expect("Setting number of channels");
            mp3_encoder
                .set_sample_rate(self.get_sample_rate())
                .expect("Setting sample rate");
            mp3_encoder.set_brate(bitrate).expect("Setting bitrate");
            mp3_encoder.set_quality(quality).expect("Setting quality");
            let tags_iter = self.get_tags().iter();
            let mut tags_map = HashMap::new();
            for tag in tags_iter {
                let key = match tag.std_key {
                    Some(key) => key,
                    None => {
                        // println!("{}", tag.key);
                        symphonia::core::meta::StandardTagKey::Comment
                    }
                };
                tags_map.insert(key, tag.value.clone());
            }

            let title = match tags_map.get(&symphonia::core::meta::StandardTagKey::TrackTitle) {
                Some(value) => value.to_string().into_bytes(),
                None => "".to_string().into_bytes(),
            };

            let artist = match tags_map.get(&symphonia::core::meta::StandardTagKey::Artist) {
                Some(value) => value.to_string().into_bytes(),
                None => "".to_string().into_bytes(),
            };

            let album = match tags_map.get(&symphonia::core::meta::StandardTagKey::Album) {
                Some(value) => value.to_string().into_bytes(),
                None => "".to_string().into_bytes(),
            };

            let year = match tags_map.get(&symphonia::core::meta::StandardTagKey::Date) {
                Some(value) => value.to_string().into_bytes(),
                None => "".to_string().into_bytes(),
            };

            let comment = match tags_map.get(&symphonia::core::meta::StandardTagKey::Comment) {
                Some(value) => value.to_string().into_bytes(),
                None => "".to_string().into_bytes(),
            };

            match mp3_encoder.set_id3_tag(Id3Tag {
                title: title.as_slice(),
                artist: artist.as_slice(),
                album: album.as_slice(),
                year: year.as_slice(),
                comment: comment.as_slice(),
                album_art: match self.get_album_art() {
                    Some(image) => &image.data,
                    None => {
                        let empty_image: &[u8] = &[];
                        empty_image
                    }
                },
            }) {
                Ok(_) => (),
                Err(err) => return Err(format!("{:?}", err)),
            };

            let mp3_encoder = match mp3_encoder.build() {
                Ok(encoder) => encoder,
                Err(err) => return Err(err.to_string()),
            };

            // encoding the input data

            if self.get_no_of_channels() == 1 {
                return encode_one_channel_input(self.get_audio_data(), mp3_encoder);
            } else {
                return encode_two_channel_input(self.get_audio_data(), mp3_encoder);
            }
        }
    }

    fn encode_two_channel_input(
        audio_data: &HashMap<AudioChannels, Vec<i32>>,
        mut mp3_encoder: Encoder,
    ) -> Result<Vec<u8>, String> {
        let left = match audio_data.get(&AudioChannels::FrontLeft) {
            Some(data) => data,
            None => match audio_data.get(&AudioChannels::SideLeft) {
                Some(data) => data,
                None => match audio_data.get(&AudioChannels::RearLeft) {
                    Some(data) => data,
                    None => return Err("Could not find appropriate left channel".to_string()),
                },
            },
        };

        let right = match audio_data.get(&AudioChannels::FrontRight) {
            Some(data) => data,
            None => match audio_data.get(&AudioChannels::SideRight) {
                Some(data) => data,
                None => match audio_data.get(&AudioChannels::RearRight) {
                    Some(data) => data,
                    None => return Err("Could not find appropriate right channel".to_string()),
                },
            },
        };

        let input = DualPcm { left, right };
        let len = input.left.len();
        // On setting the maximum image size to 16MB the
        // encoder was sometime throwing error that the output
        // buffer is not big enough for the encoded
        // data, this solved the problem.
        let len = len + (16 * 1024 * 1024);

        let mut mp3_out_buffer = Vec::new();
        mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(len));
        let encoded_size = match mp3_encoder.encode(input, mp3_out_buffer.spare_capacity_mut()) {
            Ok(size) => size,

            Err(err) => return Err(err.to_string()),
        };

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        let encoded_size =
            match mp3_encoder.flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut()) {
                Ok(size) => size,
                Err(err) => return Err(err.to_string()),
            };

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        return Ok(mp3_out_buffer);
    }

    fn encode_one_channel_input(
        audio_data: &HashMap<AudioChannels, Vec<i32>>,
        mut mp3_encoder: Encoder,
    ) -> Result<Vec<u8>, String> {
        let input = match audio_data.iter().next() {
            Some(data) => MonoPcm(data.1),
            None => return Err("Could not find any channels in the mono audio file.".to_string()),
        };

        let len = input.0.len();

        let mut mp3_out_buffer = Vec::new();
        mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(len));
        let encoded_size = match mp3_encoder.encode(input, mp3_out_buffer.spare_capacity_mut()) {
            Ok(size) => size,
            Err(err) => return Err(err.to_string()),
        };

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        let encoded_size =
            match mp3_encoder.flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut()) {
                Ok(size) => size,
                Err(err) => return Err(err.to_string()),
            };

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        return Ok(mp3_out_buffer);
    }
}

fn get_correct_quality(quality: Mp3Quality) -> mp3lame_encoder::Quality {
    match quality {
        Mp3Quality::Best => mp3lame_encoder::Quality::Best,
        Mp3Quality::SecondBest => mp3lame_encoder::Quality::SecondBest,
        Mp3Quality::NearBest => mp3lame_encoder::Quality::NearBest,
        Mp3Quality::VeryNice => mp3lame_encoder::Quality::VeryNice,
        Mp3Quality::Nice => mp3lame_encoder::Quality::Nice,
        Mp3Quality::Good => mp3lame_encoder::Quality::Good,
        Mp3Quality::Decent => mp3lame_encoder::Quality::Decent,
        Mp3Quality::Ok => mp3lame_encoder::Quality::Ok,
        Mp3Quality::SecondWorst => mp3lame_encoder::Quality::SecondWorst,
        Mp3Quality::Worst => mp3lame_encoder::Quality::Worst,
    }
}

fn get_correct_bitrate(bitrate: Mp3Bitrate) -> mp3lame_encoder::Bitrate {
    match bitrate {
        Mp3Bitrate::BitrateUnknownDoNotUse => mp3lame_encoder::Bitrate::Kbps320,
        Mp3Bitrate::Kbps8 => mp3lame_encoder::Bitrate::Kbps8,
        Mp3Bitrate::Kbps16 => mp3lame_encoder::Bitrate::Kbps16,
        Mp3Bitrate::Kbps24 => mp3lame_encoder::Bitrate::Kbps24,
        Mp3Bitrate::Kbps32 => mp3lame_encoder::Bitrate::Kbps32,
        Mp3Bitrate::Kbps40 => mp3lame_encoder::Bitrate::Kbps40,
        Mp3Bitrate::Kbps48 => mp3lame_encoder::Bitrate::Kbps48,
        Mp3Bitrate::Kbps64 => mp3lame_encoder::Bitrate::Kbps64,
        Mp3Bitrate::Kbps80 => mp3lame_encoder::Bitrate::Kbps80,
        Mp3Bitrate::Kbps96 => mp3lame_encoder::Bitrate::Kbps96,
        Mp3Bitrate::Kbps112 => mp3lame_encoder::Bitrate::Kbps112,
        Mp3Bitrate::Kbps128 => mp3lame_encoder::Bitrate::Kbps128,
        Mp3Bitrate::Kbps160 => mp3lame_encoder::Bitrate::Kbps160,
        Mp3Bitrate::Kbps192 => mp3lame_encoder::Bitrate::Kbps192,
        Mp3Bitrate::Kbps224 => mp3lame_encoder::Bitrate::Kbps224,
        Mp3Bitrate::Kbps256 => mp3lame_encoder::Bitrate::Kbps256,
        Mp3Bitrate::Kbps320 => mp3lame_encoder::Bitrate::Kbps320,
    }
}
