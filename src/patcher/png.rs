// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use png::text_metadata::ITXtChunk;

use std::fs::File;
use std::io::BufReader;

use std::io::BufWriter;
use std::path::Path;

use super::Error;

pub struct Patcher;

// https://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/baking/index.html#pngs
//
// var chunk = new iTXt({
//     keyword: 'openbadges',
//     compression: 0,
//     compressionMethod: 0,
//     languageTag: '',
//     translatedKeyword: '',
//     text: signature || JSON.stringify(assertion)
//   })

fn conv_read_err(err: png::DecodingError) -> Error {
    Error::Boxed(Box::new(err))
}

fn conv_write_err(err: png::EncodingError) -> Error {
    Error::Boxed(Box::new(err))
}

fn create_reencoder<'a, W: std::io::Write>(
    w: W,
    info: &'a png::Info,
) -> Result<png::Encoder<'a, W>, Error> {
    log::trace!("Creating encoder ...");
    let mut enc_info = info.clone();
    enc_info.interlaced = false;
    let mut encoder = png::Encoder::with_info(w, enc_info)?;
    // encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);
    // encoder.set_adaptive_filter(png::AdaptiveFilterType::NonAdaptive);
    Ok(encoder)
}

impl super::Patcher for Patcher {
    fn rewrite<P: AsRef<Path>, S: AsRef<str>>(
        input_file: P,
        output_file: P,
        verify: S,
        fail_if_verify_present: bool,
    ) -> Result<(), Error> {
        log::trace!("Opening input file '{}' ...", input_file.as_ref().display());
        let input = File::open(input_file)?;
        let input_buf = BufReader::new(input);

        log::trace!(
            "Opening output file '{}' ...",
            output_file.as_ref().display()
        );
        let output = File::create(output_file)?;
        let w = BufWriter::new(output);

        log::trace!("Decoding ...");
        let decoder = png::Decoder::new(input_buf);
        log::trace!("Creating reader ...");
        let mut reader = decoder.read_info().map_err(conv_read_err)?;
        // Allocate the output buffer.
        let mut buf = vec![
            0;
            reader
                .output_buffer_size()
                .expect("output buffer does not fit into memory")
        ];

        // let mut decoder = png::StreamingDecoder::new(input_buf);
        // let mut decoder_stream = decoder.as_;

        log::trace!("Reading info ...");
        // let mut header = decoder.read_header_info();
        let info = reader.info().clone();

        log::trace!("Creating encoder ...");
        let mut encoder = create_reencoder(w, &info)?;

        log::trace!("Re-Encoding tEXt text-chunks from input ...");
        for chunk in &info.uncompressed_latin1_text {
            encoder
                .add_text_chunk(chunk.keyword.clone(), chunk.text.clone())
                .map_err(conv_write_err)?;
        }
        log::trace!("Re-Encoding zTXt text-chunks from input ...");
        for chunk in &info.compressed_latin1_text {
            encoder
                .add_ztxt_chunk(
                    chunk.keyword.clone(),
                    chunk.get_text().map_err(conv_read_err)?,
                )
                .map_err(conv_write_err)?;
        }
        log::trace!("Re-Encoding iTXt text-chunks from input ...");
        let mut verify_already_as_proposed = false;
        for chunk in &info.utf8_text {
            let text = chunk.get_text().map_err(conv_read_err)?;
            if chunk.keyword == "openbadges" {
                if text == verify.as_ref() {
                    verify_already_as_proposed = true;
                } else if fail_if_verify_present {
                    return Err(Error::VerifyAlreadySet {
                        present: text,
                        proposed: verify.as_ref().to_owned(),
                    });
                } else {
                    // Skip writing this chunk,
                    // to write it further down with the new value.
                    continue;
                }
            }
            encoder
                .add_itxt_chunk(chunk.keyword.clone(), text)
                .map_err(conv_write_err)?;
        }

        log::trace!("Creating writer ...");
        let mut writer = encoder.write_header().map_err(conv_write_err)?;

        if !verify_already_as_proposed {
            log::trace!("Creating OpenBadge iTXt text chunk ...");
            let ob_chunk = ITXtChunk::new("openbadges", verify.as_ref());
            log::trace!("Writing OpenBadge iTXt text chunk ...");
            writer.write_text_chunk(&ob_chunk).map_err(conv_write_err)?;
        }

        // Read the next frame. An APNG might contain multiple frames.
        log::trace!("Writing input-image data ...");
        while let Ok(info) = reader.next_frame(&mut buf) {
            log::trace!("  Writing a frame ...");
            let bytes = buf
                .get(..info.buffer_size())
                .expect("Only possible if the PNG Reader is buggy");
            writer.write_image_data(bytes).map_err(conv_write_err)?;
        }

        Ok(())
    }
}
