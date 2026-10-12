//! The files of the Svelte showcase module ([`crate::showcase`]): its manifest, its panel as
//! GridCraft serves it, and the Svelte source that panel is built from.
//!
//! **Generated: not written by hand.** `panel/app.js` and `panel/app.css` are what
//! `npm run build` makes of `source/` (which is among these files) with
//! svelte 5.57.2, vite 8.3.4 and @sveltejs/vite-plugin-svelte 7.3.1.
//! To change the panel, change the source, build it, and put all of it back here together.
//!
//! `panel/app.js` holds parts of Svelte, clsx and esm-env, each under the MIT licence: their
//! notices are in `source/THIRD-PARTY.md`, which goes into every module made from these files.
//!
//! This is the fork's second exception to upstream's "Rust only" rule, after the panel's
//! `spreadsheet.js`: the owner asked for a Svelte showcase (2026-10-12). Like that script it is
//! text inside Rust source, and the repository holds no JavaScript file.

/// Every file of the module: its path inside the module, with `/` between folders, and its text.
#[rustfmt::skip]
pub(crate) const FILES: &[(&str, &str)] = &[
    ("module.toml", r#"# A GridCraft module: the Svelte showcase. See docs/decisions-aniefork.md (ADR-0001).
id = "svelte-showcase"
name = "Svelte Showcase"
version = "0.1.0"
author = "GridCraft"
description = "A Svelte app as a panel: it follows the selection, and writes to the sheet."

# What this module asks to be allowed. The user sees this list before trusting it.
permissions = ["read", "write"]

[panel]
entry = "panel/index.html"
"#),
    ("panel/index.html", r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Svelte Showcase</title>
    <link rel="stylesheet" href="app.css" />
  </head>
  <body>
    <div id="app"></div>
    <!-- GridCraft serves spreadsheet.js beside the page: it gives the page its `spreadsheet` object. -->
    <script src="spreadsheet.js"></script>
    <!-- Built from ../source with `npm run build`. -->
    <script src="app.js"></script>
  </body>
</html>
"#),
    ("panel/app.js", r#"(function(){var e=Array.isArray,t=Array.prototype.indexOf,n=Array.prototype.includes,r=Array.from,i=Object.defineProperty,a=Object.getOwnPropertyDescriptor,o=Object.getOwnPropertyDescriptors,s=Object.prototype,c=Array.prototype,l=Object.getPrototypeOf,u=Object.isExtensible,d=()=>{};function f(e){for(var t=0;t<e.length;t++)e[t]()}function p(){var e,t;return{promise:new Promise((n,r)=>{e=n,t=r}),resolve:e,reject:t}}function m(e){"@babel/helpers - typeof";return m=typeof Symbol==`function`&&typeof Symbol.iterator==`symbol`?function(e){return typeof e}:function(e){return e&&typeof Symbol==`function`&&e.constructor===Symbol&&e!==Symbol.prototype?`symbol`:typeof e},m(e)}function h(e,t){if(m(e)!=`object`||!e)return e;var n=e[Symbol.toPrimitive];if(n!==void 0){var r=n.call(e,t||`default`);if(m(r)!=`object`)return r;throw TypeError(`@@toPrimitive must return a primitive value.`)}return(t===`string`?String:Number)(e)}function ee(e){var t=h(e,`string`);return m(t)==`symbol`?t:t+``}function g(e,t,n){return(t=ee(t))in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}var _=1024,v=2048,y=4096,b=8192,x=16384,te=32768,ne=1<<25,re=65536,ie=1<<19,ae=1<<20,oe=1<<25,se=1<<21,ce=1<<22,le=1<<23,ue=Symbol(`$state`),de=Symbol(`component`),fe=Symbol(``),pe=Symbol(`attributes`),me=Symbol(`class`),he=Symbol(`style`),ge=Symbol(`text`),_e=Symbol(`form reset`),ve=new class extends Error{constructor(...e){super(...e),g(this,`name`,`StaleReactionError`),g(this,`message`,"The reaction that called `getAbortSignal()` was re-run or destroyed")}},ye=!!globalThis.document?.contentType&&globalThis.document.contentType.includes(`xml`),be={},S=Symbol(`uninitialized`);function xe(){console.warn(`https://svelte.dev/e/derived_inert`)}function Se(e){console.warn(`https://svelte.dev/e/hydration_mismatch`)}function Ce(){console.warn(`https://svelte.dev/e/svelte_boundary_reset_noop`)}var C=!1;function we(e){C=e}var w;function T(e){if(e===null)throw Se(),be;return w=e}function Te(){return T(Dn(w))}function Ee(e){if(C){if(Dn(w)!==null)throw Se(),be;w=e}}function De(e=1){if(C){for(var t=e,n=w;t--;)n=Dn(n);w=n}}function Oe(e=!0){for(var t=0,n=w;;){if(n.nodeType===8){var r=n.data;if(r===`]`){if(t===0)return n;--t}else(r===`[`||r===`[!`||r[0]===`[`&&!isNaN(Number(r.slice(1))))&&(t+=1)}var i=Dn(n);e&&n.remove(),n=i}}function ke(e){if(!e||e.nodeType!==8)throw Se(),be;return e.data}function Ae(e){return e===this.v}function je(e,t){return e==e?e!==t||typeof e==`object`&&!!e||typeof e==`function`:t==t}function Me(e){return!je(e,this.v)}function Ne(){throw Error(`https://svelte.dev/e/async_derived_orphan`)}function Pe(e,t,n){throw Error(`https://svelte.dev/e/each_key_duplicate`)}function Fe(e){throw Error(`https://svelte.dev/e/effect_in_teardown`)}function Ie(){throw Error(`https://svelte.dev/e/effect_in_unowned_derived`)}function Le(e){throw Error(`https://svelte.dev/e/effect_orphan`)}function Re(){throw Error(`https://svelte.dev/e/effect_update_depth_exceeded`)}function ze(){throw Error(`https://svelte.dev/e/state_descriptors_fixed`)}function Be(){throw Error(`https://svelte.dev/e/state_prototype_fixed`)}function Ve(){throw Error(`https://svelte.dev/e/state_unsafe_mutation`)}function He(){throw Error(`https://svelte.dev/e/svelte_boundary_reset_onerror`)}var E=null;function Ue(e){E=e}function We(e,t=!1,n){E={p:E,i:!1,c:null,e:null,s:e,x:null,r:W,l:null}}function Ge(e){var t=E,n=t.e;if(n!==null){t.e=null;for(var r of n)Un(r)}return e!==void 0&&(t.x=e),t.i=!0,E=t.p,Ke(e)}function Ke(e={}){return i(e,de,{value:!0}),e}function qe(){return!0}var Je=[];function Ye(){var e=Je;Je=[],f(e)}function Xe(e){if(Je.length===0&&!Tt){var t=Je;queueMicrotask(()=>{t===Je&&Ye()})}Je.push(e)}function Ze(){for(;Je.length>0;)Ye()}var Qe=~(v|y|_);function D(e,t){e.f=e.f&Qe|t}function $e(e){e.f&512||e.deps===null?D(e,_):D(e,y)}function et(e,t,n){e.f&2048?t.add(e):e.f&4096&&n.add(e),D(e,_)}var tt=!1;function nt(){tt||(tt=!0,document.addEventListener(`reset`,e=>{Promise.resolve().then(()=>{if(!e.defaultPrevented)for(let t of e.target.elements)t[_e]?.()})},{capture:!0}))}function rt(e){var t=V,n=W;U(null),cr(null);try{return e()}finally{U(t),cr(n)}}function it(e,t,n,r=n){e.addEventListener(t,()=>rt(n));let i=e[_e];e[_e]=i?()=>{i(),r(!0)}:()=>r(!0),nt()}function at(e,t,n,r){let i=qe()?lt:pt;var a=e.filter(e=>!e.settled),o=t.map(i);if(n.length===0&&a.length===0){r(o);return}var s=W,c=ot(),l=a.length===1?a[0].promise:a.length>1?Promise.all(a.map(e=>e.promise)):null;function u(e){if(!(s.f&16384)){c();try{r([...o,...e])}catch(e){In(e,s)}st()}}var d=ct();if(n.length===0){l.then(()=>u([])).finally(d);return}function f(){Promise.all(n.map(e=>dt(e))).then(u).catch(e=>In(e,s)).finally(d)}l?l.then(()=>{s.f&16384?d():(c(),f(),st())}):f()}function ot(){var e=W,t=V,n=E,r=M;return function(i=!0){cr(e),U(t),Ue(n),i&&!(e.f&16384)&&(r?.activate(),r?.apply())}}function st(e=!0){cr(null),U(null),Ue(null),e&&M?.deactivate()}function ct(){var e=W,t=e.b,n=M,r=!!t?.is_rendered();return t?.update_pending_count(1,n),n.increment(r,e),()=>{t?.update_pending_count(-1,n),n.decrement(r,e)}}function lt(e){var t=2|v;return W!==null&&(W.f|=ie),{ctx:E,deps:null,effects:null,equals:Ae,f:t,fn:e,reactions:null,rv:0,v:S,wv:0,parent:W,ac:null}}var ut=Symbol(`obsolete`);function dt(e,t,n){let r=W;r===null&&Ne();var i=void 0,a=fn(S),o=!V,s=new Set;return Gn(()=>{var t=W,n=p();i=n.promise;try{Promise.resolve(e()).then(n.resolve,e=>{e!==ve&&n.reject(e)}).finally(st)}catch(e){n.reject(e),st()}var c=M;if(o){if(t.f&32768)var l=ct();if(r.b?.is_rendered())c.async_deriveds.get(t)?.reject(ut);else for(let e of s.values())e.reject(ut);s.add(n),c.async_deriveds.set(t,n)}let u=(e,t=void 0)=>{l?.(),s.delete(n),t!==ut&&(c.activate(),t?(a.f|=le,gn(a,t)):(a.f&8388608&&(a.f^=le),gn(a,e)),c.deactivate())};n.promise.then(u,e=>u(null,e||`unknown`))}),Vn(()=>{for(let e of s)e.reject(ut)}),new Promise(e=>{function t(n){function r(){n===i?e(a):t(i)}n.then(r,r)}t(i)})}function ft(e){let t=lt(e);return lr(t),t}function pt(e){let t=lt(e);return t.equals=Me,t}function mt(e){var t=e.effects;if(t!==null){e.effects=null;for(var n=0;n<t.length;n+=1)B(t[n])}}function ht(e){var t,n=W,r=e.parent;if(!or&&r!==null&&e.v!==S&&r.f&24576)return xe(),e.v;cr(r);try{mt(e),t=vr(e)}finally{cr(n)}return t}function gt(e){var t=ht(e);!e.equals(t)&&(e.wv=hr(),(!M?.is_fork||e.deps===null)&&(M===null?e.v=t:(M.capture(e,t,!0),Ct?.capture(e,t,!0)),e.deps===null))?D(e,_):or||(N===null?$e(e):(Bn()||M?.is_fork)&&N.set(e,t))}function _t(e){if(e.effects!==null)for(let t of e.effects)(t.teardown||t.ac)&&(t.teardown?.(),t.ac!==null&&rt(()=>{t.ac.abort(ve),t.ac=null}),t.fn!==null&&(t.teardown=d),xr(t,0),Xn(t))}function vt(e){if(e.effects!==null)for(let t of e.effects)t.teardown&&t.fn!==null&&Sr(t)}function yt(e,t){if(t.has(e))throw TypeError(`Cannot initialize the same private elements twice on an object`)}function bt(e,t){yt(e,t),t.add(e)}function O(e,t,n){yt(e,t),t.set(e,n)}function k(e,t,n){if(typeof e==`function`?e===t:e.has(t))return arguments.length<3?t:n;throw TypeError(`Private element is not present on this object`)}function A(e,t,n){return e.set(k(e,t),n),n}function j(e,t){return e.get(k(e,t))}var xt,St=null,M=null,Ct=null,N=null,wt=null,Tt=!1,Et=!1,Dt=null,Ot=null,kt=0,At=1,jt=new WeakMap,Mt=new WeakMap,Nt=new WeakMap,Pt=new WeakMap,Ft=new WeakMap,It=new WeakMap,Lt=new WeakMap,Rt=new WeakMap,zt=new WeakMap,Bt=new WeakMap,Vt=new WeakMap,Ht=new WeakMap,Ut=new WeakMap,Wt=new WeakMap,Gt=new WeakMap,P=new WeakSet,Kt=class e{constructor(){bt(this,P),g(this,`id`,At++),O(this,jt,!1),g(this,`linked`,!0),O(this,Mt,null),O(this,Nt,null),g(this,`async_deriveds`,new Map),g(this,`current`,new Map),g(this,`previous`,new Map),O(this,Pt,new Set),O(this,Ft,new Set),O(this,It,0),O(this,Lt,new Map),O(this,Rt,null),O(this,zt,[]),O(this,Bt,[]),O(this,Vt,new Set),O(this,Ht,new Set),O(this,Ut,new Map),O(this,Wt,new Set),g(this,`is_fork`,!1),O(this,Gt,!1),St===null?St=this:(A(Nt,St,this),A(Mt,this,St)),St=this}skip_effect(e){j(Ut,this).has(e)||j(Ut,this).set(e,{d:[],m:[]}),j(Wt,this).delete(e)}unskip_effect(e,t=e=>this.schedule(e)){var n=j(Ut,this).get(e);if(n){j(Ut,this).delete(e);for(var r of n.d)D(r,v),t(r);for(r of n.m)D(r,y),t(r)}j(Wt,this).add(e)}capture(e,t,n=!1){e.v!==S&&!this.previous.has(e)&&this.previous.set(e,e.v),e.f&8388608||(this.current.set(e,[t,n]),N?.set(e,t)),this.is_fork||(e.v=t)}activate(){M=this}deactivate(){M=null,N=null}flush(){try{Et=!0,M=this,k(P,this,Yt).call(this)}finally{kt=0,wt=null,Dt=null,Ot=null,Et=!1,M=null,N=null,un.clear()}}discard(){for(let e of j(Ft,this))e(this);j(Ft,this).clear();for(let e of this.async_deriveds.values())e.reject(ut);k(P,this,en).call(this),j(Rt,this)?.resolve()}register_created_effect(e){j(Bt,this).push(e)}increment(e,t){if(A(It,this,j(It,this)+1),e){let e=j(Lt,this).get(t)??0;j(Lt,this).set(t,e+1)}}decrement(e,t){if(A(It,this,j(It,this)-1),e){let e=j(Lt,this).get(t)??0;e===1?j(Lt,this).delete(t):j(Lt,this).set(t,e-1)}j(Gt,this)||(A(Gt,this,!0),Xe(()=>{A(Gt,this,!1),this.linked&&this.flush()}))}transfer_effects(e,t){for(let t of e)j(Vt,this).add(t);for(let e of t)j(Ht,this).add(e);e.clear(),t.clear()}oncommit(e){j(Pt,this).add(e)}ondiscard(e){j(Ft,this).add(e)}settled(){return(j(Rt,this)??A(Rt,this,p())).promise}static ensure(){if(M===null){let t=M=new e;!Et&&!Tt&&Xe(()=>{j(jt,t)||t.flush()})}return M}apply(){N=null}schedule(e){wt=e,e.b?.is_pending&&e.f&16777228&&!(e.f&32768)?e.b.defer_effect(e):j(zt,this).push(e)}};xt=Kt;function qt(){if(this.is_fork)return!0;for(let n of j(Lt,this).keys()){for(var e=n,t=!1;e.parent!==null;){if(j(Ut,this).has(e)){t=!0;break}e=e.parent}if(!t)return!0}return!1}function Jt(){var e=[];for(let i of j(zt,this))if(!(i.f&16384||!(i.f&6144))){for(var t=i,n=!1;t.parent!==null;){t=t.parent;var r=t.f;if(r&96){if(!(r&1024)){n=!0;break}t.f^=_}}n||e.push(t)}return A(zt,this,[]),e}function Yt(){A(jt,this,!0);for(let e of j(Vt,this))j(Ht,this).delete(e),D(e,v),this.schedule(e);for(let e of j(Ht,this))D(e,y),this.schedule(e);this.apply();for(var e=Dt=[],t=[],n=Ot=[];j(zt,this).length>0;){kt++>1e3&&(k(P,this,en).call(this),nn());for(let n of k(P,this,Jt).call(this))try{k(P,this,Xt).call(this,n,e,t)}catch(e){throw cn(n),k(P,this,qt).call(this)||this.discard(),e}}if(M=null,n.length>0){var r=xt.ensure();for(let e of n)r.schedule(e)}if(Dt=null,Ot=null,k(P,this,qt).call(this)){k(P,this,$t).call(this,t),k(P,this,$t).call(this,e);for(let[e,t]of j(Ut,this))sn(e,t);n.length>0&&k(P,M,Yt).call(M);return}let i=k(P,this,Zt).call(this);if(i)k(P,this,$t).call(this,t),k(P,this,$t).call(this,e),k(P,i,Qt).call(i,this);else{j(Vt,this).clear(),j(Ht,this).clear();for(let e of j(Pt,this))e(this);j(Pt,this).clear(),Ct=this,an(t),an(e),Ct=null,j(Rt,this)?.resolve();var a=M;if(j(It,this)===0&&(j(zt,this).length===0||a!==null)&&k(P,this,en).call(this),j(zt,this).length>0){if(a!==null){for(let e of j(zt,this))j(zt,a).push(e);A(zt,this,[])}else a=this}a!==null&&(un.clear(),k(P,a,Yt).call(a))}}function Xt(e,t,n){e.f^=_;for(var r=e.first;r!==null;){var i=r.f,a=!!(i&96);if(!(a&&i&1024||i&8192||j(Ut,this).has(r))&&r.fn!==null){a?r.f^=_:i&4?t.push(r):gr(r)&&(i&16&&j(Ht,this).add(r),Sr(r));var o=r.first;if(o!==null){r=o;continue}}for(;r!==null;){var s=r.next;if(s!==null){r=s;break}r=r.parent}}}function Zt(){for(var e=j(Mt,this);e!==null;){if(!e.is_fork){for(let[t,[,n]]of this.current)if(e.current.has(t)&&!n)return e}e=j(Mt,e)}return null}function Qt(e){for(let[t,n]of e.current)!this.previous.has(t)&&e.previous.has(t)&&this.previous.set(t,e.previous.get(t)),this.current.set(t,n);for(let[t,n]of e.async_deriveds){let e=this.async_deriveds.get(t);e&&n.promise.then(e.resolve).catch(e.reject)}e.async_deriveds.clear(),this.transfer_effects(j(Vt,e),j(Ht,e));let t=e=>{var n=e.reactions;if(n!==null&&!(e.f&2&&!(e.f&6144)))for(let e of n){var r=e.f;if(r&2)t(e);else{var i=e;r&4194320&&!this.async_deriveds.has(i)&&(j(Ht,this).delete(i),D(i,v),this.schedule(i))}}};for(let e of this.current.keys())t(e);this.oncommit(()=>e.discard()),k(P,e,en).call(e),M=this,k(P,this,Yt).call(this)}function $t(e){for(var t=0;t<e.length;t+=1)et(e[t],j(Vt,this),j(Ht,this))}function en(){if(this.linked){var e=j(Mt,this),t=j(Nt,this);e===null||A(Nt,e,t),t===null?St=e:A(Mt,t,e),this.linked=!1}}function tn(e){var t=Tt,n=Ct;Ct=null,Tt=!0;try{var r;for(e&&(tn(),r=e());;){if(Ze(),M===null)return r;M.flush()}}finally{Tt=t,Ct=n}}function nn(){try{Re()}catch(e){In(e,wt)}}var rn=null;function an(e){var t=e.length;if(t!==0){for(var n=0;n<t;){var r=e[n++];if(!(r.f&24576)&&gr(r)&&(rn=new Set,Sr(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&$n(r),rn?.size>0)){un.clear();for(let e of rn){if(e.f&24576)continue;let t=[e],n=e.parent;for(;n!==null;)rn.has(n)&&(rn.delete(n),t.push(n)),n=n.parent;for(let e=t.length-1;e>=0;e--){let n=t[e];n.f&24576||Sr(n)}}rn.clear()}}rn=null}}function on(e){M.schedule(e)}function sn(e,t){if(!(e.f&32&&e.f&1024)){e.f&2048?t.d.push(e):e.f&4096&&t.m.push(e),D(e,_);for(var n=e.first;n!==null;)sn(n,t),n=n.next}}function cn(e){D(e,_);for(var t=e.first;t!==null;)cn(t),t=t.next}var ln=new Set,un=new Map,dn=!1;function fn(e,t){return{f:0,v:e,reactions:null,equals:Ae,rv:0,wv:0}}function F(e,t){let n=fn(e,t);return lr(n),n}function pn(e,t=!1,n=!0){let r=fn(e);return t||(r.equals=Me),r}function I(e,t,n=!1){return V!==null&&(!H||V.f&131072)&&qe()&&V.f&4325394&&(G===null||!G.has(e))&&Ve(),gn(e,n?yn(t):t,Ot)}var mn=null,hn=0;function gn(e,t,n=null){if(!e.equals(t)){or?un.set(e,t):un.has(e)||un.set(e,e.v);var r=Kt.ensure();if(r.capture(e,t),e.f&2){let t=e;e.f&2048&&ht(t),N===null&&$e(t)}e.wv=hr(),mn=null,hn=0,vn(e,v,n),mn=null,qe()&&W!==null&&W.f&1024&&!(W.f&96)&&(J===null?ur([e]):J.push(e)),!r.is_fork&&ln.size>0&&!dn&&_n()}return t}function _n(){dn=!1;for(let e of ln){e.f&1024&&D(e,y);let t;try{t=gr(e)}catch{t=!0}t&&Sr(e)}ln.clear()}function L(e){I(e,e.v+1)}function vn(e,t,n){var r=e.reactions;if(r!==null){var i=qe(),a=r.length;if(hn+=a,hn>1e5&&mn===null&&(mn=new Set),mn!==null){if(mn.has(e))return;mn.add(e)}for(var o=0;o<a;o++){var s=r[o],c=s.f;if(i||s!==W){var l=(c&v)===0;if(l&&D(s,t),c&131072)ln.add(s);else if(c&2){var u=s;N?.delete(u),vn(u,y,n)}else if(l){var d=s;c&16&&rn!==null&&rn.add(d),n===null?on(d):n.push(d)}}}}}function yn(t){if(typeof t!=`object`||!t||ue in t||de in t)return t;let n=l(t);if(n!==s&&n!==c)return t;var r=new Map,i=e(t),o=F(0),u=null,d=pr,f=e=>{if(pr===d)return e();var t=V,n=pr;U(null),mr(d);var r=e();return U(t),mr(n),r};return i&&r.set(`length`,F(t.length,u)),new Proxy(t,{defineProperty(e,t,n){(!(`value`in n)||n.configurable===!1||n.enumerable===!1||n.writable===!1)&&ze();var i=r.get(t);return i===void 0?f(()=>{var e=F(n.value,u);return r.set(t,e),e}):I(i,n.value,!0),!0},deleteProperty(e,t){var n=r.get(t);if(n===void 0){if(t in e){let e=f(()=>F(S,u));r.set(t,e),L(o)}}else I(n,S),L(o);return!0},get(e,n,i){if(n===ue)return t;var o=r.get(n),s=n in e;if(o===void 0&&(!s||a(e,n)?.writable)&&(o=f(()=>F(yn(s?e[n]:S),u)),r.set(n,o)),o!==void 0){var c=Y(o);return c===S?void 0:c}return Reflect.get(e,n,i)},getOwnPropertyDescriptor(e,t){this.has?.(e,t);var n=Reflect.getOwnPropertyDescriptor(e,t),i=r.get(t);if(i!==void 0){var a=Y(i);if(a===S)return;if(n&&`value`in n)n.value=a;else return{enumerable:!0,configurable:!0,value:a,writable:!0}}return n},has(e,t){if(t===ue)return!0;var n=r.get(t),i=n!==void 0&&n.v!==S||Reflect.has(e,t);return(n!==void 0||W!==null&&(!i||a(e,t)?.writable))&&(n===void 0&&(n=f(()=>F(i?yn(e[t]):S,u)),r.set(t,n)),Y(n)===S)?!1:i},set(e,t,n,s){var c=r.get(t),l=t in e;if(i&&t===`length`)for(var d=n;d<c.v;d+=1){var p=r.get(d+``);p===void 0?d in e&&(p=f(()=>F(S,u)),r.set(d+``,p)):I(p,S)}if(c===void 0)(!l||a(e,t)?.writable)&&(c=f(()=>F(void 0,u)),I(c,yn(n)),r.set(t,c));else{l=c.v!==S;var m=f(()=>yn(n));I(c,m)}var h=Reflect.getOwnPropertyDescriptor(e,t);if(h?.set&&h.set.call(s,n),!l){if(i&&typeof t==`string`){var ee=r.get(`length`),g=Number(t);Number.isInteger(g)&&g>=ee.v&&I(ee,g+1)}L(o)}return!0},ownKeys(e){Y(o);var t=Reflect.ownKeys(e).filter(e=>{var t=r.get(e);return t===void 0||t.v!==S});for(var[n,i]of r)i.v!==S&&!(n in e)&&t.push(n);return t},setPrototypeOf(){Be()}})}var bn,xn,Sn,Cn;function wn(){if(bn===void 0){bn=window,xn=/Firefox/.test(navigator.userAgent);var e=Element.prototype,t=Node.prototype,n=Text.prototype;Sn=a(t,`firstChild`).get,Cn=a(t,`nextSibling`).get,u(e)&&(e[me]=void 0,e[pe]=null,e[he]=void 0,e.__e=void 0),u(n)&&(n[ge]=void 0)}}function Tn(e=``){return document.createTextNode(e)}function En(e){return Sn.call(e)}function Dn(e){return Cn.call(e)}function On(e,t){if(!C)return En(e);var n=En(w);if(n===null)n=w.appendChild(Tn());else if(t&&n.nodeType!==3){var r=Tn();return n?.before(r),T(r),r}return t&&Pn(n),T(n),n}function kn(e,t=!1){if(!C){var n=En(e);return n instanceof Comment&&n.data===``?Dn(n):n}if(t){if(w?.nodeType!==3){var r=Tn();return w?.before(r),T(r),r}Pn(w)}return w}function An(e,t=!1){if(!C)return En(e);var n=On(e,t);return Ee(e),n}function R(e,t=1,n=!1){let r=C?w:e;for(var i;t--;)i=r,r=Dn(r);if(!C)return r;if(n){if(r?.nodeType!==3){var a=Tn();return r===null?i?.after(a):r.before(a),T(a),a}Pn(r)}return T(r),r}function jn(e){e.textContent=``}function Mn(){return!1}function Nn(e,t,n){return t==null||t===`http://www.w3.org/1999/xhtml`?n?document.createElement(e,{is:n}):document.createElement(e):n?document.createElementNS(t,e,{is:n}):document.createElementNS(t,e)}function Pn(e){if(e.nodeValue.length<65536)return;let t=e.nextSibling;for(;t!==null&&t.nodeType===3;)t.remove(),e.nodeValue+=t.nodeValue,t=e.nextSibling}function Fn(e){var t=W;if(t===null)return V.f|=le,e;if(!(t.f&32768)&&!(t.f&4))throw e;In(e,t)}function In(e,t){if(e===be)throw e;if(!(t!==null&&t.f&16384)){for(;t!==null;){if(t.f&128&&!(t.f&33570816)){if(!(t.f&32768))throw e;try{t.b.error(e);return}catch(t){e=t}}t=t.parent}throw e}}function Ln(e){W===null&&(V===null&&Le(e),Ie()),or&&Fe(e)}function Rn(e,t){var n=t.last;n===null?t.last=t.first=e:(n.next=e,e.prev=n,t.last=e)}function zn(e,t){var n=W;n!==null&&n.f&8192&&(e|=b);var r={ctx:E,deps:null,nodes:null,f:e|v|512,first:null,fn:t,last:null,next:null,parent:n,b:n&&n.b,prev:null,teardown:null,wv:0,ac:null};M?.register_created_effect(r);var i=r;if(e&4)Dt===null?Kt.ensure().schedule(r):Dt.push(r);else if(t!==null){try{Sr(r)}catch(e){throw B(r),e}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&!(i.f&524288)&&(i=i.first,e&16&&e&65536&&i!==null&&(i.f|=re))}if(i!==null&&(i.parent=n,n!==null&&Rn(i,n),V!==null&&V.f&2&&!(e&64))){var a=V;(a.effects??(a.effects=[])).push(i)}return r}function Bn(){return V!==null&&!H}function Vn(e){let t=zn(8,null);return D(t,_),t.teardown=e,t}function Hn(e){Ln(`$effect`);var t=W.f;if(!V&&t&32&&E!==null&&!E.i){var n=E;(n.e??(n.e=[])).push(e)}else return Un(e)}function Un(e){return zn(4|ae,e)}function Wn(e){Kt.ensure();let t=zn(64|ie,e);return(e={})=>new Promise(n=>{e.outro?er(t,()=>{B(t),n(void 0)}):(B(t),n(void 0))})}function Gn(e){return zn(ce|ie,e)}function Kn(e,t=0){return zn(8|t,e)}function qn(e,t=[],n=[],r=[]){at(r,t,n,t=>{zn(8,()=>{e(...t.map(Y))})})}function Jn(e,t=0){return zn(16|t,e)}function z(e){return zn(32|ie,e)}function Yn(e){var t=e.teardown;if(t!==null){let n=or,r=V;sr(!0),U(null);try{t.call(null)}catch(t){In(t,e.parent)}finally{sr(n),U(r)}}}function Xn(e,t=!1){var n=e.first;for(e.first=e.last=null;n!==null;){let e=n.ac;e!==null&&rt(()=>{e.abort(ve)});var r=n.next;n.f&64?n.parent=null:B(n,t),n=r}}function Zn(e){for(var t=e.first;t!==null;){var n=t.next;t.f&32||B(t),t=n}}function B(e,t=!0){var n=!1;(t||e.f&262144)&&e.nodes!==null&&e.nodes.end!==null&&(Qn(e.nodes.start,e.nodes.end),n=!0),e.f|=ne,Xn(e,t&&!n),xr(e,0);var r=e.nodes&&e.nodes.t;if(r!==null)for(let e of r)e.stop();Yn(e),e.f^=ne,e.f|=x;var i=e.parent;i!==null&&i.first!==null&&$n(e),e.next=e.prev=e.teardown=e.ctx=e.deps=e.fn=e.nodes=e.ac=e.b=null}function Qn(e,t){for(;e!==null;){var n=e===t?null:Dn(e);e.remove(),e=n}}function $n(e){var t=e.parent,n=e.prev,r=e.next;n!==null&&(n.next=r),r!==null&&(r.prev=n),t!==null&&(t.first===e&&(t.first=r),t.last===e&&(t.last=n))}function er(e,t,n=!0){var r=[];e.f|=256,tr(e,r,!0);var i=()=>{n&&B(e),t&&t()},a=r.length;if(a>0){var o=()=>--a||i();for(var s of r)s.out(o)}else i()}function tr(e,t,n){if(!(e.f&8192)){e.f^=b;var r=e.nodes&&e.nodes.t;if(r!==null)for(let e of r)(e.is_global||n)&&t.push(e);for(var i=e.first;i!==null;){var a=i.next;if(!(i.f&64)){var o=!!(i.f&65536)||!!(i.f&32)&&!!(e.f&16);tr(i,t,o?n:!1)}i=a}}}function nr(e){e.f&=-257,rr(e,!0)}function rr(e,t){if(!(e.f&256)&&e.f&8192){e.f^=b,e.f&1024||(D(e,v),Kt.ensure().schedule(e));for(var n=e.first;n!==null;){var r=n.next,i=!!(n.f&65536)||!!(n.f&32);rr(n,i?t:!1),n=r}var a=e.nodes&&e.nodes.t;if(a!==null)for(let e of a)(e.is_global||t)&&e.in()}}function ir(e,t){if(e.nodes)for(var n=e.nodes.start,r=e.nodes.end;n!==null;){var i=n===r?null:Dn(n);t.append(n),n=i}}var ar=null,or=!1;function sr(e){or=e}var V=null,H=!1;function U(e){V=e}var W=null;function cr(e){W=e}var G=null;function lr(e){V!==null&&(V.f&2097152||V.f&2)&&(G??(G=new Set)).add(e)}var K=null,q=0,J=null;function ur(e){J=e}var dr=1,fr=0,pr=fr;function mr(e){pr=e}function hr(){return++dr}function gr(e){var t=e.f;if(t&2048)return!0;if(t&4096){for(var n=e.deps,r=n.length,i=0;i<r;i++){var a=n[i];if(gr(a)&&gt(a),a.wv>e.wv)return!0}t&512&&N===null&&D(e,_)}return!1}function _r(e,t,n=!0){var r=e.reactions;if(r!==null&&!(G!==null&&G.has(e)))for(var i=0;i<r.length;i++){var a=r[i];a.f&2?_r(a,t,!1):t===a&&(n?D(a,v):a.f&1024&&D(a,y),on(a))}}function vr(e){var t=K,n=q,r=J,i=V,a=G,o=E,s=H,c=pr,l=e.f;K=null,q=0,J=null,V=l&96?null:e,G=null,Ue(e.ctx),H=!1,pr=++fr,e.ac!==null&&(rt(()=>{e.ac.abort(ve)}),e.ac=null);try{e.f|=se;var u=e.fn,d=u();e.f|=te;var f=yr(e);if(qe()&&J!==null&&!H&&f!==null&&!(e.f&6146))for(var p=0;p<J.length;p++)_r(J[p],e);if(i!==null&&i!==e){if(fr++,i.deps!==null)for(let e=0;e<n;e+=1)i.deps[e].rv=fr;if(t!==null)for(let e of t)e.rv=fr;J!==null&&(r===null?r=J:r.push(...J))}return e.f&8388608&&(e.f^=le),d}catch(t){return yr(e),Fn(t)}finally{e.f^=se,K=t,q=n,J=r,V=i,G=a,Ue(o),H=s,pr=c}}function yr(e){var t=e.deps,n=M?.is_fork;if(K!==null){var r;if(n||xr(e,q),t!==null&&q>0)for(t.length=q+K.length,r=0;r<K.length;r++)t[q+r]=K[r];else e.deps=t=K;if(Bn()&&e.f&512)for(r=q;r<t.length;r++){var i;((i=t[r]).reactions??(i.reactions=[])).push(e)}}else!n&&t!==null&&q<t.length&&(xr(e,q),t.length=q);return t}function br(e,r){let i=r.reactions;if(i!==null){var a=t.call(i,e);if(a!==-1){var o=i.length-1;o===0?i=r.reactions=null:(i[a]=i[o],i.pop())}}if(i===null&&r.f&2&&(K===null||!n.call(K,r))){var s=r;s.f&512&&(s.f^=512),s.v!==S&&$e(s),s.ac!==null&&rt(()=>{s.ac.abort(ve),s.ac=null,D(s,v)}),_t(s),xr(s,0)}}function xr(e,t){var n=e.deps;if(n!==null)for(var r=t;r<n.length;r++)br(e,n[r])}function Sr(e){var t=e.f;if(!(t&16384)){D(e,_);var n=W;W=e;try{t&16777232?Zn(e):Xn(e),Yn(e);var r=vr(e);e.teardown=typeof r==`function`?r:null,e.wv=dr}finally{W=n}}}async function Cr(){await Promise.resolve(),tn()}function Y(e){var t=!!(e.f&2);if(ar?.add(e),V!==null&&!H&&!(W!==null&&W.f&16384)&&(G===null||!G.has(e))){var r=V.deps;if(V.f&2097152)e.rv<fr&&(e.rv=fr,K===null&&r!==null&&r[q]===e?q++:K===null?K=[e]:K.push(e));else{var i;(i=V).deps??(i.deps=[]),n.call(V.deps,e)||V.deps.push(e);var a=e.reactions;a===null?e.reactions=[V]:n.call(a,V)||a.push(V)}}if(or&&un.has(e))return un.get(e);if(t){var o=e;if(or){var s=o.v;return(!(o.f&1024)&&o.reactions!==null||Tr(o))&&(s=ht(o)),un.set(o,s),s}var c=!(o.f&512)&&!H&&V!==null&&!!(V.f&512),l=(o.f&te)===0;gr(o)&&(c&&(o.f|=512),gt(o)),c&&!l&&(vt(o),wr(o))}if(N?.has(e))return N.get(e);if(e.f&8388608)throw e.v;return e.v}function wr(e){if(e.f|=512,e.deps!==null)for(let r of e.deps){var t=r.reactions;t===null?r.reactions=[e]:n.call(t,e)||t.push(e),r.f&2&&!(r.f&512)&&(vt(r),wr(r))}}function Tr(e){if(e.v===S)return!0;if(e.deps===null)return!1;for(let t of e.deps)if(un.has(t)||t.f&2&&Tr(t))return!0;return!1}function Er(e){var t=H;try{return H=!0,e()}finally{H=t}}[...`allowfullscreen.async.autofocus.autoplay.checked.controls.default.disabled.formnovalidate.indeterminate.inert.ismap.loop.multiple.muted.nomodule.novalidate.open.playsinline.readonly.required.reversed.seamless.selected.webkitdirectory.defer.disablepictureinpicture.disableremoteplayback`.split(`.`)];var Dr=[`touchstart`,`touchmove`];function Or(e){return Dr.includes(e)}var kr=Symbol(`events`),Ar=new Set,jr=new Set;function Mr(e,t,n){(t[kr]??(t[kr]={}))[e]=n}function Nr(e){for(var t=0;t<e.length;t++)Ar.add(e[t]);for(var n of jr)n(e)}var Pr=null,Fr=!1;function Ir(e){var t=this,n=t.ownerDocument,r=e.type,a=e.composedPath?.()||[],o=a[0]||e.target;Pr=e,Fr||(Fr=!0,setTimeout(()=>{Fr=!1,Pr=null}));var s=0,c=Pr===e&&e[kr];if(c){var l=a.indexOf(c);if(l!==-1&&(t===document||t===window)){e[kr]=t;return}var u=a.indexOf(t);if(u===-1)return;l<=u&&(s=l)}if(o=a[s]||e.target,o!==t){i(e,`currentTarget`,{configurable:!0,get(){return o||n}});var d=V,f=W;U(null),cr(null);try{for(var p,m=[];o!==null&&o!==t;){try{var h=o[kr]?.[r];h!=null&&(!o.disabled||e.target===o)&&h.call(o,e)}catch(e){p?m.push(e):p=e}if(e.cancelBubble)break;s++,o=s<a.length?a[s]:null}if(p){for(let e of m)queueMicrotask(()=>{throw e});throw p}}finally{e[kr]=t,delete e.currentTarget,U(d),cr(f)}}}var Lr=globalThis?.window?.trustedTypes&&globalThis.window.trustedTypes.createPolicy(`svelte-trusted-html`,{createHTML:e=>e});function Rr(e){return Lr?.createHTML(e)??e}function zr(e){var t=Nn(`template`);return t.innerHTML=Rr(e.replaceAll(`<!>`,`<!---->`)),t.content}function Br(e,t){var n=W;n.nodes===null&&(n.nodes={start:e,end:t,a:null,t:null})}function Vr(e,t){var n=!!(t&1),r=!!(t&2),i,a=!e.startsWith(`<!>`);return()=>{if(C)return Br(w,null),w;i===void 0&&(i=zr(a?e:`<!>`+e),n||(i=En(i)));var t=r||xn?document.importNode(i,!0):i.cloneNode(!0);if(n){var o=En(t),s=t.lastChild;Br(o,s)}else Br(t,t);return t}}function Hr(e,t){if(C){var n=W;(!(n.f&32768)||n.nodes.end===null)&&(n.nodes.end=w),Te()}else e!==null&&e.before(t)}function Ur(e){let t=0,n=fn(0),r;return()=>{Bn()&&(Y(n),Kn(()=>(t===0&&(r=Er(()=>e(()=>L(n)))),t+=1,()=>{Xe(()=>{--t,t===0&&(r?.(),r=void 0,L(n))})})))}}var Wr=re|ie;function Gr(e,t,n,r){new oi(e,t,n,r)}var Kr=new WeakMap,qr=new WeakMap,Jr=new WeakMap,Yr=new WeakMap,X=new WeakMap,Xr=new WeakMap,Z=new WeakMap,Zr=new WeakMap,Qr=new WeakMap,$r=new WeakMap,ei=new WeakMap,ti=new WeakMap,ni=new WeakMap,ri=new WeakMap,ii=new WeakMap,ai=new WeakMap,Q=new WeakSet,oi=class{constructor(e,t,n,r){bt(this,Q),g(this,`parent`,void 0),g(this,`is_pending`,!1),g(this,`transform_error`,void 0),O(this,Kr,void 0),O(this,qr,C?w:null),O(this,Jr,void 0),O(this,Yr,void 0),O(this,X,void 0),O(this,Xr,null),O(this,Z,null),O(this,Zr,null),O(this,Qr,null),O(this,$r,0),O(this,ei,0),O(this,ti,!1),O(this,ni,new Set),O(this,ri,new Set),O(this,ii,null),O(this,ai,Ur(()=>(A(ii,this,fn(j($r,this))),()=>{A(ii,this,null)}))),A(Kr,this,e),A(Jr,this,t),A(Yr,this,e=>{var t=W;t.b=this,t.f|=128,n(e)}),this.parent=W.b,this.transform_error=r??this.parent?.transform_error??(e=>e),A(X,this,Jn(()=>{if(C){let e=ke(j(qr,this));Te();let t=e===`[!`;if(e.startsWith(`[?`)){let t=JSON.parse(e.slice(2));k(Q,this,ci).call(this,t)}else t?k(Q,this,di).call(this):k(Q,this,si).call(this)}else k(Q,this,fi).call(this)},Wr)),C&&A(Kr,this,w)}defer_effect(e){et(e,j(ni,this),j(ri,this))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!j(Jr,this).pending}update_pending_count(e,t){k(Q,this,hi).call(this,e,t),A($r,this,j($r,this)+e),j(ii,this)&&!j(ti,this)&&(A(ti,this,!0),Xe(()=>{A(ti,this,!1),j(ii,this)&&gn(j(ii,this),j($r,this))}))}get_effect_pending(){return j(ai,this).call(this),Y(j(ii,this))}error(e){if(e===be||!j(Jr,this).onerror&&!j(Jr,this).failed)throw e;M?.is_fork?(j(Xr,this)&&M.skip_effect(j(Xr,this)),j(Z,this)&&M.skip_effect(j(Z,this)),j(Zr,this)&&M.skip_effect(j(Zr,this)),M.oncommit(()=>{k(Q,this,ui).call(this)||k(Q,this,gi).call(this,e)})):k(Q,this,gi).call(this,e)}};function si(){try{A(Xr,this,z(()=>j(Yr,this).call(this,j(Kr,this))))}catch(e){this.error(e)}}function ci(e){let t=j(Jr,this).failed,{reset:n,invoke_onerror:r}=k(Q,this,li).call(this,e);Xe(r),t&&A(Zr,this,z(()=>{t(j(Kr,this),()=>e,()=>n)}))}function li(e){var t=!1,n=!1;let r=()=>{k(Q,this,ui).call(this)||(t?Ce():(t=!0,n&&He(),j(Zr,this)!==null&&er(j(Zr,this),()=>{A(Zr,this,null)}),k(Q,this,mi).call(this,()=>{k(Q,this,fi).call(this)})))};return{reset:r,invoke_onerror:()=>{if(!k(Q,this,ui).call(this))try{n=!0,j(Jr,this).onerror?.(e,r),n=!1}catch(e){In(e,j(X,this)&&j(X,this).parent)}}}}function ui(){return(j(X,this).f&(x|ne))!==0}function di(){let e=j(Jr,this).pending;e&&(this.is_pending=!0,A(Z,this,z(()=>e(j(Kr,this)))),Xe(()=>{if(!k(Q,this,ui).call(this)){var e=A(Qr,this,document.createDocumentFragment()),t=Tn(),n=!1;e.append(t),A(Xr,this,k(Q,this,mi).call(this,()=>{try{return z(()=>j(Yr,this).call(this,t))}catch(e){try{this.error(e),n=!0}catch(e){In(e,j(X,this).parent)}return null}})),j(Xr,this)===null?(A(Qr,this,null),n&&k(Q,this,pi).call(this,M)):j(ei,this)===0&&(j(Kr,this).before(e),A(Qr,this,null),er(j(Z,this),()=>{A(Z,this,null)}),k(Q,this,pi).call(this,M))}}))}function fi(){try{if(this.is_pending=this.has_pending_snippet(),A(ei,this,0),A($r,this,0),A(Xr,this,z(()=>{j(Yr,this).call(this,j(Kr,this))})),j(ei,this)>0){var e=A(Qr,this,document.createDocumentFragment());ir(j(Xr,this),e);let t=j(Jr,this).pending;A(Z,this,z(()=>t(j(Kr,this))))}else k(Q,this,pi).call(this,M)}catch(e){this.error(e)}}function pi(e){this.is_pending=!1,e.transfer_effects(j(ni,this),j(ri,this))}function mi(e){var t=W,n=V,r=E;cr(j(X,this)),U(j(X,this)),Ue(j(X,this).ctx);try{return Kt.ensure(),e()}finally{cr(t),U(n),Ue(r)}}function hi(e,t){if(this.has_pending_snippet())A(ei,this,j(ei,this)+e),j(ei,this)===0&&(k(Q,this,pi).call(this,t),j(Z,this)&&er(j(Z,this),()=>{A(Z,this,null)}),j(Qr,this)&&(j(Kr,this).before(j(Qr,this)),A(Qr,this,null)));else if(this.parent){var n;k(Q,n=this.parent,hi).call(n,e,t)}}function gi(e){j(Xr,this)&&(B(j(Xr,this)),A(Xr,this,null)),j(Z,this)&&(B(j(Z,this)),A(Z,this,null)),j(Zr,this)&&(B(j(Zr,this)),A(Zr,this,null)),C&&(T(j(qr,this)),De(),T(Oe()));let t=j(Jr,this).failed,n=e=>{if(k(Q,this,ui).call(this))return;let{reset:n,invoke_onerror:r}=k(Q,this,li).call(this,e);r(),t&&!k(Q,this,ui).call(this)&&A(Zr,this,k(Q,this,mi).call(this,()=>{try{return z(()=>{var r=W;r.b=this,r.f|=128,t(j(Kr,this),()=>e,()=>n)})}catch(e){return In(e,j(X,this).parent),null}}))};Xe(()=>{if(!k(Q,this,ui).call(this)){var t;try{t=this.transform_error(e)}catch(e){In(e,j(X,this)&&j(X,this).parent);return}typeof t==`object`&&t&&typeof t.then==`function`?t.then(n,e=>In(e,j(X,this)&&j(X,this).parent)):n(t)}})}function _i(e,t){var n=t==null?``:typeof t==`object`?`${t}`:t;n!==(e[ge]??(e[ge]=e.nodeValue))&&(e[ge]=n,e.nodeValue=`${n}`)}function vi(e,t){return bi(e,t)}var yi=new Map;function bi(e,{target:t,anchor:n,props:i={},events:a,context:o,intro:s=!0,transformError:c}){wn();var l=void 0,u=Wn(()=>{var s=n??t.appendChild(Tn());Gr(s,{pending:()=>{}},t=>{We({});var n=E;if(o&&(n.c=o),a&&(i.$$events=a),C&&Br(t,null),l=e(t,i)||Ke(),C&&(W.nodes.end=w,w===null||w.nodeType!==8||w.data!==`]`))throw Se(),be;Ge()},c);var u=new Set,d=e=>{for(var n=0;n<e.length;n++){var r=e[n];if(!u.has(r)){u.add(r);var i=Or(r);for(let e of[t,document]){var a=yi.get(e);a===void 0&&(a=new Map,yi.set(e,a));var o=a.get(r);o===void 0?(e.addEventListener(r,Ir,{passive:i}),a.set(r,1)):a.set(r,o+1)}}}};return d(r(Ar)),jr.add(d),()=>{for(var e of u)for(let n of[t,document]){var r=yi.get(n),i=r.get(e);--i==0?(n.removeEventListener(e,Ir),r.delete(e),r.size===0&&yi.delete(n)):r.set(e,i)}jr.delete(d),s!==n&&s.parentNode?.removeChild(s)}});return xi.set(l,u),l}var xi=new WeakMap,Si=new WeakMap,Ci=new WeakMap,wi=new WeakMap,Ti=new WeakMap,Ei=new WeakMap,Di=new WeakMap,Oi=new WeakMap,ki=class{constructor(e,t=!0){g(this,`anchor`,void 0),O(this,Si,new Map),O(this,Ci,new Map),O(this,wi,new Map),O(this,Ti,new Set),O(this,Ei,!0),O(this,Di,e=>{if(j(Si,this).has(e)){var t=j(Si,this).get(e),n=j(Ci,this).get(t);if(n)nr(n),j(Ti,this).delete(t);else{var r=j(wi,this).get(t);r&&(nr(r.effect),j(Ci,this).set(t,r.effect),j(wi,this).delete(t),r.fragment.lastChild.remove(),this.anchor.before(r.fragment),n=r.effect)}for(let[t,n]of j(Si,this)){if(j(Si,this).delete(t),t===e)break;let r=j(wi,this).get(n);r&&(B(r.effect),j(wi,this).delete(n))}for(let[e,r]of j(Ci,this)){if(e===t||j(Ti,this).has(e))continue;let i=()=>{if(Array.from(j(Si,this).values()).includes(e)){var t=document.createDocumentFragment();ir(r,t),t.append(Tn()),j(wi,this).set(e,{effect:r,fragment:t})}else B(r);j(Ti,this).delete(e),j(Ci,this).delete(e)};j(Ei,this)||!n?(j(Ti,this).add(e),er(r,i,!1)):i()}}}),O(this,Oi,e=>{j(Si,this).delete(e);let t=Array.from(j(Si,this).values());for(let[e,n]of j(wi,this))t.includes(e)||(B(n.effect),j(wi,this).delete(e))}),this.anchor=e,A(Ei,this,t)}ensure(e,t){var n=M,r=Mn();if(t&&!j(Ci,this).has(e)&&!j(wi,this).has(e)){if(r){var i=document.createDocumentFragment(),a=Tn();i.append(a),j(wi,this).set(e,{effect:z(()=>t(a)),fragment:i})}else j(Ci,this).set(e,z(()=>t(this.anchor)))}if(j(Si,this).set(n,e),r){for(let[t,r]of j(Ci,this))t===e?n.unskip_effect(r):n.skip_effect(r);for(let[t,r]of j(wi,this))t===e?n.unskip_effect(r.effect):n.skip_effect(r.effect);n.oncommit(j(Di,this)),n.ondiscard(j(Oi,this))}else C&&(this.anchor=w),j(Di,this).call(this,n)}};function Ai(e,t,n=!1){var r;C&&(r=w,Te());var i=new ki(e),a=n?re:0;function o(e,t){if(C){var n=ke(r);if(e!==parseInt(n.substring(1))){var a=Oe();T(a),i.anchor=a,we(!1),i.ensure(e,t),we(!0);return}}i.ensure(e,t)}Jn(()=>{var e=!1;t((t,n=0)=>{e=!0,o(n,t)}),e||o(-1,null)},a)}function ji(e,t){return t}function Mi(e,t,n){for(var i=[],a=t.length,o,s=t.length,c=0;c<a;c++){let n=t[c];er(n,()=>{if(o){if(o.pending.delete(n),o.done.add(n),o.pending.size===0){var t=e.outrogroups;Ni(e,r(o.done)),t.delete(o),t.size===0&&(e.outrogroups=null)}}else--s},!1)}if(s===0){var l=i.length===0&&n!==null&&e.pending.size===0;if(l){var u=n,d=u.parentNode;jn(d),d.append(u),e.items.clear()}Ni(e,t,!l)}else o={pending:new Set(t),done:new Set},(e.outrogroups??(e.outrogroups=new Set)).add(o)}function Ni(e,t,n=!0){var r;if(e.pending.size>0){r=new Set;for(let t of e.pending.values())for(let n of t)r.add(e.items.get(n).e)}for(var i=0;i<t.length;i++){var a=t[i];r?.has(a)?(a.f|=oe,ir(a,document.createDocumentFragment())):B(t[i],n)}}var Pi;function Fi(t,n,i,a,o,s=null){var c=t,l=new Map;if(n&4){var u=t;c=C?T(En(u)):u.appendChild(Tn())}C&&Te();var d=null,f=pt(()=>{var t=i();return e(t)?t:t==null?[]:r(t)}),p=new Map,m=!0;function h(e){if(!(g.effect.f&16384)){g.pending.delete(e);var t=Y(f);g.fallback=d,Li(g,t,c,n,a),d!==null&&(t.length===0?d.f&33554432?(d.f^=oe,zi(d,null,c)):nr(d):er(d,()=>{d=null}))}}function ee(e){g.pending.delete(e)}var g={effect:Jn(()=>{var e=Y(f),t=e.length;let r=!1;C&&ke(c)===`[!`!=(t===0)&&(c=Oe(),T(c),we(!1),r=!0);for(var u=new Set,g=M,_=Mn(),v=0;v<t;v+=1){C&&w.nodeType===8&&w.data===`]`&&(c=w,r=!0,we(!1));var y=e[v],b=a(y,v),x=m?null:l.get(b);x?(x.v&&gn(x.v,y),x.i&&gn(x.i,v),_&&g.unskip_effect(x.e)):(x=Ri(l,m?c:Pi??(Pi=Tn()),y,b,v,o,n,i),m||(x.e.f|=oe),l.set(b,x)),u.add(b)}if(t===0&&s&&!d&&(m?d=z(()=>s(c)):(d=z(()=>s(Pi??(Pi=Tn()))),d.f|=oe)),t>u.size&&Pe(``,``,``),C&&t>0&&T(Oe()),!m){if(p.set(g,u),_){for(let[e,t]of l)u.has(e)||g.skip_effect(t.e);g.oncommit(h),g.ondiscard(ee)}else h(g)}r&&we(!0),Y(f)}),flags:n,items:l,pending:p,outrogroups:null,fallback:d};m=!1,C&&(c=w)}function Ii(e){for(;e!==null&&!(e.f&32);)e=e.next;return e}function Li(e,t,n,i,a){var o=!!(i&8),s=t.length,c=e.items,l=Ii(e.effect.first),u,d=null,f,p=[],m=[],h,ee,g,_;if(o)for(_=0;_<s;_+=1)h=t[_],ee=a(h,_),g=c.get(ee).e,g.f&33554432||(g.nodes?.a?.measure(),(f??(f=new Set)).add(g));for(_=0;_<s;_+=1){if(h=t[_],ee=a(h,_),g=c.get(ee).e,e.outrogroups!==null)for(let t of e.outrogroups)t.pending.delete(g),t.done.delete(g);if(g.f&8192&&(nr(g),o&&(g.nodes?.a?.unfix(),(f??(f=new Set)).delete(g))),g.f&33554432){if(g.f^=oe,g===l)zi(g,null,n);else{var v=d?d.next:l;g===e.effect.last&&(e.effect.last=g.prev),g.prev&&(g.prev.next=g.next),g.next&&(g.next.prev=g.prev),Bi(e,d,g),Bi(e,g,v),zi(g,v,n),d=g,p=[],m=[],l=Ii(d.next);continue}}if(g!==l){if(u!==void 0&&u.has(g)){if(p.length<m.length){var y=m[0],b;d=y.prev;var x=p[0],te=p[p.length-1];for(b=0;b<p.length;b+=1)zi(p[b],y,n);for(b=0;b<m.length;b+=1)u.delete(m[b]);Bi(e,x.prev,te.next),Bi(e,d,x),Bi(e,te,y),l=y,d=te,--_,p=[],m=[]}else u.delete(g),zi(g,l,n),Bi(e,g.prev,g.next),Bi(e,g,d===null?e.effect.first:d.next),Bi(e,d,g),d=g;continue}for(p=[],m=[];l!==null&&l!==g;)(u??(u=new Set)).add(l),m.push(l),l=Ii(l.next);if(l===null)continue}g.f&33554432||p.push(g),d=g,l=Ii(g.next)}if(e.outrogroups!==null){for(let t of e.outrogroups)t.pending.size===0&&(Ni(e,r(t.done)),e.outrogroups?.delete(t));e.outrogroups.size===0&&(e.outrogroups=null)}if(l!==null||u!==void 0){var ne=[];if(u!==void 0)for(g of u)g.f&8192||ne.push(g);for(;l!==null;)!(l.f&8192)&&l!==e.fallback&&ne.push(l),l=Ii(l.next);var re=ne.length;if(re>0){var ie=i&4&&s===0?n:null;if(o){for(_=0;_<re;_+=1)ne[_].nodes?.a?.measure();for(_=0;_<re;_+=1)ne[_].nodes?.a?.fix()}Mi(e,ne,ie)}}o&&Xe(()=>{if(f!==void 0)for(g of f)g.nodes?.a?.apply()})}function Ri(e,t,n,r,i,a,o,s){var c=o&1?o&16?fn(n):pn(n,!1,!1):null,l=o&2?fn(i):null;return{v:c,i:l,e:z(()=>(a(t,c??n,l??i,s),()=>{e.delete(r)}))}}function zi(e,t,n){if(e.nodes)for(var r=e.nodes.start,i=e.nodes.end,a=t&&!(t.f&33554432)?t.nodes.start:n;r!==null;){var o=Dn(r);if(a.before(r),r===i)return;r=o}}function Bi(e,t,n){t===null?e.effect.first=n:t.next=n,n===null?e.effect.last=t:n.prev=t}var Vi=()=>performance.now(),Hi={tick:e=>requestAnimationFrame(e),now:()=>Vi(),tasks:new Set};function Ui(){let e=Hi.now();Hi.tasks.forEach(t=>{t.c(e)||(Hi.tasks.delete(t),t.f())}),Hi.tasks.size!==0&&Hi.tick(Ui)}function Wi(e){let t;return Hi.tasks.size===0&&Hi.tick(Ui),{promise:new Promise(n=>{Hi.tasks.add(t={c:e,f:n})}),abort(){Hi.tasks.delete(t)}}}var Gi=[...` 	
\r\f\xA0\v﻿`];function Ki(e,t,n){var r=e==null?``:``+e;if(t&&(r=r?r+` `+t:t),n){for(var i of Object.keys(n))if(n[i])r=r?r+` `+i:i;else if(r.length)for(var a=i.length,o=0;(o=r.indexOf(i,o))>=0;){var s=o+a;(o===0||Gi.includes(r[o-1]))&&(s===r.length||Gi.includes(r[s]))?r=(o===0?``:r.substring(0,o))+r.substring(s+1):o=s}}return r===``?null:r}function qi(e,t=!1){var n=t?` !important;`:`;`,r=``;for(var i of Object.keys(e)){var a=e[i];a!=null&&a!==``&&(r+=` `+i+`: `+a+n)}return r}function Ji(e){return e[0]!==`-`||e[1]!==`-`?e.toLowerCase():e}function Yi(e,t){if(t){var n=``,r,i;if(Array.isArray(t)?(r=t[0],i=t[1]):r=t,e){e=String(e).replaceAll(/\/\*.*?\*\//g,``).trim();var a=!1,o=0,s=!1,c=[];r&&c.push(...Object.keys(r).map(Ji)),i&&c.push(...Object.keys(i).map(Ji));var l=0,u=-1;let t=e.length;for(var d=0;d<t;d++){var f=e[d];if(s?f===`/`&&e[d-1]===`*`&&(s=!1):a?a===f&&(a=!1):f===`/`&&e[d+1]===`*`?s=!0:f===`"`||f===`'`?a=f:f===`(`?o++:f===`)`&&o--,!s&&a===!1&&o===0){if(f===`:`&&u===-1)u=d;else if(f===`;`||d===t-1){if(u!==-1){var p=Ji(e.substring(l,u).trim());if(!c.includes(p)){f!==`;`&&d++;var m=e.substring(l,d).trim();n+=` `+m+`;`}}l=d+1,u=-1}}}}return r&&(n+=qi(r)),i&&(n+=qi(i,!0)),n=n.trim(),n===``?null:n}return e==null?null:String(e)}function Xi(e,t,n,r,i,a){var o=e[me];if(C||o!==n||o===void 0){var s=Ki(n,r,a);(!C||s!==e.getAttribute(`class`))&&(s==null?e.removeAttribute(`class`):t?e.className=s:e.setAttribute(`class`,s)),e[me]=n}else if(a&&i!==a)for(var c in a){var l=!!a[c];(i==null||l!==!!i[c])&&e.classList.toggle(c,l)}return a}function Zi(e,t={},n,r){for(var i in n){var a=n[i];t[i]!==a&&(n[i]==null?e.style.removeProperty(i):e.style.setProperty(i,a,r))}}function Qi(e,t,n,r){var i=e[he];if(C||i!==t){var a=Yi(t,r);(!C||a!==e.getAttribute(`style`))&&(a==null?e.removeAttribute(`style`):e.style.cssText=a),e[he]=t}else r&&(Array.isArray(r)?(Zi(e,n?.[0],r[0]),Zi(e,n?.[1],r[1],`important`)):Zi(e,n,r));return r}var $i=Symbol(`is custom element`),ea=Symbol(`is html`),ta=ye?`link`:`LINK`;function na(e){if(C){var t=!1,n=()=>{if(!t){if(t=!0,e.hasAttribute(`value`)){var n=e.value;ra(e,`value`,null),e.value=n}if(e.hasAttribute(`checked`)){var r=e.checked;ra(e,`checked`,null),e.checked=r}}};e[_e]=n,Xe(n),nt()}}function ra(e,t,n,r){var i=ia(e);C&&(i[t]=e.getAttribute(t),t===`src`||t===`srcset`||t===`href`&&e.nodeName===ta)||i[t]!==(i[t]=n)&&(t===`loading`&&(e[fe]=n),n==null?e.removeAttribute(t):typeof n!=`string`&&oa(e).has(t)?e[t]=n:e.setAttribute(t,n))}function ia(e){return e[pe]??(e[pe]={[$i]:e.nodeName.includes(`-`),[ea]:e.namespaceURI===`http://www.w3.org/1999/xhtml`})}var aa=new Map;function oa(e){var t=e.getAttribute(`is`)||e.nodeName,n=aa.get(t);if(n)return n;aa.set(t,n=new Set);for(var r,i=e,a=Element.prototype;a!==i;){for(var s in r=o(i),r)r[s].set&&s!==`innerHTML`&&s!==`textContent`&&s!==`innerText`&&n.add(s);i=l(i)}return n}function sa(e,t,n=t){var r=new WeakSet;it(e,`input`,async i=>{var a=i?e.defaultValue:e.value;if(a=ca(e)?la(a):a,n(a),M!==null&&r.add(M),await Cr(),a!==(a=t())){var o=e.selectionStart,s=e.selectionEnd,c=e.value.length;if(e.value=a??``,s!==null){var l=e.value.length;o===s&&s===c&&l>c?(e.selectionStart=l,e.selectionEnd=l):(e.selectionStart=o,e.selectionEnd=Math.min(s,l))}}}),(C&&e.defaultValue!==e.value||Er(t)==null&&e.value)&&(n(ca(e)?la(e.value):e.value),M!==null&&r.add(M)),Kn(()=>{var n=t();if(e===document.activeElement){var i=M;if(r.has(i))return}ca(e)&&n===la(e.value)||(e.type!==`date`||n||e.value)&&n!==e.value&&(e.value=n??``)})}function ca(e){var t=e.type;return t===`number`||t===`range`}function la(e){return e===``?null:+e}var ua,da=new WeakMap,fa=new WeakMap,pa=new WeakMap,ma=new WeakSet,ha=class{constructor(e){bt(this,ma),O(this,da,new WeakMap),O(this,fa,void 0),O(this,pa,void 0),A(pa,this,e)}observe(e,t){var n=j(da,this).get(e)||new Set;return n.add(t),j(da,this).set(e,n),k(ma,this,ga).call(this).observe(e,j(pa,this)),()=>{var n=j(da,this).get(e);n.delete(t),n.size===0&&(j(da,this).delete(e),j(fa,this).unobserve(e))}}};ua=ha;function ga(){return j(fa,this)??A(fa,this,new ResizeObserver(e=>{for(var t of e){ua.entries.set(t.target,t);for(var n of j(da,this).get(t.target)||[])n(t)}}))}if(g(ha,`entries`,new WeakMap),typeof window<`u`){var _a,va;((_a=(va=window).__svelte??(va.__svelte={})).v??(_a.v=new Set)).add(`5`)}var ya=Vr(`<div class="slot svelte-13a9xrd"><div></div></div>`),ba=Vr(`<figcaption class="svelte-13a9xrd"> </figcaption>`),xa=Vr(`<div class="bars svelte-13a9xrd" role="img" aria-label="The selected numbers as bars"></div> <!>`,1),Sa=Vr(`<p class="svelte-13a9xrd">Select some numbers to see them here.</p>`),Ca=Vr(`<figure class="svelte-13a9xrd"><!></figure>`);function wa(e,t){We(t,!0);let n=ft(()=>t.values.slice(0,40)),r=ft(()=>Math.max(0,...Y(n))),i=ft(()=>Math.min(0,...Y(n))),a=ft(()=>Y(r)-Y(i)||1);var o=Ca(),s=On(o),c=e=>{var r=xa(),o=kn(r);Fi(o,21,()=>Y(n),ji,(e,t)=>{var n=ya(),r=On(n);let o,s;Ee(n),qn((e,i,a)=>{ra(n,`title`,e),o=Xi(r,1,`bar svelte-13a9xrd`,null,o,{below:Y(t)<0}),s=Qi(r,``,s,{height:i,bottom:a})},[()=>String(Y(t)),()=>`${Math.abs(Y(t))/Y(a)*100}%`,()=>`${(Math.min(Y(t),0)-Y(i))/Y(a)*100}%`]),Hr(e,n)}),Ee(o);var s=R(o,2),c=e=>{var n=ba(),r=An(n);qn(()=>_i(r,`The first 40 of ${t.values.length??``} numbers.`)),Hr(e,n)};Ai(s,e=>{t.values.length>40&&e(c)}),Hr(e,r)},l=e=>{Hr(e,Sa())};Ai(s,e=>{Y(n).length?e(c):e(l,-1)}),Ee(o),Hr(e,o),Ge()}var Ta,Ea=!1,Da=new WeakMap,Oa=new WeakMap,ka=new WeakMap,Aa=new WeakSet;Ta=class extends Date{constructor(...e){super(...e),bt(this,Aa),O(this,Da,F(super.getTime())),O(this,Oa,new Map),O(this,ka,V),Ea||k(Aa,this,ja).call(this)}};function ja(){Ea=!0;var e=Ta.prototype,t=Date.prototype,n=Object.getOwnPropertyNames(t);for(let r of n)(r.startsWith(`get`)||r.startsWith(`to`)||r===`valueOf`)&&(e[r]=function(...e){if(e.length>0)return Y(j(Da,this)),t[r].apply(this,e);var n=j(Oa,this).get(r);if(n===void 0){let i=V;U(j(ka,this)),n=ft(()=>(Y(j(Da,this)),t[r].apply(this,e))),j(Oa,this).set(r,n),U(i)}return Y(n)}),r.startsWith(`set`)&&(e[r]=function(...e){var n=t[r].apply(this,e);return I(j(Da,this),t.getTime.call(this)),n})}var Ma,Na,Pa=[`forEach`,`isDisjointFrom`,`isSubsetOf`,`isSupersetOf`],Fa=[`difference`,`intersection`,`symmetricDifference`,`union`],Ia=!1,La=new WeakMap,Ra=new WeakMap,za=new WeakMap,Ba=new WeakMap,Va=new WeakSet;Na=Symbol.iterator,Ma=class extends Set{constructor(e){if(super(),bt(this,Va),O(this,La,new Map),O(this,Ra,F(0)),O(this,za,F(0)),O(this,Ba,pr||-1),e){for(var t of e)super.add(t);j(za,this).v=super.size}Ia||k(Va,this,Ua).call(this)}has(e){var t=super.has(e),n=j(La,this),r=n.get(e);if(r===void 0){if(!t)return Y(j(Ra,this)),!1;r=k(Va,this,Ha).call(this,!0),n.set(e,r)}return Y(r),t}add(e){return super.has(e)||(super.add(e),I(j(za,this),super.size),L(j(Ra,this))),this}delete(e){var t=super.delete(e),n=j(La,this),r=n.get(e);return r!==void 0&&(n.delete(e),I(r,!1)),t&&(I(j(za,this),super.size),L(j(Ra,this))),t}clear(){if(super.size!==0){super.clear();var e=j(La,this);for(var t of e.values())I(t,!1);e.clear(),I(j(za,this),0),L(j(Ra,this))}}keys(){return this.values()}values(){return Y(j(Ra,this)),super.values()}entries(){return Y(j(Ra,this)),super.entries()}[Na](){return this.keys()}get size(){return Y(j(za,this))}};function Ha(e){return pr===j(Ba,this)?F(e):fn(e)}function Ua(){Ia=!0;var e=Ma.prototype,t=Set.prototype;for(let n of Pa)e[n]=function(...e){return Y(j(Ra,this)),t[n].apply(this,e)};for(let n of Fa)e[n]=function(...e){Y(j(Ra,this));var r=t[n].apply(this,e);return new Ma(r)}}function Wa(e){return Wa=Object.setPrototypeOf?Object.getPrototypeOf.bind():function(e){return e.__proto__||Object.getPrototypeOf(e)},Wa(e)}function Ga(e,t){for(;!{}.hasOwnProperty.call(e,t)&&(e=Wa(e))!==null;);return e}function Ka(){return Ka=typeof Reflect<`u`&&Reflect.get?Reflect.get.bind():function(e,t,n){var r=Ga(e,t);if(r){var i=Object.getOwnPropertyDescriptor(r,t);return i.get?i.get.call(arguments.length<3?e:n):i.value}},Ka.apply(null,arguments)}function qa(e,t,n,r){var i=Ka(Wa(1&r?e.prototype:e),t,n);return 2&r&&typeof i==`function`?function(e){return i.apply(n,e)}:i}var Ja,Ya,Xa=new WeakMap,Za=new WeakMap,Qa=new WeakMap,$a=new WeakMap,eo=new WeakSet;Ya=Symbol.iterator,Ja=class extends Map{constructor(e){if(super(),bt(this,eo),O(this,Xa,new Map),O(this,Za,F(0)),O(this,Qa,F(0)),O(this,$a,pr||-1),e){for(var[t,n]of e)super.set(t,n);j(Qa,this).v=super.size}}has(e){var t=j(Xa,this),n=t.get(e);if(n===void 0){if(super.has(e))n=k(eo,this,to).call(this,0),t.set(e,n);else return Y(j(Za,this)),!1}return Y(n),!0}forEach(e,t){k(eo,this,no).call(this),super.forEach(e,t)}get(e){var t=j(Xa,this),n=t.get(e);if(n===void 0){if(super.has(e))n=k(eo,this,to).call(this,0),t.set(e,n);else{Y(j(Za,this));return}}return Y(n),super.get(e)}getOrInsert(e,t){return super.has(e)||this.set(e,t),this.get(e)}getOrInsertComputed(e,t){return super.has(e)||this.set(e,t(e)),this.get(e)}set(e,t){var n=j(Xa,this),r=n.get(e),i=super.get(e),a=super.set(e,t),o=j(Za,this);if(r===void 0)r=k(eo,this,to).call(this,0),n.set(e,r),I(j(Qa,this),super.size),L(o);else if(i!==t){L(r);var s=o.reactions===null?null:new Set(o.reactions);(s===null||!r.reactions?.every(e=>s.has(e)))&&L(o)}return a}delete(e){var t=j(Xa,this),n=t.get(e),r=super.delete(e);return n!==void 0&&(t.delete(e),I(n,-1)),r&&(I(j(Qa,this),super.size),L(j(Za,this))),r}clear(){if(super.size!==0){super.clear();var e=j(Xa,this);I(j(Qa,this),0);for(var t of e.values())I(t,-1);L(j(Za,this)),e.clear()}}keys(){return Y(j(Za,this)),super.keys()}values(){return k(eo,this,no).call(this),super.values()}entries(){return k(eo,this,no).call(this),super.entries()}[Ya](){return this.entries()}get size(){return Y(j(Qa,this)),super.size}};function to(e){return pr===j($a,this)?F(e):fn(e)}function no(){Y(j(Za,this));var e=j(Xa,this);if(j(Qa,this).v!==e.size){for(var t of qa(Ja.prototype,`keys`,this,2)([]))if(!e.has(t)){var n=k(eo,this,to).call(this,0);e.set(t,n)}}for([,n]of j(Xa,this))Y(n)}var ro,io,ao=Symbol(`replace`),$=new WeakMap,oo=new WeakMap,so=new WeakMap,co=new WeakSet;io=Symbol.iterator,ro=class extends URLSearchParams{constructor(...e){super(...e),bt(this,co),O(this,$,F(0)),O(this,oo,fo()),O(this,so,!1)}[ao](e){if(!j(so,this)&&e.toString()!==super.toString()){A(so,this,!0);for(let e of[...super.keys()])super.delete(e);for(let[t,n]of e)super.append(t,n);L(j($,this)),A(so,this,!1)}}append(e,t){super.append(e,t),k(co,this,lo).call(this),L(j($,this))}delete(e,t){var n=super.has(e,t);super.delete(e,t),n&&(k(co,this,lo).call(this),L(j($,this)))}get(e){return Y(j($,this)),super.get(e)}getAll(e){return Y(j($,this)),super.getAll(e)}has(e,t){return Y(j($,this)),super.has(e,t)}keys(){return Y(j($,this)),super.keys()}forEach(e,t){Y(j($,this)),super.forEach(e,t)}set(e,t){var n=super.getAll(e);super.set(e,t);var r=super.getAll(e);(n.length!==r.length||n.some((e,t)=>e!==r[t]))&&(k(co,this,lo).call(this),L(j($,this)))}sort(){super.sort(),k(co,this,lo).call(this),L(j($,this))}toString(){return Y(j($,this)),super.toString()}values(){return Y(j($,this)),super.values()}entries(){return Y(j($,this)),super.entries()}[io](){return this.entries()}get size(){return Y(j($,this)),super.size}};function lo(){if(!j(oo,this)||j(so,this))return;A(so,this,!0);let e=qa(ro.prototype,`toString`,this,2)([]);j(oo,this).search=e&&`?${e}`,A(so,this,!1)}var uo=null;function fo(){return uo}function po(e){return Object.prototype.toString.call(e)===`[object Date]`}function mo(e){return e}function ho(e){let t=e-1;return t*t*t+1}function go(e,t){if(e===t||e!==e)return()=>e;let n=typeof e;if(n!==typeof t||Array.isArray(e)!==Array.isArray(t))throw Error(`Cannot interpolate values of different type`);if(Array.isArray(e)){let n=t.map((t,n)=>go(e[n],t));return e=>n.map(t=>t(e))}if(n===`object`){if(!e||!t)throw Error(`Object cannot be null`);if(po(e)&&po(t)){let n=e.getTime(),r=t.getTime()-n;return e=>new Date(n+e*r)}let n=Object.keys(t),r={};return n.forEach(n=>{r[n]=go(e[n],t[n])}),e=>{let t={};return n.forEach(n=>{t[n]=r[n](e)}),t}}if(n===`number`){let n=t-e;return t=>e+t*n}return()=>t}var _o=new WeakMap,vo=new WeakMap,yo=new WeakMap,bo=new WeakMap,xo=class e{constructor(e,t={}){O(this,_o,void 0),O(this,vo,void 0),O(this,yo,void 0),O(this,bo,null),A(_o,this,F(e)),A(vo,this,F(e)),A(yo,this,t)}static of(t,n){let r=new e(t(),n);return Kn(()=>{r.set(t())}),r}set(e,t){I(j(vo,this),e);let{delay:n=0,duration:r=400,easing:i=mo,interpolate:a=go}={...j(yo,this),...t};if(r===0)return j(bo,this)?.abort(),I(j(_o,this),e),Promise.resolve();let o=Hi.now()+n,s,c=!1,l=j(bo,this);return A(bo,this,Wi(t=>{if(t<o)return!0;if(!c){c=!0;let t=j(_o,this).v;s=a(t,e),typeof r==`function`&&(r=r(t,e)),l?.abort(),l=null}let n=t-o;return n>r?(I(j(_o,this),e),!1):(I(j(_o,this),s(i(n/r))),!0)})),j(bo,this).promise}get current(){return Y(j(_o,this))}get target(){return Y(j(vo,this))}set target(e){this.set(e)}},So=Vr(`<div class="stat svelte-hvtm5t"><span class="label svelte-hvtm5t"> </span> <span class="value svelte-hvtm5t"> </span></div>`);function Co(e,t){We(t,!0);let n=xo.of(()=>t.value??0,{duration:260,easing:ho}),r=new Intl.NumberFormat(void 0,{maximumFractionDigits:2});var i=So(),a=On(i),o=An(a,!0),s=An(R(a,2),!0);Ee(i),qn(e=>{_i(o,t.label),_i(s,e)},[()=>t.value===null?`–`:r.format(n.current)]),Hr(e,i),Ge()}var wo=Vr(`<p class="said svelte-1n46o8q" role="status"> </p>`),To=Vr(`<p class="failed svelte-1n46o8q" role="alert"> </p>`),Eo=Vr(`<main class="svelte-1n46o8q"><header class="svelte-1n46o8q"><h1 class="svelte-1n46o8q">Svelte in GridCraft</h1> <p class="svelte-1n46o8q">A Svelte app, shown inside the window. Select cells in the sheet: everything here follows.</p></header> <section class="where svelte-1n46o8q" aria-label="Selection"><span class="range svelte-1n46o8q"> </span> <span class="sheet svelte-1n46o8q"> </span> <span class="size svelte-1n46o8q"> </span></section> <section class="stats svelte-1n46o8q" aria-label="Summary"></section> <!> <section class="actions svelte-1n46o8q"><button class="primary svelte-1n46o8q">Fill sample data</button> <label class="svelte-1n46o8q">Multiply by <input type="range" min="0.5" max="4" step="0.5" class="svelte-1n46o8q"/> <output class="svelte-1n46o8q"> </output></label> <button class="svelte-1n46o8q">Write to the sheet</button></section> <!> <!> <footer class="svelte-1n46o8q">The source of this panel is in the module's <code class="svelte-1n46o8q">source</code> folder.</footer></main>`);function Do(e,t){We(t,!0);let n=window.spreadsheet,r=F(yn({range:``,sheet:``,rows:[]})),i=F(2),a=F(!1),o=F(``),s=F(``),c=ft(()=>Y(r).rows.flat()),l=ft(()=>Y(c).filter(e=>typeof e==`number`&&Number.isFinite(e))),u=ft(()=>Y(l).reduce((e,t)=>e+t,0)),d=ft(()=>[{label:`Cells`,value:Y(c).length},{label:`Numbers`,value:Y(l).length},{label:`Sum`,value:Y(u)},{label:`Average`,value:Y(l).length?Y(u)/Y(l).length:null},{label:`Smallest`,value:Y(l).length?Math.min(...Y(l)):null},{label:`Largest`,value:Y(l).length?Math.max(...Y(l)):null}]),f=ft(()=>Y(r).rows.map(e=>e.map(e=>typeof e==`number`?e*Y(i):e)));async function p(){try{I(r,await n.selection(),!0),I(s,``)}catch(e){I(s,e.message,!0)}}Hn(()=>{let e=[],t=!1;p();for(let r of[`selectionChanged`,`workbookChanged`,`workbookActivated`])n.on(r,p).then(n=>t?n():e.push(n),e=>I(s,e.message,!0));return()=>{t=!0,e.forEach(e=>e())}});async function m(e,t){I(a,!0),I(o,``),I(s,``);try{await t(),I(o,e,!0),await p()}catch(e){I(s,e.message,!0)}finally{I(a,!1)}}function h(e,t,n){let[,r,i]=/^([A-Z]+)([0-9]+)$/.exec(e)||[null,`A`,`1`],a=[...r].reduce((e,t)=>e*26+t.charCodeAt(0)-64,0)+n,o=``;for(;a>0;a=Math.floor((a-1)/26))o=String.fromCharCode(65+(a-1)%26)+o;return o+(Number(i)+t)}let ee=[[`Month`,`Sales`],[`January`,120],[`February`,135],[`March`,98],[`April`,164],[`May`,181],[`June`,152]],g=()=>m(`Sample data written. It is one undo step.`,()=>{let e=(Y(r).range||`A1`).split(`!`).pop().split(`:`)[0],t=e+`:`+h(e,0,1),i=e+`:`+h(e,ee.length-1,1);return n.batch([[`range.setValues`,{range:e,values:ee}],[`home.bold`,{range:t}],[`selection.set`,{range:i}]],{label:`Sample data`})}),_=()=>m(`Numbers multiplied by ${Y(i)}.`,()=>n.write(Y(r).range,Y(f)));var v=Eo(),y=R(On(v),2),b=On(y),x=An(b,!0),te=R(b,2),ne=An(te,!0),re=An(R(te,2));Ee(y);var ie=R(y,2);Fi(ie,21,()=>Y(d),e=>e.label,(e,t)=>{Co(e,{get label(){return Y(t).label},get value(){return Y(t).value}})}),Ee(ie);var ae=R(ie,2);wa(ae,{get values(){return Y(l)}});var oe=R(ae,2),se=On(oe),ce=R(se,2),le=R(On(ce));na(le);var ue=An(R(le,2));Ee(ce);var de=R(ce,2);Ee(oe);var fe=R(oe,2),pe=e=>{var t=wo(),n=An(t,!0);qn(()=>_i(n,Y(o))),Hr(e,t)};Ai(fe,e=>{Y(o)&&e(pe)});var me=R(fe,2),he=e=>{var t=To(),n=An(t,!0);qn(()=>_i(n,Y(s))),Hr(e,t)};Ai(me,e=>{Y(s)&&e(he)}),De(2),Ee(v),qn(()=>{_i(x,Y(r).range||`…`),_i(ne,Y(r).sheet),_i(re,`${Y(r).rows.length??``} × ${Y(r).rows[0]?.length??0??``}`),se.disabled=Y(a),_i(ue,`×${Y(i)??``}`),de.disabled=Y(a)||!Y(l).length}),Mr(`click`,se,g),sa(le,()=>Y(i),e=>I(i,e)),Mr(`click`,de,_),Hr(e,v),Ge()}Nr([`click`]),vi(Do,{target:document.getElementById(`app`)})})();"#),
    ("panel/app.css", r#"figure.svelte-13a9xrd{border:1px solid var(--line);background:var(--card);border-radius:8px;margin:0;padding:10px}.bars.svelte-13a9xrd{gap:3px;height:120px;display:flex}.slot.svelte-13a9xrd{flex:1;min-width:2px;position:relative}.bar.svelte-13a9xrd{background:var(--accent);border-radius:3px 3px 0 0;transition:height .25s,bottom .25s;position:absolute;left:0;right:0}.bar.below.svelte-13a9xrd{opacity:.6;border-radius:0 0 3px 3px}p.svelte-13a9xrd,figcaption.svelte-13a9xrd{color:var(--dim);margin:0;font-size:12.5px}p.svelte-13a9xrd{place-items:center;height:120px;display:grid}figcaption.svelte-13a9xrd{margin-top:6px}.stat.svelte-hvtm5t{border:1px solid var(--line);background:var(--card);border-radius:8px;gap:2px;padding:8px 10px;display:grid}.label.svelte-hvtm5t{color:var(--dim);font-size:11.5px}.value.svelte-hvtm5t{font-variant-numeric:tabular-nums;white-space:nowrap;text-overflow:ellipsis;font-size:17px;font-weight:600;overflow:hidden}:root{--lightningcss-light:initial;--lightningcss-dark: ;color-scheme:light dark;--accent:#107c41;--ink:#1f2328;--dim:#656d76;--line:#d0d7de;--card:#f6f8fa;--page:#fff}@media (prefers-color-scheme:dark){:root{--lightningcss-light: ;--lightningcss-dark:initial;--accent:#3fb970;--ink:#e6edf3;--dim:#8d96a0;--line:#30363d;--card:#161b22;--page:#0d1117}}body{background:var(--page);color:var(--ink);margin:0;font:14px/1.45 Segoe UI,system-ui,-apple-system,BlinkMacSystemFont,Roboto,Noto Sans,Ubuntu,Cantarell,Helvetica Neue,sans-serif}main.svelte-1n46o8q{gap:14px;padding:16px;display:grid}h1.svelte-1n46o8q{margin:0 0 2px;font-size:18px}footer.svelte-1n46o8q{color:var(--dim);margin:0;font-size:12.5px}header.svelte-1n46o8q p:where(.svelte-1n46o8q){color:var(--dim);margin:0;font-size:12.5px}.where.svelte-1n46o8q{align-items:baseline;gap:8px;display:flex}.range.svelte-1n46o8q{color:var(--accent);font:600 20px/1 ui-monospace,Consolas,monospace}.sheet.svelte-1n46o8q,.size.svelte-1n46o8q{color:var(--dim)}.size.svelte-1n46o8q{font-variant-numeric:tabular-nums;margin-left:auto}.stats.svelte-1n46o8q{grid-template-columns:repeat(3,1fr);gap:8px;display:grid}.actions.svelte-1n46o8q{gap:10px;display:grid}label.svelte-1n46o8q{align-items:center;gap:8px;display:flex}input[type=range].svelte-1n46o8q{accent-color:var(--accent);flex:1}output.svelte-1n46o8q{font-variant-numeric:tabular-nums;min-width:3ch}button.svelte-1n46o8q{font:inherit;border:1px solid var(--line);background:var(--card);color:var(--ink);cursor:pointer;border-radius:6px;padding:7px 12px}button.primary.svelte-1n46o8q{background:var(--accent);border-color:var(--accent);color:#fff}button.svelte-1n46o8q:disabled{opacity:.5;cursor:default}.said.svelte-1n46o8q{color:var(--accent);margin:0}.failed.svelte-1n46o8q{color:#cf222e;margin:0}code.svelte-1n46o8q{font-family:ui-monospace,Consolas,monospace}
/*$vite$:1*/"#),
    ("source/README.md", r#"# Svelte Showcase

A GridCraft module whose panel is a Svelte app. GridCraft serves the files in `panel`; this
folder is where two of them, `app.js` and `app.css`, are built from.

- `src/App.svelte` and `src/lib/*.svelte` are the app.
- `../panel/index.html` is the page. It loads `spreadsheet.js`, which GridCraft serves beside
  the page and which gives it its `spreadsheet` object, and then `app.js`.

To change the panel:

1. Copy this `source` folder somewhere of your own. A module may hold 2000 files, and
   `node_modules` holds more; and every file in a module is part of what you trusted.
2. `npm install`
3. Build into the module's panel folder:
   `PANEL_DIR=<the module>/panel npm run build`
   (in PowerShell: `$env:PANEL_DIR = "<the module>\panel"; npm run build`)
4. GridCraft will say the module has changed. Review it and trust it again in
   Developer ▸ Modules.

The module asks for `read` and `write` and nothing more. Its panel loads nothing from the
network: the Svelte runtime is inside `app.js` (see `THIRD-PARTY.md`).
"#),
    ("source/THIRD-PARTY.md", r#"# What `panel/app.js` contains that others wrote

The built panel script holds the app and the parts of these libraries it uses. Each is under the MIT licence, whose notices follow.

## svelte 5.57.2

Copyright (c) 2016-2025 [Svelte Contributors](https://github.com/sveltejs/svelte/graphs/contributors)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## clsx 2.1.1

MIT License

Copyright (c) Luke Edwards <luke.edwards05@gmail.com> (lukeed.com)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## esm-env 1.2.2

Copyright 2022 Benjamin McCann

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
"#),
    ("source/package.json", r#"{
  "name": "svelte-showcase",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "build": "vite build"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "7.3.1",
    "svelte": "5.57.2",
    "vite": "8.3.4"
  }
}
"#),
    ("source/vite.config.js", r#"import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// Builds the panel's script and stylesheet, under the names panel/index.html asks for.
// They go to dist/, or to the folder PANEL_DIR names: the module's panel folder (see README.md).
export default defineConfig({
  plugins: [svelte()],
  base: "./",
  build: {
    outDir: process.env.PANEL_DIR || "dist",
    emptyOutDir: false,
    target: "es2020",
    cssCodeSplit: false,
    rollupOptions: {
      input: "src/main.js",
      output: { format: "iife", entryFileNames: "app.js", assetFileNames: "app[extname]" },
    },
  },
});
"#),
    ("source/src/main.js", r#"import { mount } from "svelte";
import App from "./App.svelte";

mount(App, { target: document.getElementById("app") });
"#),
    ("source/src/App.svelte", r#"<script>
  import Bars from "./lib/Bars.svelte";
  import Stat from "./lib/Stat.svelte";

  // GridCraft gives the page this object (panel/index.html loads spreadsheet.js before the app).
  const sheet = window.spreadsheet;

  let selection = $state({ range: "", sheet: "", rows: [] });
  let scale = $state(2);
  let busy = $state(false);
  let said = $state("");
  let failed = $state("");

  // Everything below follows the selection by itself. That is Svelte's part.
  let cells = $derived(selection.rows.flat());
  let numbers = $derived(cells.filter((cell) => typeof cell === "number" && Number.isFinite(cell)));
  let sum = $derived(numbers.reduce((total, n) => total + n, 0));
  let stats = $derived([
    { label: "Cells", value: cells.length },
    { label: "Numbers", value: numbers.length },
    { label: "Sum", value: sum },
    { label: "Average", value: numbers.length ? sum / numbers.length : null },
    { label: "Smallest", value: numbers.length ? Math.min(...numbers) : null },
    { label: "Largest", value: numbers.length ? Math.max(...numbers) : null },
  ]);
  let scaled = $derived(selection.rows.map((row) => row.map((cell) => (typeof cell === "number" ? cell * scale : cell))));

  async function refresh() {
    try {
      selection = await sheet.selection();
      failed = "";
    } catch (error) {
      failed = error.message;
    }
  }

  // GridCraft tells the panel when the selection moves or the workbook changes.
  $effect(() => {
    const stops = [];
    let gone = false;
    refresh();
    for (const name of ["selectionChanged", "workbookChanged", "workbookActivated"]) {
      sheet.on(name, refresh).then(
        (stop) => (gone ? stop() : stops.push(stop)),
        (error) => (failed = error.message)
      );
    }
    return () => {
      gone = true;
      stops.forEach((stop) => stop());
    };
  });

  async function act(done, doing) {
    busy = true;
    said = "";
    failed = "";
    try {
      await doing();
      said = done;
      await refresh();
    } catch (error) {
      failed = error.message;
    } finally {
      busy = false;
    }
  }

  // "B3" moved down and to the right: for naming the block a table of rows takes.
  function moved(cell, down, right) {
    const [, letters, digits] = /^([A-Z]+)([0-9]+)$/.exec(cell) || [null, "A", "1"];
    let column = [...letters].reduce((n, letter) => n * 26 + letter.charCodeAt(0) - 64, 0) + right;
    let name = "";
    for (; column > 0; column = Math.floor((column - 1) / 26)) name = String.fromCharCode(65 + ((column - 1) % 26)) + name;
    return name + (Number(digits) + down);
  }

  const sample = [
    ["Month", "Sales"],
    ["January", 120],
    ["February", 135],
    ["March", 98],
    ["April", 164],
    ["May", 181],
    ["June", 152],
  ];

  const fillSample = () =>
    act("Sample data written. It is one undo step.", () => {
      const start = (selection.range || "A1").split("!").pop().split(":")[0];
      const header = start + ":" + moved(start, 0, 1);
      const block = start + ":" + moved(start, sample.length - 1, 1);
      return sheet.batch(
        [
          ["range.setValues", { range: start, values: sample }],
          ["home.bold", { range: header }],
          ["selection.set", { range: block }],
        ],
        { label: "Sample data" }
      );
    });

  const writeScaled = () => act(`Numbers multiplied by ${scale}.`, () => sheet.write(selection.range, scaled));
</script>

<main>
  <header>
    <h1>Svelte in GridCraft</h1>
    <p>A Svelte app, shown inside the window. Select cells in the sheet: everything here follows.</p>
  </header>

  <section class="where" aria-label="Selection">
    <span class="range">{selection.range || "…"}</span>
    <span class="sheet">{selection.sheet}</span>
    <span class="size">{selection.rows.length} × {selection.rows[0]?.length ?? 0}</span>
  </section>

  <section class="stats" aria-label="Summary">
    {#each stats as stat (stat.label)}
      <Stat label={stat.label} value={stat.value} />
    {/each}
  </section>

  <Bars values={numbers} />

  <section class="actions">
    <button class="primary" onclick={fillSample} disabled={busy}>Fill sample data</button>
    <label>
      Multiply by
      <input type="range" min="0.5" max="4" step="0.5" bind:value={scale} />
      <output>×{scale}</output>
    </label>
    <button onclick={writeScaled} disabled={busy || !numbers.length}>Write to the sheet</button>
  </section>

  {#if said}<p class="said" role="status">{said}</p>{/if}
  {#if failed}<p class="failed" role="alert">{failed}</p>{/if}

  <footer>The source of this panel is in the module's <code>source</code> folder.</footer>
</main>

<style>
  :global(:root) {
    color-scheme: light dark;
    --accent: #107c41;
    --ink: #1f2328;
    --dim: #656d76;
    --line: #d0d7de;
    --card: #f6f8fa;
    --page: #ffffff;
  }
  @media (prefers-color-scheme: dark) {
    :global(:root) {
      --accent: #3fb970;
      --ink: #e6edf3;
      --dim: #8d96a0;
      --line: #30363d;
      --card: #161b22;
      --page: #0d1117;
    }
  }
  :global(body) {
    margin: 0;
    background: var(--page);
    color: var(--ink);
    font: 14px/1.45 "Segoe UI", system-ui, sans-serif;
  }
  main {
    padding: 16px;
    display: grid;
    gap: 14px;
  }
  h1 {
    margin: 0 0 2px;
    font-size: 18px;
  }
  header p,
  footer {
    margin: 0;
    color: var(--dim);
    font-size: 12.5px;
  }
  .where {
    display: flex;
    gap: 8px;
    align-items: baseline;
  }
  .range {
    font: 600 20px/1 ui-monospace, Consolas, monospace;
    color: var(--accent);
  }
  .sheet,
  .size {
    color: var(--dim);
  }
  .size {
    margin-left: auto;
    font-variant-numeric: tabular-nums;
  }
  .stats {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  .actions {
    display: grid;
    gap: 10px;
  }
  label {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  input[type="range"] {
    flex: 1;
    accent-color: var(--accent);
  }
  output {
    min-width: 3ch;
    font-variant-numeric: tabular-nums;
  }
  button {
    font: inherit;
    padding: 7px 12px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--card);
    color: var(--ink);
    cursor: pointer;
  }
  button.primary {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }
  button:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .said {
    margin: 0;
    color: var(--accent);
  }
  .failed {
    margin: 0;
    color: #cf222e;
  }
  code {
    font-family: ui-monospace, Consolas, monospace;
  }
</style>
"#),
    ("source/src/lib/Stat.svelte", r#"<script>
  import { Tween } from "svelte/motion";
  import { cubicOut } from "svelte/easing";

  let { label, value } = $props();

  // The number glides to its new value instead of jumping.
  const shown = Tween.of(() => value ?? 0, { duration: 260, easing: cubicOut });
  const format = new Intl.NumberFormat(undefined, { maximumFractionDigits: 2 });
</script>

<div class="stat">
  <span class="label">{label}</span>
  <span class="value">{value === null ? "–" : format.format(shown.current)}</span>
</div>

<style>
  .stat {
    display: grid;
    gap: 2px;
    padding: 8px 10px;
    border: 1px solid var(--line);
    border-radius: 8px;
    background: var(--card);
  }
  .label {
    color: var(--dim);
    font-size: 11.5px;
  }
  .value {
    font-size: 17px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
"#),
    ("source/src/lib/Bars.svelte", r#"<script>
  // The numbers of the selection as bars. At most so many are drawn.
  const MOST = 40;

  let { values } = $props();

  let drawn = $derived(values.slice(0, MOST));
  let top = $derived(Math.max(0, ...drawn));
  let bottom = $derived(Math.min(0, ...drawn));
  let span = $derived(top - bottom || 1);
</script>

<figure>
  {#if drawn.length}
    <div class="bars" role="img" aria-label="The selected numbers as bars">
      {#each drawn as value, index (index)}
        <div class="slot" title={String(value)}>
          <div
            class="bar"
            class:below={value < 0}
            style:height="{(Math.abs(value) / span) * 100}%"
            style:bottom="{((Math.min(value, 0) - bottom) / span) * 100}%"
          ></div>
        </div>
      {/each}
    </div>
    {#if values.length > MOST}<figcaption>The first {MOST} of {values.length} numbers.</figcaption>{/if}
  {:else}
    <p>Select some numbers to see them here.</p>
  {/if}
</figure>

<style>
  figure {
    margin: 0;
    padding: 10px;
    border: 1px solid var(--line);
    border-radius: 8px;
    background: var(--card);
  }
  .bars {
    display: flex;
    gap: 3px;
    height: 120px;
  }
  .slot {
    position: relative;
    flex: 1;
    min-width: 2px;
  }
  .bar {
    position: absolute;
    left: 0;
    right: 0;
    border-radius: 3px 3px 0 0;
    background: var(--accent);
    transition:
      height 0.25s ease,
      bottom 0.25s ease;
  }
  .bar.below {
    border-radius: 0 0 3px 3px;
    opacity: 0.6;
  }
  p,
  figcaption {
    margin: 0;
    color: var(--dim);
    font-size: 12.5px;
  }
  p {
    height: 120px;
    display: grid;
    place-items: center;
  }
  figcaption {
    margin-top: 6px;
  }
</style>
"#),
];
