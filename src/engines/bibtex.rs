// src/engines/bibtex.rs -- Rustic interface to the bibtex processor.
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::CString;

use super::tex::TexResult;
use super::{ExecutionState, IoEventBackend, TectonicBridgeApi};
use crate::errors::{ErrorKind, Result};
use crate::io::IoStack;
use crate::status::StatusBackend;
use crate::unstable_opts::UnstableOptions;

const MIN_CROSSREFS: i32 = 2;

#[derive(Default)]
pub struct BibtexEngine {}

impl BibtexEngine {
    pub fn new() -> BibtexEngine {
        Default::default()
    }

    pub fn process(
        &mut self,
        io: &mut IoStack,
        events: &mut dyn IoEventBackend,
        status: &mut dyn StatusBackend,
        aux: &str,
        unstables: &UnstableOptions,
    ) -> Result<TexResult> {
        let _guard = super::ENGINE_LOCK.lock().unwrap(); // until we're thread-safe ...

        let caux = CString::new(aux)?;

        let /*mut*/ state = ExecutionState::new(io, events, status);
        let bridge = TectonicBridgeApi::new(&state);

        let config = super::BibtexConfig {
            min_crossrefs: unstables.min_crossrefs.unwrap_or(MIN_CROSSREFS),
        };

        unsafe {
            match super::bibtex_simple_main(&*bridge, &config, caux.as_ptr()) {
                0 => Ok(TexResult::Spotless),
                1 => Ok(TexResult::Warnings),
                2 => Ok(TexResult::Errors),
                3 => Err(ErrorKind::Msg("unspecified fatal bibtex error".into()).into()),
                99 => {
                    let msg = super::tt_get_error_message().to_string();
                    Err(ErrorKind::Msg(msg).into())
                }
                x => Err(ErrorKind::Msg(format!(
                    "internal error: unexpected 'history' value {}",
                    x
                ))
                .into()),
            }
        }
    }
}
