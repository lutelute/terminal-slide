/* =============================================================================
 *  terminal-slide.js  —  Drop-in presentation system
 *  Usage: <script src="terminal-slide.js"></script>   (that's it)
 *
 *  Auto-provides:
 *    - Slide navigation (arrow / vim / touch)
 *    - Toolbar, jump grid, gallery overlay
 *    - Terminal typewriter animation  (typeInTerminal / skipAnim / togglePause)
 *    - Dual-pane sync                 (tsSyncAction)
 *    - Auto-animate: .step, .flow-box, .sp-node[data-step]
 *    - All component CSS injected automatically
 * ========================================================================== */

(function () {
  'use strict';

  /* ===================================================================
   *  1.  CSS INJECTION  — inject all styles once
   * =================================================================== */
  if (!document.getElementById('_ts-injected-css')) {
    var style = document.createElement('style');
    style.id = '_ts-injected-css';
    style.textContent = [
      /* --- Reset & Base --- */
      '*,*::before,*::after{margin:0;padding:0;box-sizing:border-box}',
      "body{font-family:-apple-system,BlinkMacSystemFont,'Hiragino Sans',sans-serif;background:#0f0f1a;color:#e0e0e0;overflow:hidden}",

      /* --- Slide --- */
      '.slide{display:none;flex-direction:column;justify-content:center;align-items:center;height:100vh;padding:2.5rem 4rem;position:absolute;top:0;left:0;width:100%}',
      '.slide.active{display:flex;animation:fadeIn .35s cubic-bezier(.33,1,.68,1)}',
      '@keyframes fadeIn{from{opacity:0;transform:translateY(20px)}to{opacity:1;transform:translateY(0)}}',

      /* --- Typography --- */
      'h1{font-size:2.8rem;color:#00d2ff;margin-bottom:.5rem;text-align:center}',
      'h2{font-size:2rem;color:#00d2ff;margin-bottom:1.2rem;text-align:center;width:100%}',
      'h3{font-size:1.4rem;color:#7fdbca;margin-bottom:.8rem}',
      'p{font-size:1.05rem;line-height:1.7;color:#bbb}',
      "code{background:#1e1e30;padding:.15em .4em;border-radius:3px;font-family:'Fira Code','SF Mono',monospace;color:#7fdbca;font-size:.9em}",

      /* --- Labels --- */
      '.label{display:inline-block;background:#00d2ff22;color:#00d2ff;padding:.2em .7em;border-radius:4px;font-size:.85rem;margin-bottom:.8rem;font-weight:600}',
      '.label.green{background:#7fdbca22;color:#7fdbca}',
      '.label.purple{background:#c792ea22;color:#c792ea}',
      '.label.orange{background:#f78c6c22;color:#f78c6c}',
      '.label.red{background:#ff537022;color:#ff5370}',
      '.label.yellow{background:#ffcb6b22;color:#ffcb6b}',

      /* --- Layout --- */
      '.card{background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:1.5rem}',
      '.cols{display:flex;gap:2rem;width:100%;max-width:1100px}',
      '.cols>*{flex:1}',
      '.progress{position:fixed;bottom:1rem;right:1.5rem;font-size:.85rem;color:#555;z-index:100}',
      '.nav-hint{position:fixed;bottom:1rem;left:1.5rem;font-size:.8rem;color:#444;z-index:100}',

      /* --- Feature list --- */
      'ul.feature-list{list-style:none;max-width:700px}',
      "ul.feature-list li{padding:.5rem 0;font-size:1.05rem;color:#bbb;border-bottom:1px solid #1a1a2e}",
      "ul.feature-list li::before{content:'\\25B8 ';color:#00d2ff}",

      /* --- Animations --- */
      '.bounce{animation:bounce 2s infinite}',
      '@keyframes bounce{0%,100%{transform:translateY(0)}50%{transform:translateY(-12px)}}',
      '.pulse{animation:pulse 2s ease-in-out infinite}',
      '@keyframes pulse{0%,100%{opacity:1;transform:scale(1)}50%{opacity:.7;transform:scale(1.05)}}',

      /* --- Step reveal --- */
      '.step{opacity:.3;transition:opacity .5s,transform .5s;transform:translateX(-5px)}',
      '.step.visible{opacity:1;transform:translateX(0)}',

      /* --- AI badge --- */
      '.ai-badge{display:inline-flex;align-items:center;gap:6px;background:linear-gradient(135deg,#c792ea33,#00d2ff33);border:1px solid #c792ea55;border-radius:20px;padding:4px 14px;font-size:.85rem;color:#c792ea}',

      /* --- Terminal --- */
      '.terminal{background:#0a0a15;border:1px solid #2a2a4a;border-radius:10px;overflow:hidden;width:100%;max-width:900px}',
      '.terminal-bar{background:#1a1a2e;padding:8px 12px;display:flex;align-items:center;gap:6px}',
      '.terminal-bar .dot{width:10px;height:10px;border-radius:50%}',
      '.terminal-bar .dot.r{background:#ff5f56}.terminal-bar .dot.y{background:#ffbd2e}.terminal-bar .dot.g{background:#27c93f}',
      '.terminal-bar span{color:#666;font-size:.8rem;margin-left:8px}',
      ".terminal-body{padding:1.2rem 1.5rem;font-family:'Fira Code','SF Mono',monospace;font-size:.95rem;min-height:180px;max-height:55vh;overflow-y:auto;line-height:1.6}",
      '.terminal-body .prompt{color:#7fdbca}',
      '.terminal-body .cmd{color:#e0e0e0}',
      '.terminal-body .output{color:#888}',
      '.terminal-body .success{color:#27c93f}',
      '.terminal-body .error{color:#ff5370}',
      '.terminal-body .info{color:#00d2ff}',
      '.terminal-body .warn{color:#ffcb6b}',

      /* --- GitHub UI mock --- */
      '.github-ui{background:#0d1117;border:1px solid #30363d;border-radius:10px;overflow:hidden;width:100%;max-width:900px}',
      '.github-header{background:#161b22;padding:10px 16px;display:flex;align-items:center;gap:10px;border-bottom:1px solid #30363d}',
      '.github-header .gh-icon{color:#e0e0e0;font-size:1.3rem}',
      '.github-header .gh-repo{color:#58a6ff;font-size:.95rem;font-weight:600}',
      '.github-body{padding:1.2rem 1.5rem}',
      '.gh-file{display:flex;align-items:center;gap:10px;padding:8px 0;border-bottom:1px solid #21262d}',
      '.gh-file .icon{color:#8b949e;font-size:1rem}',
      '.gh-file .name{color:#58a6ff;font-size:.9rem}',
      '.gh-file .msg{color:#8b949e;font-size:.85rem;flex:1;text-align:right}',
      ".gh-commit{display:flex;align-items:center;gap:8px;padding:6px 0}",
      ".gh-commit .hash{color:#58a6ff;font-family:'Fira Code',monospace;font-size:.85rem}",
      '.gh-commit .cm{color:#c9d1d9;font-size:.9rem}',
      ".gh-diff-add{background:#1a4721;color:#7ee787;padding:2px 8px;font-family:'Fira Code',monospace;font-size:.85rem;border-left:3px solid #27c93f}",
      ".gh-diff-del{background:#4d1f28;color:#ff7b72;padding:2px 8px;font-family:'Fira Code',monospace;font-size:.85rem;border-left:3px solid #ff5370}",
      ".gh-diff-ctx{background:#161b22;color:#8b949e;padding:2px 8px;font-family:'Fira Code',monospace;font-size:.85rem;border-left:3px solid #30363d}",

      /* --- Flow diagram --- */
      '.flow{display:flex;align-items:center;gap:0;justify-content:center;flex-wrap:wrap}',
      '.flow-box{background:#1a1a2e;border:2px solid #2a2a4a;border-radius:10px;padding:.8rem 1.2rem;text-align:center;min-width:120px;transition:all .5s}',
      '.flow-box.active-box{border-color:#00d2ff;box-shadow:0 0 20px rgba(0,210,255,.3)}',
      '.flow-arrow{color:#555;font-size:1.5rem;padding:0 .3rem}',

      /* --- Step progress --- */
      '.step-progress{display:flex;align-items:center;justify-content:center;gap:0;margin-bottom:1.2rem;width:100%;max-width:900px}',
      '.step-progress .sp-item{display:flex;align-items:center;gap:0}',
      '.step-progress .sp-node{padding:.3rem .7rem;border-radius:6px;font-size:.7rem;font-weight:600;background:#1a1a2e;border:1.5px solid #2a2a4a;color:#666;transition:all .3s;white-space:nowrap}',
      '.step-progress .sp-node.sp-done{background:#00d2ff15;border-color:#00d2ff55;color:#00d2ff}',
      '.step-progress .sp-node.sp-active{background:#ffcb6b22;border-color:#ffcb6b;color:#ffcb6b;box-shadow:0 0 10px rgba(255,203,107,.2)}',
      '.step-progress .sp-arrow{color:#333;font-size:.8rem;padding:0 .3rem}',
      '.step-progress .sp-arrow.sp-done{color:#00d2ff55}',

      /* --- Timeline --- */
      ".timeline{position:relative;max-width:700px;width:100%}",
      ".timeline::before{content:'';position:absolute;left:20px;top:0;bottom:0;width:2px;background:#2a2a4a}",
      '.timeline-item{position:relative;padding-left:50px;margin-bottom:1rem}',
      ".timeline-item::before{content:'';position:absolute;left:14px;top:6px;width:14px;height:14px;border-radius:50%;background:#2a2a4a;border:2px solid #0f0f1a;transition:all .5s;z-index:1}",
      '.timeline-item.done::before{background:#00d2ff;box-shadow:0 0 10px rgba(0,210,255,.4)}',
      '.timeline-item.active-item::before{background:#ffcb6b;box-shadow:0 0 10px rgba(255,203,107,.4)}',

      /* --- Branch viz --- */
      ".branch-viz{background:#0a0a15;border:1px solid #2a2a4a;border-radius:10px;padding:1rem;min-width:220px}",
      ".branch-viz svg text{font-family:-apple-system,BlinkMacSystemFont,sans-serif}",

      /* --- Anim controls --- */
      '.anim-controls{position:fixed;bottom:3rem;right:1.5rem;display:none;gap:.5rem;z-index:200}',
      '.anim-controls.show{display:flex}',
      '.anim-btn{background:#1a1a2e;border:1px solid #2a2a4a;color:#00d2ff;padding:.4rem .8rem;border-radius:6px;cursor:pointer;font-size:.85rem;transition:all .2s;font-family:inherit}',
      '.anim-btn:hover{background:#2a2a4a;border-color:#00d2ff}',
      '.anim-btn.paused{color:#ffcb6b;border-color:#ffcb6b}',

      /* --- Toolbar / Jump / Gallery --- */
      "._ts-toolbar{position:fixed;bottom:1rem;left:1.5rem;display:flex;gap:.5rem;z-index:9999;font-family:-apple-system,BlinkMacSystemFont,sans-serif}",
      '._ts-btn{background:rgba(30,30,48,.85);color:#888;border:1px solid #333;padding:.4rem .8rem;border-radius:6px;font-size:.85rem;cursor:pointer;transition:all .2s;backdrop-filter:blur(8px);-webkit-backdrop-filter:blur(8px);user-select:none}',
      '._ts-btn:hover{background:rgba(0,210,255,.15);color:#00d2ff;border-color:#00d2ff55}',
      '._ts-btn._ts-active{color:#00d2ff;border-color:#00d2ff55}',
      '._ts-jump{position:fixed;bottom:3.5rem;left:1.5rem;background:rgba(20,20,35,.95);border:1px solid #333;border-radius:10px;padding:.8rem;display:none;z-index:9999;backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);max-height:60vh;overflow-y:auto}',
      '._ts-jump.show{display:block;animation:_tsFadeUp .2s ease-out}',
      '._ts-jump-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(42px,1fr));gap:.4rem;min-width:200px}',
      '._ts-jump-item{background:rgba(30,30,48,.9);color:#aaa;border:1px solid #2a2a4a;border-radius:6px;padding:.4rem;text-align:center;cursor:pointer;font-size:.85rem;transition:all .15s}',
      '._ts-jump-item:hover{background:rgba(0,210,255,.2);color:#fff;border-color:#00d2ff}',
      '._ts-jump-item._ts-current{background:rgba(0,210,255,.25);color:#00d2ff;border-color:#00d2ff;font-weight:bold}',
      '._ts-gallery-overlay{position:fixed;inset:0;background:rgba(5,5,15,.92);z-index:9998;display:none;overflow-y:auto;backdrop-filter:blur(8px);-webkit-backdrop-filter:blur(8px)}',
      '._ts-gallery-overlay.show{display:block;animation:_tsFadeIn .25s ease-out}',
      '._ts-gallery-header{position:sticky;top:0;display:flex;justify-content:space-between;align-items:center;padding:1.2rem 2rem;background:rgba(5,5,15,.8);backdrop-filter:blur(8px);z-index:1}',
      '._ts-gallery-title{color:#00d2ff;font-size:1.2rem;font-weight:600}',
      '._ts-gallery-close{background:none;border:1px solid #333;color:#888;width:36px;height:36px;border-radius:8px;font-size:1.2rem;cursor:pointer;display:flex;align-items:center;justify-content:center;transition:all .2s}',
      '._ts-gallery-close:hover{color:#ff5370;border-color:#ff5370}',
      '._ts-gallery-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:1.2rem;padding:1.2rem 2rem 2rem}',
      '._ts-gallery-card{background:rgba(26,26,46,.9);border:1px solid #2a2a4a;border-radius:10px;cursor:pointer;transition:all .2s;overflow:hidden;position:relative}',
      '._ts-gallery-card:hover{border-color:#00d2ff;transform:translateY(-3px);box-shadow:0 8px 24px rgba(0,210,255,.12)}',
      '._ts-gallery-card._ts-current-card{border-color:#00d2ff;box-shadow:0 0 12px rgba(0,210,255,.2)}',
      '._ts-gallery-num{position:absolute;top:.6rem;left:.8rem;background:rgba(0,210,255,.2);color:#00d2ff;font-size:.75rem;font-weight:700;padding:.15rem .5rem;border-radius:4px;z-index:1}',
      '._ts-gallery-preview{height:160px;overflow:hidden;position:relative;background:#0f0f1a}',
      '._ts-gallery-preview-inner{transform:scale(0.28);transform-origin:top left;width:357%;height:357%;pointer-events:none;position:absolute;top:0;left:0}',
      '._ts-gallery-info{padding:.8rem 1rem}',
      '._ts-gallery-info h4{color:#e0e0e0;font-size:.9rem;margin:0;font-weight:500;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}',
      '._ts-gallery-info p{color:#666;font-size:.75rem;margin:.2rem 0 0}',
      '@keyframes _tsFadeUp{from{opacity:0;transform:translateY(8px)}to{opacity:1;transform:translateY(0)}}',
      '@keyframes _tsFadeIn{from{opacity:0}to{opacity:1}}',

      /* --- Landing page (body.ts-landing) --- */
      'body.ts-landing{overflow:auto;display:flex;flex-direction:column;align-items:center;padding:2.5rem 2rem;min-height:100vh}',
      '.roadmap{max-width:1000px;width:100%;margin-bottom:2rem}',
      '.roadmap h2{font-size:1.1rem;color:#555;margin-bottom:1rem;text-align:center;letter-spacing:.1em}',
      '.path{display:flex;align-items:center;justify-content:center;gap:0;flex-wrap:wrap;margin-bottom:.5rem}',
      ".path-node{background:#1a1a2e;border:2px solid #2a2a4a;border-radius:8px;padding:.4rem .8rem;font-size:.8rem;color:#bbb;text-decoration:none;transition:all .25s;cursor:pointer;white-space:nowrap}",
      '.path-node:hover{border-color:#00d2ff;color:#00d2ff;box-shadow:0 0 12px rgba(0,210,255,.2)}',
      '.path-arrow{color:#333;font-size:1rem;padding:0 .2rem}',
      '.path-label{font-size:.75rem;color:#555;text-align:center;margin-bottom:1.5rem}',
      '.path-branch{display:flex;align-items:flex-start;justify-content:center;gap:2rem;flex-wrap:wrap}',
      '.path-group{display:flex;flex-direction:column;align-items:center;gap:.5rem}',
      '.path-group-label{font-size:.7rem;color:#444;margin-bottom:.3rem;text-transform:uppercase;letter-spacing:.05em}',

      /* --- Card grid --- */
      '.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:1rem;max-width:1000px;width:100%}',
      '.grid .card{text-decoration:none;transition:all .25s;cursor:pointer;display:block;position:relative}',
      '.grid .card:hover{border-color:#00d2ff;box-shadow:0 0 20px rgba(0,210,255,.15);transform:translateY(-2px)}',
      '.grid .card .num{font-size:.75rem;color:#555;margin-bottom:.3rem}',
      '.grid .card h2{font-size:1.1rem;text-align:left;margin-bottom:.4rem}',
      '.grid .card p{font-size:.85rem;color:#888;line-height:1.4}',
      '.grid .card .meta{display:flex;gap:.5rem;margin-top:.5rem;flex-wrap:wrap}',
      '.tag{display:inline-block;padding:.15em .5em;border-radius:3px;font-size:.7rem}',
      '.diff{font-size:.7rem;padding:.15em .5em;border-radius:3px}',
      '.diff.beginner{background:#7fdbca15;color:#7fdbca}',
      '.diff.intermediate{background:#ffcb6b15;color:#ffcb6b}',
      '.diff.advanced{background:#ff537015;color:#ff5370}',
      '.prereq{font-size:.7rem;color:#444;margin-top:.3rem}',
      '.section-label{max-width:1000px;width:100%;font-size:.85rem;color:#555;margin:1.5rem 0 .8rem;padding-left:.8rem;border-left:2px solid #2a2a4a}',
      '.footer{margin-top:2rem;color:#444;font-size:.8rem;text-align:center}',
      '.footer a{color:#555;text-decoration:none}',
      '.footer a:hover{color:#00d2ff}',

      /* --- Process Steps template --- */
      '.process-slide{display:flex;flex-direction:column;align-items:center;width:100%;max-width:1100px;gap:1.5rem}',
      '.process-row{display:flex;align-items:stretch;gap:0;width:100%}',
      '.process-block{flex:1;background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:1.5rem 1.2rem;display:flex;flex-direction:column;align-items:center;text-align:center;transition:all .5s}',
      '.process-block.active-box{border-color:#00d2ff;box-shadow:0 0 20px rgba(0,210,255,.2)}',
      '.process-num{width:44px;height:44px;border-radius:50%;background:linear-gradient(135deg,#00d2ff22,#00d2ff44);color:#00d2ff;font-size:1.3rem;font-weight:700;display:flex;align-items:center;justify-content:center;margin-bottom:.8rem;border:2px solid #00d2ff55;flex-shrink:0}',
      '.process-block h3{color:#e0e0e0;font-size:1.1rem;margin-bottom:.5rem}',
      '.process-block p{color:#999;font-size:.9rem;line-height:1.5}',
      ".process-block .mini-cmd{background:#0a0a15;border:1px solid #2a2a4a;border-radius:6px;padding:.4rem .7rem;margin-top:.6rem;font-family:'Fira Code','SF Mono',monospace;font-size:.75rem;color:#7fdbca;text-align:left;width:100%;overflow-x:auto;white-space:nowrap}",
      '.process-block .mini-cmd .prompt{color:#7fdbca}.process-block .mini-cmd .cmd{color:#e0e0e0}',
      '.process-block .detail{color:#666;font-size:.75rem;margin-top:.5rem;line-height:1.4;text-align:left;width:100%}',
      ".process-block .detail li{list-style:none;padding:.15rem 0}",
      ".process-block .detail li::before{content:'\\25B8 ';color:#00d2ff88;font-size:.65rem}",
      '.process-arrow{display:flex;align-items:center;color:#00d2ff55;font-size:1.8rem;padding:0 .6rem;flex-shrink:0}',
      '.process-summary{background:#1a1a2e88;border:1px solid #2a2a4a;border-radius:8px;padding:.8rem 1.5rem;width:100%;text-align:center}',
      '.process-summary p{color:#bbb;font-size:.95rem;line-height:1.6}',
    ].join('\n');
    document.head.appendChild(style);
  }


  /* ===================================================================
   *  2.  SLIDE NAVIGATION
   * =================================================================== */
  var slides, prog, cur = 0, busy = false;

  function init() {
    slides = document.querySelectorAll('.slide');
    prog   = document.querySelector('.progress');
    if (!slides.length) return;

    /* Ensure anim-controls exist */
    if (!document.getElementById('anim-controls')) {
      var ac = document.createElement('div');
      ac.className = 'anim-controls'; ac.id = 'anim-controls';
      ac.innerHTML = '<button class="anim-btn" id="btn-skip" onclick="skipAnim()">⏭ スキップ</button><button class="anim-btn" id="btn-pause" onclick="togglePause()">⏸ 一時停止</button>';
      document.body.appendChild(ac);
    }

    if (prog) prog.textContent = '1 / ' + slides.length;
    onSlideEnter(0);
    initKeyboard();
    initTouch();
    if (slides.length >= 2) initToolbar();
  }

  function go(i) {
    if (busy || i === cur) return;
    var next = Math.max(0, Math.min(i, slides.length - 1));
    if (next === cur) return;
    busy = true;
    slides[cur].classList.remove('active');
    slides[next].style.animation = 'none'; slides[next].offsetHeight; slides[next].style.animation = '';
    slides[next].classList.add('active');
    cur = next;
    if (prog) prog.textContent = (cur + 1) + ' / ' + slides.length;
    onSlideEnter(cur);
    setTimeout(function () { busy = false }, 350);
  }

  /* Expose globally — slide-specific scripts call these */
  window.go = go;


  /* ===================================================================
   *  3.  KEYBOARD  (Vim + arrows + shortcuts)
   * =================================================================== */
  function initKeyboard() {
    document.addEventListener('keydown', function (e) {
      if (e.target.tagName === 'TEXTAREA' || e.target.tagName === 'INPUT') return;
      if (['ArrowRight','l','j','n',' '].indexOf(e.key) >= 0) { e.preventDefault(); go(cur + 1) }
      else if (['ArrowLeft','h','k'].indexOf(e.key) >= 0)     { e.preventDefault(); go(cur - 1) }
      else if (e.key === 'g') go(0);
      else if (e.key === 'G') go(slides.length - 1);
      else if (e.key === 's') { skipAnim() }
      else if (e.key === 'p') { togglePause() }
    });
  }


  /* ===================================================================
   *  4.  TOUCH / SWIPE
   * =================================================================== */
  function initTouch() {
    var sx = 0, sy = 0;
    document.addEventListener('touchstart', function (e) {
      sx = e.changedTouches[0].screenX; sy = e.changedTouches[0].screenY;
    }, { passive: true });
    document.addEventListener('touchend', function (e) {
      var dx = e.changedTouches[0].screenX - sx;
      var dy = e.changedTouches[0].screenY - sy;
      if (Math.abs(dx) < 50 || Math.abs(dy) > Math.abs(dx)) return;
      if (dx < 0) go(cur + 1); else go(cur - 1);
    }, { passive: true });
  }


  /* ===================================================================
   *  5.  SLIDE ENTER HOOK  (auto-animate components)
   * =================================================================== */
  function onSlideEnter(idx) {
    var slide = slides[idx];

    /* .step → staggered reveal */
    var steps = slide.querySelectorAll('.step');
    steps.forEach(function (s, i) {
      s.classList.remove('visible');
      setTimeout(function () { s.classList.add('visible') }, 300 + i * 600);
    });

    /* .flow-box → sequential glow */
    var flowBoxes = slide.querySelectorAll('.flow-box');
    flowBoxes.forEach(function (b, i) {
      b.classList.remove('active-box');
      setTimeout(function () { b.classList.add('active-box') }, 400 + i * 800);
    });

    /* .sp-node[data-step] → progress fill */
    var spNodes = slide.querySelectorAll('.sp-node[data-step]');
    spNodes.forEach(function (n, i) {
      n.classList.remove('sp-done', 'sp-active');
      setTimeout(function () {
        if (i < spNodes.length - 1) n.classList.add('sp-done');
        else n.classList.add('sp-active');
      }, 200 + i * 400);
    });

    /* Fire custom event — page scripts can listen: slide.addEventListener('ts-enter', fn) */
    slide.dispatchEvent(new CustomEvent('ts-enter', { detail: { index: idx } }));

    /* Call global onSlideEnter if user defined one */
    if (typeof window._onSlideEnter === 'function') window._onSlideEnter(idx);
  }


  /* ===================================================================
   *  6.  TERMINAL ANIMATION ENGINE
   * =================================================================== */
  var _anim = { timeouts: [], paused: false, lines: [], rendered: 0, el: null, btnId: null, running: false, syncTimeouts: [] };

  function _renderLine(l) {
    var div = document.createElement('div');
    if (l.type === 'prompt')       div.innerHTML = '<span class="prompt">$ </span><span class="cmd">' + l.text + '</span>';
    else if (l.type === 'comment') div.innerHTML = '<span class="info"># ' + l.text + '</span>';
    else                           div.innerHTML = '<span class="' + (l.cls || 'output') + '">' + l.text + '</span>';
    _anim.el.appendChild(div);
    _anim.el.scrollTop = _anim.el.scrollHeight;
  }

  function _scheduleLines(startIdx) {
    var delay = 0;
    for (var i = startIdx; i < _anim.lines.length; i++) {
      delay += _anim.lines[i].pause || 500;
      (function (idx, d) {
        var t = setTimeout(function () { if (_anim.paused) return; _renderLine(_anim.lines[idx]); _anim.rendered = idx + 1 }, d);
        _anim.timeouts.push(t);
      })(i, delay);
    }
    var t = setTimeout(function () { if (_anim.paused) return; _animDone() }, delay + 500);
    _anim.timeouts.push(t);
  }

  function _animDone() {
    _anim.running = false;
    if (_anim.btnId) { var b = document.getElementById(_anim.btnId); if (b) b.disabled = false }
    var c = document.getElementById('anim-controls'); if (c) c.classList.remove('show');
  }

  function _cancelTimers() {
    _anim.timeouts.forEach(clearTimeout); _anim.timeouts = [];
    _anim.syncTimeouts.forEach(clearTimeout); _anim.syncTimeouts = [];
  }

  function typeInTerminal(id, lines, btnId) {
    skipAnim();
    _anim.el = document.getElementById(id); if (!_anim.el) return;
    _anim.el.innerHTML = '';
    _anim.btnId = btnId || null;
    _anim.lines = lines; _anim.rendered = 0; _anim.paused = false; _anim.running = true; _anim.syncTimeouts = [];
    if (btnId) { var b = document.getElementById(btnId); if (b) b.disabled = true }
    var c = document.getElementById('anim-controls'); if (c) c.classList.add('show');
    var pb = document.getElementById('btn-pause');
    if (pb) { pb.textContent = '⏸ 一時停止'; pb.classList.remove('paused') }
    _scheduleLines(0);
  }

  function skipAnim() {
    if (!_anim.running) return;
    _cancelTimers();
    for (var i = _anim.rendered; i < _anim.lines.length; i++) _renderLine(_anim.lines[i]);
    _anim.rendered = _anim.lines.length; _anim.paused = false;
    _animDone();
  }

  function togglePause() {
    if (!_anim.running) return;
    var pb = document.getElementById('btn-pause');
    if (_anim.paused) {
      _anim.paused = false;
      if (pb) { pb.textContent = '⏸ 一時停止'; pb.classList.remove('paused') }
      _scheduleLines(_anim.rendered);
    } else {
      _anim.paused = true; _cancelTimers();
      if (pb) { pb.textContent = '▶ 再開'; pb.classList.add('paused') }
    }
  }

  function tsSyncAction(delayMs, fn) {
    var t = setTimeout(function () { if (!_anim.paused) fn() }, delayMs);
    _anim.syncTimeouts.push(t);
  }

  function resetTerm(id) { var el = document.getElementById(id); if (el) el.innerHTML = '' }

  /* Expose globally */
  window.typeInTerminal = typeInTerminal;
  window.skipAnim       = skipAnim;
  window.togglePause    = togglePause;
  window.tsSyncAction   = tsSyncAction;
  window.resetTerm      = resetTerm;


  /* ===================================================================
   *  7.  TOOLBAR + JUMP GRID + GALLERY
   * =================================================================== */
  function initToolbar() {
    function getCurIdx() {
      for (var i = 0; i < slides.length; i++) if (slides[i].classList.contains('active')) return i;
      return 0;
    }
    function getTitle(el) { var h = el.querySelector('h1,h2,h3,h4'); return h ? h.textContent.trim() : '' }

    /* Hide old nav-hint */
    var oldHint = document.querySelector('.nav-hint');
    if (oldHint) oldHint.style.display = 'none';

    /* Toolbar */
    var toolbar = document.createElement('div');
    toolbar.className = '_ts-toolbar';
    toolbar.innerHTML = '<button class="_ts-btn _ts-counter-btn">1 / ' + slides.length + '</button><button class="_ts-btn _ts-grid-btn" title="全体表示 (Gallery)">&#9638;</button>';
    document.body.appendChild(toolbar);
    var counterBtn = toolbar.querySelector('._ts-counter-btn');
    var gridBtn    = toolbar.querySelector('._ts-grid-btn');

    /* Jump Popup */
    var jumpEl   = document.createElement('div'); jumpEl.className = '_ts-jump';
    var jumpGrid = document.createElement('div'); jumpGrid.className = '_ts-jump-grid';
    for (var i = 0; i < slides.length; i++) {
      var item = document.createElement('div');
      item.className = '_ts-jump-item'; item.textContent = i + 1; item.dataset.idx = i;
      item.addEventListener('click', function () { go(parseInt(this.dataset.idx)); closeJump(); updateUI() });
      jumpGrid.appendChild(item);
    }
    jumpEl.appendChild(jumpGrid);
    document.body.appendChild(jumpEl);

    var jumpOpen = false;
    function toggleJump() { jumpOpen = !jumpOpen; if (jumpOpen) { jumpEl.classList.add('show'); counterBtn.classList.add('_ts-active'); highlightJumpCurrent() } else closeJump() }
    function closeJump()  { jumpOpen = false; jumpEl.classList.remove('show'); counterBtn.classList.remove('_ts-active') }
    function highlightJumpCurrent() { var c = getCurIdx(); var items = jumpGrid.querySelectorAll('._ts-jump-item'); for (var j = 0; j < items.length; j++) items[j].classList.toggle('_ts-current', j === c) }
    counterBtn.addEventListener('click', function (e) { e.stopPropagation(); closeGallery(); toggleJump() });

    /* Gallery Overlay */
    var gallery = document.createElement('div'); gallery.className = '_ts-gallery-overlay';
    var gHTML = '<div class="_ts-gallery-header"><span class="_ts-gallery-title">全スライド一覧</span><button class="_ts-gallery-close">&times;</button></div><div class="_ts-gallery-grid">';
    for (var i = 0; i < slides.length; i++) {
      var title = getTitle(slides[i]) || 'Slide ' + (i + 1);
      gHTML += '<div class="_ts-gallery-card" data-idx="' + i + '"><span class="_ts-gallery-num">' + (i + 1) + '</span><div class="_ts-gallery-preview"><div class="_ts-gallery-preview-inner" data-slide-idx="' + i + '"></div></div><div class="_ts-gallery-info"><h4>' + title + '</h4><p>' + (i + 1) + ' / ' + slides.length + '</p></div></div>';
    }
    gHTML += '</div>';
    gallery.innerHTML = gHTML;
    document.body.appendChild(gallery);

    /* Clone slide contents into preview cards */
    var previews = gallery.querySelectorAll('._ts-gallery-preview-inner');
    for (var i = 0; i < previews.length; i++) {
      var idx = parseInt(previews[i].dataset.slideIdx);
      var clone = slides[idx].cloneNode(true);
      clone.style.display = 'flex'; clone.style.position = 'static'; clone.style.animation = 'none';
      clone.classList.remove('out-left', 'out-right'); clone.classList.add('active');
      previews[i].appendChild(clone);
    }

    var cards = gallery.querySelectorAll('._ts-gallery-card');
    for (var i = 0; i < cards.length; i++) {
      cards[i].addEventListener('click', function () { go(parseInt(this.dataset.idx)); closeGallery(); updateUI() });
    }

    var galleryOpen = false;
    function openGallery()  { galleryOpen = true; updateUI(); gallery.classList.add('show'); gridBtn.classList.add('_ts-active') }
    function closeGallery() { galleryOpen = false; gallery.classList.remove('show'); gridBtn.classList.remove('_ts-active') }
    function highlightGalleryCard() { var c = getCurIdx(); for (var j = 0; j < cards.length; j++) cards[j].classList.toggle('_ts-current-card', j === c) }
    gallery.querySelector('._ts-gallery-close').addEventListener('click', function (e) { e.stopPropagation(); closeGallery() });
    gridBtn.addEventListener('click', function (e) { e.stopPropagation(); closeJump(); if (galleryOpen) closeGallery(); else openGallery() });

    /* Escape key */
    document.addEventListener('keydown', function (e) {
      if (e.key === 'Escape') {
        if (galleryOpen)  { closeGallery(); e.stopPropagation(); e.preventDefault() }
        else if (jumpOpen) { closeJump();   e.stopPropagation(); e.preventDefault() }
      }
    }, true);

    /* Close on outside click */
    document.addEventListener('click', function (e) {
      if (jumpOpen && !jumpEl.contains(e.target) && !counterBtn.contains(e.target)) closeJump();
    });

    /* Sync UI */
    function updateUI() {
      var c = getCurIdx();
      counterBtn.textContent = (c + 1) + ' / ' + slides.length;
      if (jumpOpen)    highlightJumpCurrent();
      if (galleryOpen) highlightGalleryCard();
    }
    var lastIdx = getCurIdx();
    setInterval(function () { var c = getCurIdx(); if (c !== lastIdx) { lastIdx = c; updateUI() } }, 200);
    updateUI();
  }


  /* ===================================================================
   *  8.  BOOT
   * =================================================================== */
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }

})();
