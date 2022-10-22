use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{JsValue};
use std::f64::consts::PI;

enum Msg {
    AddOne,
}

struct App {
    cabinet: Cabinet,
    node_ref: NodeRef
}

struct Cabinet {
    port_length: i32,
    port_flare_radius: i32,
    port_external_diameter: i32,
    box_length: i32,
    box_height: i32,
    box_width: i32,
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
        let port_max_radius = f64::from(self.port_external_diameter) / 2.0;
        let port_min_radius = port_max_radius - (R - d); 
        return port_min_radius * 2.0;
    }
    
    fn resonant_frequency(&self) -> f64 {
        let L_actual = f64::from(self.port_length)/1000.0;
        let r_fit = f64::from(self.port_flare_radius)/1000.0;
        let D_min = self.port_min_diameter() / 1000.0;
        let A_min = PI*(D_min/2.0).powi(2);
        let L_effective = L_actual + D_min;

        let A_effective = A_min * (1.0 + 0.576*(L_actual/(2.0*r_fit)));
        
        // calculate volume
        let b_l = f64::from(self.box_length)/1000.0;
        let b_w = f64::from(self.box_width)/1000.0;
        let b_h = f64::from(self.box_height)/1000.0;
        let V_box = b_l * b_w * b_h;

        let speed_of_sound_constant = 343.0 / (2.0 * PI); 
        
        let frequency = speed_of_sound_constant * (A_effective / (L_effective * V_box)).sqrt();
        return frequency;
    }

}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        let default_box = Cabinet {
            port_length: 250,
            port_external_diameter: 150,
            port_flare_radius: 200,
            box_height: 700,
            box_length: 400,
            box_width: 400
        };
        Self {
           cabinet: default_box,
           node_ref: NodeRef::default() 
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
       match msg {
           Msg::AddOne => {
               self.cabinet.box_length += 10;
               true
           }
       } 
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <canvas ref={self.node_ref.clone()} width="500" height="400" />
                <table>
                    <tr>{ "Box width" }<td></td><td>{self.cabinet.box_width}</td></tr>
                    <tr>{ "Box height" }<td></td><td>{self.cabinet.box_height}</td></tr>
                    <tr>{ "Box length" }<td></td><td>{self.cabinet.box_length}</td></tr>
                    <tr>{ "Port length" }<td></td><td>{self.cabinet.port_length}</td></tr>
                    <tr>{ "Port external diameter" }<td></td><td>{self.cabinet.port_external_diameter}</td></tr>
                    <tr>{ "Port minimum diameter" }<td></td><td>{self.cabinet.port_min_diameter()}</td></tr>
                    <tr>{ "Frequency" }<td></td><td>{self.cabinet.resonant_frequency()}</td></tr>
                </table>
                <p><button onclick={link.callback(|_| Msg::AddOne)}>{ "+10" }</button></p>
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
        
        let scale_factor: f64 = 2.0;
        let square_width = f64::from(self.cabinet.box_length)/scale_factor;
        let square_height = f64::from(self.cabinet.box_height)/scale_factor;

        ctx2d.begin_path();
        ctx2d.set_stroke_style(&JsValue::from("grey"));
        ctx2d.move_to(border, border);
        ctx2d.line_to(border + square_width, border);
        ctx2d.line_to(border + square_width, border + square_height);
        ctx2d.line_to(border, border + square_height);
        ctx2d.close_path();
        ctx2d.stroke();
        
        let square_mid = square_height / 2.0;
        let min_radius_scaled = self.cabinet.port_min_diameter() / (2.0*scale_factor);
        let port_length_scaled: f64 = f64::from(self.cabinet.port_length) / (2.0*scale_factor); 
        let flair_radius_scaled = f64::from(self.cabinet.port_flare_radius) / scale_factor;
        let alpha = self.cabinet.port_flare_arc_start();

        ctx2d.begin_path();
        ctx2d.set_stroke_style(&JsValue::from("blue"));
        ctx2d.arc(
            border + square_width - port_length_scaled,
            border + square_mid - min_radius_scaled - flair_radius_scaled,
            flair_radius_scaled,
            alpha,
            PI - alpha
        ).unwrap();

        ctx2d.arc(
            border + square_width - port_length_scaled,
            border + square_mid + min_radius_scaled + flair_radius_scaled,
            flair_radius_scaled,
            PI + alpha,
            2.0*PI - alpha
        ).unwrap();
        ctx2d.stroke();
       
    }
}

fn main() {
    yew::start_app::<App>();
}
