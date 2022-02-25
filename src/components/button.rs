#[component]
pub fn Child<'a, 'b, G: Html>(ctx: ScopeRef<'a>, props: (&'a Signal<i32>, &'a dyn Fn(Event))) -> View<G> {
    let (item, callback) = props;
    view! { ctx,
        p {(item.get())}
        input(type="button", on:click=callback)
    }
}

#[component]
pub fn Connector<'a, G: Html>(
    ctx: ScopeRef<'a>, 
    props: (&'a Signal<i32>, &'a dyn Fn(ScopeRef<'a>, (&'a Signal<i32>, &'a dyn Fn(Event))) -> View<G>)
) -> View<G> {
    let (item, Comp) = props;
    let add_one = |_| item.set(*item.get() + 1);

    view! {ctx,
        Comp((item, &add_one))
    }
}