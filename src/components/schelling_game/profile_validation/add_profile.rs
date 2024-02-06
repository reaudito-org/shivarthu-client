use crate::components::api::ipfs_request::ipfs_call_json_string;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;
use crate::components::markdown::markdown_field::MarkdownField;
use crate::components::navigation::nav::Nav;
use crate::components::upload::upload_video::FileUpload;
use crate::services::error::ErrorString;
use json::object;
use leptos::ev::SubmitEvent;
use leptos::*;

async fn get_cid_post(
    name: String,
    details: String,
    profile_video_cid: String,
) -> Result<String, ErrorString> {
    let data = object! {
          version: "1.0",
          name: name,
          details: details,
          profile_video_cid: profile_video_cid,
    };
    let json_string = json::stringify(data);
    let response =
        ipfs_call_json_string(DEFAULT_IPFS_PROVIDER, &json_string, "ipfs".to_owned()).await;
    Ok(response)
}

#[derive(Copy, Clone)]
struct CidContext(ReadSignal<String>);

#[component]
pub fn AddProfile() -> impl IntoView {
    let (name, set_name) = create_signal(String::from(""));
    let (markdown, set_markdown) = create_signal(String::from(""));
    let (video_cid, set_video_cid) = create_signal(String::from(""));
    let (post_cid, set_post_cid) = create_signal(String::from(""));

    provide_context(CidContext(post_cid));

    let submit_action = create_action(
        |(name, details, profile_video_cid): &(String, String, String)| {
            let name = name.to_owned();
            let details = details.to_owned();
            let profile_video_cid = profile_video_cid.to_owned();

            async move { get_cid_post(name, details, profile_video_cid).await }
        },
    );
    let pending = submit_action.pending();
    let submit_action_value = submit_action.value();

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        submit_action.dispatch((name(), markdown(), video_cid()));
    };

    let cid_value = move || {
        let cid_result = submit_action_value();
        let cid = match cid_result {
            Some(result_cid) => {
                let cid_value_in_result = match result_cid {
                    Ok(value) => {
                        gloo::console::log!(format!("Got string: {}", value.clone()));
                        value
                    }
                    Err(error) => {
                        gloo::console::log!(format!("Error: {:?}", error));
                        String::from("")
                    }
                };
                cid_value_in_result
            }
            None => String::from(""),
        };
        set_post_cid(cid.clone());
        cid
    };

    view! {
        <>
            <Nav/>
            <div>
                <form
                    class="max-w-5xl mx-auto max-md:mx-10"
                    id="add-profile-submit-from"
                    on:submit=submit_click
                >

                    <div class="mb-5">
                        <label
                            for="profile-name"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            Your Name
                        </label>
                        <input
                            type="text"
                            id="profile-name"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            required
                            prop:value=move || name()
                            on:input=move |e| set_name(event_target_value(&e))
                        />
                    </div>
                    <div class="mb-5">
                        <label
                            for="profile-details"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            Profile Details
                        </label>
                        <MarkdownField
                            set_markdown=set_markdown
                            name=String::from("profile-details")
                            class=String::from(
                                "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                            )
                        />

                        <div class="mb-5">
                            <label
                                for="profile-video"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Profile Video
                            </label>
                            <FileUpload
                                set_cid_props=set_video_cid
                                accept_file_type=String::from("video/mp4")
                            />
                        </div>

                        <button
                            type="submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Submit
                        </button>

                        <p>{move || pending().then(|| "Loading...")}</p>
                        <p>"Pending: " <code>{move || format!("{:#?}", pending())}</code></p>
                        <p>
                            "Value: " <code>{move || cid_value()}</code>
                        </p>

                    </div>
                </form>
            </div>
        </>
    }
}
