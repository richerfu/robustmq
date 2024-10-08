// Copyright 2023 RobustMQ Team
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use protocol::mqtt::common::QoS;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MQTTCluster {
    pub session_expiry_interval: u32,
    pub topic_alias_max: u16,
    pub max_qos: QoS,
    pub retain_available: AvailableFlag,
    pub wildcard_subscription_available: AvailableFlag,
    pub max_packet_size: u32,
    pub subscription_identifiers_available: AvailableFlag,
    pub shared_subscription_available: AvailableFlag,
    pub max_server_keep_alive: u16,
    pub default_server_keep_alive: u16,
    pub receive_max: u16,
    pub secret_free_login: bool,
    pub max_message_expiry_interval: u64,
    pub client_pkid_persistent: bool,
    pub is_self_protection_status: bool,
    pub tcp_max_connection_num: u64,
    pub tcps_max_connection_num: u64,
    pub websocket_max_connection_num: u64,
    pub websockets_max_connection_num: u64,
    pub send_max_try_mut_times: u64,
    pub send_try_mut_sleep_time_ms: u64,
}

impl MQTTCluster {
    pub fn new() -> Self {
        return MQTTCluster {
            session_expiry_interval: 1800,
            topic_alias_max: 65535,
            max_qos: QoS::ExactlyOnce,
            retain_available: AvailableFlag::Enable,
            max_packet_size: 1024 * 1024 * 10,
            wildcard_subscription_available: AvailableFlag::Enable,
            subscription_identifiers_available: AvailableFlag::Enable,
            shared_subscription_available: AvailableFlag::Enable,
            max_server_keep_alive: 3600,
            default_server_keep_alive: 60,
            receive_max: 65535,
            secret_free_login: false,
            max_message_expiry_interval: 315360000,
            client_pkid_persistent: false,
            is_self_protection_status: false,
            tcp_max_connection_num: 1000,
            tcps_max_connection_num: 1000,
            websocket_max_connection_num: 1000,
            websockets_max_connection_num: 1000,
            send_max_try_mut_times: 128,
            send_try_mut_sleep_time_ms: 100,
        };
    }
    pub fn receive_max(&self) -> u16 {
        return self.receive_max;
    }

    pub fn max_qos(&self) -> QoS {
        return self.max_qos;
    }

    pub fn retain_available(&self) -> u8 {
        return self.retain_available.clone() as u8;
    }

    pub fn max_packet_size(&self) -> u32 {
        return self.max_packet_size;
    }

    pub fn topic_alias_max(&self) -> u16 {
        return self.topic_alias_max;
    }

    pub fn wildcard_subscription_available(&self) -> u8 {
        return self.wildcard_subscription_available.clone() as u8;
    }

    pub fn subscription_identifiers_available(&self) -> u8 {
        return self.subscription_identifiers_available.clone() as u8;
    }

    pub fn shared_subscription_available(&self) -> u8 {
        return self.shared_subscription_available.clone() as u8;
    }

    pub fn is_secret_free_login(&self) -> bool {
        return self.secret_free_login;
    }

    pub fn encode(&self) -> Vec<u8> {
        return serde_json::to_vec(&self).unwrap();
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub enum AvailableFlag {
    #[default]
    Disable,
    Enable,
}

#[cfg(test)]
mod tests {
    use crate::mqtt::cluster::AvailableFlag;

    #[test]
    fn client34_connect_test() {
        assert_eq!(AvailableFlag::Disable as u8, 0);
        assert_eq!(AvailableFlag::Enable as u8, 1);
    }
}
