/*****************************************************************************
 *   Ledger App Boilerplate Rust.
 *   (c) 2023 Ledger SAS.
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
 *****************************************************************************/
use crate::app_ui::pairing_test::ui_display_pairing_test;
use crate::AppSW;
use ledger_device_sdk::io::Comm;

pub fn handler_pairing_test(
    comm: &mut Comm,
) -> Result<(), AppSW> {
    // Try to get data from comm
    let data: &[u8] = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    ui_display_pairing_test(data.to_vec())?;
    Ok(())
}
