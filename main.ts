const el = (id: string): HTMLElement => document.getElementById(id)!
const sleep = (ms: number) => new Promise<void>(r => setTimeout(r, ms))

const split = (root: HTMLElement): HTMLSpanElement[] => {
	const tw = document.createTreeWalker(root, NodeFilter.SHOW_TEXT), ns: Text[] = [], ws: HTMLSpanElement[] = []
	while (tw.nextNode()) ns.push(tw.currentNode as Text)
	for (const n of ns) {
		const f = document.createDocumentFragment()
		for (const p of n.textContent!.split(/(\s+)/)) {
			if (!p) continue
			if (/^\s+$/.test(p)) { f.append(p); continue }
			const w = document.createElement("span")
			w.textContent = p; w.className = "invisible"; f.append(w); ws.push(w)
		}
		n.replaceWith(f)
	}
	return ws
}

const run = async () => {
	const cmd = el("cmd"), cur = el("cur"), ln = el("line"), sp = el("spin"), out = el("out")
	const ws = split(out)
	out.querySelectorAll("p,h2,ul,li,footer").forEach(b => b.classList.add("invisible"))
	const show = (w: HTMLElement) => { for (let e: HTMLElement | null = w; e && e !== out; e = e.parentElement) e.classList.remove("invisible") }
	const put = (b: HTMLSpanElement[]) => { b.forEach(show); b[b.length - 1].append(sp) }

	const t = cmd.textContent!
	cmd.textContent = ""; cur.classList.remove("hidden")
	await sleep(500)
	for (const c of t) { cmd.textContent += c; await sleep(70 + Math.random() * 90) }
	await sleep(250)
	cur.classList.add("hidden"); ln.classList.remove("hidden")

	let done = false, f = 0
	const spin = async () => { while (!done) { sp.textContent = "/-\\|"[f++ % 4]; await sleep(110) } }
	spin()
	await sleep(900)
	ln.classList.add("hidden"); sp.classList.add("ml-2")
	put(ws.slice(0, 1)); await sleep(400)
	for (let i = 1; i < ws.length; i += 3) { put(ws.slice(i, i + 3)); await sleep(200) }
	done = true; sp.remove()
}

if (!matchMedia("(prefers-reduced-motion: reduce)").matches) run()
