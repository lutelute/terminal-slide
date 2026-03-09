/// Self-contained JS+CSS snippet injected into HTML presentations.
/// Provides slide jump, gallery view, and click-to-navigate features.
/// Takes over keyboard navigation to ensure state consistency.
pub const SNIPPET: &str = r##"
<!-- terminal-slide navigation -->
<style>
._ts-toolbar{position:fixed;bottom:1rem;left:1.5rem;display:flex;gap:.5rem;z-index:99999;font-family:-apple-system,BlinkMacSystemFont,sans-serif}
._ts-btn{background:rgba(30,30,48,.85);color:#888;border:1px solid #333;padding:.4rem .8rem;border-radius:6px;font-size:.85rem;cursor:pointer;transition:all .2s;backdrop-filter:blur(8px);-webkit-backdrop-filter:blur(8px);user-select:none}
._ts-btn:hover{background:rgba(0,210,255,.15);color:#00d2ff;border-color:#00d2ff55}
._ts-btn._ts-active{color:#00d2ff;border-color:#00d2ff55}

._ts-jump{position:fixed;bottom:3.5rem;left:1.5rem;background:rgba(20,20,35,.95);border:1px solid #333;border-radius:10px;padding:.8rem;display:none;z-index:99999;backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px);max-height:60vh;overflow-y:auto}
._ts-jump.show{display:block;animation:_tsFadeUp .2s ease-out}
._ts-jump-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(42px,1fr));gap:.4rem;min-width:200px}
._ts-jump-item{background:rgba(30,30,48,.9);color:#aaa;border:1px solid #2a2a4a;border-radius:6px;padding:.4rem;text-align:center;cursor:pointer;font-size:.85rem;transition:all .15s}
._ts-jump-item:hover{background:rgba(0,210,255,.2);color:#fff;border-color:#00d2ff}
._ts-jump-item._ts-current{background:rgba(0,210,255,.25);color:#00d2ff;border-color:#00d2ff;font-weight:bold}

._ts-gallery-overlay{position:fixed;inset:0;background:rgba(5,5,15,.92);z-index:99998;display:none;overflow-y:auto;backdrop-filter:blur(8px);-webkit-backdrop-filter:blur(8px)}
._ts-gallery-overlay.show{display:block;animation:_tsFadeIn .25s ease-out}
._ts-gallery-header{position:sticky;top:0;display:flex;justify-content:space-between;align-items:center;padding:1.2rem 2rem;background:rgba(5,5,15,.8);backdrop-filter:blur(8px);z-index:1}
._ts-gallery-title{color:#00d2ff;font-size:1.2rem;font-weight:600}
._ts-gallery-close{background:none;border:1px solid #333;color:#888;width:36px;height:36px;border-radius:8px;font-size:1.2rem;cursor:pointer;display:flex;align-items:center;justify-content:center;transition:all .2s}
._ts-gallery-close:hover{color:#ff5370;border-color:#ff5370}
._ts-gallery-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:1.2rem;padding:1.2rem 2rem 2rem}
._ts-gallery-card{background:rgba(26,26,46,.9);border:1px solid #2a2a4a;border-radius:10px;cursor:pointer;transition:all .2s;overflow:hidden;position:relative}
._ts-gallery-card:hover{border-color:#00d2ff;transform:translateY(-3px);box-shadow:0 8px 24px rgba(0,210,255,.12)}
._ts-gallery-card._ts-current-card{border-color:#00d2ff;box-shadow:0 0 12px rgba(0,210,255,.2)}
._ts-gallery-num{position:absolute;top:.6rem;left:.8rem;background:rgba(0,210,255,.2);color:#00d2ff;font-size:.75rem;font-weight:700;padding:.15rem .5rem;border-radius:4px;z-index:1}
._ts-gallery-preview{height:160px;overflow:hidden;position:relative;background:#0f0f1a}
._ts-gallery-preview-inner{transform:scale(0.28);transform-origin:top left;width:357%;height:357%;pointer-events:none;position:absolute;top:0;left:0}
._ts-gallery-info{padding:.8rem 1rem}
._ts-gallery-info h4{color:#e0e0e0;font-size:.9rem;margin:0;font-weight:500;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
._ts-gallery-info p{color:#666;font-size:.75rem;margin:.2rem 0 0}

._ts-export-menu{position:fixed;bottom:3.5rem;left:1.5rem;background:rgba(20,20,35,.95);border:1px solid #333;border-radius:10px;padding:.6rem;display:none;z-index:99999;backdrop-filter:blur(12px);-webkit-backdrop-filter:blur(12px)}
._ts-export-menu.show{display:block;animation:_tsFadeUp .2s ease-out}
._ts-export-item{display:flex;align-items:center;gap:.6rem;padding:.5rem .8rem;border-radius:6px;cursor:pointer;color:#aaa;font-size:.85rem;transition:all .15s;white-space:nowrap}
._ts-export-item:hover{background:rgba(0,210,255,.15);color:#fff}
._ts-export-item span{color:#666;font-size:.75rem}
._ts-export-item._ts-loading{opacity:.5;pointer-events:none}

@keyframes _tsFadeUp{from{opacity:0;transform:translateY(8px)}to{opacity:1;transform:translateY(0)}}
@keyframes _tsFadeIn{from{opacity:0}to{opacity:1}}
</style>
<script>
(function(){
  var slides=document.querySelectorAll('.slide');
  if(slides.length<2)return;

  // === Core navigation (takes over from user's JS) ===
  var _tsCur=0;
  // Find initial active slide
  for(var i=0;i<slides.length;i++){if(slides[i].classList.contains('active')){_tsCur=i;break}}

  // Clean ALL slides: remove active and any transition classes
  function cleanAllSlides(){
    for(var i=0;i<slides.length;i++){
      slides[i].classList.remove('active','out-left','out-right');
      slides[i].style.animation='none';
    }
  }

  function jumpTo(idx){
    if(idx<0||idx>=slides.length)return;
    idx=Math.max(0,Math.min(idx,slides.length-1));
    if(idx===_tsCur)return;
    cleanAllSlides();
    slides[idx].offsetHeight; // force reflow
    slides[idx].style.animation='';
    slides[idx].classList.add('active');
    _tsCur=idx;
    updateUI(idx);
    // Call user's onSlideEnter hook if defined (for charts, animations, etc.)
    if(typeof window.onSlideEnter==='function'){try{window.onSlideEnter(idx)}catch(e){}}
    // Dispatch custom event for any other listeners
    document.dispatchEvent(new CustomEvent('ts:slidechange',{detail:{index:idx,total:slides.length}}));
  }

  // Intercept ALL keyboard navigation at capture phase to prevent
  // the user's JS from running its own stale navigation
  document.addEventListener('keydown',function(e){
    if(e.target.tagName==='TEXTAREA'||e.target.tagName==='INPUT')return;
    if(galleryOpen||jumpOpen){
      if(e.key==='Escape'){
        if(galleryOpen)closeGallery();
        else if(jumpOpen)closeJump();
        e.stopImmediatePropagation();e.preventDefault();
      }
      return;
    }
    var handled=true;
    if(['ArrowRight','l','j','n',' '].indexOf(e.key)>=0){jumpTo(_tsCur+1)}
    else if(['ArrowLeft','h','k','p'].indexOf(e.key)>=0){jumpTo(_tsCur-1)}
    else if(e.key==='g'){jumpTo(0)}
    else if(e.key==='G'){jumpTo(slides.length-1)}
    else{handled=false}
    if(handled){e.stopImmediatePropagation();e.preventDefault()}
  },true);

  function getTitle(el){
    var h=el.querySelector('h1,h2,h3,h4');
    return h?h.textContent.trim():'';
  }

  // --- Toolbar ---
  var toolbar=document.createElement('div');
  toolbar.className='_ts-toolbar';
  toolbar.innerHTML='<button class="_ts-btn _ts-counter-btn">1 / '+slides.length+'</button><button class="_ts-btn _ts-grid-btn" title="Gallery view">&#9638;</button><button class="_ts-btn _ts-export-btn" title="Export">&#8615;</button>';
  document.body.appendChild(toolbar);
  var counterBtn=toolbar.querySelector('._ts-counter-btn');
  var gridBtn=toolbar.querySelector('._ts-grid-btn');

  // --- Jump Popup ---
  var jump=document.createElement('div');
  jump.className='_ts-jump';
  var jumpGrid=document.createElement('div');
  jumpGrid.className='_ts-jump-grid';
  for(var i=0;i<slides.length;i++){
    var item=document.createElement('div');
    item.className='_ts-jump-item';
    item.textContent=i+1;
    item.dataset.idx=i;
    item.addEventListener('click',function(e){
      var idx=parseInt(this.dataset.idx);
      jumpTo(idx);
      closeJump();
    });
    jumpGrid.appendChild(item);
  }
  jump.appendChild(jumpGrid);
  document.body.appendChild(jump);

  var jumpOpen=false;
  function toggleJump(){
    jumpOpen=!jumpOpen;
    if(jumpOpen){
      jump.classList.add('show');
      counterBtn.classList.add('_ts-active');
      highlightJumpCurrent();
    }else{closeJump()}
  }
  function closeJump(){
    jumpOpen=false;
    jump.classList.remove('show');
    counterBtn.classList.remove('_ts-active');
  }
  function highlightJumpCurrent(){
    var items=jumpGrid.querySelectorAll('._ts-jump-item');
    for(var i=0;i<items.length;i++){
      items[i].classList.toggle('_ts-current',i===_tsCur);
    }
  }
  counterBtn.addEventListener('click',function(e){e.stopPropagation();closeGallery();toggleJump()});

  // --- Gallery Overlay ---
  var gallery=document.createElement('div');
  gallery.className='_ts-gallery-overlay';
  var galleryHTML='<div class="_ts-gallery-header"><span class="_ts-gallery-title">All Slides</span><button class="_ts-gallery-close">&times;</button></div><div class="_ts-gallery-grid">';
  for(var i=0;i<slides.length;i++){
    var title=getTitle(slides[i])||'Slide '+(i+1);
    galleryHTML+='<div class="_ts-gallery-card" data-idx="'+i+'"><span class="_ts-gallery-num">'+(i+1)+'</span><div class="_ts-gallery-preview"><div class="_ts-gallery-preview-inner" data-slide-idx="'+i+'"></div></div><div class="_ts-gallery-info"><h4>'+title+'</h4><p>'+(i+1)+' / '+slides.length+'</p></div></div>';
  }
  galleryHTML+='</div>';
  gallery.innerHTML=galleryHTML;
  document.body.appendChild(gallery);

  // Clone slide contents into previews
  var previews=gallery.querySelectorAll('._ts-gallery-preview-inner');
  for(var i=0;i<previews.length;i++){
    var idx=parseInt(previews[i].dataset.slideIdx);
    var clone=slides[idx].cloneNode(true);
    clone.style.display='flex';
    clone.style.position='static';
    clone.style.animation='none';
    clone.classList.remove('out-left','out-right');
    clone.classList.add('active');
    previews[i].appendChild(clone);
  }

  // Gallery card click
  var cards=gallery.querySelectorAll('._ts-gallery-card');
  for(var i=0;i<cards.length;i++){
    cards[i].addEventListener('click',function(){
      var idx=parseInt(this.dataset.idx);
      closeGallery();
      jumpTo(idx);
    });
  }

  var galleryOpen=false;
  function openGallery(){
    galleryOpen=true;
    highlightGalleryCard();
    gallery.classList.add('show');
    gridBtn.classList.add('_ts-active');
  }
  function closeGallery(){
    galleryOpen=false;
    gallery.classList.remove('show');
    gridBtn.classList.remove('_ts-active');
  }
  function highlightGalleryCard(){
    for(var i=0;i<cards.length;i++){
      cards[i].classList.toggle('_ts-current-card',i===_tsCur);
    }
  }
  gallery.querySelector('._ts-gallery-close').addEventListener('click',function(e){e.stopPropagation();closeGallery()});
  gridBtn.addEventListener('click',function(e){e.stopPropagation();closeJump();if(galleryOpen)closeGallery();else openGallery()});

  // --- Export Menu ---
  var exportBtn=toolbar.querySelector('._ts-export-btn');
  var exportMenu=document.createElement('div');
  exportMenu.className='_ts-export-menu';
  exportMenu.innerHTML='<div class="_ts-export-item" data-fmt="pdf">PDF <span>via Chrome/pandoc</span></div><div class="_ts-export-item" data-fmt="pptx">PPTX <span>via pandoc</span></div><div class="_ts-export-item" data-fmt="md">Markdown <span>via pandoc</span></div>';
  document.body.appendChild(exportMenu);
  var exportOpen=false;
  function toggleExport(){
    exportOpen=!exportOpen;
    if(exportOpen){exportMenu.classList.add('show');exportBtn.classList.add('_ts-active');closeJump();closeGallery()}
    else{closeExport()}
  }
  function closeExport(){exportOpen=false;exportMenu.classList.remove('show');exportBtn.classList.remove('_ts-active')}
  exportBtn.addEventListener('click',function(e){e.stopPropagation();toggleExport()});
  exportMenu.querySelectorAll('._ts-export-item').forEach(function(item){
    item.addEventListener('click',function(e){
      e.stopPropagation();
      var fmt=this.dataset.fmt;
      var el=this;
      el.classList.add('_ts-loading');
      el.textContent='Exporting...';
      fetch('/_api/export/'+fmt).then(function(r){
        if(!r.ok)return r.json().then(function(j){throw new Error(j.error||'Export failed')});
        var cd=r.headers.get('content-disposition')||'';
        var fn_match=cd.match(/filename="(.+?)"/);
        var filename=fn_match?fn_match[1]:'export.'+fmt;
        return r.blob().then(function(b){return{blob:b,filename:filename}});
      }).then(function(d){
        var a=document.createElement('a');
        a.href=URL.createObjectURL(d.blob);
        a.download=d.filename;
        a.click();
        URL.revokeObjectURL(a.href);
        closeExport();
      }).catch(function(err){
        alert('Export error: '+err.message);
      }).finally(function(){
        el.classList.remove('_ts-loading');
        el.innerHTML={'pdf':'PDF <span>via Chrome/pandoc</span>','pptx':'PPTX <span>via pandoc</span>','md':'Markdown <span>via pandoc</span>'}[fmt]||fmt;
      });
    });
  });

  // Close on outside click
  document.addEventListener('click',function(e){
    if(jumpOpen&&!jump.contains(e.target)&&!counterBtn.contains(e.target))closeJump();
    if(exportOpen&&!exportMenu.contains(e.target)&&!exportBtn.contains(e.target))closeExport();
  });

  // --- Keep UI in sync ---
  function updateUI(idx){
    counterBtn.textContent=(idx+1)+' / '+slides.length;
    if(jumpOpen)highlightJumpCurrent();
    if(galleryOpen)highlightGalleryCard();
    var prog=document.querySelector('.progress');
    if(prog)prog.textContent=(idx+1)+' / '+slides.length;
  }

  // Initial sync
  updateUI(_tsCur);
})();
</script>
"##;
