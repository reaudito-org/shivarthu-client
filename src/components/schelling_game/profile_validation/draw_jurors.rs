use crate::components::schelling_game::profile_validation::change_period::ChangePeriod;
use crate::components::schelling_game::profile_validation::draw_jurors_sign_in::SignTransaction;
use crate::components::schelling_game::profile_validation::rpc::drawing_period_end::DrawingEndBlock;
use crate::components::schelling_game::profile_validation::storage::get_period::GetPeriod;
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::*;


#[component]
pub fn DrawJurors(profile_user_account: String) -> impl IntoView {
    // gloo::console::log!(profile_user_account());
    let (current_view, set_current_view) = create_signal(View::Form);
    let (iterations, set_iterations) = create_signal::<Result<u64, ErrorString>>(Ok(0));
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        set_current_view(View::Success);
    };

    let iteration_function = move |value: String| {
        let iteration_value = value.parse::<u64>().expect("Invalid input");
        gloo::console::log!(iteration_value);

        set_iterations(Ok(iteration_value));
    };

    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div class="max-w-5xl mx-auto max-md:mx-10">
                    <GetPeriod profile_user_account=profile_user_account.clone()/>
                    <DrawingEndBlock profile_user_account=profile_user_account.clone()/>
                    <ChangePeriod profile_user_account=profile_user_account.clone()/>

                    <form id="draw-juror-submit-from" on:submit=submit_click>
                        <div class="mb-5">
                            <label
                                for="draw-jurors"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Number of Draw Jurors
                            </label>
                            <input
                                type="number"
                                id="iterations"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |e| iteration_function(event_target_value(&e))
                            />
                        </div>
                        <button
                            type="submit"
                            id="draw-jurors-submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Draw Jurors
                        </button>
                    </form>
                </div>
            }
        }
        View::Success => {
            view! {
                <div>
                    <SignTransaction
                        iterations=iterations().unwrap()
                        profile_user_account=profile_user_account.clone()
                    />

                </div>
            }
        }
    };

    view! { <div>{move || render_view()}</div> }
}
