use yew::prelude::*;
use web_sys::{HtmlInputElement};

use crate::cabinet::*;
use crate::views::*;

mod cabinet;
mod views;

enum Msg {
    ChangeVolume(u32),
    ChangeRadius(u32),
    ChangeLength(u32),
    ChangeHeight(u32),
    ChangeWidth(u32),
}

struct App {
    cabinet: CabinetDimensions,
}

fn parse_to_message<M>(e: Event, constructor: M) -> Option<Msg> where M: Fn(u32) -> Msg {
    let input: HtmlInputElement = e.target_unchecked_into();
    match input.value().parse::<u32>() {
        Ok(volume) => Some((constructor)(volume)),
        Err(_) => None,
    }
}

impl Component for App {
    type Message = Msg;
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
           Msg::ChangeHeight(height) => {
               self.cabinet.port_external_height = height;
               true
           },
           Msg::ChangeWidth(width)=> {
               self.cabinet.port_external_width = width;
               true
           },
           Msg::ChangeLength(length)=> {
               self.cabinet.port_length = length;
               true
           },
           Msg::ChangeRadius(radius)=> {
               self.cabinet.port_flare_radius = radius;
               true
           },
           Msg::ChangeVolume(volume)=> {
               self.cabinet.box_volume = volume;
               true
           },
       } 
    }
    
    
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        
        let cabinet: CabinetModel = CabinetModel::from(self.cabinet);
        let nfr = cabinet.nfr_ratio();
        let port_md = cabinet.port_min_diameter();
        let freq = cabinet.resonant_frequency();
        
        html! {
            <div>
                <FrontView {cabinet} />
                <table>
                    <tr><td>{ "Box volume" }</td><td>
                        <input
                            type="text"
                            value={self.cabinet.box_volume.to_string()}
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeVolume) })} /> {" litres"}
                    </td></tr>
                    <tr><td>{ "Port length" }</td><td>
                        <input
                            type="text"
                            value={self.cabinet.port_length.to_string()}
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeLength) })} /> {" millimetres"}
                    </td></tr>
                    <tr><td>{ "Port height" }</td><td>
                        <input
                            type="text"
                            value={self.cabinet.port_external_height.to_string()}
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeHeight) })} /> {" millimetres"}
                    </td></tr>
                    <tr><td>{ "Port width" }</td><td>
                        <input
                            type="text"
                            value={self.cabinet.port_external_width.to_string()}
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeWidth) })} /> {" millimetres"}
                    </td></tr>
                    <tr><td>{ "Port flair radius" }</td><td>
                        <input
                            type="text"
                            value={self.cabinet.port_flare_radius.to_string()}
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeRadius) })} /> {" millimetres"}
                    </td></tr>

                    <tr><td>{ "Normalized flair ratio" }</td><td>{nfr} { " (recommended to be 0.5)" }</td></tr>
                    <tr><td>{ "Port minimum diameter" }</td><td>{port_md} { " millimeters" }</td></tr>
                    <tr><td>{ "Frequency" }</td><td>{freq} { " Hertz" }</td></tr>
                </table>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}