use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Document, Element, HtmlElement, Node, Text};

fn doc() -> Document { web_sys::window().unwrap().document().unwrap() }
fn el(id: &str) -> HtmlElement { doc().get_element_by_id(id).unwrap().unchecked_into() }

async fn sleep(ms: i32) {
	let p = js_sys::Promise::new(&mut |r, _| { web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&r, ms).unwrap(); });
	JsFuture::from(p).await.unwrap();
}

fn split(root: &Element) -> Vec<Element> {
	let d = doc(); let tw = d.create_tree_walker_with_what_to_show(root, 4).unwrap();
	let mut ns: Vec<Text> = vec![]; let mut ws = vec![];
	while let Ok(Some(n)) = tw.next_node() { ns.push(n.unchecked_into()) }
	for n in ns {
		let f = d.create_document_fragment(); let t = n.text_content().unwrap_or_default();
		let mut w = String::new();
		let mut flush = |w: &mut String| { if w.is_empty() { return }; let s = d.create_element("span").unwrap(); s.set_text_content(Some(w)); s.set_class_name("invisible"); f.append_with_node_1(&s).unwrap(); ws.push(s); w.clear() };
		for c in t.chars() {
			if c.is_whitespace() { flush(&mut w); f.append_with_str_1(&c.to_string()).unwrap() } else { w.push(c) }
		}
		flush(&mut w);
		n.replace_with_with_node_1(&f).unwrap();
	}
	ws
}

fn show(w: &Element, out: &Node) {
	let mut e = Some(w.clone());
	while let Some(x) = e { if x.is_same_node(Some(out)) { break }; x.class_list().remove_1("invisible").unwrap(); e = x.parent_element() }
}

async fn run() {
	let (cmd, cur, ln, sp, out) = (el("cmd"), el("cur"), el("line"), el("spin"), el("out"));
	let ws = split(&out);
	let bl = out.query_selector_all("p,h2,ul,li,footer").unwrap();
	for i in 0..bl.length() { bl.item(i).unwrap().unchecked_into::<Element>().class_list().add_1("invisible").unwrap() }
	let put = |b: &[Element]| { for w in b { show(w, &out) }; b[b.len() - 1].append_with_node_1(&sp).unwrap() };

	let t = cmd.text_content().unwrap_or_default();
	cmd.set_text_content(Some("")); cur.class_list().remove_1("hidden").unwrap();
	sleep(500).await;
	let mut s = String::new();
	for c in t.chars() { s.push(c); cmd.set_text_content(Some(&s)); sleep(70 + (js_sys::Math::random() * 90.0) as i32).await }
	sleep(250).await;
	cur.class_list().add_1("hidden").unwrap(); ln.class_list().remove_1("hidden").unwrap();

	let done = Rc::new(Cell::new(false));
	let (d, s2) = (done.clone(), sp.clone());
	spawn_local(async move { let mut f = 0; while !d.get() { s2.set_text_content(Some(["/", "-", "\\", "|"][f % 4])); f += 1; sleep(110).await } });
	sleep(900).await;
	ln.class_list().add_1("hidden").unwrap(); sp.class_list().add_1("ml-2").unwrap();
	put(&ws[..1]); sleep(400).await;
	for b in ws[1..].chunks(3) { put(b); sleep(200).await }
	done.set(true); sp.remove();
}

#[wasm_bindgen(start)]
pub fn start() {
	let w = web_sys::window().unwrap();
	if w.match_media("(prefers-reduced-motion: reduce)").ok().flatten().is_some_and(|m| m.matches()) { return }
	spawn_local(run());
}
