use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d, DragEvent, DataTransfer, Element};
use wasm_bindgen::{JsCast, JsValue};
use std::fs;
use std::io::{BufWriter, Write};


fn main() {
    
    // let mut file = fs::File::create("src/names.txt").unwrap();
    // let paths = fs::read_dir("Images").unwrap();

    // for path in paths {
    //     file.write_all(format!("{}\n",path.unwrap().path().display().to_string()).as_bytes());
    // }
    yew::start_app::<RootComponent>();
}



pub enum Msg {
    DragStart(DragEvent, (usize, usize)),
    Drag(DragEvent, usize),
    AllowDrop(DragEvent),
    DoDrop(DragEvent, usize),
}

struct RootComponent{
    content: Vec<Vec<usize>>,
    images: Vec<&'static str>,
    drag_start_loc: (usize, usize),
    drag_image: usize
}

impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let layers: usize = 6;
        
        let mut images: Vec<&'static str> = include_str!("names.txt").split("\n").collect::<Vec<&'static str>>();
        images.pop();
        let mut content: Vec<Vec<usize>> = vec![Vec::new();layers];
        for (img_idx, name) in images.iter().enumerate(){
            content[layers-1].push(img_idx);
        }

        //let images: Vec<String> = vec![String::from("Images/marko.png"),String::from("Images/marko.png",String::from("Images/marko.png",String::from("Images/marko.png",String::from("Images/marko.png"]
        RootComponent { content, images, drag_start_loc: (0,0), drag_image: 0}
    }

    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DragStart(e, pos) => {
                self.drag_start_loc=pos;
                // self.test=false;
            }
            Msg::Drag(e, id) => {
                self.drag_image = id;
                //e.data_transfer().unwrap().set_data("text", id.as_ref());
            }
            Msg::AllowDrop(e) => {
                e.prevent_default();
            }
            Msg::DoDrop(e, i) => {
                e.prevent_default();
                let img_id = self.content[self.drag_start_loc.0].swap_remove(self.drag_start_loc.1);
                self.content[i].push(img_id);
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html!{
            <>
                <h1>{"idk what im doing!!!!"}</h1>
                /*
                <div class={"droptarget"} ondrop={link.callback(|e| Msg::DoDrop(e))} ondragover={link.callback(|e| Msg::AllowDrop(e))}>
                    <img ondragstart={link.callback(|e| Msg::DragStart(e))} ondrag={link.callback(|e| Msg::Drag(e))} draggable="true" src="Images/marko.png" alt="Marko" width=128 height=128/>
                </div>
                <p>{"hey"}</p>
                <div class={"droptarget"} ondrop={link.callback(|e| Msg::DoDrop(e))} ondragover={link.callback(|e| Msg::AllowDrop(e))}>
                    
                </div>*/
                //{
                //self.images.iter().enumerate().map(|(i,s)|{
                //    html!{<p>{s.clone()}</p>}
                //}).collect::<Html>()}
                {
                    self.content.iter().enumerate().map(|(i,r)|{
                        html!{
                            <div class={"droptarget"} ondrop={link.callback(move |e| Msg::DoDrop(e, i))} ondragover={link.callback(|e| Msg::AllowDrop(e))}>
                                {r.iter().enumerate().map(|(j,c)|{
                                    let id = c.clone();
                                    html!{
                                        <img ondragstart={link.callback(move |e| Msg::DragStart(e, (i.clone(),j.clone())))} ondrag={link.callback(move |e| Msg::Drag(e, id))} draggable="true" src={self.images[c.clone()]} alt={self.images[c.clone()]} width=128 height=128/>
                                    }
                                }).collect::<Html>()}
                            </div>
                        }
                    }).collect::<Html>()
                }

                // <p>{web_sys::f}</p>
            </>
        }
    }
}

