use crate::constants::constant::NODE_URL;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::*;

async fn load_data(profile_user_account: String, check_account: String) -> bool {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();

    let result: bool = client
        .request(
            "profilevalidation_selectedjuror",
            rpc_params![profile_user_account, check_account],
        )
        .await
        .unwrap();
    result
}

#[component]
pub fn JurorSelected(
    profile_user_account: String,
    check_account: ReadSignal<String>,
) -> impl IntoView {
    let async_data = create_resource(
        move || (profile_user_account.clone(), check_account().clone()),
        |(profile_user_account, check_account)| async move {
            load_data(profile_user_account, check_account).await
        },
    );
    view! {
        <div>
        {move || match async_data.get(){
            None => view! {<p><span class="loading loading-dots loading-xs"></span></p>}.into_view(),
            Some(data) => view! {<p>{data}</p>}.into_view()

        }}
        </div>
    }
}
