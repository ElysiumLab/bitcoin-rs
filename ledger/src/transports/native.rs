use crate::{
    common::{APDUAnswer, APDUCommand},
    errors::LedgerError,
    transports::hid,
};

/// Transport struct for non-wasm arch
pub struct NativeTransport(hid::TransportNativeHID);

impl NativeTransport {
    /// Instantiate a new transport
    pub fn new() -> Result<Self, hid::NativeTransportError> {
        hid::TransportNativeHID::new().map(Self)
    }

    /// Send an APDU command to the device, and receive a response
    pub async fn exchange(&self, command: &APDUCommand) -> Result<APDUAnswer, LedgerError> {
        self.0.exchange(command).map_err(Into::into)
    }
}

/*******************************************************************************
*   (c) 2020 ZondaX GmbH
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
