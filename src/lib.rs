use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlInputElement, Window};

fn window() -> Window {
    web_sys::window().expect("window unavailable")
}

fn document() -> Document {
    window().document().expect("document unavailable")
}

fn esc(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn render(app: &Element) {
    let html = r#"
    <div class="shell">
      <aside class="rail">
        <div class="brand"><span class="brand-mark">W</span><span>WORKSHOP</span></div>
        <nav class="nav">
          <button class="nav-item active" data-view="overview"><span>⌂</span><b>Overview</b></button>
          <button class="nav-item" data-view="knowledge"><span>◈</span><b>Knowledge</b></button>
          <button class="nav-item" data-view="projects"><span>◇</span><b>Projects</b></button>
          <button class="nav-item" data-view="lab"><span>△</span><b>Lab</b></button>
          <button class="nav-item" data-view="tools"><span>◌</span><b>Tools</b></button>
        </nav>
        <div class="rail-bottom">
          <div class="system"><i></i><span>CORE ONLINE</span></div>
          <button class="settings">⚙ <span>Settings</span></button>
        </div>
      </aside>

      <main class="main">
        <header class="topbar">
          <div><span class="eyebrow">PERSONAL LAB / 01</span><h1>Workshop</h1></div>
          <div class="top-actions"><span class="status-pill"><i></i> local</span><button class="icon-btn" id="focus-command">⌘ K</button></div>
        </header>

        <section class="command-card">
          <div class="command-label">COMMAND</div>
          <div class="command-row"><span class="prompt">›</span><input id="command" autocomplete="off" placeholder="Ask Workshop anything…"/><button id="run">Run</button></div>
          <div class="suggestions"><button data-command="Show what I'm learning">learning</button><button data-command="Open active projects">projects</button><button data-command="Start a new experiment">experiment</button><button data-command="Map my knowledge">map</button></div>
        </section>

        <section class="hero-grid">
          <div class="panel constellation-panel">
            <div class="panel-head"><div><span class="kicker">KNOWLEDGE FIELD</span><h2>Where everything connects</h2></div><span class="count">18 nodes</span></div>
            <div class="constellation">
              <div class="orbit o1"></div><div class="orbit o2"></div><div class="orbit o3"></div>
              <div class="node core"><span>AI</span><small>core</small></div>
              <div class="node n1"><span>Python</span><small>active</small></div>
              <div class="node n2"><span>Math</span><small>foundation</small></div>
              <div class="node n3"><span>Systems</span><small>thinking</small></div>
              <div class="node n4"><span>Rust</span><small>building</small></div>
              <div class="node n5"><span>WebAssembly</span><small>experiment</small></div>
              <svg viewBox="0 0 700 420" preserveAspectRatio="none"><line x1="350" y1="210" x2="150" y2="100"/><line x1="350" y1="210" x2="570" y2="105"/><line x1="350" y1="210" x2="135" y2="310"/><line x1="350" y1="210" x2="560" y2="300"/><line x1="350" y1="210" x2="455" y2="80"/><line x1="150" y1="100" x2="570" y2="105"/></svg>
            </div>
          </div>

          <div class="stack">
            <div class="panel progress-panel"><div class="panel-head"><span class="kicker">CURRENT VECTOR</span><span>62%</span></div><h3>Build an AI from first principles</h3><div class="bar"><i></i></div><div class="meta"><span>6 / 10 stages</span><span>continuity matters</span></div></div>
            <div class="panel state-panel"><span class="kicker">WORKING MEMORY</span><p id="memory">Building the bridge between understanding and making.</p><button id="capture">＋ Capture thought</button></div>
          </div>
        </section>

        <section class="lower-grid">
          <div class="panel"><div class="panel-head"><div><span class="kicker">ACTIVE PROJECTS</span><h2>Things being made</h2></div><button class="text-btn" data-view="projects">view all →</button></div><div class="project-list">
            <article><span class="project-icon">W</span><div><b>Workshop</b><p>Personal knowledge + experimentation environment</p></div><em>building</em></article>
            <article><span class="project-icon">λ</span><div><b>Local AI</b><p>Small models, training loops, memory and tools</p></div><em>research</em></article>
            <article><span class="project-icon">↗</span><div><b>Experiments</b><p>Ideas that may become something larger</p></div><em>open</em></article>
          </div></div>
          <div class="panel activity"><div class="panel-head"><div><span class="kicker">LAB LOG</span><h2>Recent activity</h2></div></div><div class="log"><div><span>02:41</span><p>Mapped Rust → WASM architecture</p></div><div><span>00:18</span><p>Connected mathematics to neural networks</p></div><div><span>Yesterday</span><p>Started a new learning path</p></div><div><span>Yesterday</span><p>Captured an idea for Workshop Core</p></div></div></div>
        </section>

        <footer><span>WORKSHOP / v0.1</span><span>Rust + WebAssembly · running locally in the browser</span></footer>
      </main>
    </div>
    <div id="toast" class="toast"></div>
    "#;
    app.set_inner_html(html);
}

fn wire() {
    let doc = document();
    let app = doc.get_element_by_id("app").unwrap();
    render(&app);

    let toast = doc.get_element_by_id("toast").unwrap();
    let command = doc.get_element_by_id("command").unwrap().dyn_into::<HtmlInputElement>().unwrap();
    let memory = doc.get_element_by_id("memory").unwrap();

    let command_buttons = doc.query_selector_all("[data-command]").unwrap();
    for i in 0..command_buttons.length() {
        if let Some(button) = command_buttons.item(i) {
            if let Some(el) = button.dyn_ref::<Element>() {
            let value = el.get_attribute("data-command").unwrap_or_default();
            let input = command.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
                input.set_value(&value);
                let _ = input.focus();
            });
            el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
                closure.forget();
            }
        }
    }

    let run = doc.get_element_by_id("run").unwrap();
    let run_toast = toast.clone();
    let run_memory = memory.clone();
    let run_input = command.clone();
    let run_closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
        let text = run_input.value();
        if text.trim().is_empty() { return; }
        run_memory.set_text_content(Some(&format!("Captured command: {}", text)));
        run_toast.set_text_content(Some("Command captured in working memory"));
        run_toast.set_class_name("toast show");
    });
    run.add_event_listener_with_callback("click", run_closure.as_ref().unchecked_ref()).unwrap();
    run_closure.forget();

    let capture = doc.get_element_by_id("capture").unwrap();
    let capture_toast = toast.clone();
    let capture_memory = memory.clone();
    let capture_closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
        capture_memory.set_text_content(Some("Thought captured. The lab remembers the thread."));
        capture_toast.set_text_content(Some("Thought captured"));
        capture_toast.set_class_name("toast show");
    });
    capture.add_event_listener_with_callback("click", capture_closure.as_ref().unchecked_ref()).unwrap();
    capture_closure.forget();

    let focus = doc.get_element_by_id("focus-command").unwrap();
    let focus_input = command.clone();
    let focus_closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| { let _ = focus_input.focus(); });
    focus.add_event_listener_with_callback("click", focus_closure.as_ref().unchecked_ref()).unwrap();
    focus_closure.forget();

    let view_buttons = doc.query_selector_all("[data-view]").unwrap();
    for i in 0..view_buttons.length() {
        if let Some(button) = view_buttons.item(i) {
            if let Some(el) = button.dyn_ref::<Element>() {
            let label = el.get_attribute("data-view").unwrap_or_default();
            let toast = toast.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
                toast.set_text_content(Some(&format!("{} view · interface layer ready", esc(&label))));
                toast.set_class_name("toast show");
            });
            el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
                closure.forget();
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    wire();
}
