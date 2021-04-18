/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;

use espanso_config::matches::{Match, MatchEffect};

use crate::engine::{event::{Event, matches::DetectedMatch, render::RenderingRequestedEvent}, process::Multiplexer};

pub trait MatchProvider<'a> {
  fn get(&self, match_id: i32) -> Option<&'a Match>;
}

pub struct MultiplexAdapter<'a> {
  provider: &'a dyn MatchProvider<'a>,
}

impl<'a> MultiplexAdapter<'a> {
  pub fn new(provider: &'a dyn MatchProvider<'a>) -> Self {
    Self { provider }
  }
}

impl<'a> Multiplexer for MultiplexAdapter<'a> {
  fn convert(&self, detected_match: DetectedMatch) -> Option<Event> {
    let m = self.provider.get(detected_match.id)?;

    match &m.effect {
      MatchEffect::Text(_) => Some(Event::RenderingRequested(RenderingRequestedEvent {
        match_id: detected_match.id,
        trigger: detected_match.trigger,
        left_separator: detected_match.left_separator,
        right_separator: detected_match.right_separator,
        trigger_args: detected_match.args,
      })),
      // TODO: think about rich text and image
      MatchEffect::None => None,
    }
  }
}