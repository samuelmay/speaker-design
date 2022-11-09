use yew::{Component,function_component,functional::{use_node_ref,use_effect},NodeRef,Context,Html,html,Properties};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{JsCast,JsValue};
use crate::cabinet::{CabinetModel};
use std::f64::consts::PI;

#[derive(Clone, PartialEq, Properties)]
pub struct SchematicViewProps {
    pub cabinet: CabinetModel
}
pub struct FrontView {
    node_ref: NodeRef
} 

impl Component for FrontView {
    type Message = ();
    type Properties = SchematicViewProps;
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
           node_ref: NodeRef::default() 
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cabinet = &ctx.props().cabinet;
        
        let canvas_width = format!("{:0}", cabinet.port_external_width + 20.0);
        let canvas_height = format!("{:0}", cabinet.port_external_height + 20.0);

        html! {
            <div class={ "speaker-view" }>
                <h3>{"Front View"}</h3>
                <canvas ref={self.node_ref.clone()} width={ canvas_width.to_string() } height={canvas_height.to_string()} />
            </div>
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
        let min_radius_scaled = cabinet.port_min_diameter() / (2.0*scale_factor);

        ctx2d.begin_path();
        ctx2d.set_stroke_style(&JsValue::from("brown"));
        ctx2d.move_to(border, border);
        ctx2d.line_to(border, border + port_height_scaled);
        ctx2d.move_to(border + port_width_scaled, border + port_height_scaled);
        ctx2d.line_to(border + port_width_scaled, border);
        ctx2d.close_path();
        ctx2d.stroke();

        ctx2d.set_fill_style(&JsValue::from("brown"));
        
        let front_view_box_height = port_height_scaled/2.0-min_radius_scaled;
        ctx2d.fill_rect(border, border, port_width_scaled, front_view_box_height);
        ctx2d.fill_rect(border, border + port_height_scaled - front_view_box_height, port_width_scaled, front_view_box_height);
       
    }
}

#[function_component(SideView)]
pub fn side_view(props: &SchematicViewProps) -> Html {
    let canvas_width = format!("{:0}", props.cabinet.port_length + 20.0);
    let canvas_height = format!("{:0}", props.cabinet.port_external_height + 20.0);

    let canvas_ref = use_node_ref();
    
    {
        let cabinet = props.cabinet.clone();
        let canvas = canvas_ref.clone();
        use_effect(move || {
            let canvas_element = canvas.cast::<HtmlCanvasElement>().unwrap(); 
            let ctx2d : CanvasRenderingContext2d = canvas_element
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
       
            let full_width = f64::from(canvas_element.width());
            let full_height = f64::from(canvas_element.height());
            ctx2d.clear_rect(0.0,0.0,full_width, full_height);
            let border: f64 = 10.0;
            
            let scale_factor: f64 = 1.0;
           
            let port_height_scaled = cabinet.port_external_height / scale_factor;
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
            
            || {}
            
        });
    }
        
    html! {
        <div class={ "speaker-view" }>
            <h3>{"Side View"}</h3>
            <canvas ref={canvas_ref} width={ canvas_width.to_string() } height={canvas_height.to_string()} />
        </div>
    }
}

#[derive(PartialEq,Properties)]
pub struct CalculationsProps {
    pub nfr: f64,
    pub minimum_diameter: f64,
    pub frequency: f64,
}

#[function_component(Calculations)]
pub fn calculations(props: &CalculationsProps) -> Html {
    html! {
        <div class={ "speaker-info" }>
            <h3>{"Calculated Parameters"}</h3>
            <table>
                <tr><td>{ "Normalized flair ratio" }</td><td>{ &props.nfr } { " (recommended to be 0.5)" }</td></tr>
                <tr><td>{ "Port minimum diameter" }</td><td>{ &props.minimum_diameter } { " millimeters" }</td></tr>
                <tr><td>{ "Frequency" }</td><td>{ &props.frequency } { " Hertz" }</td></tr>
            </table>
        </div>
    }
}
