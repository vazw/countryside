use tracing::info;

use nostr_relay::metrics::{counter, describe_counter};
use nostr_relay::{
    message::{ClientMessage, IncomingMessage, OutgoingMessage},
    setting::SettingWrapper,
    Extension, ExtensionMessageResult, List, Session,
};
use serde::Deserialize;

pub struct UserPubkey(String);

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct ZoneSetting {
    pub enabled: bool,
    pub country_code: List,
}

#[derive(Default, Debug)]
pub struct Zone {
    setting: ZoneSetting,
}

impl Zone {
    pub fn new() -> Self {
        describe_counter!(
            "nostr_relay_zone_note_saved",
            "The total count of note saved on allowed zone"
        );
        Self {
            setting: ZoneSetting::default(),
        }
    }
    pub fn match_zone(&self, zone: &str) -> bool {
        self.setting.country_code.contains(&zone.to_lowercase())
    }
}

impl Extension for Zone {
    fn name(&self) -> &'static str {
        "zone"
    }

    fn setting(&mut self, setting: &SettingWrapper) {
        let mut w = setting.write();
        self.setting = w.parse_extension(self.name());
        info!("Zone setting: {:?}", self.setting);
        if self.setting.enabled {
            w.add_information(
                "language_tags".to_string(),
                self.setting
                    .country_code
                    .clone()
                    .iter()
                    .map(|x| x.to_lowercase())
                    .collect::<Vec<String>>()
                    .into(),
            );
        }
    }

    fn message(
        &self,
        msg: ClientMessage,
        session: &mut Session,
        _ctx: &mut <Session as actix::Actor>::Context,
    ) -> ExtensionMessageResult {
        if self.setting.enabled {
            match &msg.msg {
                IncomingMessage::Event(event) => {
                    if let Some(UserPubkey(pk)) = session.get::<UserPubkey>() {
                        if !self.match_zone(session.zone())
                            || !pk.eq(&event.pubkey_str())
                        {
                            return OutgoingMessage::ok(
                                &event.id_str(),
                                false,
                                &format!(
                                    "Not allowed country {}",
                                    session.zone()
                                ),
                            )
                            .into();
                        }
                    } else {
                        return OutgoingMessage::ok(
                            &event.id_str(),
                            false,
                            "auth-required: need reconnect",
                        )
                        .into();
                    }
                    info!(
                        "Recieved from {}: {}",
                        session.ip(),
                        &event.content()
                    );
                    counter!("nostr_relay_zone_note_saved", "command" => "EVENT", "name" => session.zone().to_string()).increment(1);
                }
                IncomingMessage::Auth(event) => {
                    let pk = event.pubkey_str().clone();
                    session.set(UserPubkey(pk));
                }
                _ => {}
            }
        }
        ExtensionMessageResult::Continue(msg)
    }
}
