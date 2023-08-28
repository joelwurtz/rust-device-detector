use anyhow::Result;

#[cfg(test)]
mod test;

use serde::Serialize;

use crate::client_hints::ClientHint;
use crate::parsers::client::ClientType;
use crate::parsers::device::DeviceType;
use crate::parsers::{bot, client, device, oss};

// TODO we should Box KnownDevice as it is much larger than Bot
#[allow(clippy::large_enum_variant)]
pub enum Detection {
    Known(KnownDevice),
    Bot(bot::Bot),
}

#[derive(Debug, Serialize)]
pub struct KnownDevice {
    pub client: Option<client::Client>,
    pub device: device::Device,
    pub os: Option<oss::OS>,
}

impl Detection {
    pub fn to_value(self) -> serde_json::Value {
        match self {
            Detection::Known(known) => {
                let is = serde_json::json!({
                    "desktop": known.is_desktop(),
                    "mobile": known.is_mobile(),
                    "touch_enabled": known.is_touch_enabled(),
                    "smart_phone": known.is_smart_phone(),
                    "feature_phone": known.is_feature_phone(),
                    "browser": known.is_browser(),
                    "camera": known.is_camera(),
                    // TODO rename from car
                    "car_browser": known.is_car_browser(),
                    "feed_reader": known.is_feed_reader(),
                    "console": known.is_console(),
                    "library": known.is_library(),
                    "media_player": known.is_media_player(),
                    // TODO rename from portable_mp3
                    "portable_media_player": known.is_portable_media_player(),
                    "mobile_app": known.is_mobile_app(),
                    "television": known.is_television(),
                    "smart_display": known.is_smart_display(),
                    "tablet": known.is_tablet(),
                    "smart_speaker": known.is_smart_speaker(),
                    "pim": known.is_pim(),
                    "peripheral": known.is_peripheral(),
                    "robot": false,

                });

                let mut val = serde_json::to_value(known).unwrap();
                val["is"] = is;
                val
            }
            Detection::Bot(bot) => serde_json::to_value(bot).unwrap(),
        }
    }
}

impl KnownDevice {
    pub fn is_mobile(&self) -> bool {
        if self.device.mobile_client_hint {
            return true;
        }

        if let Some(device_type) = &self.device.device_type {
            if [
                DeviceType::FeaturePhone,
                DeviceType::SmartPhone,
                DeviceType::Tablet,
                DeviceType::Phablet,
                DeviceType::Camera,
                DeviceType::PortableMediaPlayer,
            ]
            .contains(device_type)
            {
                return true;
            }

            if [
                DeviceType::Television,
                DeviceType::SmartDisplay,
                DeviceType::Console,
            ]
            .contains(device_type)
            {
                return false;
            }
        }

        if let Some(client) = &self.client {
            if device::uses_mobile_browser(client) {
                return true;
            }
        }

        !self.is_desktop()
    }

    pub fn is_touch_enabled(&self) -> bool {
        self.device.touch_enabled
    }

    pub fn is_bot(&self) -> bool {
        false
    }

    pub fn is_pim(&self) -> bool {
        self.client
            .as_ref()
            .map(|x| x.r#type == ClientType::Pim)
            .unwrap_or(false)
    }
    pub fn is_feed_reader(&self) -> bool {
        self.client
            .as_ref()
            .map(|x| x.r#type == ClientType::FeedReader)
            .unwrap_or(false)
    }

    pub fn is_mobile_app(&self) -> bool {
        self.client
            .as_ref()
            .map(|x| x.r#type == ClientType::MobileApp)
            .unwrap_or(false)
    }

    pub fn is_media_player(&self) -> bool {
        self.client
            .as_ref()
            .map(|x| x.r#type == ClientType::MediaPlayer)
            .unwrap_or(false)
    }

    pub fn is_browser(&self) -> bool {
        self.client
            .as_ref()
            .map(|x| x.r#type == ClientType::Browser)
            .unwrap_or(false)
    }

    pub fn is_library(&self) -> bool {
        self.client
            .as_ref()
            .map(|x| x.r#type == ClientType::Library)
            .unwrap_or(false)
    }

    pub fn is_desktop(&self) -> bool {
        // the php library duplicates logic but as far as I can
        // tell it should be equivalent to this.
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::Desktop)
            .unwrap_or(false)
    }

    pub fn is_console(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::Console)
            .unwrap_or(false)
    }

    pub fn is_car_browser(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::CarBrowser)
            .unwrap_or(false)
    }
    pub fn is_camera(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::Camera)
            .unwrap_or(false)
    }
    pub fn is_portable_media_player(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::PortableMediaPlayer)
            .unwrap_or(false)
    }
    pub fn is_notebook(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::Notebook)
            .unwrap_or(false)
    }

    pub fn is_television(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::Television)
            .unwrap_or(false)
    }

    pub fn is_smart_display(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::SmartDisplay)
            .unwrap_or(false)
    }

    pub fn is_feature_phone(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::FeaturePhone)
            .unwrap_or(false)
    }

    pub fn is_smart_phone(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::SmartPhone)
            .unwrap_or(false)
    }

    pub fn is_tablet(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::Tablet)
            .unwrap_or(false)
    }

    pub fn is_smart_speaker(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::SmartSpeaker)
            .unwrap_or(false)
    }

    pub fn is_peripheral(&self) -> bool {
        self.device
            .device_type
            .as_ref()
            .map(|x| *x == DeviceType::Peripheral)
            .unwrap_or(false)
    }
}

// use std::alloc::System;
// use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

pub fn parse(ua: &str, headers: Option<Vec<(String, String)>>) -> Result<Detection> {
    // println!("parsing {}", ua);
    // let reg = stats_alloc::Region::new(&INSTRUMENTED_SYSTEM);

    if let Some(bot) = bot::lookup_bot(ua)? {
        return Ok(Detection::Bot(bot));
    }

    let client_hints = match headers {
        Some(headers) => Some(ClientHint::from_headers(headers)?),
        None => None,
    };

    let os = oss::lookup(ua, client_hints.as_ref())?;

    let client = client::lookup(ua, client_hints.as_ref())?;

    let device = device::lookup(ua, client.as_ref(), client_hints.as_ref(), os.as_ref())?;

    let known = Detection::Known(KnownDevice { client, device, os });

    // let ch = reg.change();
    // println!("allocations over parse: {:#?} remaining {}", ch, ch.bytes_allocated - ch.bytes_deallocated);
    // println!("allocations over parse {} {}", ch.bytes_allocated - ch.bytes_deallocated, size);

    Ok(known)
}
