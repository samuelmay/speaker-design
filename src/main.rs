use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};
use wasm_bindgen::{JsValue};
use std::f64::consts::PI;

enum Msg {
    ChangeVolume(i32),
    ChangeRadius(i32),
    ChangeLength(i32),
    ChangeHeight(i32),
    ChangeWidth(i32),
}

struct App {
    cabinet: Cabinet,
    node_ref: NodeRef
}

struct Cabinet {
    port_length: i32,
    port_flare_radius: i32,
    port_external_width: i32,
    port_external_height: i32,
    box_volume: i32,
}

impl Cabinet {
    fn port_flare_arc_start(&self) -> f64 {
        let l: f64 = f64::from(self.port_length) / 2.0; 
        let R = f64::from(self.port_flare_radius);
        return (l/R).acos();
    }

    fn port_min_diameter(&self) -> f64 {
        let l: f64 = f64::from(self.port_length) / 2.0; 
        let R = f64::from(self.port_flare_radius);
        let d = (R.powi(2) - l.powi(2)).sqrt();
        let port_max_radius = f64::from(self.port_external_height) / 2.0;
        let port_min_radius = port_max_radius - (R - d); 
        return port_min_radius * 2.0;
    }
    
    fn resonant_frequency(&self) -> f64 {
        let L_actual = f64::from(self.port_length)/1000.0;
        let r_fit = f64::from(self.port_flare_radius)/1000.0;
        let D_min = self.port_min_diameter() / 1000.0;
        
        // special calculation for Jack based on square cross section
        //let A_min = PI*(D_min/2.0).powi(2);
        let A_min = D_min * f64::from(self.port_external_width);
        let L_effective = L_actual + D_min;

        let A_effective = A_min * (1.0 + 0.576*(L_actual/(2.0*r_fit)));
        
        // calculate volume. Convert liters to m3
        let V_box = f64::from(self.box_volume)/1000.0;

        let speed_of_sound_constant = 343.0 / (2.0 * PI); 
        
        let frequency = speed_of_sound_constant * (A_effective / (L_effective * V_box)).sqrt();
        return frequency;
    }

}

fn parse_to_message<M>(e: Event, constructor: M) -> Option<Msg> where M: Fn(i32) -> Msg {
    let input: HtmlInputElement = e.target_unchecked_into();
    match input.value().parse::<i32>() {
        Ok(volume) => Some((constructor)(volume)),
        Err(_) => None,
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        let default_box = Cabinet {
            port_length: 250,
            port_external_width: 250,
            port_external_height: 150,
            port_flare_radius: 200,
            box_volume: 161,
        };
        Self {
           cabinet: default_box,
           node_ref: NodeRef::default() 
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
        
        let canvas_width = self.cabinet.port_length + self.cabinet.port_external_width + 30;
        let canvas_height = self.cabinet.port_external_height + 20;
        
        html! {
            <div>
                <canvas ref={self.node_ref.clone()} width={canvas_width.to_string()} height={canvas_height.to_string()} />
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
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeLength) })} /> {" milimetres"}
                    </td></tr>
                    <tr><td>{ "Port height" }</td><td>
                        <input
                            type="text"
                            value={self.cabinet.port_external_height.to_string()}
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeHeight) })} /> {" milimetres"}
                    </td></tr>
                    <tr><td>{ "Port width" }</td><td>
                        <input
                            type="text"
                            value={self.cabinet.port_external_width.to_string()}
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeWidth) })} /> {" milimetres"}
                    </td></tr>
                    <tr><td>{ "Port flair radius" }</td><td>
                        <input
                            type="text"
                            value={self.cabinet.port_flare_radius.to_string()}
                            onchange={link.batch_callback(|e:Event| { parse_to_message(e, Msg::ChangeRadius) })} /> {" milimetres"}
                    </td></tr>

                    <tr><td>{ "Port minimum diameter" }</td><td>{self.cabinet.port_min_diameter()} { " milimeters" }</td></tr>
                    <tr><td>{ "Frequency" }</td><td>{self.cabinet.resonant_frequency()} { " hertz" }</td></tr>
                </table>
            </div>
        }
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap(); 
        let ctx2d : CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        
        let full_width = f64::from(canvas.width());
        let full_height = f64::from(canvas.height());
        ctx2d.clear_rect(0.0,0.0,full_width, full_height);
        let border: f64 = 10.0;
        
        let scale_factor: f64 = 1.0;
       
        let port_height_scaled = f64::from(self.cabinet.port_external_height) / scale_factor;
        let square_mid = port_height_scaled/2.0 + border;
        let min_radius_scaled = self.cabinet.port_min_diameter() / (2.0*scale_factor);
        let port_length_scaled: f64 = f64::from(self.cabinet.port_length) / (2.0*scale_factor); 
        let flair_radius_scaled = f64::from(self.cabinet.port_flare_radius) / scale_factor;
        let alpha = self.cabinet.port_flare_arc_start();

        ctx2d.begin_path();
        ctx2d.set_stroke_style(&JsValue::from("brown"));
        ctx2d.arc(
            border + port_length_scaled,
            border + square_mid - min_radius_scaled - flair_radius_scaled,
            flair_radius_scaled,
            alpha,
            PI - alpha
        ).unwrap();
        ctx2d.stroke();

        ctx2d.begin_path();
        ctx2d.arc(
            border + port_length_scaled,
            border + square_mid + min_radius_scaled + flair_radius_scaled,
            flair_radius_scaled,
            PI + alpha,
            2.0*PI - alpha
        ).unwrap();
        ctx2d.set_stroke_style(&JsValue::from("brown"));
        ctx2d.stroke();
       
    }
}

fn main() {
    yew::start_app::<App>();
}
