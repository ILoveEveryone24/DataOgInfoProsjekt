use yew::prelude::*;

enum Msg
{

}

struct Login
{

}

impl Component for Login
{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self
    {
        Self{}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool
    {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        let link = ctx.link();
        html!
        {
            <>
                <div class="container">
                    <h1>{"Log in"}</h1>
                    <form>
                        <input placeholder="Username..."/>
                        <br/>
                        <input type="password" placeholder="Password..."/>
                    </form>
                </div>
            </>
        }
    }
}

fn main()
{
    yew::start_app::<Login>();
}