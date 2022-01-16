use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Car {
    id: usize,
    make: String,
    model: String,
    condition: String,
}

#[derive(Clone, Properties, PartialEq)]
struct CarListProps {
    cars: Vec<Car>,
    on_click: Callback<Car>,
}

#[derive(Clone, Properties, PartialEq)]
struct CarDetailsProps {
    car: Car,
}

#[function_component(CarList)]
fn car_list(CarListProps { cars, on_click }: &CarListProps) -> Html {
    let on_click = on_click.clone();

    cars.iter()
        .map(|car| {
            let on_car_select = {
                let on_click = on_click.clone();
                let car = car.clone();

                Callback::from(move |_| {
                    on_click.emit(car.clone());
                })
            };
            html! {
                <div onclick={on_car_select}>{ format!("{} {}", car.make, car.model)}</div>
            }
        })
        .collect()
}

#[function_component(CarDetails)]
fn car_details(CarDetailsProps { car }: &CarDetailsProps) -> Html {
    html! {
        <div>
            <div> {car.make.clone()} </div>
            <div> {car.model.clone()} </div>
            <div> {car.condition.clone()} </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let cars = vec![Car {
        id: 1,
        make: "Ford".to_string(),
        model: "Mustang".to_string(),
        condition: "Almost new. One owner.".to_string(),
    }];

    let selected_car = use_state(|| None);

    let on_car_selection = {
        let selected_car = selected_car.clone();
        Callback::from(move |car: Car| selected_car.set(Some(car)))
    };

    let details = selected_car.as_ref().map(|car| {
        html! {
            <CarDetails car={car.clone()} />
        }
    });

    html! {
        <>
        <h1>{ "Hello World" }</h1>
        <CarList cars={cars} on_click={on_car_selection.clone()}/>
        <div>
        { for details}
        </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
