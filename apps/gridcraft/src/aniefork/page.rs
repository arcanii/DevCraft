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
    ("app.js", r#"(function(){var e=Array.isArray,t=Array.prototype.indexOf,n=Array.prototype.includes,r=Array.from,i=Object.defineProperty,a=Object.getOwnPropertyDescriptor,o=Object.getOwnPropertyDescriptors,s=Object.prototype,c=Array.prototype,l=Object.getPrototypeOf,u=Object.isExtensible,d=()=>{};function f(e){for(var t=0;t<e.length;t++)e[t]()}function p(){var e,t;return{promise:new Promise((n,r)=>{e=n,t=r}),resolve:e,reject:t}}function m(e){"@babel/helpers - typeof";return m=typeof Symbol==`function`&&typeof Symbol.iterator==`symbol`?function(e){return typeof e}:function(e){return e&&typeof Symbol==`function`&&e.constructor===Symbol&&e!==Symbol.prototype?`symbol`:typeof e},m(e)}function h(e,t){if(m(e)!=`object`||!e)return e;var n=e[Symbol.toPrimitive];if(n!==void 0){var r=n.call(e,t||`default`);if(m(r)!=`object`)return r;throw TypeError(`@@toPrimitive must return a primitive value.`)}return(t===`string`?String:Number)(e)}function g(e){var t=h(e,`string`);return m(t)==`symbol`?t:t+``}function _(e,t,n){return(t=g(t))in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}var v=1024,y=2048,b=4096,x=8192,S=16384,ee=32768,C=1<<25,te=65536,ne=1<<19,re=1<<20,ie=1<<25,ae=1<<21,oe=1<<22,se=1<<23,ce=Symbol(`$state`),le=Symbol(`component`),ue=Symbol(`legacy props`),de=Symbol(``),fe=Symbol(`attributes`),pe=Symbol(`class`),me=Symbol(`style`),he=Symbol(`text`),ge=Symbol(`form reset`),_e=new class extends Error{constructor(...e){super(...e),_(this,`name`,`StaleReactionError`),_(this,`message`,"The reaction that called `getAbortSignal()` was re-run or destroyed")}},ve=!!globalThis.document?.contentType&&globalThis.document.contentType.includes(`xml`),ye={},w=Symbol(`uninitialized`);function be(){console.warn(`https://svelte.dev/e/derived_inert`)}function xe(e){console.warn(`https://svelte.dev/e/hydration_mismatch`)}function Se(){console.warn(`https://svelte.dev/e/select_multiple_invalid_value`)}function Ce(){console.warn(`https://svelte.dev/e/svelte_boundary_reset_noop`)}var T=!1;function we(e){T=e}var E;function Te(e){if(e===null)throw xe(),ye;return E=e}function Ee(){return Te(Fn(E))}function D(e){if(T){if(Fn(E)!==null)throw xe(),ye;E=e}}function De(e=1){if(T){for(var t=e,n=E;t--;)n=Fn(n);E=n}}function Oe(e=!0){for(var t=0,n=E;;){if(n.nodeType===8){var r=n.data;if(r===`]`){if(t===0)return n;--t}else(r===`[`||r===`[!`||r[0]===`[`&&!isNaN(Number(r.slice(1))))&&(t+=1)}var i=Fn(n);e&&n.remove(),n=i}}function ke(e){if(!e||e.nodeType!==8)throw xe(),ye;return e.data}function Ae(e){return e===this.v}function je(e,t){return e==e?e!==t||typeof e==`object`&&!!e||typeof e==`function`:t==t}function Me(e){return!je(e,this.v)}function Ne(){throw Error(`https://svelte.dev/e/async_derived_orphan`)}function Pe(e,t,n){throw Error(`https://svelte.dev/e/each_key_duplicate`)}function Fe(e){throw Error(`https://svelte.dev/e/effect_in_teardown`)}function Ie(){throw Error(`https://svelte.dev/e/effect_in_unowned_derived`)}function Le(e){throw Error(`https://svelte.dev/e/effect_orphan`)}function Re(){throw Error(`https://svelte.dev/e/effect_update_depth_exceeded`)}function ze(e){throw Error(`https://svelte.dev/e/props_invalid_value`)}function Be(){throw Error(`https://svelte.dev/e/state_descriptors_fixed`)}function Ve(){throw Error(`https://svelte.dev/e/state_prototype_fixed`)}function He(){throw Error(`https://svelte.dev/e/state_unsafe_mutation`)}function Ue(){throw Error(`https://svelte.dev/e/svelte_boundary_reset_onerror`)}var We=null;function Ge(e){We=e}function Ke(e,t=!1,n){We={p:We,i:!1,c:null,e:null,s:e,x:null,r:V,l:null}}function qe(e){var t=We,n=t.e;if(n!==null){t.e=null;for(var r of n)Jn(r)}return e!==void 0&&(t.x=e),t.i=!0,We=t.p,Je(e)}function Je(e={}){return i(e,le,{value:!0}),e}function Ye(){return!0}var Xe=[];function Ze(){var e=Xe;Xe=[],f(e)}function Qe(e){if(Xe.length===0&&!Ot){var t=Xe;queueMicrotask(()=>{t===Xe&&Ze()})}Xe.push(e)}function $e(){for(;Xe.length>0;)Ze()}var et=~(y|b|v);function O(e,t){e.f=e.f&et|t}function tt(e){e.f&512||e.deps===null?O(e,v):O(e,b)}function nt(e,t,n){e.f&2048?t.add(e):e.f&4096&&n.add(e),O(e,v)}var rt=!1;function it(){rt||(rt=!0,document.addEventListener(`reset`,e=>{Promise.resolve().then(()=>{if(!e.defaultPrevented)for(let t of e.target.elements)t[ge]?.()})},{capture:!0}))}function at(e){var t=B,n=V;gr(null),_r(null);try{return e()}finally{gr(t),_r(n)}}function ot(e,t,n,r=n){e.addEventListener(t,()=>at(n));let i=e[ge];e[ge]=i?()=>{i(),r(!0)}:()=>r(!0),it()}function st(e,t,n,r){let i=Ye()?dt:ht;var a=e.filter(e=>!e.settled),o=t.map(i);if(n.length===0&&a.length===0){r(o);return}var s=V,c=ct(),l=a.length===1?a[0].promise:a.length>1?Promise.all(a.map(e=>e.promise)):null;function u(e){if(!(s.f&16384)){c();try{r([...o,...e])}catch(e){Vn(e,s)}lt()}}var d=ut();if(n.length===0){l.then(()=>u([])).finally(d);return}function f(){Promise.all(n.map(e=>pt(e))).then(u).catch(e=>Vn(e,s)).finally(d)}l?l.then(()=>{s.f&16384?d():(c(),f(),lt())}):f()}function ct(){var e=V,t=B,n=We,r=N;return function(i=!0){_r(e),gr(t),Ge(n),i&&!(e.f&16384)&&(r?.activate(),r?.apply())}}function lt(e=!0){_r(null),gr(null),Ge(null),e&&N?.deactivate()}function ut(){var e=V,t=e.b,n=N,r=!!t?.is_rendered();return t?.update_pending_count(1,n),n.increment(r,e),()=>{t?.update_pending_count(-1,n),n.decrement(r,e)}}function dt(e){var t=2|y;return V!==null&&(V.f|=ne),{ctx:We,deps:null,effects:null,equals:Ae,f:t,fn:e,reactions:null,rv:0,v:w,wv:0,parent:V,ac:null}}var ft=Symbol(`obsolete`);function pt(e,t,n){let r=V;r===null&&Ne();var i=void 0,a=gn(w),o=!B,s=new Set;return Zn(()=>{var t=V,n=p();i=n.promise;try{Promise.resolve(e()).then(n.resolve,e=>{e!==_e&&n.reject(e)}).finally(lt)}catch(e){n.reject(e),lt()}var c=N;if(o){if(t.f&32768)var l=ut();if(r.b?.is_rendered())c.async_deriveds.get(t)?.reject(ft);else for(let e of s.values())e.reject(ft);s.add(n),c.async_deriveds.set(t,n)}let u=(e,t=void 0)=>{l?.(),s.delete(n),t!==ft&&(c.activate(),t?(a.f|=se,xn(a,t)):(a.f&8388608&&(a.f^=se),xn(a,e)),c.deactivate())};n.promise.then(u,e=>u(null,e||`unknown`))}),Kn(()=>{for(let e of s)e.reject(ft)}),new Promise(e=>{function t(n){function r(){n===i?e(a):t(i)}n.then(r,r)}t(i)})}function mt(e){let t=dt(e);return yr(t),t}function ht(e){let t=dt(e);return t.equals=Me,t}function gt(e){var t=e.effects;if(t!==null){e.effects=null;for(var n=0;n<t.length;n+=1)ir(t[n])}}function _t(e){var t,n=V,r=e.parent;if(!pr&&r!==null&&e.v!==w&&r.f&24576)return be(),e.v;_r(r);try{gt(e),t=jr(e)}finally{_r(n)}return t}function vt(e){var t=_t(e);!e.equals(t)&&(e.wv=Or(),(!N?.is_fork||e.deps===null)&&(N===null?e.v=t:(N.capture(e,t,!0),Tt?.capture(e,t,!0)),e.deps===null))?O(e,v):pr||(Et===null?tt(e):(Gn()||N?.is_fork)&&Et.set(e,t))}function yt(e){if(e.effects!==null)for(let t of e.effects)(t.teardown||t.ac)&&(t.teardown?.(),t.ac!==null&&at(()=>{t.ac.abort(_e),t.ac=null}),t.fn!==null&&(t.teardown=d),Pr(t,0),nr(t))}function bt(e){if(e.effects!==null)for(let t of e.effects)t.teardown&&t.fn!==null&&Fr(t)}function xt(e,t){if(t.has(e))throw TypeError(`Cannot initialize the same private elements twice on an object`)}function St(e,t){xt(e,t),t.add(e)}function k(e,t,n){xt(e,t),t.set(e,n)}function A(e,t,n){if(typeof e==`function`?e===t:e.has(t))return arguments.length<3?t:n;throw TypeError(`Private element is not present on this object`)}function j(e,t,n){return e.set(A(e,t),n),n}function M(e,t){return e.get(A(e,t))}var Ct,wt=null,N=null,Tt=null,Et=null,Dt=null,Ot=!1,kt=!1,At=null,jt=null,Mt=0,Nt=1,Pt=new WeakMap,Ft=new WeakMap,It=new WeakMap,Lt=new WeakMap,Rt=new WeakMap,zt=new WeakMap,Bt=new WeakMap,Vt=new WeakMap,Ht=new WeakMap,Ut=new WeakMap,Wt=new WeakMap,Gt=new WeakMap,Kt=new WeakMap,qt=new WeakMap,Jt=new WeakMap,Yt=new WeakSet,Xt=class e{constructor(){St(this,Yt),_(this,`id`,Nt++),k(this,Pt,!1),_(this,`linked`,!0),k(this,Ft,null),k(this,It,null),_(this,`async_deriveds`,new Map),_(this,`current`,new Map),_(this,`previous`,new Map),k(this,Lt,new Set),k(this,Rt,new Set),k(this,zt,0),k(this,Bt,new Map),k(this,Vt,null),k(this,Ht,[]),k(this,Ut,[]),k(this,Wt,new Set),k(this,Gt,new Set),k(this,Kt,new Map),k(this,qt,new Set),_(this,`is_fork`,!1),k(this,Jt,!1),wt===null?wt=this:(j(It,wt,this),j(Ft,this,wt)),wt=this}skip_effect(e){M(Kt,this).has(e)||M(Kt,this).set(e,{d:[],m:[]}),M(qt,this).delete(e)}unskip_effect(e,t=e=>this.schedule(e)){var n=M(Kt,this).get(e);if(n){M(Kt,this).delete(e);for(var r of n.d)O(r,y),t(r);for(r of n.m)O(r,b),t(r)}M(qt,this).add(e)}capture(e,t,n=!1){e.v!==w&&!this.previous.has(e)&&this.previous.set(e,e.v),e.f&8388608||(this.current.set(e,[t,n]),Et?.set(e,t)),this.is_fork||(e.v=t)}activate(){N=this}deactivate(){N=null,Et=null}flush(){try{kt=!0,N=this,A(Yt,this,$t).call(this)}finally{Mt=0,Dt=null,At=null,jt=null,kt=!1,N=null,Et=null,mn.clear()}}discard(){for(let e of M(Rt,this))e(this);M(Rt,this).clear();for(let e of this.async_deriveds.values())e.reject(ft);A(Yt,this,an).call(this),M(Vt,this)?.resolve()}register_created_effect(e){M(Ut,this).push(e)}increment(e,t){if(j(zt,this,M(zt,this)+1),e){let e=M(Bt,this).get(t)??0;M(Bt,this).set(t,e+1)}}decrement(e,t){if(j(zt,this,M(zt,this)-1),e){let e=M(Bt,this).get(t)??0;e===1?M(Bt,this).delete(t):M(Bt,this).set(t,e-1)}M(Jt,this)||(j(Jt,this,!0),Qe(()=>{j(Jt,this,!1),this.linked&&this.flush()}))}transfer_effects(e,t){for(let t of e)M(Wt,this).add(t);for(let e of t)M(Gt,this).add(e);e.clear(),t.clear()}oncommit(e){M(Lt,this).add(e)}ondiscard(e){M(Rt,this).add(e)}settled(){return(M(Vt,this)??j(Vt,this,p())).promise}static ensure(){if(N===null){let t=N=new e;!kt&&!Ot&&Qe(()=>{M(Pt,t)||t.flush()})}return N}apply(){Et=null}schedule(e){Dt=e,e.b?.is_pending&&e.f&16777228&&!(e.f&32768)?e.b.defer_effect(e):M(Ht,this).push(e)}};Ct=Xt;function Zt(){if(this.is_fork)return!0;for(let n of M(Bt,this).keys()){for(var e=n,t=!1;e.parent!==null;){if(M(Kt,this).has(e)){t=!0;break}e=e.parent}if(!t)return!0}return!1}function Qt(){var e=[];for(let i of M(Ht,this))if(!(i.f&16384||!(i.f&6144))){for(var t=i,n=!1;t.parent!==null;){t=t.parent;var r=t.f;if(r&96){if(!(r&1024)){n=!0;break}t.f^=v}}n||e.push(t)}return j(Ht,this,[]),e}function $t(){j(Pt,this,!0);for(let e of M(Wt,this))M(Gt,this).delete(e),O(e,y),this.schedule(e);for(let e of M(Gt,this))O(e,b),this.schedule(e);this.apply();for(var e=At=[],t=[],n=jt=[];M(Ht,this).length>0;){Mt++>1e3&&(A(Yt,this,an).call(this),sn());for(let n of A(Yt,this,Qt).call(this))try{A(Yt,this,en).call(this,n,e,t)}catch(e){throw fn(n),A(Yt,this,Zt).call(this)||this.discard(),e}}if(N=null,n.length>0){var r=Ct.ensure();for(let e of n)r.schedule(e)}if(At=null,jt=null,A(Yt,this,Zt).call(this)){A(Yt,this,rn).call(this,t),A(Yt,this,rn).call(this,e);for(let[e,t]of M(Kt,this))dn(e,t);n.length>0&&A(Yt,N,$t).call(N);return}let i=A(Yt,this,tn).call(this);if(i)A(Yt,this,rn).call(this,t),A(Yt,this,rn).call(this,e),A(Yt,i,nn).call(i,this);else{M(Wt,this).clear(),M(Gt,this).clear();for(let e of M(Lt,this))e(this);M(Lt,this).clear(),Tt=this,ln(t),ln(e),Tt=null,M(Vt,this)?.resolve();var a=N;if(M(zt,this)===0&&(M(Ht,this).length===0||a!==null)&&A(Yt,this,an).call(this),M(Ht,this).length>0){if(a!==null){for(let e of M(Ht,this))M(Ht,a).push(e);j(Ht,this,[])}else a=this}a!==null&&(mn.clear(),A(Yt,a,$t).call(a))}}function en(e,t,n){e.f^=v;for(var r=e.first;r!==null;){var i=r.f,a=!!(i&96);if(!(a&&i&1024||i&8192||M(Kt,this).has(r))&&r.fn!==null){a?r.f^=v:i&4?t.push(r):kr(r)&&(i&16&&M(Gt,this).add(r),Fr(r));var o=r.first;if(o!==null){r=o;continue}}for(;r!==null;){var s=r.next;if(s!==null){r=s;break}r=r.parent}}}function tn(){for(var e=M(Ft,this);e!==null;){if(!e.is_fork){for(let[t,[,n]]of this.current)if(e.current.has(t)&&!n)return e}e=M(Ft,e)}return null}function nn(e){for(let[t,n]of e.current)!this.previous.has(t)&&e.previous.has(t)&&this.previous.set(t,e.previous.get(t)),this.current.set(t,n);for(let[t,n]of e.async_deriveds){let e=this.async_deriveds.get(t);e&&n.promise.then(e.resolve).catch(e.reject)}e.async_deriveds.clear(),this.transfer_effects(M(Wt,e),M(Gt,e));let t=e=>{var n=e.reactions;if(n!==null&&!(e.f&2&&!(e.f&6144)))for(let e of n){var r=e.f;if(r&2)t(e);else{var i=e;r&4194320&&!this.async_deriveds.has(i)&&(M(Gt,this).delete(i),O(i,y),this.schedule(i))}}};for(let e of this.current.keys())t(e);this.oncommit(()=>e.discard()),A(Yt,e,an).call(e),N=this,A(Yt,this,$t).call(this)}function rn(e){for(var t=0;t<e.length;t+=1)nt(e[t],M(Wt,this),M(Gt,this))}function an(){if(this.linked){var e=M(Ft,this),t=M(It,this);e===null||j(It,e,t),t===null?wt=e:j(Ft,t,e),this.linked=!1}}function on(e){var t=Ot,n=Tt;Tt=null,Ot=!0;try{var r;for(e&&(on(),r=e());;){if($e(),N===null)return r;N.flush()}}finally{Ot=t,Tt=n}}function sn(){try{Re()}catch(e){Vn(e,Dt)}}var cn=null;function ln(e){var t=e.length;if(t!==0){for(var n=0;n<t;){var r=e[n++];if(!(r.f&24576)&&kr(r)&&(cn=new Set,Fr(r),r.deps===null&&r.first===null&&r.nodes===null&&r.teardown===null&&r.ac===null&&or(r),cn?.size>0)){mn.clear();for(let e of cn){if(e.f&24576)continue;let t=[e],n=e.parent;for(;n!==null;)cn.has(n)&&(cn.delete(n),t.push(n)),n=n.parent;for(let e=t.length-1;e>=0;e--){let n=t[e];n.f&24576||Fr(n)}}cn.clear()}}cn=null}}function un(e){N.schedule(e)}function dn(e,t){if(!(e.f&32&&e.f&1024)){e.f&2048?t.d.push(e):e.f&4096&&t.m.push(e),O(e,v);for(var n=e.first;n!==null;)dn(n,t),n=n.next}}function fn(e){O(e,v);for(var t=e.first;t!==null;)fn(t),t=t.next}var pn=new Set,mn=new Map,hn=!1;function gn(e,t){return{f:0,v:e,reactions:null,equals:Ae,rv:0,wv:0}}function _n(e,t){let n=gn(e,t);return yr(n),n}function vn(e,t=!1,n=!0){let r=gn(e);return t||(r.equals=Me),r}function P(e,t,n=!1){return B!==null&&(!hr||B.f&131072)&&Ye()&&B.f&4325394&&(vr===null||!vr.has(e))&&He(),xn(e,n?Tn(t):t,jt)}var yn=null,bn=0;function xn(e,t,n=null){if(!e.equals(t)){pr?mn.set(e,t):mn.has(e)||mn.set(e,e.v);var r=Xt.ensure();if(r.capture(e,t),e.f&2){let t=e;e.f&2048&&_t(t),Et===null&&tt(t)}e.wv=Or(),yn=null,bn=0,wn(e,y,n),yn=null,Ye()&&V!==null&&V.f&1024&&!(V.f&96)&&(Sr===null?Cr([e]):Sr.push(e)),!r.is_fork&&pn.size>0&&!hn&&Sn()}return t}function Sn(){hn=!1;for(let e of pn){e.f&1024&&O(e,b);let t;try{t=kr(e)}catch{t=!0}t&&Fr(e)}pn.clear()}function Cn(e){P(e,e.v+1)}function wn(e,t,n){var r=e.reactions;if(r!==null){var i=Ye(),a=r.length;if(bn+=a,bn>1e5&&yn===null&&(yn=new Set),yn!==null){if(yn.has(e))return;yn.add(e)}for(var o=0;o<a;o++){var s=r[o],c=s.f;if(i||s!==V){var l=(c&y)===0;if(l&&O(s,t),c&131072)pn.add(s);else if(c&2){var u=s;Et?.delete(u),wn(u,b,n)}else if(l){var d=s;c&16&&cn!==null&&cn.add(d),n===null?un(d):n.push(d)}}}}}function Tn(t){if(typeof t!=`object`||!t||ce in t||le in t)return t;let n=l(t);if(n!==s&&n!==c)return t;var r=new Map,i=e(t),o=_n(0),u=null,d=Er,f=e=>{if(Er===d)return e();var t=B,n=Er;gr(null),Dr(d);var r=e();return gr(t),Dr(n),r};return i&&r.set(`length`,_n(t.length,u)),new Proxy(t,{defineProperty(e,t,n){(!(`value`in n)||n.configurable===!1||n.enumerable===!1||n.writable===!1)&&Be();var i=r.get(t);return i===void 0?f(()=>{var e=_n(n.value,u);return r.set(t,e),e}):P(i,n.value,!0),!0},deleteProperty(e,t){var n=r.get(t);if(n===void 0){if(t in e){let e=f(()=>_n(w,u));r.set(t,e),Cn(o)}}else P(n,w),Cn(o);return!0},get(e,n,i){if(n===ce)return t;var o=r.get(n),s=n in e;if(o===void 0&&(!s||a(e,n)?.writable)&&(o=f(()=>_n(Tn(s?e[n]:w),u)),r.set(n,o)),o!==void 0){var c=H(o);return c===w?void 0:c}return Reflect.get(e,n,i)},getOwnPropertyDescriptor(e,t){this.has?.(e,t);var n=Reflect.getOwnPropertyDescriptor(e,t),i=r.get(t);if(i!==void 0){var a=H(i);if(a===w)return;if(n&&`value`in n)n.value=a;else return{enumerable:!0,configurable:!0,value:a,writable:!0}}return n},has(e,t){if(t===ce)return!0;var n=r.get(t),i=n!==void 0&&n.v!==w||Reflect.has(e,t);return(n!==void 0||V!==null&&(!i||a(e,t)?.writable))&&(n===void 0&&(n=f(()=>_n(i?Tn(e[t]):w,u)),r.set(t,n)),H(n)===w)?!1:i},set(e,t,n,s){var c=r.get(t),l=t in e;if(i&&t===`length`)for(var d=n;d<c.v;d+=1){var p=r.get(d+``);p===void 0?d in e&&(p=f(()=>_n(w,u)),r.set(d+``,p)):P(p,w)}if(c===void 0)(!l||a(e,t)?.writable)&&(c=f(()=>_n(void 0,u)),P(c,Tn(n)),r.set(t,c));else{l=c.v!==w;var m=f(()=>Tn(n));P(c,m)}var h=Reflect.getOwnPropertyDescriptor(e,t);if(h?.set&&h.set.call(s,n),!l){if(i&&typeof t==`string`){var g=r.get(`length`),_=Number(t);Number.isInteger(_)&&_>=g.v&&P(g,_+1)}Cn(o)}return!0},ownKeys(e){H(o);var t=Reflect.ownKeys(e).filter(e=>{var t=r.get(e);return t===void 0||t.v!==w});for(var[n,i]of r)i.v!==w&&!(n in e)&&t.push(n);return t},setPrototypeOf(){Ve()}})}function En(e){try{if(typeof e==`object`&&e&&ce in e)return e[ce]}catch{}return e}function Dn(e,t){return Object.is(En(e),En(t))}var On,kn,An,jn;function Mn(){if(On===void 0){On=window,kn=/Firefox/.test(navigator.userAgent);var e=Element.prototype,t=Node.prototype,n=Text.prototype;An=a(t,`firstChild`).get,jn=a(t,`nextSibling`).get,u(e)&&(e[pe]=void 0,e[fe]=null,e[me]=void 0,e.__e=void 0),u(n)&&(n[he]=void 0)}}function Nn(e=``){return document.createTextNode(e)}function Pn(e){return An.call(e)}function Fn(e){return jn.call(e)}function F(e,t){if(!T)return Pn(e);var n=Pn(E);if(n===null)n=E.appendChild(Nn());else if(t&&n.nodeType!==3){var r=Nn();return n?.before(r),Te(r),r}return t&&zn(n),Te(n),n}function I(e,t=!1){if(!T){var n=Pn(e);return n instanceof Comment&&n.data===``?Fn(n):n}if(t){if(E?.nodeType!==3){var r=Nn();return E?.before(r),Te(r),r}zn(E)}return E}function L(e,t=!1){if(!T)return Pn(e);var n=F(e,t);return D(e),n}function R(e,t=1,n=!1){let r=T?E:e;for(var i;t--;)i=r,r=Fn(r);if(!T)return r;if(n){if(r?.nodeType!==3){var a=Nn();return r===null?i?.after(a):r.before(a),Te(a),a}zn(r)}return Te(r),r}function In(e){e.textContent=``}function Ln(){return!1}function Rn(e,t,n){return t==null||t===`http://www.w3.org/1999/xhtml`?n?document.createElement(e,{is:n}):document.createElement(e):n?document.createElementNS(t,e,{is:n}):document.createElementNS(t,e)}function zn(e){if(e.nodeValue.length<65536)return;let t=e.nextSibling;for(;t!==null&&t.nodeType===3;)t.remove(),e.nodeValue+=t.nodeValue,t=e.nextSibling}function Bn(e){var t=V;if(t===null)return B.f|=se,e;if(!(t.f&32768)&&!(t.f&4))throw e;Vn(e,t)}function Vn(e,t){if(e===ye)throw e;if(!(t!==null&&t.f&16384)){for(;t!==null;){if(t.f&128&&!(t.f&33570816)){if(!(t.f&32768))throw e;try{t.b.error(e);return}catch(t){e=t}}t=t.parent}throw e}}function Hn(e){V===null&&(B===null&&Le(e),Ie()),pr&&Fe(e)}function Un(e,t){var n=t.last;n===null?t.last=t.first=e:(n.next=e,e.prev=n,t.last=e)}function Wn(e,t){var n=V;n!==null&&n.f&8192&&(e|=x);var r={ctx:We,deps:null,nodes:null,f:e|y|512,first:null,fn:t,last:null,next:null,parent:n,b:n&&n.b,prev:null,teardown:null,wv:0,ac:null};N?.register_created_effect(r);var i=r;if(e&4)At===null?Xt.ensure().schedule(r):At.push(r);else if(t!==null){try{Fr(r)}catch(e){throw ir(r),e}i.deps===null&&i.teardown===null&&i.nodes===null&&i.first===i.last&&!(i.f&524288)&&(i=i.first,e&16&&e&65536&&i!==null&&(i.f|=te))}if(i!==null&&(i.parent=n,n!==null&&Un(i,n),B!==null&&B.f&2&&!(e&64))){var a=B;(a.effects??(a.effects=[])).push(i)}return r}function Gn(){return B!==null&&!hr}function Kn(e){let t=Wn(8,null);return O(t,v),t.teardown=e,t}function qn(e){Hn(`$effect`);var t=V.f;if(!B&&t&32&&We!==null&&!We.i){var n=We;(n.e??(n.e=[])).push(e)}else return Jn(e)}function Jn(e){return Wn(4|re,e)}function Yn(e){Xt.ensure();let t=Wn(64|ne,e);return(e={})=>new Promise(n=>{e.outro?sr(t,()=>{ir(t),n(void 0)}):(ir(t),n(void 0))})}function Xn(e){return Wn(4,e)}function Zn(e){return Wn(oe|ne,e)}function Qn(e,t=0){return Wn(8|t,e)}function z(e,t=[],n=[],r=[]){st(r,t,n,t=>{Wn(8,()=>{e(...t.map(H))})})}function $n(e,t=0){return Wn(16|t,e)}function er(e){return Wn(32|ne,e)}function tr(e){var t=e.teardown;if(t!==null){let n=pr,r=B;mr(!0),gr(null);try{t.call(null)}catch(t){Vn(t,e.parent)}finally{mr(n),gr(r)}}}function nr(e,t=!1){var n=e.first;for(e.first=e.last=null;n!==null;){let e=n.ac;e!==null&&at(()=>{e.abort(_e)});var r=n.next;n.f&64?n.parent=null:ir(n,t),n=r}}function rr(e){for(var t=e.first;t!==null;){var n=t.next;t.f&32||ir(t),t=n}}function ir(e,t=!0){var n=!1;(t||e.f&262144)&&e.nodes!==null&&e.nodes.end!==null&&(ar(e.nodes.start,e.nodes.end),n=!0),e.f|=C,nr(e,t&&!n),Pr(e,0);var r=e.nodes&&e.nodes.t;if(r!==null)for(let e of r)e.stop();tr(e),e.f^=C,e.f|=S;var i=e.parent;i!==null&&i.first!==null&&or(e),e.next=e.prev=e.teardown=e.ctx=e.deps=e.fn=e.nodes=e.ac=e.b=null}function ar(e,t){for(;e!==null;){var n=e===t?null:Fn(e);e.remove(),e=n}}function or(e){var t=e.parent,n=e.prev,r=e.next;n!==null&&(n.next=r),r!==null&&(r.prev=n),t!==null&&(t.first===e&&(t.first=r),t.last===e&&(t.last=n))}function sr(e,t,n=!0){var r=[];e.f|=256,cr(e,r,!0);var i=()=>{n&&ir(e),t&&t()},a=r.length;if(a>0){var o=()=>--a||i();for(var s of r)s.out(o)}else i()}function cr(e,t,n){if(!(e.f&8192)){e.f^=x;var r=e.nodes&&e.nodes.t;if(r!==null)for(let e of r)(e.is_global||n)&&t.push(e);for(var i=e.first;i!==null;){var a=i.next;if(!(i.f&64)){var o=!!(i.f&65536)||!!(i.f&32)&&!!(e.f&16);cr(i,t,o?n:!1)}i=a}}}function lr(e){e.f&=-257,ur(e,!0)}function ur(e,t){if(!(e.f&256)&&e.f&8192){e.f^=x,e.f&1024||(O(e,y),Xt.ensure().schedule(e));for(var n=e.first;n!==null;){var r=n.next,i=!!(n.f&65536)||!!(n.f&32);ur(n,i?t:!1),n=r}var a=e.nodes&&e.nodes.t;if(a!==null)for(let e of a)(e.is_global||t)&&e.in()}}function dr(e,t){if(e.nodes)for(var n=e.nodes.start,r=e.nodes.end;n!==null;){var i=n===r?null:Fn(n);t.append(n),n=i}}var fr=null,pr=!1;function mr(e){pr=e}var B=null,hr=!1;function gr(e){B=e}var V=null;function _r(e){V=e}var vr=null;function yr(e){B!==null&&(B.f&2097152||B.f&2)&&(vr??(vr=new Set)).add(e)}var br=null,xr=0,Sr=null;function Cr(e){Sr=e}var wr=1,Tr=0,Er=Tr;function Dr(e){Er=e}function Or(){return++wr}function kr(e){var t=e.f;if(t&2048)return!0;if(t&4096){for(var n=e.deps,r=n.length,i=0;i<r;i++){var a=n[i];if(kr(a)&&vt(a),a.wv>e.wv)return!0}t&512&&Et===null&&O(e,v)}return!1}function Ar(e,t,n=!0){var r=e.reactions;if(r!==null&&!(vr!==null&&vr.has(e)))for(var i=0;i<r.length;i++){var a=r[i];a.f&2?Ar(a,t,!1):t===a&&(n?O(a,y):a.f&1024&&O(a,b),un(a))}}function jr(e){var t=br,n=xr,r=Sr,i=B,a=vr,o=We,s=hr,c=Er,l=e.f;br=null,xr=0,Sr=null,B=l&96?null:e,vr=null,Ge(e.ctx),hr=!1,Er=++Tr,e.ac!==null&&(at(()=>{e.ac.abort(_e)}),e.ac=null);try{e.f|=ae;var u=e.fn,d=u();e.f|=ee;var f=Mr(e);if(Ye()&&Sr!==null&&!hr&&f!==null&&!(e.f&6146))for(var p=0;p<Sr.length;p++)Ar(Sr[p],e);if(i!==null&&i!==e){if(Tr++,i.deps!==null)for(let e=0;e<n;e+=1)i.deps[e].rv=Tr;if(t!==null)for(let e of t)e.rv=Tr;Sr!==null&&(r===null?r=Sr:r.push(...Sr))}return e.f&8388608&&(e.f^=se),d}catch(t){return Mr(e),Bn(t)}finally{e.f^=ae,br=t,xr=n,Sr=r,B=i,vr=a,Ge(o),hr=s,Er=c}}function Mr(e){var t=e.deps,n=N?.is_fork;if(br!==null){var r;if(n||Pr(e,xr),t!==null&&xr>0)for(t.length=xr+br.length,r=0;r<br.length;r++)t[xr+r]=br[r];else e.deps=t=br;if(Gn()&&e.f&512)for(r=xr;r<t.length;r++){var i;((i=t[r]).reactions??(i.reactions=[])).push(e)}}else!n&&t!==null&&xr<t.length&&(Pr(e,xr),t.length=xr);return t}function Nr(e,r){let i=r.reactions;if(i!==null){var a=t.call(i,e);if(a!==-1){var o=i.length-1;o===0?i=r.reactions=null:(i[a]=i[o],i.pop())}}if(i===null&&r.f&2&&(br===null||!n.call(br,r))){var s=r;s.f&512&&(s.f^=512),s.v!==w&&tt(s),s.ac!==null&&at(()=>{s.ac.abort(_e),s.ac=null,O(s,y)}),yt(s),Pr(s,0)}}function Pr(e,t){var n=e.deps;if(n!==null)for(var r=t;r<n.length;r++)Nr(e,n[r])}function Fr(e){var t=e.f;if(!(t&16384)){O(e,v);var n=V;V=e;try{t&16777232?rr(e):nr(e),tr(e);var r=jr(e);e.teardown=typeof r==`function`?r:null,e.wv=wr}finally{V=n}}}async function Ir(){await Promise.resolve(),on()}function H(e){var t=!!(e.f&2);if(fr?.add(e),B!==null&&!hr&&!(V!==null&&V.f&16384)&&(vr===null||!vr.has(e))){var r=B.deps;if(B.f&2097152)e.rv<Tr&&(e.rv=Tr,br===null&&r!==null&&r[xr]===e?xr++:br===null?br=[e]:br.push(e));else{var i;(i=B).deps??(i.deps=[]),n.call(B.deps,e)||B.deps.push(e);var a=e.reactions;a===null?e.reactions=[B]:n.call(a,B)||a.push(B)}}if(pr&&mn.has(e))return mn.get(e);if(t){var o=e;if(pr){var s=o.v;return(!(o.f&1024)&&o.reactions!==null||Rr(o))&&(s=_t(o)),mn.set(o,s),s}var c=!(o.f&512)&&!hr&&B!==null&&!!(B.f&512),l=(o.f&ee)===0;kr(o)&&(c&&(o.f|=512),vt(o)),c&&!l&&(bt(o),Lr(o))}if(Et?.has(e))return Et.get(e);if(e.f&8388608)throw e.v;return e.v}function Lr(e){if(e.f|=512,e.deps!==null)for(let r of e.deps){var t=r.reactions;t===null?r.reactions=[e]:n.call(t,e)||t.push(e),r.f&2&&!(r.f&512)&&(bt(r),Lr(r))}}function Rr(e){if(e.v===w)return!0;if(e.deps===null)return!1;for(let t of e.deps)if(mn.has(t)||t.f&2&&Rr(t))return!0;return!1}function zr(e){var t=hr;try{return hr=!0,e()}finally{hr=t}}[...`allowfullscreen.async.autofocus.autoplay.checked.controls.default.disabled.formnovalidate.indeterminate.inert.ismap.loop.multiple.muted.nomodule.novalidate.open.playsinline.readonly.required.reversed.seamless.selected.webkitdirectory.defer.disablepictureinpicture.disableremoteplayback`.split(`.`)];var Br=[`touchstart`,`touchmove`];function Vr(e){return Br.includes(e)}var Hr=Symbol(`events`),Ur=new Set,Wr=new Set;function Gr(e,t,n,r={}){function i(e){if(r.capture||Xr.call(t,e),!e.cancelBubble)return at(()=>n?.call(this,e))}return e.startsWith(`pointer`)||e.startsWith(`touch`)||e===`wheel`?(i.__removed=!1,Qe(()=>{i.__removed||t.addEventListener(e,i,r)})):t.addEventListener(e,i,r),i}function Kr(e,t,n,r,i){var a={capture:r,passive:i},o=Gr(e,t,n,a);(t===document.body||t===window||t===document||t instanceof HTMLMediaElement)&&Kn(()=>{o.__removed=!0,t.removeEventListener(e,o,a)})}function U(e,t,n){(t[Hr]??(t[Hr]={}))[e]=n}function qr(e){for(var t=0;t<e.length;t++)Ur.add(e[t]);for(var n of Wr)n(e)}var Jr=null,Yr=!1;function Xr(e){var t=this,n=t.ownerDocument,r=e.type,a=e.composedPath?.()||[],o=a[0]||e.target;Jr=e,Yr||(Yr=!0,setTimeout(()=>{Yr=!1,Jr=null}));var s=0,c=Jr===e&&e[Hr];if(c){var l=a.indexOf(c);if(l!==-1&&(t===document||t===window)){e[Hr]=t;return}var u=a.indexOf(t);if(u===-1)return;l<=u&&(s=l)}if(o=a[s]||e.target,o!==t){i(e,`currentTarget`,{configurable:!0,get(){return o||n}});var d=B,f=V;gr(null),_r(null);try{for(var p,m=[];o!==null&&o!==t;){try{var h=o[Hr]?.[r];h!=null&&(!o.disabled||e.target===o)&&h.call(o,e)}catch(e){p?m.push(e):p=e}if(e.cancelBubble)break;s++,o=s<a.length?a[s]:null}if(p){for(let e of m)queueMicrotask(()=>{throw e});throw p}}finally{e[Hr]=t,delete e.currentTarget,gr(d),_r(f)}}}var Zr=globalThis?.window?.trustedTypes&&globalThis.window.trustedTypes.createPolicy(`svelte-trusted-html`,{createHTML:e=>e});function Qr(e){return Zr?.createHTML(e)??e}function $r(e){var t=Rn(`template`);return t.innerHTML=Qr(e.replaceAll(`<!>`,`<!---->`)),t.content}function ei(e,t){var n=V;n.nodes===null&&(n.nodes={start:e,end:t,a:null,t:null})}function W(e,t){var n=!!(t&1),r=!!(t&2),i,a=!e.startsWith(`<!>`);return()=>{if(T)return ei(E,null),E;i===void 0&&(i=$r(a?e:`<!>`+e),n||(i=Pn(i)));var t=r||kn?document.importNode(i,!0):i.cloneNode(!0);if(n){var o=Pn(t),s=t.lastChild;ei(o,s)}else ei(t,t);return t}}function ti(e,t,n=`svg`){var r=!e.startsWith(`<!>`),i=!!(t&1),a=`<${n}>${r?e:`<!>`+e}</${n}>`,o;return()=>{if(T)return ei(E,null),E;if(!o){var e=Pn($r(a));if(i)for(o=document.createDocumentFragment();Pn(e);)o.appendChild(Pn(e));else o=Pn(e)}var t=o.cloneNode(!0);if(i){var n=Pn(t),r=t.lastChild;ei(n,r)}else ei(t,t);return t}}function ni(e,t){return ti(e,t,`svg`)}function ri(e=``){if(!T){var t=Nn(e+``);return ei(t,t),t}var n=E;return n.nodeType===3?zn(n):(n.before(n=Nn()),Te(n)),ei(n,n),n}function ii(){if(T)return ei(E,null),E;var e=document.createDocumentFragment(),t=document.createComment(``),n=Nn();return e.append(t,n),ei(t,n),e}function G(e,t){if(T){var n=V;(!(n.f&32768)||n.nodes.end===null)&&(n.nodes.end=E),Ee()}else e!==null&&e.before(t)}function ai(e){let t=0,n=gn(0),r;return()=>{Gn()&&(H(n),Qn(()=>(t===0&&(r=zr(()=>e(()=>Cn(n)))),t+=1,()=>{Qe(()=>{--t,t===0&&(r?.(),r=void 0,Cn(n))})})))}}var oi=te|ne;function si(e,t,n,r){new wi(e,t,n,r)}var ci=new WeakMap,li=new WeakMap,ui=new WeakMap,di=new WeakMap,fi=new WeakMap,pi=new WeakMap,mi=new WeakMap,hi=new WeakMap,gi=new WeakMap,_i=new WeakMap,vi=new WeakMap,yi=new WeakMap,bi=new WeakMap,xi=new WeakMap,Si=new WeakMap,Ci=new WeakMap,K=new WeakSet,wi=class{constructor(e,t,n,r){St(this,K),_(this,`parent`,void 0),_(this,`is_pending`,!1),_(this,`transform_error`,void 0),k(this,ci,void 0),k(this,li,T?E:null),k(this,ui,void 0),k(this,di,void 0),k(this,fi,void 0),k(this,pi,null),k(this,mi,null),k(this,hi,null),k(this,gi,null),k(this,_i,0),k(this,vi,0),k(this,yi,!1),k(this,bi,new Set),k(this,xi,new Set),k(this,Si,null),k(this,Ci,ai(()=>(j(Si,this,gn(M(_i,this))),()=>{j(Si,this,null)}))),j(ci,this,e),j(ui,this,t),j(di,this,e=>{var t=V;t.b=this,t.f|=128,n(e)}),this.parent=V.b,this.transform_error=r??this.parent?.transform_error??(e=>e),j(fi,this,$n(()=>{if(T){let e=ke(M(li,this));Ee();let t=e===`[!`;if(e.startsWith(`[?`)){let t=JSON.parse(e.slice(2));A(K,this,Ei).call(this,t)}else t?A(K,this,ki).call(this):A(K,this,Ti).call(this)}else A(K,this,Ai).call(this)},oi)),T&&j(ci,this,E)}defer_effect(e){nt(e,M(bi,this),M(xi,this))}is_rendered(){return!this.is_pending&&(!this.parent||this.parent.is_rendered())}has_pending_snippet(){return!!M(ui,this).pending}update_pending_count(e,t){A(K,this,Ni).call(this,e,t),j(_i,this,M(_i,this)+e),M(Si,this)&&!M(yi,this)&&(j(yi,this,!0),Qe(()=>{j(yi,this,!1),M(Si,this)&&xn(M(Si,this),M(_i,this))}))}get_effect_pending(){return M(Ci,this).call(this),H(M(Si,this))}error(e){if(e===ye||!M(ui,this).onerror&&!M(ui,this).failed)throw e;N?.is_fork?(M(pi,this)&&N.skip_effect(M(pi,this)),M(mi,this)&&N.skip_effect(M(mi,this)),M(hi,this)&&N.skip_effect(M(hi,this)),N.oncommit(()=>{A(K,this,Oi).call(this)||A(K,this,Pi).call(this,e)})):A(K,this,Pi).call(this,e)}};function Ti(){try{j(pi,this,er(()=>M(di,this).call(this,M(ci,this))))}catch(e){this.error(e)}}function Ei(e){let t=M(ui,this).failed,{reset:n,invoke_onerror:r}=A(K,this,Di).call(this,e);Qe(r),t&&j(hi,this,er(()=>{t(M(ci,this),()=>e,()=>n)}))}function Di(e){var t=!1,n=!1;let r=()=>{A(K,this,Oi).call(this)||(t?Ce():(t=!0,n&&Ue(),M(hi,this)!==null&&sr(M(hi,this),()=>{j(hi,this,null)}),A(K,this,Mi).call(this,()=>{A(K,this,Ai).call(this)})))};return{reset:r,invoke_onerror:()=>{if(!A(K,this,Oi).call(this))try{n=!0,M(ui,this).onerror?.(e,r),n=!1}catch(e){Vn(e,M(fi,this)&&M(fi,this).parent)}}}}function Oi(){return(M(fi,this).f&(S|C))!==0}function ki(){let e=M(ui,this).pending;e&&(this.is_pending=!0,j(mi,this,er(()=>e(M(ci,this)))),Qe(()=>{if(!A(K,this,Oi).call(this)){var e=j(gi,this,document.createDocumentFragment()),t=Nn(),n=!1;e.append(t),j(pi,this,A(K,this,Mi).call(this,()=>{try{return er(()=>M(di,this).call(this,t))}catch(e){try{this.error(e),n=!0}catch(e){Vn(e,M(fi,this).parent)}return null}})),M(pi,this)===null?(j(gi,this,null),n&&A(K,this,ji).call(this,N)):M(vi,this)===0&&(M(ci,this).before(e),j(gi,this,null),sr(M(mi,this),()=>{j(mi,this,null)}),A(K,this,ji).call(this,N))}}))}function Ai(){try{if(this.is_pending=this.has_pending_snippet(),j(vi,this,0),j(_i,this,0),j(pi,this,er(()=>{M(di,this).call(this,M(ci,this))})),M(vi,this)>0){var e=j(gi,this,document.createDocumentFragment());dr(M(pi,this),e);let t=M(ui,this).pending;j(mi,this,er(()=>t(M(ci,this))))}else A(K,this,ji).call(this,N)}catch(e){this.error(e)}}function ji(e){this.is_pending=!1,e.transfer_effects(M(bi,this),M(xi,this))}function Mi(e){var t=V,n=B,r=We;_r(M(fi,this)),gr(M(fi,this)),Ge(M(fi,this).ctx);try{return Xt.ensure(),e()}finally{_r(t),gr(n),Ge(r)}}function Ni(e,t){if(this.has_pending_snippet())j(vi,this,M(vi,this)+e),M(vi,this)===0&&(A(K,this,ji).call(this,t),M(mi,this)&&sr(M(mi,this),()=>{j(mi,this,null)}),M(gi,this)&&(M(ci,this).before(M(gi,this)),j(gi,this,null)));else if(this.parent){var n;A(K,n=this.parent,Ni).call(n,e,t)}}function Pi(e){M(pi,this)&&(ir(M(pi,this)),j(pi,this,null)),M(mi,this)&&(ir(M(mi,this)),j(mi,this,null)),M(hi,this)&&(ir(M(hi,this)),j(hi,this,null)),T&&(Te(M(li,this)),De(),Te(Oe()));let t=M(ui,this).failed,n=e=>{if(A(K,this,Oi).call(this))return;let{reset:n,invoke_onerror:r}=A(K,this,Di).call(this,e);r(),t&&!A(K,this,Oi).call(this)&&j(hi,this,A(K,this,Mi).call(this,()=>{try{return er(()=>{var r=V;r.b=this,r.f|=128,t(M(ci,this),()=>e,()=>n)})}catch(e){return Vn(e,M(fi,this).parent),null}}))};Qe(()=>{if(!A(K,this,Oi).call(this)){var t;try{t=this.transform_error(e)}catch(e){Vn(e,M(fi,this)&&M(fi,this).parent);return}typeof t==`object`&&t&&typeof t.then==`function`?t.then(n,e=>Vn(e,M(fi,this)&&M(fi,this).parent)):n(t)}})}function q(e,t){var n=t==null?``:typeof t==`object`?`${t}`:t;n!==(e[he]??(e[he]=e.nodeValue))&&(e[he]=n,e.nodeValue=`${n}`)}function Fi(e,t){return Li(e,t)}var Ii=new Map;function Li(e,{target:t,anchor:n,props:i={},events:a,context:o,intro:s=!0,transformError:c}){Mn();var l=void 0,u=Yn(()=>{var s=n??t.appendChild(Nn());si(s,{pending:()=>{}},t=>{Ke({});var n=We;if(o&&(n.c=o),a&&(i.$$events=a),T&&ei(t,null),l=e(t,i)||Je(),T&&(V.nodes.end=E,E===null||E.nodeType!==8||E.data!==`]`))throw xe(),ye;qe()},c);var u=new Set,d=e=>{for(var n=0;n<e.length;n++){var r=e[n];if(!u.has(r)){u.add(r);var i=Vr(r);for(let e of[t,document]){var a=Ii.get(e);a===void 0&&(a=new Map,Ii.set(e,a));var o=a.get(r);o===void 0?(e.addEventListener(r,Xr,{passive:i}),a.set(r,1)):a.set(r,o+1)}}}};return d(r(Ur)),Wr.add(d),()=>{for(var e of u)for(let n of[t,document]){var r=Ii.get(n),i=r.get(e);--i==0?(n.removeEventListener(e,Xr),r.delete(e),r.size===0&&Ii.delete(n)):r.set(e,i)}Wr.delete(d),s!==n&&s.parentNode?.removeChild(s)}});return Ri.set(l,u),l}var Ri=new WeakMap,zi=new WeakMap,Bi=new WeakMap,Vi=new WeakMap,Hi=new WeakMap,Ui=new WeakMap,Wi=new WeakMap,Gi=new WeakMap,Ki=class{constructor(e,t=!0){_(this,`anchor`,void 0),k(this,zi,new Map),k(this,Bi,new Map),k(this,Vi,new Map),k(this,Hi,new Set),k(this,Ui,!0),k(this,Wi,e=>{if(M(zi,this).has(e)){var t=M(zi,this).get(e),n=M(Bi,this).get(t);if(n)lr(n),M(Hi,this).delete(t);else{var r=M(Vi,this).get(t);r&&(lr(r.effect),M(Bi,this).set(t,r.effect),M(Vi,this).delete(t),r.fragment.lastChild.remove(),this.anchor.before(r.fragment),n=r.effect)}for(let[t,n]of M(zi,this)){if(M(zi,this).delete(t),t===e)break;let r=M(Vi,this).get(n);r&&(ir(r.effect),M(Vi,this).delete(n))}for(let[e,r]of M(Bi,this)){if(e===t||M(Hi,this).has(e))continue;let i=()=>{if(Array.from(M(zi,this).values()).includes(e)){var t=document.createDocumentFragment();dr(r,t),t.append(Nn()),M(Vi,this).set(e,{effect:r,fragment:t})}else ir(r);M(Hi,this).delete(e),M(Bi,this).delete(e)};M(Ui,this)||!n?(M(Hi,this).add(e),sr(r,i,!1)):i()}}}),k(this,Gi,e=>{M(zi,this).delete(e);let t=Array.from(M(zi,this).values());for(let[e,n]of M(Vi,this))t.includes(e)||(ir(n.effect),M(Vi,this).delete(e))}),this.anchor=e,j(Ui,this,t)}ensure(e,t){var n=N,r=Ln();if(t&&!M(Bi,this).has(e)&&!M(Vi,this).has(e)){if(r){var i=document.createDocumentFragment(),a=Nn();i.append(a),M(Vi,this).set(e,{effect:er(()=>t(a)),fragment:i})}else M(Bi,this).set(e,er(()=>t(this.anchor)))}if(M(zi,this).set(n,e),r){for(let[t,r]of M(Bi,this))t===e?n.unskip_effect(r):n.skip_effect(r);for(let[t,r]of M(Vi,this))t===e?n.unskip_effect(r.effect):n.skip_effect(r.effect);n.oncommit(M(Wi,this)),n.ondiscard(M(Gi,this))}else T&&(this.anchor=E),M(Wi,this).call(this,n)}};function J(e,t,n=!1){var r;T&&(r=E,Ee());var i=new Ki(e),a=n?te:0;function o(e,t){if(T){var n=ke(r);if(e!==parseInt(n.substring(1))){var a=Oe();Te(a),i.anchor=a,we(!1),i.ensure(e,t),we(!0);return}}i.ensure(e,t)}$n(()=>{var e=!1;t((t,n=0)=>{e=!0,o(n,t)}),e||o(-1,null)},a)}var qi=Symbol(`NaN`);function Ji(e,t,n){T&&Ee();var r=new Ki(e),i=!Ye();$n(()=>{var e=t();e!==e&&(e=qi),i&&typeof e==`object`&&e&&(e={}),r.ensure(e,n)})}function Y(e,t){return t}function Yi(e,t,n){for(var i=[],a=t.length,o,s=t.length,c=0;c<a;c++){let n=t[c];sr(n,()=>{if(o){if(o.pending.delete(n),o.done.add(n),o.pending.size===0){var t=e.outrogroups;Xi(e,r(o.done)),t.delete(o),t.size===0&&(e.outrogroups=null)}}else--s},!1)}if(s===0){var l=i.length===0&&n!==null&&e.pending.size===0;if(l){var u=n,d=u.parentNode;In(d),d.append(u),e.items.clear()}Xi(e,t,!l)}else o={pending:new Set(t),done:new Set},(e.outrogroups??(e.outrogroups=new Set)).add(o)}function Xi(e,t,n=!0){var r;if(e.pending.size>0){r=new Set;for(let t of e.pending.values())for(let n of t)r.add(e.items.get(n).e)}for(var i=0;i<t.length;i++){var a=t[i];r?.has(a)?(a.f|=ie,dr(a,document.createDocumentFragment())):ir(t[i],n)}}var Zi;function X(t,n,i,a,o,s=null){var c=t,l=new Map;if(n&4){var u=t;c=T?Te(Pn(u)):u.appendChild(Nn())}T&&Ee();var d=null,f=ht(()=>{var t=i();return e(t)?t:t==null?[]:r(t)}),p=new Map,m=!0;function h(e){if(!(_.effect.f&16384)){_.pending.delete(e);var t=H(f);_.fallback=d,$i(_,t,c,n,a),d!==null&&(t.length===0?d.f&33554432?(d.f^=ie,ta(d,null,c)):lr(d):sr(d,()=>{d=null}))}}function g(e){_.pending.delete(e)}var _={effect:$n(()=>{var e=H(f),t=e.length;let r=!1;T&&ke(c)===`[!`!=(t===0)&&(c=Oe(),Te(c),we(!1),r=!0);for(var u=new Set,_=N,v=Ln(),y=0;y<t;y+=1){T&&E.nodeType===8&&E.data===`]`&&(c=E,r=!0,we(!1));var b=e[y],x=a(b,y),S=m?null:l.get(x);S?(S.v&&xn(S.v,b),S.i&&xn(S.i,y),v&&_.unskip_effect(S.e)):(S=ea(l,m?c:Zi??(Zi=Nn()),b,x,y,o,n,i),m||(S.e.f|=ie),l.set(x,S)),u.add(x)}if(t===0&&s&&!d&&(m?d=er(()=>s(c)):(d=er(()=>s(Zi??(Zi=Nn()))),d.f|=ie)),t>u.size&&Pe(``,``,``),T&&t>0&&Te(Oe()),!m){if(p.set(_,u),v){for(let[e,t]of l)u.has(e)||_.skip_effect(t.e);_.oncommit(h),_.ondiscard(g)}else h(_)}r&&we(!0),H(f)}),flags:n,items:l,pending:p,outrogroups:null,fallback:d};m=!1,T&&(c=E)}function Qi(e){for(;e!==null&&!(e.f&32);)e=e.next;return e}function $i(e,t,n,i,a){var o=!!(i&8),s=t.length,c=e.items,l=Qi(e.effect.first),u,d=null,f,p=[],m=[],h,g,_,v;if(o)for(v=0;v<s;v+=1)h=t[v],g=a(h,v),_=c.get(g).e,_.f&33554432||(_.nodes?.a?.measure(),(f??(f=new Set)).add(_));for(v=0;v<s;v+=1){if(h=t[v],g=a(h,v),_=c.get(g).e,e.outrogroups!==null)for(let t of e.outrogroups)t.pending.delete(_),t.done.delete(_);if(_.f&8192&&(lr(_),o&&(_.nodes?.a?.unfix(),(f??(f=new Set)).delete(_))),_.f&33554432){if(_.f^=ie,_===l)ta(_,null,n);else{var y=d?d.next:l;_===e.effect.last&&(e.effect.last=_.prev),_.prev&&(_.prev.next=_.next),_.next&&(_.next.prev=_.prev),na(e,d,_),na(e,_,y),ta(_,y,n),d=_,p=[],m=[],l=Qi(d.next);continue}}if(_!==l){if(u!==void 0&&u.has(_)){if(p.length<m.length){var b=m[0],x;d=b.prev;var S=p[0],ee=p[p.length-1];for(x=0;x<p.length;x+=1)ta(p[x],b,n);for(x=0;x<m.length;x+=1)u.delete(m[x]);na(e,S.prev,ee.next),na(e,d,S),na(e,ee,b),l=b,d=ee,--v,p=[],m=[]}else u.delete(_),ta(_,l,n),na(e,_.prev,_.next),na(e,_,d===null?e.effect.first:d.next),na(e,d,_),d=_;continue}for(p=[],m=[];l!==null&&l!==_;)(u??(u=new Set)).add(l),m.push(l),l=Qi(l.next);if(l===null)continue}_.f&33554432||p.push(_),d=_,l=Qi(_.next)}if(e.outrogroups!==null){for(let t of e.outrogroups)t.pending.size===0&&(Xi(e,r(t.done)),e.outrogroups?.delete(t));e.outrogroups.size===0&&(e.outrogroups=null)}if(l!==null||u!==void 0){var C=[];if(u!==void 0)for(_ of u)_.f&8192||C.push(_);for(;l!==null;)!(l.f&8192)&&l!==e.fallback&&C.push(l),l=Qi(l.next);var te=C.length;if(te>0){var ne=i&4&&s===0?n:null;if(o){for(v=0;v<te;v+=1)C[v].nodes?.a?.measure();for(v=0;v<te;v+=1)C[v].nodes?.a?.fix()}Yi(e,C,ne)}}o&&Qe(()=>{if(f!==void 0)for(_ of f)_.nodes?.a?.apply()})}function ea(e,t,n,r,i,a,o,s){var c=o&1?o&16?gn(n):vn(n,!1,!1):null,l=o&2?gn(i):null;return{v:c,i:l,e:er(()=>(a(t,c??n,l??i,s),()=>{e.delete(r)}))}}function ta(e,t,n){if(e.nodes)for(var r=e.nodes.start,i=e.nodes.end,a=t&&!(t.f&33554432)?t.nodes.start:n;r!==null;){var o=Fn(r);if(a.before(r),r===i)return;r=o}}function na(e,t,n){t===null?e.effect.first=n:t.next=n,n===null?e.effect.last=t:n.prev=t}function ra(e){var t,n,r=``;if(typeof e==`string`||typeof e==`number`)r+=e;else if(typeof e==`object`){if(Array.isArray(e)){var i=e.length;for(t=0;t<i;t++)e[t]&&(n=ra(e[t]))&&(r&&(r+=` `),r+=n)}else for(n in e)e[n]&&(r&&(r+=` `),r+=n)}return r}function ia(){for(var e,t,n=0,r=``,i=arguments.length;n<i;n++)(e=arguments[n])&&(t=ra(e))&&(r&&(r+=` `),r+=t);return r}function aa(e){return typeof e==`object`?ia(e):e??``}var oa=[...` 	
\r\f\xA0\v﻿`];function sa(e,t,n){var r=e==null?``:``+e;if(t&&(r=r?r+` `+t:t),n){for(var i of Object.keys(n))if(n[i])r=r?r+` `+i:i;else if(r.length)for(var a=i.length,o=0;(o=r.indexOf(i,o))>=0;){var s=o+a;(o===0||oa.includes(r[o-1]))&&(s===r.length||oa.includes(r[s]))?r=(o===0?``:r.substring(0,o))+r.substring(s+1):o=s}}return r===``?null:r}function Z(e,t,n,r,i,a){var o=e[pe];if(T||o!==n||o===void 0){var s=sa(n,r,a);(!T||s!==e.getAttribute(`class`))&&(s==null?e.removeAttribute(`class`):t?e.className=s:e.setAttribute(`class`,s)),e[pe]=n}else if(a&&i!==a)for(var c in a){var l=!!a[c];(i==null||l!==!!i[c])&&e.classList.toggle(c,l)}return a}function ca(e,t){t?e.hasAttribute(`selected`)||e.setAttribute(`selected`,``):e.removeAttribute(`selected`)}function la(t,n){var r=t.__defaultValue,i=t.multiple,a=i?r??[]:null;if(!i||e(a)){var o=t.selectedIndex,s=n&&i?new Set(t.selectedOptions):null;for(var c of t.options){var l=pa(c);ca(c,i?a.includes(l):Dn(l,r))}if(n){if(s!==null)for(c of t.options){var u=s.has(c);c.selected!==u&&(c.selected=u)}else t.selectedIndex!==o&&(t.selectedIndex=o)}}}function ua(t,n,r=!1){if(t.multiple){if(n==null)return;if(!e(n))return Se();for(var i of t.options)i.selected=n.includes(pa(i))}else{for(i of t.options)if(Dn(pa(i),n)){i.selected=!0;return}(!r||n!==void 0)&&(t.selectedIndex=-1)}}function da(e){var t=new MutationObserver(t=>{t.every(ma)||(`__defaultValue`in e&&la(e,!1),`__value`in e&&ua(e,e.__value))});t.observe(e,{childList:!0,subtree:!0,attributes:!0,attributeFilter:[`value`]}),Kn(()=>{t.disconnect()})}function fa(e,t,n=t){var r=new WeakSet,i=!0;ot(e,`change`,t=>{var i=t?`[selected]`:`:checked`,a;if(e.multiple)a=[].map.call(e.querySelectorAll(i),pa);else{var o=e.querySelector(i)??e.querySelector(`option:not([disabled])`);a=o&&pa(o)}n(a),e.__value=a,N!==null&&r.add(N)}),Xn(()=>{var a=t();if(e===document.activeElement){var o=N;if(r.has(o))return}if(ua(e,a,i),i&&a===void 0){var s=e.querySelector(`:checked`);s!==null&&(a=pa(s),n(a))}e.__value=a,i=!1})}function pa(e){return`__value`in e?e.__value:e.value}function ma(e){if(e.target.closest(`selectedcontent`)!==null)return!0;if(e.type===`childList`){var t=[...e.addedNodes,...e.removedNodes];return t.length>0&&t.every(e=>e.nodeName===`SELECTEDCONTENT`)}return!1}var ha=Symbol(`is custom element`),ga=Symbol(`is html`),_a=ve?`link`:`LINK`,va=ve?`progress`:`PROGRESS`;function ya(e){if(T){var t=!1,n=()=>{if(!t){if(t=!0,e.hasAttribute(`value`)){var n=e.value;Q(e,`value`,null),e.value=n}if(e.hasAttribute(`checked`)){var r=e.checked;Q(e,`checked`,null),e.checked=r}}};e[ge]=n,Qe(n),it()}}function ba(e,t){var n=Sa(e);n.value!==(n.value=t??void 0)&&(e.value!==t||t===0&&e.nodeName===va)&&(e.value=t??``)}function xa(e,t){var n=Sa(e);n.checked!==(n.checked=t??void 0)&&(e.checked=t)}function Q(e,t,n,r){var i=Sa(e);T&&(i[t]=e.getAttribute(t),t===`src`||t===`srcset`||t===`href`&&e.nodeName===_a)||i[t]!==(i[t]=n)&&(t===`loading`&&(e[de]=n),n==null?e.removeAttribute(t):typeof n!=`string`&&wa(e).has(t)?e[t]=n:e.setAttribute(t,n))}function Sa(e){return e[fe]??(e[fe]={[ha]:e.nodeName.includes(`-`),[ga]:e.namespaceURI===`http://www.w3.org/1999/xhtml`})}var Ca=new Map;function wa(e){var t=e.getAttribute(`is`)||e.nodeName,n=Ca.get(t);if(n)return n;Ca.set(t,n=new Set);for(var r,i=e,a=Element.prototype;a!==i;){for(var s in r=o(i),r)r[s].set&&s!==`innerHTML`&&s!==`textContent`&&s!==`innerText`&&n.add(s);i=l(i)}return n}function Ta(e,t,n=t){var r=new WeakSet;ot(e,`input`,async i=>{var a=i?e.defaultValue:e.value;if(a=Da(e)?Oa(a):a,n(a),N!==null&&r.add(N),await Ir(),a!==(a=t())){var o=e.selectionStart,s=e.selectionEnd,c=e.value.length;if(e.value=a??``,s!==null){var l=e.value.length;o===s&&s===c&&l>c?(e.selectionStart=l,e.selectionEnd=l):(e.selectionStart=o,e.selectionEnd=Math.min(s,l))}}}),(T&&e.defaultValue!==e.value||zr(t)==null&&e.value)&&(n(Da(e)?Oa(e.value):e.value),N!==null&&r.add(N)),Qn(()=>{var n=t();if(e===document.activeElement){var i=N;if(r.has(i))return}Da(e)&&n===Oa(e.value)||(e.type!==`date`||n||e.value)&&n!==e.value&&(e.value=n??``)})}function Ea(e,t,n=t){ot(e,`change`,t=>{n(t?e.defaultChecked:e.checked)}),(T&&e.defaultChecked!==e.checked||zr(t)==null)&&n(e.checked),Qn(()=>{e.checked=!!t()})}function Da(e){var t=e.type;return t===`number`||t===`range`}function Oa(e){return e===``?null:+e}var ka,Aa=new WeakMap,ja=new WeakMap,Ma=new WeakMap,Na=new WeakSet,Pa=class{constructor(e){St(this,Na),k(this,Aa,new WeakMap),k(this,ja,void 0),k(this,Ma,void 0),j(Ma,this,e)}observe(e,t){var n=M(Aa,this).get(e)||new Set;return n.add(t),M(Aa,this).set(e,n),A(Na,this,Fa).call(this).observe(e,M(Ma,this)),()=>{var n=M(Aa,this).get(e);n.delete(t),n.size===0&&(M(Aa,this).delete(e),M(ja,this).unobserve(e))}}};ka=Pa;function Fa(){return M(ja,this)??j(ja,this,new ResizeObserver(e=>{for(var t of e){ka.entries.set(t.target,t);for(var n of M(Aa,this).get(t.target)||[])n(t)}}))}_(Pa,`entries`,new WeakMap);var Ia=!1;function La(e){var t=Ia;try{return Ia=!1,[e(),Ia]}finally{Ia=t}}function Ra(e,t,n,r){var i=!0,o=!!(n&8),s=!!(n&16),c=r,l=!0,u=void 0,d=()=>s&&i?(u??(u=dt(r)),H(u)):(l&&(l=!1,c=s?zr(r):r),c);let f;if(o){var p=ce in e||ue in e;f=a(e,t)?.set??(p&&t in e?n=>e[t]=n:void 0)}var m,h=!1;o?[m,h]=La(()=>e[t]):m=e[t],m===void 0&&r!==void 0&&(m=d(),f&&(i&&ze(t),f(m)));var g=i?()=>{var n=e[t];return n===void 0?d():(l=!0,n)}:()=>{var n=e[t];return n!==void 0&&(c=void 0),n===void 0?c:n};if(i&&!(n&4))return g;if(f){var _=e.$$legacy;return(function(e,t){return arguments.length>0?((!i||!t||_||h)&&f(t?g():e),e):g()})}var v=!1,y=(n&1?dt:ht)(()=>(v=!1,g()));o&&H(y);var b=V;return(function(e,t){if(arguments.length>0){let n=t?H(y):i&&o?Tn(e):e;return P(y,n),v=!0,c!==void 0&&(c=n),e}return pr&&v||b.f&16384?y.v:H(y)})}if(typeof window<`u`){var za,Ba;((za=(Ba=window).__svelte??(Ba.__svelte={})).v??(za.v=new Set)).add(`5`)}var Va=Tn({data:null});function Ha(e){Va.data=e}function $(e,t={}){let n=JSON.stringify({...t,act:e});window.ipc&&typeof window.ipc.postMessage==`function`&&window.ipc.postMessage(n)}var Ua={trusted:`trusted`,ok:`ok`,changed:`changed`,problem:`problem`,error:`error`,untrusted:`none`,off:`none`};function Wa(e){return typeof e==`string`&&Object.hasOwn(Ua,e)&&Ua[e]||`none`}function Ga(e){return`d`+Math.min(3,Math.max(0,Number.isInteger(e)?e:0))}var Ka={comment:`tk-comment`,string:`tk-string`,number:`tk-number`,keyword:`tk-keyword`,name:`tk-name`};function qa(e){return typeof e==`string`&&Object.hasOwn(Ka,e)&&Ka[e]||`tk-plain`}var Ja={python:`b-python`,svelte:`b-svelte`,c:`b-c`};function Ya(e){return typeof e==`string`&&Object.hasOwn(Ja,e)&&Ja[e]||`b-plain`}var Xa={rebuilt:`rebuilt`,partly:`partly`,left:`left`};function Za(e){return typeof e==`string`&&Object.hasOwn(Xa,e)&&Xa[e]||`left`}var Qa=Tn({typed:``});function $a(e){let t=Qa.typed.toLowerCase().split(/\s+/).filter(Boolean),n=typeof e.search==`string`?e.search:``;return t.every(e=>n.includes(e))}var eo=ni(`<svg viewBox="0 0 16 16" aria-hidden="true"><path class="handle" d="M5.5 5.5 L5.5 3.2 L10.5 3.2 L10.5 5.5"></path><rect class="body" x="1.5" y="5.5" width="13" height="8.5" rx="1.5"></rect><path class="lid" d="M3 5.5 L13 5.5 A1.5 1.5 0 0 1 14.5 7 L14.5 8.8 L1.5 8.8 L1.5 7 A1.5 1.5 0 0 1 3 5.5 Z"></path><rect class="latch" x="6.7" y="7.5" width="2.6" height="2.8" rx="0.6"></rect></svg>`),to=ni(`<svg viewBox="0 0 16 16" aria-hidden="true"><path d="M8 1.5 L13.5 3.5 L13.5 8 L12.6 10.6 L10.8 12.8 L8 14.5 L5.2 12.8 L3.4 10.6 L2.5 8 L2.5 3.5 Z"></path></svg>`);function no(e,t){let n=Ra(t,`big`,3,!1);var r=ii(),i=I(r),a=e=>{var t=eo();let r;z(()=>r=Z(t,0,`icon tools`,null,r,{big:n()})),G(e,t)},o=e=>{var t=to();let r;z(()=>r=Z(t,0,`icon shield`,null,r,{big:n()})),G(e,t)};J(i,e=>{t.name===`tools`?e(a):t.name===`shield`&&e(o,1)}),G(e,r)}var ro=W(`<div><p class="navhead"> </p> <button class="navadd">+</button></div>`),io=W(`<p> </p>`),ao=W(`<input class="navfilter" type="search" maxlength="80" title="Shows the kits that say every word typed here, of themselves or of their projects"/> <!>`,1),oo=W(`<!> <!> <!>`,1),so=W(`<button title="Its projects, and what it needs to run them"><!> <span class="words"><span class="label"> </span> <span class="hint"> </span></span></button>`),co=W(`<span aria-hidden="true"></span>`),lo=W(`<span class="hint"> </span>`),uo=W(`<span class="badge" title="Waiting for your answer"> </span>`),fo=W(`<button><!> <!> <span class="words"><span class="label"> </span> <!></span> <!></button>`),po=W(`<nav aria-label="The parts of the Development window"></nav>`);function mo(e,t){Ke(t,!0);let n=mt(()=>t.rows.filter(e=>e.t===`kit`)),r=mt(()=>H(n).length>0&&!H(n).some($a));var i=po();X(i,21,()=>t.rows,Y,(e,t)=>{var n=ii(),i=I(n),a=e=>{var n=oo(),i=I(n),a=e=>{var n=ro(),r=F(n),i=L(r,!0),a=R(r,2);D(n),z(e=>{Z(n,1,`navheadrow ${e??``}`),q(i,H(t).label),Q(a,`title`,H(t).add),Q(a,`aria-label`,H(t).add)},[()=>Ga(H(t).depth)]),U(`click`,a,()=>$(`loadKit`)),G(e,n)},o=e=>{var n=io(),r=L(n,!0);z(e=>{Z(n,1,`navhead ${e??``}`),q(r,H(t).label)},[()=>Ga(H(t).depth)]),G(e,n)};J(i,e=>{H(t).add?e(a):e(o,-1)});var s=R(i,2),c=e=>{var n=io(),r=L(n,!0);z(e=>{Z(n,1,`navsum ${e??``}`),q(r,H(t).says)},[()=>Ga(H(t).depth)]),G(e,n)};J(s,e=>{H(t).says&&e(c)});var l=R(s,2),u=e=>{var n=ao(),i=I(n);ya(i);var a=R(i,2),o=e=>{var n=io(),r=L(n,!0);z(e=>{Z(n,1,`navsum ${e??``}`),q(r,H(t).none)},[()=>Ga(H(t).depth)]),G(e,n)};J(a,e=>{H(r)&&e(o)}),z(()=>{Q(i,`placeholder`,H(t).filter),Q(i,`aria-label`,H(t).filter)}),Ta(i,()=>Qa.typed,e=>Qa.typed=e),G(e,n)};J(l,e=>{H(t).filter&&e(u)}),G(e,n)},o=e=>{var n=ii(),r=I(n),i=e=>{var n=so();let r;var i=F(n);no(i,{get name(){return H(t).icon}});var a=R(i,2),o=F(a),s=L(o,!0),c=L(R(o,2));D(a),D(n),z(()=>{r=Z(n,1,`navkit`,null,r,{selected:H(t).selected,within:H(t).within}),Q(n,`aria-current`,H(t).selected?`page`:void 0),q(s,H(t).label),q(c,`${H(t).hint??``}${H(t).projects?`  ·  `+H(t).projects:``}`)}),U(`click`,n,()=>$(`go`,{to:H(t).go})),G(e,n)},a=mt(()=>$a(H(t)));J(r,e=>{H(a)&&e(i)}),G(e,n)},s=e=>{var n=fo();let r;var i=F(n),a=e=>{var n=co();z(e=>Z(n,1,`dot ${e??``}`),[()=>Wa(H(t).mark)]),G(e,n)};J(i,e=>{H(t).mark&&e(a)});var o=R(i,2),s=e=>{no(e,{get name(){return H(t).icon}})};J(o,e=>{H(t).icon&&e(s)});var c=R(o,2),l=F(c),u=L(l,!0),d=R(l,2),f=e=>{var n=lo(),r=L(n,!0);z(()=>q(r,H(t).hint)),G(e,n)};J(d,e=>{H(t).hint&&e(f)}),D(c);var p=R(c,2),m=e=>{var n=uo(),r=L(n,!0);z(()=>q(r,H(t).badge)),G(e,n)};J(p,e=>{H(t).badge&&e(m)}),D(n),z(e=>{r=Z(n,1,`navitem ${e??``}`,null,r,{selected:H(t).selected}),Q(n,`aria-current`,H(t).selected?`page`:void 0),q(u,H(t).label)},[()=>Ga(H(t).depth)]),U(`click`,n,()=>$(`go`,{to:H(t).go})),G(e,n)};J(i,e=>{H(t).t===`heading`?e(a):H(t).t===`kit`?e(o,1):H(t).t===`item`&&e(s,2)}),G(e,n)}),D(i),G(e,i),qe()}qr([`click`]);var ho={python:`M14.25.18l.9.2.73.26.59.3.45.32.34.34.25.34.16.33.1.3.04.26.02.2-.01.13V8.5l-.05.63-.13.55-.21.46-.26.38-.3.31-.33.25-.35.19-.35.14-.33.1-.3.07-.26.04-.21.02H8.77l-.69.05-.59.14-.5.22-.41.27-.33.32-.27.35-.2.36-.15.37-.1.35-.07.32-.04.27-.02.21v3.06H3.17l-.21-.03-.28-.07-.32-.12-.35-.18-.36-.26-.36-.36-.35-.46-.32-.59-.28-.73-.21-.88-.14-1.05-.05-1.23.06-1.22.16-1.04.24-.87.32-.71.36-.57.4-.44.42-.33.42-.24.4-.16.36-.1.32-.05.24-.01h.16l.06.01h8.16v-.83H6.18l-.01-2.75-.02-.37.05-.34.11-.31.17-.28.25-.26.31-.23.38-.2.44-.18.51-.15.58-.12.64-.1.71-.06.77-.04.84-.02 1.27.05zm-6.3 1.98l-.23.33-.08.41.08.41.23.34.33.22.41.09.41-.09.33-.22.23-.34.08-.41-.08-.41-.23-.33-.33-.22-.41-.09-.41.09zm13.09 3.95l.28.06.32.12.35.18.36.27.36.35.35.47.32.59.28.73.21.88.14 1.04.05 1.23-.06 1.23-.16 1.04-.24.86-.32.71-.36.57-.4.45-.42.33-.42.24-.4.16-.36.09-.32.05-.24.02-.16-.01h-8.22v.82h5.84l.01 2.76.02.36-.05.34-.11.31-.17.29-.25.25-.31.24-.38.2-.44.17-.51.15-.58.13-.64.09-.71.07-.77.04-.84.01-1.27-.04-1.07-.14-.9-.2-.73-.25-.59-.3-.45-.33-.34-.34-.25-.34-.16-.33-.1-.3-.04-.25-.02-.2.01-.13v-5.34l.05-.64.13-.54.21-.46.26-.38.3-.32.33-.24.35-.2.35-.14.33-.1.3-.06.26-.04.21-.02.13-.01h5.84l.69-.05.59-.14.5-.21.41-.28.33-.32.27-.35.2-.36.15-.36.1-.35.07-.32.04-.28.02-.21V6.07h2.09l.14.01zm-6.47 14.25l-.23.33-.08.41.08.41.23.33.33.23.41.08.41-.08.33-.23.23-.33.08-.41-.08-.41-.23-.33-.33-.23-.41-.08-.41.08z`,svelte:`M10.354 21.125a4.44 4.44 0 0 1-4.765-1.767 4.109 4.109 0 0 1-.703-3.107 3.898 3.898 0 0 1 .134-.522l.105-.321.287.21a7.21 7.21 0 0 0 2.186 1.092l.208.063-.02.208a1.253 1.253 0 0 0 .226.83 1.337 1.337 0 0 0 1.435.533 1.231 1.231 0 0 0 .343-.15l5.59-3.562a1.164 1.164 0 0 0 .524-.778 1.242 1.242 0 0 0-.211-.937 1.338 1.338 0 0 0-1.435-.533 1.23 1.23 0 0 0-.343.15l-2.133 1.36a4.078 4.078 0 0 1-1.135.499 4.44 4.44 0 0 1-4.765-1.766 4.108 4.108 0 0 1-.702-3.108 3.855 3.855 0 0 1 1.742-2.582l5.589-3.563a4.072 4.072 0 0 1 1.135-.499 4.44 4.44 0 0 1 4.765 1.767 4.109 4.109 0 0 1 .703 3.107 3.943 3.943 0 0 1-.134.522l-.105.321-.286-.21a7.204 7.204 0 0 0-2.187-1.093l-.208-.063.02-.207a1.255 1.255 0 0 0-.226-.831 1.337 1.337 0 0 0-1.435-.532 1.231 1.231 0 0 0-.343.15L8.62 9.368a1.162 1.162 0 0 0-.524.778 1.24 1.24 0 0 0 .211.937 1.338 1.338 0 0 0 1.435.533 1.235 1.235 0 0 0 .344-.151l2.132-1.36a4.067 4.067 0 0 1 1.135-.498 4.44 4.44 0 0 1 4.765 1.766 4.108 4.108 0 0 1 .702 3.108 3.857 3.857 0 0 1-1.742 2.583l-5.589 3.562a4.072 4.072 0 0 1-1.135.499m10.358-17.95C18.484-.015 14.082-.96 10.9 1.068L5.31 4.63a6.412 6.412 0 0 0-2.896 4.295 6.753 6.753 0 0 0 .666 4.336 6.43 6.43 0 0 0-.96 2.396 6.833 6.833 0 0 0 1.168 5.167c2.229 3.19 6.63 4.135 9.812 2.108l5.59-3.562a6.41 6.41 0 0 0 2.896-4.295 6.756 6.756 0 0 0-.665-4.336 6.429 6.429 0 0 0 .958-2.396 6.831 6.831 0 0 0-1.167-5.168Z`,c:`M16.5921 9.1962s-.354-3.298-3.627-3.39c-3.2741-.09-4.9552 2.474-4.9552 6.14 0 3.6651 1.858 6.5972 5.0451 6.5972 3.184 0 3.5381-3.665 3.5381-3.665l6.1041.365s.36 3.31-2.196 5.836c-2.552 2.5241-5.6901 2.9371-7.8762 2.9201-2.19-.017-5.2261.034-8.1602-2.97-2.938-3.0101-3.436-5.9302-3.436-8.8002 0-2.8701.556-6.6702 4.047-9.5502C7.444.72 9.849 0 12.254 0c10.0422 0 10.7172 9.2602 10.7172 9.2602z`};function go(e){return typeof e==`string`&&Object.hasOwn(ho,e)&&ho[e]||null}var _o=ni(`<svg class="mark" viewBox="0 0 24 24" aria-hidden="true"><path></path></svg>`),vo=W(`<span><!> <span> </span></span>`);function yo(e,t){Ke(t,!0);let n=mt(()=>go(t.part.code));var r=vo(),i=F(r),a=e=>{var t=_o(),r=L(t);z(()=>Q(r,`d`,H(n))),G(e,t)};J(i,e=>{H(n)&&e(a)});var o=L(R(i,2),!0);D(r),z(e=>{Z(r,1,`tech ${e??``}`),q(o,t.part.label)},[()=>Ya(t.part.code)]),G(e,r),qe()}var bo=W(`<p class="small version"> </p>`),xo=W(`<span class="chip"> </span>`),So=W(`<p> </p>`),Co=W(`<p class="small"> </p>`),wo=W(`<p class="warn"> </p>`),To=W(`<p> </p> <p> </p> <!> <!> <!>`,1),Eo=W(`<p class="small empty"> </p>`),Do=W(`<p class="small warn"> </p>`),Oo=W(`<section class="box kit"><div class="boxhead"><!> <h2 class="boxtitle"> </h2> <!> <span class="chip good"> </span> <span class="grow"></span> <button title="Its projects, and what it needs to run them">Open</button></div> <p> </p> <!> <!></section>`),ko=W(`<h2 class="boxtitle"> </h2>`),Ao=W(`<button title="Read the macros this workbook carries. GridCraft does not run them.">Read Its Macros</button>`),jo=W(`<header class="head"><div class="titled"><!> <h1 class="title"> </h1></div> <!> <p class="lede"> </p></header> <section class="box"><div class="boxhead"><span aria-hidden="true"></span> <h2 class="boxtitle"> </h2> <!> <span class="grow"></span> <button title="How far agents may go, how they connect, and what they ask for">Open</button></div> <!></section> <div class="secthead"><h2 class="sect"> </h2> <span class="small"> </span></div> <!> <!> <p class="small"> </p> <h2 class="sect">Open workbook</h2> <section class="box" aria-label="Open workbook"><div class="boxhead"><!> <span class="grow"></span> <!></div> <p> </p> <!></section> <div class="row"><button title="What your projects print and log, what failed, and what agents asked for">Console</button> <span class="small"> </span></div>`,1);function Mo(e,t){Ke(t,!0);let n=mt(()=>t.pane.server),r=mt(()=>t.pane.workbook),i=mt(()=>t.pane.kits.filter($a));var a=jo(),o=I(a),s=F(o),c=F(s);no(c,{name:`tools`,big:!0});var l=L(R(c,2),!0);D(s);var u=R(s,2),d=e=>{var n=bo(),r=L(n,!0);z(()=>q(r,t.pane.version)),G(e,n)};J(u,e=>{t.pane.version&&e(d)});var f=L(R(u,2),!0);D(o);var p=R(o,2),m=F(p),h=F(m),g=R(h,2),_=L(g,!0),v=R(g,2),y=e=>{var t=xo(),r=L(t,!0);z(()=>q(r,H(n).level)),G(e,t)};J(v,e=>{H(n).available&&e(y)});var b=R(v,4);D(m);var x=R(m,2),S=e=>{var t=So(),r=L(t,!0);z(()=>q(r,H(n).unavailable)),G(e,t)},ee=e=>{var t=To(),r=I(t),i=L(r,!0),a=R(r,2),o=L(a,!0),s=R(a,2),c=e=>{var t=Co(),r=L(t,!0);z(()=>q(r,H(n).clients)),G(e,t)};J(s,e=>{H(n).clients&&e(c)});var l=R(s,2),u=e=>{var t=wo(),r=L(t,!0);z(()=>q(r,H(n).waiting)),G(e,t)};J(l,e=>{H(n).waiting&&e(u)});var d=R(l,2),f=e=>{var t=Co(),r=L(t,!0);z(()=>q(r,H(n).plan)),G(e,t)};J(d,e=>{H(n).plan&&e(f)}),z(e=>{q(i,H(n).says),Z(a,1,`status ${e??``}`),q(o,H(n).status.text)},[()=>Wa(H(n).status.kind)]),G(e,t)};J(x,e=>{H(n).available?e(ee,-1):e(S)}),D(p);var C=R(p,2),te=F(C),ne=L(te,!0),re=L(R(te,2),!0);D(C);var ie=R(C,2),ae=e=>{var n=Eo(),r=L(n,!0);z(()=>q(r,t.pane.kitsNone)),G(e,n)};J(ie,e=>{t.pane.kits.length>0&&H(i).length===0&&e(ae)});var oe=R(ie,2);X(oe,17,()=>H(i),Y,(e,t)=>{var n=Oo(),r=F(n),i=F(r);no(i,{name:`shield`});var a=R(i,2),o=L(a,!0),s=R(a,2);X(s,17,()=>H(t).badges,Y,(e,t)=>{yo(e,{get part(){return H(t)}})});var c=R(s,2),l=L(c,!0),u=R(c,4);D(r);var d=R(r,2),f=L(d,!0),p=R(d,2),m=e=>{var n=Do(),r=L(n,!0);z(()=>q(r,H(t).notYet)),G(e,n)};J(p,e=>{H(t).notYet&&e(m)});var h=R(p,2),g=e=>{var n=Co(),r=L(n);z(()=>q(r,`${H(t).projects??``}${H(t).space?`  ·  `+H(t).space:``}`)),G(e,n)};J(h,e=>{H(t).projects&&e(g)}),D(n),z(()=>{Q(n,`aria-label`,H(t).name),q(o,H(t).name),q(l,H(t).standing),Q(u,`aria-label`,`Open ${H(t).name??``}`),q(f,H(t).says)}),U(`click`,u,()=>$(`go`,{to:H(t).go})),G(e,n)});var se=R(oe,2),ce=L(se,!0),le=R(se,4),ue=F(le),de=F(ue),fe=e=>{var t=ko(),n=L(t,!0);z(()=>q(n,H(r).title)),G(e,t)};J(de,e=>{H(r).title&&e(fe)});var pe=R(de,4),me=e=>{var t=Ao();U(`click`,t,()=>$(`go`,{to:`macros`})),G(e,t)};J(pe,e=>{H(r).macros&&e(me)}),D(ue);var he=R(ue,2);let ge;var _e=L(he,!0),ve=R(he,2),ye=e=>{var t=Co(),n=L(t,!0);z(()=>q(n,H(r).rebuilt)),G(e,t)};J(ve,e=>{H(r).rebuilt&&e(ye)}),D(le);var w=R(le,2),be=F(w),xe=L(R(be,2),!0);D(w),z(e=>{q(l,t.pane.title),q(f,t.pane.intro),Q(p,`aria-label`,H(n).name),Z(h,1,`dot ${e??``}`),q(_,H(n).name),Q(b,`aria-label`,`Open ${H(n).name??``}`),q(ne,t.pane.kitsTitle),q(re,t.pane.kitsSummary),q(ce,t.pane.kitsNote),ge=Z(he,1,``,null,ge,{warn:H(r).problem}),q(_e,H(r).says),q(xe,t.pane.console)},[()=>Wa(H(n).available?H(n).status.kind:`off`)]),U(`click`,b,()=>$(`go`,{to:H(n).go})),U(`click`,be,()=>$(`go`,{to:`console`})),G(e,a),qe()}qr([`click`]);var No=W(`<p class="lede"> </p>`),Po=W(`<p> </p>`),Fo=W(`<div class="item waits"><span class="strong"> </span> <code> </code> <span class="grow"></span> <span class="small"> </span> <button class="primary" title="See everything this request would do, and answer it">Look…</button></div>`),Io=W(`<h2 class="sect"> </h2> <!>`,1),Lo=W(`<label class="level"><input type="radio" name="level"/> <span> </span></label>`),Ro=W(`<p class="small"> </p>`),zo=W(`<div class="row"><button title="Copies the command that adds this server to Claude Code. It holds the token.">Copy Setup for Claude Code</button> <button title="Copies an MCP settings entry, for other agents. It holds the token.">Copy Setup as JSON</button> <button title="Makes a new token: an agent that holds the old one can no longer connect">New Token</button></div>`),Bo=W(`<button title="None of its steps is carried out after this. What was done stays done.">Stop Plan</button>`),Vo=W(`<li><span> </span><span class="state"> </span></li>`),Ho=W(`<div class="item plan"><div class="head"><span class="strong"> </span> <span> </span> <span class="grow"></span> <span class="small"> </span> <!></div> <ol></ol></div>`),Uo=W(`<h2 class="sect">Plans</h2> <!>`,1),Wo=W(`<div class="row"><span> </span> <button class="link">Ask again</button></div>`),Go=W(`<h2 class="sect">Not asked again until GridCraft closes</h2> <!>`,1),Ko=W(`<h2 class="sect">Lately</h2> <!> <p class="small"> </p>`,1),qo=W(`<!> <h2 class="sect">How far agents may go</h2> <section class="box"><div class="levels" role="radiogroup" aria-label="How far agents may go"></div> <p class="small"> </p></section> <h2 class="sect">Connection</h2> <section class="box"><p> </p> <!> <div class="row"><label for="port">Port:</label> <input id="port" class="port" type="text" inputmode="numeric" maxlength="5"/> <button>Set Port</button></div> <!></section> <!> <!> <!>`,1),Jo=W(`<header class="head"><h1 class="title"> </h1> <!></header> <!>`,1);function Yo(e,t){Ke(t,!0);let n=_n(``),r=_n(!1);qn(()=>{!H(r)&&t.pane.available&&P(n,t.pane.portText,!0)});function i(e,n){e.preventDefault(),n!==t.pane.level&&$(`level`,{level:n})}function a(){$(`port`,{port:H(n)}),P(r,!1)}var o=Jo(),s=I(o),c=F(s),l=L(c,!0),u=R(c,2),d=e=>{var n=No(),r=L(n,!0);z(()=>q(r,t.pane.intro)),G(e,n)};J(u,e=>{t.pane.available&&e(d)}),D(s);var f=R(s,2),p=e=>{var n=Po(),r=L(n,!0);z(()=>q(r,t.pane.unavailable)),G(e,n)},m=e=>{var o=qo(),s=I(o),c=e=>{var n=Io(),r=I(n),i=L(r,!0);X(R(r,2),17,()=>t.pane.waiting,Y,(e,t)=>{var n=Fo(),r=F(n),i=L(r,!0),a=R(r,2),o=L(a,!0),s=R(a,4),c=L(s,!0),l=R(s,2);D(n),z(()=>{q(i,H(t).kind),q(o,H(t).target),q(c,H(t).named)}),U(`click`,l,()=>$(`look`,{id:H(t).id})),G(e,n)}),z(()=>q(i,t.pane.waitingTitle)),G(e,n)};J(s,e=>{t.pane.waiting.length&&e(c)});var l=R(s,4),u=F(l);X(u,21,()=>t.pane.levels,Y,(e,n)=>{var r=Lo(),a=F(r);ya(a);var o=L(R(a,2),!0);D(r),z(()=>{Q(r,`title`,H(n).meaning),xa(a,H(n).code===t.pane.level),q(o,H(n).label)}),U(`click`,a,e=>i(e,H(n).code)),G(e,r)}),D(u);var d=L(R(u,2),!0);D(l);var f=R(l,4),p=F(f),m=L(p,!0),h=R(p,2),g=e=>{var n=Ro(),r=L(n,!0);z(()=>q(r,t.pane.clients)),G(e,n)};J(h,e=>{t.pane.clients&&e(g)});var _=R(h,2),v=R(F(_),2);ya(v);var y=R(v,2);D(_);var b=R(_,2),x=e=>{var t=zo(),n=F(t),r=R(n,2),i=R(r,2);D(t),U(`click`,n,()=>$(`copySetup`,{json:!1})),U(`click`,r,()=>$(`copySetup`,{json:!0})),U(`click`,i,()=>$(`newToken`)),G(e,t)};J(b,e=>{t.pane.serving&&e(x)}),D(f);var S=R(f,2),ee=e=>{var n=Uo();X(R(I(n),2),17,()=>t.pane.plans,Y,(e,t)=>{var n=Ho(),r=F(n),i=F(r),a=L(i,!0),o=R(i,2),s=L(o,!0),c=R(o,4),l=L(c,!0),u=R(c,2),d=e=>{var n=Bo();U(`click`,n,()=>$(`stopPlan`,{id:H(t).id})),G(e,n)};J(u,e=>{H(t).live&&e(d)}),D(r);var f=R(r,2);X(f,21,()=>H(t).steps,Y,(e,t)=>{var n=Vo();let r;var i=F(n),a=L(i,!0),o=L(R(i),!0);D(n),z(()=>{r=Z(n,1,`small`,null,r,{done:H(t).state===`done`,failed:H(t).state===`failed`,running:H(t).state===`running`}),q(a,H(t).what),q(o,H(t).stands)}),G(e,n)}),D(f),D(n),z(()=>{q(a,H(t).name),q(s,H(t).title),q(l,H(t).stands)}),G(e,n)}),G(e,n)};J(S,e=>{t.pane.plans.length&&e(ee)});var C=R(S,2),te=e=>{var n=Go();X(R(I(n),2),17,()=>t.pane.unasked,Y,(e,t)=>{var n=Wo(),r=F(n),i=L(r,!0),a=R(r,2);D(n),z(()=>q(i,H(t).covers)),U(`click`,a,()=>$(`askAgain`,{project:H(t).project,kind:H(t).kind})),G(e,n)}),G(e,n)};J(C,e=>{t.pane.unasked.length&&e(te)});var ne=R(C,2),re=e=>{var n=Ko(),r=R(I(n),2);X(r,17,()=>t.pane.lately,Y,(e,t)=>{var n=Ro(),r=L(n,!0);z(()=>q(r,H(t))),G(e,n)});var i=L(R(r,2),!0);z(()=>q(i,t.pane.latelyNote)),G(e,n)};J(ne,e=>{t.pane.lately.length&&e(re)}),z(e=>{q(d,t.pane.meaning),Z(p,1,`status ${e??``}`),q(m,t.pane.status.text)},[()=>Wa(t.pane.status.kind)]),U(`input`,v,()=>P(r,!0)),Ta(v,()=>H(n),e=>P(n,e)),U(`click`,y,a),G(e,o)};J(f,e=>{t.pane.available?e(m,-1):e(p)}),z(()=>q(l,t.pane.title)),G(e,o),qe()}qr([`click`,`input`]);var Xo=W(`<p class="warn"> </p>`),Zo=W(`<p> </p>`),Qo=W(`<section class="box"><p> </p> <p class="small"><!> <!></p> <div class="row"><button title="The kit its projects run by"> </button> <span class="grow"></span> <span class="small"> </span> <button title="Your operating system will ask you to confirm">Remove Kit</button></div></section>`),$o=W(`<p class="small fact wide" role="status"> </p>`),es=W(`<form class="fact wide"><label class="small" for="python"> </label> <input id="python" class="name" type="text" maxlength="400"/> <button type="submit" title="Keeps the name, and asks that Python what it is">Check Python</button></form> <!>`,1),ts=W(`<div class="fact wide"><span class="small">Runs with</span> <span> </span></div>`),ns=W(`<button type="button" title="Opens a file dialog to choose the zig program">Browse…</button>`),rs=W(`<form class="fact wide"><label class="small" for="compiler"> </label> <input id="compiler" class="name" type="text" maxlength="400"/> <!> <button type="submit" title="Keeps the name, and asks the compiler what it is">Check Compiler</button></form>`),is=W(`<div class="fact wide"><span class="small"> </span> <span> </span> <span class="small"> </span></div>`),as=W(`<button>Open Folder</button>`),os=W(`<p class="small empty"> </p>`),ss=W(`<span class="chip"> </span>`),cs=W(`<span class="hint"> </span>`),ls=W(`<button class="entry" title="What it is for, what it asks, and what is in it"><span aria-hidden="true"></span> <span class="words"><span class="line1"><span class="name"> </span> <!></span> <!></span> <span> </span> <span class="chevron" aria-hidden="true">›</span></button>`),us=W(`<div class="list"></div>`),ds=W(`<label class="tick"><input type="checkbox"/> <span>It is the code of <strong> </strong>, the workbook that is open</span></label>`),fs=W(`<button title="Creates this kit's showcase project. It does nothing until you trust it.">Create Showcase</button>`),ps=W(`<button title="Shows its panel inside GridCraft's window, beside the sheet">Dock Showcase</button>`),ms=W(`<button>Open in Browser</button>`),hs=W(`<!> <!>`,1),gs=W(`<h2 class="sect">Showcase</h2> <div class="row"><span> </span> <!></div>`,1),_s=W(`<!> <section class="box facts"><div class="fact"><span class="small">Projects</span> <span> </span></div> <div class="fact"><span class="small">Console</span> <span> </span> <button title="What your projects print and log, what failed, and what agents asked for">Open</button></div> <!> <!> <!> <div class="fact wide"><span class="small">Projects folder</span> <code class="path"> </code> <!></div></section> <h2 class="sect"> </h2> <p class="small"> </p> <!> <h2 class="sect">New project</h2> <form class="box"><div class="row"><label for="newname">Name:</label> <input id="newname" class="name" type="text" maxlength="80" placeholder="Its name"/> <button class="primary" type="submit" title="Creates a starter project. It does nothing until you trust it.">Create Project</button></div> <!></form> <!>`,1),vs=W(`<header class="head"><div class="titled"><!> <h1 class="title"> </h1> <!> <span class="chip good"> </span></div> <p class="lede"> </p> <!></header> <!>`,1);function ys(e,t){Ke(t,!0);let n=_n(``),r=_n(!1),i=_n(null),a=mt(()=>H(i)??t.pane.compiler?.named??``),o=_n(null),s=mt(()=>H(o)??t.pane.pythonField?.named??``);function c(e){e.preventDefault(),$(`checkPython`,{python:H(s)})}function l(e){e.preventDefault(),$(`checkCompiler`,{compiler:H(a)})}function u(e){e.preventDefault(),$(`createProject`,{name:H(n),forWorkbook:H(r)&&!!t.pane.newFor,kit:t.pane.id}),P(n,``),P(r,!1)}var d=vs(),f=I(d),p=F(f),m=F(p);no(m,{name:`shield`,big:!0});var h=R(m,2),g=L(h,!0),_=R(h,2);X(_,17,()=>t.pane.badges,Y,(e,t)=>{yo(e,{get part(){return H(t)}})});var v=L(R(_,2),!0);D(p);var y=R(p,2),b=L(y,!0),x=R(y,2),S=e=>{var n=Xo(),r=L(n,!0);z(()=>q(r,t.pane.notYet)),G(e,n)};J(x,e=>{t.pane.notYet&&e(S)}),D(f);var ee=R(f,2),C=e=>{var n=Zo(),r=L(n,!0);z(()=>q(r,t.pane.unavailable)),G(e,n)},te=e=>{var d=_s(),f=I(d),p=e=>{var n=Qo(),r=F(n),i=L(r,!0),a=R(r,2),o=F(a),s=e=>{var n=ri();z(()=>q(n,`Version ${t.pane.package.version??``}.`)),G(e,n)};J(o,e=>{t.pane.package.version&&e(s)});var c=R(o,2),l=e=>{var n=ri();z(()=>q(n,`By ${t.pane.package.author??``}.`)),G(e,n)};J(c,e=>{t.pane.package.author&&e(l)}),D(a);var u=R(a,2),d=F(u),f=L(d,!0),p=R(d,4),m=L(p,!0),h=R(p,2);D(u),D(n),z(()=>{Q(n,`aria-label`,t.pane.package.title),q(i,t.pane.package.says),q(f,t.pane.package.base),q(m,t.pane.package.removeSays)}),U(`click`,d,()=>$(`go`,{to:t.pane.package.baseGo})),U(`click`,h,()=>$(`removeKit`,{id:t.pane.id})),G(e,n)};J(f,e=>{t.pane.package&&e(p)});var m=R(f,2),h=F(m),g=L(R(F(h),2),!0);D(h);var _=R(h,2),v=R(F(_),2),y=L(v,!0),b=R(v,2);D(_);var x=R(_,2),S=e=>{var n=es(),r=I(n),i=F(r),a=L(i,!0),l=R(i,2);ya(l),De(2),D(r);var u=R(r,2),d=e=>{var n=$o(),r=L(n,!0);z(()=>q(r,t.pane.pythonField.status)),G(e,n)};J(u,e=>{t.pane.pythonField.status&&e(d)}),z(()=>{q(a,t.pane.pythonField.title),ba(l,H(s)),Q(l,`placeholder`,t.pane.pythonField.hint),Q(l,`title`,t.pane.pythonField.says)}),Kr(`submit`,r,c),U(`input`,l,e=>P(o,e.target.value,!0)),G(e,n)},ee=e=>{var n=ts(),r=L(R(F(n),2),!0);D(n),z(()=>q(r,t.pane.python)),G(e,n)};J(x,e=>{t.pane.pythonField?e(S):t.pane.python&&e(ee,1)});var C=R(x,2),te=e=>{var n=rs(),r=F(n),o=L(r,!0),s=R(r,2);ya(s);var c=R(s,2),u=e=>{var t=ns();U(`click`,t,()=>{P(i,null),$(`pickCompiler`)}),G(e,t)};J(c,e=>{t.pane.compiler.canPick&&e(u)}),De(2),D(n),z(()=>{q(o,t.pane.compiler.title),ba(s,H(a)),Q(s,`placeholder`,t.pane.compiler.says),Q(s,`title`,t.pane.compiler.hint)}),Kr(`submit`,n,l),U(`input`,s,e=>P(i,e.target.value,!0)),G(e,n)};J(C,e=>{t.pane.compiler&&e(te)});var ne=R(C,2),re=e=>{var n=is(),r=F(n),i=L(r,!0),a=R(r,2),o=L(a,!0),s=L(R(a,2),!0);D(n),z(()=>{Q(n,`title`,t.pane.space.counts),q(i,t.pane.space.title),q(o,t.pane.space.says),q(s,t.pane.space.counts)}),G(e,n)};J(ne,e=>{t.pane.space&&e(re)});var ie=R(ne,2),ae=R(F(ie),2),oe=L(ae,!0),se=R(ae,2),ce=e=>{var t=as();U(`click`,t,()=>$(`openFolder`)),G(e,t)};J(se,e=>{t.pane.canOpenFolder&&e(ce)}),D(ie),D(m);var le=R(m,2),ue=L(le,!0),de=R(le,2),fe=L(de,!0),pe=R(de,2),me=e=>{var n=os(),r=L(n,!0);z(()=>q(r,t.pane.projectsEmpty)),G(e,n)},he=e=>{var n=us();X(n,21,()=>t.pane.projects,Y,(e,t)=>{var n=ls(),r=F(n),i=R(r,2),a=F(i),o=F(a),s=L(o,!0),c=R(o,2),l=e=>{var n=ss(),r=L(n,!0);z(()=>q(r,H(t).for)),G(e,n)};J(c,e=>{H(t).for&&e(l)}),D(a);var u=R(a,2),d=e=>{var n=cs(),r=L(n,!0);z(()=>q(r,H(t).says)),G(e,n)};J(u,e=>{H(t).says&&e(d)}),D(i);var f=R(i,2),p=L(f,!0);De(2),D(n),z((e,i)=>{n.disabled=!H(t).go,Z(r,1,`dot ${e??``}`),q(s,H(t).name),Z(f,1,`standing ${i??``}`),q(p,H(t).statusLabel)},[()=>Wa(H(t).status),()=>Wa(H(t).status)]),U(`click`,n,()=>$(`go`,{to:H(t).go})),G(e,n)}),D(n),G(e,n)};J(pe,e=>{t.pane.projects.length===0?e(me):e(he,-1)});var ge=R(pe,4),_e=F(ge),ve=R(F(_e),2);ya(ve);var ye=R(ve,2);D(_e);var w=R(_e,2),be=e=>{var n=ds(),i=F(n);ya(i);var a=R(i,2),o=L(R(F(a)),!0);De(),D(a),D(n),z(()=>q(o,t.pane.newFor)),Ea(i,()=>H(r),e=>P(r,e)),G(e,n)};J(w,e=>{t.pane.newFor&&e(be)}),D(ge);var xe=R(ge,2),Se=e=>{var n=gs(),r=R(I(n),2),i=F(r),a=L(i,!0),o=R(i,2),s=e=>{var t=fs();U(`click`,t,()=>$(`createShowcase`)),G(e,t)},c=e=>{var n=hs(),r=I(n),i=e=>{var t=ps();U(`click`,t,()=>$(`showShowcase`,{docked:!0})),G(e,t)};J(r,e=>{t.pane.showcase.canDock&&e(i)});var a=R(r,2),o=e=>{var t=ms();U(`click`,t,()=>$(`showShowcase`,{docked:!1})),G(e,t)};J(a,e=>{t.pane.showcase.canBrowse&&e(o)}),G(e,n)};J(o,e=>{t.pane.showcase.stands===`none`?e(s):t.pane.showcase.stands===`trusted`&&e(c,1)}),D(r),z(()=>q(a,t.pane.showcase.text)),G(e,n)};J(xe,e=>{t.pane.showcase&&e(Se)}),z(e=>{q(g,t.pane.projectsSays),q(y,t.pane.console),Q(ae,`title`,t.pane.folder),q(oe,t.pane.folder),q(ue,t.pane.projectsTitle),q(fe,t.pane.projectsNote),ye.disabled=e},[()=>!H(n).trim()]),U(`click`,b,()=>$(`go`,{to:`console`})),Kr(`submit`,ge,u),Ta(ve,()=>H(n),e=>P(n,e)),G(e,d)};J(ee,e=>{t.pane.available?e(te,-1):e(C)}),z(()=>{q(g,t.pane.title),q(v,t.pane.standing),q(b,t.pane.says)}),G(e,d),qe()}qr([`click`,`input`]);var bs=W(`<span class="small"> </span>`),xs=W(`<p class="lede"> </p>`),Ss=W(`<p class="warn"> </p>`),Cs=W(`<p class="error" role="alert"> </p>`),ws=W(`<p> </p>`),Ts=W(`<p class="bullet"> </p>`),Es=W(`<p class="bullet"><code> </code> </p>`),Ds=W(`<p class="bullet warn"> </p>`),Os=W(`<button>Turn Network Off</button>`),ks=W(`<button title="Asks you once more, in a question of your operating system's own">Turn Network On…</button>`),As=W(`<h2 class="sect"> </h2> <p class="small"> </p> <!> <!> <!> <div class="row"><span> </span> <span> </span> <span class="grow"></span> <!></div> <p class="small"> </p>`,1),js=W(`<dt class="small"> </dt> <dd><code> </code></dd>`,1),Ms=W(`<button> </button>`),Ns=W(`<h2 class="sect"> </h2> <p class="small"> </p> <div class="filelist"><!> <!></div>`,1),Ps=W(`<span> </span>`),Fs=W(`<span class="ln"></span><!> `,1),Is=W(`<p class="small"> </p>`),Ls=W(`<pre class="fileview"></pre> <!>`,1),Rs=W(`<div class="row"><strong> </strong> <button>Close File</button></div> <!>`,1),zs=W(`<pre class="output"> </pre>`),Bs=W(`<h2 class="sect"> </h2> <p class="small"> </p> <div class="row"><button class="primary" title="Runs your C compiler over this project's files, and keeps the program beside your settings">Build</button> <span role="status"> </span></div> <!> <!>`,1),Vs=W(`<button title="Shows its panel inside GridCraft's window, beside the sheet">Dock Panel</button>`),Hs=W(`<button title="Opens its panel in your browser">Open in Browser</button>`),Us=W(`<span class="small" role="status">It is docked in the window.</span>`),Ws=W(`<div class="row"><!> <!> <!></div>`),Gs=W(`<h2 class="sect"> </h2> <p class="small"> </p> <!> <!>`,1),Ks=W(`<span class="arrow" aria-hidden="true">→</span><code> </code>`,1),qs=W(`<span class="hint"> </span>`),Js=W(`<span class="hint warn"> </span>`),Ys=W(`<div class="entry still"><span class="words"><span class="line1"><code> </code> <!></span> <!> <!></span> <span> </span></div>`),Xs=W(`<div class="list plain"></div>`),Zs=W(`<h2 class="sect"> </h2> <!> <!>`,1),Qs=W(`<button title="Runs this script with the selection as its data"> </button>`),$s=W(`<h2 class="sect">Run</h2> <p class="small">Runs one of its scripts on the cells that are selected.</p> <div class="row"></div>`,1),ec=W(`<span class="small" role="status">Running…</span> <span class="grow"></span> <button>Stop</button>`,1),tc=W(`<pre> </pre>`),nc=W(`<p class="small">It printed:</p><pre class="output"> </pre>`,1),rc=W(`<section class="box" aria-label="What the script gave back"><div class="boxhead"><strong> </strong> <!></div> <!> <!></section>`),ic=W(`<p class="warn" role="alert"> </p> <div class="row"><button title="Shows it as it is now, to decide on that">Look Again</button></div>`,1),ac=W(`<!> <label class="tick"><input type="checkbox"/> <span> </span></label> <div class="row"><button class="primary" title="Trusts it as you see it here: what it asks, and the files it holds now"> </button> <span class="small"> </span></div>`,1),oc=W(`<div class="row"><button title="It does nothing from then on, until you trust it again">Stop Trusting</button></div>`),sc=W(`<h2 class="sect"> </h2> <section class="box trustbox"><p> </p> <!> <!></section>`,1),cc=W(`<div class="row"><button>Open Project Folder</button></div>`),lc=W(`<header class="head"><div class="titled"><span aria-hidden="true"></span> <h1 class="title"> </h1> <!> <!> <span class="grow"></span> <span class="chip"> </span> <span class="chip"> </span> <span> </span></div> <!></header> <p> </p> <p class="small"> </p> <!> <!> <h2 class="sect"> </h2> <section class="box"></section> <!> <h2 class="sect">What is in it</h2> <dl class="parts"></dl> <!> <!> <!> <!> <!> <!> <!> <!> <!>`,1);function uc(e,t){Ke(t,!0);let n=_n(!1),r=mt(()=>t.pane.trust),i=mt(()=>t.pane.run);var a=lc(),o=I(a),s=F(o),c=F(s),l=R(c,2),u=L(l,!0),d=R(l,2),f=e=>{var n=bs(),r=L(n,!0);z(()=>q(r,t.pane.version)),G(e,n)};J(d,e=>{t.pane.version&&e(f)});var p=R(d,2),m=e=>{var n=bs(),r=L(n);z(()=>q(r,`by ${t.pane.author??``}`)),G(e,n)};J(p,e=>{t.pane.author&&e(m)});var h=R(p,4),g=L(h,!0),_=R(h,2),v=L(_,!0),y=R(_,2),b=L(y,!0);D(s);var x=R(s,2),S=e=>{var n=xs(),r=L(n,!0);z(()=>q(r,t.pane.description)),G(e,n)};J(x,e=>{t.pane.description&&e(S)}),D(o);var ee=R(o,2),C=L(ee,!0),te=R(ee,2),ne=L(te,!0),re=R(te,2),ie=e=>{var n=Ss(),r=L(n,!0);z(()=>q(r,t.pane.notYet)),G(e,n)};J(re,e=>{t.pane.notYet&&e(ie)});var ae=R(re,2),oe=e=>{var n=Cs(),r=L(n);z(()=>q(r,`This project can't be used: ${t.pane.problem??``}.`)),G(e,n)};J(ae,e=>{t.pane.problem&&e(oe)});var se=R(ae,2),ce=L(se,!0),le=R(se,2);X(le,21,()=>t.pane.asks,Y,(e,t)=>{var n=ws();let r;var i=L(n,!0);z(()=>{r=Z(n,1,`bullet`,null,r,{warn:H(t).warn}),q(i,H(t).text)}),G(e,n)},e=>{var n=Ts(),r=L(n,!0);z(()=>q(r,t.pane.asksNothing)),G(e,n)}),D(le);var ue=R(le,2),de=e=>{var n=As(),r=I(n),i=L(r,!0),a=R(r,2),o=L(a,!0),s=R(a,2);X(s,17,()=>t.pane.network.addresses,Y,(e,t)=>{var n=Es(),r=F(n),i=L(r,!0),a=R(r,1,!0);D(n),z(()=>{q(i,H(t).text),q(a,H(t).note?`  ·  `+H(t).note:``)}),G(e,n)});var c=R(s,2),l=e=>{var n=Ts(),r=L(n,!0);z(()=>q(r,t.pane.network.listen)),G(e,n)};J(c,e=>{t.pane.network.listen&&e(l)});var u=R(c,2);X(u,17,()=>t.pane.network.warnings,Y,(e,t)=>{var n=Ds(),r=L(n,!0);z(()=>q(r,H(t))),G(e,n)});var d=R(u,2),f=F(d);let p;var m=L(f,!0),h=R(f,2),g=L(h,!0),_=R(h,4),v=e=>{var n=Os();U(`click`,n,()=>$(`network`,{id:t.pane.id,on:!1})),G(e,n)},y=e=>{var n=ks();z(()=>n.disabled=!t.pane.network.canTurnOn),U(`click`,n,()=>$(`network`,{id:t.pane.id,on:!0})),G(e,n)};J(_,e=>{t.pane.network.on?e(v):e(y,-1)}),D(d);var b=L(R(d,2),!0);z(()=>{q(i,t.pane.network.title),q(o,t.pane.network.says),p=Z(f,1,`chip`,null,p,{good:t.pane.network.on}),q(m,t.pane.network.on?`On`:`Off`),q(g,t.pane.network.state),q(b,t.pane.network.carried)}),G(e,n)};J(ue,e=>{t.pane.network&&e(de)});var fe=R(ue,4);X(fe,21,()=>t.pane.parts,Y,(e,t)=>{var n=js(),r=I(n),i=L(r,!0),a=R(r,2),o=L(F(a),!0);D(a),z(()=>{q(i,H(t).label),q(o,H(t).value)}),G(e,n)}),D(fe);var pe=R(fe,2),me=e=>{var n=Ns(),r=I(n),i=L(r,!0),a=R(r,2),o=L(a,!0),s=R(a,2),c=F(s);X(c,17,()=>t.pane.files.list,Y,(e,n)=>{var r=Ms();let i;var a=L(r,!0);z(()=>{i=Z(r,1,`fileitem`,null,i,{chosen:t.pane.view&&t.pane.view.path===H(n)}),q(a,H(n))}),U(`click`,r,()=>$(`viewFile`,{id:t.pane.id,path:H(n)})),G(e,r)});var l=R(c,2),u=e=>{var n=bs(),r=L(n,!0);z(()=>q(r,t.pane.files.more)),G(e,n)};J(l,e=>{t.pane.files.more&&e(u)}),D(s),z(()=>{q(i,t.pane.files.title),q(o,t.pane.files.says)}),G(e,n)};J(pe,e=>{t.pane.files&&e(me)});var he=R(pe,2),ge=e=>{var n=Rs(),r=I(n),i=F(r),a=L(i,!0),o=R(i,2);D(r);var s=R(r,2),c=e=>{var n=Ss(),r=L(n,!0);z(()=>q(r,t.pane.view.problem)),G(e,n)},l=e=>{var n=Ls(),r=I(n);X(r,21,()=>t.pane.view.lines,Y,(e,t,n)=>{var r=Fs(),i=I(r);i.textContent=n+1;var a=R(i);X(a,17,()=>H(t),Y,(e,t)=>{var n=Ps(),r=L(n,!0);z(e=>{Z(n,1,e),q(r,H(t)[0])},[()=>aa(qa(H(t)[1]))]),G(e,n)});var o=R(a,1,!0);o.nodeValue=`
`,G(e,r)}),D(r);var i=R(r,2),a=e=>{var n=Is(),r=L(n,!0);z(()=>q(r,t.pane.view.more)),G(e,n)};J(i,e=>{t.pane.view.more&&e(a)}),G(e,n)};J(s,e=>{t.pane.view.problem?e(c):e(l,-1)}),z(()=>q(a,t.pane.view.says)),U(`click`,o,()=>$(`closeFile`)),G(e,n)};J(he,e=>{t.pane.view&&e(ge)});var _e=R(he,2),ve=e=>{var n=Bs(),r=I(n),i=L(r,!0),a=R(r,2),o=L(a,!0),s=R(a,2),c=F(s),l=R(c,2);let u;var d=L(l,!0);D(s);var f=R(s,2),p=e=>{var n=Is(),r=L(n,!0);z(()=>q(r,t.pane.build.needsTrust)),G(e,n)};J(f,e=>{t.pane.build.needsTrust&&e(p)});var m=R(f,2),h=e=>{var n=zs(),r=L(n,!0);z(()=>q(r,t.pane.build.output)),G(e,n)};J(m,e=>{t.pane.build.output&&e(h)}),z(()=>{q(i,t.pane.build.title),q(o,t.pane.build.builds),c.disabled=!t.pane.build.can,u=Z(l,1,``,null,u,{warn:t.pane.build.state===`failed`||t.pane.build.state===`stale`}),q(d,t.pane.build.says)}),U(`click`,c,()=>$(`build`,{id:t.pane.id})),G(e,n)};J(_e,e=>{t.pane.build&&e(ve)});var ye=R(_e,2),w=e=>{var n=Gs(),r=I(n),i=L(r,!0),a=R(r,2),o=L(a,!0),s=R(a,2),c=e=>{var n=Ws(),r=F(n),i=e=>{var n=Vs();U(`click`,n,()=>$(`showPanel`,{id:t.pane.id,docked:!0})),G(e,n)};J(r,e=>{t.pane.panel.canDock&&e(i)});var a=R(r,2),o=e=>{var n=Hs();U(`click`,n,()=>$(`showPanel`,{id:t.pane.id,docked:!1})),G(e,n)};J(a,e=>{t.pane.panel.canBrowse&&e(o)});var s=R(a,2),c=e=>{G(e,Us())};J(s,e=>{t.pane.panel.docked&&e(c)}),D(n),G(e,n)};J(s,e=>{(t.pane.panel.canDock||t.pane.panel.canBrowse||t.pane.panel.docked)&&e(c)});var l=R(s,2),u=e=>{var n=Is(),r=L(n,!0);z(()=>q(r,t.pane.panel.needsTrust)),G(e,n)};J(l,e=>{t.pane.panel.needsTrust&&e(u)}),z(()=>{q(i,t.pane.panel.title),q(o,t.pane.panel.says)}),G(e,n)};J(ye,e=>{t.pane.panel&&e(w)});var be=R(ye,2),xe=e=>{var n=Zs(),r=I(n),i=L(r,!0),a=R(r,2),o=e=>{var n=ws();let r;var i=L(n,!0);z(()=>{r=Z(n,1,`small`,null,r,{warn:t.pane.rebuilt.problem}),q(i,t.pane.rebuilt.says)}),G(e,n)};J(a,e=>{t.pane.rebuilt.says&&e(o)});var s=R(a,2),c=e=>{var n=Xs();X(n,21,()=>t.pane.rebuilt.rows,Y,(e,n)=>{var r=Ys(),i=F(r),a=F(i),o=F(a),s=L(o,!0),c=R(o,2),l=e=>{var t=Ks(),r=L(R(I(t)),!0);z(()=>q(r,H(n).script)),G(e,t)};J(c,e=>{H(n).script&&e(l)}),D(a);var u=R(a,2),d=e=>{var t=qs(),r=L(t,!0);z(()=>q(r,H(n).note)),G(e,t)};J(u,e=>{H(n).note&&e(d)});var f=R(u,2),p=e=>{var n=Js(),r=L(n,!0);z(()=>q(r,t.pane.rebuilt.missing)),G(e,n)};J(f,e=>{H(n).scriptMissing&&e(p)}),D(i);var m=R(i,2),h=L(m,!0);D(r),z(e=>{q(s,H(n).from),Z(m,1,`chip state ${e??``}`),q(h,H(n).stateLabel)},[()=>Za(H(n).state)]),G(e,r)}),D(n),G(e,n)};J(s,e=>{t.pane.rebuilt.rows.length&&e(c)}),z(()=>q(i,t.pane.rebuilt.title)),G(e,n)};J(be,e=>{t.pane.rebuilt&&e(xe)});var Se=R(be,2),Ce=e=>{var n=$s(),r=R(I(n),4);X(r,21,()=>t.pane.scripts,Y,(e,n)=>{var r=Qs(),a=L(r,!0);z(()=>{r.disabled=H(i)&&H(i).state===`running`,q(a,H(n))}),U(`click`,r,()=>$(`run`,{id:t.pane.id,script:H(n)})),G(e,r)}),D(r),G(e,n)};J(Se,e=>{t.pane.scripts&&t.pane.scripts.length&&e(Ce)});var T=R(Se,2),we=e=>{var n=rc(),r=F(n),a=F(r),o=L(a,!0),s=R(a,2),c=e=>{var n=ec();U(`click`,R(I(n),4),()=>$(`stopRun`,{id:t.pane.id})),G(e,n)},l=e=>{var t=Ps();let n;var r=L(t,!0);z(()=>{n=Z(t,1,`chip`,null,n,{good:H(i).ok,problem:!H(i).ok}),q(r,H(i).ok?`Finished`:`Failed`)}),G(e,t)};J(s,e=>{H(i).state===`running`?e(c):e(l,-1)}),D(r);var u=R(r,2),d=e=>{var t=tc();let n;var r=L(t,!0);z(()=>{n=Z(t,1,`output`,null,n,{warn:H(i).state!==`running`&&!H(i).ok}),q(r,H(i).text)}),G(e,t)};J(u,e=>{H(i).text&&e(d)});var f=R(u,2),p=e=>{var t=nc(),n=L(R(I(t)),!0);z(()=>q(n,H(i).printed)),G(e,t)};J(f,e=>{H(i).printed&&e(p)}),D(n),z(()=>q(o,H(i).script)),G(e,n)};J(T,e=>{H(i)&&e(we)});var E=R(T,2),Te=e=>{var i=sc(),a=I(i),o=L(a,!0),s=R(a,2),c=F(s),l=L(c,!0),u=R(c,2),d=e=>{var n=ic(),i=I(n),a=L(i,!0),o=L(R(i,2));z(()=>q(a,H(r).moved)),U(`click`,o,()=>$(`lookAgain`,{id:t.pane.id})),G(e,n)},f=e=>{var i=ac(),a=I(i),o=e=>{var t=Ss(),n=L(t,!0);z(()=>q(n,H(r).cant)),G(e,t)};J(a,e=>{H(r).cant&&e(o)});var s=R(a,2),c=F(s);ya(c);var l=L(R(c,2),!0);D(s);var u=R(s,2),d=F(u),f=L(d,!0),p=L(R(d,2),!0);D(u),z(()=>{c.disabled=!H(r).canTrust,q(l,H(r).follow),d.disabled=!H(r).canTrust,q(f,H(r).trustLabel),q(p,H(r).confirms)}),Ea(c,()=>H(n),e=>P(n,e)),U(`click`,d,()=>$(`trust`,{id:t.pane.id,follow:H(n)})),G(e,i)};J(u,e=>{H(r).moved?e(d):H(r).open&&e(f,1)});var p=R(u,2),m=e=>{var n=oc();U(`click`,L(n),()=>$(`stopTrusting`,{id:t.pane.id})),G(e,n)};J(p,e=>{H(r).canStop&&e(m)}),D(s),z(()=>{q(o,H(r).title),q(l,t.pane.statusSays)}),G(e,i)};J(E,e=>{H(r)&&e(Te)});var Ee=R(E,2),De=e=>{var n=cc();U(`click`,L(n),()=>$(`openProjectFolder`,{id:t.pane.id})),G(e,n)};J(Ee,e=>{t.pane.canOpenFolder&&e(De)}),z((e,n)=>{Z(c,1,`dot big ${e??``}`),q(u,t.pane.title),q(g,t.pane.kit),q(v,t.pane.kindLabel),Z(y,1,`chip standing ${n??``}`),q(b,t.pane.statusLabel),q(C,t.pane.forSays),q(ne,t.pane.statusSays),q(ce,t.pane.asksTitle)},[()=>Wa(t.pane.status),()=>Wa(t.pane.status)]),G(e,a),qe()}qr([`click`]);var dc=W(`<button class="fileitem" title="Puts this call in the line"> </button>`),fc=W(`<div class="filelist"></div>`),pc=W(`<p class="small"> </p>`),mc=W(`<span class="small" role="status">Running…</span> <span class="grow"></span> <button>Stop</button>`,1),hc=W(`<span> </span>`),gc=W(`<pre> </pre>`),_c=W(`<p class="small">It printed:</p><pre class="output"> </pre>`,1),vc=W(`<section class="box" aria-label="What the call gave back"><div class="boxhead"><code> </code> <!></div> <!> <!></section>`),yc=W(`<button class="fileitem"> </button>`),bc=W(`<p class="small">Earlier, to run again:</p> <div class="filelist"></div>`,1),xc=W(`<h2 class="sect"> </h2> <p class="small"> </p> <form class="row"><input class="name immediate" type="text" maxlength="2000" spellcheck="false" autocomplete="off"/> <button class="primary" type="submit" title="Runs this call now, as you typed it">Run</button></form> <!> <!> <!> <h2 class="sect">Entries</h2>`,1),Sc=W(`<option> </option>`),Cc=W(`<select aria-label="Whose entries are shown"><option> </option><!></select>`),wc=W(`<p class="small empty"> </p>`),Tc=W(`<div><p class="small"> </p> <pre> </pre></div>`),Ec=W(`<div class="log"></div>`),Dc=W(`<header class="head"><h1 class="title"> </h1> <p class="lede"> </p></header> <!> <div class="row"><span> </span> <span class="grow"></span> <!> <button title="Copies what is shown, as text">Copy</button> <button title="Forgets every entry">Clear</button></div> <!> <p class="small"> </p>`,1);function Oc(e,t){Ke(t,!0);let n=_n(``),r=mt(()=>t.pane.projects.includes(H(n))?H(n):``),i=mt(()=>H(r)?t.pane.entries.filter(e=>e.project===H(r)):t.pane.entries),a=_n(``),o=mt(()=>t.pane.immediate),s=mt(()=>H(o)?H(o).run:null),c=mt(()=>!!H(s)&&H(s).state===`running`);function l(e){e.preventDefault(),H(a).trim()&&$(`immediate`,{line:H(a)})}let u={error:`bad`,refused:`bad`,agent:`agent`};function d(e){return typeof e==`string`&&Object.hasOwn(u,e)&&u[e]||`plain`}var f=Dc(),p=I(f),m=F(p),h=L(m,!0),g=L(R(m,2),!0);D(p);var _=R(p,2),v=e=>{var t=xc(),n=I(t),r=L(n,!0),i=R(n,2),u=L(i,!0),d=R(i,2),f=F(d);ya(f);var p=R(f,2);D(d);var m=R(d,2),h=e=>{var t=fc();X(t,21,()=>H(o).names,Y,(e,t)=>{var n=dc(),r=L(n,!0);z(()=>q(r,H(t))),U(`click`,n,()=>P(a,H(t)+`()`)),G(e,n)}),D(t),G(e,t)},g=e=>{var t=pc(),n=L(t,!0);z(()=>q(n,H(o).nothing)),G(e,t)};J(m,e=>{H(o).names.length?e(h):e(g,-1)});var _=R(m,2),v=e=>{var t=vc(),n=F(t),r=F(n),i=L(r,!0),a=R(r,2),o=e=>{var t=mc();U(`click`,R(I(t),4),()=>$(`stopRun`,{id:H(s).project})),G(e,t)},l=e=>{var t=hc();let n;var r=L(t,!0);z(()=>{n=Z(t,1,`chip`,null,n,{good:H(s).ok,problem:!H(s).ok}),q(r,H(s).ok?`Finished`:`Failed`)}),G(e,t)};J(a,e=>{H(c)?e(o):e(l,-1)}),D(n);var u=R(n,2),d=e=>{var t=gc();let n;var r=L(t,!0);z(()=>{n=Z(t,1,`output`,null,n,{warn:!H(c)&&!H(s).ok}),q(r,H(s).text)}),G(e,t)};J(u,e=>{H(s).text&&e(d)});var f=R(u,2),p=e=>{var t=_c(),n=L(R(I(t)),!0);z(()=>q(n,H(s).printed)),G(e,t)};J(f,e=>{H(s).printed&&e(p)}),D(t),z(()=>q(i,H(s).line)),G(e,t)};J(_,e=>{H(s)&&e(v)});var y=R(_,2),b=e=>{var t=bc(),n=R(I(t),2);X(n,21,()=>H(o).earlier,Y,(e,t)=>{var n=yc(),r=L(n,!0);z(()=>q(r,H(t))),U(`click`,n,()=>P(a,H(t),!0)),G(e,n)}),D(n),G(e,t)};J(y,e=>{H(o).earlier.length&&e(b)}),De(2),z(e=>{q(r,H(o).title),q(u,H(o).says),Q(f,`placeholder`,H(o).hint),Q(f,`aria-label`,H(o).title),p.disabled=e},[()=>!H(a).trim()||H(c)]),Kr(`submit`,d,l),Ta(f,()=>H(a),e=>P(a,e)),G(e,t)};J(_,e=>{H(o)&&e(v)});var y=R(_,2),b=F(y),x=L(b,!0),S=R(b,4),ee=e=>{var r=Cc(),i=F(r),a=L(i,!0);i.value=i.__value=``,X(R(i),17,()=>t.pane.projects,Y,(e,t)=>{var n=Sc(),r=L(n,!0),i={};z(()=>{q(r,H(t)),i!==(i=H(t))&&(n.value=(n.__value=i)??``)}),G(e,n)}),D(r),da(r),z(()=>q(a,t.pane.all)),fa(r,()=>H(n),e=>P(n,e)),G(e,r)};J(S,e=>{t.pane.projects.length>1&&e(ee)});var C=R(S,2),te=R(C,2);D(y);var ne=R(y,2),re=e=>{var n=wc(),r=L(n,!0);z(()=>q(r,t.pane.empty)),G(e,n)},ie=e=>{var t=Ec();X(t,21,()=>H(i),e=>e.seq,(e,t)=>{var n=Tc(),r=F(n),i=L(r,!0),a=L(R(r,2),!0);D(n),z(e=>{Z(n,1,`logentry ${e??``}`),q(i,H(t).who),q(a,H(t).text)},[()=>d(H(t).kind)]),G(e,n)}),D(t),G(e,t)};J(ne,e=>{H(i).length===0?e(re):e(ie,-1)});var ae=L(R(ne,2),!0);z(()=>{q(h,t.pane.title),q(g,t.pane.intro),q(x,t.pane.says),C.disabled=H(i).length===0,te.disabled=t.pane.entries.length===0,q(ae,t.pane.note)}),U(`click`,C,()=>$(`copyConsole`,{project:H(r)||null})),U(`click`,te,()=>$(`clearConsole`)),G(e,f),qe()}qr([`click`]);var kc=W(`<p class="warn"> </p>`),Ac=W(`<p> </p>`),jc=W(`<p class="small"> </p>`),Mc=W(`<button> </button>`),Nc=W(`<span class="ln"></span><span class="tk-plain"> </span> `,1),Pc=W(`<p><strong> </strong></p> <!> <!> <!> <!> <pre class="fileview"></pre> <!>`,1),Fc=W(`<p class="small"> </p> <!> <div class="row"><span class="small"> </span> <button title="Copies a few words to hand your agent. It holds nothing of the macros, and no token.">Copy a Request for Your Agent</button></div> <div class="twocol"><div class="filelist column" aria-label="The modules"></div> <div class="reading"><!></div></div>`,1),Ic=W(`<header class="head"><h1 class="title"> </h1> <p class="lede"> </p></header> <!> <p class="small"> </p>`,1);function Lc(e,t){Ke(t,!0);let n=mt(()=>t.pane.module);var r=Ic(),i=I(r),a=F(i),o=L(a,!0),s=L(R(a,2),!0);D(i);var c=R(i,2),l=e=>{var n=kc(),r=L(n,!0);z(()=>q(r,t.pane.problem)),G(e,n)},u=e=>{var n=Ac(),r=L(n,!0);z(()=>q(r,t.pane.says)),G(e,n)},d=e=>{var r=Fc(),i=I(r),a=L(i,!0),o=R(i,2),s=e=>{var n=jc(),r=L(n,!0);z(()=>q(r,t.pane.covered)),G(e,n)};J(o,e=>{t.pane.covered&&e(s)});var c=R(o,2),l=F(c),u=L(l,!0),d=R(l,2);D(c);var f=R(c,2),p=F(f);X(p,21,()=>t.pane.modules,Y,(e,t)=>{var n=Mc();let r;var i=L(n,!0);z(()=>{r=Z(n,1,`fileitem`,null,r,{chosen:H(t).chosen}),Q(n,`title`,H(t).kind),q(i,H(t).name)}),U(`click`,n,()=>$(`readMacro`,{module:H(t).name})),G(e,n)}),D(p);var m=R(p,2),h=F(m),g=e=>{var t=Pc(),r=I(t),i=L(F(r),!0);D(r);var a=R(r,2),o=e=>{var t=kc(),r=L(t,!0);z(()=>q(r,H(n).problem)),G(e,t)};J(a,e=>{H(n).problem&&e(o)});var s=R(a,2),c=e=>{var t=kc(),r=L(t,!0);z(()=>q(r,H(n).cut)),G(e,t)};J(s,e=>{H(n).cut&&e(c)});var l=R(s,2),u=e=>{var t=jc(),r=L(t,!0);z(()=>q(r,H(n).procedures)),G(e,t)};J(l,e=>{H(n).procedures&&e(u)});var d=R(l,2);X(d,17,()=>H(n).rebuilt,Y,(e,t)=>{var n=Ac();let r;var i=L(n,!0);z(()=>{r=Z(n,1,`small`,null,r,{warn:H(t).warn}),q(i,H(t).text)}),G(e,n)});var f=R(d,2);X(f,21,()=>H(n).lines,Y,(e,t,n)=>{var r=Nc(),i=I(r);i.textContent=n+1;var a=R(i),o=L(a,!0),s=R(a,1,!0);s.nodeValue=`
`,z(()=>q(o,H(t))),G(e,r)}),D(f);var p=R(f,2),m=e=>{var t=jc(),r=L(t,!0);z(()=>q(r,H(n).more)),G(e,t)};J(p,e=>{H(n).more&&e(m)}),z(()=>q(i,H(n).says)),G(e,t)};J(h,e=>{H(n)&&e(g)}),D(m),D(f),z(()=>{q(a,t.pane.says),q(u,t.pane.rebuild)}),U(`click`,d,()=>$(`copyRequest`)),G(e,r)};J(c,e=>{t.pane.problem?e(l):t.pane.has?e(d,-1):e(u,1)});var f=L(R(c,2),!0);z(()=>{q(o,t.pane.title),q(s,t.pane.intro),q(f,t.pane.note)}),G(e,r),qe()}qr([`click`]);var Rc=W(`<p> </p>`),zc=W(`<p class="warn"> </p>`),Bc=W(`<p class="small"> </p>`),Vc=W(`<p class="strong"> </p>`),Hc=W(`<button class="link"> </button>`),Uc=W(`<p class="small toggle"><span> </span> <!></p>`),Wc=W(`<span class="no"></span>`),Gc=W(`<div class="line"><!><span class="src"> </span></div>`),Kc=W(`<div class="code" role="group" aria-label="The text"></div> <!>`,1),qc=W(`<div class="line skip"><span class="no"></span><span class="mark"></span><span class="src"> </span></div>`),Jc=W(`<div><span class="no"> </span><span class="mark"> </span><span class="src"> </span></div>`),Yc=W(`<div class="code" role="group" aria-label="The changes"></div> <!>`,1),Xc=W(`<section class="step"><h2><span> </span> <code> </code></h2> <!></section>`);function Zc(e,t){Ke(t,!0);let n={gone:`-`,new:`+`,same:` `};var r=ii();X(I(r),17,()=>t.blocks,Y,(e,t)=>{var r=ii(),i=I(r),a=e=>{var n=Rc(),r=L(n,!0);z(()=>q(r,H(t).text)),G(e,n)},o=e=>{var n=zc(),r=L(n,!0);z(()=>q(r,H(t).text)),G(e,n)},s=e=>{var n=Bc(),r=L(n,!0);z(()=>q(r,H(t).text)),G(e,n)},c=e=>{var n=Vc(),r=L(n,!0);z(()=>q(r,H(t).text)),G(e,n)},l=e=>{var n=Rc();let r;var i=L(n,!0);z(()=>{r=Z(n,1,`bullet`,null,r,{warn:H(t).warn}),q(i,H(t).text)}),G(e,n)},u=e=>{var n=Uc(),r=F(n),i=L(r,!0),a=R(r,2),o=e=>{var n=Hc(),r=L(n,!0);z(()=>q(r,H(t).button)),U(`click`,n,()=>$(`whole`,{whole:H(t).whole})),G(e,n)};J(a,e=>{H(t).button&&e(o)}),D(n),z(()=>q(i,H(t).text)),G(e,n)},d=e=>{var n=Kc(),r=I(n);X(r,21,()=>H(t).lines,Y,(e,n,r)=>{var i=Gc(),a=F(i),o=e=>{var t=Wc();t.textContent=r+1,G(e,t)};J(a,e=>{H(t).numbered&&e(o)});var s=L(R(a),!0);D(i),z(()=>q(s,H(n))),G(e,i)}),D(r);var i=R(r,2),a=e=>{var n=zc(),r=L(n,!0);z(()=>q(r,H(t).cut)),G(e,n)};J(i,e=>{H(t).cut&&e(a)}),G(e,n)},f=e=>{var r=Yc(),i=I(r);X(i,21,()=>H(t).rows,Y,(e,t)=>{var r=ii(),i=I(r),a=e=>{var n=qc(),r=L(R(F(n),2));D(n),z(()=>q(r,`⋯ ${H(t).text??``}`)),G(e,n)},o=e=>{var r=Jc(),i=F(r),a=L(i,!0),o=R(i),s=L(o,!0),c=L(R(o),!0);D(r),z(()=>{Z(r,1,`line ${H(t).k??``}`),q(a,H(t).n),q(s,n[H(t).k]??` `),q(c,H(t).text)}),G(e,r)};J(i,e=>{H(t).k===`skip`?e(a):e(o,-1)}),G(e,r)}),D(i);var a=R(i,2),o=e=>{var n=zc(),r=L(n,!0);z(()=>q(r,H(t).cut)),G(e,n)};J(a,e=>{H(t).cut&&e(o)}),G(e,r)},p=e=>{var n=Xc(),r=F(n),i=F(r),a=L(i,!0),o=L(R(i,2),!0);D(r),Zc(R(r,2),{get blocks(){return H(t).blocks}}),D(n),z(()=>{q(a,H(t).heading),q(o,H(t).target)}),G(e,n)};J(i,e=>{H(t).t===`note`?e(a):H(t).t===`warn`?e(o,1):H(t).t===`small`?e(s,2):H(t).t===`strong`?e(c,3):H(t).t===`bullet`?e(l,4):H(t).t===`toggle`?e(u,5):H(t).t===`text`?e(d,6):H(t).t===`diff`?e(f,7):H(t).t===`step`&&e(p,8)}),G(e,r)}),G(e,r),qe()}qr([`click`]);var Qc=W(`<p class="warn"> </p>`),$c=W(`<button class="choice"> </button>`),el=W(`<section class="card"><header><h1> </h1> <span class="named"> </span></header> <div><p class="target"> </p> <!></div> <div class="means"><!></div> <p class="question"> </p> <!> <div class="choices"><button class="choice"> </button> <!> <div class="refuse"><button class="choice"> </button> <input type="text" maxlength="400"/></div> <button class="choice quiet"> </button></div> <p class="small"> </p></section>`);function tl(e,t){Ke(t,!0);let n=_n(``),r=_n(!1);qn(()=>{let e=t.card.readyIn;if(e<=0){P(r,!0);return}let n=setTimeout(()=>P(r,!0),e);return()=>clearTimeout(n)});let i=mt(()=>H(r)&&!t.card.blocked);var a=el(),o=F(a),s=F(o),c=L(s,!0),l=L(R(s,2),!0);D(o);var u=R(o,2);let d;var f=F(u),p=L(f,!0);Zc(R(f,2),{get blocks(){return t.card.content}}),D(u);var m=R(u,2);Zc(F(m),{get blocks(){return t.card.means}}),D(m);var h=R(m,2),g=L(h,!0),_=R(h,2),v=e=>{var n=Qc(),r=L(n,!0);z(()=>q(r,t.card.blockedWhy)),G(e,n)};J(_,e=>{t.card.blockedWhy&&e(v)});var y=R(_,2),b=F(y),x=L(b,!0),S=R(b,2),ee=e=>{var n=$c(),r=L(n,!0);z(()=>{n.disabled=!H(i),q(r,t.card.yesAnd)}),U(`click`,n,()=>$(`allowNotAgain`,{id:t.card.id})),G(e,n)};J(S,e=>{t.card.yesAnd&&e(ee)});var C=R(S,2),te=F(C),ne=L(te,!0),re=R(te,2);ya(re),D(C);var ie=R(C,2),ae=L(ie,!0);D(y);var oe=L(R(y,2),!0);D(a),z(()=>{Q(a,`aria-label`,t.card.kind),q(c,t.card.kind),q(l,t.card.named),d=Z(u,1,`block`,null,d,{long:t.card.long}),q(p,t.card.target),q(g,t.card.question),b.disabled=!H(i),q(x,t.card.yes),q(ne,t.card.no),Q(re,`placeholder`,t.card.reasonHint),Q(re,`aria-label`,t.card.reasonHint),q(ae,t.card.notNow),q(oe,t.card.asker)}),U(`click`,b,()=>$(`allow`,{id:t.card.id})),U(`click`,te,()=>$(`refuse`,{id:t.card.id,reason:H(n)})),Ta(re,()=>H(n),e=>P(n,e)),U(`click`,ie,()=>$(`notNow`)),G(e,a),qe()}qr([`click`]);var nl=W(`<main class="waiting"><p>Waiting for GridCraft…</p></main>`),rl=W(`<p class="error" role="alert"> </p>`),il=W(`<p class="said" role="status"> </p>`),al=W(`<main class="alone"><!> <!> <!></main>`),ol=W(`<span class="sep" aria-hidden="true">›</span>`),sl=W(`<button class="crumb"> </button>`),cl=W(`<span> </span>`),ll=W(`<!> <!>`,1),ul=W(`<p class="above"></p>`),dl=W(`<div class="shell"><aside class="side"><!></aside> <main class="part"><div class="page"><!> <!> <!> <!></div></main></div>`);function fl(e,t){Ke(t,!0);let n=mt(()=>Va.data),r=mt(()=>H(n)?H(n).pane:null);var i=ii(),a=I(i),o=e=>{G(e,nl())},s=e=>{var t=al(),r=F(t);Ji(r,()=>H(n).looking.id,e=>{tl(e,{get card(){return H(n).looking}})});var i=R(r,2),a=e=>{var t=rl(),r=L(t,!0);z(()=>q(r,H(n).error)),G(e,t)};J(i,e=>{H(n).error&&e(a)});var o=R(i,2),s=e=>{var t=il(),r=L(t,!0);z(()=>q(r,H(n).said)),G(e,t)};J(o,e=>{H(n).said&&e(s)}),D(t),G(e,t)},c=e=>{var t=dl(),i=F(t);mo(F(i),{get rows(){return H(n).nav}}),D(i);var a=R(i,2),o=F(a),s=F(o),c=e=>{var t=ul();X(t,21,()=>H(r).above,Y,(e,t,n)=>{var i=ll(),a=I(i),o=e=>{G(e,ol())};J(a,e=>{n>0&&e(o)});var s=R(a,2),c=e=>{var i=sl(),a=L(i,!0);z(()=>q(a,H(t))),U(`click`,i,()=>$(`go`,{to:H(r).aboveGo[n]})),G(e,i)},l=e=>{var n=cl(),r=L(n,!0);z(()=>q(r,H(t))),G(e,n)};J(s,e=>{H(r).aboveGo&&H(r).aboveGo[n]?e(c):e(l,-1)}),G(e,i)}),D(t),G(e,t)};J(s,e=>{H(r).above&&H(r).above.length&&e(c)});var l=R(s,2),u=e=>{Mo(e,{get pane(){return H(r)}})},d=e=>{Yo(e,{get pane(){return H(r)}})},f=e=>{Oc(e,{get pane(){return H(r)}})},p=e=>{Lc(e,{get pane(){return H(r)}})},m=e=>{ys(e,{get pane(){return H(r)}})},h=e=>{var t=ii();Ji(I(t),()=>H(r).id,e=>{uc(e,{get pane(){return H(r)}})}),G(e,t)};J(l,e=>{H(r).kind===`overview`?e(u):H(r).kind===`server`?e(d,1):H(r).kind===`console`?e(f,2):H(r).kind===`macros`?e(p,3):H(r).kind===`kit`?e(m,4):H(r).kind===`project`&&e(h,5)});var g=R(l,2),_=e=>{var t=rl(),r=L(t,!0);z(()=>q(r,H(n).error)),G(e,t)};J(g,e=>{H(n).error&&e(_)});var v=R(g,2),y=e=>{var t=il(),r=L(t,!0);z(()=>q(r,H(n).said)),G(e,t)};J(v,e=>{H(n).said&&e(y)}),D(o),D(a),D(t),G(e,t)};J(a,e=>{H(n)?H(n).looking?e(s,1):e(c,-1):e(o)}),G(e,i),qe()}qr([`click`]);var pl=[`window`,`text`,`dim`,`accent`,`onAccent`,`danger`,`warn`,`border`,`input`,`block`,`hover`,`added`,`removed`];window.gridcraftState=e=>{let t=e&&e.theme;if(t){let e=document.documentElement;for(let n of pl)typeof t[n]==`string`&&e.style.setProperty(`--${n}`,t[n]);e.style.setProperty(`color-scheme`,t.dark?`dark`:`light`)}Ha(e)},Fi(fl,{target:document.getElementById(`app`)}),$(`ready`)})();"#),
    ("app.css", r#":root{--window:#fff;--text:#1f1f1f;--dim:#666;--accent:#0f7b46;--onAccent:#fff;--danger:#c0392b;--warn:#b85c00;--border:#d0d0d0;--input:#fff;--block:#f4f4f4;--hover:#ececec;--added:#0f7b462e;--removed:#c0392b2e;font:14px/1.45 Segoe UI,system-ui,-apple-system,BlinkMacSystemFont,Roboto,Noto Sans,Ubuntu,Cantarell,Helvetica Neue,sans-serif}*{box-sizing:border-box}html,body{background:var(--window);height:100%;color:var(--text);margin:0}#app{height:100%}.shell{grid-template-columns:236px minmax(0,1fr);height:100%;display:grid}.side{background:var(--block);border-right:1px solid var(--border);flex-direction:column;min-height:0;display:flex}.side nav{flex:1;min-height:0;padding:10px 8px;overflow-y:auto}.part{min-width:0;overflow-y:auto}.page{max-width:800px;padding:20px 28px 28px}main.alone{max-width:920px;margin:0 auto;padding:14px 20px 20px}main.waiting{color:var(--dim);text-align:center;padding-top:40px}.navitem{text-align:left;background:0 0;border:none;border-radius:6px;align-items:center;gap:8px;width:100%;margin:1px 0;padding:5px 8px;display:flex}.navitem:hover:not(:disabled){background:var(--hover)}.navitem.selected{background:var(--window);box-shadow:inset 3px 0 0 var(--accent), 0 0 0 1px var(--border)}.navitem.selected .label{font-weight:600}.navitem .words{flex-direction:column;flex:1;min-width:0;display:flex}.navitem .label,.navitem .hint{text-overflow:ellipsis;white-space:nowrap;overflow:hidden}.navitem .hint{color:var(--dim);font-size:11.5px}.navitem.d1>.words:first-child{padding-left:17px}.navitem.d0 .label{font-size:15px;font-weight:600}.navhead{letter-spacing:.06em;text-transform:uppercase;color:var(--dim);margin:14px 0 3px;font-size:11px;font-weight:600}.d0{padding-left:8px}.d1{padding-left:16px}.d2{padding-left:28px}.d3{padding-left:36px}.navsum{color:var(--dim);margin:0 0 5px;font-size:11.5px}input.navfilter{border:1px solid var(--border);background:var(--input);width:100%;color:var(--text);font:inherit;border-radius:12px;margin:2px 0 4px;padding:4px 8px;font-size:12.5px}.secthead{flex-wrap:wrap;align-items:baseline;gap:10px;margin:22px 0 7px;display:flex}.secthead .sect{margin:0}.navheadrow{justify-content:space-between;align-items:center;margin:14px 0 3px;display:flex}.navheadrow .navhead{margin:0}button.navadd{width:24px;height:24px;color:var(--accent);border-radius:12px;padding:0;font-size:16px;font-weight:600;line-height:20px}button.navadd:hover:not(:disabled){border-color:var(--accent)}.navkit{border:1px solid var(--border);background:var(--window);text-align:left;border-radius:12px;align-items:center;gap:9px;width:100%;margin:6px 0;padding:8px 10px;display:flex}.navkit:hover:not(:disabled){background:var(--window);border-color:var(--accent)}.navkit.selected,.navkit.within{border-color:var(--accent);box-shadow:inset 3px 0 0 var(--accent)}.navkit .words{flex-direction:column;flex:1;min-width:0;display:flex}.navkit .label,.navkit .hint{text-overflow:ellipsis;white-space:nowrap;overflow:hidden}.navkit .label{font-weight:600}.navkit .hint{color:var(--dim);font-size:11.5px}.tech{color:#fff;letter-spacing:.09em;text-transform:uppercase;white-space:nowrap;background:#555;border-radius:3px;flex:none;align-items:center;gap:6px;padding:0 9px;font-size:10.5px;font-weight:700;line-height:22px;display:inline-flex}.tech.b-plain{background:#555}.tech.b-python{background:#3670a0}.tech.b-svelte{background:#f1413d}.tech.b-c{background:#00599c}.tech .mark{fill:#fff;flex:none;width:13px;height:13px}.tech.b-python .mark{fill:#ffdd54}.icon{flex:none;width:18px;height:18px}.icon.big{width:24px;height:24px}.icon.tools .handle{fill:none;stroke:currentColor;stroke-width:1.3px;stroke-linejoin:round}.icon.tools .body{fill:#d13b2f}.icon.tools .lid{fill:#a82c23}.icon.tools .latch{fill:#f6e7b8}.icon.shield path{fill:var(--added);stroke:var(--accent);stroke-width:1.3px;stroke-linejoin:round}.badge{background:var(--accent);min-width:19px;color:var(--onAccent);text-align:center;border-radius:10px;flex:none;padding:0 6px;font-size:11.5px;font-weight:600}.dot{border:1.5px solid var(--dim);border-radius:50%;flex:none;width:9px;height:9px}.dot.big{width:11px;height:11px}.dot.trusted,.dot.ok{background:var(--accent);border-color:var(--accent)}.dot.changed{background:var(--warn);border-color:var(--warn)}.dot.problem,.dot.error{background:var(--danger);border-color:var(--danger)}.above{color:var(--dim);flex-wrap:wrap;align-items:center;gap:6px;margin:0 0 4px;font-size:12px;display:flex}button.crumb{color:var(--dim);background:0 0;border:none;padding:0;font-size:12px}button.crumb:hover:not(:disabled){color:var(--accent);background:0 0;text-decoration:underline}.head{margin-bottom:14px}.titled{flex-wrap:wrap;align-items:center;gap:8px;display:flex}.title{border:none;margin:0;padding:0;font-size:21px;font-weight:600}.lede{color:var(--dim);margin:5px 0 0}.sect{letter-spacing:.06em;text-transform:uppercase;color:var(--dim);margin:22px 0 7px;font-size:11.5px;font-weight:600}.box{border:1px solid var(--border);border-radius:8px;margin:8px 0;padding:12px 14px}.box.kit{border-radius:12px}.boxhead{flex-wrap:wrap;align-items:center;gap:8px;margin-bottom:4px;display:flex}.boxtitle{margin:0;font-size:15px;font-weight:600}.chip{border:1px solid var(--border);color:var(--dim);white-space:nowrap;border-radius:10px;padding:0 8px;font-size:11.5px;line-height:18px}.chip.good,.chip.trusted,.chip.rebuilt{border-color:var(--accent);color:var(--accent)}.chip.changed,.chip.partly{border-color:var(--warn);color:var(--warn)}.chip.problem{border-color:var(--danger);color:var(--danger)}.facts{grid-template-columns:1fr 1fr;gap:10px 18px;display:grid}.fact{align-items:center;gap:8px;min-width:0;display:flex}.fact.wide{grid-column:1/-1}.fact .small:first-child{flex:none;width:104px}.fact button{flex:none;margin-left:auto}.grouphead{flex-wrap:wrap;align-items:baseline;gap:10px;margin:14px 0 4px;display:flex}.grouphead h3{margin:0;font-size:14px;font-weight:600}.empty{border:1px dashed var(--border);border-radius:8px;margin:2px 0 0;padding:8px 14px}.list{border:1px solid var(--border);border-radius:8px;overflow:hidden}.entry{background:var(--window);text-align:left;border:none;border-radius:0;align-items:center;gap:10px;width:100%;padding:8px 12px;display:flex}.entry+.entry{border-top:1px solid var(--border)}.entry.still{cursor:default}.entry .words{flex-direction:column;flex:1;gap:2px;min-width:0;display:flex}.entry .line1{flex-wrap:wrap;align-items:center;gap:8px;display:flex}.entry .name{font-weight:600}.entry .hint{color:var(--dim);overflow-wrap:anywhere;font-size:12.5px}.entry .hint.warn{color:var(--warn)}.entry .arrow,.entry .chevron{color:var(--dim)}.standing{color:var(--dim);flex:none;font-size:12.5px}.standing.trusted{color:var(--accent);font-weight:600}.standing.changed{color:var(--warn);font-weight:600}.standing.problem{color:var(--danger);font-weight:600}.tick{cursor:pointer;align-items:center;gap:8px;width:-moz-fit-content;width:fit-content;margin-top:4px;display:flex}.tick input{accent-color:var(--accent);margin:0}input.name{flex:1;min-width:120px;max-width:320px}.parts{grid-template-columns:max-content minmax(0,1fr);gap:5px 14px;margin:0;display:grid}.parts dt,.parts dd{margin:0}.parts dd{overflow-wrap:anywhere}h1{border-bottom:1px solid var(--border);margin:16px 0 6px;padding-bottom:5px;font-size:15px;font-weight:600}h2{margin:14px 0 4px;font-size:13.5px;font-weight:600}p{margin:4px 0}.small{color:var(--dim);font-size:12.5px}.strong{font-weight:600}.warn,.bullet.warn,.small.warn{color:var(--warn);font-weight:600}.bullet{text-indent:-10px;padding-left:14px}.bullet:before{content:"•  "}code,.code,.target{font-family:Cascadia Mono,Consolas,SF Mono,Menlo,monospace;font-size:12.5px}button{font:inherit;color:var(--text);background:var(--input);border:1px solid var(--border);cursor:pointer;white-space:nowrap;border-radius:5px;padding:4px 10px}button:hover:not(:disabled){background:var(--hover)}button:focus-visible{outline:2px solid var(--accent);outline-offset:1px}input:focus-visible{outline:2px solid var(--accent);outline-offset:1px}button:disabled{color:var(--dim);cursor:default;opacity:.65}button.primary{background:var(--accent);border-color:var(--accent);color:var(--onAccent);font-weight:600}button.primary:hover:not(:disabled){background:var(--accent);filter:brightness(1.08)}button.link{color:var(--accent);background:0 0;border:none;padding:0 4px;text-decoration:underline}input[type=text]{font:inherit;color:var(--text);background:var(--input);border:1px solid var(--border);border-radius:5px;padding:4px 8px}.row{flex-wrap:wrap;align-items:center;gap:8px;margin:6px 0;display:flex}input.name.immediate{max-width:none;font-family:Consolas,Cascadia Mono,ui-monospace,monospace}.log{flex-direction:column;gap:6px;margin:8px 0;display:flex}.logentry{border:1px solid var(--border);border-left-width:3px;border-radius:6px;padding:6px 10px}.logentry.bad{border-left-color:var(--danger)}.logentry.agent{border-left-color:var(--accent)}.logentry p{margin:0 0 2px}.logentry pre{white-space:pre-wrap;overflow-wrap:anywhere;margin:0;font-size:12.5px}select{font:inherit;color:var(--text);background:var(--input);border:1px solid var(--border);border-radius:5px;padding:3px 6px}.twocol{grid-template-columns:190px minmax(0,1fr);gap:14px;margin:10px 0;display:grid}.filelist.column{flex-direction:column;align-items:stretch;gap:3px;display:flex}.filelist.column .fileitem{text-align:left}.reading p{margin:0 0 4px}.trustbox p{margin:0 0 6px}.output{background:var(--block);white-space:pre-wrap;overflow-wrap:anywhere;border-radius:6px;max-height:16em;margin:6px 0;padding:8px 10px;font:12px ui-monospace,Consolas,monospace;overflow:auto}.grow{flex:1}.port{width:76px}.path{text-overflow:ellipsis;white-space:nowrap;min-width:0;color:var(--dim);overflow:hidden}.levels{flex-direction:column;gap:4px;margin:2px 0 6px;display:flex}.level{cursor:pointer;align-items:center;gap:8px;width:-moz-fit-content;width:fit-content;display:flex}.level input{accent-color:var(--accent);margin:0}.status{font-weight:600}.status.ok{color:var(--accent)}.status.error{color:var(--danger)}.status.none{color:var(--dim);font-weight:400}.item{border:1px solid var(--border);border-radius:8px;flex-wrap:wrap;align-items:center;gap:8px;margin:6px 0;padding:8px 12px;display:flex}.item.waits{border-color:var(--accent);box-shadow:inset 3px 0 0 var(--accent)}.item.plan{display:block}.item .head{flex-wrap:wrap;align-items:center;gap:8px;margin:0;display:flex}.item ol{margin:6px 0 2px;padding-left:2px;list-style:none}.item li .state{margin-left:8px}.item li .state:before{content:"·  "}.item li.done{color:var(--text)}.item li.failed .state{color:var(--danger);font-weight:600}.item li.running .state{color:var(--accent);font-weight:600}.card header{align-items:baseline;gap:12px;margin-bottom:8px;display:flex}.card header h1{border:none;flex:1;margin:0;padding:0;font-size:17px}.named{color:var(--dim);font-size:12.5px}.block{background:var(--block);border:1px solid var(--border);border-radius:6px;margin-bottom:8px;padding:8px 12px}.block.long{min-height:160px;max-height:calc(100vh - 330px);overflow-y:auto}.block.long .code{max-height:none}.target{overflow-wrap:anywhere;margin-bottom:6px;font-weight:600}.code{background:var(--window);border:1px solid var(--border);border-radius:4px;max-height:300px;margin:6px 0;padding:6px 0;line-height:1.4;overflow:auto}.line{white-space:pre;width:max-content;min-width:100%;display:flex}.line .no{text-align:right;width:46px;color:var(--dim);-webkit-user-select:none;user-select:none;flex:none;padding-right:8px}.line .mark{-webkit-user-select:none;user-select:none;flex:none;width:18px}.line .src{padding-right:12px}.line.gone{background:var(--removed)}.line.gone .mark{color:var(--danger)}.line.new{background:var(--added)}.line.new .mark{color:var(--accent)}.line.skip{color:var(--dim)}.step{border-top:1px solid var(--border);margin-top:10px;padding-top:8px}.step h2{margin:0 0 4px;font-size:13.5px}.step h2 code{overflow-wrap:anywhere;margin-left:6px;font-weight:400}.toggle{align-items:center;gap:8px;display:flex}.question{margin:12px 0 6px;font-weight:600}.choices{flex-direction:column;align-items:flex-start;gap:6px;margin-bottom:8px;display:flex}.choice{text-align:left;white-space:normal;max-width:100%;padding:5px 12px}.choice.quiet{color:var(--dim)}.refuse{gap:8px;width:100%;display:flex}.refuse input{flex:1;min-width:0}.said,.error{border:1px solid var(--border);border-radius:6px;margin-top:16px;padding:7px 11px;font-size:13px}.error{border-color:var(--danger);color:var(--danger);font-weight:600}.entry{white-space:normal}.filelist{flex-wrap:wrap;gap:4px 6px;max-height:7.5em;margin:4px 0 8px;display:flex;overflow:auto}button.fileitem{background:0 0;padding:1px 7px;font-family:Cascadia Mono,Consolas,SF Mono,Menlo,monospace;font-size:12px}button.fileitem.chosen{border-color:var(--accent);color:var(--accent);font-weight:600}.fileview{background:var(--block);border:1px solid var(--border);white-space:pre;-moz-tab-size:4;tab-size:4;border-radius:5px;max-height:26em;margin:6px 0;padding:8px 10px;font-family:Cascadia Mono,Consolas,SF Mono,Menlo,monospace;font-size:12.5px;line-height:1.45;overflow:auto}.fileview .ln{text-align:right;min-width:2.6em;color:var(--dim);opacity:.7;-webkit-user-select:none;user-select:none;margin-right:1.1em;display:inline-block}.tk-comment{color:var(--dim);font-style:italic}.tk-string{color:var(--warn)}.tk-number{color:var(--danger)}.tk-keyword{color:var(--accent);font-weight:600}.tk-name{font-weight:600}
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
  (`Server.svelte`), the Console (`Console.svelte`, with the immediate line over it), the open workbook's macros
  (`Macros.svelte`), a development kit (`Kit.svelte`) or one of its projects
  (`Project.svelte`, where a project is also trusted and its scripts run). One request's card takes the place of all of it (`Review.svelte`, and
  `Blocks.svelte` for what a card holds). `marks.js` turns what GridCraft says of how
  something stands into one of a fixed set of class names. `Icon.svelte` draws the page's
  two pictures, a toolbox and a shield, as shapes: no picture is a file. What a kit is made
  of is drawn as a badge (`Badge.svelte`): its name on a colour, with its mark. The marks are
  other people's (`logos.js`, from Simple Icons): `THIRD-PARTY.md` says whose, and that they
  are here as the fork's exception to the rule against anyone else's iconography.

## Rules for whoever changes it

- **Everything a request holds is drawn as text.** No `{@html ...}`, no `innerHTML`, no
  attribute built from what GridCraft sent other than `class` from a fixed set of names.
- **The page is served under a policy that allows its own script and stylesheet and nothing
  else:** no inline script, no inline style, no `eval`, nothing from the network. Set styles
  through CSS classes, or through the style object from script.
- **A message is sent only in answer to a click.** GridCraft takes a message from this page
  as the person's own doing. What is typed to filter the kits (`filter.svelte.js`) stays in
  the page: nothing is sent for it.

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

# The marks in the badges

`src/logos.js` holds three shapes that are other people's marks, each drawn in a badge beside
its name to say what a development kit is made of. They are built into `app.js`.

| Mark | Whose it is | The shape is from |
|---|---|---|
| Python | The Python Software Foundation's. "Python" and the Python logos are its trademarks. | Simple Icons 16.35.0, `icons/python.svg` |
| Svelte | The Svelte project's. | Simple Icons 16.35.0, `icons/svelte.svg` |
| C | Nobody's in particular: the letter in its hexagon, as the language is commonly marked. | Simple Icons 16.35.0, `icons/c.svg` |

Simple Icons (https://simpleicons.org, https://github.com/simple-icons/simple-icons) is
released under CC0 1.0 Universal. That licence covers the drawings; it gives no right to the
marks themselves, which stay their owners'. Each is used here unchanged, in one colour, only to
name the thing it stands for. GridCraft is not affiliated with, or endorsed by, the Python
Software Foundation or the Svelte project.

These are in the fork (arcanii/DevCraft) by its owner's decision of 2026-10-13, as an
exception to upstream's rule that the repository holds no other party's iconography. They are
not to be offered upstream.
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

/* A row of the list with no mark before its words keeps them in line with the rows that have one. */
.navitem.d1 > .words:first-child {
  padding-left: 17px;
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

/* Over the kits: how many they are and the space they use, and the field they are filtered by. */
.navsum {
  margin: 0 0 5px;
  font-size: 11.5px;
  color: var(--dim);
}

input.navfilter {
  width: 100%;
  margin: 2px 0 4px;
  padding: 4px 8px;
  border: 1px solid var(--border);
  border-radius: 12px;
  background: var(--input);
  color: var(--text);
  font: inherit;
  font-size: 12.5px;
}

.secthead {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 10px;
  margin: 22px 0 7px;
}

.secthead .sect {
  margin: 0;
}

/* The heading over the kits, with the way to load another beside it. */
.navheadrow {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin: 14px 0 3px;
}

.navheadrow .navhead {
  margin: 0;
}

button.navadd {
  width: 24px;
  height: 24px;
  padding: 0;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 600;
  line-height: 20px;
  color: var(--accent);
}

button.navadd:hover:not(:disabled) {
  border-color: var(--accent);
}

/* A development kit: set apart from the rest of the list, in a frame of its own. */
.navkit {
  display: flex;
  align-items: center;
  gap: 9px;
  width: 100%;
  margin: 6px 0;
  padding: 8px 10px;
  border: 1px solid var(--border);
  border-radius: 12px;
  background: var(--window);
  text-align: left;
}

.navkit:hover:not(:disabled) {
  background: var(--window);
  border-color: var(--accent);
}

.navkit.selected,
.navkit.within {
  border-color: var(--accent);
  box-shadow: inset 3px 0 0 var(--accent);
}

.navkit .words {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-width: 0;
}

.navkit .label,
.navkit .hint {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.navkit .label {
  font-weight: 600;
}

.navkit .hint {
  font-size: 11.5px;
  color: var(--dim);
}

/* What a kit is made of, as a badge: its mark and its name in capitals, on the colour it is
   known by. Drawn here, from shapes this page holds (logos.js): nothing is fetched. */
.tech {
  display: inline-flex;
  flex: none;
  align-items: center;
  gap: 6px;
  padding: 0 9px;
  border-radius: 3px;
  background: #555555;
  color: #ffffff;
  font-size: 10.5px;
  font-weight: 700;
  letter-spacing: 0.09em;
  line-height: 22px;
  text-transform: uppercase;
  white-space: nowrap;
}

.tech.b-plain {
  background: #555555;
}

.tech.b-python {
  background: #3670a0;
}

.tech.b-svelte {
  background: #f1413d;
}

.tech.b-c {
  background: #00599c;
}

.tech .mark {
  flex: none;
  width: 13px;
  height: 13px;
  fill: #ffffff;
}

.tech.b-python .mark {
  fill: #ffdd54;
}

/* The page's two pictures. */
.icon {
  flex: none;
  width: 18px;
  height: 18px;
}

.icon.big {
  width: 24px;
  height: 24px;
}

.icon.tools .handle {
  fill: none;
  stroke: currentColor;
  stroke-width: 1.3;
  stroke-linejoin: round;
}

.icon.tools .body {
  fill: #d13b2f;
}

.icon.tools .lid {
  fill: #a82c23;
}

.icon.tools .latch {
  fill: #f6e7b8;
}

.icon.shield path {
  fill: var(--added);
  stroke: var(--accent);
  stroke-width: 1.3;
  stroke-linejoin: round;
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
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 6px;
  margin: 0 0 4px;
  font-size: 12px;
  color: var(--dim);
}

button.crumb {
  padding: 0;
  border: none;
  background: none;
  color: var(--dim);
  font-size: 12px;
}

button.crumb:hover:not(:disabled) {
  background: none;
  color: var(--accent);
  text-decoration: underline;
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

.box.kit {
  border-radius: 12px;
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

/* The immediate line: a call is typed as code is. */
input.name.immediate {
  max-width: none;
  font-family: Consolas, "Cascadia Mono", ui-monospace, monospace;
}

/* The Console's entries, and a module beside the list of modules. */
.log {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin: 8px 0;
}

.logentry {
  padding: 6px 10px;
  border: 1px solid var(--border);
  border-left-width: 3px;
  border-radius: 6px;
}

.logentry.bad {
  border-left-color: var(--danger);
}

.logentry.agent {
  border-left-color: var(--accent);
}

.logentry p {
  margin: 0 0 2px;
}

.logentry pre {
  margin: 0;
  font-size: 12.5px;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}

select {
  font: inherit;
  color: var(--text);
  background: var(--input);
  border: 1px solid var(--border);
  border-radius: 5px;
  padding: 3px 6px;
}

.twocol {
  display: grid;
  grid-template-columns: 190px minmax(0, 1fr);
  gap: 14px;
  margin: 10px 0;
}

.filelist.column {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 3px;
}

.filelist.column .fileitem {
  text-align: left;
}

.reading p {
  margin: 0 0 4px;
}

.trustbox p {
  margin: 0 0 6px;
}

/* What a compiler said of a project that did not build. */
.output {
  max-height: 16em;
  overflow: auto;
  margin: 6px 0;
  padding: 8px 10px;
  border-radius: 6px;
  background: var(--block);
  font: 12px ui-monospace, Consolas, monospace;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
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

/* A project's files, and the one that is being read. */
.filelist {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 6px;
  max-height: 7.5em;
  overflow: auto;
  margin: 4px 0 8px;
}

button.fileitem {
  font-family: "Cascadia Mono", Consolas, "SF Mono", Menlo, monospace;
  font-size: 12px;
  padding: 1px 7px;
  background: transparent;
}

button.fileitem.chosen {
  border-color: var(--accent);
  color: var(--accent);
  font-weight: 600;
}

.fileview {
  font-family: "Cascadia Mono", Consolas, "SF Mono", Menlo, monospace;
  font-size: 12.5px;
  line-height: 1.45;
  background: var(--block);
  border: 1px solid var(--border);
  border-radius: 5px;
  padding: 8px 10px;
  margin: 6px 0;
  max-height: 26em;
  overflow: auto;
  white-space: pre;
  tab-size: 4;
}

.fileview .ln {
  display: inline-block;
  min-width: 2.6em;
  margin-right: 1.1em;
  text-align: right;
  color: var(--dim);
  opacity: 0.7;
  user-select: none;
}

.tk-comment {
  color: var(--dim);
  font-style: italic;
}

.tk-string {
  color: var(--warn);
}

.tk-number {
  color: var(--danger);
}

.tk-keyword {
  color: var(--accent);
  font-weight: 600;
}

.tk-name {
  font-weight: 600;
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

// What a piece of a file's line is, as a class name: plain when GridCraft says nothing of it.
const PIECES = { comment: "tk-comment", string: "tk-string", number: "tk-number", keyword: "tk-keyword", name: "tk-name" };

export function piece(code) {
  return (typeof code === "string" && Object.hasOwn(PIECES, code) && PIECES[code]) || "tk-plain";
}

// Which badge something a kit is made of carries, as a class name: plain when GridCraft
// names none this page has a colour for.
const BADGES = { python: "b-python", svelte: "b-svelte", c: "b-c" };

export function badge(code) {
  return (typeof code === "string" && Object.hasOwn(BADGES, code) && BADGES[code]) || "b-plain";
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
  import Console from "./Console.svelte";
  import Macros from "./Macros.svelte";
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
    </aside>
    <main class="part">
      <div class="page">
        {#if pane.above && pane.above.length}
          <!-- Where this part is, each place a way back to it. -->
          <p class="above">
            {#each pane.above as place, n}
              {#if n > 0}<span class="sep" aria-hidden="true">›</span>{/if}
              {#if pane.aboveGo && pane.aboveGo[n]}
                <button class="crumb" onclick={() => send("go", { to: pane.aboveGo[n] })}>{place}</button>
              {:else}
                <span>{place}</span>
              {/if}
            {/each}
          </p>
        {/if}
        {#if pane.kind === "overview"}
          <Overview {pane} />
        {:else if pane.kind === "server"}
          <Server {pane} />
        {:else if pane.kind === "console"}
          <Console {pane} />
        {:else if pane.kind === "macros"}
          <Macros {pane} />
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
  // The list down the window's side, as GridCraft lists it: Development at the top; under it
  // the MCP server, and the development kits, each set apart in a frame of its own with its
  // shield. Over the kits: how many they are and the space they use, and a field to filter
  // them by. A kit's projects are not listed here: they are in the kit's own part.
  import { send } from "./host.js";
  import { mark, depth } from "./marks.js";
  import { filter, shown } from "./filter.svelte.js";
  import Icon from "./Icon.svelte";

  let { rows } = $props();

  let kits = $derived(rows.filter((row) => row.t === "kit"));
  let none = $derived(kits.length > 0 && !kits.some(shown));
</script>

<nav aria-label="The parts of the Development window">
  {#each rows as row}
    {#if row.t === "heading"}
      {#if row.add}
        <!-- The way to load another kit: GridCraft opens the operating system's own file
             dialog for its package. The page names no file. -->
        <div class="navheadrow {depth(row.depth)}">
          <p class="navhead">{row.label}</p>
          <button class="navadd" title={row.add} aria-label={row.add} onclick={() => send("loadKit")}>+</button>
        </div>
      {:else}
        <p class="navhead {depth(row.depth)}">{row.label}</p>
      {/if}
      {#if row.says}<p class="navsum {depth(row.depth)}">{row.says}</p>{/if}
      {#if row.filter}
        <input
          class="navfilter"
          type="search"
          maxlength="80"
          bind:value={filter.typed}
          placeholder={row.filter}
          aria-label={row.filter}
          title="Shows the kits that say every word typed here, of themselves or of their projects"
        />
        {#if none}<p class="navsum {depth(row.depth)}">{row.none}</p>{/if}
      {/if}
    {:else if row.t === "kit"}
      {#if shown(row)}
        <!-- A kit whose project is shown is where the person is, too: its row is the way back. -->
        <button
          class="navkit"
          class:selected={row.selected}
          class:within={row.within}
          aria-current={row.selected ? "page" : undefined}
          title="Its projects, and what it needs to run them"
          onclick={() => send("go", { to: row.go })}
        >
          <Icon name={row.icon} />
          <span class="words">
            <span class="label">{row.label}</span>
            <span class="hint">{row.hint}{row.projects ? "  ·  " + row.projects : ""}</span>
          </span>
        </button>
      {/if}
    {:else if row.t === "item"}
      <button
        class="navitem {depth(row.depth)}"
        class:selected={row.selected}
        aria-current={row.selected ? "page" : undefined}
        onclick={() => send("go", { to: row.go })}
      >
        {#if row.mark}<span class="dot {mark(row.mark)}" aria-hidden="true"></span>{/if}
        {#if row.icon}<Icon name={row.icon} />{/if}
        <span class="words">
          <span class="label">{row.label}</span>
          {#if row.hint}<span class="hint">{row.hint}</span>{/if}
        </span>
        {#if row.badge}<span class="badge" title="Waiting for your answer">{row.badge}</span>{/if}
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
  import { shown } from "./filter.svelte.js";
  import Icon from "./Icon.svelte";
  import Badge from "./Badge.svelte";

  let { pane } = $props();

  let server = $derived(pane.server);
  let workbook = $derived(pane.workbook);
  // The kits that pass what is typed in the list's filter: the same ones the list shows.
  let kits = $derived(pane.kits.filter(shown));
</script>

<header class="head">
  <div class="titled">
    <Icon name="tools" big />
    <h1 class="title">{pane.title}</h1>
  </div>
  {#if pane.version}<p class="small version">{pane.version}</p>{/if}
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

<div class="secthead">
  <h2 class="sect">{pane.kitsTitle}</h2>
  <span class="small">{pane.kitsSummary}</span>
</div>
{#if pane.kits.length > 0 && kits.length === 0}<p class="small empty">{pane.kitsNone}</p>{/if}
{#each kits as kit}
  <section class="box kit" aria-label={kit.name}>
    <div class="boxhead">
      <Icon name="shield" />
      <h2 class="boxtitle">{kit.name}</h2>
      {#each kit.badges as part}<Badge {part} />{/each}
      <span class="chip good">{kit.standing}</span>
      <span class="grow"></span>
      <button onclick={() => send("go", { to: kit.go })} aria-label="Open {kit.name}" title="Its projects, and what it needs to run them">Open</button>
    </div>
    <p>{kit.says}</p>
    {#if kit.notYet}<p class="small warn">{kit.notYet}</p>{/if}
    {#if kit.projects}<p class="small">{kit.projects}{kit.space ? "  ·  " + kit.space : ""}</p>{/if}
  </section>
{/each}
<p class="small">{pane.kitsNote}</p>

<h2 class="sect">Open workbook</h2>
<section class="box" aria-label="Open workbook">
  <div class="boxhead">
    {#if workbook.title}<h2 class="boxtitle">{workbook.title}</h2>{/if}
    <span class="grow"></span>
    {#if workbook.macros}
      <button onclick={() => send("go", { to: "macros" })} title="Read the macros this workbook carries. GridCraft does not run them.">Read Its Macros</button>
    {/if}
  </div>
  <p class:warn={workbook.problem}>{workbook.says}</p>
  {#if workbook.rebuilt}<p class="small">{workbook.rebuilt}</p>{/if}
</section>

<div class="row">
  <button onclick={() => send("go", { to: "console" })} title="What your projects print and log, what failed, and what agents asked for">Console</button>
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
        <button class="link" onclick={() => send("askAgain", { project: rule.project, kind: rule.kind })}>Ask again</button>
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
  // A development kit's part: what it is, what it can't do yet, what it needs to run, its
  // projects (one list: a project that is one workbook's code says so beside its name), a
  // new project, and (for the kit that has one) its showcase. GridCraft says which of these
  // a kit has.
  import { send } from "./host.js";
  import { mark } from "./marks.js";
  import Icon from "./Icon.svelte";
  import Badge from "./Badge.svelte";

  let { pane } = $props();

  // The name of a new project is the person's to type; whether it is the open workbook's
  // code is theirs to tick. Which workbook that is, GridCraft says.
  let name = $state("");
  let forWorkbook = $state(false);

  // The compiler that builds a kit's projects is the person's to name: what is typed here is
  // sent when they ask for it to be checked. It starts as what GridCraft has kept.
  let compiler = $state(null);
  let named = $derived(compiler ?? pane.compiler?.named ?? "");

  // So is the Python that runs a kit's scripts.
  let python = $state(null);
  let pythonNamed = $derived(python ?? pane.pythonField?.named ?? "");

  function checkPython(event) {
    event.preventDefault();
    send("checkPython", { python: pythonNamed });
  }

  function check(event) {
    event.preventDefault();
    send("checkCompiler", { compiler: named });
  }

  function create(event) {
    event.preventDefault();
    // The kit is the one whose part this is: GridCraft makes none for a kit it doesn't have.
    send("createProject", { name, forWorkbook: forWorkbook && !!pane.newFor, kit: pane.id });
    name = "";
    forWorkbook = false;
  }
</script>

<header class="head">
  <div class="titled">
    <Icon name="shield" big />
    <h1 class="title">{pane.title}</h1>
    {#each pane.badges as part}<Badge {part} />{/each}
    <span class="chip good">{pane.standing}</span>
  </div>
  <p class="lede">{pane.says}</p>
  {#if pane.notYet}<p class="warn">{pane.notYet}</p>{/if}
</header>

{#if !pane.available}
  <p>{pane.unavailable}</p>
{:else}
  {#if pane.package}
    <!-- A kit loaded from a package: whose it is, what it stands on, and the way to remove it. -->
    <section class="box" aria-label={pane.package.title}>
      <p>{pane.package.says}</p>
      <p class="small">
        {#if pane.package.version}Version {pane.package.version}.{/if}
        {#if pane.package.author} By {pane.package.author}.{/if}
      </p>
      <div class="row">
        <button onclick={() => send("go", { to: pane.package.baseGo })} title="The kit its projects run by">{pane.package.base}</button>
        <span class="grow"></span>
        <span class="small">{pane.package.removeSays}</span>
        <button onclick={() => send("removeKit", { id: pane.id })} title="Your operating system will ask you to confirm">Remove Kit</button>
      </div>
    </section>
  {/if}
  <section class="box facts">
    <div class="fact">
      <span class="small">Projects</span>
      <span>{pane.projectsSays}</span>
    </div>
    <div class="fact">
      <span class="small">Console</span>
      <span>{pane.console}</span>
      <button onclick={() => send("go", { to: "console" })} title="What your projects print and log, what failed, and what agents asked for">Open</button>
    </div>
    {#if pane.pythonField}
      <form class="fact wide" onsubmit={checkPython}>
        <label class="small" for="python">{pane.pythonField.title}</label>
        <input id="python" class="name" type="text" maxlength="400" value={pythonNamed} oninput={(event) => (python = event.target.value)} placeholder={pane.pythonField.hint} title={pane.pythonField.says} />
        <button type="submit" title="Keeps the name, and asks that Python what it is">Check Python</button>
      </form>
      {#if pane.pythonField.status}<p class="small fact wide" role="status">{pane.pythonField.status}</p>{/if}
    {:else if pane.python}
      <div class="fact wide">
        <span class="small">Runs with</span>
        <span>{pane.python}</span>
      </div>
    {/if}
    {#if pane.compiler}
      <form class="fact wide" onsubmit={check}>
        <label class="small" for="compiler">{pane.compiler.title}</label>
        <input id="compiler" class="name" type="text" maxlength="400" value={named} oninput={(event) => (compiler = event.target.value)} placeholder={pane.compiler.says} title={pane.compiler.hint} />
        {#if pane.compiler.canPick}
          <!-- GridCraft opens the operating system's own file dialog: the page names no file.
               What was typed here gives way to what is chosen there. -->
          <button
            type="button"
            title="Opens a file dialog to choose the zig program"
            onclick={() => {
              compiler = null;
              send("pickCompiler");
            }}>Browse…</button
          >
        {/if}
        <button type="submit" title="Keeps the name, and asks the compiler what it is">Check Compiler</button>
      </form>
    {/if}
    {#if pane.space}
      <div class="fact wide" title={pane.space.counts}>
        <span class="small">{pane.space.title}</span>
        <span>{pane.space.says}</span>
        <span class="small">{pane.space.counts}</span>
      </div>
    {/if}
    <div class="fact wide">
      <span class="small">Projects folder</span>
      <code class="path" title={pane.folder}>{pane.folder}</code>
      {#if pane.canOpenFolder}<button onclick={() => send("openFolder")}>Open Folder</button>{/if}
    </div>
  </section>

  <h2 class="sect">{pane.projectsTitle}</h2>
  <p class="small">{pane.projectsNote}</p>
  {#if pane.projects.length === 0}
    <p class="small empty">{pane.projectsEmpty}</p>
  {:else}
    <div class="list">
      {#each pane.projects as project}
        <button class="entry" disabled={!project.go} onclick={() => send("go", { to: project.go })} title="What it is for, what it asks, and what is in it">
          <span class="dot {mark(project.status)}" aria-hidden="true"></span>
          <span class="words">
            <span class="line1">
              <span class="name">{project.name}</span>
              {#if project.for}<span class="chip">{project.for}</span>{/if}
            </span>
            {#if project.says}<span class="hint">{project.says}</span>{/if}
          </span>
          <span class="standing {mark(project.status)}">{project.statusLabel}</span>
          <span class="chevron" aria-hidden="true">›</span>
        </button>
      {/each}
    </div>
  {/if}

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

  {#if pane.showcase}
    <h2 class="sect">Showcase</h2>
    <div class="row">
      <span>{pane.showcase.text}</span>
      {#if pane.showcase.stands === "none"}
        <button onclick={() => send("createShowcase")} title="Creates this kit's showcase project. It does nothing until you trust it.">Create Showcase</button>
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
{/if}
"#),
    ("src/Project.svelte", r#"<script>
  // One project: what it is for, where it stands, what it asks to be allowed, what is in it,
  // and what it says it rebuilds of its workbook's macros. Its name, its description and its
  // notes are its author's words: they are drawn as text, like everything else.
  import { send } from "./host.js";
  // (`state` is taken in under another name: beside the rune `$state` it would be read as a store.)
  import { mark, piece, state as became } from "./marks.js";

  let { pane } = $props();

  // Whether trust is to follow the project's files as they change: the person's to tick. It
  // is sent with the click that asks for trust, and kept nowhere else.
  let follow = $state(false);
  let trust = $derived(pane.trust);
  let run = $derived(pane.run);
</script>

<header class="head">
  <div class="titled">
    <span class="dot big {mark(pane.status)}" aria-hidden="true"></span>
    <h1 class="title">{pane.title}</h1>
    {#if pane.version}<span class="small">{pane.version}</span>{/if}
    {#if pane.author}<span class="small">by {pane.author}</span>{/if}
    <span class="grow"></span>
    <span class="chip">{pane.kit}</span>
    <span class="chip">{pane.kindLabel}</span>
    <span class="chip standing {mark(pane.status)}">{pane.statusLabel}</span>
  </div>
  {#if pane.description}<p class="lede">{pane.description}</p>{/if}
</header>

<p>{pane.forSays}</p>
<p class="small">{pane.statusSays}</p>
{#if pane.notYet}<p class="warn">{pane.notYet}</p>{/if}
{#if pane.problem}<p class="error" role="alert">This project can't be used: {pane.problem}.</p>{/if}

<h2 class="sect">{pane.asksTitle}</h2>
<section class="box">
  {#each pane.asks as ask}
    <p class="bullet" class:warn={ask.warn}>{ask.text}</p>
  {:else}
    <p class="bullet">{pane.asksNothing}</p>
  {/each}
</section>

{#if pane.network}
  <!-- The network a project would reach. It is the person's to turn on, here and by itself:
       trusting the project turns nothing on. The page says which project, never where to. -->
  <h2 class="sect">{pane.network.title}</h2>
  <p class="small">{pane.network.says}</p>
  {#each pane.network.addresses as address}
    <p class="bullet"><code>{address.text}</code>{address.note ? "  ·  " + address.note : ""}</p>
  {/each}
  {#if pane.network.listen}<p class="bullet">{pane.network.listen}</p>{/if}
  {#each pane.network.warnings as warning}
    <p class="bullet warn">{warning}</p>
  {/each}
  <div class="row">
    <span class="chip" class:good={pane.network.on}>{pane.network.on ? "On" : "Off"}</span>
    <span>{pane.network.state}</span>
    <span class="grow"></span>
    {#if pane.network.on}
      <button onclick={() => send("network", { id: pane.id, on: false })}>Turn Network Off</button>
    {:else}
      <button disabled={!pane.network.canTurnOn} title="Asks you once more, in a question of your operating system's own" onclick={() => send("network", { id: pane.id, on: true })}>Turn Network On…</button>
    {/if}
  </div>
  <p class="small">{pane.network.carried}</p>
{/if}

<h2 class="sect">What is in it</h2>
<dl class="parts">
  {#each pane.parts as part}
    <dt class="small">{part.label}</dt>
    <dd><code>{part.value}</code></dd>
  {/each}
</dl>

{#if pane.files}
  <h2 class="sect">{pane.files.title}</h2>
  <p class="small">{pane.files.says}</p>
  <div class="filelist">
    {#each pane.files.list as path}
      <button class="fileitem" class:chosen={pane.view && pane.view.path === path} onclick={() => send("viewFile", { id: pane.id, path })}>{path}</button>
    {/each}
    {#if pane.files.more}<span class="small">{pane.files.more}</span>{/if}
  </div>
{/if}

{#if pane.view}
  <!-- A file of the project's, as text. Each piece is drawn as text, in a class from a fixed
       set that says what it is: nothing in a file is followed, loaded or run. -->
  <div class="row">
    <strong>{pane.view.says}</strong>
    <button onclick={() => send("closeFile")}>Close File</button>
  </div>
  {#if pane.view.problem}
    <p class="warn">{pane.view.problem}</p>
  {:else}
    <pre class="fileview">{#each pane.view.lines as line, n}<span class="ln">{n + 1}</span>{#each line as part}<span class={piece(part[1])}>{part[0]}</span>{/each}{"\n"}{/each}</pre>
    {#if pane.view.more}<p class="small">{pane.view.more}</p>{/if}
  {/if}
{/if}

{#if pane.build}
  <h2 class="sect">{pane.build.title}</h2>
  <p class="small">{pane.build.builds}</p>
  <div class="row">
    <button class="primary" disabled={!pane.build.can} onclick={() => send("build", { id: pane.id })} title="Runs your C compiler over this project's files, and keeps the program beside your settings">Build</button>
    <span class:warn={pane.build.state === "failed" || pane.build.state === "stale"} role="status">{pane.build.says}</span>
  </div>
  {#if pane.build.needsTrust}<p class="small">{pane.build.needsTrust}</p>{/if}
  {#if pane.build.output}<pre class="output">{pane.build.output}</pre>{/if}
{/if}

{#if pane.panel}
  <h2 class="sect">{pane.panel.title}</h2>
  <p class="small">{pane.panel.says}</p>
  {#if pane.panel.canDock || pane.panel.canBrowse || pane.panel.docked}
    <div class="row">
      {#if pane.panel.canDock}
        <button onclick={() => send("showPanel", { id: pane.id, docked: true })} title="Shows its panel inside GridCraft's window, beside the sheet">Dock Panel</button>
      {/if}
      {#if pane.panel.canBrowse}
        <button onclick={() => send("showPanel", { id: pane.id, docked: false })} title="Opens its panel in your browser">Open in Browser</button>
      {/if}
      {#if pane.panel.docked}<span class="small" role="status">It is docked in the window.</span>{/if}
    </div>
  {/if}
  {#if pane.panel.needsTrust}<p class="small">{pane.panel.needsTrust}</p>{/if}
{/if}

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
          <span class="chip state {became(row.state)}">{row.stateLabel}</span>
        </div>
      {/each}
    </div>
  {/if}
{/if}

{#if pane.scripts && pane.scripts.length}
  <h2 class="sect">Run</h2>
  <p class="small">Runs one of its scripts on the cells that are selected.</p>
  <div class="row">
    {#each pane.scripts as script}
      <button disabled={run && run.state === "running"} onclick={() => send("run", { id: pane.id, script })} title="Runs this script with the selection as its data">{script}</button>
    {/each}
  </div>
{/if}
{#if run}
  <!-- What a script gave back and printed is the project's own text: it is drawn as text. -->
  <section class="box" aria-label="What the script gave back">
    <div class="boxhead">
      <strong>{run.script}</strong>
      {#if run.state === "running"}
        <span class="small" role="status">Running…</span>
        <span class="grow"></span>
        <button onclick={() => send("stopRun", { id: pane.id })}>Stop</button>
      {:else}
        <span class="chip" class:good={run.ok} class:problem={!run.ok}>{run.ok ? "Finished" : "Failed"}</span>
      {/if}
    </div>
    {#if run.text}<pre class="output" class:warn={run.state !== "running" && !run.ok}>{run.text}</pre>{/if}
    {#if run.printed}<p class="small">It printed:</p><pre class="output">{run.printed}</pre>{/if}
  </section>
{/if}

{#if trust}
  <h2 class="sect">{trust.title}</h2>
  <section class="box trustbox">
    <p>{pane.statusSays}</p>
    {#if trust.moved}
      <p class="warn" role="alert">{trust.moved}</p>
      <div class="row">
        <button onclick={() => send("lookAgain", { id: pane.id })} title="Shows it as it is now, to decide on that">Look Again</button>
      </div>
    {:else if trust.open}
      {#if trust.cant}<p class="warn">{trust.cant}</p>{/if}
      <label class="tick">
        <input type="checkbox" bind:checked={follow} disabled={!trust.canTrust} />
        <span>{trust.follow}</span>
      </label>
      <div class="row">
        <button class="primary" disabled={!trust.canTrust} onclick={() => send("trust", { id: pane.id, follow })} title="Trusts it as you see it here: what it asks, and the files it holds now">{trust.trustLabel}</button>
        <span class="small">{trust.confirms}</span>
      </div>
    {/if}
    {#if trust.canStop}
      <div class="row">
        <button onclick={() => send("stopTrusting", { id: pane.id })} title="It does nothing from then on, until you trust it again">Stop Trusting</button>
      </div>
    {/if}
  </section>
{/if}

{#if pane.canOpenFolder}
  <div class="row">
    <button onclick={() => send("openProjectFolder", { id: pane.id })}>Open Project Folder</button>
  </div>
{/if}
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
    ("src/Icon.svelte", r#"<script>
  // One of the page's few pictures, by name: a toolbox for Development, a shield for a
  // development kit. They are shapes drawn here: nothing is loaded, and a name GridCraft
  // didn't mean draws nothing. GridCraft's own window draws the same two from the same
  // points (crates/ui-egui/src/aniefork/icons.rs).
  let { name, big = false } = $props();
</script>

{#if name === "tools"}
  <svg class="icon tools" class:big viewBox="0 0 16 16" aria-hidden="true">
    <path class="handle" d="M5.5 5.5 L5.5 3.2 L10.5 3.2 L10.5 5.5" />
    <rect class="body" x="1.5" y="5.5" width="13" height="8.5" rx="1.5" />
    <path class="lid" d="M3 5.5 L13 5.5 A1.5 1.5 0 0 1 14.5 7 L14.5 8.8 L1.5 8.8 L1.5 7 A1.5 1.5 0 0 1 3 5.5 Z" />
    <rect class="latch" x="6.7" y="7.5" width="2.6" height="2.8" rx="0.6" />
  </svg>
{:else if name === "shield"}
  <svg class="icon shield" class:big viewBox="0 0 16 16" aria-hidden="true">
    <path d="M8 1.5 L13.5 3.5 L13.5 8 L12.6 10.6 L10.8 12.8 L8 14.5 L5.2 12.8 L3.4 10.6 L2.5 8 L2.5 3.5 Z" />
  </svg>
{/if}
"#),
    ("src/Badge.svelte", r#"<script>
  // What a kit is made of, as a badge: its mark, and its name in capitals, on the colour it
  // is known by. GridCraft says the name and a code; the colour and the mark are this page's
  // own, looked up by the code from a fixed few. A code it doesn't know is a grey badge with
  // the name and no mark.
  import { badge } from "./marks.js";
  import { logo } from "./logos.js";

  let { part } = $props();

  let mark = $derived(logo(part.code));
</script>

<span class="tech {badge(part.code)}">
  {#if mark}<svg class="mark" viewBox="0 0 24 24" aria-hidden="true"><path d={mark} /></svg>{/if}
  <span>{part.label}</span>
</span>
"#),
    ("src/logos.js", r#"// The marks that stand in a badge, each as the one path it is drawn from, on a grid of 24.
//
// They are other people's marks: Python's is the Python Software Foundation's, Svelte's is
// the Svelte project's, and C's is the letter in its hexagon as it is commonly drawn. The
// shapes are from Simple Icons 16.35.0 (https://simpleicons.org, CC0 1.0), unchanged:
// icons/python.svg, icons/svelte.svg, icons/c.svg. THIRD-PARTY.md has the notice. They are
// here by the fork owner's decision (2026-10-13), as an exception to the rule that the
// repository holds nobody else's iconography; they are not to be offered upstream.
//
// A mark is looked up by a code from a fixed few, never drawn from what GridCraft sent.
const MARKS = {
  python: "M14.25.18l.9.2.73.26.59.3.45.32.34.34.25.34.16.33.1.3.04.26.02.2-.01.13V8.5l-.05.63-.13.55-.21.46-.26.38-.3.31-.33.25-.35.19-.35.14-.33.1-.3.07-.26.04-.21.02H8.77l-.69.05-.59.14-.5.22-.41.27-.33.32-.27.35-.2.36-.15.37-.1.35-.07.32-.04.27-.02.21v3.06H3.17l-.21-.03-.28-.07-.32-.12-.35-.18-.36-.26-.36-.36-.35-.46-.32-.59-.28-.73-.21-.88-.14-1.05-.05-1.23.06-1.22.16-1.04.24-.87.32-.71.36-.57.4-.44.42-.33.42-.24.4-.16.36-.1.32-.05.24-.01h.16l.06.01h8.16v-.83H6.18l-.01-2.75-.02-.37.05-.34.11-.31.17-.28.25-.26.31-.23.38-.2.44-.18.51-.15.58-.12.64-.1.71-.06.77-.04.84-.02 1.27.05zm-6.3 1.98l-.23.33-.08.41.08.41.23.34.33.22.41.09.41-.09.33-.22.23-.34.08-.41-.08-.41-.23-.33-.33-.22-.41-.09-.41.09zm13.09 3.95l.28.06.32.12.35.18.36.27.36.35.35.47.32.59.28.73.21.88.14 1.04.05 1.23-.06 1.23-.16 1.04-.24.86-.32.71-.36.57-.4.45-.42.33-.42.24-.4.16-.36.09-.32.05-.24.02-.16-.01h-8.22v.82h5.84l.01 2.76.02.36-.05.34-.11.31-.17.29-.25.25-.31.24-.38.2-.44.17-.51.15-.58.13-.64.09-.71.07-.77.04-.84.01-1.27-.04-1.07-.14-.9-.2-.73-.25-.59-.3-.45-.33-.34-.34-.25-.34-.16-.33-.1-.3-.04-.25-.02-.2.01-.13v-5.34l.05-.64.13-.54.21-.46.26-.38.3-.32.33-.24.35-.2.35-.14.33-.1.3-.06.26-.04.21-.02.13-.01h5.84l.69-.05.59-.14.5-.21.41-.28.33-.32.27-.35.2-.36.15-.36.1-.35.07-.32.04-.28.02-.21V6.07h2.09l.14.01zm-6.47 14.25l-.23.33-.08.41.08.41.23.33.33.23.41.08.41-.08.33-.23.23-.33.08-.41-.08-.41-.23-.33-.33-.23-.41-.08-.41.08z",
  svelte: "M10.354 21.125a4.44 4.44 0 0 1-4.765-1.767 4.109 4.109 0 0 1-.703-3.107 3.898 3.898 0 0 1 .134-.522l.105-.321.287.21a7.21 7.21 0 0 0 2.186 1.092l.208.063-.02.208a1.253 1.253 0 0 0 .226.83 1.337 1.337 0 0 0 1.435.533 1.231 1.231 0 0 0 .343-.15l5.59-3.562a1.164 1.164 0 0 0 .524-.778 1.242 1.242 0 0 0-.211-.937 1.338 1.338 0 0 0-1.435-.533 1.23 1.23 0 0 0-.343.15l-2.133 1.36a4.078 4.078 0 0 1-1.135.499 4.44 4.44 0 0 1-4.765-1.766 4.108 4.108 0 0 1-.702-3.108 3.855 3.855 0 0 1 1.742-2.582l5.589-3.563a4.072 4.072 0 0 1 1.135-.499 4.44 4.44 0 0 1 4.765 1.767 4.109 4.109 0 0 1 .703 3.107 3.943 3.943 0 0 1-.134.522l-.105.321-.286-.21a7.204 7.204 0 0 0-2.187-1.093l-.208-.063.02-.207a1.255 1.255 0 0 0-.226-.831 1.337 1.337 0 0 0-1.435-.532 1.231 1.231 0 0 0-.343.15L8.62 9.368a1.162 1.162 0 0 0-.524.778 1.24 1.24 0 0 0 .211.937 1.338 1.338 0 0 0 1.435.533 1.235 1.235 0 0 0 .344-.151l2.132-1.36a4.067 4.067 0 0 1 1.135-.498 4.44 4.44 0 0 1 4.765 1.766 4.108 4.108 0 0 1 .702 3.108 3.857 3.857 0 0 1-1.742 2.583l-5.589 3.562a4.072 4.072 0 0 1-1.135.499m10.358-17.95C18.484-.015 14.082-.96 10.9 1.068L5.31 4.63a6.412 6.412 0 0 0-2.896 4.295 6.753 6.753 0 0 0 .666 4.336 6.43 6.43 0 0 0-.96 2.396 6.833 6.833 0 0 0 1.168 5.167c2.229 3.19 6.63 4.135 9.812 2.108l5.59-3.562a6.41 6.41 0 0 0 2.896-4.295 6.756 6.756 0 0 0-.665-4.336 6.429 6.429 0 0 0 .958-2.396 6.831 6.831 0 0 0-1.167-5.168Z",
  c: "M16.5921 9.1962s-.354-3.298-3.627-3.39c-3.2741-.09-4.9552 2.474-4.9552 6.14 0 3.6651 1.858 6.5972 5.0451 6.5972 3.184 0 3.5381-3.665 3.5381-3.665l6.1041.365s.36 3.31-2.196 5.836c-2.552 2.5241-5.6901 2.9371-7.8762 2.9201-2.19-.017-5.2261.034-8.1602-2.97-2.938-3.0101-3.436-5.9302-3.436-8.8002 0-2.8701.556-6.6702 4.047-9.5502C7.444.72 9.849 0 12.254 0c10.0422 0 10.7172 9.2602 10.7172 9.2602z",
};

export function logo(code) {
  return (typeof code === "string" && Object.hasOwn(MARKS, code) && MARKS[code]) || null;
}
"#),
    ("src/filter.svelte.js", r#"// What is typed to filter the development kits by. It is the page's own, and stays in the
// page: nothing is sent to GridCraft for it. The list down the side has the field; the top
// of the window shows the same kits.
export const filter = $state({ typed: "" });

// Whether a kit is shown: what it says of itself and of its projects (GridCraft hands that
// over in small letters, as `search`) holds every word that was typed.
export function shown(kit) {
  const words = filter.typed.toLowerCase().split(/\s+/).filter(Boolean);
  const says = typeof kit.search === "string" ? kit.search : "";
  return words.every((word) => says.includes(word));
}
"#),
    ("src/Console.svelte", r#"<script>
  // The Console: what projects printed and logged, what failed, and what agents asked for,
  // oldest first; and over it the immediate line, where the person calls a function of a
  // project they trust and reads what comes back. Every entry, and everything a call gives
  // back, is text a project wrote: it is drawn as text. Which project's entries are shown is
  // the page's own choice; clearing them, and running a line, are the person's, by a click.
  import { send } from "./host.js";

  let { pane } = $props();

  // The project whose entries are shown: none for all of them.
  let only = $state("");
  let chosen = $derived(pane.projects.includes(only) ? only : "");
  let entries = $derived(chosen ? pane.entries.filter((entry) => entry.project === chosen) : pane.entries);

  // The line being typed. It is sent when the person runs it, and kept nowhere else.
  let line = $state("");
  let immediate = $derived(pane.immediate);
  let ran = $derived(immediate ? immediate.run : null);
  let busy = $derived(!!ran && ran.state === "running");

  function run(event) {
    event.preventDefault();
    if (line.trim()) send("immediate", { line });
  }

  const KINDS = { error: "bad", refused: "bad", agent: "agent" };
  function kind(code) {
    return (typeof code === "string" && Object.hasOwn(KINDS, code) && KINDS[code]) || "plain";
  }
</script>

<header class="head">
  <h1 class="title">{pane.title}</h1>
  <p class="lede">{pane.intro}</p>
</header>

{#if immediate}
  <h2 class="sect">{immediate.title}</h2>
  <p class="small">{immediate.says}</p>
  <form class="row" onsubmit={run}>
    <input class="name immediate" type="text" maxlength="2000" bind:value={line} placeholder={immediate.hint} aria-label={immediate.title} spellcheck="false" autocomplete="off" />
    <button class="primary" type="submit" disabled={!line.trim() || busy} title="Runs this call now, as you typed it">Run</button>
  </form>
  {#if immediate.names.length}
    <div class="filelist">
      {#each immediate.names as name}
        <button class="fileitem" title="Puts this call in the line" onclick={() => (line = name + "()")}>{name}</button>
      {/each}
    </div>
  {:else}
    <p class="small">{immediate.nothing}</p>
  {/if}
  {#if ran}
    <!-- What a call gave back and printed is the project's own text: it is drawn as text. -->
    <section class="box" aria-label="What the call gave back">
      <div class="boxhead">
        <code>{ran.line}</code>
        {#if busy}
          <span class="small" role="status">Running…</span>
          <span class="grow"></span>
          <button onclick={() => send("stopRun", { id: ran.project })}>Stop</button>
        {:else}
          <span class="chip" class:good={ran.ok} class:problem={!ran.ok}>{ran.ok ? "Finished" : "Failed"}</span>
        {/if}
      </div>
      {#if ran.text}<pre class="output" class:warn={!busy && !ran.ok}>{ran.text}</pre>{/if}
      {#if ran.printed}<p class="small">It printed:</p><pre class="output">{ran.printed}</pre>{/if}
    </section>
  {/if}
  {#if immediate.earlier.length}
    <p class="small">Earlier, to run again:</p>
    <div class="filelist">
      {#each immediate.earlier as earlier}
        <button class="fileitem" onclick={() => (line = earlier)}>{earlier}</button>
      {/each}
    </div>
  {/if}
  <h2 class="sect">Entries</h2>
{/if}

<div class="row">
  <span>{pane.says}</span>
  <span class="grow"></span>
  {#if pane.projects.length > 1}
    <select bind:value={only} aria-label="Whose entries are shown">
      <option value="">{pane.all}</option>
      {#each pane.projects as project}<option value={project}>{project}</option>{/each}
    </select>
  {/if}
  <button disabled={entries.length === 0} onclick={() => send("copyConsole", { project: chosen || null })} title="Copies what is shown, as text">Copy</button>
  <button disabled={pane.entries.length === 0} onclick={() => send("clearConsole")} title="Forgets every entry">Clear</button>
</div>

{#if entries.length === 0}
  <p class="small empty">{pane.empty}</p>
{:else}
  <div class="log">
    {#each entries as entry (entry.seq)}
      <div class="logentry {kind(entry.kind)}">
        <p class="small">{entry.who}</p>
        <pre>{entry.text}</pre>
      </div>
    {/each}
  </div>
{/if}
<p class="small">{pane.note}</p>
"#),
    ("src/Macros.svelte", r#"<script>
  // The macros (VBA) the open workbook carries, to read: its modules, and the one chosen with
  // its lines. GridCraft does not run them. What a module holds was written by whoever made
  // the workbook: every line is drawn as text.
  import { send } from "./host.js";

  let { pane } = $props();

  let module = $derived(pane.module);
</script>

<header class="head">
  <h1 class="title">{pane.title}</h1>
  <p class="lede">{pane.intro}</p>
</header>

{#if pane.problem}
  <p class="warn">{pane.problem}</p>
{:else if !pane.has}
  <p>{pane.says}</p>
{:else}
  <p class="small">{pane.says}</p>
  {#if pane.covered}<p class="small">{pane.covered}</p>{/if}
  <div class="row">
    <span class="small">{pane.rebuild}</span>
    <button onclick={() => send("copyRequest")} title="Copies a few words to hand your agent. It holds nothing of the macros, and no token.">Copy a Request for Your Agent</button>
  </div>

  <div class="twocol">
    <div class="filelist column" aria-label="The modules">
      {#each pane.modules as one}
        <button class="fileitem" class:chosen={one.chosen} title={one.kind} onclick={() => send("readMacro", { module: one.name })}>{one.name}</button>
      {/each}
    </div>
    <div class="reading">
      {#if module}
        <p><strong>{module.says}</strong></p>
        {#if module.problem}<p class="warn">{module.problem}</p>{/if}
        {#if module.cut}<p class="warn">{module.cut}</p>{/if}
        {#if module.procedures}<p class="small">{module.procedures}</p>{/if}
        {#each module.rebuilt as line}<p class="small" class:warn={line.warn}>{line.text}</p>{/each}
        <pre class="fileview">{#each module.lines as line, n}<span class="ln">{n + 1}</span><span class="tk-plain">{line}</span>{"\n"}{/each}</pre>
        {#if module.more}<p class="small">{module.more}</p>{/if}
      {/if}
    </div>
  </div>
{/if}
<p class="small">{pane.note}</p>
"#),
];
