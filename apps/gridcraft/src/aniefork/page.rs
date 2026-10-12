//! The Development window's page ([`super::window`]): its files as the shell serves them, and
//! the Svelte project they are built from.
//!
//! **Generated: not written by hand.** `app.js` and `app.css` are what `npm run build` makes
//! of the project in [`SOURCE`] with svelte 5.57.2, vite 8.3.4 and
//! @sveltejs/vite-plugin-svelte 7.3.1. To change the page: write the project out, change and
//! build it, and take it back in (two tools among this crate's tests; the project's README.md
//! says how). A test checks that this file is what the tool writes.
//!
//! `app.js` holds parts of Svelte (and can hold parts of clsx and esm-env), each under the MIT
//! licence: their notices are in the project's `THIRD-PARTY.md`.
//!
//! This is the fork's third exception to upstream's "Rust only" rule, after the panel's
//! `spreadsheet.js` and the Svelte showcase: the owner asked for the Development window in
//! Svelte (2026-10-12). Like those it is text inside Rust source: the repository holds no
//! JavaScript file, and building GridCraft builds none.

/// The page as the shell serves it: each file's name, and its text.
#[rustfmt::skip]
pub(crate) const FILES: &[(&str, &str)] = &[
    ("index.html", r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Development</title>
    <link rel="stylesheet" href="app.css" />
  </head>
  <body>
    <div id="app"></div>
    <!-- Built from the Svelte source beside this file with `npm run build`. -->
    <script src="app.js"></script>
  </body>
</html>
"#),
    ("app.js", r#"(function(){var e=Array.isArray,t=Array.prototype.indexOf,n=Array.prototype.includes,r=Array.from,i=Object.defineProperty,a=Object.getOwnPropertyDescriptor,o=Object.getOwnPropertyDescriptors,s=Object.prototype,c=Array.prototype,l=Object.getPrototypeOf,u=Object.isExtensible,d=()=>{};function f(e){for(var t=0;t<e.length;t++)e[t]()}function p(){var e,t;return{promise:new Promise((n,r)=>{e=n,t=r}),resolve:e,reject:t}}function m(e){"@babel/helpers - typeof";return m=typeof Symbol==`function`&&typeof Symbol.iterator==`symbol`?function(e){return typeof e}:function(e){return e&&typeof Symbol==`function`&&e.constructor===Symbol&&e!==Symbol.prototype?`symbol`:typeof e},m(e)}function h(e,t){if(m(e)!=`object`||!e)return e;var n=e[Symbol.toPrimitive];if(n!==void 0){var r=n.call(e,t||`default`);if(m(r)!=`object`)return r;throw TypeError(`@@toPrimitive must return a primitive value.`)}return(t===`string`?String:Number)(e)}function ee(e){var t=h(e,`string`);return m(t)==`symbol`?t:t+``}function g(e,t,n){return(t=ee(t))in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}var _=1024,v=2048,y=4096,b=8192,x=16384,te=32768,S=1<<25,ne=65536,C=1<<19,re=1<<20,ie=1<<25,ae=1<<21,oe=1<<22,se=1<<23,ce=Symbol(`$state`),le=Symbol(`component`),ue=Symbol(``),de=Symbol(`attributes`),fe=Symbol(`class`),pe=Symbol(`style`),me=Symbol(`text`),he=Symbol(`form reset`),ge=new class extends Error{constructor(...e){super(...e),g(this,`name`,`StaleReactionError`),g(this,`message`,"The reaction that called `getAbortSignal()` was re-run or destroyed")}},_e=!!globalThis.document?.contentType&&globalThis.document.contentType.includes(`xml`),ve={},w=Symbol(`uninitialized`);function ye(){console.warn(`https://svelte.dev/e/derived_inert`)}function be(e){console.warn(`https://svelte.dev/e/hydration_mismatch`)}function xe(){console.warn(`https://svelte.dev/e/svelte_boundary_reset_noop`)}var T=!1;function Se(e){T=e}var E;function Ce(e){if(e===null)throw be(),ve;return E=e}function we(){return Ce(kn(E))}function D(e){if(T){if(kn(E)!==null)throw be(),ve;E=e}}function Te(e=1){if(T){for(var t=e,n=E;t--;)n=kn(n);E=n}}function Ee(e=!0){for(var t=0,n=E;;){if(n.nodeType===8){var r=n.data;if(r===`]`){if(t===0)return n;--t}else(r===`[`||r===`[!`||r[0]===`[`&&!isNaN(Number(r.slice(1))))&&(t+=1)}var i=kn(n);e&&n.remove(),n=i}}function De(e){if(!e||e.nodeType!==8)throw be(),ve;return e.data}function Oe(e){return e===this.v}function ke(e,t){return e==e?e!==t||typeof e==`object`&&!!e||typeof e==`function`:t==t}function Ae(e){return!ke(e,this.v)}function je(){throw Error(`https://svelte.dev/e/async_derived_orphan`)}function Me(e,t,n){throw Error(`https://svelte.dev/e/each_key_duplicate`)}function Ne(e){throw Error(`https://svelte.dev/e/effect_in_teardown`)}function Pe(){throw Error(`https://svelte.dev/e/effect_in_unowned_derived`)}function Fe(e){throw Error(`https://svelte.dev/e/effect_orphan`)}function Ie(){throw Error(`https://svelte.dev/e/effect_update_depth_exceeded`)}function Le(){throw Error(`https://svelte.dev/e/state_descriptors_fixed`)}function Re(){throw Error(`https://svelte.dev/e/state_prototype_fixed`)}function ze(){throw Error(`https://svelte.dev/e/state_unsafe_mutation`)}function Be(){throw Error(`https://svelte.dev/e/svelte_boundary_reset_onerror`)}var Ve=null;function He(e){Ve=e}function Ue(e,t=!1,n){Ve={p:Ve,i:!1,c:null,e:null,s:e,x:null,r:H,l:null}}function We(e){var t=Ve,n=t.e;if(n!==null){t.e=null;for(var r of n)Hn(r)}return e!==void 0&&(t.x=e),t.i=!0,Ve=t.p,Ge(e)}function Ge(e={}){return i(e,le,{value:!0}),e}function Ke(){return!0}var qe=[];function Je(){var e=qe;qe=[],f(e)}function Ye(e){if(qe.length===0&&!Tt){var t=qe;queueMicrotask(()=>{t===qe&&Je()})}qe.push(e)}function Xe(){for(;qe.length>0;)Je()}var Ze=~(v|y|_);function O(e,t){e.f=e.f&Ze|t}function Qe(e){e.f&512||e.deps===null?O(e,_):O(e,y)}function $e(e,t,n){e.f&2048?t.add(e):e.f&4096&&n.add(e),O(e,_)}var et=!1;function tt(){et||(et=!0,document.addEventListener(`reset`,e=>{Promise.resolve().then(()=>{if(!e.defaultPrevented)for(let t of e.target.elements)t[he]?.()})},{capture:!0}))}function nt(e){var t=V,n=H;lr(null),ur(null);try{return e()}finally{lr(t),ur(n)}}function rt(e,t,n,r=n){e.addEventListener(t,()=>nt(n));let i=e[he];e[he]=i?()=>{i(),r(!0)}:()=>r(!0),tt()}function it(e,t,n,r){let i=Ke()?ct:ft;var a=e.filter(e=>!e.settled),o=t.map(i);if(n.length===0&&a.length===0){r(o);return}var s=H,c=at(),l=a.length===1?a[0].promise:a.length>1?Promise.all(a.map(e=>e.promise)):null;function u(e){if(!(s.f&16384)){c();try{r([...o,...e])}catch(e){Fn(e,s)}ot()}}var d=st();if(n.length===0){l.then(()=>u([])).finally(d);return}function f(){Promise.all(n.map(e=>ut(e))).then(u).catch(e=>Fn(e,s)).finally(d)}l?l.then(()=>{s.f&16384?d():(c(),f(),ot())}):f()}function at(){var e=H,t=V,n=Ve,r=N;return function(i=!0){ur(e),lr(t),He(n),i&&!(e.f&16384)&&(r?.activate(),r?.apply())}}function ot(e=!0){ur(null),lr(null),He(null),e&&N?.deactivate()}function st(){var e=H,t=e.b,n=N,r=!!t?.is_rendered();return t?.update_pending_count(1,n),n.increment(r,e),()=>{t?.update_pending_count(-1,n),n.decrement(r,e)}}function ct(e){var t=2|v;return H!==null&&(H.f|=C),{ctx:Ve,deps:null,effects:null,equals:Oe,f:t,fn:e,reactions:null,rv:0,v:w,wv:0,parent:H,ac:null}}var lt=Symbol(`obsolete`);function ut(e,t,n){let r=H;r===null&&je();var i=void 0,a=fn(w),o=!V,s=new Set;return Wn(()=>{var t=H,n=p();i=n.promise;try{Promise.resolve(e()).then(n.resolve,e=>{e!==ge&&n.reject(e)}).finally(ot)}catch(e){n.reject(e),ot()}var c=N;if(o){if(t.f&32768)var l=st();if(r.b?.is_rendered())c.async_deriveds.get(t)?.reject(lt);else for(let e of s.values())e.reject(lt);s.add(n),c.async_deriveds.set(t,n)}let u=(e,t=void 0)=>{l?.(),s.delete(n),t!==lt&&(c.activate(),t?(a.f|=se,_n(a,t)):(a.f&8388608&&(a.f^=se),_n(a,e)),c.deactivate())};n.promise.then(u,e=>u(null,e||`unknown`))}),Bn(()=>{for(let e of s)e.reject(lt)}),new Promise(e=>{function t(n){function r(){n===i?e(a):t(i)}n.then(r,r)}t(i)})}function dt(e){let t=ct(e);return fr(t),t}function ft(e){let t=ct(e);return t.equals=Ae,t}function pt(e){var t=e.effects;if(t!==null){e.effects=null;for(var n=0;n<t.length;n+=1)Zn(t[n])}}function mt(e){var t,n=H,r=e.parent;if(!or&&r!==null&&e.v!==w&&r.f&24576)return ye(),e.v;ur(r);try{pt(e),t=wr(e)}finally{ur(n)}return t}function ht(e){var t=mt(e);!e.equals(t)&&(e.wv=xr(),(!N?.is_fork||e.deps===null)&&(N===null?e.v=t:(N.capture(e,t,!0),St?.capture(e,t,!0)),e.deps===null))?O(e,_):or||(Ct===null?Qe(e):(zn()||N?.is_fork)&&Ct.set(e,t))}function gt(e){if(e.effects!==null)for(let t of e.effects)(t.teardown||t.ac)&&(t.teardown?.(),t.ac!==null&&nt(()=>{t.ac.abort(ge),t.ac=null}),t.fn!==null&&(t.teardown=d),Dr(t,0),Yn(t))}function _t(e){if(e.effects!==null)for(let t of e.effects)t.teardown&&t.fn!==null&&Or(t)}function vt(e,t){if(t.has(e))throw TypeError(`Cannot initialize the same private elements twice on an object`)}function yt(e,t){vt(e,t),t.add(e)}function k(e,t,n){vt(e,t),t.set(e,n)}function A(e,t,n){if(typeof e==`function`?e===t:e.has(t))return arguments.length<3?t:n;throw TypeError(`Private element is not present on this object`)}function j(e,t,n){return e.set(A(e,t),n),n}function M(e,t){return e.get(A(e,t))}var bt,xt=null,N=null,St=null,Ct=null,wt=null,Tt=!1,Et=!1,Dt=null,Ot=null,kt=0,At=1,jt=new WeakMap,Mt=new WeakMap,Nt=new WeakMap,Pt=new WeakMap,Ft=new WeakMap,It=new WeakMap,Lt=new WeakMap,Rt=new WeakMap,zt=new WeakMap,Bt=new WeakMap,Vt=new WeakMap,Ht=new WeakMap,Ut=new WeakMap,Wt=new WeakMap,Gt=new WeakMap,P=new WeakSet,Kt=class e{constructor(){yt(this,P),g(this,`id`,At++),k(this,jt,!1),g(this,`linked`,!0),k(this,Mt,null),k(this,Nt,null),g(this,`async_deriveds`,new Map),g(this,`current`,new Map),g(this,`previous`,new Map),k(this,Pt,new Set),k(this,Ft,new Set),k(this,It,0),k(this,Lt,new Map),k(this,Rt,null),k(this,zt,[]),k(this,Bt,[]),k(this,Vt,new Set),k(this,Ht,new Set),k(this,Ut,new Map),k(this,Wt,new Set),g(this,`is_fork`,!1),k(this,Gt,!1),xt===null?xt=this:(j(Nt,xt,this),j(Mt,this,xt)),xt=this}skip_effect(e){M(Ut,this).has(e)||M(Ut,this).set(e,{d:[],m:[]}),M(Wt,this).delete(e)}unskip_effect(e,t=e=>this.schedule(e)){var n=M(Ut,this).get(e);if(n){M(Ut,this).delete(e);for(var r of n.d)O(r,v),t(r);for(r of n.m)O(r,y),t(r)}M(Wt,this).add(e)}capture(e,t,n=!1){e.v!==w&&!this.previous.has(e)&&this.previous.set(e,e.v),e.f&8388608||(this.current.set(e,[t,n]),Ct?.set(e,t)),this.is_fork||(e.v=t)}activate(){N=this}deactivate(){N=null,Ct=null}flush(){try{Et=!0,N=this,A(P,this,Yt).call(this)}finally{kt=0,wt=null,Dt=null,Ot=null,Et=!1,N=null,Ct=null,un.clear()}}discard(){for(let e of M(Ft,this))e(this);M(Ft,this).clear();for(let e of this.async_deriveds.values())e.reject(lt);A(P,this,en).call(this),M(Rt,this)?.resolve()}register_created_effect(e){M(Bt,this).push(e)}increment(e,t){if(j(It,this,M(It,this)+1),e){let e=M(Lt,this).get(t)??0;M(Lt,this).set(t,e+1)}}decrement(e,t){if(j(It,this,M(It,this)-1),e){let e=M(Lt,this).get(t)??0;e===1?M(Lt,this).delete(t):M(Lt,this).set(t,e-1)}M(Gt,this)||(j(Gt,this,!0),Ye(()=>{j(Gt,this,!1),this.linked&&this.flush()}))}transfer_effects(e,t){for(let t of e)M(Vt,this).add(t);for(let e of t)M(Ht,this).add(e);e.clear(),t.clear()}oncommit(e){M(Pt,this).add(e)}ondiscard(e){M(Ft,this).add(e)}settled(){return(M(Rt,this)??j(Rt,this,p())).promise}static ensure(){if(N===null){let t=N=new e;!Et&&!Tt&&Ye(()=>{M(jt,t)||t.flush()})}return N}apply(){Ct=null}schedule(e){wt=e,e.b?.is_pending&&e.f&16777228&&!(e.f&32768)?e.b.defer_effect(e):M(zt,this).push(e)}};bt=Kt;function qt(){if(this.is_fork)return!0;for(let n of M(Lt,this).keys()){for(var e=n,t=!1;e.parent!==null;){if(M(Ut,this).has(e)){t=!0;break}e=e.parent}if(!t)return!0}return!1}function Jt(){var e=[];for(let i of M(zt,this))if(!(i.f&16384||!(i.f&6144))){for(var t=i,n=!1;t.parent!==null;){t=t.parent;var r=t.f;if(r&96){if(!(r&1024)){n=!0;break}t.f^=_}}n||e.push(t)}return j(zt,this,[]),e}function Yt(){j(jt,this,!0);for(let e of M(Vt,this))M(Ht,this).delete(e),O(e,v),this.schedule(e);for(let e of M(Ht,this))O(e,y),this.schedule(e);this.apply();for(var e=Dt=[],t=[],n=Ot=[];M(zt,this).length>0;){kt++>1e3&&(A(P,this,en).call(this),nn());for(let n of A(P,this,Jt).call(this))try{A(P,this,Xt).call(this,n,e,t)}catch(e){throw cn(n),A(P,this,qt).call(this)||this.discard(),e}}if(N=null,n.length>0){var r=bt.ensure();for(let e of n)r.schedule(e)}if(Dt=null,Ot=null,A(P,this,qt).call(this)){A(P,this,$t).call(this,t),A(P,this,$t).call(this,e);for(let[e,t]of M(Ut,this))sn(e,t);n.length>0&&A(P,N,Yt).call(N);return}let i=A(P,this,Zt).call(this);if(i)A(P,this,$t).call(this,t),A(P,this,$t).call(this,e),A(P,i,Qt).call(i,this);else{M(Vt,this).clear(),M(Ht,this).clear();for(let e of M(Pt,this))e(this);M(Pt,this).clear(),St=this,an(t),an(e),St=null,M(Rt,this)?.resolve();var a=N;if(M(It,this)===0&&(M(zt,this).length===0||a!==null)&&A(P,this,en).call(this),M(zt,this).length>0){if(a!==null){for(let e of M(zt,this))M(zt,a).push(e);j(zt,this,[])}else a=this}a!==null&&(un.clear(),A(P,a,Yt).call(a))}}function Xt(e,t,n){e.f^=_;for(var r=e.first;r!==null;){var i=r.f,a=!!(i&96);if(!(a&&i&1024||i&8192||M(Ut,this).has(r))&&r.fn!==null){a?r.f^=_:i&4?t.push(r):Sr(r)&&(i&16&&M(Ht,this).add(r),Or(r));var o=r.first;if(o!==null){r=o;continue}}for(;r!==null;){var s=r.next;if(s!==null){r=s;break}r=r.parent}}}function Zt(){for(var e=M(Mt,this);e!==null;){if(!e.is_fork){for(let[t,[,n]]of this.current)if(e.current.has(t)&&!n)return e}e=M(Mt,e)}return null}function Qt(e){for(let[t,n]of e.current)!this.previous.has(t)&&e.previous.has(t)&&this.previous.set(t,e.previous.get(t)),this.current.set(t,n);for(let[t,n]of e.async_deriveds){let e=this.async_deriveds.get(t);e&&n.promise.then(e.resolve).catch(e.reject)}e.async_deriveds.clear(),this.transfer_effects(M(Vt,e),M(Ht,e));let t=e=>{var n=e.reactions;if(n!==null&&!(e.f&2&&!(e.f&6144)))for(let e of n){var r=e.f;if(r&2)t(e);else{var i=e;r&4194320&&!this.async_deriveds.has(i)&&(M(Ht,this).delete(i),O(i,v),this.schedule(i))}}};for(let e of this.current.keys())t(e);this.oncommit(()=>e.discard()),A(P,e,en).call(e),N=this,A(P,this,Yt).call(this)}function $t(e){for(var t=0;t<e.length;t+=1)$e(e[t],M(Vt,this),M(Ht,this))}function en(){if(this.linked){var e=M(Mt,this),t=M(Nt,this);e===null||j(Nt,e,t),t===null?xt=e:j(Mt,t,e),this.linked=!1}}function tn(e){var t=Tt,n=St;St=null,Tt=!0;try{var r;for(e&&(tn(),r=e());;){if(Xe(),N===null)return r;N.flush()}}finally{Tt=t,St=n}}function nn(){try{Ie()}catch(e){Fn(e,wt)}}var rn=null;function an(e){var t=e.length;if(t!==0){for(var n=0;n<t;){var r=e[n++];if(!(r.f&24576)&&Sr(r)&&(rn=new Set,Or(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&$n(r),rn?.size>0)){un.clear();for(let e of rn){if(e.f&24576)continue;let t=[e],n=e.parent;for(;n!==null;)rn.has(n)&&(rn.delete(n),t.push(n)),n=n.parent;for(let e=t.length-1;e>=0;e--){let n=t[e];n.f&24576||Or(n)}}rn.clear()}}rn=null}}function on(e){N.schedule(e)}function sn(e,t){if(!(e.f&32&&e.f&1024)){e.f&2048?t.d.push(e):e.f&4096&&t.m.push(e),O(e,_);for(var n=e.first;n!==null;)sn(n,t),n=n.next}}function cn(e){O(e,_);for(var t=e.first;t!==null;)cn(t),t=t.next}var ln=new Set,un=new Map,dn=!1;function fn(e,t){return{f:0,v:e,reactions:null,equals:Oe,rv:0,wv:0}}function pn(e,t){let n=fn(e,t);return fr(n),n}function mn(e,t=!1,n=!0){let r=fn(e);return t||(r.equals=Ae),r}function F(e,t,n=!1){return V!==null&&(!cr||V.f&131072)&&Ke()&&V.f&4325394&&(dr===null||!dr.has(e))&&ze(),_n(e,n?xn(t):t,Ot)}var hn=null,gn=0;function _n(e,t,n=null){if(!e.equals(t)){or?un.set(e,t):un.has(e)||un.set(e,e.v);var r=Kt.ensure();if(r.capture(e,t),e.f&2){let t=e;e.f&2048&&mt(t),Ct===null&&Qe(t)}e.wv=xr(),hn=null,gn=0,bn(e,v,n),hn=null,Ke()&&H!==null&&H.f&1024&&!(H.f&96)&&(hr===null?gr([e]):hr.push(e)),!r.is_fork&&ln.size>0&&!dn&&vn()}return t}function vn(){dn=!1;for(let e of ln){e.f&1024&&O(e,y);let t;try{t=Sr(e)}catch{t=!0}t&&Or(e)}ln.clear()}function yn(e){F(e,e.v+1)}function bn(e,t,n){var r=e.reactions;if(r!==null){var i=Ke(),a=r.length;if(gn+=a,gn>1e5&&hn===null&&(hn=new Set),hn!==null){if(hn.has(e))return;hn.add(e)}for(var o=0;o<a;o++){var s=r[o],c=s.f;if(i||s!==H){var l=(c&v)===0;if(l&&O(s,t),c&131072)ln.add(s);else if(c&2){var u=s;Ct?.delete(u),bn(u,y,n)}else if(l){var d=s;c&16&&rn!==null&&rn.add(d),n===null?on(d):n.push(d)}}}}}function xn(t){if(typeof t!=`object`||!t||ce in t||le in t)return t;let n=l(t);if(n!==s&&n!==c)return t;var r=new Map,i=e(t),o=pn(0),u=null,d=yr,f=e=>{if(yr===d)return e();var t=V,n=yr;lr(null),br(d);var r=e();return lr(t),br(n),r};return i&&r.set(`length`,pn(t.length,u)),new Proxy(t,{defineProperty(e,t,n){(!(`value`in n)||n.configurable===!1||n.enumerable===!1||n.writable===!1)&&Le();var i=r.get(t);return i===void 0?f(()=>{var e=pn(n.value,u);return r.set(t,e),e}):F(i,n.value,!0),!0},deleteProperty(e,t){var n=r.get(t);if(n===void 0){if(t in e){let e=f(()=>pn(w,u));r.set(t,e),yn(o)}}else F(n,w),yn(o);return!0},get(e,n,i){if(n===ce)return t;var o=r.get(n),s=n in e;if(o===void 0&&(!s||a(e,n)?.writable)&&(o=f(()=>pn(xn(s?e[n]:w),u)),r.set(n,o)),o!==void 0){var c=U(o);return c===w?void 0:c}return Reflect.get(e,n,i)},getOwnPropertyDescriptor(e,t){this.has?.(e,t);var n=Reflect.getOwnPropertyDescriptor(e,t),i=r.get(t);if(i!==void 0){var a=U(i);if(a===w)return;if(n&&`value`in n)n.value=a;else return{enumerable:!0,configurable:!0,value:a,writable:!0}}return n},has(e,t){if(t===ce)return!0;var n=r.get(t),i=n!==void 0&&n.v!==w||Reflect.has(e,t);return(n!==void 0||H!==null&&(!i||a(e,t)?.writable))&&(n===void 0&&(n=f(()=>pn(i?xn(e[t]):w,u)),r.set(t,n)),U(n)===w)?!1:i},set(e,t,n,s){var c=r.get(t),l=t in e;if(i&&t===`length`)for(var d=n;d<c.v;d+=1){var p=r.get(d+``);p===void 0?d in e&&(p=f(()=>pn(w,u)),r.set(d+``,p)):F(p,w)}if(c===void 0)(!l||a(e,t)?.writable)&&(c=f(()=>pn(void 0,u)),F(c,xn(n)),r.set(t,c));else{l=c.v!==w;var m=f(()=>xn(n));F(c,m)}var h=Reflect.getOwnPropertyDescriptor(e,t);if(h?.set&&h.set.call(s,n),!l){if(i&&typeof t==`string`){var ee=r.get(`length`),g=Number(t);Number.isInteger(g)&&g>=ee.v&&F(ee,g+1)}yn(o)}return!0},ownKeys(e){U(o);var t=Reflect.ownKeys(e).filter(e=>{var t=r.get(e);return t===void 0||t.v!==w});for(var[n,i]of r)i.v!==w&&!(n in e)&&t.push(n);return t},setPrototypeOf(){Re()}})}var Sn,Cn,wn,Tn;function En(){if(Sn===void 0){Sn=window,Cn=/Firefox/.test(navigator.userAgent);var e=Element.prototype,t=Node.prototype,n=Text.prototype;wn=a(t,`firstChild`).get,Tn=a(t,`nextSibling`).get,u(e)&&(e[fe]=void 0,e[de]=null,e[pe]=void 0,e.__e=void 0),u(n)&&(n[me]=void 0)}}function Dn(e=``){return document.createTextNode(e)}function On(e){return wn.call(e)}function kn(e){return Tn.call(e)}function I(e,t){if(!T)return On(e);var n=On(E);if(n===null)n=E.appendChild(Dn());else if(t&&n.nodeType!==3){var r=Dn();return n?.before(r),Ce(r),r}return t&&Nn(n),Ce(n),n}function L(e,t=!1){if(!T){var n=On(e);return n instanceof Comment&&n.data===``?kn(n):n}if(t){if(E?.nodeType!==3){var r=Dn();return E?.before(r),Ce(r),r}Nn(E)}return E}function R(e,t=!1){if(!T)return On(e);var n=I(e,t);return D(e),n}function z(e,t=1,n=!1){let r=T?E:e;for(var i;t--;)i=r,r=kn(r);if(!T)return r;if(n){if(r?.nodeType!==3){var a=Dn();return r===null?i?.after(a):r.before(a),Ce(a),a}Nn(r)}return Ce(r),r}function An(e){e.textContent=``}function jn(){return!1}function Mn(e,t,n){return t==null||t===`http://www.w3.org/1999/xhtml`?n?document.createElement(e,{is:n}):document.createElement(e):n?document.createElementNS(t,e,{is:n}):document.createElementNS(t,e)}function Nn(e){if(e.nodeValue.length<65536)return;let t=e.nextSibling;for(;t!==null&&t.nodeType===3;)t.remove(),e.nodeValue+=t.nodeValue,t=e.nextSibling}function Pn(e){var t=H;if(t===null)return V.f|=se,e;if(!(t.f&32768)&&!(t.f&4))throw e;Fn(e,t)}function Fn(e,t){if(e===ve)throw e;if(!(t!==null&&t.f&16384)){for(;t!==null;){if(t.f&128&&!(t.f&33570816)){if(!(t.f&32768))throw e;try{t.b.error(e);return}catch(t){e=t}}t=t.parent}throw e}}function In(e){H===null&&(V===null&&Fe(e),Pe()),or&&Ne(e)}function Ln(e,t){var n=t.last;n===null?t.last=t.first=e:(n.next=e,e.prev=n,t.last=e)}function Rn(e,t){var n=H;n!==null&&n.f&8192&&(e|=b);var r={ctx:Ve,deps:null,nodes:null,f:e|v|512,first:null,fn:t,last:null,next:null,parent:n,b:n&&n.b,prev:null,teardown:null,wv:0,ac:null};N?.register_created_effect(r);var i=r;if(e&4)Dt===null?Kt.ensure().schedule(r):Dt.push(r);else if(t!==null){try{Or(r)}catch(e){throw Zn(r),e}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&!(i.f&524288)&&(i=i.first,e&16&&e&65536&&i!==null&&(i.f|=ne))}if(i!==null&&(i.parent=n,n!==null&&Ln(i,n),V!==null&&V.f&2&&!(e&64))){var a=V;(a.effects??(a.effects=[])).push(i)}return r}function zn(){return V!==null&&!cr}function Bn(e){let t=Rn(8,null);return O(t,_),t.teardown=e,t}function Vn(e){In(`$effect`);var t=H.f;if(!V&&t&32&&Ve!==null&&!Ve.i){var n=Ve;(n.e??(n.e=[])).push(e)}else return Hn(e)}function Hn(e){return Rn(4|re,e)}function Un(e){Kt.ensure();let t=Rn(64|C,e);return(e={})=>new Promise(n=>{e.outro?er(t,()=>{Zn(t),n(void 0)}):(Zn(t),n(void 0))})}function Wn(e){return Rn(oe|C,e)}function Gn(e,t=0){return Rn(8|t,e)}function B(e,t=[],n=[],r=[]){it(r,t,n,t=>{Rn(8,()=>{e(...t.map(U))})})}function Kn(e,t=0){return Rn(16|t,e)}function qn(e){return Rn(32|C,e)}function Jn(e){var t=e.teardown;if(t!==null){let n=or,r=V;sr(!0),lr(null);try{t.call(null)}catch(t){Fn(t,e.parent)}finally{sr(n),lr(r)}}}function Yn(e,t=!1){var n=e.first;for(e.first=e.last=null;n!==null;){let e=n.ac;e!==null&&nt(()=>{e.abort(ge)});var r=n.next;n.f&64?n.parent=null:Zn(n,t),n=r}}function Xn(e){for(var t=e.first;t!==null;){var n=t.next;t.f&32||Zn(t),t=n}}function Zn(e,t=!0){var n=!1;(t||e.f&262144)&&e.nodes!==null&&e.nodes.end!==null&&(Qn(e.nodes.start,e.nodes.end),n=!0),e.f|=S,Yn(e,t&&!n),Dr(e,0);var r=e.nodes&&e.nodes.t;if(r!==null)for(let e of r)e.stop();Jn(e),e.f^=S,e.f|=x;var i=e.parent;i!==null&&i.first!==null&&$n(e),e.next=e.prev=e.teardown=e.ctx=e.deps=e.fn=e.nodes=e.ac=e.b=null}function Qn(e,t){for(;e!==null;){var n=e===t?null:kn(e);e.remove(),e=n}}function $n(e){var t=e.parent,n=e.prev,r=e.next;n!==null&&(n.next=r),r!==null&&(r.prev=n),t!==null&&(t.first===e&&(t.first=r),t.last===e&&(t.last=n))}function er(e,t,n=!0){var r=[];e.f|=256,tr(e,r,!0);var i=()=>{n&&Zn(e),t&&t()},a=r.length;if(a>0){var o=()=>--a||i();for(var s of r)s.out(o)}else i()}function tr(e,t,n){if(!(e.f&8192)){e.f^=b;var r=e.nodes&&e.nodes.t;if(r!==null)for(let e of r)(e.is_global||n)&&t.push(e);for(var i=e.first;i!==null;){var a=i.next;if(!(i.f&64)){var o=!!(i.f&65536)||!!(i.f&32)&&!!(e.f&16);tr(i,t,o?n:!1)}i=a}}}function nr(e){e.f&=-257,rr(e,!0)}function rr(e,t){if(!(e.f&256)&&e.f&8192){e.f^=b,e.f&1024||(O(e,v),Kt.ensure().schedule(e));for(var n=e.first;n!==null;){var r=n.next,i=!!(n.f&65536)||!!(n.f&32);rr(n,i?t:!1),n=r}var a=e.nodes&&e.nodes.t;if(a!==null)for(let e of a)(e.is_global||t)&&e.in()}}function ir(e,t){if(e.nodes)for(var n=e.nodes.start,r=e.nodes.end;n!==null;){var i=n===r?null:kn(n);t.append(n),n=i}}var ar=null,or=!1;function sr(e){or=e}var V=null,cr=!1;function lr(e){V=e}var H=null;function ur(e){H=e}var dr=null;function fr(e){V!==null&&(V.f&2097152||V.f&2)&&(dr??(dr=new Set)).add(e)}var pr=null,mr=0,hr=null;function gr(e){hr=e}var _r=1,vr=0,yr=vr;function br(e){yr=e}function xr(){return++_r}function Sr(e){var t=e.f;if(t&2048)return!0;if(t&4096){for(var n=e.deps,r=n.length,i=0;i<r;i++){var a=n[i];if(Sr(a)&&ht(a),a.wv>e.wv)return!0}t&512&&Ct===null&&O(e,_)}return!1}function Cr(e,t,n=!0){var r=e.reactions;if(r!==null&&!(dr!==null&&dr.has(e)))for(var i=0;i<r.length;i++){var a=r[i];a.f&2?Cr(a,t,!1):t===a&&(n?O(a,v):a.f&1024&&O(a,y),on(a))}}function wr(e){var t=pr,n=mr,r=hr,i=V,a=dr,o=Ve,s=cr,c=yr,l=e.f;pr=null,mr=0,hr=null,V=l&96?null:e,dr=null,He(e.ctx),cr=!1,yr=++vr,e.ac!==null&&(nt(()=>{e.ac.abort(ge)}),e.ac=null);try{e.f|=ae;var u=e.fn,d=u();e.f|=te;var f=Tr(e);if(Ke()&&hr!==null&&!cr&&f!==null&&!(e.f&6146))for(var p=0;p<hr.length;p++)Cr(hr[p],e);if(i!==null&&i!==e){if(vr++,i.deps!==null)for(let e=0;e<n;e+=1)i.deps[e].rv=vr;if(t!==null)for(let e of t)e.rv=vr;hr!==null&&(r===null?r=hr:r.push(...hr))}return e.f&8388608&&(e.f^=se),d}catch(t){return Tr(e),Pn(t)}finally{e.f^=ae,pr=t,mr=n,hr=r,V=i,dr=a,He(o),cr=s,yr=c}}function Tr(e){var t=e.deps,n=N?.is_fork;if(pr!==null){var r;if(n||Dr(e,mr),t!==null&&mr>0)for(t.length=mr+pr.length,r=0;r<pr.length;r++)t[mr+r]=pr[r];else e.deps=t=pr;if(zn()&&e.f&512)for(r=mr;r<t.length;r++){var i;((i=t[r]).reactions??(i.reactions=[])).push(e)}}else!n&&t!==null&&mr<t.length&&(Dr(e,mr),t.length=mr);return t}function Er(e,r){let i=r.reactions;if(i!==null){var a=t.call(i,e);if(a!==-1){var o=i.length-1;o===0?i=r.reactions=null:(i[a]=i[o],i.pop())}}if(i===null&&r.f&2&&(pr===null||!n.call(pr,r))){var s=r;s.f&512&&(s.f^=512),s.v!==w&&Qe(s),s.ac!==null&&nt(()=>{s.ac.abort(ge),s.ac=null,O(s,v)}),gt(s),Dr(s,0)}}function Dr(e,t){var n=e.deps;if(n!==null)for(var r=t;r<n.length;r++)Er(e,n[r])}function Or(e){var t=e.f;if(!(t&16384)){O(e,_);var n=H;H=e;try{t&16777232?Xn(e):Yn(e),Jn(e);var r=wr(e);e.teardown=typeof r==`function`?r:null,e.wv=_r}finally{H=n}}}async function kr(){await Promise.resolve(),tn()}function U(e){var t=!!(e.f&2);if(ar?.add(e),V!==null&&!cr&&!(H!==null&&H.f&16384)&&(dr===null||!dr.has(e))){var r=V.deps;if(V.f&2097152)e.rv<vr&&(e.rv=vr,pr===null&&r!==null&&r[mr]===e?mr++:pr===null?pr=[e]:pr.push(e));else{var i;(i=V).deps??(i.deps=[]),n.call(V.deps,e)||V.deps.push(e);var a=e.reactions;a===null?e.reactions=[V]:n.call(a,V)||a.push(V)}}if(or&&un.has(e))return un.get(e);if(t){var o=e;if(or){var s=o.v;return(!(o.f&1024)&&o.reactions!==null||jr(o))&&(s=mt(o)),un.set(o,s),s}var c=!(o.f&512)&&!cr&&V!==null&&!!(V.f&512),l=(o.f&te)===0;Sr(o)&&(c&&(o.f|=512),ht(o)),c&&!l&&(_t(o),Ar(o))}if(Ct?.has(e))return Ct.get(e);if(e.f&8388608)throw e.v;return e.v}function Ar(e){if(e.f|=512,e.deps!==null)for(let r of e.deps){var t=r.reactions;t===null?r.reactions=[e]:n.call(t,e)||t.push(e),r.f&2&&!(r.f&512)&&(_t(r),Ar(r))}}function jr(e){if(e.v===w)return!0;if(e.deps===null)return!1;for(let t of e.deps)if(un.has(t)||t.f&2&&jr(t))return!0;return!1}function Mr(e){var t=cr;try{return cr=!0,e()}finally{cr=t}}[...`allowfullscreen.async.autofocus.autoplay.checked.controls.default.disabled.formnovalidate.indeterminate.inert.ismap.loop.multiple.muted.nomodule.novalidate.open.playsinline.readonly.required.reversed.seamless.selected.webkitdirectory.defer.disablepictureinpicture.disableremoteplayback`.split(`.`)];var Nr=[`touchstart`,`touchmove`];function Pr(e){return Nr.includes(e)}var Fr=Symbol(`events`),Ir=new Set,Lr=new Set;function Rr(e,t,n,r={}){function i(e){if(r.capture||Ur.call(t,e),!e.cancelBubble)return nt(()=>n?.call(this,e))}return e.startsWith(`pointer`)||e.startsWith(`touch`)||e===`wheel`?(i.__removed=!1,Ye(()=>{i.__removed||t.addEventListener(e,i,r)})):t.addEventListener(e,i,r),i}function zr(e,t,n,r,i){var a={capture:r,passive:i},o=Rr(e,t,n,a);(t===document.body||t===window||t===document||t instanceof HTMLMediaElement)&&Bn(()=>{o.__removed=!0,t.removeEventListener(e,o,a)})}function W(e,t,n){(t[Fr]??(t[Fr]={}))[e]=n}function Br(e){for(var t=0;t<e.length;t++)Ir.add(e[t]);for(var n of Lr)n(e)}var Vr=null,Hr=!1;function Ur(e){var t=this,n=t.ownerDocument,r=e.type,a=e.composedPath?.()||[],o=a[0]||e.target;Vr=e,Hr||(Hr=!0,setTimeout(()=>{Hr=!1,Vr=null}));var s=0,c=Vr===e&&e[Fr];if(c){var l=a.indexOf(c);if(l!==-1&&(t===document||t===window)){e[Fr]=t;return}var u=a.indexOf(t);if(u===-1)return;l<=u&&(s=l)}if(o=a[s]||e.target,o!==t){i(e,`currentTarget`,{configurable:!0,get(){return o||n}});var d=V,f=H;lr(null),ur(null);try{for(var p,m=[];o!==null&&o!==t;){try{var h=o[Fr]?.[r];h!=null&&(!o.disabled||e.target===o)&&h.call(o,e)}catch(e){p?m.push(e):p=e}if(e.cancelBubble)break;s++,o=s<a.length?a[s]:null}if(p){for(let e of m)queueMicrotask(()=>{throw e});throw p}}finally{e[Fr]=t,delete e.currentTarget,lr(d),ur(f)}}}var Wr=globalThis?.window?.trustedTypes&&globalThis.window.trustedTypes.createPolicy(`svelte-trusted-html`,{createHTML:e=>e});function Gr(e){return Wr?.createHTML(e)??e}function Kr(e){var t=Mn(`template`);return t.innerHTML=Gr(e.replaceAll(`<!>`,`<!---->`)),t.content}function qr(e,t){var n=H;n.nodes===null&&(n.nodes={start:e,end:t,a:null,t:null})}function G(e,t){var n=!!(t&1),r=!!(t&2),i,a=!e.startsWith(`<!>`);return()=>{if(T)return qr(E,null),E;i===void 0&&(i=Kr(a?e:`<!>`+e),n||(i=On(i)));var t=r||Cn?document.importNode(i,!0):i.cloneNode(!0);if(n){var o=On(t),s=t.lastChild;qr(o,s)}else qr(t,t);return t}}function Jr(){if(T)return qr(E,null),E;var e=document.createDocumentFragment(),t=document.createComment(``),n=Dn();return e.append(t,n),qr(t,n),e}function K(e,t){if(T){var n=H;(!(n.f&32768)||n.nodes.end===null)&&(n.nodes.end=E),we()}else e!==null&&e.before(t)}function Yr(e){let t=0,n=fn(0),r;return()=>{zn()&&(U(n),Gn(()=>(t===0&&(r=Mr(()=>e(()=>yn(n)))),t+=1,()=>{Ye(()=>{--t,t===0&&(r?.(),r=void 0,yn(n))})})))}}var Xr=ne|C;function Zr(e,t,n,r){new mi(e,t,n,r)}var Qr=new WeakMap,$r=new WeakMap,ei=new WeakMap,ti=new WeakMap,ni=new WeakMap,ri=new WeakMap,ii=new WeakMap,ai=new WeakMap,oi=new WeakMap,si=new WeakMap,ci=new WeakMap,li=new WeakMap,ui=new WeakMap,di=new WeakMap,fi=new WeakMap,pi=new WeakMap,q=new WeakSet,mi=class{constructor(e,t,n,r){yt(this,q),g(this,`parent`,void 0),g(this,`is_pending`,!1),g(this,`transform_error`,void 0),k(this,Qr,void 0),k(this,$r,T?E:null),k(this,ei,void 0),k(this,ti,void 0),k(this,ni,void 0),k(this,ri,null),k(this,ii,null),k(this,ai,null),k(this,oi,null),k(this,si,0),k(this,ci,0),k(this,li,!1),k(this,ui,new Set),k(this,di,new Set),k(this,fi,null),k(this,pi,Yr(()=>(j(fi,this,fn(M(si,this))),()=>{j(fi,this,null)}))),j(Qr,this,e),j(ei,this,t),j(ti,this,e=>{var t=H;t.b=this,t.f|=128,n(e)}),this.parent=H.b,this.transform_error=r??this.parent?.transform_error??(e=>e),j(ni,this,Kn(()=>{if(T){let e=De(M($r,this));we();let t=e===`[!`;if(e.startsWith(`[?`)){let t=JSON.parse(e.slice(2));A(q,this,gi).call(this,t)}else t?A(q,this,yi).call(this):A(q,this,hi).call(this)}else A(q,this,bi).call(this)},Xr)),T&&j(Qr,this,E)}defer_effect(e){$e(e,M(ui,this),M(di,this))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!M(ei,this).pending}update_pending_count(e,t){A(q,this,Ci).call(this,e,t),j(si,this,M(si,this)+e),M(fi,this)&&!M(li,this)&&(j(li,this,!0),Ye(()=>{j(li,this,!1),M(fi,this)&&_n(M(fi,this),M(si,this))}))}get_effect_pending(){return M(pi,this).call(this),U(M(fi,this))}error(e){if(e===ve||!M(ei,this).onerror&&!M(ei,this).failed)throw e;N?.is_fork?(M(ri,this)&&N.skip_effect(M(ri,this)),M(ii,this)&&N.skip_effect(M(ii,this)),M(ai,this)&&N.skip_effect(M(ai,this)),N.oncommit(()=>{A(q,this,vi).call(this)||A(q,this,wi).call(this,e)})):A(q,this,wi).call(this,e)}};function hi(){try{j(ri,this,qn(()=>M(ti,this).call(this,M(Qr,this))))}catch(e){this.error(e)}}function gi(e){let t=M(ei,this).failed,{reset:n,invoke_onerror:r}=A(q,this,_i).call(this,e);Ye(r),t&&j(ai,this,qn(()=>{t(M(Qr,this),()=>e,()=>n)}))}function _i(e){var t=!1,n=!1;let r=()=>{A(q,this,vi).call(this)||(t?xe():(t=!0,n&&Be(),M(ai,this)!==null&&er(M(ai,this),()=>{j(ai,this,null)}),A(q,this,Si).call(this,()=>{A(q,this,bi).call(this)})))};return{reset:r,invoke_onerror:()=>{if(!A(q,this,vi).call(this))try{n=!0,M(ei,this).onerror?.(e,r),n=!1}catch(e){Fn(e,M(ni,this)&&M(ni,this).parent)}}}}function vi(){return(M(ni,this).f&(x|S))!==0}function yi(){let e=M(ei,this).pending;e&&(this.is_pending=!0,j(ii,this,qn(()=>e(M(Qr,this)))),Ye(()=>{if(!A(q,this,vi).call(this)){var e=j(oi,this,document.createDocumentFragment()),t=Dn(),n=!1;e.append(t),j(ri,this,A(q,this,Si).call(this,()=>{try{return qn(()=>M(ti,this).call(this,t))}catch(e){try{this.error(e),n=!0}catch(e){Fn(e,M(ni,this).parent)}return null}})),M(ri,this)===null?(j(oi,this,null),n&&A(q,this,xi).call(this,N)):M(ci,this)===0&&(M(Qr,this).before(e),j(oi,this,null),er(M(ii,this),()=>{j(ii,this,null)}),A(q,this,xi).call(this,N))}}))}function bi(){try{if(this.is_pending=this.has_pending_snippet(),j(ci,this,0),j(si,this,0),j(ri,this,qn(()=>{M(ti,this).call(this,M(Qr,this))})),M(ci,this)>0){var e=j(oi,this,document.createDocumentFragment());ir(M(ri,this),e);let t=M(ei,this).pending;j(ii,this,qn(()=>t(M(Qr,this))))}else A(q,this,xi).call(this,N)}catch(e){this.error(e)}}function xi(e){this.is_pending=!1,e.transfer_effects(M(ui,this),M(di,this))}function Si(e){var t=H,n=V,r=Ve;ur(M(ni,this)),lr(M(ni,this)),He(M(ni,this).ctx);try{return Kt.ensure(),e()}finally{ur(t),lr(n),He(r)}}function Ci(e,t){if(this.has_pending_snippet())j(ci,this,M(ci,this)+e),M(ci,this)===0&&(A(q,this,xi).call(this,t),M(ii,this)&&er(M(ii,this),()=>{j(ii,this,null)}),M(oi,this)&&(M(Qr,this).before(M(oi,this)),j(oi,this,null)));else if(this.parent){var n;A(q,n=this.parent,Ci).call(n,e,t)}}function wi(e){M(ri,this)&&(Zn(M(ri,this)),j(ri,this,null)),M(ii,this)&&(Zn(M(ii,this)),j(ii,this,null)),M(ai,this)&&(Zn(M(ai,this)),j(ai,this,null)),T&&(Ce(M($r,this)),Te(),Ce(Ee()));let t=M(ei,this).failed,n=e=>{if(A(q,this,vi).call(this))return;let{reset:n,invoke_onerror:r}=A(q,this,_i).call(this,e);r(),t&&!A(q,this,vi).call(this)&&j(ai,this,A(q,this,Si).call(this,()=>{try{return qn(()=>{var r=H;r.b=this,r.f|=128,t(M(Qr,this),()=>e,()=>n)})}catch(e){return Fn(e,M(ni,this).parent),null}}))};Ye(()=>{if(!A(q,this,vi).call(this)){var t;try{t=this.transform_error(e)}catch(e){Fn(e,M(ni,this)&&M(ni,this).parent);return}typeof t==`object`&&t&&typeof t.then==`function`?t.then(n,e=>Fn(e,M(ni,this)&&M(ni,this).parent)):n(t)}})}function J(e,t){var n=t==null?``:typeof t==`object`?`${t}`:t;n!==(e[me]??(e[me]=e.nodeValue))&&(e[me]=n,e.nodeValue=`${n}`)}function Ti(e,t){return Di(e,t)}var Ei=new Map;function Di(e,{target:t,anchor:n,props:i={},events:a,context:o,intro:s=!0,transformError:c}){En();var l=void 0,u=Un(()=>{var s=n??t.appendChild(Dn());Zr(s,{pending:()=>{}},t=>{Ue({});var n=Ve;if(o&&(n.c=o),a&&(i.$$events=a),T&&qr(t,null),l=e(t,i)||Ge(),T&&(H.nodes.end=E,E===null||E.nodeType!==8||E.data!==`]`))throw be(),ve;We()},c);var u=new Set,d=e=>{for(var n=0;n<e.length;n++){var r=e[n];if(!u.has(r)){u.add(r);var i=Pr(r);for(let e of[t,document]){var a=Ei.get(e);a===void 0&&(a=new Map,Ei.set(e,a));var o=a.get(r);o===void 0?(e.addEventListener(r,Ur,{passive:i}),a.set(r,1)):a.set(r,o+1)}}}};return d(r(Ir)),Lr.add(d),()=>{for(var e of u)for(let n of[t,document]){var r=Ei.get(n),i=r.get(e);--i==0?(n.removeEventListener(e,Ur),r.delete(e),r.size===0&&Ei.delete(n)):r.set(e,i)}Lr.delete(d),s!==n&&s.parentNode?.removeChild(s)}});return Oi.set(l,u),l}var Oi=new WeakMap,ki=new WeakMap,Ai=new WeakMap,ji=new WeakMap,Mi=new WeakMap,Ni=new WeakMap,Pi=new WeakMap,Fi=new WeakMap,Ii=class{constructor(e,t=!0){g(this,`anchor`,void 0),k(this,ki,new Map),k(this,Ai,new Map),k(this,ji,new Map),k(this,Mi,new Set),k(this,Ni,!0),k(this,Pi,e=>{if(M(ki,this).has(e)){var t=M(ki,this).get(e),n=M(Ai,this).get(t);if(n)nr(n),M(Mi,this).delete(t);else{var r=M(ji,this).get(t);r&&(nr(r.effect),M(Ai,this).set(t,r.effect),M(ji,this).delete(t),r.fragment.lastChild.remove(),this.anchor.before(r.fragment),n=r.effect)}for(let[t,n]of M(ki,this)){if(M(ki,this).delete(t),t===e)break;let r=M(ji,this).get(n);r&&(Zn(r.effect),M(ji,this).delete(n))}for(let[e,r]of M(Ai,this)){if(e===t||M(Mi,this).has(e))continue;let i=()=>{if(Array.from(M(ki,this).values()).includes(e)){var t=document.createDocumentFragment();ir(r,t),t.append(Dn()),M(ji,this).set(e,{effect:r,fragment:t})}else Zn(r);M(Mi,this).delete(e),M(Ai,this).delete(e)};M(Ni,this)||!n?(M(Mi,this).add(e),er(r,i,!1)):i()}}}),k(this,Fi,e=>{M(ki,this).delete(e);let t=Array.from(M(ki,this).values());for(let[e,n]of M(ji,this))t.includes(e)||(Zn(n.effect),M(ji,this).delete(e))}),this.anchor=e,j(Ni,this,t)}ensure(e,t){var n=N,r=jn();if(t&&!M(Ai,this).has(e)&&!M(ji,this).has(e)){if(r){var i=document.createDocumentFragment(),a=Dn();i.append(a),M(ji,this).set(e,{effect:qn(()=>t(a)),fragment:i})}else M(Ai,this).set(e,qn(()=>t(this.anchor)))}if(M(ki,this).set(n,e),r){for(let[t,r]of M(Ai,this))t===e?n.unskip_effect(r):n.skip_effect(r);for(let[t,r]of M(ji,this))t===e?n.unskip_effect(r.effect):n.skip_effect(r.effect);n.oncommit(M(Pi,this)),n.ondiscard(M(Fi,this))}else T&&(this.anchor=E),M(Pi,this).call(this,n)}};function Y(e,t,n=!1){var r;T&&(r=E,we());var i=new Ii(e),a=n?ne:0;function o(e,t){if(T){var n=De(r);if(e!==parseInt(n.substring(1))){var a=Ee();Ce(a),i.anchor=a,Se(!1),i.ensure(e,t),Se(!0);return}}i.ensure(e,t)}Kn(()=>{var e=!1;t((t,n=0)=>{e=!0,o(n,t)}),e||o(-1,null)},a)}var Li=Symbol(`NaN`);function Ri(e,t,n){T&&we();var r=new Ii(e),i=!Ke();Kn(()=>{var e=t();e!==e&&(e=Li),i&&typeof e==`object`&&e&&(e={}),r.ensure(e,n)})}function X(e,t){return t}function zi(e,t,n){for(var i=[],a=t.length,o,s=t.length,c=0;c<a;c++){let n=t[c];er(n,()=>{if(o){if(o.pending.delete(n),o.done.add(n),o.pending.size===0){var t=e.outrogroups;Bi(e,r(o.done)),t.delete(o),t.size===0&&(e.outrogroups=null)}}else--s},!1)}if(s===0){var l=i.length===0&&n!==null&&e.pending.size===0;if(l){var u=n,d=u.parentNode;An(d),d.append(u),e.items.clear()}Bi(e,t,!l)}else o={pending:new Set(t),done:new Set},(e.outrogroups??(e.outrogroups=new Set)).add(o)}function Bi(e,t,n=!0){var r;if(e.pending.size>0){r=new Set;for(let t of e.pending.values())for(let n of t)r.add(e.items.get(n).e)}for(var i=0;i<t.length;i++){var a=t[i];r?.has(a)?(a.f|=ie,ir(a,document.createDocumentFragment())):Zn(t[i],n)}}var Vi;function Z(t,n,i,a,o,s=null){var c=t,l=new Map;if(n&4){var u=t;c=T?Ce(On(u)):u.appendChild(Dn())}T&&we();var d=null,f=ft(()=>{var t=i();return e(t)?t:t==null?[]:r(t)}),p=new Map,m=!0;function h(e){if(!(g.effect.f&16384)){g.pending.delete(e);var t=U(f);g.fallback=d,Ui(g,t,c,n,a),d!==null&&(t.length===0?d.f&33554432?(d.f^=ie,Gi(d,null,c)):nr(d):er(d,()=>{d=null}))}}function ee(e){g.pending.delete(e)}var g={effect:Kn(()=>{var e=U(f),t=e.length;let r=!1;T&&De(c)===`[!`!=(t===0)&&(c=Ee(),Ce(c),Se(!1),r=!0);for(var u=new Set,g=N,_=jn(),v=0;v<t;v+=1){T&&E.nodeType===8&&E.data===`]`&&(c=E,r=!0,Se(!1));var y=e[v],b=a(y,v),x=m?null:l.get(b);x?(x.v&&_n(x.v,y),x.i&&_n(x.i,v),_&&g.unskip_effect(x.e)):(x=Wi(l,m?c:Vi??(Vi=Dn()),y,b,v,o,n,i),m||(x.e.f|=ie),l.set(b,x)),u.add(b)}if(t===0&&s&&!d&&(m?d=qn(()=>s(c)):(d=qn(()=>s(Vi??(Vi=Dn()))),d.f|=ie)),t>u.size&&Me(``,``,``),T&&t>0&&Ce(Ee()),!m){if(p.set(g,u),_){for(let[e,t]of l)u.has(e)||g.skip_effect(t.e);g.oncommit(h),g.ondiscard(ee)}else h(g)}r&&Se(!0),U(f)}),flags:n,items:l,pending:p,outrogroups:null,fallback:d};m=!1,T&&(c=E)}function Hi(e){for(;e!==null&&!(e.f&32);)e=e.next;return e}function Ui(e,t,n,i,a){var o=!!(i&8),s=t.length,c=e.items,l=Hi(e.effect.first),u,d=null,f,p=[],m=[],h,ee,g,_;if(o)for(_=0;_<s;_+=1)h=t[_],ee=a(h,_),g=c.get(ee).e,g.f&33554432||(g.nodes?.a?.measure(),(f??(f=new Set)).add(g));for(_=0;_<s;_+=1){if(h=t[_],ee=a(h,_),g=c.get(ee).e,e.outrogroups!==null)for(let t of e.outrogroups)t.pending.delete(g),t.done.delete(g);if(g.f&8192&&(nr(g),o&&(g.nodes?.a?.unfix(),(f??(f=new Set)).delete(g))),g.f&33554432){if(g.f^=ie,g===l)Gi(g,null,n);else{var v=d?d.next:l;g===e.effect.last&&(e.effect.last=g.prev),g.prev&&(g.prev.next=g.next),g.next&&(g.next.prev=g.prev),Ki(e,d,g),Ki(e,g,v),Gi(g,v,n),d=g,p=[],m=[],l=Hi(d.next);continue}}if(g!==l){if(u!==void 0&&u.has(g)){if(p.length<m.length){var y=m[0],b;d=y.prev;var x=p[0],te=p[p.length-1];for(b=0;b<p.length;b+=1)Gi(p[b],y,n);for(b=0;b<m.length;b+=1)u.delete(m[b]);Ki(e,x.prev,te.next),Ki(e,d,x),Ki(e,te,y),l=y,d=te,--_,p=[],m=[]}else u.delete(g),Gi(g,l,n),Ki(e,g.prev,g.next),Ki(e,g,d===null?e.effect.first:d.next),Ki(e,d,g),d=g;continue}for(p=[],m=[];l!==null&&l!==g;)(u??(u=new Set)).add(l),m.push(l),l=Hi(l.next);if(l===null)continue}g.f&33554432||p.push(g),d=g,l=Hi(g.next)}if(e.outrogroups!==null){for(let t of e.outrogroups)t.pending.size===0&&(Bi(e,r(t.done)),e.outrogroups?.delete(t));e.outrogroups.size===0&&(e.outrogroups=null)}if(l!==null||u!==void 0){var S=[];if(u!==void 0)for(g of u)g.f&8192||S.push(g);for(;l!==null;)!(l.f&8192)&&l!==e.fallback&&S.push(l),l=Hi(l.next);var ne=S.length;if(ne>0){var C=i&4&&s===0?n:null;if(o){for(_=0;_<ne;_+=1)S[_].nodes?.a?.measure();for(_=0;_<ne;_+=1)S[_].nodes?.a?.fix()}zi(e,S,C)}}o&&Ye(()=>{if(f!==void 0)for(g of f)g.nodes?.a?.apply()})}function Wi(e,t,n,r,i,a,o,s){var c=o&1?o&16?fn(n):mn(n,!1,!1):null,l=o&2?fn(i):null;return{v:c,i:l,e:qn(()=>(a(t,c??n,l??i,s),()=>{e.delete(r)}))}}function Gi(e,t,n){if(e.nodes)for(var r=e.nodes.start,i=e.nodes.end,a=t&&!(t.f&33554432)?t.nodes.start:n;r!==null;){var o=kn(r);if(a.before(r),r===i)return;r=o}}function Ki(e,t,n){t===null?e.effect.first=n:t.next=n,n===null?e.effect.last=t:n.prev=t}var qi=[...` 	
\r\f\xA0\v﻿`];function Ji(e,t,n){var r=e==null?``:``+e;if(t&&(r=r?r+` `+t:t),n){for(var i of Object.keys(n))if(n[i])r=r?r+` `+i:i;else if(r.length)for(var a=i.length,o=0;(o=r.indexOf(i,o))>=0;){var s=o+a;(o===0||qi.includes(r[o-1]))&&(s===r.length||qi.includes(r[s]))?r=(o===0?``:r.substring(0,o))+r.substring(s+1):o=s}}return r===``?null:r}function Q(e,t,n,r,i,a){var o=e[fe];if(T||o!==n||o===void 0){var s=Ji(n,r,a);(!T||s!==e.getAttribute(`class`))&&(s==null?e.removeAttribute(`class`):t?e.className=s:e.setAttribute(`class`,s)),e[fe]=n}else if(a&&i!==a)for(var c in a){var l=!!a[c];(i==null||l!==!!i[c])&&e.classList.toggle(c,l)}return a}var Yi=Symbol(`is custom element`),Xi=Symbol(`is html`),Zi=_e?`link`:`LINK`;function Qi(e){if(T){var t=!1,n=()=>{if(!t){if(t=!0,e.hasAttribute(`value`)){var n=e.value;ea(e,`value`,null),e.value=n}if(e.hasAttribute(`checked`)){var r=e.checked;ea(e,`checked`,null),e.checked=r}}};e[he]=n,Ye(n),tt()}}function $i(e,t){var n=ta(e);n.checked!==(n.checked=t??void 0)&&(e.checked=t)}function ea(e,t,n,r){var i=ta(e);T&&(i[t]=e.getAttribute(t),t===`src`||t===`srcset`||t===`href`&&e.nodeName===Zi)||i[t]!==(i[t]=n)&&(t===`loading`&&(e[ue]=n),n==null?e.removeAttribute(t):typeof n!=`string`&&ra(e).has(t)?e[t]=n:e.setAttribute(t,n))}function ta(e){return e[de]??(e[de]={[Yi]:e.nodeName.includes(`-`),[Xi]:e.namespaceURI===`http://www.w3.org/1999/xhtml`})}var na=new Map;function ra(e){var t=e.getAttribute(`is`)||e.nodeName,n=na.get(t);if(n)return n;na.set(t,n=new Set);for(var r,i=e,a=Element.prototype;a!==i;){for(var s in r=o(i),r)r[s].set&&s!==`innerHTML`&&s!==`textContent`&&s!==`innerText`&&n.add(s);i=l(i)}return n}function ia(e,t,n=t){var r=new WeakSet;rt(e,`input`,async i=>{var a=i?e.defaultValue:e.value;if(a=oa(e)?sa(a):a,n(a),N!==null&&r.add(N),await kr(),a!==(a=t())){var o=e.selectionStart,s=e.selectionEnd,c=e.value.length;if(e.value=a??``,s!==null){var l=e.value.length;o===s&&s===c&&l>c?(e.selectionStart=l,e.selectionEnd=l):(e.selectionStart=o,e.selectionEnd=Math.min(s,l))}}}),(T&&e.defaultValue!==e.value||Mr(t)==null&&e.value)&&(n(oa(e)?sa(e.value):e.value),N!==null&&r.add(N)),Gn(()=>{var n=t();if(e===document.activeElement){var i=N;if(r.has(i))return}oa(e)&&n===sa(e.value)||(e.type!==`date`||n||e.value)&&n!==e.value&&(e.value=n??``)})}function aa(e,t,n=t){rt(e,`change`,t=>{n(t?e.defaultChecked:e.checked)}),(T&&e.defaultChecked!==e.checked||Mr(t)==null)&&n(e.checked),Gn(()=>{e.checked=!!t()})}function oa(e){var t=e.type;return t===`number`||t===`range`}function sa(e){return e===``?null:+e}var ca,la=new WeakMap,ua=new WeakMap,da=new WeakMap,fa=new WeakSet,pa=class{constructor(e){yt(this,fa),k(this,la,new WeakMap),k(this,ua,void 0),k(this,da,void 0),j(da,this,e)}observe(e,t){var n=M(la,this).get(e)||new Set;return n.add(t),M(la,this).set(e,n),A(fa,this,ma).call(this).observe(e,M(da,this)),()=>{var n=M(la,this).get(e);n.delete(t),n.size===0&&(M(la,this).delete(e),M(ua,this).unobserve(e))}}};ca=pa;function ma(){return M(ua,this)??j(ua,this,new ResizeObserver(e=>{for(var t of e){ca.entries.set(t.target,t);for(var n of M(la,this).get(t.target)||[])n(t)}}))}if(g(pa,`entries`,new WeakMap),typeof window<`u`){var ha,ga;((ha=(ga=window).__svelte??(ga.__svelte={})).v??(ha.v=new Set)).add(`5`)}var _a=xn({data:null});function va(e){_a.data=e}function $(e,t={}){let n=JSON.stringify({...t,act:e});window.ipc&&typeof window.ipc.postMessage==`function`&&window.ipc.postMessage(n)}var ya={trusted:`trusted`,ok:`ok`,changed:`changed`,problem:`problem`,error:`error`,untrusted:`none`,off:`none`};function ba(e){return typeof e==`string`&&Object.hasOwn(ya,e)&&ya[e]||`none`}function xa(e){return`d`+Math.min(3,Math.max(0,Number.isInteger(e)?e:0))}var Sa={rebuilt:`rebuilt`,partly:`partly`,left:`left`};function Ca(e){return typeof e==`string`&&Object.hasOwn(Sa,e)&&Sa[e]||`left`}var wa=G(`<p> </p>`),Ta=G(`<span aria-hidden="true"></span>`),Ea=G(`<span class="hint"> </span>`),Da=G(`<span class="badge" title="Waiting for your answer"> </span>`),Oa=G(`<button><!> <span class="words"><span class="label"> </span> <!></span> <!></button>`),ka=G(`<button><span class="words"><span class="hint"> </span></span></button>`),Aa=G(`<nav aria-label="The parts of the Development window"></nav>`);function ja(e,t){Ue(t,!0);var n=Aa();Z(n,21,()=>t.rows,X,(e,t)=>{var n=Jr(),r=L(n),i=e=>{var n=wa(),r=R(n,!0);B(e=>{Q(n,1,`navhead ${e??``}`),J(r,U(t).label)},[()=>xa(U(t).depth)]),K(e,n)},a=e=>{var n=Oa();let r;var i=I(n),a=e=>{var n=Ta();B(e=>Q(n,1,`dot ${e??``}`),[()=>ba(U(t).mark)]),K(e,n)};Y(i,e=>{U(t).mark&&e(a)});var o=z(i,2),s=I(o),c=R(s,!0),l=z(s,2),u=e=>{var n=Ea(),r=R(n,!0);B(()=>J(r,U(t).hint)),K(e,n)};Y(l,e=>{U(t).hint&&e(u)}),D(o);var d=z(o,2),f=e=>{var n=Da(),r=R(n,!0);B(()=>J(r,U(t).badge)),K(e,n)};Y(d,e=>{U(t).badge&&e(f)}),D(n),B(e=>{r=Q(n,1,`navitem ${e??``}`,null,r,{selected:U(t).selected}),ea(n,`aria-current`,U(t).selected?`page`:void 0),J(c,U(t).label)},[()=>xa(U(t).depth)]),W(`click`,n,()=>$(`go`,{to:U(t).go})),K(e,n)},o=e=>{var n=ka(),r=I(n),i=R(I(r),!0);D(r),D(n),B(e=>{Q(n,1,`navitem more ${e??``}`),J(i,U(t).label)},[()=>xa(U(t).depth)]),W(`click`,n,()=>$(`go`,{to:U(t).go})),K(e,n)};Y(r,e=>{U(t).t===`heading`?e(i):U(t).t===`item`?e(a,1):U(t).t===`more`&&e(o,2)}),K(e,n)}),D(n),K(e,n),We()}Br([`click`]);var Ma=G(`<span class="chip"> </span>`),Na=G(`<p> </p>`),Pa=G(`<p class="small"> </p>`),Fa=G(`<p class="warn"> </p>`),Ia=G(`<p> </p> <p> </p> <!> <!> <!>`,1),La=G(`<section class="box"><div class="boxhead"><h2 class="boxtitle"> </h2> <!> <span class="chip good"> </span> <span class="grow"></span> <button title="Its projects, and what it needs to run them">Open</button></div> <p> </p> <!></section>`),Ra=G(`<h2 class="boxtitle"> </h2>`),za=G(`<button title="Read the macros this workbook carries. GridCraft does not run them.">Macros…</button>`),Ba=G(`<header class="head"><h1 class="title"> </h1> <p class="lede"> </p></header> <section class="box"><div class="boxhead"><span aria-hidden="true"></span> <h2 class="boxtitle"> </h2> <!> <span class="grow"></span> <button title="How far agents may go, how they connect, and what they ask for">Open</button></div> <!></section> <h2 class="sect"> </h2> <!> <p class="small"> </p> <h2 class="sect">Open workbook</h2> <section class="box" aria-label="Open workbook"><div class="boxhead"><!> <span class="grow"></span> <!></div> <p> </p> <!></section> <div class="row"><button title="What your projects print and log, what failed, and what agents asked for">Console…</button> <span class="small"> </span></div>`,1);function Va(e,t){Ue(t,!0);let n=dt(()=>t.pane.server),r=dt(()=>t.pane.workbook);var i=Ba(),a=L(i),o=I(a),s=R(o,!0),c=R(z(o,2),!0);D(a);var l=z(a,2),u=I(l),d=I(u),f=z(d,2),p=R(f,!0),m=z(f,2),h=e=>{var t=Ma(),r=R(t,!0);B(()=>J(r,U(n).level)),K(e,t)};Y(m,e=>{U(n).available&&e(h)});var ee=z(m,4);D(u);var g=z(u,2),_=e=>{var t=Na(),r=R(t,!0);B(()=>J(r,U(n).unavailable)),K(e,t)},v=e=>{var t=Ia(),r=L(t),i=R(r,!0),a=z(r,2),o=R(a,!0),s=z(a,2),c=e=>{var t=Pa(),r=R(t,!0);B(()=>J(r,U(n).clients)),K(e,t)};Y(s,e=>{U(n).clients&&e(c)});var l=z(s,2),u=e=>{var t=Fa(),r=R(t,!0);B(()=>J(r,U(n).waiting)),K(e,t)};Y(l,e=>{U(n).waiting&&e(u)});var d=z(l,2),f=e=>{var t=Pa(),r=R(t,!0);B(()=>J(r,U(n).plan)),K(e,t)};Y(d,e=>{U(n).plan&&e(f)}),B(e=>{J(i,U(n).says),Q(a,1,`status ${e??``}`),J(o,U(n).status.text)},[()=>ba(U(n).status.kind)]),K(e,t)};Y(g,e=>{U(n).available?e(v,-1):e(_)}),D(l);var y=z(l,2),b=R(y,!0),x=z(y,2);Z(x,17,()=>t.pane.kits,X,(e,t)=>{var n=La(),r=I(n),i=I(r),a=R(i,!0),o=z(i,2);Z(o,17,()=>U(t).madeOf,X,(e,t)=>{var n=Ma(),r=R(n,!0);B(()=>J(r,U(t))),K(e,n)});var s=z(o,2),c=R(s,!0),l=z(s,4);D(r);var u=z(r,2),d=R(u,!0),f=z(u,2),p=e=>{var n=Pa(),r=R(n,!0);B(()=>J(r,U(t).projects)),K(e,n)};Y(f,e=>{U(t).projects&&e(p)}),D(n),B(()=>{ea(n,`aria-label`,U(t).name),J(a,U(t).name),J(c,U(t).standing),ea(l,`aria-label`,`Open ${U(t).name??``}`),J(d,U(t).says)}),W(`click`,l,()=>$(`go`,{to:U(t).go})),K(e,n)});var te=z(x,2),S=R(te,!0),ne=z(te,4),C=I(ne),re=I(C),ie=e=>{var t=Ra(),n=R(t,!0);B(()=>J(n,U(r).title)),K(e,t)};Y(re,e=>{U(r).title&&e(ie)});var ae=z(re,4),oe=e=>{var t=za();W(`click`,t,()=>$(`open`,{name:`macros`})),K(e,t)};Y(ae,e=>{U(r).macros&&e(oe)}),D(C);var se=z(C,2);let ce;var le=R(se,!0),ue=z(se,2),de=e=>{var t=Pa(),n=R(t,!0);B(()=>J(n,U(r).rebuilt)),K(e,t)};Y(ue,e=>{U(r).rebuilt&&e(de)}),D(ne);var fe=z(ne,2),pe=I(fe),me=R(z(pe,2),!0);D(fe),B(e=>{J(s,t.pane.title),J(c,t.pane.intro),ea(l,`aria-label`,U(n).name),Q(d,1,`dot ${e??``}`),J(p,U(n).name),ea(ee,`aria-label`,`Open ${U(n).name??``}`),J(b,t.pane.kitsTitle),J(S,t.pane.kitsNote),ce=Q(se,1,``,null,ce,{warn:U(r).problem}),J(le,U(r).says),J(me,t.pane.console)},[()=>ba(U(n).available?U(n).status.kind:`off`)]),W(`click`,ee,()=>$(`go`,{to:U(n).go})),W(`click`,pe,()=>$(`open`,{name:`console`})),K(e,i),We()}Br([`click`]);var Ha=G(`<p class="lede"> </p>`),Ua=G(`<p> </p>`),Wa=G(`<div class="item waits"><span class="strong"> </span> <code> </code> <span class="grow"></span> <span class="small"> </span> <button class="primary" title="See everything this request would do, and answer it">Look…</button></div>`),Ga=G(`<h2 class="sect"> </h2> <!>`,1),Ka=G(`<label class="level"><input type="radio" name="level"/> <span> </span></label>`),qa=G(`<p class="small"> </p>`),Ja=G(`<div class="row"><button title="Copies the command that adds this server to Claude Code. It holds the token.">Copy Setup for Claude Code</button> <button title="Copies an MCP settings entry, for other agents. It holds the token.">Copy Setup as JSON</button> <button title="Makes a new token: an agent that holds the old one can no longer connect">New Token</button></div>`),Ya=G(`<button title="None of its steps is carried out after this. What was done stays done.">Stop Plan</button>`),Xa=G(`<li><span> </span><span class="state"> </span></li>`),Za=G(`<div class="item plan"><div class="head"><span class="strong"> </span> <span> </span> <span class="grow"></span> <span class="small"> </span> <!></div> <ol></ol></div>`),Qa=G(`<h2 class="sect">Plans</h2> <!>`,1),$a=G(`<div class="row"><span> </span> <button class="link">Ask again</button></div>`),eo=G(`<h2 class="sect">Not asked again until GridCraft closes</h2> <!>`,1),to=G(`<h2 class="sect">Lately</h2> <!> <p class="small"> </p>`,1),no=G(`<!> <h2 class="sect">How far agents may go</h2> <section class="box"><div class="levels" role="radiogroup" aria-label="How far agents may go"></div> <p class="small"> </p></section> <h2 class="sect">Connection</h2> <section class="box"><p> </p> <!> <div class="row"><label for="port">Port:</label> <input id="port" class="port" type="text" inputmode="numeric" maxlength="5"/> <button>Set Port</button></div> <!></section> <!> <!> <!>`,1),ro=G(`<header class="head"><h1 class="title"> </h1> <!></header> <!>`,1);function io(e,t){Ue(t,!0);let n=pn(``),r=pn(!1);Vn(()=>{!U(r)&&t.pane.available&&F(n,t.pane.portText,!0)});function i(e,n){e.preventDefault(),n!==t.pane.level&&$(`level`,{level:n})}function a(){$(`port`,{port:U(n)}),F(r,!1)}var o=ro(),s=L(o),c=I(s),l=R(c,!0),u=z(c,2),d=e=>{var n=Ha(),r=R(n,!0);B(()=>J(r,t.pane.intro)),K(e,n)};Y(u,e=>{t.pane.available&&e(d)}),D(s);var f=z(s,2),p=e=>{var n=Ua(),r=R(n,!0);B(()=>J(r,t.pane.unavailable)),K(e,n)},m=e=>{var o=no(),s=L(o),c=e=>{var n=Ga(),r=L(n),i=R(r,!0);Z(z(r,2),17,()=>t.pane.waiting,X,(e,t)=>{var n=Wa(),r=I(n),i=R(r,!0),a=z(r,2),o=R(a,!0),s=z(a,4),c=R(s,!0),l=z(s,2);D(n),B(()=>{J(i,U(t).kind),J(o,U(t).target),J(c,U(t).named)}),W(`click`,l,()=>$(`look`,{id:U(t).id})),K(e,n)}),B(()=>J(i,t.pane.waitingTitle)),K(e,n)};Y(s,e=>{t.pane.waiting.length&&e(c)});var l=z(s,4),u=I(l);Z(u,21,()=>t.pane.levels,X,(e,n)=>{var r=Ka(),a=I(r);Qi(a);var o=R(z(a,2),!0);D(r),B(()=>{ea(r,`title`,U(n).meaning),$i(a,U(n).code===t.pane.level),J(o,U(n).label)}),W(`click`,a,e=>i(e,U(n).code)),K(e,r)}),D(u);var d=R(z(u,2),!0);D(l);var f=z(l,4),p=I(f),m=R(p,!0),h=z(p,2),ee=e=>{var n=qa(),r=R(n,!0);B(()=>J(r,t.pane.clients)),K(e,n)};Y(h,e=>{t.pane.clients&&e(ee)});var g=z(h,2),_=z(I(g),2);Qi(_);var v=z(_,2);D(g);var y=z(g,2),b=e=>{var t=Ja(),n=I(t),r=z(n,2),i=z(r,2);D(t),W(`click`,n,()=>$(`copySetup`,{json:!1})),W(`click`,r,()=>$(`copySetup`,{json:!0})),W(`click`,i,()=>$(`newToken`)),K(e,t)};Y(y,e=>{t.pane.serving&&e(b)}),D(f);var x=z(f,2),te=e=>{var n=Qa();Z(z(L(n),2),17,()=>t.pane.plans,X,(e,t)=>{var n=Za(),r=I(n),i=I(r),a=R(i,!0),o=z(i,2),s=R(o,!0),c=z(o,4),l=R(c,!0),u=z(c,2),d=e=>{var n=Ya();W(`click`,n,()=>$(`stopPlan`,{id:U(t).id})),K(e,n)};Y(u,e=>{U(t).live&&e(d)}),D(r);var f=z(r,2);Z(f,21,()=>U(t).steps,X,(e,t)=>{var n=Xa();let r;var i=I(n),a=R(i,!0),o=R(z(i),!0);D(n),B(()=>{r=Q(n,1,`small`,null,r,{done:U(t).state===`done`,failed:U(t).state===`failed`,running:U(t).state===`running`}),J(a,U(t).what),J(o,U(t).stands)}),K(e,n)}),D(f),D(n),B(()=>{J(a,U(t).name),J(s,U(t).title),J(l,U(t).stands)}),K(e,n)}),K(e,n)};Y(x,e=>{t.pane.plans.length&&e(te)});var S=z(x,2),ne=e=>{var n=eo();Z(z(L(n),2),17,()=>t.pane.unasked,X,(e,t)=>{var n=$a(),r=I(n),i=R(r,!0),a=z(r,2);D(n),B(()=>J(i,U(t).covers)),W(`click`,a,()=>$(`askAgain`,{module:U(t).module,kind:U(t).kind})),K(e,n)}),K(e,n)};Y(S,e=>{t.pane.unasked.length&&e(ne)});var C=z(S,2),re=e=>{var n=to(),r=z(L(n),2);Z(r,17,()=>t.pane.lately,X,(e,t)=>{var n=qa(),r=R(n,!0);B(()=>J(r,U(t))),K(e,n)});var i=R(z(r,2),!0);B(()=>J(i,t.pane.latelyNote)),K(e,n)};Y(C,e=>{t.pane.lately.length&&e(re)}),B(e=>{J(d,t.pane.meaning),Q(p,1,`status ${e??``}`),J(m,t.pane.status.text)},[()=>ba(t.pane.status.kind)]),W(`input`,_,()=>F(r,!0)),ia(_,()=>U(n),e=>F(n,e)),W(`click`,v,a),K(e,o)};Y(f,e=>{t.pane.available?e(m,-1):e(p)}),B(()=>J(l,t.pane.title)),K(e,o),We()}Br([`click`,`input`]);var ao=G(`<span class="chip"> </span>`),oo=G(`<p> </p>`),so=G(`<button>Open Folder</button>`),co=G(`<p class="small empty"> </p>`),lo=G(`<span class="hint"> </span>`),uo=G(`<button class="entry" title="What it is for, what it asks, and what is in it"><span aria-hidden="true"></span> <span class="words"><span class="line1"><span class="name"> </span> <!></span> <!></span> <span> </span> <span class="chevron" aria-hidden="true">›</span></button>`),fo=G(`<div class="list"></div>`),po=G(`<div class="grouphead"><h3> </h3> <span class="small"> </span></div> <!>`,1),mo=G(`<label class="tick"><input type="checkbox"/> <span>It is the code of <strong> </strong>, the workbook that is open</span></label>`),ho=G(`<button title="Creates the Svelte Showcase project. It does nothing until you trust it.">Create Showcase</button>`),go=G(`<button title="Shows its panel inside GridCraft's window, beside the sheet">Dock Showcase</button>`),_o=G(`<button>Open in Browser</button>`),vo=G(`<!> <!>`,1),yo=G(`<section class="box facts"><div class="fact"><span class="small">Projects</span> <span> </span> <button title="Review, trust and run your projects; name the Python that runs their scripts">Modules…</button></div> <div class="fact"><span class="small">Console</span> <span> </span> <button title="What your projects print and log, what failed, and what agents asked for">Console…</button></div> <div class="fact wide"><span class="small">Runs with</span> <span> </span></div> <div class="fact wide"><span class="small">Projects folder</span> <code class="path"> </code> <!></div></section> <h2 class="sect"> </h2> <p class="small"> </p> <!> <h2 class="sect">New project</h2> <form class="box"><div class="row"><label for="newname">Name:</label> <input id="newname" class="name" type="text" maxlength="80" placeholder="Its name"/> <button class="primary" type="submit" title="Creates a starter project. It does nothing until you trust it.">Create Project</button></div> <!></form> <h2 class="sect">Showcase</h2> <div class="row"><span> </span> <!></div>`,1),bo=G(`<header class="head"><div class="titled"><h1 class="title"> </h1> <!> <span class="chip good"> </span></div> <p class="lede"> </p></header> <!>`,1);function xo(e,t){Ue(t,!0);let n=pn(``),r=pn(!1);function i(e){e.preventDefault(),$(`createProject`,{name:U(n),forWorkbook:U(r)&&!!t.pane.newFor}),F(n,``),F(r,!1)}var a=bo(),o=L(a),s=I(o),c=I(s),l=R(c,!0),u=z(c,2);Z(u,17,()=>t.pane.madeOf,X,(e,t)=>{var n=ao(),r=R(n,!0);B(()=>J(r,U(t))),K(e,n)});var d=R(z(u,2),!0);D(s);var f=R(z(s,2),!0);D(o);var p=z(o,2),m=e=>{var n=oo(),r=R(n,!0);B(()=>J(r,t.pane.unavailable)),K(e,n)},h=e=>{var a=yo(),o=L(a),s=I(o),c=z(I(s),2),l=R(c,!0),u=z(c,2);D(s);var d=z(s,2),f=z(I(d),2),p=R(f,!0),m=z(f,2);D(d);var h=z(d,2),ee=R(z(I(h),2),!0);D(h);var g=z(h,2),_=z(I(g),2),v=R(_,!0),y=z(_,2),b=e=>{var t=so();W(`click`,t,()=>$(`openFolder`)),K(e,t)};Y(y,e=>{t.pane.canOpenFolder&&e(b)}),D(g),D(o);var x=z(o,2),te=R(x,!0),S=z(x,2),ne=R(S,!0),C=z(S,2);Z(C,17,()=>t.pane.groups,X,(e,t)=>{var n=po(),r=L(n),i=I(r),a=R(i,!0),o=R(z(i,2),!0);D(r);var s=z(r,2),c=e=>{var n=co(),r=R(n,!0);B(()=>J(r,U(t).empty)),K(e,n)},l=e=>{var n=fo();Z(n,21,()=>U(t).projects,X,(e,t)=>{var n=uo(),r=I(n),i=z(r,2),a=I(i),o=I(a),s=R(o,!0),c=z(o,2),l=e=>{var n=ao(),r=R(n);B(()=>J(r,`${U(t).workbook??``}${U(t).open?`  ·  open`:``}`)),K(e,n)};Y(c,e=>{U(t).workbook&&e(l)}),D(a);var u=z(a,2),d=e=>{var n=lo(),r=R(n,!0);B(()=>J(r,U(t).says)),K(e,n)};Y(u,e=>{U(t).says&&e(d)}),D(i);var f=z(i,2),p=R(f,!0);Te(2),D(n),B((e,i)=>{n.disabled=!U(t).go,Q(r,1,`dot ${e??``}`),J(s,U(t).name),Q(f,1,`standing ${i??``}`),J(p,U(t).statusLabel)},[()=>ba(U(t).status),()=>ba(U(t).status)]),W(`click`,n,()=>$(`go`,{to:U(t).go})),K(e,n)}),D(n),K(e,n)};Y(s,e=>{U(t).projects.length===0?e(c):e(l,-1)}),B(()=>{J(a,U(t).label),J(o,U(t).says)}),K(e,n)});var re=z(C,4),ie=I(re),ae=z(I(ie),2);Qi(ae);var oe=z(ae,2);D(ie);var se=z(ie,2),ce=e=>{var n=mo(),i=I(n);Qi(i);var a=z(i,2),o=R(z(I(a)),!0);Te(),D(a),D(n),B(()=>J(o,t.pane.newFor)),aa(i,()=>U(r),e=>F(r,e)),K(e,n)};Y(se,e=>{t.pane.newFor&&e(ce)}),D(re);var le=z(re,4),ue=I(le),de=R(ue,!0),fe=z(ue,2),pe=e=>{var t=ho();W(`click`,t,()=>$(`createShowcase`)),K(e,t)},me=e=>{var n=vo(),r=L(n),i=e=>{var t=go();W(`click`,t,()=>$(`showShowcase`,{docked:!0})),K(e,t)};Y(r,e=>{t.pane.showcase.canDock&&e(i)});var a=z(r,2),o=e=>{var t=_o();W(`click`,t,()=>$(`showShowcase`,{docked:!1})),K(e,t)};Y(a,e=>{t.pane.showcase.canBrowse&&e(o)}),K(e,n)};Y(fe,e=>{t.pane.showcase.stands===`none`?e(pe):t.pane.showcase.stands===`trusted`&&e(me,1)}),D(le),B(e=>{J(l,t.pane.projectsSays),J(p,t.pane.console),J(ee,t.pane.python),ea(_,`title`,t.pane.folder),J(v,t.pane.folder),J(te,t.pane.projectsTitle),J(ne,t.pane.projectsNote),oe.disabled=e,J(de,t.pane.showcase.text)},[()=>!U(n).trim()]),W(`click`,u,()=>$(`open`,{name:`modules`})),W(`click`,m,()=>$(`open`,{name:`console`})),zr(`submit`,re,i),ia(ae,()=>U(n),e=>F(n,e)),K(e,a)};Y(p,e=>{t.pane.available?e(h,-1):e(m)}),B(()=>{J(l,t.pane.title),J(d,t.pane.standing),J(f,t.pane.says)}),K(e,a),We()}Br([`click`]);var So=G(`<span class="small"> </span>`),Co=G(`<p class="lede"> </p>`),wo=G(`<p class="error" role="alert"> </p>`),To=G(`<p> </p>`),Eo=G(`<p class="bullet"> </p>`),Do=G(`<dt class="small"> </dt> <dd><code> </code></dd>`,1),Oo=G(`<span class="arrow" aria-hidden="true">→</span><code> </code>`,1),ko=G(`<span class="hint"> </span>`),Ao=G(`<span class="hint warn"> </span>`),jo=G(`<div class="entry still"><span class="words"><span class="line1"><code> </code> <!></span> <!> <!></span> <span> </span></div>`),Mo=G(`<div class="list plain"></div>`),No=G(`<h2 class="sect"> </h2> <!> <!>`,1),Po=G(`<button>Open Project Folder</button>`),Fo=G(`<header class="head"><div class="titled"><span aria-hidden="true"></span> <h1 class="title"> </h1> <!> <!> <span class="grow"></span> <span class="chip"> </span> <span> </span></div> <!></header> <p> </p> <p class="small"> </p> <!> <h2 class="sect"> </h2> <section class="box"></section> <h2 class="sect">What is in it</h2> <dl class="parts"></dl> <!> <h2 class="sect">Manage</h2> <p class="small"> </p> <div class="row"><button title="Review, trust and run your projects">Modules…</button> <!></div>`,1);function Io(e,t){Ue(t,!0);var n=Fo(),r=L(n),i=I(r),a=I(i),o=z(a,2),s=R(o,!0),c=z(o,2),l=e=>{var n=So(),r=R(n,!0);B(()=>J(r,t.pane.version)),K(e,n)};Y(c,e=>{t.pane.version&&e(l)});var u=z(c,2),d=e=>{var n=So(),r=R(n);B(()=>J(r,`by ${t.pane.author??``}`)),K(e,n)};Y(u,e=>{t.pane.author&&e(d)});var f=z(u,4),p=R(f,!0),m=z(f,2),h=R(m,!0);D(i);var ee=z(i,2),g=e=>{var n=Co(),r=R(n,!0);B(()=>J(r,t.pane.description)),K(e,n)};Y(ee,e=>{t.pane.description&&e(g)}),D(r);var _=z(r,2),v=R(_,!0),y=z(_,2),b=R(y,!0),x=z(y,2),te=e=>{var n=wo(),r=R(n);B(()=>J(r,`This project can't be used: ${t.pane.problem??``}.`)),K(e,n)};Y(x,e=>{t.pane.problem&&e(te)});var S=z(x,2),ne=R(S,!0),C=z(S,2);Z(C,21,()=>t.pane.asks,X,(e,t)=>{var n=To();let r;var i=R(n,!0);B(()=>{r=Q(n,1,`bullet`,null,r,{warn:U(t).warn}),J(i,U(t).text)}),K(e,n)},e=>{var n=Eo(),r=R(n,!0);B(()=>J(r,t.pane.asksNothing)),K(e,n)}),D(C);var re=z(C,4);Z(re,21,()=>t.pane.parts,X,(e,t)=>{var n=Do(),r=L(n),i=R(r,!0),a=z(r,2),o=R(I(a),!0);D(a),B(()=>{J(i,U(t).label),J(o,U(t).value)}),K(e,n)}),D(re);var ie=z(re,2),ae=e=>{var n=No(),r=L(n),i=R(r,!0),a=z(r,2),o=e=>{var n=To();let r;var i=R(n,!0);B(()=>{r=Q(n,1,`small`,null,r,{warn:t.pane.rebuilt.problem}),J(i,t.pane.rebuilt.says)}),K(e,n)};Y(a,e=>{t.pane.rebuilt.says&&e(o)});var s=z(a,2),c=e=>{var n=Mo();Z(n,21,()=>t.pane.rebuilt.rows,X,(e,n)=>{var r=jo(),i=I(r),a=I(i),o=I(a),s=R(o,!0),c=z(o,2),l=e=>{var t=Oo(),r=R(z(L(t)),!0);B(()=>J(r,U(n).script)),K(e,t)};Y(c,e=>{U(n).script&&e(l)}),D(a);var u=z(a,2),d=e=>{var t=ko(),r=R(t,!0);B(()=>J(r,U(n).note)),K(e,t)};Y(u,e=>{U(n).note&&e(d)});var f=z(u,2),p=e=>{var n=Ao(),r=R(n,!0);B(()=>J(r,t.pane.rebuilt.missing)),K(e,n)};Y(f,e=>{U(n).scriptMissing&&e(p)}),D(i);var m=z(i,2),h=R(m,!0);D(r),B(e=>{J(s,U(n).from),Q(m,1,`chip state ${e??``}`),J(h,U(n).stateLabel)},[()=>Ca(U(n).state)]),K(e,r)}),D(n),K(e,n)};Y(s,e=>{t.pane.rebuilt.rows.length&&e(c)}),B(()=>J(i,t.pane.rebuilt.title)),K(e,n)};Y(ie,e=>{t.pane.rebuilt&&e(ae)});var oe=z(ie,4),se=R(oe,!0),ce=z(oe,2),le=I(ce),ue=z(le,2),de=e=>{var n=Po();W(`click`,n,()=>$(`openProjectFolder`,{id:t.pane.id})),K(e,n)};Y(ue,e=>{t.pane.canOpenFolder&&e(de)}),D(ce),B((e,n)=>{Q(a,1,`dot big ${e??``}`),J(s,t.pane.title),J(p,t.pane.kindLabel),Q(m,1,`chip standing ${n??``}`),J(h,t.pane.statusLabel),J(v,t.pane.forSays),J(b,t.pane.statusSays),J(ne,t.pane.asksTitle),J(se,t.pane.manage)},[()=>ba(t.pane.status),()=>ba(t.pane.status)]),W(`click`,le,()=>$(`open`,{name:`modules`})),K(e,n),We()}Br([`click`]);var Lo=G(`<p> </p>`),Ro=G(`<p class="warn"> </p>`),zo=G(`<p class="small"> </p>`),Bo=G(`<p class="strong"> </p>`),Vo=G(`<button class="link"> </button>`),Ho=G(`<p class="small toggle"><span> </span> <!></p>`),Uo=G(`<span class="no"></span>`),Wo=G(`<div class="line"><!><span class="src"> </span></div>`),Go=G(`<div class="code" role="group" aria-label="The text"></div> <!>`,1),Ko=G(`<div class="line skip"><span class="no"></span><span class="mark"></span><span class="src"> </span></div>`),qo=G(`<div><span class="no"> </span><span class="mark"> </span><span class="src"> </span></div>`),Jo=G(`<div class="code" role="group" aria-label="The changes"></div> <!>`,1),Yo=G(`<section class="step"><h2><span> </span> <code> </code></h2> <!></section>`);function Xo(e,t){Ue(t,!0);let n={gone:`-`,new:`+`,same:` `};var r=Jr();Z(L(r),17,()=>t.blocks,X,(e,t)=>{var r=Jr(),i=L(r),a=e=>{var n=Lo(),r=R(n,!0);B(()=>J(r,U(t).text)),K(e,n)},o=e=>{var n=Ro(),r=R(n,!0);B(()=>J(r,U(t).text)),K(e,n)},s=e=>{var n=zo(),r=R(n,!0);B(()=>J(r,U(t).text)),K(e,n)},c=e=>{var n=Bo(),r=R(n,!0);B(()=>J(r,U(t).text)),K(e,n)},l=e=>{var n=Lo();let r;var i=R(n,!0);B(()=>{r=Q(n,1,`bullet`,null,r,{warn:U(t).warn}),J(i,U(t).text)}),K(e,n)},u=e=>{var n=Ho(),r=I(n),i=R(r,!0),a=z(r,2),o=e=>{var n=Vo(),r=R(n,!0);B(()=>J(r,U(t).button)),W(`click`,n,()=>$(`whole`,{whole:U(t).whole})),K(e,n)};Y(a,e=>{U(t).button&&e(o)}),D(n),B(()=>J(i,U(t).text)),K(e,n)},d=e=>{var n=Go(),r=L(n);Z(r,21,()=>U(t).lines,X,(e,n,r)=>{var i=Wo(),a=I(i),o=e=>{var t=Uo();t.textContent=r+1,K(e,t)};Y(a,e=>{U(t).numbered&&e(o)});var s=R(z(a),!0);D(i),B(()=>J(s,U(n))),K(e,i)}),D(r);var i=z(r,2),a=e=>{var n=Ro(),r=R(n,!0);B(()=>J(r,U(t).cut)),K(e,n)};Y(i,e=>{U(t).cut&&e(a)}),K(e,n)},f=e=>{var r=Jo(),i=L(r);Z(i,21,()=>U(t).rows,X,(e,t)=>{var r=Jr(),i=L(r),a=e=>{var n=Ko(),r=R(z(I(n),2));D(n),B(()=>J(r,`⋯ ${U(t).text??``}`)),K(e,n)},o=e=>{var r=qo(),i=I(r),a=R(i,!0),o=z(i),s=R(o,!0),c=R(z(o),!0);D(r),B(()=>{Q(r,1,`line ${U(t).k??``}`),J(a,U(t).n),J(s,n[U(t).k]??` `),J(c,U(t).text)}),K(e,r)};Y(i,e=>{U(t).k===`skip`?e(a):e(o,-1)}),K(e,r)}),D(i);var a=z(i,2),o=e=>{var n=Ro(),r=R(n,!0);B(()=>J(r,U(t).cut)),K(e,n)};Y(a,e=>{U(t).cut&&e(o)}),K(e,r)},p=e=>{var n=Yo(),r=I(n),i=I(r),a=R(i,!0),o=R(z(i,2),!0);D(r),Xo(z(r,2),{get blocks(){return U(t).blocks}}),D(n),B(()=>{J(a,U(t).heading),J(o,U(t).target)}),K(e,n)};Y(i,e=>{U(t).t===`note`?e(a):U(t).t===`warn`?e(o,1):U(t).t===`small`?e(s,2):U(t).t===`strong`?e(c,3):U(t).t===`bullet`?e(l,4):U(t).t===`toggle`?e(u,5):U(t).t===`text`?e(d,6):U(t).t===`diff`?e(f,7):U(t).t===`step`&&e(p,8)}),K(e,r)}),K(e,r),We()}Br([`click`]);var Zo=G(`<p class="warn"> </p>`),Qo=G(`<button class="choice"> </button>`),$o=G(`<section class="card"><header><h1> </h1> <span class="named"> </span></header> <div><p class="target"> </p> <!></div> <div class="means"><!></div> <p class="question"> </p> <!> <div class="choices"><button class="choice"> </button> <!> <div class="refuse"><button class="choice"> </button> <input type="text" maxlength="400"/></div> <button class="choice quiet"> </button></div> <p class="small"> </p></section>`);function es(e,t){Ue(t,!0);let n=pn(``),r=pn(!1);Vn(()=>{let e=t.card.readyIn;if(e<=0){F(r,!0);return}let n=setTimeout(()=>F(r,!0),e);return()=>clearTimeout(n)});let i=dt(()=>U(r)&&!t.card.blocked);var a=$o(),o=I(a),s=I(o),c=R(s,!0),l=R(z(s,2),!0);D(o);var u=z(o,2);let d;var f=I(u),p=R(f,!0);Xo(z(f,2),{get blocks(){return t.card.content}}),D(u);var m=z(u,2);Xo(I(m),{get blocks(){return t.card.means}}),D(m);var h=z(m,2),ee=R(h,!0),g=z(h,2),_=e=>{var n=Zo(),r=R(n,!0);B(()=>J(r,t.card.blockedWhy)),K(e,n)};Y(g,e=>{t.card.blockedWhy&&e(_)});var v=z(g,2),y=I(v),b=R(y,!0),x=z(y,2),te=e=>{var n=Qo(),r=R(n,!0);B(()=>{n.disabled=!U(i),J(r,t.card.yesAnd)}),W(`click`,n,()=>$(`allowNotAgain`,{id:t.card.id})),K(e,n)};Y(x,e=>{t.card.yesAnd&&e(te)});var S=z(x,2),ne=I(S),C=R(ne,!0),re=z(ne,2);Qi(re),D(S);var ie=z(S,2),ae=R(ie,!0);D(v);var oe=R(z(v,2),!0);D(a),B(()=>{ea(a,`aria-label`,t.card.kind),J(c,t.card.kind),J(l,t.card.named),d=Q(u,1,`block`,null,d,{long:t.card.long}),J(p,t.card.target),J(ee,t.card.question),y.disabled=!U(i),J(b,t.card.yes),J(C,t.card.no),ea(re,`placeholder`,t.card.reasonHint),ea(re,`aria-label`,t.card.reasonHint),J(ae,t.card.notNow),J(oe,t.card.asker)}),W(`click`,y,()=>$(`allow`,{id:t.card.id})),W(`click`,ne,()=>$(`refuse`,{id:t.card.id,reason:U(n)})),ia(re,()=>U(n),e=>F(n,e)),W(`click`,ie,()=>$(`notNow`)),K(e,a),We()}Br([`click`]);var ts=G(`<main class="waiting"><p>Waiting for GridCraft…</p></main>`),ns=G(`<p class="error" role="alert"> </p>`),rs=G(`<p class="said" role="status"> </p>`),is=G(`<main class="alone"><!> <!> <!></main>`),as=G(`<p class="above"> </p>`),os=G(`<div class="shell"><aside class="side"><!> <footer><button class="link" title="Shows this in GridCraft's own window instead, until GridCraft closes">Use GridCraft's built-in window</button></footer></aside> <main class="part"><div class="page"><!> <!> <!> <!></div></main></div>`);function ss(e,t){Ue(t,!0);let n=dt(()=>_a.data),r=dt(()=>U(n)?U(n).pane:null);var i=Jr(),a=L(i),o=e=>{K(e,ts())},s=e=>{var t=is(),r=I(t);Ri(r,()=>U(n).looking.id,e=>{es(e,{get card(){return U(n).looking}})});var i=z(r,2),a=e=>{var t=ns(),r=R(t,!0);B(()=>J(r,U(n).error)),K(e,t)};Y(i,e=>{U(n).error&&e(a)});var o=z(i,2),s=e=>{var t=rs(),r=R(t,!0);B(()=>J(r,U(n).said)),K(e,t)};Y(o,e=>{U(n).said&&e(s)}),D(t),K(e,t)},c=e=>{var t=os(),i=I(t),a=I(i);ja(a,{get rows(){return U(n).nav}});var o=R(z(a,2));D(i);var s=z(i,2),c=I(s),l=I(c),u=e=>{var t=as(),n=R(t,!0);B(e=>J(n,e),[()=>U(r).above.join(`  ›  `)]),K(e,t)};Y(l,e=>{U(r).above&&U(r).above.length&&e(u)});var d=z(l,2),f=e=>{Va(e,{get pane(){return U(r)}})},p=e=>{io(e,{get pane(){return U(r)}})},m=e=>{xo(e,{get pane(){return U(r)}})},h=e=>{var t=Jr();Ri(L(t),()=>U(r).id,e=>{Io(e,{get pane(){return U(r)}})}),K(e,t)};Y(d,e=>{U(r).kind===`overview`?e(f):U(r).kind===`server`?e(p,1):U(r).kind===`kit`?e(m,2):U(r).kind===`project`&&e(h,3)});var ee=z(d,2),g=e=>{var t=ns(),r=R(t,!0);B(()=>J(r,U(n).error)),K(e,t)};Y(ee,e=>{U(n).error&&e(g)});var _=z(ee,2),v=e=>{var t=rs(),r=R(t,!0);B(()=>J(r,U(n).said)),K(e,t)};Y(_,e=>{U(n).said&&e(v)}),D(c),D(s),D(t),W(`click`,o,()=>$(`builtin`)),K(e,t)};Y(a,e=>{U(n)?U(n).looking?e(s,1):e(c,-1):e(o)}),K(e,i),We()}Br([`click`]);var cs=[`window`,`text`,`dim`,`accent`,`onAccent`,`danger`,`warn`,`border`,`input`,`block`,`hover`,`added`,`removed`];window.gridcraftState=e=>{let t=e&&e.theme;if(t){let e=document.documentElement;for(let n of cs)typeof t[n]==`string`&&e.style.setProperty(`--${n}`,t[n]);e.style.setProperty(`color-scheme`,t.dark?`dark`:`light`)}va(e)},Ti(ss,{target:document.getElementById(`app`)}),$(`ready`)})();"#),
    ("app.css", r#":root{--window:#fff;--text:#1f1f1f;--dim:#666;--accent:#0f7b46;--onAccent:#fff;--danger:#c0392b;--warn:#b85c00;--border:#d0d0d0;--input:#fff;--block:#f4f4f4;--hover:#ececec;--added:#0f7b462e;--removed:#c0392b2e;font:14px/1.45 Segoe UI,system-ui,-apple-system,BlinkMacSystemFont,Roboto,Noto Sans,Ubuntu,Cantarell,Helvetica Neue,sans-serif}*{box-sizing:border-box}html,body{background:var(--window);height:100%;color:var(--text);margin:0}#app{height:100%}.shell{grid-template-columns:236px minmax(0,1fr);height:100%;display:grid}.side{background:var(--block);border-right:1px solid var(--border);flex-direction:column;min-height:0;display:flex}.side nav{flex:1;min-height:0;padding:10px 8px;overflow-y:auto}.side footer{border-top:1px solid var(--border);text-align:left;margin:0;padding:8px 10px;font-size:12px}.part{min-width:0;overflow-y:auto}.page{max-width:800px;padding:20px 28px 28px}main.alone{max-width:920px;margin:0 auto;padding:14px 20px 20px}main.waiting{color:var(--dim);text-align:center;padding-top:40px}.navitem{text-align:left;background:0 0;border:none;border-radius:6px;align-items:center;gap:8px;width:100%;margin:1px 0;padding:5px 8px;display:flex}.navitem:hover:not(:disabled){background:var(--hover)}.navitem.selected{background:var(--window);box-shadow:inset 3px 0 0 var(--accent), 0 0 0 1px var(--border)}.navitem.selected .label{font-weight:600}.navitem .words{flex-direction:column;flex:1;min-width:0;display:flex}.navitem .label,.navitem .hint{text-overflow:ellipsis;white-space:nowrap;overflow:hidden}.navitem .hint{color:var(--dim);font-size:11.5px}.navitem.d0 .label{font-size:15px;font-weight:600}.navhead{letter-spacing:.06em;text-transform:uppercase;color:var(--dim);margin:14px 0 3px;font-size:11px;font-weight:600}.d0{padding-left:8px}.d1{padding-left:16px}.d2{padding-left:28px}.d3{padding-left:36px}.navhead.d2{letter-spacing:0;text-transform:none;margin:8px 0 1px;font-size:11.5px}.badge{background:var(--accent);min-width:19px;color:var(--onAccent);text-align:center;border-radius:10px;flex:none;padding:0 6px;font-size:11.5px;font-weight:600}.dot{border:1.5px solid var(--dim);border-radius:50%;flex:none;width:9px;height:9px}.dot.big{width:11px;height:11px}.dot.trusted,.dot.ok{background:var(--accent);border-color:var(--accent)}.dot.changed{background:var(--warn);border-color:var(--warn)}.dot.problem,.dot.error{background:var(--danger);border-color:var(--danger)}.above{color:var(--dim);margin:0 0 4px;font-size:12px}.head{margin-bottom:14px}.titled{flex-wrap:wrap;align-items:center;gap:8px;display:flex}.title{border:none;margin:0;padding:0;font-size:21px;font-weight:600}.lede{color:var(--dim);margin:5px 0 0}.sect{letter-spacing:.06em;text-transform:uppercase;color:var(--dim);margin:22px 0 7px;font-size:11.5px;font-weight:600}.box{border:1px solid var(--border);border-radius:8px;margin:8px 0;padding:12px 14px}.boxhead{flex-wrap:wrap;align-items:center;gap:8px;margin-bottom:4px;display:flex}.boxtitle{margin:0;font-size:15px;font-weight:600}.chip{border:1px solid var(--border);color:var(--dim);white-space:nowrap;border-radius:10px;padding:0 8px;font-size:11.5px;line-height:18px}.chip.good,.chip.trusted,.chip.rebuilt{border-color:var(--accent);color:var(--accent)}.chip.changed,.chip.partly{border-color:var(--warn);color:var(--warn)}.chip.problem{border-color:var(--danger);color:var(--danger)}.facts{grid-template-columns:1fr 1fr;gap:10px 18px;display:grid}.fact{align-items:center;gap:8px;min-width:0;display:flex}.fact.wide{grid-column:1/-1}.fact .small:first-child{flex:none;width:104px}.fact button{flex:none;margin-left:auto}.grouphead{flex-wrap:wrap;align-items:baseline;gap:10px;margin:14px 0 4px;display:flex}.grouphead h3{margin:0;font-size:14px;font-weight:600}.empty{border:1px dashed var(--border);border-radius:8px;margin:2px 0 0;padding:8px 14px}.list{border:1px solid var(--border);border-radius:8px;overflow:hidden}.entry{background:var(--window);text-align:left;border:none;border-radius:0;align-items:center;gap:10px;width:100%;padding:8px 12px;display:flex}.entry+.entry{border-top:1px solid var(--border)}.entry.still{cursor:default}.entry .words{flex-direction:column;flex:1;gap:2px;min-width:0;display:flex}.entry .line1{flex-wrap:wrap;align-items:center;gap:8px;display:flex}.entry .name{font-weight:600}.entry .hint{color:var(--dim);overflow-wrap:anywhere;font-size:12.5px}.entry .hint.warn{color:var(--warn)}.entry .arrow,.entry .chevron{color:var(--dim)}.standing{color:var(--dim);flex:none;font-size:12.5px}.standing.trusted{color:var(--accent);font-weight:600}.standing.changed{color:var(--warn);font-weight:600}.standing.problem{color:var(--danger);font-weight:600}.tick{cursor:pointer;align-items:center;gap:8px;width:-moz-fit-content;width:fit-content;margin-top:4px;display:flex}.tick input{accent-color:var(--accent);margin:0}input.name{flex:1;min-width:120px;max-width:320px}.parts{grid-template-columns:max-content minmax(0,1fr);gap:5px 14px;margin:0;display:grid}.parts dt,.parts dd{margin:0}.parts dd{overflow-wrap:anywhere}h1{border-bottom:1px solid var(--border);margin:16px 0 6px;padding-bottom:5px;font-size:15px;font-weight:600}h2{margin:14px 0 4px;font-size:13.5px;font-weight:600}p{margin:4px 0}.small{color:var(--dim);font-size:12.5px}.strong{font-weight:600}.warn,.bullet.warn,.small.warn{color:var(--warn);font-weight:600}.bullet{text-indent:-10px;padding-left:14px}.bullet:before{content:"•  "}code,.code,.target{font-family:Cascadia Mono,Consolas,SF Mono,Menlo,monospace;font-size:12.5px}button{font:inherit;color:var(--text);background:var(--input);border:1px solid var(--border);cursor:pointer;white-space:nowrap;border-radius:5px;padding:4px 10px}button:hover:not(:disabled){background:var(--hover)}button:focus-visible{outline:2px solid var(--accent);outline-offset:1px}input:focus-visible{outline:2px solid var(--accent);outline-offset:1px}button:disabled{color:var(--dim);cursor:default;opacity:.65}button.primary{background:var(--accent);border-color:var(--accent);color:var(--onAccent);font-weight:600}button.primary:hover:not(:disabled){background:var(--accent);filter:brightness(1.08)}button.link{color:var(--accent);background:0 0;border:none;padding:0 4px;text-decoration:underline}input[type=text]{font:inherit;color:var(--text);background:var(--input);border:1px solid var(--border);border-radius:5px;padding:4px 8px}.row{flex-wrap:wrap;align-items:center;gap:8px;margin:6px 0;display:flex}.grow{flex:1}.port{width:76px}.path{text-overflow:ellipsis;white-space:nowrap;min-width:0;color:var(--dim);overflow:hidden}.levels{flex-direction:column;gap:4px;margin:2px 0 6px;display:flex}.level{cursor:pointer;align-items:center;gap:8px;width:-moz-fit-content;width:fit-content;display:flex}.level input{accent-color:var(--accent);margin:0}.status{font-weight:600}.status.ok{color:var(--accent)}.status.error{color:var(--danger)}.status.none{color:var(--dim);font-weight:400}.item{border:1px solid var(--border);border-radius:8px;flex-wrap:wrap;align-items:center;gap:8px;margin:6px 0;padding:8px 12px;display:flex}.item.waits{border-color:var(--accent);box-shadow:inset 3px 0 0 var(--accent)}.item.plan{display:block}.item .head{flex-wrap:wrap;align-items:center;gap:8px;margin:0;display:flex}.item ol{margin:6px 0 2px;padding-left:2px;list-style:none}.item li .state{margin-left:8px}.item li .state:before{content:"·  "}.item li.done{color:var(--text)}.item li.failed .state{color:var(--danger);font-weight:600}.item li.running .state{color:var(--accent);font-weight:600}.card header{align-items:baseline;gap:12px;margin-bottom:8px;display:flex}.card header h1{border:none;flex:1;margin:0;padding:0;font-size:17px}.named{color:var(--dim);font-size:12.5px}.block{background:var(--block);border:1px solid var(--border);border-radius:6px;margin-bottom:8px;padding:8px 12px}.block.long{min-height:160px;max-height:calc(100vh - 330px);overflow-y:auto}.block.long .code{max-height:none}.target{overflow-wrap:anywhere;margin-bottom:6px;font-weight:600}.code{background:var(--window);border:1px solid var(--border);border-radius:4px;max-height:300px;margin:6px 0;padding:6px 0;line-height:1.4;overflow:auto}.line{white-space:pre;width:max-content;min-width:100%;display:flex}.line .no{text-align:right;width:46px;color:var(--dim);-webkit-user-select:none;user-select:none;flex:none;padding-right:8px}.line .mark{-webkit-user-select:none;user-select:none;flex:none;width:18px}.line .src{padding-right:12px}.line.gone{background:var(--removed)}.line.gone .mark{color:var(--danger)}.line.new{background:var(--added)}.line.new .mark{color:var(--accent)}.line.skip{color:var(--dim)}.step{border-top:1px solid var(--border);margin-top:10px;padding-top:8px}.step h2{margin:0 0 4px;font-size:13.5px}.step h2 code{overflow-wrap:anywhere;margin-left:6px;font-weight:400}.toggle{align-items:center;gap:8px;display:flex}.question{margin:12px 0 6px;font-weight:600}.choices{flex-direction:column;align-items:flex-start;gap:6px;margin-bottom:8px;display:flex}.choice{text-align:left;white-space:normal;max-width:100%;padding:5px 12px}.choice.quiet{color:var(--dim)}.refuse{gap:8px;width:100%;display:flex}.refuse input{flex:1;min-width:0}.said,.error{border:1px solid var(--border);border-radius:6px;margin-top:16px;padding:7px 11px;font-size:13px}.error{border-color:var(--danger);color:var(--danger);font-weight:600}.entry{white-space:normal}
/*$vite$:1*/"#),
];

/// The project the page is built from: each file's path, and its text. It is for whoever
/// changes the page (its README.md says how); the app itself doesn't hold it.
#[cfg(test)]
#[rustfmt::skip]
pub(crate) const SOURCE: &[(&str, &str)] = &[
    ("README.md", r#"# The Development window, as a page

GridCraft's Development window (Developer > Development) shown as a web page, in a window of
its own, where the desktop shell has a web view. It is a Svelte app.

GridCraft says what the window shows and the page draws it; what the person clicks goes back
as a message. The page decides nothing: every sentence about a request, and every rule about
answering one, is GridCraft's (`crates/ui-egui/src/aniefork/card.rs`, `kits.rs`, `devpage.rs`).

- `index.html` is the page. It loads `app.css` and `app.js` and nothing else.
- `src/` is the app. `main.js` takes what GridCraft says (`window.gridcraftState`), `host.js`
  sends what was clicked. `App.svelte` shows the list of the window's parts down the side
  (`Nav.svelte`) and the one part beside it: the top (`Overview.svelte`), the MCP server
  (`Server.svelte`), a development kit (`Kit.svelte`) or one of its projects
  (`Project.svelte`). One request's card takes the place of all of it (`Review.svelte`, and
  `Blocks.svelte` for what a card holds). `marks.js` turns what GridCraft says of how
  something stands into one of a fixed set of class names.

## Rules for whoever changes it

- **Everything a request holds is drawn as text.** No `{@html ...}`, no `innerHTML`, no
  attribute built from what GridCraft sent other than `class` from a fixed set of names.
- **The page is served under a policy that allows its own script and stylesheet and nothing
  else:** no inline script, no inline style, no `eval`, nothing from the network. Set styles
  through CSS classes, or through the style object from script.
- **A message is sent only in answer to a click.** GridCraft takes a message from this page
  as the person's own doing.

## Building it

The built `app.js` and `app.css` are kept in GridCraft's source beside this source
(`apps/gridcraft/src/aniefork/page.rs`, generated). To change the page:

    cargo test -p gridcraft -- --ignored write_the_pages_source   # writes this project to target/development-page
    cd target/development-page
    npm install --ignore-scripts
    npm run build
    cd ../..
    cargo test -p gridcraft -- --ignored take_the_built_page      # writes page.rs from the project and its dist/

Nothing is fetched or built when GridCraft itself is built.
"#),
    ("THIRD-PARTY.md", r#"# What `app.js` contains that others wrote

The built script holds the page and the parts of Svelte it uses, which can include parts of clsx and esm-env. Each is under the MIT licence, whose notices follow.

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
    ("package.json", r#"{
  "name": "gridcraft-development-page",
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
    ("vite.config.js", r#"import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// Builds the page's script and stylesheet, under the names index.html asks for: dist/app.js
// and dist/app.css. One script, no modules, nothing loaded at run time: the page is served
// under a policy that allows its own two files and nothing else.
export default defineConfig({
  plugins: [svelte()],
  base: "./",
  build: {
    outDir: "dist",
    emptyOutDir: true,
    target: "es2020",
    cssCodeSplit: false,
    rollupOptions: {
      input: "src/main.js",
      output: { format: "iife", entryFileNames: "app.js", assetFileNames: "app[extname]" },
    },
  },
});
"#),
    ("index.html", r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Development</title>
    <link rel="stylesheet" href="app.css" />
  </head>
  <body>
    <div id="app"></div>
    <!-- Built from the Svelte source beside this file with `npm run build`. -->
    <script src="app.js"></script>
  </body>
</html>
"#),
    ("src/main.js", r#"import { mount } from "svelte";
import App from "./App.svelte";
import { show } from "./view.svelte.js";
import { send } from "./host.js";
import "./app.css";

// The colours are GridCraft's own, so that the window looks like the app in light and dark.
const COLOURS = ["window", "text", "dim", "accent", "onAccent", "danger", "warn", "border", "input", "block", "hover", "added", "removed"];

// GridCraft calls this with everything the window shows, whenever any of it changes.
window.gridcraftState = (data) => {
  const theme = data && data.theme;
  if (theme) {
    const root = document.documentElement;
    for (const name of COLOURS) {
      if (typeof theme[name] === "string") root.style.setProperty(`--${name}`, theme[name]);
    }
    root.style.setProperty("color-scheme", theme.dark ? "dark" : "light");
  }
  show(data);
};

mount(App, { target: document.getElementById("app") });
send("ready");
"#),
    ("src/host.js", r#"// The page's one way to GridCraft: a message, as text. GridCraft takes a message from this
// page as the click of the person at the screen, so nothing here sends one except in answer
// to a click. Another shell (not the system web view) would change this file and no other.
export function send(act, more = {}) {
  const message = JSON.stringify({ ...more, act });
  if (window.ipc && typeof window.ipc.postMessage === "function") {
    window.ipc.postMessage(message);
  }
}
"#),
    ("src/view.svelte.js", r#"// What GridCraft last said the window shows. The page keeps nothing else of its own but what
// the person is typing.
export const view = $state({ data: null });

export function show(data) {
  view.data = data;
}
"#),
    ("src/app.css", r#"/* The Development window. The colours are set by GridCraft (main.js): these are what the
   page shows before it has heard from it. */
:root {
  --window: #ffffff;
  --text: #1f1f1f;
  --dim: #666666;
  --accent: #0f7b46;
  --onAccent: #ffffff;
  --danger: #c0392b;
  --warn: #b85c00;
  --border: #d0d0d0;
  --input: #ffffff;
  --block: #f4f4f4;
  --hover: #ececec;
  --added: rgba(15, 123, 70, 0.18);
  --removed: rgba(192, 57, 43, 0.18);
  font: 14px/1.45 "Segoe UI", system-ui, -apple-system, sans-serif;
}

* {
  box-sizing: border-box;
}

html,
body {
  margin: 0;
  height: 100%;
  background: var(--window);
  color: var(--text);
}

#app {
  height: 100%;
}

/* The window: a list of its parts down the side, and the one that is shown beside it. */
.shell {
  display: grid;
  grid-template-columns: 236px minmax(0, 1fr);
  height: 100%;
}

.side {
  display: flex;
  flex-direction: column;
  min-height: 0;
  background: var(--block);
  border-right: 1px solid var(--border);
}

.side nav {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 10px 8px;
}

.side footer {
  margin: 0;
  padding: 8px 10px;
  border-top: 1px solid var(--border);
  text-align: left;
  font-size: 12px;
}

.part {
  min-width: 0;
  overflow-y: auto;
}

.page {
  max-width: 800px;
  padding: 20px 28px 28px;
}

/* A request's card stands alone, without the list. */
main.alone {
  max-width: 920px;
  margin: 0 auto;
  padding: 14px 20px 20px;
}

main.waiting {
  color: var(--dim);
  padding-top: 40px;
  text-align: center;
}

/* The list. */
.navitem {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  margin: 1px 0;
  padding: 5px 8px;
  border: none;
  border-radius: 6px;
  background: none;
  text-align: left;
}

.navitem:hover:not(:disabled) {
  background: var(--hover);
}

.navitem.selected {
  background: var(--window);
  box-shadow: inset 3px 0 0 var(--accent), 0 0 0 1px var(--border);
}

.navitem.selected .label {
  font-weight: 600;
}

.navitem .words {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-width: 0;
}

.navitem .label,
.navitem .hint {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.navitem .hint {
  font-size: 11.5px;
  color: var(--dim);
}

.navitem.d0 .label {
  font-size: 15px;
  font-weight: 600;
}

.navhead {
  margin: 14px 0 3px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: var(--dim);
}

.d0 {
  padding-left: 8px;
}

.d1 {
  padding-left: 16px;
}

.d2 {
  padding-left: 28px;
}

.d3 {
  padding-left: 36px;
}

.navhead.d2 {
  margin: 8px 0 1px;
  font-size: 11.5px;
  letter-spacing: 0;
  text-transform: none;
}

.badge {
  flex: none;
  min-width: 19px;
  padding: 0 6px;
  border-radius: 10px;
  background: var(--accent);
  color: var(--onAccent);
  font-size: 11.5px;
  font-weight: 600;
  text-align: center;
}

/* How something stands, at a glance. */
.dot {
  flex: none;
  width: 9px;
  height: 9px;
  border: 1.5px solid var(--dim);
  border-radius: 50%;
}

.dot.big {
  width: 11px;
  height: 11px;
}

.dot.trusted,
.dot.ok {
  background: var(--accent);
  border-color: var(--accent);
}

.dot.changed {
  background: var(--warn);
  border-color: var(--warn);
}

.dot.problem,
.dot.error {
  background: var(--danger);
  border-color: var(--danger);
}

/* A part: its place, its title, what it is. */
.above {
  margin: 0 0 4px;
  font-size: 12px;
  color: var(--dim);
}

.head {
  margin-bottom: 14px;
}

.titled {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.title {
  margin: 0;
  padding: 0;
  border: none;
  font-size: 21px;
  font-weight: 600;
}

.lede {
  margin: 5px 0 0;
  color: var(--dim);
}

.sect {
  margin: 22px 0 7px;
  font-size: 11.5px;
  font-weight: 600;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: var(--dim);
}

.box {
  margin: 8px 0;
  padding: 12px 14px;
  border: 1px solid var(--border);
  border-radius: 8px;
}

.boxhead {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.boxtitle {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
}

.chip {
  padding: 0 8px;
  border: 1px solid var(--border);
  border-radius: 10px;
  font-size: 11.5px;
  line-height: 18px;
  color: var(--dim);
  white-space: nowrap;
}

.chip.good,
.chip.trusted,
.chip.rebuilt {
  border-color: var(--accent);
  color: var(--accent);
}

.chip.changed,
.chip.partly {
  border-color: var(--warn);
  color: var(--warn);
}

.chip.problem {
  border-color: var(--danger);
  color: var(--danger);
}

/* What a kit needs to run, in a few lines. */
.facts {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px 18px;
}

.fact {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.fact.wide {
  grid-column: 1 / -1;
}

.fact .small:first-child {
  flex: none;
  width: 104px;
}

.fact button {
  flex: none;
  margin-left: auto;
}

/* A kit's projects, and what a project rebuilds. */
.grouphead {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 10px;
  margin: 14px 0 4px;
}

.grouphead h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
}

.empty {
  margin: 2px 0 0;
  padding: 8px 14px;
  border: 1px dashed var(--border);
  border-radius: 8px;
}

.list {
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
}

.entry {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  border-radius: 0;
  background: var(--window);
  text-align: left;
}

.entry + .entry {
  border-top: 1px solid var(--border);
}

.entry.still {
  cursor: default;
}

.entry .words {
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: 2px;
  min-width: 0;
}

.entry .line1 {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.entry .name {
  font-weight: 600;
}

.entry .hint {
  font-size: 12.5px;
  color: var(--dim);
  overflow-wrap: anywhere;
}

.entry .hint.warn {
  color: var(--warn);
}

.entry .arrow,
.entry .chevron {
  color: var(--dim);
}

.standing {
  flex: none;
  font-size: 12.5px;
  color: var(--dim);
}

.standing.trusted {
  color: var(--accent);
  font-weight: 600;
}

.standing.changed {
  color: var(--warn);
  font-weight: 600;
}

.standing.problem {
  color: var(--danger);
  font-weight: 600;
}

.tick {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
  width: fit-content;
  cursor: pointer;
}

.tick input {
  accent-color: var(--accent);
  margin: 0;
}

input.name {
  flex: 1;
  min-width: 120px;
  max-width: 320px;
}

.parts {
  display: grid;
  grid-template-columns: max-content minmax(0, 1fr);
  gap: 5px 14px;
  margin: 0;
}

.parts dt,
.parts dd {
  margin: 0;
}

.parts dd {
  overflow-wrap: anywhere;
}

h1 {
  font-size: 15px;
  font-weight: 600;
  margin: 16px 0 6px;
  padding-bottom: 5px;
  border-bottom: 1px solid var(--border);
}

h2 {
  font-size: 13.5px;
  font-weight: 600;
  margin: 14px 0 4px;
}

p {
  margin: 4px 0;
}

.small {
  font-size: 12.5px;
  color: var(--dim);
}

.strong {
  font-weight: 600;
}

.warn,
.bullet.warn,
.small.warn {
  color: var(--warn);
  font-weight: 600;
}

.bullet {
  padding-left: 14px;
  text-indent: -10px;
}

.bullet::before {
  content: "•  ";
}

code,
.code,
.target {
  font-family: "Cascadia Mono", Consolas, "SF Mono", Menlo, monospace;
  font-size: 12.5px;
}

button {
  font: inherit;
  color: var(--text);
  background: var(--input);
  border: 1px solid var(--border);
  border-radius: 5px;
  padding: 4px 10px;
  cursor: pointer;
  white-space: nowrap;
}

button:hover:not(:disabled) {
  background: var(--hover);
}

button:focus-visible,
input:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 1px;
}

button:disabled {
  color: var(--dim);
  cursor: default;
  opacity: 0.65;
}

button.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: var(--onAccent);
  font-weight: 600;
}

button.primary:hover:not(:disabled) {
  background: var(--accent);
  filter: brightness(1.08);
}

button.link {
  border: none;
  background: none;
  color: var(--accent);
  padding: 0 4px;
  text-decoration: underline;
}

input[type="text"] {
  font: inherit;
  color: var(--text);
  background: var(--input);
  border: 1px solid var(--border);
  border-radius: 5px;
  padding: 4px 8px;
}

.row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin: 6px 0;
}

.grow {
  flex: 1;
}

.port {
  width: 76px;
}

.path {
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--dim);
}

/* The levels. */
.levels {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin: 2px 0 6px;
}

.level {
  display: flex;
  align-items: center;
  gap: 8px;
  width: fit-content;
  cursor: pointer;
}

.level input {
  accent-color: var(--accent);
  margin: 0;
}

.status {
  font-weight: 600;
}

.status.ok {
  color: var(--accent);
}

.status.error {
  color: var(--danger);
}

.status.none {
  color: var(--dim);
  font-weight: 400;
}

/* What waits, and the plans. */
.item {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 8px 12px;
  margin: 6px 0;
}

.item.waits {
  border-color: var(--accent);
  box-shadow: inset 3px 0 0 var(--accent);
}

.item.plan {
  display: block;
}

.item .head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin: 0;
}

/* GridCraft numbers a plan's steps itself. */
.item ol {
  margin: 6px 0 2px;
  padding-left: 2px;
  list-style: none;
}

.item li .state {
  margin-left: 8px;
}

.item li .state::before {
  content: "·  ";
}

.item li.done {
  color: var(--text);
}

.item li.failed .state {
  color: var(--danger);
  font-weight: 600;
}

.item li.running .state {
  color: var(--accent);
  font-weight: 600;
}

/* A request, in full. */
.card header {
  display: flex;
  align-items: baseline;
  gap: 12px;
  margin-bottom: 8px;
}

.card header h1 {
  border: none;
  margin: 0;
  padding: 0;
  font-size: 17px;
  flex: 1;
}

.named {
  font-size: 12.5px;
  color: var(--dim);
}

.block {
  background: var(--block);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 8px 12px;
  margin-bottom: 8px;
}

/* A plan's steps scroll inside the card, so that the answer stays in reach. */
.block.long {
  max-height: calc(100vh - 330px);
  min-height: 160px;
  overflow-y: auto;
}

/* Inside a plan the card scrolls as a whole: a step's lines are all there, one after another. */
.block.long .code {
  max-height: none;
}

.target {
  font-weight: 600;
  margin-bottom: 6px;
  overflow-wrap: anywhere;
}

.code {
  background: var(--window);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 6px 0;
  margin: 6px 0;
  max-height: 300px;
  overflow: auto;
  line-height: 1.4;
}

.line {
  display: flex;
  width: max-content;
  min-width: 100%;
  white-space: pre;
}

.line .no {
  flex: none;
  width: 46px;
  padding-right: 8px;
  text-align: right;
  color: var(--dim);
  user-select: none;
}

.line .mark {
  flex: none;
  width: 18px;
  user-select: none;
}

.line .src {
  padding-right: 12px;
}

.line.gone {
  background: var(--removed);
}

.line.gone .mark {
  color: var(--danger);
}

.line.new {
  background: var(--added);
}

.line.new .mark {
  color: var(--accent);
}

.line.skip {
  color: var(--dim);
}

.step {
  border-top: 1px solid var(--border);
  margin-top: 10px;
  padding-top: 8px;
}

.step h2 {
  margin: 0 0 4px;
  font-size: 13.5px;
}

.step h2 code {
  font-weight: 400;
  margin-left: 6px;
  overflow-wrap: anywhere;
}

.toggle {
  display: flex;
  align-items: center;
  gap: 8px;
}

.question {
  font-weight: 600;
  margin: 12px 0 6px;
}

.choices {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 6px;
  margin-bottom: 8px;
}

.choice {
  text-align: left;
  padding: 5px 12px;
  max-width: 100%;
  white-space: normal;
}

.choice.quiet {
  color: var(--dim);
}

.refuse {
  display: flex;
  gap: 8px;
  width: 100%;
}

.refuse input {
  flex: 1;
  min-width: 0;
}

.said,
.error {
  margin-top: 16px;
  padding: 7px 11px;
  border-radius: 6px;
  border: 1px solid var(--border);
  font-size: 13px;
}

.error {
  border-color: var(--danger);
  color: var(--danger);
  font-weight: 600;
}

/* A row that is a button holds words that wrap. */
.entry {
  white-space: normal;
}
"#),
    ("src/marks.js", r#"// How something stands, as one of a fixed set of class names. What GridCraft sends is looked
// up here, never used as a class by itself: a word it didn't expect is "none".
const MARKS = { trusted: "trusted", ok: "ok", changed: "changed", problem: "problem", error: "error", untrusted: "none", off: "none" };

export function mark(code) {
  return (typeof code === "string" && Object.hasOwn(MARKS, code) && MARKS[code]) || "none";
}

// How deep a row of the list sits, as a class name: d0 to d3.
export function depth(value) {
  const whole = Number.isInteger(value) ? value : 0;
  return "d" + Math.min(3, Math.max(0, whole));
}

// What became of a procedure, as a class name.
const STATES = { rebuilt: "rebuilt", partly: "partly", left: "left" };

export function state(code) {
  return (typeof code === "string" && Object.hasOwn(STATES, code) && STATES[code]) || "left";
}
"#),
    ("src/App.svelte", r#"<script>
  // The Development window, as a page. GridCraft says what it shows (view.data) and the page
  // draws it: a list of its parts down the side, and the one part that is shown beside it.
  // Every sentence comes from GridCraft, the same words its own window uses. What the person
  // clicks goes back as a message (host.js).
  import { view } from "./view.svelte.js";
  import { send } from "./host.js";
  import Nav from "./Nav.svelte";
  import Overview from "./Overview.svelte";
  import Server from "./Server.svelte";
  import Kit from "./Kit.svelte";
  import Project from "./Project.svelte";
  import Review from "./Review.svelte";

  let data = $derived(view.data);
  let pane = $derived(data ? data.pane : null);
</script>

{#if !data}
  <main class="waiting"><p>Waiting for GridCraft…</p></main>
{:else if data.looking}
  <!-- A request being looked at takes the place of everything else: what is being decided is
       all there is to look at. A new card is a new component: what was typed for the one
       before is gone with it. -->
  <main class="alone">
    {#key data.looking.id}
      <Review card={data.looking} />
    {/key}
    {#if data.error}<p class="error" role="alert">{data.error}</p>{/if}
    {#if data.said}<p class="said" role="status">{data.said}</p>{/if}
  </main>
{:else}
  <div class="shell">
    <aside class="side">
      <Nav rows={data.nav} />
      <footer>
        <button class="link" onclick={() => send("builtin")} title="Shows this in GridCraft's own window instead, until GridCraft closes">Use GridCraft's built-in window</button>
      </footer>
    </aside>
    <main class="part">
      <div class="page">
        {#if pane.above && pane.above.length}
          <p class="above">{pane.above.join("  ›  ")}</p>
        {/if}
        {#if pane.kind === "overview"}
          <Overview {pane} />
        {:else if pane.kind === "server"}
          <Server {pane} />
        {:else if pane.kind === "kit"}
          <Kit {pane} />
        {:else if pane.kind === "project"}
          <!-- Another project is another component: nothing typed for one is kept for the next. -->
          {#key pane.id}
            <Project {pane} />
          {/key}
        {/if}
        {#if data.error}<p class="error" role="alert">{data.error}</p>{/if}
        {#if data.said}<p class="said" role="status">{data.said}</p>{/if}
      </div>
    </main>
  </div>
{/if}
"#),
    ("src/Nav.svelte", r#"<script>
  // The list down the window's side: every part that can be shown, as GridCraft lists them.
  // Development at the top; under it the MCP server, and the development kits with their
  // projects.
  import { send } from "./host.js";
  import { mark, depth } from "./marks.js";

  let { rows } = $props();
</script>

<nav aria-label="The parts of the Development window">
  {#each rows as row}
    {#if row.t === "heading"}
      <p class="navhead {depth(row.depth)}">{row.label}</p>
    {:else if row.t === "item"}
      <button
        class="navitem {depth(row.depth)}"
        class:selected={row.selected}
        aria-current={row.selected ? "page" : undefined}
        onclick={() => send("go", { to: row.go })}
      >
        {#if row.mark}<span class="dot {mark(row.mark)}" aria-hidden="true"></span>{/if}
        <span class="words">
          <span class="label">{row.label}</span>
          {#if row.hint}<span class="hint">{row.hint}</span>{/if}
        </span>
        {#if row.badge}<span class="badge" title="Waiting for your answer">{row.badge}</span>{/if}
      </button>
    {:else if row.t === "more"}
      <button class="navitem more {depth(row.depth)}" onclick={() => send("go", { to: row.go })}>
        <span class="words"><span class="hint">{row.label}</span></span>
      </button>
    {/if}
  {/each}
</nav>
"#),
    ("src/Overview.svelte", r#"<script>
  // The top of the window: the MCP server, the development kits and the open workbook, each
  // at a glance, with the way to its own part.
  import { send } from "./host.js";
  import { mark } from "./marks.js";

  let { pane } = $props();

  let server = $derived(pane.server);
  let workbook = $derived(pane.workbook);
</script>

<header class="head">
  <h1 class="title">{pane.title}</h1>
  <p class="lede">{pane.intro}</p>
</header>

<section class="box" aria-label={server.name}>
  <div class="boxhead">
    <span class="dot {mark(server.available ? server.status.kind : 'off')}" aria-hidden="true"></span>
    <h2 class="boxtitle">{server.name}</h2>
    {#if server.available}<span class="chip">{server.level}</span>{/if}
    <span class="grow"></span>
    <button onclick={() => send("go", { to: server.go })} aria-label="Open {server.name}" title="How far agents may go, how they connect, and what they ask for">Open</button>
  </div>
  {#if !server.available}
    <p>{server.unavailable}</p>
  {:else}
    <p>{server.says}</p>
    <p class="status {mark(server.status.kind)}">{server.status.text}</p>
    {#if server.clients}<p class="small">{server.clients}</p>{/if}
    {#if server.waiting}
      <p class="warn">{server.waiting}</p>
    {/if}
    {#if server.plan}<p class="small">{server.plan}</p>{/if}
  {/if}
</section>

<h2 class="sect">{pane.kitsTitle}</h2>
{#each pane.kits as kit}
  <section class="box" aria-label={kit.name}>
    <div class="boxhead">
      <h2 class="boxtitle">{kit.name}</h2>
      {#each kit.madeOf as part}<span class="chip">{part}</span>{/each}
      <span class="chip good">{kit.standing}</span>
      <span class="grow"></span>
      <button onclick={() => send("go", { to: kit.go })} aria-label="Open {kit.name}" title="Its projects, and what it needs to run them">Open</button>
    </div>
    <p>{kit.says}</p>
    {#if kit.projects}<p class="small">{kit.projects}</p>{/if}
  </section>
{/each}
<p class="small">{pane.kitsNote}</p>

<h2 class="sect">Open workbook</h2>
<section class="box" aria-label="Open workbook">
  <div class="boxhead">
    {#if workbook.title}<h2 class="boxtitle">{workbook.title}</h2>{/if}
    <span class="grow"></span>
    {#if workbook.macros}
      <button onclick={() => send("open", { name: "macros" })} title="Read the macros this workbook carries. GridCraft does not run them.">Macros…</button>
    {/if}
  </div>
  <p class:warn={workbook.problem}>{workbook.says}</p>
  {#if workbook.rebuilt}<p class="small">{workbook.rebuilt}</p>{/if}
</section>

<div class="row">
  <button onclick={() => send("open", { name: "console" })} title="What your projects print and log, what failed, and what agents asked for">Console…</button>
  <span class="small">{pane.console}</span>
</div>
"#),
    ("src/Server.svelte", r#"<script>
  // The MCP server's part: how far agents may go, how to connect, what waits for an answer,
  // the plan that was approved, and what was asked lately.
  import { send } from "./host.js";
  import { mark } from "./marks.js";

  let { pane } = $props();

  // The port is the person's to type: the page keeps what they typed until they set it.
  let port = $state("");
  let typed = $state(false);
  $effect(() => {
    if (!typed && pane.available) port = pane.portText;
  });

  // The level shown is the one GridCraft says is set: a click asks for another, and the
  // mark moves only when GridCraft has set it (raising it asks the operating system first).
  function choose(event, code) {
    event.preventDefault();
    if (code !== pane.level) send("level", { level: code });
  }

  function setPort() {
    send("port", { port });
    typed = false;
  }
</script>

<header class="head">
  <h1 class="title">{pane.title}</h1>
  {#if pane.available}<p class="lede">{pane.intro}</p>{/if}
</header>

{#if !pane.available}
  <p>{pane.unavailable}</p>
{:else}
  {#if pane.waiting.length}
    <h2 class="sect">{pane.waitingTitle}</h2>
    {#each pane.waiting as asked}
      <div class="item waits">
        <span class="strong">{asked.kind}</span>
        <code>{asked.target}</code>
        <span class="grow"></span>
        <span class="small">{asked.named}</span>
        <button class="primary" onclick={() => send("look", { id: asked.id })} title="See everything this request would do, and answer it">Look…</button>
      </div>
    {/each}
  {/if}

  <h2 class="sect">How far agents may go</h2>
  <section class="box">
    <div class="levels" role="radiogroup" aria-label="How far agents may go">
      {#each pane.levels as level}
        <label class="level" title={level.meaning}>
          <input type="radio" name="level" checked={level.code === pane.level} onclick={(event) => choose(event, level.code)} />
          <span>{level.label}</span>
        </label>
      {/each}
    </div>
    <p class="small">{pane.meaning}</p>
  </section>

  <h2 class="sect">Connection</h2>
  <section class="box">
    <p class="status {mark(pane.status.kind)}">{pane.status.text}</p>
    {#if pane.clients}<p class="small">{pane.clients}</p>{/if}
    <div class="row">
      <label for="port">Port:</label>
      <input id="port" class="port" type="text" inputmode="numeric" maxlength="5" bind:value={port} oninput={() => (typed = true)} />
      <button onclick={setPort}>Set Port</button>
    </div>
    {#if pane.serving}
      <div class="row">
        <button onclick={() => send("copySetup", { json: false })} title="Copies the command that adds this server to Claude Code. It holds the token.">Copy Setup for Claude Code</button>
        <button onclick={() => send("copySetup", { json: true })} title="Copies an MCP settings entry, for other agents. It holds the token.">Copy Setup as JSON</button>
        <button onclick={() => send("newToken")} title="Makes a new token: an agent that holds the old one can no longer connect">New Token</button>
      </div>
    {/if}
  </section>

  {#if pane.plans.length}
    <h2 class="sect">Plans</h2>
    {#each pane.plans as plan}
      <div class="item plan">
        <div class="head">
          <span class="strong">{plan.name}</span>
          <span>{plan.title}</span>
          <span class="grow"></span>
          <span class="small">{plan.stands}</span>
          {#if plan.live}
            <button onclick={() => send("stopPlan", { id: plan.id })} title="None of its steps is carried out after this. What was done stays done.">Stop Plan</button>
          {/if}
        </div>
        <ol>
          {#each plan.steps as step}
            <li class="small" class:done={step.state === "done"} class:failed={step.state === "failed"} class:running={step.state === "running"}>
              <span>{step.what}</span><span class="state">{step.stands}</span>
            </li>
          {/each}
        </ol>
      </div>
    {/each}
  {/if}

  {#if pane.unasked.length}
    <h2 class="sect">Not asked again until GridCraft closes</h2>
    {#each pane.unasked as rule}
      <div class="row">
        <span>{rule.covers}</span>
        <button class="link" onclick={() => send("askAgain", { module: rule.module, kind: rule.kind })}>Ask again</button>
      </div>
    {/each}
  {/if}

  {#if pane.lately.length}
    <h2 class="sect">Lately</h2>
    {#each pane.lately as line}
      <p class="small">{line}</p>
    {/each}
    <p class="small">{pane.latelyNote}</p>
  {/if}
{/if}
"#),
    ("src/Kit.svelte", r#"<script>
  // A development kit's part: what it is, what it needs to run, its projects by what they are
  // for (the code of one workbook, or a library), a new project, and its showcase.
  import { send } from "./host.js";
  import { mark } from "./marks.js";

  let { pane } = $props();

  // The name of a new project is the person's to type; whether it is the open workbook's
  // code is theirs to tick. Which workbook that is, GridCraft says.
  let name = $state("");
  let forWorkbook = $state(false);

  function create(event) {
    event.preventDefault();
    send("createProject", { name, forWorkbook: forWorkbook && !!pane.newFor });
    name = "";
    forWorkbook = false;
  }
</script>

<header class="head">
  <div class="titled">
    <h1 class="title">{pane.title}</h1>
    {#each pane.madeOf as part}<span class="chip">{part}</span>{/each}
    <span class="chip good">{pane.standing}</span>
  </div>
  <p class="lede">{pane.says}</p>
</header>

{#if !pane.available}
  <p>{pane.unavailable}</p>
{:else}
  <section class="box facts">
    <div class="fact">
      <span class="small">Projects</span>
      <span>{pane.projectsSays}</span>
      <button onclick={() => send("open", { name: "modules" })} title="Review, trust and run your projects; name the Python that runs their scripts">Modules…</button>
    </div>
    <div class="fact">
      <span class="small">Console</span>
      <span>{pane.console}</span>
      <button onclick={() => send("open", { name: "console" })} title="What your projects print and log, what failed, and what agents asked for">Console…</button>
    </div>
    <div class="fact wide">
      <span class="small">Runs with</span>
      <span>{pane.python}</span>
    </div>
    <div class="fact wide">
      <span class="small">Projects folder</span>
      <code class="path" title={pane.folder}>{pane.folder}</code>
      {#if pane.canOpenFolder}<button onclick={() => send("openFolder")}>Open Folder</button>{/if}
    </div>
  </section>

  <h2 class="sect">{pane.projectsTitle}</h2>
  <p class="small">{pane.projectsNote}</p>
  {#each pane.groups as group}
    <div class="grouphead">
      <h3>{group.label}</h3>
      <span class="small">{group.says}</span>
    </div>
    {#if group.projects.length === 0}
      <p class="small empty">{group.empty}</p>
    {:else}
      <div class="list">
        {#each group.projects as project}
          <button class="entry" disabled={!project.go} onclick={() => send("go", { to: project.go })} title="What it is for, what it asks, and what is in it">
            <span class="dot {mark(project.status)}" aria-hidden="true"></span>
            <span class="words">
              <span class="line1">
                <span class="name">{project.name}</span>
                {#if project.workbook}<span class="chip">{project.workbook}{project.open ? "  ·  open" : ""}</span>{/if}
              </span>
              {#if project.says}<span class="hint">{project.says}</span>{/if}
            </span>
            <span class="standing {mark(project.status)}">{project.statusLabel}</span>
            <span class="chevron" aria-hidden="true">›</span>
          </button>
        {/each}
      </div>
    {/if}
  {/each}

  <h2 class="sect">New project</h2>
  <form class="box" onsubmit={create}>
    <div class="row">
      <label for="newname">Name:</label>
      <input id="newname" class="name" type="text" maxlength="80" bind:value={name} placeholder="Its name" />
      <button class="primary" type="submit" disabled={!name.trim()} title="Creates a starter project. It does nothing until you trust it.">Create Project</button>
    </div>
    {#if pane.newFor}
      <label class="tick">
        <input type="checkbox" bind:checked={forWorkbook} />
        <span>It is the code of <strong>{pane.newFor}</strong>, the workbook that is open</span>
      </label>
    {/if}
  </form>

  <h2 class="sect">Showcase</h2>
  <div class="row">
    <span>{pane.showcase.text}</span>
    {#if pane.showcase.stands === "none"}
      <button onclick={() => send("createShowcase")} title="Creates the Svelte Showcase project. It does nothing until you trust it.">Create Showcase</button>
    {:else if pane.showcase.stands === "trusted"}
      {#if pane.showcase.canDock}
        <button onclick={() => send("showShowcase", { docked: true })} title="Shows its panel inside GridCraft's window, beside the sheet">Dock Showcase</button>
      {/if}
      {#if pane.showcase.canBrowse}
        <button onclick={() => send("showShowcase", { docked: false })}>Open in Browser</button>
      {/if}
    {/if}
  </div>
{/if}
"#),
    ("src/Project.svelte", r#"<script>
  // One project: what it is for, where it stands, what it asks to be allowed, what is in it,
  // and what it says it rebuilds of its workbook's macros. Its name, its description and its
  // notes are its author's words: they are drawn as text, like everything else.
  import { send } from "./host.js";
  import { mark, state } from "./marks.js";

  let { pane } = $props();
</script>

<header class="head">
  <div class="titled">
    <span class="dot big {mark(pane.status)}" aria-hidden="true"></span>
    <h1 class="title">{pane.title}</h1>
    {#if pane.version}<span class="small">{pane.version}</span>{/if}
    {#if pane.author}<span class="small">by {pane.author}</span>{/if}
    <span class="grow"></span>
    <span class="chip">{pane.kindLabel}</span>
    <span class="chip standing {mark(pane.status)}">{pane.statusLabel}</span>
  </div>
  {#if pane.description}<p class="lede">{pane.description}</p>{/if}
</header>

<p>{pane.forSays}</p>
<p class="small">{pane.statusSays}</p>
{#if pane.problem}<p class="error" role="alert">This project can't be used: {pane.problem}.</p>{/if}

<h2 class="sect">{pane.asksTitle}</h2>
<section class="box">
  {#each pane.asks as ask}
    <p class="bullet" class:warn={ask.warn}>{ask.text}</p>
  {:else}
    <p class="bullet">{pane.asksNothing}</p>
  {/each}
</section>

<h2 class="sect">What is in it</h2>
<dl class="parts">
  {#each pane.parts as part}
    <dt class="small">{part.label}</dt>
    <dd><code>{part.value}</code></dd>
  {/each}
</dl>

{#if pane.rebuilt}
  <h2 class="sect">{pane.rebuilt.title}</h2>
  {#if pane.rebuilt.says}<p class="small" class:warn={pane.rebuilt.problem}>{pane.rebuilt.says}</p>{/if}
  {#if pane.rebuilt.rows.length}
    <div class="list plain">
      {#each pane.rebuilt.rows as row}
        <div class="entry still">
          <span class="words">
            <span class="line1">
              <code>{row.from}</code>
              {#if row.script}<span class="arrow" aria-hidden="true">→</span><code>{row.script}</code>{/if}
            </span>
            {#if row.note}<span class="hint">{row.note}</span>{/if}
            {#if row.scriptMissing}<span class="hint warn">{pane.rebuilt.missing}</span>{/if}
          </span>
          <span class="chip state {state(row.state)}">{row.stateLabel}</span>
        </div>
      {/each}
    </div>
  {/if}
{/if}

<h2 class="sect">Manage</h2>
<p class="small">{pane.manage}</p>
<div class="row">
  <button onclick={() => send("open", { name: "modules" })} title="Review, trust and run your projects">Modules…</button>
  {#if pane.canOpenFolder}<button onclick={() => send("openProjectFolder", { id: pane.id })}>Open Project Folder</button>{/if}
</div>
"#),
    ("src/Review.svelte", r#"<script>
  // One request, in full, for the person to answer: what it is, the thing itself, what it
  // means, and the choices. GridCraft decides what a click here does, and holds to its own
  // rules whatever this page allows: a yes in the card's first moment on screen, or to a card
  // that isn't the one on screen, answers nothing.
  import { send } from "./host.js";
  import Blocks from "./Blocks.svelte";

  let { card } = $props();

  // What the person would tell the agent with a no. It is theirs: it stays in the page until
  // they press no.
  let reason = $state("");
  // A card that has only just come on screen can't be said yes to yet.
  let settled = $state(false);

  $effect(() => {
    const wait = card.readyIn;
    if (wait <= 0) {
      settled = true;
      return;
    }
    const timer = setTimeout(() => (settled = true), wait);
    return () => clearTimeout(timer);
  });

  let open = $derived(settled && !card.blocked);
</script>

<section class="card" aria-label={card.kind}>
  <header>
    <h1>{card.kind}</h1>
    <span class="named">{card.named}</span>
  </header>

  <div class="block" class:long={card.long}>
    <p class="target">{card.target}</p>
    <Blocks blocks={card.content} />
  </div>

  <div class="means">
    <Blocks blocks={card.means} />
  </div>

  <p class="question">{card.question}</p>
  {#if card.blockedWhy}<p class="warn">{card.blockedWhy}</p>{/if}

  <div class="choices">
    <button class="choice" disabled={!open} onclick={() => send("allow", { id: card.id })}>{card.yes}</button>
    {#if card.yesAnd}
      <button class="choice" disabled={!open} onclick={() => send("allowNotAgain", { id: card.id })}>{card.yesAnd}</button>
    {/if}
    <div class="refuse">
      <button class="choice" onclick={() => send("refuse", { id: card.id, reason })}>{card.no}</button>
      <input type="text" bind:value={reason} maxlength="400" placeholder={card.reasonHint} aria-label={card.reasonHint} />
    </div>
    <button class="choice quiet" onclick={() => send("notNow")}>{card.notNow}</button>
  </div>

  <p class="small">{card.asker}</p>
</section>
"#),
    ("src/Blocks.svelte", r#"<script>
  // What a card holds, block by block, as GridCraft lists it: words, a file's lines, the lines
  // a change takes out and puts in, and for a plan its steps, each with blocks of its own.
  // Everything is drawn as text: nothing a request holds is ever taken as markup.
  import { send } from "./host.js";
  import Blocks from "./Blocks.svelte";

  let { blocks } = $props();

  const MARK = { gone: "-", new: "+", same: " " };
</script>

{#each blocks as block}
  {#if block.t === "note"}
    <p>{block.text}</p>
  {:else if block.t === "warn"}
    <p class="warn">{block.text}</p>
  {:else if block.t === "small"}
    <p class="small">{block.text}</p>
  {:else if block.t === "strong"}
    <p class="strong">{block.text}</p>
  {:else if block.t === "bullet"}
    <p class="bullet" class:warn={block.warn}>{block.text}</p>
  {:else if block.t === "toggle"}
    <p class="small toggle">
      <span>{block.text}</span>
      {#if block.button}
        <button class="link" onclick={() => send("whole", { whole: block.whole })}>{block.button}</button>
      {/if}
    </p>
  {:else if block.t === "text"}
    <div class="code" role="group" aria-label="The text">
      {#each block.lines as line, index}
        <div class="line">
          {#if block.numbered}<span class="no">{index + 1}</span>{/if}<span class="src">{line}</span>
        </div>
      {/each}
    </div>
    {#if block.cut}<p class="warn">{block.cut}</p>{/if}
  {:else if block.t === "diff"}
    <div class="code" role="group" aria-label="The changes">
      {#each block.rows as row}
        {#if row.k === "skip"}
          <div class="line skip"><span class="no"></span><span class="mark"></span><span class="src">⋯ {row.text}</span></div>
        {:else}
          <div class="line {row.k}"><span class="no">{row.n}</span><span class="mark">{MARK[row.k] ?? " "}</span><span class="src">{row.text}</span></div>
        {/if}
      {/each}
    </div>
    {#if block.cut}<p class="warn">{block.cut}</p>{/if}
  {:else if block.t === "step"}
    <section class="step">
      <h2><span>{block.heading}</span> <code>{block.target}</code></h2>
      <Blocks blocks={block.blocks} />
    </section>
  {/if}
{/each}
"#),
];
