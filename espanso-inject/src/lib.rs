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

use anyhow::Result;

pub mod keys;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(target_os = "linux")]
mod x11;

#[cfg(target_os = "linux")]
mod evdev;

#[cfg(target_os = "macos")]
mod mac;

#[macro_use]
extern crate lazy_static;

pub trait Injector {
  fn send_string(&self, string: &str) -> Result<()>;
  fn send_keys(&self, keys: &[keys::Key], delay: i32) -> Result<()>;
  fn send_key_combination(&self, keys: &[keys::Key], delay: i32) -> Result<()>;
}


#[allow(dead_code)]
pub struct InjectorOptions {
  // Only relevant in Linux systems
  use_evdev: bool,
}

impl Default for InjectorOptions {
  fn default() -> Self {
    Self {
      use_evdev: false,
    }
  }
}

#[cfg(target_os = "windows")]
pub fn get_injector(_options: InjectorOptions) -> impl Injector {
  win32::Win32Injector::new()
}

#[cfg(target_os = "macos")]
pub fn get_injector(_options: InjectorOptions) -> impl Injector {
  mac::MacInjector::new()
}

#[cfg(target_os = "linux")]
pub fn get_injector(options: InjectorOptions) -> impl Injector {
  // TODO: differenciate based on the options
}
