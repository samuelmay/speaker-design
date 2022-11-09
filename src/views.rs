use yew::{Component,function_component,functional::{use_node_ref,use_effect},NodeRef,Context,Html,html,Properties};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{JsCast,JsValue};
use js_sys::Array;
use crate::cabinet::{CabinetModel};
use std::f64::consts::PI;

#[derive(Clone, PartialEq, Properties)]
pub struct SchematicViewProps {
    pub cabinet: CabinetModel,
    pub scale: f64,
}

#[function_component(FrontView)]
pub fn front_view(props: &SchematicViewProps) -> Html {
    let border: f64 = 40.0;
    let canvas_width = format!("{:0}", props.cabinet.port_external_width*props.scale + 2.0*border);
    let canvas_height = format!("{:0}", props.cabinet.port_external_height*props.scale + 2.0*border);
    
    let canvas_ref = use_node_ref();

    {
        let cabinet = props.cabinet.clone();
        let canvas = canvas_ref.clone();
        let scale_factor = props.scale;
        use_effect(move || {
            let canvas_element = canvas.cast::<HtmlCanvasElement>().unwrap();
            let ctx2d : CanvasRenderingContext2d = canvas_element
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            
            ctx2d.save();
            ctx2d.translate(border, border).unwrap();
            ctx2d.scale(scale_factor, scale_factor).unwrap();

            let full_width = f64::from(canvas_element.width());
            let full_height = f64::from(canvas_element.height());
            ctx2d.clear_rect(0.0,0.0,full_width, full_height);
           
            let min_radius = cabinet.port_min_diameter() / 2.0;

            ctx2d.begin_path();
            ctx2d.set_stroke_style(&JsValue::from("brown"));
            ctx2d.move_to(0.0,0.0);
            ctx2d.line_to(0.0, cabinet.port_external_height);
            ctx2d.move_to(cabinet.port_external_width, cabinet.port_external_height);
            ctx2d.line_to(cabinet.port_external_width, 0.0);
            ctx2d.close_path();
            ctx2d.stroke();

            ctx2d.set_fill_style(&JsValue::from("brown"));
            
            let front_view_box_height = cabinet.port_external_height/2.0-min_radius;
            ctx2d.fill_rect(0.0, 0.0, cabinet.port_external_width, front_view_box_height);
            ctx2d.fill_rect(0.0, cabinet.port_external_height - front_view_box_height, cabinet.port_external_width, front_view_box_height);

            let inner_edge_y = (cabinet.port_external_height/2.0) - (cabinet.port_min_diameter() / 2.0);
            draw_labeled_arrow(&ctx2d,0.0,-10.0,cabinet.port_external_width,0.0);
            draw_labeled_arrow(&ctx2d,cabinet.port_external_width+10.0,0.0,cabinet.port_external_height,PI/2.0);
            draw_labeled_arrow(&ctx2d,cabinet.port_external_width/2.0,inner_edge_y,cabinet.port_min_diameter(), PI/2.0);

            || {}
        });

    }

    html! {
        <div class={ "speaker-view" }>
            <h3>{"Front View"}</h3>
            <canvas ref={canvas_ref} width={ canvas_width.to_string() } height={canvas_height.to_string()} />
        </div>
    }
}

fn draw_arrow(ctx2d: &CanvasRenderingContext2d, x: f64, y: f64, length: f64, theta: f64) {
    ctx2d.save();
    ctx2d.translate(x, y).unwrap();
    ctx2d.rotate(theta).unwrap();
    ctx2d.begin_path();
    ctx2d.set_stroke_style(&JsValue::from("blue"));
    
    // arrowhead
    ctx2d.move_to(0.0, 0.0);
    ctx2d.line_to(4.0, -3.0);
    ctx2d.move_to(0.0, 0.0);
    ctx2d.line_to(4.0, 3.0);

    // main line
    ctx2d.move_to(0.0, 0.0);
    ctx2d.line_to(length, 0.0);

    // second arrowhead
    ctx2d.move_to(length, 0.0);
    ctx2d.line_to(length-4.0, -3.0);
    ctx2d.move_to(length, 0.0);
    ctx2d.line_to(length-4.0, 3.0);

    ctx2d.stroke();
    ctx2d.restore();

}

fn draw_label(ctx2d: &CanvasRenderingContext2d, x: f64, y: f64, label: &str) {
    ctx2d.save();
    
    ctx2d.set_fill_style(&JsValue::from("blue"));
    ctx2d.fill_text(&label,x,y).unwrap();

    ctx2d.restore();
}

fn draw_labeled_arrow(ctx2d: &CanvasRenderingContext2d, x: f64, y: f64, length: f64, theta: f64) {
    let label = format!("{:.0}mm",length);
    let text_offset = 5.0;
    let text_centering_factor = 15.0;
    ctx2d.save();
    ctx2d.translate(x, y).unwrap();
    ctx2d.rotate(theta).unwrap();
    
    draw_arrow(ctx2d,0.0,0.0,length,0.0);
    draw_label(ctx2d,length/2.0-text_centering_factor,-text_offset, &label);

    ctx2d.restore();
}

#[function_component(SideView)]
pub fn side_view(props: &SchematicViewProps) -> Html {
    let border: f64 = 40.0;
    let canvas_width = format!("{:0}", props.cabinet.port_length*props.scale + 2.0*border);
    let canvas_height = format!("{:0}", props.cabinet.port_external_height*props.scale + 2.0*border);

    let canvas_ref = use_node_ref();
    
    {
        let cabinet = props.cabinet.clone();
        let canvas = canvas_ref.clone();
        let scale_factor = props.scale;
        use_effect(move || {
            let canvas_element = canvas.cast::<HtmlCanvasElement>().unwrap(); 
            let ctx2d : CanvasRenderingContext2d = canvas_element
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            
            ctx2d.save();
            ctx2d.translate(border, border).unwrap();
            ctx2d.scale(scale_factor, scale_factor).unwrap();
       
            let full_width = f64::from(canvas_element.width());
            let full_height = f64::from(canvas_element.height());
            ctx2d.clear_rect(0.0,0.0,full_width, full_height);
            
            let square_mid = cabinet.port_external_height/2.0;
            let min_radius = cabinet.port_min_diameter() / 2.0;
            let alpha = cabinet.port_flare_arc_start();

            ctx2d.begin_path();
            ctx2d.set_fill_style(&JsValue::from("brown"));
            ctx2d.set_stroke_style(&JsValue::from("brown"));
            ctx2d.arc(
                cabinet.port_length / 2.0,
                square_mid - min_radius - cabinet.port_flare_radius,
                cabinet.port_flare_radius,
                alpha,
                PI - alpha
            ).unwrap();
            ctx2d.close_path();
            ctx2d.fill();

            ctx2d.begin_path();
            ctx2d.arc(
                cabinet.port_length / 2.0,
                square_mid + min_radius + cabinet.port_flare_radius,
                cabinet.port_flare_radius,
                PI + alpha,
                2.0*PI - alpha
            ).unwrap();
            ctx2d.close_path();
            ctx2d.fill();
            
            draw_labeled_arrow(&ctx2d,0.0,-10.0,cabinet.port_length,0.0);
            draw_labeled_arrow(&ctx2d,cabinet.port_length+10.0,0.0,cabinet.port_external_height,PI/2.0);
            draw_labeled_arrow(&ctx2d,cabinet.port_length/2.0,square_mid-min_radius,min_radius*2.0,PI/2.0);
            draw_arrow(
                &ctx2d,
                cabinet.port_length/2.0,
                square_mid + min_radius + cabinet.port_flare_radius,
                cabinet.port_flare_radius,
                (-8.0/9.0)*(PI/2.0)
            );
            let radius_label = format!("radius {:.0}mm",cabinet.port_flare_radius);
            draw_label(&ctx2d, 0.0,cabinet.port_external_height + 20.0,&radius_label);
            ctx2d.restore();
            
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
