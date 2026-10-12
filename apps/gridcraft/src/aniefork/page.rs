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
    ("app.js", r#"(function(){var e=Array.isArray,t=Array.prototype.indexOf,n=Array.prototype.includes,r=Array.from,i=Object.defineProperty,a=Object.getOwnPropertyDescriptor,o=Object.getOwnPropertyDescriptors,s=Object.prototype,c=Array.prototype,l=Object.getPrototypeOf,u=Object.isExtensible,d=()=>{};function f(e){for(var t=0;t<e.length;t++)e[t]()}function p(){var e,t;return{promise:new Promise((n,r)=>{e=n,t=r}),resolve:e,reject:t}}function m(e){"@babel/helpers - typeof";return m=typeof Symbol==`function`&&typeof Symbol.iterator==`symbol`?function(e){return typeof e}:function(e){return e&&typeof Symbol==`function`&&e.constructor===Symbol&&e!==Symbol.prototype?`symbol`:typeof e},m(e)}function h(e,t){if(m(e)!=`object`||!e)return e;var n=e[Symbol.toPrimitive];if(n!==void 0){var r=n.call(e,t||`default`);if(m(r)!=`object`)return r;throw TypeError(`@@toPrimitive must return a primitive value.`)}return(t===`string`?String:Number)(e)}function g(e){var t=h(e,`string`);return m(t)==`symbol`?t:t+``}function _(e,t,n){return(t=g(t))in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}var v=1024,y=2048,b=4096,x=8192,S=16384,ee=32768,te=1<<25,ne=65536,re=1<<19,ie=1<<20,ae=1<<25,oe=1<<21,se=1<<22,ce=1<<23,le=Symbol(`$state`),ue=Symbol(`component`),de=Symbol(``),fe=Symbol(`attributes`),pe=Symbol(`class`),me=Symbol(`style`),he=Symbol(`text`),ge=Symbol(`form reset`),_e=new class extends Error{constructor(...e){super(...e),_(this,`name`,`StaleReactionError`),_(this,`message`,"The reaction that called `getAbortSignal()` was re-run or destroyed")}},ve=!!globalThis.document?.contentType&&globalThis.document.contentType.includes(`xml`),ye={},C=Symbol(`uninitialized`);function be(){console.warn(`https://svelte.dev/e/derived_inert`)}function xe(e){console.warn(`https://svelte.dev/e/hydration_mismatch`)}function Se(){console.warn(`https://svelte.dev/e/svelte_boundary_reset_noop`)}var w=!1;function Ce(e){w=e}var T;function we(e){if(e===null)throw xe(),ye;return T=e}function Te(){return we(kn(T))}function E(e){if(w){if(kn(T)!==null)throw xe(),ye;T=e}}function Ee(e=1){if(w){for(var t=e,n=T;t--;)n=kn(n);T=n}}function De(e=!0){for(var t=0,n=T;;){if(n.nodeType===8){var r=n.data;if(r===`]`){if(t===0)return n;--t}else(r===`[`||r===`[!`||r[0]===`[`&&!isNaN(Number(r.slice(1))))&&(t+=1)}var i=kn(n);e&&n.remove(),n=i}}function Oe(e){if(!e||e.nodeType!==8)throw xe(),ye;return e.data}function ke(e){return e===this.v}function Ae(e,t){return e==e?e!==t||typeof e==`object`&&!!e||typeof e==`function`:t==t}function je(e){return!Ae(e,this.v)}function Me(){throw Error(`https://svelte.dev/e/async_derived_orphan`)}function Ne(e,t,n){throw Error(`https://svelte.dev/e/each_key_duplicate`)}function Pe(e){throw Error(`https://svelte.dev/e/effect_in_teardown`)}function Fe(){throw Error(`https://svelte.dev/e/effect_in_unowned_derived`)}function Ie(e){throw Error(`https://svelte.dev/e/effect_orphan`)}function Le(){throw Error(`https://svelte.dev/e/effect_update_depth_exceeded`)}function Re(){throw Error(`https://svelte.dev/e/state_descriptors_fixed`)}function ze(){throw Error(`https://svelte.dev/e/state_prototype_fixed`)}function Be(){throw Error(`https://svelte.dev/e/state_unsafe_mutation`)}function Ve(){throw Error(`https://svelte.dev/e/svelte_boundary_reset_onerror`)}var D=null;function He(e){D=e}function Ue(e,t=!1,n){D={p:D,i:!1,c:null,e:null,s:e,x:null,r:U,l:null}}function We(e){var t=D,n=t.e;if(n!==null){t.e=null;for(var r of n)Hn(r)}return e!==void 0&&(t.x=e),t.i=!0,D=t.p,Ge(e)}function Ge(e={}){return i(e,ue,{value:!0}),e}function Ke(){return!0}var qe=[];function Je(){var e=qe;qe=[],f(e)}function Ye(e){if(qe.length===0&&!Tt){var t=qe;queueMicrotask(()=>{t===qe&&Je()})}qe.push(e)}function Xe(){for(;qe.length>0;)Je()}var Ze=~(y|b|v);function O(e,t){e.f=e.f&Ze|t}function Qe(e){e.f&512||e.deps===null?O(e,v):O(e,b)}function $e(e,t,n){e.f&2048?t.add(e):e.f&4096&&n.add(e),O(e,v)}var et=!1;function tt(){et||(et=!0,document.addEventListener(`reset`,e=>{Promise.resolve().then(()=>{if(!e.defaultPrevented)for(let t of e.target.elements)t[ge]?.()})},{capture:!0}))}function nt(e){var t=H,n=U;cr(null),lr(null);try{return e()}finally{cr(t),lr(n)}}function rt(e,t,n,r=n){e.addEventListener(t,()=>nt(n));let i=e[ge];e[ge]=i?()=>{i(),r(!0)}:()=>r(!0),tt()}function it(e,t,n,r){let i=Ke()?ct:ft;var a=e.filter(e=>!e.settled),o=t.map(i);if(n.length===0&&a.length===0){r(o);return}var s=U,c=at(),l=a.length===1?a[0].promise:a.length>1?Promise.all(a.map(e=>e.promise)):null;function u(e){if(!(s.f&16384)){c();try{r([...o,...e])}catch(e){Fn(e,s)}ot()}}var d=st();if(n.length===0){l.then(()=>u([])).finally(d);return}function f(){Promise.all(n.map(e=>ut(e))).then(u).catch(e=>Fn(e,s)).finally(d)}l?l.then(()=>{s.f&16384?d():(c(),f(),ot())}):f()}function at(){var e=U,t=H,n=D,r=N;return function(i=!0){lr(e),cr(t),He(n),i&&!(e.f&16384)&&(r?.activate(),r?.apply())}}function ot(e=!0){lr(null),cr(null),He(null),e&&N?.deactivate()}function st(){var e=U,t=e.b,n=N,r=!!t?.is_rendered();return t?.update_pending_count(1,n),n.increment(r,e),()=>{t?.update_pending_count(-1,n),n.decrement(r,e)}}function ct(e){var t=2|y;return U!==null&&(U.f|=re),{ctx:D,deps:null,effects:null,equals:ke,f:t,fn:e,reactions:null,rv:0,v:C,wv:0,parent:U,ac:null}}var lt=Symbol(`obsolete`);function ut(e,t,n){let r=U;r===null&&Me();var i=void 0,a=fn(C),o=!H,s=new Set;return Wn(()=>{var t=U,n=p();i=n.promise;try{Promise.resolve(e()).then(n.resolve,e=>{e!==_e&&n.reject(e)}).finally(ot)}catch(e){n.reject(e),ot()}var c=N;if(o){if(t.f&32768)var l=st();if(r.b?.is_rendered())c.async_deriveds.get(t)?.reject(lt);else for(let e of s.values())e.reject(lt);s.add(n),c.async_deriveds.set(t,n)}let u=(e,t=void 0)=>{l?.(),s.delete(n),t!==lt&&(c.activate(),t?(a.f|=ce,_n(a,t)):(a.f&8388608&&(a.f^=ce),_n(a,e)),c.deactivate())};n.promise.then(u,e=>u(null,e||`unknown`))}),Bn(()=>{for(let e of s)e.reject(lt)}),new Promise(e=>{function t(n){function r(){n===i?e(a):t(i)}n.then(r,r)}t(i)})}function dt(e){let t=ct(e);return dr(t),t}function ft(e){let t=ct(e);return t.equals=je,t}function pt(e){var t=e.effects;if(t!==null){e.effects=null;for(var n=0;n<t.length;n+=1)V(t[n])}}function mt(e){var t,n=U,r=e.parent;if(!ar&&r!==null&&e.v!==C&&r.f&24576)return be(),e.v;lr(r);try{pt(e),t=Sr(e)}finally{lr(n)}return t}function ht(e){var t=mt(e);!e.equals(t)&&(e.wv=yr(),(!N?.is_fork||e.deps===null)&&(N===null?e.v=t:(N.capture(e,t,!0),St?.capture(e,t,!0)),e.deps===null))?O(e,v):ar||(Ct===null?Qe(e):(zn()||N?.is_fork)&&Ct.set(e,t))}function gt(e){if(e.effects!==null)for(let t of e.effects)(t.teardown||t.ac)&&(t.teardown?.(),t.ac!==null&&nt(()=>{t.ac.abort(_e),t.ac=null}),t.fn!==null&&(t.teardown=d),Tr(t,0),Yn(t))}function _t(e){if(e.effects!==null)for(let t of e.effects)t.teardown&&t.fn!==null&&Er(t)}function vt(e,t){if(t.has(e))throw TypeError(`Cannot initialize the same private elements twice on an object`)}function yt(e,t){vt(e,t),t.add(e)}function k(e,t,n){vt(e,t),t.set(e,n)}function A(e,t,n){if(typeof e==`function`?e===t:e.has(t))return arguments.length<3?t:n;throw TypeError(`Private element is not present on this object`)}function j(e,t,n){return e.set(A(e,t),n),n}function M(e,t){return e.get(A(e,t))}var bt,xt=null,N=null,St=null,Ct=null,wt=null,Tt=!1,Et=!1,Dt=null,Ot=null,kt=0,At=1,jt=new WeakMap,Mt=new WeakMap,Nt=new WeakMap,Pt=new WeakMap,Ft=new WeakMap,It=new WeakMap,Lt=new WeakMap,Rt=new WeakMap,zt=new WeakMap,Bt=new WeakMap,Vt=new WeakMap,Ht=new WeakMap,Ut=new WeakMap,Wt=new WeakMap,Gt=new WeakMap,P=new WeakSet,Kt=class e{constructor(){yt(this,P),_(this,`id`,At++),k(this,jt,!1),_(this,`linked`,!0),k(this,Mt,null),k(this,Nt,null),_(this,`async_deriveds`,new Map),_(this,`current`,new Map),_(this,`previous`,new Map),k(this,Pt,new Set),k(this,Ft,new Set),k(this,It,0),k(this,Lt,new Map),k(this,Rt,null),k(this,zt,[]),k(this,Bt,[]),k(this,Vt,new Set),k(this,Ht,new Set),k(this,Ut,new Map),k(this,Wt,new Set),_(this,`is_fork`,!1),k(this,Gt,!1),xt===null?xt=this:(j(Nt,xt,this),j(Mt,this,xt)),xt=this}skip_effect(e){M(Ut,this).has(e)||M(Ut,this).set(e,{d:[],m:[]}),M(Wt,this).delete(e)}unskip_effect(e,t=e=>this.schedule(e)){var n=M(Ut,this).get(e);if(n){M(Ut,this).delete(e);for(var r of n.d)O(r,y),t(r);for(r of n.m)O(r,b),t(r)}M(Wt,this).add(e)}capture(e,t,n=!1){e.v!==C&&!this.previous.has(e)&&this.previous.set(e,e.v),e.f&8388608||(this.current.set(e,[t,n]),Ct?.set(e,t)),this.is_fork||(e.v=t)}activate(){N=this}deactivate(){N=null,Ct=null}flush(){try{Et=!0,N=this,A(P,this,Yt).call(this)}finally{kt=0,wt=null,Dt=null,Ot=null,Et=!1,N=null,Ct=null,un.clear()}}discard(){for(let e of M(Ft,this))e(this);M(Ft,this).clear();for(let e of this.async_deriveds.values())e.reject(lt);A(P,this,en).call(this),M(Rt,this)?.resolve()}register_created_effect(e){M(Bt,this).push(e)}increment(e,t){if(j(It,this,M(It,this)+1),e){let e=M(Lt,this).get(t)??0;M(Lt,this).set(t,e+1)}}decrement(e,t){if(j(It,this,M(It,this)-1),e){let e=M(Lt,this).get(t)??0;e===1?M(Lt,this).delete(t):M(Lt,this).set(t,e-1)}M(Gt,this)||(j(Gt,this,!0),Ye(()=>{j(Gt,this,!1),this.linked&&this.flush()}))}transfer_effects(e,t){for(let t of e)M(Vt,this).add(t);for(let e of t)M(Ht,this).add(e);e.clear(),t.clear()}oncommit(e){M(Pt,this).add(e)}ondiscard(e){M(Ft,this).add(e)}settled(){return(M(Rt,this)??j(Rt,this,p())).promise}static ensure(){if(N===null){let t=N=new e;!Et&&!Tt&&Ye(()=>{M(jt,t)||t.flush()})}return N}apply(){Ct=null}schedule(e){wt=e,e.b?.is_pending&&e.f&16777228&&!(e.f&32768)?e.b.defer_effect(e):M(zt,this).push(e)}};bt=Kt;function qt(){if(this.is_fork)return!0;for(let n of M(Lt,this).keys()){for(var e=n,t=!1;e.parent!==null;){if(M(Ut,this).has(e)){t=!0;break}e=e.parent}if(!t)return!0}return!1}function Jt(){var e=[];for(let i of M(zt,this))if(!(i.f&16384||!(i.f&6144))){for(var t=i,n=!1;t.parent!==null;){t=t.parent;var r=t.f;if(r&96){if(!(r&1024)){n=!0;break}t.f^=v}}n||e.push(t)}return j(zt,this,[]),e}function Yt(){j(jt,this,!0);for(let e of M(Vt,this))M(Ht,this).delete(e),O(e,y),this.schedule(e);for(let e of M(Ht,this))O(e,b),this.schedule(e);this.apply();for(var e=Dt=[],t=[],n=Ot=[];M(zt,this).length>0;){kt++>1e3&&(A(P,this,en).call(this),nn());for(let n of A(P,this,Jt).call(this))try{A(P,this,Xt).call(this,n,e,t)}catch(e){throw cn(n),A(P,this,qt).call(this)||this.discard(),e}}if(N=null,n.length>0){var r=bt.ensure();for(let e of n)r.schedule(e)}if(Dt=null,Ot=null,A(P,this,qt).call(this)){A(P,this,$t).call(this,t),A(P,this,$t).call(this,e);for(let[e,t]of M(Ut,this))sn(e,t);n.length>0&&A(P,N,Yt).call(N);return}let i=A(P,this,Zt).call(this);if(i)A(P,this,$t).call(this,t),A(P,this,$t).call(this,e),A(P,i,Qt).call(i,this);else{M(Vt,this).clear(),M(Ht,this).clear();for(let e of M(Pt,this))e(this);M(Pt,this).clear(),St=this,an(t),an(e),St=null,M(Rt,this)?.resolve();var a=N;if(M(It,this)===0&&(M(zt,this).length===0||a!==null)&&A(P,this,en).call(this),M(zt,this).length>0){if(a!==null){for(let e of M(zt,this))M(zt,a).push(e);j(zt,this,[])}else a=this}a!==null&&(un.clear(),A(P,a,Yt).call(a))}}function Xt(e,t,n){e.f^=v;for(var r=e.first;r!==null;){var i=r.f,a=!!(i&96);if(!(a&&i&1024||i&8192||M(Ut,this).has(r))&&r.fn!==null){a?r.f^=v:i&4?t.push(r):br(r)&&(i&16&&M(Ht,this).add(r),Er(r));var o=r.first;if(o!==null){r=o;continue}}for(;r!==null;){var s=r.next;if(s!==null){r=s;break}r=r.parent}}}function Zt(){for(var e=M(Mt,this);e!==null;){if(!e.is_fork){for(let[t,[,n]]of this.current)if(e.current.has(t)&&!n)return e}e=M(Mt,e)}return null}function Qt(e){for(let[t,n]of e.current)!this.previous.has(t)&&e.previous.has(t)&&this.previous.set(t,e.previous.get(t)),this.current.set(t,n);for(let[t,n]of e.async_deriveds){let e=this.async_deriveds.get(t);e&&n.promise.then(e.resolve).catch(e.reject)}e.async_deriveds.clear(),this.transfer_effects(M(Vt,e),M(Ht,e));let t=e=>{var n=e.reactions;if(n!==null&&!(e.f&2&&!(e.f&6144)))for(let e of n){var r=e.f;if(r&2)t(e);else{var i=e;r&4194320&&!this.async_deriveds.has(i)&&(M(Ht,this).delete(i),O(i,y),this.schedule(i))}}};for(let e of this.current.keys())t(e);this.oncommit(()=>e.discard()),A(P,e,en).call(e),N=this,A(P,this,Yt).call(this)}function $t(e){for(var t=0;t<e.length;t+=1)$e(e[t],M(Vt,this),M(Ht,this))}function en(){if(this.linked){var e=M(Mt,this),t=M(Nt,this);e===null||j(Nt,e,t),t===null?xt=e:j(Mt,t,e),this.linked=!1}}function tn(e){var t=Tt,n=St;St=null,Tt=!0;try{var r;for(e&&(tn(),r=e());;){if(Xe(),N===null)return r;N.flush()}}finally{Tt=t,St=n}}function nn(){try{Le()}catch(e){Fn(e,wt)}}var rn=null;function an(e){var t=e.length;if(t!==0){for(var n=0;n<t;){var r=e[n++];if(!(r.f&24576)&&br(r)&&(rn=new Set,Er(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&Qn(r),rn?.size>0)){un.clear();for(let e of rn){if(e.f&24576)continue;let t=[e],n=e.parent;for(;n!==null;)rn.has(n)&&(rn.delete(n),t.push(n)),n=n.parent;for(let e=t.length-1;e>=0;e--){let n=t[e];n.f&24576||Er(n)}}rn.clear()}}rn=null}}function on(e){N.schedule(e)}function sn(e,t){if(!(e.f&32&&e.f&1024)){e.f&2048?t.d.push(e):e.f&4096&&t.m.push(e),O(e,v);for(var n=e.first;n!==null;)sn(n,t),n=n.next}}function cn(e){O(e,v);for(var t=e.first;t!==null;)cn(t),t=t.next}var ln=new Set,un=new Map,dn=!1;function fn(e,t){return{f:0,v:e,reactions:null,equals:ke,rv:0,wv:0}}function pn(e,t){let n=fn(e,t);return dr(n),n}function mn(e,t=!1,n=!0){let r=fn(e);return t||(r.equals=je),r}function F(e,t,n=!1){return H!==null&&(!sr||H.f&131072)&&Ke()&&H.f&4325394&&(ur===null||!ur.has(e))&&Be(),_n(e,n?xn(t):t,Ot)}var hn=null,gn=0;function _n(e,t,n=null){if(!e.equals(t)){ar?un.set(e,t):un.has(e)||un.set(e,e.v);var r=Kt.ensure();if(r.capture(e,t),e.f&2){let t=e;e.f&2048&&mt(t),Ct===null&&Qe(t)}e.wv=yr(),hn=null,gn=0,bn(e,y,n),hn=null,Ke()&&U!==null&&U.f&1024&&!(U.f&96)&&(pr===null?mr([e]):pr.push(e)),!r.is_fork&&ln.size>0&&!dn&&vn()}return t}function vn(){dn=!1;for(let e of ln){e.f&1024&&O(e,b);let t;try{t=br(e)}catch{t=!0}t&&Er(e)}ln.clear()}function yn(e){F(e,e.v+1)}function bn(e,t,n){var r=e.reactions;if(r!==null){var i=Ke(),a=r.length;if(gn+=a,gn>1e5&&hn===null&&(hn=new Set),hn!==null){if(hn.has(e))return;hn.add(e)}for(var o=0;o<a;o++){var s=r[o],c=s.f;if(i||s!==U){var l=(c&y)===0;if(l&&O(s,t),c&131072)ln.add(s);else if(c&2){var u=s;Ct?.delete(u),bn(u,b,n)}else if(l){var d=s;c&16&&rn!==null&&rn.add(d),n===null?on(d):n.push(d)}}}}}function xn(t){if(typeof t!=`object`||!t||le in t||ue in t)return t;let n=l(t);if(n!==s&&n!==c)return t;var r=new Map,i=e(t),o=pn(0),u=null,d=_r,f=e=>{if(_r===d)return e();var t=H,n=_r;cr(null),vr(d);var r=e();return cr(t),vr(n),r};return i&&r.set(`length`,pn(t.length,u)),new Proxy(t,{defineProperty(e,t,n){(!(`value`in n)||n.configurable===!1||n.enumerable===!1||n.writable===!1)&&Re();var i=r.get(t);return i===void 0?f(()=>{var e=pn(n.value,u);return r.set(t,e),e}):F(i,n.value,!0),!0},deleteProperty(e,t){var n=r.get(t);if(n===void 0){if(t in e){let e=f(()=>pn(C,u));r.set(t,e),yn(o)}}else F(n,C),yn(o);return!0},get(e,n,i){if(n===le)return t;var o=r.get(n),s=n in e;if(o===void 0&&(!s||a(e,n)?.writable)&&(o=f(()=>pn(xn(s?e[n]:C),u)),r.set(n,o)),o!==void 0){var c=G(o);return c===C?void 0:c}return Reflect.get(e,n,i)},getOwnPropertyDescriptor(e,t){this.has?.(e,t);var n=Reflect.getOwnPropertyDescriptor(e,t),i=r.get(t);if(i!==void 0){var a=G(i);if(a===C)return;if(n&&`value`in n)n.value=a;else return{enumerable:!0,configurable:!0,value:a,writable:!0}}return n},has(e,t){if(t===le)return!0;var n=r.get(t),i=n!==void 0&&n.v!==C||Reflect.has(e,t);return(n!==void 0||U!==null&&(!i||a(e,t)?.writable))&&(n===void 0&&(n=f(()=>pn(i?xn(e[t]):C,u)),r.set(t,n)),G(n)===C)?!1:i},set(e,t,n,s){var c=r.get(t),l=t in e;if(i&&t===`length`)for(var d=n;d<c.v;d+=1){var p=r.get(d+``);p===void 0?d in e&&(p=f(()=>pn(C,u)),r.set(d+``,p)):F(p,C)}if(c===void 0)(!l||a(e,t)?.writable)&&(c=f(()=>pn(void 0,u)),F(c,xn(n)),r.set(t,c));else{l=c.v!==C;var m=f(()=>xn(n));F(c,m)}var h=Reflect.getOwnPropertyDescriptor(e,t);if(h?.set&&h.set.call(s,n),!l){if(i&&typeof t==`string`){var g=r.get(`length`),_=Number(t);Number.isInteger(_)&&_>=g.v&&F(g,_+1)}yn(o)}return!0},ownKeys(e){G(o);var t=Reflect.ownKeys(e).filter(e=>{var t=r.get(e);return t===void 0||t.v!==C});for(var[n,i]of r)i.v!==C&&!(n in e)&&t.push(n);return t},setPrototypeOf(){ze()}})}var Sn,Cn,wn,Tn;function En(){if(Sn===void 0){Sn=window,Cn=/Firefox/.test(navigator.userAgent);var e=Element.prototype,t=Node.prototype,n=Text.prototype;wn=a(t,`firstChild`).get,Tn=a(t,`nextSibling`).get,u(e)&&(e[pe]=void 0,e[fe]=null,e[me]=void 0,e.__e=void 0),u(n)&&(n[he]=void 0)}}function Dn(e=``){return document.createTextNode(e)}function On(e){return wn.call(e)}function kn(e){return Tn.call(e)}function I(e,t){if(!w)return On(e);var n=On(T);if(n===null)n=T.appendChild(Dn());else if(t&&n.nodeType!==3){var r=Dn();return n?.before(r),we(r),r}return t&&Nn(n),we(n),n}function L(e,t=!1){if(!w){var n=On(e);return n instanceof Comment&&n.data===``?kn(n):n}if(t){if(T?.nodeType!==3){var r=Dn();return T?.before(r),we(r),r}Nn(T)}return T}function R(e,t=!1){if(!w)return On(e);var n=I(e,t);return E(e),n}function z(e,t=1,n=!1){let r=w?T:e;for(var i;t--;)i=r,r=kn(r);if(!w)return r;if(n){if(r?.nodeType!==3){var a=Dn();return r===null?i?.after(a):r.before(a),we(a),a}Nn(r)}return we(r),r}function An(e){e.textContent=``}function jn(){return!1}function Mn(e,t,n){return t==null||t===`http://www.w3.org/1999/xhtml`?n?document.createElement(e,{is:n}):document.createElement(e):n?document.createElementNS(t,e,{is:n}):document.createElementNS(t,e)}function Nn(e){if(e.nodeValue.length<65536)return;let t=e.nextSibling;for(;t!==null&&t.nodeType===3;)t.remove(),e.nodeValue+=t.nodeValue,t=e.nextSibling}function Pn(e){var t=U;if(t===null)return H.f|=ce,e;if(!(t.f&32768)&&!(t.f&4))throw e;Fn(e,t)}function Fn(e,t){if(e===ye)throw e;if(!(t!==null&&t.f&16384)){for(;t!==null;){if(t.f&128&&!(t.f&33570816)){if(!(t.f&32768))throw e;try{t.b.error(e);return}catch(t){e=t}}t=t.parent}throw e}}function In(e){U===null&&(H===null&&Ie(e),Fe()),ar&&Pe(e)}function Ln(e,t){var n=t.last;n===null?t.last=t.first=e:(n.next=e,e.prev=n,t.last=e)}function Rn(e,t){var n=U;n!==null&&n.f&8192&&(e|=x);var r={ctx:D,deps:null,nodes:null,f:e|y|512,first:null,fn:t,last:null,next:null,parent:n,b:n&&n.b,prev:null,teardown:null,wv:0,ac:null};N?.register_created_effect(r);var i=r;if(e&4)Dt===null?Kt.ensure().schedule(r):Dt.push(r);else if(t!==null){try{Er(r)}catch(e){throw V(r),e}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&!(i.f&524288)&&(i=i.first,e&16&&e&65536&&i!==null&&(i.f|=ne))}if(i!==null&&(i.parent=n,n!==null&&Ln(i,n),H!==null&&H.f&2&&!(e&64))){var a=H;(a.effects??(a.effects=[])).push(i)}return r}function zn(){return H!==null&&!sr}function Bn(e){let t=Rn(8,null);return O(t,v),t.teardown=e,t}function Vn(e){In(`$effect`);var t=U.f;if(!H&&t&32&&D!==null&&!D.i){var n=D;(n.e??(n.e=[])).push(e)}else return Hn(e)}function Hn(e){return Rn(4|ie,e)}function Un(e){Kt.ensure();let t=Rn(64|re,e);return(e={})=>new Promise(n=>{e.outro?$n(t,()=>{V(t),n(void 0)}):(V(t),n(void 0))})}function Wn(e){return Rn(se|re,e)}function Gn(e,t=0){return Rn(8|t,e)}function B(e,t=[],n=[],r=[]){it(r,t,n,t=>{Rn(8,()=>{e(...t.map(G))})})}function Kn(e,t=0){return Rn(16|t,e)}function qn(e){return Rn(32|re,e)}function Jn(e){var t=e.teardown;if(t!==null){let n=ar,r=H;or(!0),cr(null);try{t.call(null)}catch(t){Fn(t,e.parent)}finally{or(n),cr(r)}}}function Yn(e,t=!1){var n=e.first;for(e.first=e.last=null;n!==null;){let e=n.ac;e!==null&&nt(()=>{e.abort(_e)});var r=n.next;n.f&64?n.parent=null:V(n,t),n=r}}function Xn(e){for(var t=e.first;t!==null;){var n=t.next;t.f&32||V(t),t=n}}function V(e,t=!0){var n=!1;(t||e.f&262144)&&e.nodes!==null&&e.nodes.end!==null&&(Zn(e.nodes.start,e.nodes.end),n=!0),e.f|=te,Yn(e,t&&!n),Tr(e,0);var r=e.nodes&&e.nodes.t;if(r!==null)for(let e of r)e.stop();Jn(e),e.f^=te,e.f|=S;var i=e.parent;i!==null&&i.first!==null&&Qn(e),e.next=e.prev=e.teardown=e.ctx=e.deps=e.fn=e.nodes=e.ac=e.b=null}function Zn(e,t){for(;e!==null;){var n=e===t?null:kn(e);e.remove(),e=n}}function Qn(e){var t=e.parent,n=e.prev,r=e.next;n!==null&&(n.next=r),r!==null&&(r.prev=n),t!==null&&(t.first===e&&(t.first=r),t.last===e&&(t.last=n))}function $n(e,t,n=!0){var r=[];e.f|=256,er(e,r,!0);var i=()=>{n&&V(e),t&&t()},a=r.length;if(a>0){var o=()=>--a||i();for(var s of r)s.out(o)}else i()}function er(e,t,n){if(!(e.f&8192)){e.f^=x;var r=e.nodes&&e.nodes.t;if(r!==null)for(let e of r)(e.is_global||n)&&t.push(e);for(var i=e.first;i!==null;){var a=i.next;if(!(i.f&64)){var o=!!(i.f&65536)||!!(i.f&32)&&!!(e.f&16);er(i,t,o?n:!1)}i=a}}}function tr(e){e.f&=-257,nr(e,!0)}function nr(e,t){if(!(e.f&256)&&e.f&8192){e.f^=x,e.f&1024||(O(e,y),Kt.ensure().schedule(e));for(var n=e.first;n!==null;){var r=n.next,i=!!(n.f&65536)||!!(n.f&32);nr(n,i?t:!1),n=r}var a=e.nodes&&e.nodes.t;if(a!==null)for(let e of a)(e.is_global||t)&&e.in()}}function rr(e,t){if(e.nodes)for(var n=e.nodes.start,r=e.nodes.end;n!==null;){var i=n===r?null:kn(n);t.append(n),n=i}}var ir=null,ar=!1;function or(e){ar=e}var H=null,sr=!1;function cr(e){H=e}var U=null;function lr(e){U=e}var ur=null;function dr(e){H!==null&&(H.f&2097152||H.f&2)&&(ur??(ur=new Set)).add(e)}var W=null,fr=0,pr=null;function mr(e){pr=e}var hr=1,gr=0,_r=gr;function vr(e){_r=e}function yr(){return++hr}function br(e){var t=e.f;if(t&2048)return!0;if(t&4096){for(var n=e.deps,r=n.length,i=0;i<r;i++){var a=n[i];if(br(a)&&ht(a),a.wv>e.wv)return!0}t&512&&Ct===null&&O(e,v)}return!1}function xr(e,t,n=!0){var r=e.reactions;if(r!==null&&!(ur!==null&&ur.has(e)))for(var i=0;i<r.length;i++){var a=r[i];a.f&2?xr(a,t,!1):t===a&&(n?O(a,y):a.f&1024&&O(a,b),on(a))}}function Sr(e){var t=W,n=fr,r=pr,i=H,a=ur,o=D,s=sr,c=_r,l=e.f;W=null,fr=0,pr=null,H=l&96?null:e,ur=null,He(e.ctx),sr=!1,_r=++gr,e.ac!==null&&(nt(()=>{e.ac.abort(_e)}),e.ac=null);try{e.f|=oe;var u=e.fn,d=u();e.f|=ee;var f=Cr(e);if(Ke()&&pr!==null&&!sr&&f!==null&&!(e.f&6146))for(var p=0;p<pr.length;p++)xr(pr[p],e);if(i!==null&&i!==e){if(gr++,i.deps!==null)for(let e=0;e<n;e+=1)i.deps[e].rv=gr;if(t!==null)for(let e of t)e.rv=gr;pr!==null&&(r===null?r=pr:r.push(...pr))}return e.f&8388608&&(e.f^=ce),d}catch(t){return Cr(e),Pn(t)}finally{e.f^=oe,W=t,fr=n,pr=r,H=i,ur=a,He(o),sr=s,_r=c}}function Cr(e){var t=e.deps,n=N?.is_fork;if(W!==null){var r;if(n||Tr(e,fr),t!==null&&fr>0)for(t.length=fr+W.length,r=0;r<W.length;r++)t[fr+r]=W[r];else e.deps=t=W;if(zn()&&e.f&512)for(r=fr;r<t.length;r++){var i;((i=t[r]).reactions??(i.reactions=[])).push(e)}}else!n&&t!==null&&fr<t.length&&(Tr(e,fr),t.length=fr);return t}function wr(e,r){let i=r.reactions;if(i!==null){var a=t.call(i,e);if(a!==-1){var o=i.length-1;o===0?i=r.reactions=null:(i[a]=i[o],i.pop())}}if(i===null&&r.f&2&&(W===null||!n.call(W,r))){var s=r;s.f&512&&(s.f^=512),s.v!==C&&Qe(s),s.ac!==null&&nt(()=>{s.ac.abort(_e),s.ac=null,O(s,y)}),gt(s),Tr(s,0)}}function Tr(e,t){var n=e.deps;if(n!==null)for(var r=t;r<n.length;r++)wr(e,n[r])}function Er(e){var t=e.f;if(!(t&16384)){O(e,v);var n=U;U=e;try{t&16777232?Xn(e):Yn(e),Jn(e);var r=Sr(e);e.teardown=typeof r==`function`?r:null,e.wv=hr}finally{U=n}}}async function Dr(){await Promise.resolve(),tn()}function G(e){var t=!!(e.f&2);if(ir?.add(e),H!==null&&!sr&&!(U!==null&&U.f&16384)&&(ur===null||!ur.has(e))){var r=H.deps;if(H.f&2097152)e.rv<gr&&(e.rv=gr,W===null&&r!==null&&r[fr]===e?fr++:W===null?W=[e]:W.push(e));else{var i;(i=H).deps??(i.deps=[]),n.call(H.deps,e)||H.deps.push(e);var a=e.reactions;a===null?e.reactions=[H]:n.call(a,H)||a.push(H)}}if(ar&&un.has(e))return un.get(e);if(t){var o=e;if(ar){var s=o.v;return(!(o.f&1024)&&o.reactions!==null||kr(o))&&(s=mt(o)),un.set(o,s),s}var c=!(o.f&512)&&!sr&&H!==null&&!!(H.f&512),l=(o.f&ee)===0;br(o)&&(c&&(o.f|=512),ht(o)),c&&!l&&(_t(o),Or(o))}if(Ct?.has(e))return Ct.get(e);if(e.f&8388608)throw e.v;return e.v}function Or(e){if(e.f|=512,e.deps!==null)for(let r of e.deps){var t=r.reactions;t===null?r.reactions=[e]:n.call(t,e)||t.push(e),r.f&2&&!(r.f&512)&&(_t(r),Or(r))}}function kr(e){if(e.v===C)return!0;if(e.deps===null)return!1;for(let t of e.deps)if(un.has(t)||t.f&2&&kr(t))return!0;return!1}function Ar(e){var t=sr;try{return sr=!0,e()}finally{sr=t}}[...`allowfullscreen.async.autofocus.autoplay.checked.controls.default.disabled.formnovalidate.indeterminate.inert.ismap.loop.multiple.muted.nomodule.novalidate.open.playsinline.readonly.required.reversed.seamless.selected.webkitdirectory.defer.disablepictureinpicture.disableremoteplayback`.split(`.`)];var jr=[`touchstart`,`touchmove`];function Mr(e){return jr.includes(e)}var Nr=Symbol(`events`),Pr=new Set,Fr=new Set;function K(e,t,n){(t[Nr]??(t[Nr]={}))[e]=n}function Ir(e){for(var t=0;t<e.length;t++)Pr.add(e[t]);for(var n of Fr)n(e)}var Lr=null,Rr=!1;function zr(e){var t=this,n=t.ownerDocument,r=e.type,a=e.composedPath?.()||[],o=a[0]||e.target;Lr=e,Rr||(Rr=!0,setTimeout(()=>{Rr=!1,Lr=null}));var s=0,c=Lr===e&&e[Nr];if(c){var l=a.indexOf(c);if(l!==-1&&(t===document||t===window)){e[Nr]=t;return}var u=a.indexOf(t);if(u===-1)return;l<=u&&(s=l)}if(o=a[s]||e.target,o!==t){i(e,`currentTarget`,{configurable:!0,get(){return o||n}});var d=H,f=U;cr(null),lr(null);try{for(var p,m=[];o!==null&&o!==t;){try{var h=o[Nr]?.[r];h!=null&&(!o.disabled||e.target===o)&&h.call(o,e)}catch(e){p?m.push(e):p=e}if(e.cancelBubble)break;s++,o=s<a.length?a[s]:null}if(p){for(let e of m)queueMicrotask(()=>{throw e});throw p}}finally{e[Nr]=t,delete e.currentTarget,cr(d),lr(f)}}}var Br=globalThis?.window?.trustedTypes&&globalThis.window.trustedTypes.createPolicy(`svelte-trusted-html`,{createHTML:e=>e});function Vr(e){return Br?.createHTML(e)??e}function Hr(e){var t=Mn(`template`);return t.innerHTML=Vr(e.replaceAll(`<!>`,`<!---->`)),t.content}function Ur(e,t){var n=U;n.nodes===null&&(n.nodes={start:e,end:t,a:null,t:null})}function q(e,t){var n=!!(t&1),r=!!(t&2),i,a=!e.startsWith(`<!>`);return()=>{if(w)return Ur(T,null),T;i===void 0&&(i=Hr(a?e:`<!>`+e),n||(i=On(i)));var t=r||Cn?document.importNode(i,!0):i.cloneNode(!0);if(n){var o=On(t),s=t.lastChild;Ur(o,s)}else Ur(t,t);return t}}function Wr(){if(w)return Ur(T,null),T;var e=document.createDocumentFragment(),t=document.createComment(``),n=Dn();return e.append(t,n),Ur(t,n),e}function J(e,t){if(w){var n=U;(!(n.f&32768)||n.nodes.end===null)&&(n.nodes.end=T),Te()}else e!==null&&e.before(t)}function Gr(e){let t=0,n=fn(0),r;return()=>{zn()&&(G(n),Gn(()=>(t===0&&(r=Ar(()=>e(()=>yn(n)))),t+=1,()=>{Ye(()=>{--t,t===0&&(r?.(),r=void 0,yn(n))})})))}}var Kr=ne|re;function qr(e,t,n,r){new li(e,t,n,r)}var Jr=new WeakMap,Yr=new WeakMap,Xr=new WeakMap,Zr=new WeakMap,Y=new WeakMap,Qr=new WeakMap,$r=new WeakMap,ei=new WeakMap,ti=new WeakMap,ni=new WeakMap,ri=new WeakMap,ii=new WeakMap,ai=new WeakMap,oi=new WeakMap,si=new WeakMap,ci=new WeakMap,X=new WeakSet,li=class{constructor(e,t,n,r){yt(this,X),_(this,`parent`,void 0),_(this,`is_pending`,!1),_(this,`transform_error`,void 0),k(this,Jr,void 0),k(this,Yr,w?T:null),k(this,Xr,void 0),k(this,Zr,void 0),k(this,Y,void 0),k(this,Qr,null),k(this,$r,null),k(this,ei,null),k(this,ti,null),k(this,ni,0),k(this,ri,0),k(this,ii,!1),k(this,ai,new Set),k(this,oi,new Set),k(this,si,null),k(this,ci,Gr(()=>(j(si,this,fn(M(ni,this))),()=>{j(si,this,null)}))),j(Jr,this,e),j(Xr,this,t),j(Zr,this,e=>{var t=U;t.b=this,t.f|=128,n(e)}),this.parent=U.b,this.transform_error=r??this.parent?.transform_error??(e=>e),j(Y,this,Kn(()=>{if(w){let e=Oe(M(Yr,this));Te();let t=e===`[!`;if(e.startsWith(`[?`)){let t=JSON.parse(e.slice(2));A(X,this,di).call(this,t)}else t?A(X,this,mi).call(this):A(X,this,ui).call(this)}else A(X,this,hi).call(this)},Kr)),w&&j(Jr,this,T)}defer_effect(e){$e(e,M(ai,this),M(oi,this))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!M(Xr,this).pending}update_pending_count(e,t){A(X,this,vi).call(this,e,t),j(ni,this,M(ni,this)+e),M(si,this)&&!M(ii,this)&&(j(ii,this,!0),Ye(()=>{j(ii,this,!1),M(si,this)&&_n(M(si,this),M(ni,this))}))}get_effect_pending(){return M(ci,this).call(this),G(M(si,this))}error(e){if(e===ye||!M(Xr,this).onerror&&!M(Xr,this).failed)throw e;N?.is_fork?(M(Qr,this)&&N.skip_effect(M(Qr,this)),M($r,this)&&N.skip_effect(M($r,this)),M(ei,this)&&N.skip_effect(M(ei,this)),N.oncommit(()=>{A(X,this,pi).call(this)||A(X,this,yi).call(this,e)})):A(X,this,yi).call(this,e)}};function ui(){try{j(Qr,this,qn(()=>M(Zr,this).call(this,M(Jr,this))))}catch(e){this.error(e)}}function di(e){let t=M(Xr,this).failed,{reset:n,invoke_onerror:r}=A(X,this,fi).call(this,e);Ye(r),t&&j(ei,this,qn(()=>{t(M(Jr,this),()=>e,()=>n)}))}function fi(e){var t=!1,n=!1;let r=()=>{A(X,this,pi).call(this)||(t?Se():(t=!0,n&&Ve(),M(ei,this)!==null&&$n(M(ei,this),()=>{j(ei,this,null)}),A(X,this,_i).call(this,()=>{A(X,this,hi).call(this)})))};return{reset:r,invoke_onerror:()=>{if(!A(X,this,pi).call(this))try{n=!0,M(Xr,this).onerror?.(e,r),n=!1}catch(e){Fn(e,M(Y,this)&&M(Y,this).parent)}}}}function pi(){return(M(Y,this).f&(S|te))!==0}function mi(){let e=M(Xr,this).pending;e&&(this.is_pending=!0,j($r,this,qn(()=>e(M(Jr,this)))),Ye(()=>{if(!A(X,this,pi).call(this)){var e=j(ti,this,document.createDocumentFragment()),t=Dn(),n=!1;e.append(t),j(Qr,this,A(X,this,_i).call(this,()=>{try{return qn(()=>M(Zr,this).call(this,t))}catch(e){try{this.error(e),n=!0}catch(e){Fn(e,M(Y,this).parent)}return null}})),M(Qr,this)===null?(j(ti,this,null),n&&A(X,this,gi).call(this,N)):M(ri,this)===0&&(M(Jr,this).before(e),j(ti,this,null),$n(M($r,this),()=>{j($r,this,null)}),A(X,this,gi).call(this,N))}}))}function hi(){try{if(this.is_pending=this.has_pending_snippet(),j(ri,this,0),j(ni,this,0),j(Qr,this,qn(()=>{M(Zr,this).call(this,M(Jr,this))})),M(ri,this)>0){var e=j(ti,this,document.createDocumentFragment());rr(M(Qr,this),e);let t=M(Xr,this).pending;j($r,this,qn(()=>t(M(Jr,this))))}else A(X,this,gi).call(this,N)}catch(e){this.error(e)}}function gi(e){this.is_pending=!1,e.transfer_effects(M(ai,this),M(oi,this))}function _i(e){var t=U,n=H,r=D;lr(M(Y,this)),cr(M(Y,this)),He(M(Y,this).ctx);try{return Kt.ensure(),e()}finally{lr(t),cr(n),He(r)}}function vi(e,t){if(this.has_pending_snippet())j(ri,this,M(ri,this)+e),M(ri,this)===0&&(A(X,this,gi).call(this,t),M($r,this)&&$n(M($r,this),()=>{j($r,this,null)}),M(ti,this)&&(M(Jr,this).before(M(ti,this)),j(ti,this,null)));else if(this.parent){var n;A(X,n=this.parent,vi).call(n,e,t)}}function yi(e){M(Qr,this)&&(V(M(Qr,this)),j(Qr,this,null)),M($r,this)&&(V(M($r,this)),j($r,this,null)),M(ei,this)&&(V(M(ei,this)),j(ei,this,null)),w&&(we(M(Yr,this)),Ee(),we(De()));let t=M(Xr,this).failed,n=e=>{if(A(X,this,pi).call(this))return;let{reset:n,invoke_onerror:r}=A(X,this,fi).call(this,e);r(),t&&!A(X,this,pi).call(this)&&j(ei,this,A(X,this,_i).call(this,()=>{try{return qn(()=>{var r=U;r.b=this,r.f|=128,t(M(Jr,this),()=>e,()=>n)})}catch(e){return Fn(e,M(Y,this).parent),null}}))};Ye(()=>{if(!A(X,this,pi).call(this)){var t;try{t=this.transform_error(e)}catch(e){Fn(e,M(Y,this)&&M(Y,this).parent);return}typeof t==`object`&&t&&typeof t.then==`function`?t.then(n,e=>Fn(e,M(Y,this)&&M(Y,this).parent)):n(t)}})}function Z(e,t){var n=t==null?``:typeof t==`object`?`${t}`:t;n!==(e[he]??(e[he]=e.nodeValue))&&(e[he]=n,e.nodeValue=`${n}`)}function bi(e,t){return Si(e,t)}var xi=new Map;function Si(e,{target:t,anchor:n,props:i={},events:a,context:o,intro:s=!0,transformError:c}){En();var l=void 0,u=Un(()=>{var s=n??t.appendChild(Dn());qr(s,{pending:()=>{}},t=>{Ue({});var n=D;if(o&&(n.c=o),a&&(i.$$events=a),w&&Ur(t,null),l=e(t,i)||Ge(),w&&(U.nodes.end=T,T===null||T.nodeType!==8||T.data!==`]`))throw xe(),ye;We()},c);var u=new Set,d=e=>{for(var n=0;n<e.length;n++){var r=e[n];if(!u.has(r)){u.add(r);var i=Mr(r);for(let e of[t,document]){var a=xi.get(e);a===void 0&&(a=new Map,xi.set(e,a));var o=a.get(r);o===void 0?(e.addEventListener(r,zr,{passive:i}),a.set(r,1)):a.set(r,o+1)}}}};return d(r(Pr)),Fr.add(d),()=>{for(var e of u)for(let n of[t,document]){var r=xi.get(n),i=r.get(e);--i==0?(n.removeEventListener(e,zr),r.delete(e),r.size===0&&xi.delete(n)):r.set(e,i)}Fr.delete(d),s!==n&&s.parentNode?.removeChild(s)}});return Ci.set(l,u),l}var Ci=new WeakMap,wi=new WeakMap,Ti=new WeakMap,Ei=new WeakMap,Di=new WeakMap,Oi=new WeakMap,ki=new WeakMap,Ai=new WeakMap,ji=class{constructor(e,t=!0){_(this,`anchor`,void 0),k(this,wi,new Map),k(this,Ti,new Map),k(this,Ei,new Map),k(this,Di,new Set),k(this,Oi,!0),k(this,ki,e=>{if(M(wi,this).has(e)){var t=M(wi,this).get(e),n=M(Ti,this).get(t);if(n)tr(n),M(Di,this).delete(t);else{var r=M(Ei,this).get(t);r&&(tr(r.effect),M(Ti,this).set(t,r.effect),M(Ei,this).delete(t),r.fragment.lastChild.remove(),this.anchor.before(r.fragment),n=r.effect)}for(let[t,n]of M(wi,this)){if(M(wi,this).delete(t),t===e)break;let r=M(Ei,this).get(n);r&&(V(r.effect),M(Ei,this).delete(n))}for(let[e,r]of M(Ti,this)){if(e===t||M(Di,this).has(e))continue;let i=()=>{if(Array.from(M(wi,this).values()).includes(e)){var t=document.createDocumentFragment();rr(r,t),t.append(Dn()),M(Ei,this).set(e,{effect:r,fragment:t})}else V(r);M(Di,this).delete(e),M(Ti,this).delete(e)};M(Oi,this)||!n?(M(Di,this).add(e),$n(r,i,!1)):i()}}}),k(this,Ai,e=>{M(wi,this).delete(e);let t=Array.from(M(wi,this).values());for(let[e,n]of M(Ei,this))t.includes(e)||(V(n.effect),M(Ei,this).delete(e))}),this.anchor=e,j(Oi,this,t)}ensure(e,t){var n=N,r=jn();if(t&&!M(Ti,this).has(e)&&!M(Ei,this).has(e)){if(r){var i=document.createDocumentFragment(),a=Dn();i.append(a),M(Ei,this).set(e,{effect:qn(()=>t(a)),fragment:i})}else M(Ti,this).set(e,qn(()=>t(this.anchor)))}if(M(wi,this).set(n,e),r){for(let[t,r]of M(Ti,this))t===e?n.unskip_effect(r):n.skip_effect(r);for(let[t,r]of M(Ei,this))t===e?n.unskip_effect(r.effect):n.skip_effect(r.effect);n.oncommit(M(ki,this)),n.ondiscard(M(Ai,this))}else w&&(this.anchor=T),M(ki,this).call(this,n)}};function Q(e,t,n=!1){var r;w&&(r=T,Te());var i=new ji(e),a=n?ne:0;function o(e,t){if(w){var n=Oe(r);if(e!==parseInt(n.substring(1))){var a=De();we(a),i.anchor=a,Ce(!1),i.ensure(e,t),Ce(!0);return}}i.ensure(e,t)}Kn(()=>{var e=!1;t((t,n=0)=>{e=!0,o(n,t)}),e||o(-1,null)},a)}var Mi=Symbol(`NaN`);function Ni(e,t,n){w&&Te();var r=new ji(e),i=!Ke();Kn(()=>{var e=t();e!==e&&(e=Mi),i&&typeof e==`object`&&e&&(e={}),r.ensure(e,n)})}function Pi(e,t){return t}function Fi(e,t,n){for(var i=[],a=t.length,o,s=t.length,c=0;c<a;c++){let n=t[c];$n(n,()=>{if(o){if(o.pending.delete(n),o.done.add(n),o.pending.size===0){var t=e.outrogroups;Ii(e,r(o.done)),t.delete(o),t.size===0&&(e.outrogroups=null)}}else--s},!1)}if(s===0){var l=i.length===0&&n!==null&&e.pending.size===0;if(l){var u=n,d=u.parentNode;An(d),d.append(u),e.items.clear()}Ii(e,t,!l)}else o={pending:new Set(t),done:new Set},(e.outrogroups??(e.outrogroups=new Set)).add(o)}function Ii(e,t,n=!0){var r;if(e.pending.size>0){r=new Set;for(let t of e.pending.values())for(let n of t)r.add(e.items.get(n).e)}for(var i=0;i<t.length;i++){var a=t[i];r?.has(a)?(a.f|=ae,rr(a,document.createDocumentFragment())):V(t[i],n)}}var Li;function Ri(t,n,i,a,o,s=null){var c=t,l=new Map;if(n&4){var u=t;c=w?we(On(u)):u.appendChild(Dn())}w&&Te();var d=null,f=ft(()=>{var t=i();return e(t)?t:t==null?[]:r(t)}),p=new Map,m=!0;function h(e){if(!(_.effect.f&16384)){_.pending.delete(e);var t=G(f);_.fallback=d,Bi(_,t,c,n,a),d!==null&&(t.length===0?d.f&33554432?(d.f^=ae,Hi(d,null,c)):tr(d):$n(d,()=>{d=null}))}}function g(e){_.pending.delete(e)}var _={effect:Kn(()=>{var e=G(f),t=e.length;let r=!1;w&&Oe(c)===`[!`!=(t===0)&&(c=De(),we(c),Ce(!1),r=!0);for(var u=new Set,_=N,v=jn(),y=0;y<t;y+=1){w&&T.nodeType===8&&T.data===`]`&&(c=T,r=!0,Ce(!1));var b=e[y],x=a(b,y),S=m?null:l.get(x);S?(S.v&&_n(S.v,b),S.i&&_n(S.i,y),v&&_.unskip_effect(S.e)):(S=Vi(l,m?c:Li??(Li=Dn()),b,x,y,o,n,i),m||(S.e.f|=ae),l.set(x,S)),u.add(x)}if(t===0&&s&&!d&&(m?d=qn(()=>s(c)):(d=qn(()=>s(Li??(Li=Dn()))),d.f|=ae)),t>u.size&&Ne(``,``,``),w&&t>0&&we(De()),!m){if(p.set(_,u),v){for(let[e,t]of l)u.has(e)||_.skip_effect(t.e);_.oncommit(h),_.ondiscard(g)}else h(_)}r&&Ce(!0),G(f)}),flags:n,items:l,pending:p,outrogroups:null,fallback:d};m=!1,w&&(c=T)}function zi(e){for(;e!==null&&!(e.f&32);)e=e.next;return e}function Bi(e,t,n,i,a){var o=!!(i&8),s=t.length,c=e.items,l=zi(e.effect.first),u,d=null,f,p=[],m=[],h,g,_,v;if(o)for(v=0;v<s;v+=1)h=t[v],g=a(h,v),_=c.get(g).e,_.f&33554432||(_.nodes?.a?.measure(),(f??(f=new Set)).add(_));for(v=0;v<s;v+=1){if(h=t[v],g=a(h,v),_=c.get(g).e,e.outrogroups!==null)for(let t of e.outrogroups)t.pending.delete(_),t.done.delete(_);if(_.f&8192&&(tr(_),o&&(_.nodes?.a?.unfix(),(f??(f=new Set)).delete(_))),_.f&33554432){if(_.f^=ae,_===l)Hi(_,null,n);else{var y=d?d.next:l;_===e.effect.last&&(e.effect.last=_.prev),_.prev&&(_.prev.next=_.next),_.next&&(_.next.prev=_.prev),Ui(e,d,_),Ui(e,_,y),Hi(_,y,n),d=_,p=[],m=[],l=zi(d.next);continue}}if(_!==l){if(u!==void 0&&u.has(_)){if(p.length<m.length){var b=m[0],x;d=b.prev;var S=p[0],ee=p[p.length-1];for(x=0;x<p.length;x+=1)Hi(p[x],b,n);for(x=0;x<m.length;x+=1)u.delete(m[x]);Ui(e,S.prev,ee.next),Ui(e,d,S),Ui(e,ee,b),l=b,d=ee,--v,p=[],m=[]}else u.delete(_),Hi(_,l,n),Ui(e,_.prev,_.next),Ui(e,_,d===null?e.effect.first:d.next),Ui(e,d,_),d=_;continue}for(p=[],m=[];l!==null&&l!==_;)(u??(u=new Set)).add(l),m.push(l),l=zi(l.next);if(l===null)continue}_.f&33554432||p.push(_),d=_,l=zi(_.next)}if(e.outrogroups!==null){for(let t of e.outrogroups)t.pending.size===0&&(Ii(e,r(t.done)),e.outrogroups?.delete(t));e.outrogroups.size===0&&(e.outrogroups=null)}if(l!==null||u!==void 0){var te=[];if(u!==void 0)for(_ of u)_.f&8192||te.push(_);for(;l!==null;)!(l.f&8192)&&l!==e.fallback&&te.push(l),l=zi(l.next);var ne=te.length;if(ne>0){var re=i&4&&s===0?n:null;if(o){for(v=0;v<ne;v+=1)te[v].nodes?.a?.measure();for(v=0;v<ne;v+=1)te[v].nodes?.a?.fix()}Fi(e,te,re)}}o&&Ye(()=>{if(f!==void 0)for(_ of f)_.nodes?.a?.apply()})}function Vi(e,t,n,r,i,a,o,s){var c=o&1?o&16?fn(n):mn(n,!1,!1):null,l=o&2?fn(i):null;return{v:c,i:l,e:qn(()=>(a(t,c??n,l??i,s),()=>{e.delete(r)}))}}function Hi(e,t,n){if(e.nodes)for(var r=e.nodes.start,i=e.nodes.end,a=t&&!(t.f&33554432)?t.nodes.start:n;r!==null;){var o=kn(r);if(a.before(r),r===i)return;r=o}}function Ui(e,t,n){t===null?e.effect.first=n:t.next=n,n===null?e.effect.last=t:n.prev=t}var Wi=[...` 	
\r\f\xA0\v﻿`];function Gi(e,t,n){var r=e==null?``:``+e;if(t&&(r=r?r+` `+t:t),n){for(var i of Object.keys(n))if(n[i])r=r?r+` `+i:i;else if(r.length)for(var a=i.length,o=0;(o=r.indexOf(i,o))>=0;){var s=o+a;(o===0||Wi.includes(r[o-1]))&&(s===r.length||Wi.includes(r[s]))?r=(o===0?``:r.substring(0,o))+r.substring(s+1):o=s}}return r===``?null:r}function Ki(e,t,n,r,i,a){var o=e[pe];if(w||o!==n||o===void 0){var s=Gi(n,r,a);(!w||s!==e.getAttribute(`class`))&&(s==null?e.removeAttribute(`class`):t?e.className=s:e.setAttribute(`class`,s)),e[pe]=n}else if(a&&i!==a)for(var c in a){var l=!!a[c];(i==null||l!==!!i[c])&&e.classList.toggle(c,l)}return a}var qi=Symbol(`is custom element`),Ji=Symbol(`is html`),Yi=ve?`link`:`LINK`;function Xi(e){if(w){var t=!1,n=()=>{if(!t){if(t=!0,e.hasAttribute(`value`)){var n=e.value;Qi(e,`value`,null),e.value=n}if(e.hasAttribute(`checked`)){var r=e.checked;Qi(e,`checked`,null),e.checked=r}}};e[ge]=n,Ye(n),tt()}}function Zi(e,t){var n=$i(e);n.checked!==(n.checked=t??void 0)&&(e.checked=t)}function Qi(e,t,n,r){var i=$i(e);w&&(i[t]=e.getAttribute(t),t===`src`||t===`srcset`||t===`href`&&e.nodeName===Yi)||i[t]!==(i[t]=n)&&(t===`loading`&&(e[de]=n),n==null?e.removeAttribute(t):typeof n!=`string`&&ta(e).has(t)?e[t]=n:e.setAttribute(t,n))}function $i(e){return e[fe]??(e[fe]={[qi]:e.nodeName.includes(`-`),[Ji]:e.namespaceURI===`http://www.w3.org/1999/xhtml`})}var ea=new Map;function ta(e){var t=e.getAttribute(`is`)||e.nodeName,n=ea.get(t);if(n)return n;ea.set(t,n=new Set);for(var r,i=e,a=Element.prototype;a!==i;){for(var s in r=o(i),r)r[s].set&&s!==`innerHTML`&&s!==`textContent`&&s!==`innerText`&&n.add(s);i=l(i)}return n}function na(e,t,n=t){var r=new WeakSet;rt(e,`input`,async i=>{var a=i?e.defaultValue:e.value;if(a=ra(e)?ia(a):a,n(a),N!==null&&r.add(N),await Dr(),a!==(a=t())){var o=e.selectionStart,s=e.selectionEnd,c=e.value.length;if(e.value=a??``,s!==null){var l=e.value.length;o===s&&s===c&&l>c?(e.selectionStart=l,e.selectionEnd=l):(e.selectionStart=o,e.selectionEnd=Math.min(s,l))}}}),(w&&e.defaultValue!==e.value||Ar(t)==null&&e.value)&&(n(ra(e)?ia(e.value):e.value),N!==null&&r.add(N)),Gn(()=>{var n=t();if(e===document.activeElement){var i=N;if(r.has(i))return}ra(e)&&n===ia(e.value)||(e.type!==`date`||n||e.value)&&n!==e.value&&(e.value=n??``)})}function ra(e){var t=e.type;return t===`number`||t===`range`}function ia(e){return e===``?null:+e}var aa,oa=new WeakMap,sa=new WeakMap,ca=new WeakMap,la=new WeakSet,ua=class{constructor(e){yt(this,la),k(this,oa,new WeakMap),k(this,sa,void 0),k(this,ca,void 0),j(ca,this,e)}observe(e,t){var n=M(oa,this).get(e)||new Set;return n.add(t),M(oa,this).set(e,n),A(la,this,da).call(this).observe(e,M(ca,this)),()=>{var n=M(oa,this).get(e);n.delete(t),n.size===0&&(M(oa,this).delete(e),M(sa,this).unobserve(e))}}};aa=ua;function da(){return M(sa,this)??j(sa,this,new ResizeObserver(e=>{for(var t of e){aa.entries.set(t.target,t);for(var n of M(oa,this).get(t.target)||[])n(t)}}))}if(_(ua,`entries`,new WeakMap),typeof window<`u`){var fa,pa;((fa=(pa=window).__svelte??(pa.__svelte={})).v??(fa.v=new Set)).add(`5`)}var ma=xn({data:null});function ha(e){ma.data=e}function $(e,t={}){let n=JSON.stringify({...t,act:e});window.ipc&&typeof window.ipc.postMessage==`function`&&window.ipc.postMessage(n)}var ga=q(`<p> </p>`),_a=q(`<label class="level"><input type="radio" name="level"/> <span> </span></label>`),va=q(`<p class="small"> </p>`),ya=q(`<div class="row"><button title="Copies the command that adds this server to Claude Code. It holds the token.">Copy Setup for Claude Code</button> <button title="Copies an MCP settings entry, for other agents. It holds the token.">Copy Setup as JSON</button> <button title="Makes a new token: an agent that holds the old one can no longer connect">New Token</button></div>`),ba=q(`<div class="item"><span class="strong"> </span> <code> </code> <span class="grow"></span> <span class="small"> </span> <button title="See everything this request would do, and answer it">Look…</button></div>`),xa=q(`<h2> </h2> <!>`,1),Sa=q(`<button title="None of its steps is carried out after this. What was done stays done.">Stop Plan</button>`),Ca=q(`<li><span> </span><span class="state"> </span></li>`),wa=q(`<div class="item plan"><div class="head"><span class="strong"> </span> <span> </span> <span class="grow"></span> <span class="small"> </span> <!></div> <ol></ol></div>`),Ta=q(`<h2>Plans</h2> <!>`,1),Ea=q(`<div class="row"><span> </span> <button class="link">Ask again</button></div>`),Da=q(`<h2>Not asked again until GridCraft closes</h2> <!>`,1),Oa=q(`<h2>Lately</h2> <!> <p class="small"> </p>`,1),ka=q(`<p> </p> <div class="levels" role="radiogroup" aria-label="How far agents may go"></div> <p class="small"> </p> <p> </p> <!> <div class="row"><label for="port">Port:</label> <input id="port" class="port" type="text" inputmode="numeric" maxlength="5"/> <button>Set Port</button></div> <!> <!> <!> <!> <!>`,1),Aa=q(`<button>Open Folder</button>`),ja=q(`<button title="Creates the Svelte Showcase module. It does nothing until you trust it.">Create Showcase</button>`),Ma=q(`<button title="Shows its panel inside GridCraft's window, beside the sheet">Dock Showcase</button>`),Na=q(`<button>Open in Browser</button>`),Pa=q(`<!> <!>`,1),Fa=q(`<div class="row"><button title="Create, review, trust and run modules; name the Python that runs their scripts">Modules…</button> <span> </span> <span class="gap"></span> <button title="What your modules print and log, what failed, and what agents asked for">Console…</button> <span> </span></div> <div class="row"><span class="small"> </span> <span class="small">Modules folder:</span> <code class="path"> </code> <!></div> <div class="row"><span class="strong">Showcase</span> <span> </span> <!></div>`,1),Ia=q(`<p class="intro"> </p> <section><h1>Agents</h1> <!></section> <section><h1>Tools</h1> <!></section> <footer><button class="link" title="Shows this in GridCraft's own window instead, until GridCraft closes">Use GridCraft's built-in window</button></footer>`,1);function La(e,t){Ue(t,!0);let n=dt(()=>t.data.agents),r=dt(()=>t.data.tools),i=pn(``),a=pn(!1);Vn(()=>{!G(a)&&G(n)&&F(i,G(n).portText,!0)});function o(e,t){e.preventDefault(),t!==G(n).level&&$(`level`,{level:t})}function s(){$(`port`,{port:G(i)}),F(a,!1)}var c=Ia(),l=L(c),u=R(l,!0),d=z(l,2),f=z(I(d),2),p=e=>{var t=ga(),r=R(t,!0);B(()=>Z(r,G(n).unavailable)),J(e,t)},m=e=>{var t=ka(),r=L(t),c=R(r,!0),l=z(r,2);Ri(l,21,()=>G(n).levels,Pi,(e,t)=>{var r=_a(),i=I(r);Xi(i);var a=R(z(i,2),!0);E(r),B(()=>{Qi(r,`title`,G(t).meaning),Zi(i,G(t).code===G(n).level),Z(a,G(t).label)}),K(`click`,i,e=>o(e,G(t).code)),J(e,r)}),E(l);var u=z(l,2),d=R(u,!0),f=z(u,2),p=R(f,!0),m=z(f,2),h=e=>{var t=va(),r=R(t,!0);B(()=>Z(r,G(n).clients)),J(e,t)};Q(m,e=>{G(n).clients&&e(h)});var g=z(m,2),_=z(I(g),2);Xi(_);var v=z(_,2);E(g);var y=z(g,2),b=e=>{var t=ya(),n=I(t),r=z(n,2),i=z(r,2);E(t),K(`click`,n,()=>$(`copySetup`,{json:!1})),K(`click`,r,()=>$(`copySetup`,{json:!0})),K(`click`,i,()=>$(`newToken`)),J(e,t)};Q(y,e=>{G(n).serving&&e(b)});var x=z(y,2),S=e=>{var t=xa(),r=L(t),i=R(r,!0);Ri(z(r,2),17,()=>G(n).waiting,Pi,(e,t)=>{var n=ba(),r=I(n),i=R(r,!0),a=z(r,2),o=R(a,!0),s=z(a,4),c=R(s,!0),l=z(s,2);E(n),B(()=>{Z(i,G(t).kind),Z(o,G(t).target),Z(c,G(t).named)}),K(`click`,l,()=>$(`look`,{id:G(t).id})),J(e,n)}),B(()=>Z(i,G(n).waitingTitle)),J(e,t)};Q(x,e=>{G(n).waiting.length&&e(S)});var ee=z(x,2),te=e=>{var t=Ta();Ri(z(L(t),2),17,()=>G(n).plans,Pi,(e,t)=>{var n=wa(),r=I(n),i=I(r),a=R(i,!0),o=z(i,2),s=R(o,!0),c=z(o,4),l=R(c,!0),u=z(c,2),d=e=>{var n=Sa();K(`click`,n,()=>$(`stopPlan`,{id:G(t).id})),J(e,n)};Q(u,e=>{G(t).live&&e(d)}),E(r);var f=z(r,2);Ri(f,21,()=>G(t).steps,Pi,(e,t)=>{var n=Ca(),r=I(n),i=R(r,!0),a=R(z(r),!0);E(n),B(()=>{Ki(n,1,`small ${G(t).state??``}`),Z(i,G(t).what),Z(a,G(t).stands)}),J(e,n)}),E(f),E(n),B(()=>{Z(a,G(t).name),Z(s,G(t).title),Z(l,G(t).stands)}),J(e,n)}),J(e,t)};Q(ee,e=>{G(n).plans.length&&e(te)});var ne=z(ee,2),re=e=>{var t=Da();Ri(z(L(t),2),17,()=>G(n).unasked,Pi,(e,t)=>{var n=Ea(),r=I(n),i=R(r,!0),a=z(r,2);E(n),B(()=>Z(i,G(t).covers)),K(`click`,a,()=>$(`askAgain`,{module:G(t).module,kind:G(t).kind})),J(e,n)}),J(e,t)};Q(ne,e=>{G(n).unasked.length&&e(re)});var ie=z(ne,2),ae=e=>{var t=Oa(),r=z(L(t),2);Ri(r,17,()=>G(n).lately,Pi,(e,t)=>{var n=va(),r=R(n,!0);B(()=>Z(r,G(t))),J(e,n)});var i=R(z(r,2),!0);B(()=>Z(i,G(n).latelyNote)),J(e,t)};Q(ie,e=>{G(n).lately.length&&e(ae)}),B(()=>{Z(c,G(n).intro),Z(d,G(n).meaning),Ki(f,1,`status ${G(n).status.kind??``}`),Z(p,G(n).status.text)}),K(`input`,_,()=>F(a,!0)),na(_,()=>G(i),e=>F(i,e)),K(`click`,v,s),J(e,t)};Q(f,e=>{G(n).available?e(m,-1):e(p)}),E(d);var h=z(d,2),g=z(I(h),2),_=e=>{var t=ga(),n=R(t,!0);B(()=>Z(n,G(r).unavailable)),J(e,t)},v=e=>{var t=Fa(),n=L(t),i=I(n),a=z(i,2),o=R(a,!0),s=z(a,4),c=R(z(s,2),!0);E(n);var l=z(n,2),u=I(l),d=R(u,!0),f=z(u,4),p=R(f,!0),m=z(f,2),h=e=>{var t=Aa();K(`click`,t,()=>$(`openFolder`)),J(e,t)};Q(m,e=>{G(r).canOpenFolder&&e(h)}),E(l);var g=z(l,2),_=z(I(g),2),v=R(_,!0),y=z(_,2),b=e=>{var t=ja();K(`click`,t,()=>$(`createShowcase`)),J(e,t)},x=e=>{var t=Pa(),n=L(t),i=e=>{var t=Ma();K(`click`,t,()=>$(`showShowcase`,{docked:!0})),J(e,t)};Q(n,e=>{G(r).showcase.canDock&&e(i)});var a=z(n,2),o=e=>{var t=Na();K(`click`,t,()=>$(`showShowcase`,{docked:!1})),J(e,t)};Q(a,e=>{G(r).showcase.canBrowse&&e(o)}),J(e,t)};Q(y,e=>{G(r).showcase.stands===`none`?e(b):G(r).showcase.stands===`trusted`&&e(x,1)}),E(g),B(()=>{Z(o,G(r).modules),Z(c,G(r).console),Z(d,G(r).python),Qi(f,`title`,G(r).folder),Z(p,G(r).folder),Z(v,G(r).showcase.text)}),K(`click`,i,()=>$(`open`,{name:`modules`})),K(`click`,s,()=>$(`open`,{name:`console`})),J(e,t)};Q(g,e=>{G(r).available?e(v,-1):e(_)}),E(h);var y=R(z(h,2));B(()=>Z(u,t.data.intro)),K(`click`,y,()=>$(`builtin`)),J(e,c),We()}Ir([`click`,`input`]);var Ra=q(`<p> </p>`),za=q(`<p class="warn"> </p>`),Ba=q(`<p class="small"> </p>`),Va=q(`<p class="strong"> </p>`),Ha=q(`<button class="link"> </button>`),Ua=q(`<p class="small toggle"><span> </span> <!></p>`),Wa=q(`<span class="no"></span>`),Ga=q(`<div class="line"><!><span class="src"> </span></div>`),Ka=q(`<div class="code" role="group" aria-label="The text"></div> <!>`,1),qa=q(`<div class="line skip"><span class="no"></span><span class="mark"></span><span class="src"> </span></div>`),Ja=q(`<div><span class="no"> </span><span class="mark"> </span><span class="src"> </span></div>`),Ya=q(`<div class="code" role="group" aria-label="The changes"></div> <!>`,1),Xa=q(`<section class="step"><h2><span> </span> <code> </code></h2> <!></section>`);function Za(e,t){Ue(t,!0);let n={gone:`-`,new:`+`,same:` `};var r=Wr();Ri(L(r),17,()=>t.blocks,Pi,(e,t)=>{var r=Wr(),i=L(r),a=e=>{var n=Ra(),r=R(n,!0);B(()=>Z(r,G(t).text)),J(e,n)},o=e=>{var n=za(),r=R(n,!0);B(()=>Z(r,G(t).text)),J(e,n)},s=e=>{var n=Ba(),r=R(n,!0);B(()=>Z(r,G(t).text)),J(e,n)},c=e=>{var n=Va(),r=R(n,!0);B(()=>Z(r,G(t).text)),J(e,n)},l=e=>{var n=Ra();let r;var i=R(n,!0);B(()=>{r=Ki(n,1,`bullet`,null,r,{warn:G(t).warn}),Z(i,G(t).text)}),J(e,n)},u=e=>{var n=Ua(),r=I(n),i=R(r,!0),a=z(r,2),o=e=>{var n=Ha(),r=R(n,!0);B(()=>Z(r,G(t).button)),K(`click`,n,()=>$(`whole`,{whole:G(t).whole})),J(e,n)};Q(a,e=>{G(t).button&&e(o)}),E(n),B(()=>Z(i,G(t).text)),J(e,n)},d=e=>{var n=Ka(),r=L(n);Ri(r,21,()=>G(t).lines,Pi,(e,n,r)=>{var i=Ga(),a=I(i),o=e=>{var t=Wa();t.textContent=r+1,J(e,t)};Q(a,e=>{G(t).numbered&&e(o)});var s=R(z(a),!0);E(i),B(()=>Z(s,G(n))),J(e,i)}),E(r);var i=z(r,2),a=e=>{var n=za(),r=R(n,!0);B(()=>Z(r,G(t).cut)),J(e,n)};Q(i,e=>{G(t).cut&&e(a)}),J(e,n)},f=e=>{var r=Ya(),i=L(r);Ri(i,21,()=>G(t).rows,Pi,(e,t)=>{var r=Wr(),i=L(r),a=e=>{var n=qa(),r=R(z(I(n),2));E(n),B(()=>Z(r,`⋯ ${G(t).text??``}`)),J(e,n)},o=e=>{var r=Ja(),i=I(r),a=R(i,!0),o=z(i),s=R(o,!0),c=R(z(o),!0);E(r),B(()=>{Ki(r,1,`line ${G(t).k??``}`),Z(a,G(t).n),Z(s,n[G(t).k]??` `),Z(c,G(t).text)}),J(e,r)};Q(i,e=>{G(t).k===`skip`?e(a):e(o,-1)}),J(e,r)}),E(i);var a=z(i,2),o=e=>{var n=za(),r=R(n,!0);B(()=>Z(r,G(t).cut)),J(e,n)};Q(a,e=>{G(t).cut&&e(o)}),J(e,r)},p=e=>{var n=Xa(),r=I(n),i=I(r),a=R(i,!0),o=R(z(i,2),!0);E(r),Za(z(r,2),{get blocks(){return G(t).blocks}}),E(n),B(()=>{Z(a,G(t).heading),Z(o,G(t).target)}),J(e,n)};Q(i,e=>{G(t).t===`note`?e(a):G(t).t===`warn`?e(o,1):G(t).t===`small`?e(s,2):G(t).t===`strong`?e(c,3):G(t).t===`bullet`?e(l,4):G(t).t===`toggle`?e(u,5):G(t).t===`text`?e(d,6):G(t).t===`diff`?e(f,7):G(t).t===`step`&&e(p,8)}),J(e,r)}),J(e,r),We()}Ir([`click`]);var Qa=q(`<p class="warn"> </p>`),$a=q(`<button class="choice"> </button>`),eo=q(`<section class="card"><header><h1> </h1> <span class="named"> </span></header> <div><p class="target"> </p> <!></div> <div class="means"><!></div> <p class="question"> </p> <!> <div class="choices"><button class="choice"> </button> <!> <div class="refuse"><button class="choice"> </button> <input type="text" maxlength="400"/></div> <button class="choice quiet"> </button></div> <p class="small"> </p></section>`);function to(e,t){Ue(t,!0);let n=pn(``),r=pn(!1);Vn(()=>{let e=t.card.readyIn;if(e<=0){F(r,!0);return}let n=setTimeout(()=>F(r,!0),e);return()=>clearTimeout(n)});let i=dt(()=>G(r)&&!t.card.blocked);var a=eo(),o=I(a),s=I(o),c=R(s,!0),l=R(z(s,2),!0);E(o);var u=z(o,2);let d;var f=I(u),p=R(f,!0);Za(z(f,2),{get blocks(){return t.card.content}}),E(u);var m=z(u,2);Za(I(m),{get blocks(){return t.card.means}}),E(m);var h=z(m,2),g=R(h,!0),_=z(h,2),v=e=>{var n=Qa(),r=R(n,!0);B(()=>Z(r,t.card.blockedWhy)),J(e,n)};Q(_,e=>{t.card.blockedWhy&&e(v)});var y=z(_,2),b=I(y),x=R(b,!0),S=z(b,2),ee=e=>{var n=$a(),r=R(n,!0);B(()=>{n.disabled=!G(i),Z(r,t.card.yesAnd)}),K(`click`,n,()=>$(`allowNotAgain`,{id:t.card.id})),J(e,n)};Q(S,e=>{t.card.yesAnd&&e(ee)});var te=z(S,2),ne=I(te),re=R(ne,!0),ie=z(ne,2);Xi(ie),E(te);var ae=z(te,2),oe=R(ae,!0);E(y);var se=R(z(y,2),!0);E(a),B(()=>{Qi(a,`aria-label`,t.card.kind),Z(c,t.card.kind),Z(l,t.card.named),d=Ki(u,1,`block`,null,d,{long:t.card.long}),Z(p,t.card.target),Z(g,t.card.question),b.disabled=!G(i),Z(x,t.card.yes),Z(re,t.card.no),Qi(ie,`placeholder`,t.card.reasonHint),Qi(ie,`aria-label`,t.card.reasonHint),Z(oe,t.card.notNow),Z(se,t.card.asker)}),K(`click`,b,()=>$(`allow`,{id:t.card.id})),K(`click`,ne,()=>$(`refuse`,{id:t.card.id,reason:G(n)})),na(ie,()=>G(n),e=>F(n,e)),K(`click`,ae,()=>$(`notNow`)),J(e,a),We()}Ir([`click`]);var no=q(`<main class="waiting"><p>Waiting for GridCraft…</p></main>`),ro=q(`<p class="error" role="alert"> </p>`),io=q(`<p class="said" role="status"> </p>`),ao=q(`<main><!> <!> <!></main>`);function oo(e,t){Ue(t,!0);let n=dt(()=>ma.data);var r=Wr(),i=L(r),a=e=>{J(e,no())},o=e=>{var t=ao(),r=I(t),i=e=>{var t=Wr();Ni(L(t),()=>G(n).looking.id,e=>{to(e,{get card(){return G(n).looking}})}),J(e,t)},a=e=>{La(e,{get data(){return G(n)}})};Q(r,e=>{G(n).looking?e(i):e(a,-1)});var o=z(r,2),s=e=>{var t=ro(),r=R(t,!0);B(()=>Z(r,G(n).error)),J(e,t)};Q(o,e=>{G(n).error&&e(s)});var c=z(o,2),l=e=>{var t=io(),r=R(t,!0);B(()=>Z(r,G(n).said)),J(e,t)};Q(c,e=>{G(n).said&&e(l)}),E(t),J(e,t)};Q(i,e=>{G(n)?e(o,-1):e(a)}),J(e,r),We()}var so=[`window`,`text`,`dim`,`accent`,`onAccent`,`danger`,`warn`,`border`,`input`,`block`,`hover`,`added`,`removed`];window.gridcraftState=e=>{let t=e&&e.theme;if(t){let e=document.documentElement;for(let n of so)typeof t[n]==`string`&&e.style.setProperty(`--${n}`,t[n]);e.style.setProperty(`color-scheme`,t.dark?`dark`:`light`)}ha(e)},bi(oo,{target:document.getElementById(`app`)}),$(`ready`)})();"#),
    ("app.css", r#":root{--window:#fff;--text:#1f1f1f;--dim:#666;--accent:#0f7b46;--onAccent:#fff;--danger:#c0392b;--warn:#b85c00;--border:#d0d0d0;--input:#fff;--block:#f4f4f4;--hover:#ececec;--added:#0f7b462e;--removed:#c0392b2e;font:14px/1.45 Segoe UI,system-ui,-apple-system,BlinkMacSystemFont,Roboto,Noto Sans,Ubuntu,Cantarell,Helvetica Neue,sans-serif}*{box-sizing:border-box}html,body{background:var(--window);height:100%;color:var(--text);margin:0}main{max-width:920px;margin:0 auto;padding:14px 20px 20px}main.waiting{color:var(--dim);text-align:center;padding-top:40px}h1{border-bottom:1px solid var(--border);margin:16px 0 6px;padding-bottom:5px;font-size:15px;font-weight:600}h2{margin:14px 0 4px;font-size:13.5px;font-weight:600}p{margin:4px 0}.intro{color:var(--dim);margin:0}.small{color:var(--dim);font-size:12.5px}.strong{font-weight:600}.warn,.bullet.warn{color:var(--warn);font-weight:600}.bullet{text-indent:-10px;padding-left:14px}.bullet:before{content:"•  "}code,.code,.target{font-family:Cascadia Mono,Consolas,SF Mono,Menlo,monospace;font-size:12.5px}button{font:inherit;color:var(--text);background:var(--input);border:1px solid var(--border);cursor:pointer;border-radius:5px;padding:4px 10px}button:hover:not(:disabled){background:var(--hover)}button:focus-visible{outline:2px solid var(--accent);outline-offset:1px}input:focus-visible{outline:2px solid var(--accent);outline-offset:1px}button:disabled{color:var(--dim);cursor:default;opacity:.65}button.link{color:var(--accent);background:0 0;border:none;padding:0 4px;text-decoration:underline}input[type=text]{font:inherit;color:var(--text);background:var(--input);border:1px solid var(--border);border-radius:5px;padding:4px 8px}.row{flex-wrap:wrap;align-items:center;gap:8px;margin:6px 0;display:flex}.gap{width:10px}.grow{flex:1}.port{width:76px}.path{text-overflow:ellipsis;white-space:nowrap;max-width:320px;color:var(--dim);overflow:hidden}.levels{flex-direction:column;gap:3px;margin:6px 0;display:flex}.level{cursor:pointer;align-items:center;gap:8px;width:-moz-fit-content;width:fit-content;display:flex}.level input{accent-color:var(--accent);margin:0}.status{margin-top:8px;font-weight:600}.status.ok{color:var(--accent)}.status.error{color:var(--danger)}.status.off{color:var(--dim);font-weight:400}.item{border:1px solid var(--border);border-radius:6px;flex-wrap:wrap;align-items:center;gap:8px;margin:5px 0;padding:6px 10px;display:flex}.item.plan{display:block}.item .head{flex-wrap:wrap;align-items:center;gap:8px;display:flex}.item ol{margin:6px 0 2px;padding-left:22px}.item li .state{margin-left:8px}.item li .state:before{content:"·  "}.item li.done{color:var(--text)}.item li.failed .state{color:var(--danger);font-weight:600}.item li.running .state{color:var(--accent);font-weight:600}.card header{align-items:baseline;gap:12px;margin-bottom:8px;display:flex}.card header h1{border:none;flex:1;margin:0;padding:0;font-size:17px}.named{color:var(--dim);font-size:12.5px}.block{background:var(--block);border:1px solid var(--border);border-radius:6px;margin-bottom:8px;padding:8px 12px}.block.long{min-height:160px;max-height:calc(100vh - 330px);overflow-y:auto}.block.long .code{max-height:none}.target{overflow-wrap:anywhere;margin-bottom:6px;font-weight:600}.code{background:var(--window);border:1px solid var(--border);border-radius:4px;max-height:300px;margin:6px 0;padding:6px 0;line-height:1.4;overflow:auto}.line{white-space:pre;width:max-content;min-width:100%;display:flex}.line .no{text-align:right;width:46px;color:var(--dim);-webkit-user-select:none;user-select:none;flex:none;padding-right:8px}.line .mark{-webkit-user-select:none;user-select:none;flex:none;width:18px}.line .src{padding-right:12px}.line.gone{background:var(--removed)}.line.gone .mark{color:var(--danger)}.line.new{background:var(--added)}.line.new .mark{color:var(--accent)}.line.skip{color:var(--dim)}.step{border-top:1px solid var(--border);margin-top:10px;padding-top:8px}.step h2{margin:0 0 4px;font-size:13.5px}.step h2 code{overflow-wrap:anywhere;margin-left:6px;font-weight:400}.toggle{align-items:center;gap:8px;display:flex}.question{margin:12px 0 6px;font-weight:600}.choices{flex-direction:column;align-items:flex-start;gap:6px;margin-bottom:8px;display:flex}.choice{text-align:left;max-width:100%;padding:5px 12px}.choice.quiet{color:var(--dim)}.refuse{gap:8px;width:100%;display:flex}.refuse input{flex:1;min-width:0}.said,.error{border:1px solid var(--border);border-radius:5px;margin-top:12px;padding:6px 10px;font-size:13px}.error{border-color:var(--danger);color:var(--danger);font-weight:600}footer{text-align:right;margin-top:18px;font-size:12.5px}
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
answering one, is GridCraft's (`crates/ui-egui/src/aniefork/card.rs`, `devpage.rs`).

- `index.html` is the page. It loads `app.css` and `app.js` and nothing else.
- `src/` is the app. `main.js` takes what GridCraft says (`window.gridcraftState`), `host.js`
  sends what was clicked, `App.svelte` chooses between the whole window (`Home.svelte`) and
  one request's card (`Review.svelte`), and `Blocks.svelte` draws what a card holds.

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

main {
  max-width: 920px;
  margin: 0 auto;
  padding: 14px 20px 20px;
}

main.waiting {
  color: var(--dim);
  padding-top: 40px;
  text-align: center;
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

.intro {
  color: var(--dim);
  margin: 0;
}

.small {
  font-size: 12.5px;
  color: var(--dim);
}

.strong {
  font-weight: 600;
}

.warn,
.bullet.warn {
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

.gap {
  width: 10px;
}

.grow {
  flex: 1;
}

.port {
  width: 76px;
}

.path {
  max-width: 320px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--dim);
}

/* The levels. */
.levels {
  display: flex;
  flex-direction: column;
  gap: 3px;
  margin: 6px 0;
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
  margin-top: 8px;
}

.status.ok {
  color: var(--accent);
}

.status.error {
  color: var(--danger);
}

.status.off {
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
  border-radius: 6px;
  padding: 6px 10px;
  margin: 5px 0;
}

.item.plan {
  display: block;
}

.item .head {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.item ol {
  margin: 6px 0 2px;
  padding-left: 22px;
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
  margin-top: 12px;
  padding: 6px 10px;
  border-radius: 5px;
  border: 1px solid var(--border);
  font-size: 13px;
}

.error {
  border-color: var(--danger);
  color: var(--danger);
  font-weight: 600;
}

footer {
  margin-top: 18px;
  text-align: right;
  font-size: 12.5px;
}
"#),
    ("src/App.svelte", r#"<script>
  // The Development window, as a page. GridCraft says what it shows (view.data) and the page
  // draws it: every sentence about a request comes from GridCraft, the same words its own
  // window uses. What the person clicks goes back as a message (host.js).
  import { view } from "./view.svelte.js";
  import Home from "./Home.svelte";
  import Review from "./Review.svelte";

  let data = $derived(view.data);
</script>

{#if !data}
  <main class="waiting"><p>Waiting for GridCraft…</p></main>
{:else}
  <main>
    {#if data.looking}
      <!-- A request being looked at takes the place of the rest. A new card is a new component:
           what was typed for the one before is gone with it. -->
      {#key data.looking.id}
        <Review card={data.looking} />
      {/key}
    {:else}
      <Home {data} />
    {/if}
    {#if data.error}<p class="error" role="alert">{data.error}</p>{/if}
    {#if data.said}<p class="said" role="status">{data.said}</p>{/if}
  </main>
{/if}
"#),
    ("src/Home.svelte", r#"<script>
  // The window when no request is being looked at: how far agents may go, how to connect,
  // what waits for an answer, the plan that was approved, and the development tools.
  import { send } from "./host.js";

  let { data } = $props();

  let agents = $derived(data.agents);
  let tools = $derived(data.tools);

  // The port is the person's to type: the page keeps what they typed until they set it.
  let port = $state("");
  let typed = $state(false);
  $effect(() => {
    if (!typed && agents) port = agents.portText;
  });

  // The level shown is the one GridCraft says is set: a click asks for another, and the
  // mark moves only when GridCraft has set it (raising it asks the operating system first).
  function choose(event, code) {
    event.preventDefault();
    if (code !== agents.level) send("level", { level: code });
  }

  function setPort() {
    send("port", { port });
    typed = false;
  }
</script>

<p class="intro">{data.intro}</p>

<section>
  <h1>Agents</h1>
  {#if !agents.available}
    <p>{agents.unavailable}</p>
  {:else}
    <p>{agents.intro}</p>
    <div class="levels" role="radiogroup" aria-label="How far agents may go">
      {#each agents.levels as level}
        <label class="level" title={level.meaning}>
          <input type="radio" name="level" checked={level.code === agents.level} onclick={(event) => choose(event, level.code)} />
          <span>{level.label}</span>
        </label>
      {/each}
    </div>
    <p class="small">{agents.meaning}</p>

    <p class="status {agents.status.kind}">{agents.status.text}</p>
    {#if agents.clients}<p class="small">{agents.clients}</p>{/if}

    <div class="row">
      <label for="port">Port:</label>
      <input id="port" class="port" type="text" inputmode="numeric" maxlength="5" bind:value={port} oninput={() => (typed = true)} />
      <button onclick={setPort}>Set Port</button>
    </div>
    {#if agents.serving}
      <div class="row">
        <button onclick={() => send("copySetup", { json: false })} title="Copies the command that adds this server to Claude Code. It holds the token.">Copy Setup for Claude Code</button>
        <button onclick={() => send("copySetup", { json: true })} title="Copies an MCP settings entry, for other agents. It holds the token.">Copy Setup as JSON</button>
        <button onclick={() => send("newToken")} title="Makes a new token: an agent that holds the old one can no longer connect">New Token</button>
      </div>
    {/if}

    {#if agents.waiting.length}
      <h2>{agents.waitingTitle}</h2>
      {#each agents.waiting as asked}
        <div class="item">
          <span class="strong">{asked.kind}</span>
          <code>{asked.target}</code>
          <span class="grow"></span>
          <span class="small">{asked.named}</span>
          <button onclick={() => send("look", { id: asked.id })} title="See everything this request would do, and answer it">Look…</button>
        </div>
      {/each}
    {/if}

    {#if agents.plans.length}
      <h2>Plans</h2>
      {#each agents.plans as plan}
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
              <li class="small {step.state}"><span>{step.what}</span><span class="state">{step.stands}</span></li>
            {/each}
          </ol>
        </div>
      {/each}
    {/if}

    {#if agents.unasked.length}
      <h2>Not asked again until GridCraft closes</h2>
      {#each agents.unasked as rule}
        <div class="row">
          <span>{rule.covers}</span>
          <button class="link" onclick={() => send("askAgain", { module: rule.module, kind: rule.kind })}>Ask again</button>
        </div>
      {/each}
    {/if}

    {#if agents.lately.length}
      <h2>Lately</h2>
      {#each agents.lately as line}
        <p class="small">{line}</p>
      {/each}
      <p class="small">{agents.latelyNote}</p>
    {/if}
  {/if}
</section>

<section>
  <h1>Tools</h1>
  {#if !tools.available}
    <p>{tools.unavailable}</p>
  {:else}
    <div class="row">
      <button onclick={() => send("open", { name: "modules" })} title="Create, review, trust and run modules; name the Python that runs their scripts">Modules…</button>
      <span>{tools.modules}</span>
      <span class="gap"></span>
      <button onclick={() => send("open", { name: "console" })} title="What your modules print and log, what failed, and what agents asked for">Console…</button>
      <span>{tools.console}</span>
    </div>
    <div class="row">
      <span class="small">{tools.python}</span>
      <span class="small">Modules folder:</span>
      <code class="path" title={tools.folder}>{tools.folder}</code>
      {#if tools.canOpenFolder}<button onclick={() => send("openFolder")}>Open Folder</button>{/if}
    </div>
    <div class="row">
      <span class="strong">Showcase</span>
      <span>{tools.showcase.text}</span>
      {#if tools.showcase.stands === "none"}
        <button onclick={() => send("createShowcase")} title="Creates the Svelte Showcase module. It does nothing until you trust it.">Create Showcase</button>
      {:else if tools.showcase.stands === "trusted"}
        {#if tools.showcase.canDock}
          <button onclick={() => send("showShowcase", { docked: true })} title="Shows its panel inside GridCraft's window, beside the sheet">Dock Showcase</button>
        {/if}
        {#if tools.showcase.canBrowse}
          <button onclick={() => send("showShowcase", { docked: false })}>Open in Browser</button>
        {/if}
      {/if}
    </div>
  {/if}
</section>

<footer>
  <button class="link" onclick={() => send("builtin")} title="Shows this in GridCraft's own window instead, until GridCraft closes">Use GridCraft's built-in window</button>
</footer>
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
