use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::profile_validation::rpc::juror_selected::JurorSelected;
use leptos::*;
use leptos_router::*;

#[component]
pub fn JurorSelectedCheck() -> impl IntoView {
    let params = use_params_map();

    let profile_user_account = move || {
        params.with(|params| {
            params
                .get("profile_user_account")
                .cloned()
                .unwrap_or_default()
        })
    };

    let (check_account, set_check_account) = create_signal(String::from(""));

    let account = untrack(move || profile_user_account());

    let on_account = move |ev| {
        let account_value = event_target_value(&ev);
        set_check_account(account_value);
    };

    view! {
        <div>
            <Nav/>
            <div class="container mx-auto px-10">
                <h1>Check if an account selected as juror:</h1>
                <br/>
                <input
                    type="text"
                    placeholder="Enter account address here"
                    id="juror-address-checking"
                    class="input input-bordered w-full max-w-xs"
                    on:input={on_account}
                />
                <br/>
                <br/>
                <JurorSelected profile_user_account={account} check_account={check_account}/>
            </div>
        </div>
    }
}
