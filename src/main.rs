use yew::prelude::*;

use crate::cabinet::*;
use crate::views::*;
use crate::dimensions::*;

mod cabinet;
mod views;
mod dimensions;

pub enum DimensionsMsg {
    Change(CabinetDimensions),
}
struct App {
    cabinet: CabinetDimensions,
}

impl Component for App {
    type Message = DimensionsMsg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        let default_box = CabinetDimensions {
            port_length: 120,
            port_external_width: 100,
            port_external_height: 92,
            port_flare_radius: 120,
            box_volume: 161,
        };
        Self {
           cabinet: default_box,
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
       match msg {
           DimensionsMsg::Change(dimensions) => {
               self.cabinet = dimensions; 
               true
           },
       }
    }
    
    
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let model: CabinetModel = CabinetModel::from(self.cabinet);
        let nfr = model.nfr_ratio();
        let minimum_diameter = model.port_min_diameter();
        let frequency = model.resonant_frequency();
        
        let on_change = ctx.link().callback(DimensionsMsg::Change);
        
        html! {
            <>
                <header><h1>{ "Speaker Port Design App" }</h1></header>
                <div id={ "app-container" }>
                    <SideView cabinet={model} />
                    <FrontView cabinet={model} />
                    <DimensionsForm starting_dimensions={self.cabinet} {on_change} />
                    <Calculations {nfr} {minimum_diameter} {frequency} />
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}