use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::*;
use polkadot::runtime_types::pallet_project_tips::types::TippingName;
use polkadot::runtime_types::pallet_support::Content;

fn match_tipping_name(name: &str) -> Option<TippingName> {
    match name {
        "SmallTipper" => Some(TippingName::SmallTipper),
        "BigTipper" => Some(TippingName::BigTipper),
        "SmallSpender" => Some(TippingName::SmallSpender),
        "MediumSpender" => Some(TippingName::MediumSpender),
        "BigSpender" => Some(TippingName::BigSpender),
        _ => None,
    }
}
#[component]
pub fn SignTransaction(
    post_cid: String,
    department_id: u64,
    tip_name: String,
    funding_needed: u128,
) -> impl IntoView {
    view! {
        <ExtensionSignIn
            post_cid={post_cid}
            department_id={department_id}
            tip_name={tip_name}
            funding_needed={funding_needed}
        />
    }
}

#[component]
pub fn ExtensionSignIn(
    post_cid: String,
    department_id: u64,
    tip_name: String,
    funding_needed: u128,
) -> impl IntoView {
    let (account_load, set_account_load) = create_signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load={set_account_load}/>
                </div>
            }
        } else if !account_load().0.is_empty() && !account_load().1.is_empty() {
            view! {
                <div>
                    <ExtensionTransaction
                        post_cid={post_cid.clone()}
                        department_id={department_id.clone()}
                        tip_name={tip_name.clone()}
                        funding_needed={funding_needed.clone()}
                        account_address={account_load().0}
                        account_source={account_load().1}
                    />
                </div>
            }
        } else {
            view! { <div>{"Some Error Occured"}</div> }
        }
    };
    view! { <div>{move || render_html()}</div> }
}

#[component]
pub fn ExtensionTransaction(
    post_cid: String,
    department_id: u64,
    tip_name: String,
    funding_needed: u128,
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = create_signal(String::from(""));
    let (extrinsic_success, set_extrinsic_success) = create_signal(String::from(""));
    let transaction_resource = create_local_resource(
        move || {
            (
                post_cid.clone(),
                department_id.clone(),
                tip_name.clone(),
                funding_needed.clone(),
                account_address.clone(),
                account_source.clone(),
                set_error,
                set_extrinsic_success,
            )
        },
        move |(
            post_cid,
            department_id,
            tip_name,
            funding_needed,
            account_address,
            account_source,
            set_error,
            set_extrinsic_success,
        )| async move {
            let content: Content = Content::IPFS(post_cid.as_bytes().to_vec());

            let tipping_name = match_tipping_name(&tip_name);

            let tx = polkadot::tx().project_tips().create_project(
                department_id,
                content,
                tipping_name.unwrap(),
                funding_needed,
            );
            sign_in_with_extension(
                tx,
                account_address,
                account_source,
                set_error,
                set_extrinsic_success,
            )
            .await;
        },
    );

    let loading = transaction_resource.loading();
    let is_loading = move || {
        if loading() {
            view! {
                <div class="alert">
                    <span class="loading loading-spinner"></span>
                    "Loading... Please sign with extension."
                </div>
            }
        } else {
            view! { <div class="alert">"Idle."</div> }
        }
    };

    let error_fn = move || {
        if !error().is_empty() {
            view! {
                <div role="alert" class="alert alert-error">
                    {move || error()}
                </div>
            }
        } else {
            view! { <div></div> }
        }
    };

    let extrinsic_success_fn = move || {
        if !extrinsic_success().is_empty() {
            view! {
                <div role="alert" class="alert alert-success">
                    {move || extrinsic_success()}
                </div>
            }
        } else {
            view! { <div></div> }
        }
    };

    view! {
        <div class="md:container md:mx-auto">
            <div>{move || transaction_resource.get()}</div>
            <br/>
            <div>{move || is_loading()}</div>
            <br/>
            <div>{move || error_fn()}</div>
            <br/>
            <div>{move || extrinsic_success_fn()}</div>

        </div>
    }
}
