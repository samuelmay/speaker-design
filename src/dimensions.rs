use yew::{Component,Callback,Context,Event,Html,html,Properties,TargetCast};
use web_sys::{HtmlInputElement};
use crate::cabinet::{CabinetDimensions};


pub enum FormMsg {
    ChangeVolume(u32),
    ChangeRadius(u32),
    ChangeLength(u32),
    ChangeHeight(u32),
    ChangeWidth(u32),
}



#[derive(Clone,PartialEq,Properties)]
pub struct DimensionsFormProps {
    pub starting_dimensions: CabinetDimensions,
    pub on_change: Callback<CabinetDimensions>,
}
pub struct DimensionsForm {
    cabinet: CabinetDimensions,
}

fn parse_to_message<M>(e: Event, constructor: M) -> Option<FormMsg> where M: Fn(u32) -> FormMsg {
    let input: HtmlInputElement = e.target_unchecked_into();
    match input.value().parse::<u32>() {
        Ok(volume) => Some((constructor)(volume)),
        Err(_) => None,
    }
}


impl Component for DimensionsForm {
    type Message = FormMsg;
    type Properties = DimensionsFormProps;

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
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
       match msg {
           FormMsg::ChangeHeight(height) => {
               self.cabinet.port_external_height = height;
           },
           FormMsg::ChangeWidth(width)=> {
               self.cabinet.port_external_width = width;
           },
           FormMsg::ChangeLength(length)=> {
               self.cabinet.port_length = length;
           },
           FormMsg::ChangeRadius(radius)=> {
               self.cabinet.port_flare_radius = radius;
           },
           FormMsg::ChangeVolume(volume)=> {
               self.cabinet.box_volume = volume;
           },
       } 
       
       ctx.props().on_change.emit(self.cabinet);
       true
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <table>
                <tr><td>{ "Box volume" }</td><td>
                    <input
                        type="text"
                        value={self.cabinet.box_volume.to_string()}
                        onchange={link.batch_callback(|e:Event| { parse_to_message(e, FormMsg::ChangeVolume) })} /> {" litres"}
                </td></tr>
                <tr><td>{ "Port length" }</td><td>
                    <input
                        type="text"
                        value={self.cabinet.port_length.to_string()}
                        onchange={link.batch_callback(|e:Event| { parse_to_message(e, FormMsg::ChangeLength) })} /> {" millimetres"}
                </td></tr>
                <tr><td>{ "Port height" }</td><td>
                    <input
                        type="text"
                        value={self.cabinet.port_external_height.to_string()}
                        onchange={link.batch_callback(|e:Event| { parse_to_message(e, FormMsg::ChangeHeight) })} /> {" millimetres"}
                </td></tr>
                <tr><td>{ "Port width" }</td><td>
                    <input
                        type="text"
                        value={self.cabinet.port_external_width.to_string()}
                        onchange={link.batch_callback(|e:Event| { parse_to_message(e, FormMsg::ChangeWidth) })} /> {" millimetres"}
                </td></tr>
                <tr><td>{ "Port flair radius" }</td><td>
                    <input
                        type="text"
                        value={self.cabinet.port_flare_radius.to_string()}
                        onchange={link.batch_callback(|e:Event| { parse_to_message(e, FormMsg::ChangeRadius) })} /> {" millimetres"}
                </td></tr>
            </table>
        }
    }
}