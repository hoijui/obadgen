// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use xml::reader::EventReader;
use xml::writer::EmitterConfig;

// <svg
//    version="1.1"
//    xmlns="http://www.w3.org/2000/svg"
//    xmlns:xlink="http://www.w3.org/1999/xlink"
//    preserveAspectRatio="xMidYMid meet"
//    viewBox="0 0 640 640"
//    xmlns:openbadges="http://openbadges.org">
//        <openbadges:assertion
//            verify="https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json">
//        </openbadges:assertion>
//
// <!-- rest of SVG content -->
//
// </svg>

pub struct Patcher;

fn conv_read_err(err: xml::reader::Error) -> Error {
    Error::Boxed(Box::new(err))
}

fn conv_write_err(err: xml::writer::Error) -> Error {
    Error::Boxed(Box::new(err))
}

fn add_assertion<W: std::io::Write, S: AsRef<str>>(
    writer: &mut xml::writer::EventWriter<W>,
    verify: S,
) -> Result<(), Error> {
    log::info!(
        "openbadges:assertion - not yet present (as first element after '<svg ...>') -> adding it!"
    );
    // Not yet present (as first element after "<svg ...>")
    // -> create and add it!
    let ob_start = xml::writer::XmlEvent::start_element("openbadges:assertion")
        .attr("verify", verify.as_ref());
    let ob_end = xml::writer::XmlEvent::end_element();
    let ob_assert_elems: [xml::writer::XmlEvent; 2] = [ob_start.into(), ob_end.into()];
    for elem in ob_assert_elems {
        writer.write(elem).map_err(conv_write_err)?;
    }

    Ok(())
}

fn add_namespace<W: std::io::Write>(
    writer: &mut xml::writer::EventWriter<W>,
    evt_in: &xml::reader::XmlEvent,
    evt_out_opt: Option<xml::writer::XmlEvent>,
    passed_init_elem: &mut bool,
) -> Result<(), Error> {
    if let Some(mut evt_out) = evt_out_opt {
        if let xml::writer::XmlEvent::StartElement {
            name,
            attributes: _,
            ref mut namespace,
        } = evt_out
        {
            if name.local_name.to_lowercase() == "svg"
                && (name.namespace.is_none()
                    || *name.namespace.as_ref().unwrap() == "http://www.w3.org/2000/svg")
            {
                *passed_init_elem = true;
                if namespace.contains("openbadges") {
                    log::info!("Namespace 'openbadges' is present!");
                } else {
                    log::info!("Namespace 'openbadges' is *NOT* present!");
                    namespace
                        .to_mut()
                        .put("openbadges", "http://openbadges.org");
                }
            }
        }

        writer.write(evt_out).map_err(conv_write_err)?;
    } else {
        log::trace!("XML reader event needs no writer equivalent: {evt_in:?}");
    }

    Ok(())
}

impl super::Patcher for Patcher {
    fn rewrite<P: AsRef<Path>, S: AsRef<str>>(
        input_file: P,
        output_file: P,
        verify: S,
        fail_if_verify_present: bool,
    ) -> Result<(), Error> {
        let input = File::open(input_file)?;
        let input_buf = BufReader::new(input);

        let output = File::create(output_file)?;
        let mut writer = EmitterConfig::new()
            // .perform_indent(true)
            .create_writer(output);

        let parser = EventReader::new(input_buf);
        let mut passed_init_elem = false;
        for evt_in_res in parser {
            let evt_in = evt_in_res.map_err(conv_read_err)?;
            let mut evt_out_opt = Box::new(evt_in.as_writer_event());

            if passed_init_elem {
                // evt_in now holds the first element after the initial one ("<svg ...>").
                // According to the Open Badge spec.,
                // this would have to be "<openbadges:assertion ...>".
                // If it is, we either want to throw an error for it already existing,
                // or we want to modify it,
                // or we just let it be, if it already contains the content we are supposed to set.
                // If it is *not*, we want to insert it.
                let mut verify_attr_val = None;
                let mut is_ob_assert = false;
                if let xml::reader::XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace: _,
                } = &evt_in
                {
                    if Some("http://openbadges.org") == name.namespace.as_deref()
                        && name.local_name == "assertion"
                    {
                        for attr in attributes {
                            if attr.name.local_name == "verify" {
                                verify_attr_val = Some(attr.value.clone());
                            }
                        }
                        is_ob_assert = true;
                    }
                };
                let mut add_new_ob_assert = false;
                if is_ob_assert {
                    if let Some(val) = verify_attr_val {
                        if val == verify.as_ref() {
                            log::info!("openbadges:assertion - verify is already set to the desired value!");
                        } else if fail_if_verify_present {
                            return Err(Error::VerifyAlreadySet {
                                present: val,
                                proposed: verify.as_ref().to_owned(),
                            });
                        } else {
                            // overwrite it
                            log::info!("openbadges:assertion - verify is already set to an other value -> overwriting!");
                            evt_out_opt = Box::new(Some(
                                xml::writer::XmlEvent::start_element("openbadges:assertion")
                                    .attr("verify", verify.as_ref())
                                    .into(),
                            ));
                        }
                    } else {
                        log::info!("openbadges:assertion - not yet present (as first element after '<svg ...>') (1/2) -> adding it!");
                        // Not yet present (as first element after "<svg ...>")
                        // -> create and add it!
                        add_new_ob_assert = true;
                    }
                } else {
                    log::info!("openbadges:assertion - not yet present (as first element after '<svg ...>') (2/2) -> adding it!");
                    add_new_ob_assert = true;
                }
                if add_new_ob_assert {
                    add_assertion(&mut writer, &verify)?;
                }
                passed_init_elem = false;
            }
            add_namespace(&mut writer, &evt_in, *evt_out_opt, &mut passed_init_elem)?;
        }

        Ok(())
    }
}
