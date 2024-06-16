pub mod mp3 {
    use std::collections::HashMap;

    use mp3lame_encoder::{Builder, DualPcm, Encoder, FlushNoGap, Id3Tag, MonoPcm};

    use crate::raw_audio_data::raw_audio_data::{AudioChannels, RawAudioData};

    pub trait Mp3Encoder {
        fn encode_to_mp3(
            &self,
            quality: mp3lame_encoder::Quality,
            bit_rate: mp3lame_encoder::Bitrate,
        ) -> Result<Vec<u8>, String>;
    }

    impl Mp3Encoder for RawAudioData {
        fn encode_to_mp3(
            &self,
            quality: mp3lame_encoder::Quality,
            bit_rate: mp3lame_encoder::Bitrate,
        ) -> Result<Vec<u8>, String> {
            // TODO
            // 1. find a way to set the album art in the output mp3 file                                        -- done
            // 2. manage and map the channels in the raw audio data to the mp3 file                             -- actually done now, managing 1 or 2 channels
            // 3. try and make options to add more tags which are not atcually exposed by mp3lame_encoder

            // readying the encoder

            let mut mp3_encoder = Builder::new().expect("Create LAME builder");
            mp3_encoder
                .set_num_channels(self.get_no_of_channels())
                .expect("Setting number of channels");
            mp3_encoder
                .set_sample_rate(self.get_sample_rate())
                .expect("Setting sample rate");
            mp3_encoder.set_brate(bit_rate).expect("Setting bit_rate");
            mp3_encoder.set_quality(quality).expect("Setting quality");
            let tags_iter = self.get_tags().iter();
            let mut tags_map = HashMap::new();
            for tag in tags_iter {
                let key = match tag.std_key {
                    Some(key) => key,
                    None => {
                        println!("{}", tag.key);
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
                Err(_) => return Err("error in setting id3 tags".to_owned()),
            };

            let mp3_encoder = mp3_encoder.build().expect("Failed to build mp3 encoder.");

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

        let mut mp3_out_buffer = Vec::new();
        mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(len));
        let encoded_size = mp3_encoder
            .encode(input, mp3_out_buffer.spare_capacity_mut())
            .expect("To encode");

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        let encoded_size = mp3_encoder
            .flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut())
            .expect("to flush");

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
        let encoded_size = mp3_encoder
            .encode(input, mp3_out_buffer.spare_capacity_mut())
            .expect("To encode");

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        let encoded_size = mp3_encoder
            .flush::<FlushNoGap>(mp3_out_buffer.spare_capacity_mut())
            .expect("to flush");

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        return Ok(mp3_out_buffer);
    }
}
