use web_sys::js_sys::Function;
use yew::prelude::*;
// use yew_hooks::prelude::*;
use web_sys::{window, ClipboardEvent, DataTransfer, Document, DragEvent, Element, HtmlInputElement, Url};
use wasm_bindgen::{JsCast, JsValue};


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
    DoDropOn(DragEvent, (usize,usize)),
    DoTouchEnd(TouchEvent, usize),
    DoTouchStart(TouchEvent, (usize, usize)),
    DoTouchOnto(TouchEvent, (usize, usize)),
    DoTouchDrop(TouchEvent, usize),
    ResetImages,
    AddImages,
    AddingImages(String),
    NewColumn,
    DelLastColumn,
    Screenshot,
    PasteImage(ClipboardEvent),
    None
}

struct RootComponent{
    content: Vec<Vec<usize>>,
    images: Vec<String>,
    drag_start_loc: Option<(usize, usize)>,
    drag_image: Option<usize>,
    textbox_content: String,
    screenshot_func: Function,
    test_stuff: String
}

impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let layers: usize = 6;
        
        let mut images: Vec<String> = include_str!("names.txt").split("\n").map(|s| String::from(s)).collect::<Vec<String>>();
        images.pop();
        let mut content: Vec<Vec<usize>> = vec![Vec::new();layers];
        for (img_idx, name) in images.iter().enumerate(){
            content[layers-1].push(img_idx);
        }
        let jsfunc = "let div =
                document.getElementById('tierlist');
                window.scrollTo(0,0);
                html2canvas(div).then(
                function (canvas) {
                    canvas.toBlob(function(blob) { 
                        const item = new ClipboardItem({ \"image/png\": blob });
                        navigator.clipboard.write([item]); 
                    });
                })";
        //let images: Vec<String> = vec![String::from("Images/marko.png"),String::from("Images/marko.png",String::from("Images/marko.png",String::from("Images/marko.png",String::from("Images/marko.png"]
        RootComponent { content, images, drag_start_loc: None, drag_image: None, textbox_content: String::new(), test_stuff:String::new(), screenshot_func: Function::new_no_args(jsfunc)}
    }

    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DragStart(e, pos) => {
                self.drag_start_loc=Some(pos);
                // self.test=false;
            }
            Msg::Drag(e, id) => {
                self.drag_image = Some(id);
                //e.data_transfer().unwrap().set_data("text", id.as_ref());
            }
            Msg::AllowDrop(e) => {
                e.prevent_default();
            }
            Msg::DoDrop(e, i) => {
                e.prevent_default();
                if self.drag_start_loc.is_some() && self.drag_start_loc.unwrap().0!=i{
                    let img_id = self.content[self.drag_start_loc.unwrap().0].remove(self.drag_start_loc.unwrap().1);
                    self.content[i].push(img_id);
                }
                self.drag_start_loc=None;
                self.drag_image=None;
            }
            Msg::DoDropOn(e, pos) => {
                if self.drag_start_loc.is_some(){
                    let img_id = self.content[self.drag_start_loc.unwrap().0].remove(self.drag_start_loc.unwrap().1);
                    self.content[pos.0].insert(pos.1, img_id);
                    self.drag_start_loc=Some(pos);
                    self.drag_image=Some(img_id);
                }
            }
            Msg::DoTouchEnd(e, i) => {
                //e.prevent_default();
                if self.drag_start_loc.is_some() && self.drag_start_loc.unwrap().0!=i{
                    let img_id = self.content[self.drag_start_loc.unwrap().0].remove(self.drag_start_loc.unwrap().1);
                    self.content[i].push(img_id);
                }
                self.drag_start_loc=None;
                self.drag_image=None;
            }
            Msg::DoTouchStart(e, pos) => {
                //e.prevent_default();
                self.drag_start_loc=Some(pos);
                self.drag_image=Some(self.content[pos.0][pos.1]);
            }
            Msg::DoTouchOnto(e, pos) => {
                if self.drag_start_loc.is_some() && self.drag_start_loc.unwrap().0==pos.0{
                    let img_id = self.content[self.drag_start_loc.unwrap().0].remove(self.drag_start_loc.unwrap().1);
                    self.content[pos.0].insert(pos.1, img_id);
                    self.drag_start_loc=Some(pos);
                    self.drag_image=Some(img_id);
                }
            }
            Msg::DoTouchDrop(e,r) => {
                if self.drag_start_loc.is_some() && self.drag_start_loc.unwrap().0!=r{
                    let img_id = self.content[self.drag_start_loc.unwrap().0].remove(self.drag_start_loc.unwrap().1);
                    self.content[r].push(img_id);
                    self.drag_start_loc=None;
                    self.drag_image=None;
                }
            }
            Msg::ResetImages => {
                self.images=Vec::new();
                self.content = vec![Vec::new();self.content.len()];
            }
            Msg::AddImages => {
                let content_len = self.content.len();
                for l in self.textbox_content.split(" "){
                    self.images.push(String::from(l));
                    self.content[content_len-1].push(self.images.len()-1);
                }
                self.textbox_content=String::new();
            }
            Msg::AddingImages(s) => {
                self.textbox_content=s;
            }
            Msg::None => {
                return false
            }
            Msg::NewColumn => {
                self.content.push(Vec::new());
            }
            Msg::DelLastColumn => {
                self.content.pop();
            }
            Msg::Screenshot => {
                let _ = self.screenshot_func.call0(&window().unwrap());
            }
            Msg::PasteImage(e) => {
                let clipboard_item = match(e.clipboard_data()){
                    Some(d) => d.files().unwrap().item(0),
                    None => None
                };
                match clipboard_item{
                    Some(itm) => {
                        if itm.type_().starts_with("image"){
                            let imgsrc = Url::create_object_url_with_blob(&itm).unwrap();
                            self.images.push(imgsrc);
                            let content_len = self.content.len();
                            self.content[content_len-1].push(self.images.len()-1);
                        }
                    }
                    None => ()
                };
                // self.textbox_content=e.type_();
                // self.test_stuff= e.clipboard_data().unwrap().files().unwrap().item(0).unwrap().type_();
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <>
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
                <div id="tierlist">
                <input type={"text"} class={"inputname"} placeholder="Insert Name"/>
                <hr/>
                {
                    self.content.iter().enumerate().map(|(i,r)|{
                        html!{
                            <div class={"droptarget"} ondrop={link.callback(move |e| Msg::DoDrop(e, i))} ondragover={link.callback(|e| Msg::AllowDrop(e))} ontouchstart={link.callback(move|e| Msg::DoTouchDrop(e, i))}> // ontouchstart={link.callback(move|e| Msg::DoTouchEnd(e, i))
                                {r.iter().enumerate().map(|(j,c)|{
                                    let id = c.clone();
                                    html!{
                                        if self.drag_start_loc.is_some() && self.drag_start_loc.unwrap().0==i && self.drag_start_loc.unwrap().1==j{
                                            <img ondragstart={link.callback(move |e| Msg::DragStart(e, (i.clone(),j.clone())))} ondrag={link.callback(move |e| Msg::Drag(e, id))} ontouchstart={link.callback(move |e| Msg::DoTouchEnd(e, i.clone()))} draggable="true" src={self.images[c.clone()].clone()} alt={self.images[c.clone()].clone()} class="no-touch-move touch-selected" width=128 height=128/>
                                        }else if self.drag_start_loc.is_some() && self.drag_start_loc.unwrap().0==i{
                                            <img ondragstart={link.callback(move |e| Msg::DragStart(e, (i.clone(),j.clone())))} ondrop={link.callback(move |e| Msg::DoDropOn(e, (i,j)))} ondragover={link.callback(|e| Msg::AllowDrop(e))} ontouchend={link.callback(move |e| Msg::DoTouchOnto(e, (i.clone(),j.clone())))} draggable="true" src={self.images[c.clone()].clone()} alt={self.images[c.clone()].clone()} class="no-touch-move 1" width=128 height=128/>
                                        }else{
                                            <img ondragstart={link.callback(move |e| Msg::DragStart(e, (i.clone(),j.clone())))} ondrop={link.callback(move |e| Msg::DoDropOn(e, (i,j)))} ondragover={link.callback(|e| Msg::AllowDrop(e))} ontouchstart={link.callback(move |e| Msg::DoTouchStart(e, (i.clone(),j.clone())))} draggable="true" src={self.images[c.clone()].clone()} alt={self.images[c.clone()].clone()} class="no-touch-move 2" width=128 height=128/>
                                        }
                                    }
                                }).collect::<Html>()}
                            </div>
                        }
                    }).collect::<Html>()
                }
                </div>
                <hr/>
                <div class="change-image-section">
                <button class="smaller-button" onclick={link.callback(|_| Msg::Screenshot)}>{"Take Screenshot"}</button>

                <button class="smaller-button" onclick={link.callback(|_| Msg::NewColumn)}>{"Add Row"}</button>
                <button class="smaller-button" onclick={link.callback(|_| Msg::DelLastColumn)}>{"Remove Row"}</button>
                <br/>
                // <p>{"If you want to use your own images, clear all images then copy paste images or image links into box below (can either be added seperately or a space-separated list of links)"}</p>
                    <button onclick={link.callback(|_| Msg::ResetImages)}>{"Clear Images"}</button>
                    <input class="add-image-links" type="text" value={self.textbox_content.clone()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::AddingImages(input.value())})} onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {Msg::AddImages} else{Msg::None}})} onpaste={link.callback(|e: Event| Msg::PasteImage(e.unchecked_into::<ClipboardEvent>()))} placeholder="paste image or image link"/>
                    <button onclick={link.callback(|_| Msg::AddImages)}>{"Add Images"}</button>
                </div>
            </>
        }
    }
}

