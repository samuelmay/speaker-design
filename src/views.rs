use yew::{Component,NodeRef,Context,Html,html,Properties};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{JsCast,JsValue};
use crate::cabinet::{CabinetModel};
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FrontViewProps {
    pub cabinet: CabinetModel
}
pub struct FrontView {
    node_ref: NodeRef
} 

impl Component for FrontView {
    type Message = ();
    type Properties = FrontViewProps;
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
           node_ref: NodeRef::default() 
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cabinet = &ctx.props().cabinet;
        
        let canvas_width = format!("{:0}", cabinet.port_length + cabinet.port_external_width + 30.0);
        let canvas_height = format!("{:0}", cabinet.port_external_height + 20.0);

        html! {
            <canvas ref={self.node_ref.clone()} width={ canvas_width.to_string() } height={canvas_height.to_string()} />
        }
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let cabinet = &ctx.props().cabinet;
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
       
        let port_height_scaled = cabinet.port_external_height / scale_factor;
        let port_width_scaled = cabinet.port_external_width / scale_factor;
        let square_mid = port_height_scaled/2.0 + border;
        let min_radius_scaled = cabinet.port_min_diameter() / (2.0*scale_factor);
        let port_length_scaled: f64 = cabinet.port_length / (2.0*scale_factor); 
        let flair_radius_scaled = cabinet.port_flare_radius / scale_factor;
        let alpha = cabinet.port_flare_arc_start();

        ctx2d.begin_path();
        ctx2d.set_fill_style(&JsValue::from("brown"));
        ctx2d.set_stroke_style(&JsValue::from("brown"));
        ctx2d.arc(
            border + port_length_scaled,
            square_mid - min_radius_scaled - flair_radius_scaled,
            flair_radius_scaled,
            alpha,
            PI - alpha
        ).unwrap();
        ctx2d.close_path();
        ctx2d.fill();

        ctx2d.begin_path();
        ctx2d.arc(
            border + port_length_scaled,
            square_mid + min_radius_scaled + flair_radius_scaled,
            flair_radius_scaled,
            PI + alpha,
            2.0*PI - alpha
        ).unwrap();
        ctx2d.close_path();
        ctx2d.fill();
        
        let far_border = 2.0*border + 2.0*port_length_scaled;
        ctx2d.begin_path();
        ctx2d.move_to(far_border, border);
        ctx2d.line_to(far_border, border + port_height_scaled);
        ctx2d.move_to(far_border + port_width_scaled, border + port_height_scaled);
        ctx2d.line_to(far_border + port_width_scaled, border);
        ctx2d.close_path();
        ctx2d.stroke();

        ctx2d.set_fill_style(&JsValue::from("brown"));
        
        let front_view_box_height = port_height_scaled/2.0-min_radius_scaled;
        ctx2d.fill_rect(far_border, border, port_width_scaled, front_view_box_height);
        ctx2d.fill_rect(far_border, border + port_height_scaled - front_view_box_height, port_width_scaled, front_view_box_height);
       
    }
}

