use yew::prelude::*;

struct Model{
    value:i64 
}


#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model{
        value:0        
    });

    // function that can be called from out HTML

    let onclick = {
        let state = state.clone();


        // functions that are needed to be called from our HTML should be encapsulated in our callback mechanism


        // closure with our business logic
        // callback expects a closure that takes one parameter we dont care
        // closures in rust borrow references to any variables  that they refernce outside the closure
        // move to take ownership

        Callback::from(move |_|{
            state.set(Model{
                value : state.value + 1
            })
        })
    };




    html!  {
        <div class="mt-5 card card-body">
        
        <button class="btn btn-success w-25 m-auto" {onclick} >{"+1"}</button>
        <p class="h3 text-secondary mt-5">{ state.value }</p>
        </div>
    }
}


fn main(){
    yew::start_app::<App>();
}