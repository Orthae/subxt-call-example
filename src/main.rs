use std::str::FromStr;
use ink_macro::selector_bytes;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;
use subxt_signer::bip39::Mnemonic;

mod aleph;

#[tokio::main]
async fn main() {
   redeem().await;
}

async fn revoke() {
    let client: OnlineClient<PolkadotConfig> = OnlineClient::from_url("wss://ws.test.azero.dev:443").await.unwrap();

    let alice_mnemonic = Mnemonic::from_str("bottom drive obey lake curtain smoke basket hold race lonely fit walk").unwrap();
    let alice = subxt_signer::sr25519::Keypair::from_phrase(&alice_mnemonic, None).unwrap();

    let contract = AccountId32::from_str("5Ha96o1vfRjTkTN9UKSUZ9S9CKK8fSXETspC8TfRe6VeUbD9").unwrap();
    let gas_limit = aleph::azero_13_0::runtime_types::sp_weights::weight_v2::Weight { ref_time: 20_000_000_000, proof_size: 20_000_000_000 };

    let mut call_data: Vec<u8> = Vec::default();
    let mut selector = selector_bytes!("revoke").to_vec();
    call_data.append(&mut selector);

    let call = aleph::azero_13_0::tx()
        .contracts().call(contract.into(), 0, gas_limit, None, call_data);

    client.tx()
        .sign_and_submit_then_watch_default(&call, &alice)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
}

async fn redeem() {
    let client: OnlineClient<PolkadotConfig> = OnlineClient::from_url("wss://ws.test.azero.dev:443").await.unwrap();

    let alice_mnemonic = Mnemonic::from_str("bottom drive obey lake curtain smoke basket hold race lonely fit walk").unwrap();
    let alice = subxt_signer::sr25519::Keypair::from_phrase(&alice_mnemonic, None).unwrap();

    let contract = AccountId32::from_str("5CeHfFNYgVrENTUteabigUxjD9UWHSRvoLE7X4EoJoubNMWh").unwrap();
    let gas_limit = aleph::azero_13_0::runtime_types::sp_weights::weight_v2::Weight { ref_time: 20_000_000_000, proof_size: 20_000_000_000 };

    let mut call_data: Vec<u8> = Vec::default();
    let mut selector = selector_bytes!("redeem").to_vec();

    let id: String = "d1d4bf5b-aac6-4708-a371-1f1a764dbfdf".to_string();
    let signature: [u8; 65] = [162, 142, 178, 139, 126, 14, 217, 33, 133, 206, 152, 210, 84, 239, 17, 158, 51, 151, 128, 160, 149, 131, 61, 161, 250, 149, 220, 162, 184, 97, 213, 46, 60, 239, 12, 111, 218, 234, 3, 124, 101, 134, 214, 192, 243, 177, 126, 136, 20, 242, 115, 238, 135, 70, 41, 156, 204, 143, 224, 51, 231, 14, 155, 72, 0];
    let recipient: AccountId32 = AccountId32::from_str("5Ha96o1vfRjTkTN9UKSUZ9S9CKK8fSXETspC8TfRe6VeUbD9").unwrap();
    let args = (id, signature, recipient);

    call_data.append(&mut selector);
    call_data.append(&mut scale::Encode::encode(&args));

    let call = aleph::azero_13_0::tx()
        .contracts().call(contract.into(), 0, gas_limit, None, call_data);

    client.tx()
        .sign_and_submit_then_watch_default(&call, &alice)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
}
