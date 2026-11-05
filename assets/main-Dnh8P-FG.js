const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["./event-BZ79yHFs.js","./three-D-bSB6gY.js","./index-BVTacFom.js"])))=>i.map(i=>d[i]);
import{B as fh,V as rt,C as $t,M as Mo,P as gc,I as Ka,a as Ke,D as ma,R as $n,F as V1,N as Ts,b as ct,c as Im,d as Jm,e as Et,G as ka,S as Nv,f as _l,g as xt,h as sa,i as st,j as K1,k as hh,l as Dn,m as ya,n as th,o as Y1,p as Ba,q as wo,L as Yo,r as ua,s as ws,O as W1,t as Dv,u as Wa,A as Uv,v as Ov,w as ah,U as Ol,W as Ms,H as kl,x as Rl,T as kv,y as Bv,z as Ou,Z as ku,E as $m,J as e1,K as Hv,Q as Lv,X as jv,Y as qv,_ as nh,$ as Gv,a0 as Fv,a1 as Vv,a2 as Kv,a3 as Yv,a4 as Wv,a5 as X1,a6 as Xv,a7 as Pv,a8 as Zv,a9 as Qv,aa as t1,ab as dh,ac as Ge,ad as Iv,ae as P1,af as Z1,ag as Jv,ah as $v,ai as Xu,aj as Pu,ak as e2,al as t2,am as a2,an as Fo,ao as n2,ap as o2,aq as a1,ar as l2,as as s2,at as i2,au as n1,av as fi,aw as sr,ax as ir,ay as o1,az as r2,aA as c2,aB as u2,aC as f2,aD as h2,aE as d2}from"./three-D-bSB6gY.js";(function(){const l=document.createElement("link").relList;if(l&&l.supports&&l.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))u(i);new MutationObserver(i=>{for(const f of i)if(f.type==="childList")for(const h of f.addedNodes)h.tagName==="LINK"&&h.rel==="modulepreload"&&u(h)}).observe(document,{childList:!0,subtree:!0});function r(i){const f={};return i.integrity&&(f.integrity=i.integrity),i.referrerPolicy&&(f.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?f.credentials="include":i.crossOrigin==="anonymous"?f.credentials="omit":f.credentials="same-origin",f}function u(i){if(i.ep)return;i.ep=!0;const f=r(i);fetch(i.href,f)}})();var kf={exports:{}},dc={};/**
 * @license React
 * react-jsx-runtime.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var l1;function m2(){if(l1)return dc;l1=1;var o=Symbol.for("react.transitional.element"),l=Symbol.for("react.fragment");function r(u,i,f){var h=null;if(f!==void 0&&(h=""+f),i.key!==void 0&&(h=""+i.key),"key"in i){f={};for(var v in i)v!=="key"&&(f[v]=i[v])}else f=i;return i=f.ref,{$$typeof:o,type:u,key:h,ref:i!==void 0?i:null,props:f}}return dc.Fragment=l,dc.jsx=r,dc.jsxs=r,dc}var s1;function p2(){return s1||(s1=1,kf.exports=m2()),kf.exports}var se=p2(),Bf={exports:{}},lt={};/**
 * @license React
 * react.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var i1;function v2(){if(i1)return lt;i1=1;var o=Symbol.for("react.transitional.element"),l=Symbol.for("react.portal"),r=Symbol.for("react.fragment"),u=Symbol.for("react.strict_mode"),i=Symbol.for("react.profiler"),f=Symbol.for("react.consumer"),h=Symbol.for("react.context"),v=Symbol.for("react.forward_ref"),m=Symbol.for("react.suspense"),p=Symbol.for("react.memo"),b=Symbol.for("react.lazy"),S=Symbol.for("react.activity"),C=Symbol.iterator;function x(E){return E===null||typeof E!="object"?null:(E=C&&E[C]||E["@@iterator"],typeof E=="function"?E:null)}var D={isMounted:function(){return!1},enqueueForceUpdate:function(){},enqueueReplaceState:function(){},enqueueSetState:function(){}},H=Object.assign,q={};function P(E,Z,T){this.props=E,this.context=Z,this.refs=q,this.updater=T||D}P.prototype.isReactComponent={},P.prototype.setState=function(E,Z){if(typeof E!="object"&&typeof E!="function"&&E!=null)throw Error("takes an object of state variables to update or a function which returns an object of state variables.");this.updater.enqueueSetState(this,E,Z,"setState")},P.prototype.forceUpdate=function(E){this.updater.enqueueForceUpdate(this,E,"forceUpdate")};function K(){}K.prototype=P.prototype;function U(E,Z,T){this.props=E,this.context=Z,this.refs=q,this.updater=T||D}var z=U.prototype=new K;z.constructor=U,H(z,P.prototype),z.isPureReactComponent=!0;var R=Array.isArray;function V(){}var J={H:null,A:null,T:null,S:null},Q=Object.prototype.hasOwnProperty;function ne(E,Z,T){var F=T.ref;return{$$typeof:o,type:E,key:Z,ref:F!==void 0?F:null,props:T}}function B(E,Z){return ne(E.type,Z,E.props)}function ae(E){return typeof E=="object"&&E!==null&&E.$$typeof===o}function I(E){var Z={"=":"=0",":":"=2"};return"$"+E.replace(/[=:]/g,function(T){return Z[T]})}var ge=/\/+/g;function Re(E,Z){return typeof E=="object"&&E!==null&&E.key!=null?I(""+E.key):Z.toString(36)}function le(E){switch(E.status){case"fulfilled":return E.value;case"rejected":throw E.reason;default:switch(typeof E.status=="string"?E.then(V,V):(E.status="pending",E.then(function(Z){E.status==="pending"&&(E.status="fulfilled",E.value=Z)},function(Z){E.status==="pending"&&(E.status="rejected",E.reason=Z)})),E.status){case"fulfilled":return E.value;case"rejected":throw E.reason}}throw E}function G(E,Z,T,F,ce){var ue=typeof E;(ue==="undefined"||ue==="boolean")&&(E=null);var Ee=!1;if(E===null)Ee=!0;else switch(ue){case"bigint":case"string":case"number":Ee=!0;break;case"object":switch(E.$$typeof){case o:case l:Ee=!0;break;case b:return Ee=E._init,G(Ee(E._payload),Z,T,F,ce)}}if(Ee)return ce=ce(E),Ee=F===""?"."+Re(E,0):F,R(ce)?(T="",Ee!=null&&(T=Ee.replace(ge,"$&/")+"/"),G(ce,Z,T,"",function(oe){return oe})):ce!=null&&(ae(ce)&&(ce=B(ce,T+(ce.key==null||E&&E.key===ce.key?"":(""+ce.key).replace(ge,"$&/")+"/")+Ee)),Z.push(ce)),1;Ee=0;var Te=F===""?".":F+":";if(R(E))for(var Ue=0;Ue<E.length;Ue++)F=E[Ue],ue=Te+Re(F,Ue),Ee+=G(F,Z,T,ue,ce);else if(Ue=x(E),typeof Ue=="function")for(E=Ue.call(E),Ue=0;!(F=E.next()).done;)F=F.value,ue=Te+Re(F,Ue++),Ee+=G(F,Z,T,ue,ce);else if(ue==="object"){if(typeof E.then=="function")return G(le(E),Z,T,F,ce);throw Z=String(E),Error("Objects are not valid as a React child (found: "+(Z==="[object Object]"?"object with keys {"+Object.keys(E).join(", ")+"}":Z)+"). If you meant to render a collection of children, use an array instead.")}return Ee}function j(E,Z,T){if(E==null)return E;var F=[],ce=0;return G(E,F,"","",function(ue){return Z.call(T,ue,ce++)}),F}function me(E){if(E._status===-1){var Z=E._result;Z=Z(),Z.then(function(T){(E._status===0||E._status===-1)&&(E._status=1,E._result=T)},function(T){(E._status===0||E._status===-1)&&(E._status=2,E._result=T)}),E._status===-1&&(E._status=0,E._result=Z)}if(E._status===1)return E._result.default;throw E._result}var X=typeof reportError=="function"?reportError:function(E){if(typeof window=="object"&&typeof window.ErrorEvent=="function"){var Z=new window.ErrorEvent("error",{bubbles:!0,cancelable:!0,message:typeof E=="object"&&E!==null&&typeof E.message=="string"?String(E.message):String(E),error:E});if(!window.dispatchEvent(Z))return}else if(typeof process=="object"&&typeof process.emit=="function"){process.emit("uncaughtException",E);return}console.error(E)},we={map:j,forEach:function(E,Z,T){j(E,function(){Z.apply(this,arguments)},T)},count:function(E){var Z=0;return j(E,function(){Z++}),Z},toArray:function(E){return j(E,function(Z){return Z})||[]},only:function(E){if(!ae(E))throw Error("React.Children.only expected to receive a single React element child.");return E}};return lt.Activity=S,lt.Children=we,lt.Component=P,lt.Fragment=r,lt.Profiler=i,lt.PureComponent=U,lt.StrictMode=u,lt.Suspense=m,lt.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE=J,lt.__COMPILER_RUNTIME={__proto__:null,c:function(E){return J.H.useMemoCache(E)}},lt.cache=function(E){return function(){return E.apply(null,arguments)}},lt.cacheSignal=function(){return null},lt.cloneElement=function(E,Z,T){if(E==null)throw Error("The argument must be a React element, but you passed "+E+".");var F=H({},E.props),ce=E.key;if(Z!=null)for(ue in Z.key!==void 0&&(ce=""+Z.key),Z)!Q.call(Z,ue)||ue==="key"||ue==="__self"||ue==="__source"||ue==="ref"&&Z.ref===void 0||(F[ue]=Z[ue]);var ue=arguments.length-2;if(ue===1)F.children=T;else if(1<ue){for(var Ee=Array(ue),Te=0;Te<ue;Te++)Ee[Te]=arguments[Te+2];F.children=Ee}return ne(E.type,ce,F)},lt.createContext=function(E){return E={$$typeof:h,_currentValue:E,_currentValue2:E,_threadCount:0,Provider:null,Consumer:null},E.Provider=E,E.Consumer={$$typeof:f,_context:E},E},lt.createElement=function(E,Z,T){var F,ce={},ue=null;if(Z!=null)for(F in Z.key!==void 0&&(ue=""+Z.key),Z)Q.call(Z,F)&&F!=="key"&&F!=="__self"&&F!=="__source"&&(ce[F]=Z[F]);var Ee=arguments.length-2;if(Ee===1)ce.children=T;else if(1<Ee){for(var Te=Array(Ee),Ue=0;Ue<Ee;Ue++)Te[Ue]=arguments[Ue+2];ce.children=Te}if(E&&E.defaultProps)for(F in Ee=E.defaultProps,Ee)ce[F]===void 0&&(ce[F]=Ee[F]);return ne(E,ue,ce)},lt.createRef=function(){return{current:null}},lt.forwardRef=function(E){return{$$typeof:v,render:E}},lt.isValidElement=ae,lt.lazy=function(E){return{$$typeof:b,_payload:{_status:-1,_result:E},_init:me}},lt.memo=function(E,Z){return{$$typeof:p,type:E,compare:Z===void 0?null:Z}},lt.startTransition=function(E){var Z=J.T,T={};J.T=T;try{var F=E(),ce=J.S;ce!==null&&ce(T,F),typeof F=="object"&&F!==null&&typeof F.then=="function"&&F.then(V,X)}catch(ue){X(ue)}finally{Z!==null&&T.types!==null&&(Z.types=T.types),J.T=Z}},lt.unstable_useCacheRefresh=function(){return J.H.useCacheRefresh()},lt.use=function(E){return J.H.use(E)},lt.useActionState=function(E,Z,T){return J.H.useActionState(E,Z,T)},lt.useCallback=function(E,Z){return J.H.useCallback(E,Z)},lt.useContext=function(E){return J.H.useContext(E)},lt.useDebugValue=function(){},lt.useDeferredValue=function(E,Z){return J.H.useDeferredValue(E,Z)},lt.useEffect=function(E,Z){return J.H.useEffect(E,Z)},lt.useEffectEvent=function(E){return J.H.useEffectEvent(E)},lt.useId=function(){return J.H.useId()},lt.useImperativeHandle=function(E,Z,T){return J.H.useImperativeHandle(E,Z,T)},lt.useInsertionEffect=function(E,Z){return J.H.useInsertionEffect(E,Z)},lt.useLayoutEffect=function(E,Z){return J.H.useLayoutEffect(E,Z)},lt.useMemo=function(E,Z){return J.H.useMemo(E,Z)},lt.useOptimistic=function(E,Z){return J.H.useOptimistic(E,Z)},lt.useReducer=function(E,Z,T){return J.H.useReducer(E,Z,T)},lt.useRef=function(E){return J.H.useRef(E)},lt.useState=function(E){return J.H.useState(E)},lt.useSyncExternalStore=function(E,Z,T){return J.H.useSyncExternalStore(E,Z,T)},lt.useTransition=function(){return J.H.useTransition()},lt.version="19.2.8",lt}var r1;function mh(){return r1||(r1=1,Bf.exports=v2()),Bf.exports}var pt=mh(),Hf={exports:{}},mc={},Lf={exports:{}},jf={};/**
 * @license React
 * scheduler.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var c1;function g2(){return c1||(c1=1,(function(o){function l(G,j){var me=G.length;G.push(j);e:for(;0<me;){var X=me-1>>>1,we=G[X];if(0<i(we,j))G[X]=j,G[me]=we,me=X;else break e}}function r(G){return G.length===0?null:G[0]}function u(G){if(G.length===0)return null;var j=G[0],me=G.pop();if(me!==j){G[0]=me;e:for(var X=0,we=G.length,E=we>>>1;X<E;){var Z=2*(X+1)-1,T=G[Z],F=Z+1,ce=G[F];if(0>i(T,me))F<we&&0>i(ce,T)?(G[X]=ce,G[F]=me,X=F):(G[X]=T,G[Z]=me,X=Z);else if(F<we&&0>i(ce,me))G[X]=ce,G[F]=me,X=F;else break e}}return j}function i(G,j){var me=G.sortIndex-j.sortIndex;return me!==0?me:G.id-j.id}if(o.unstable_now=void 0,typeof performance=="object"&&typeof performance.now=="function"){var f=performance;o.unstable_now=function(){return f.now()}}else{var h=Date,v=h.now();o.unstable_now=function(){return h.now()-v}}var m=[],p=[],b=1,S=null,C=3,x=!1,D=!1,H=!1,q=!1,P=typeof setTimeout=="function"?setTimeout:null,K=typeof clearTimeout=="function"?clearTimeout:null,U=typeof setImmediate<"u"?setImmediate:null;function z(G){for(var j=r(p);j!==null;){if(j.callback===null)u(p);else if(j.startTime<=G)u(p),j.sortIndex=j.expirationTime,l(m,j);else break;j=r(p)}}function R(G){if(H=!1,z(G),!D)if(r(m)!==null)D=!0,V||(V=!0,I());else{var j=r(p);j!==null&&le(R,j.startTime-G)}}var V=!1,J=-1,Q=5,ne=-1;function B(){return q?!0:!(o.unstable_now()-ne<Q)}function ae(){if(q=!1,V){var G=o.unstable_now();ne=G;var j=!0;try{e:{D=!1,H&&(H=!1,K(J),J=-1),x=!0;var me=C;try{t:{for(z(G),S=r(m);S!==null&&!(S.expirationTime>G&&B());){var X=S.callback;if(typeof X=="function"){S.callback=null,C=S.priorityLevel;var we=X(S.expirationTime<=G);if(G=o.unstable_now(),typeof we=="function"){S.callback=we,z(G),j=!0;break t}S===r(m)&&u(m),z(G)}else u(m);S=r(m)}if(S!==null)j=!0;else{var E=r(p);E!==null&&le(R,E.startTime-G),j=!1}}break e}finally{S=null,C=me,x=!1}j=void 0}}finally{j?I():V=!1}}}var I;if(typeof U=="function")I=function(){U(ae)};else if(typeof MessageChannel<"u"){var ge=new MessageChannel,Re=ge.port2;ge.port1.onmessage=ae,I=function(){Re.postMessage(null)}}else I=function(){P(ae,0)};function le(G,j){J=P(function(){G(o.unstable_now())},j)}o.unstable_IdlePriority=5,o.unstable_ImmediatePriority=1,o.unstable_LowPriority=4,o.unstable_NormalPriority=3,o.unstable_Profiling=null,o.unstable_UserBlockingPriority=2,o.unstable_cancelCallback=function(G){G.callback=null},o.unstable_forceFrameRate=function(G){0>G||125<G?console.error("forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported"):Q=0<G?Math.floor(1e3/G):5},o.unstable_getCurrentPriorityLevel=function(){return C},o.unstable_next=function(G){switch(C){case 1:case 2:case 3:var j=3;break;default:j=C}var me=C;C=j;try{return G()}finally{C=me}},o.unstable_requestPaint=function(){q=!0},o.unstable_runWithPriority=function(G,j){switch(G){case 1:case 2:case 3:case 4:case 5:break;default:G=3}var me=C;C=G;try{return j()}finally{C=me}},o.unstable_scheduleCallback=function(G,j,me){var X=o.unstable_now();switch(typeof me=="object"&&me!==null?(me=me.delay,me=typeof me=="number"&&0<me?X+me:X):me=X,G){case 1:var we=-1;break;case 2:we=250;break;case 5:we=1073741823;break;case 4:we=1e4;break;default:we=5e3}return we=me+we,G={id:b++,callback:j,priorityLevel:G,startTime:me,expirationTime:we,sortIndex:-1},me>X?(G.sortIndex=me,l(p,G),r(m)===null&&G===r(p)&&(H?(K(J),J=-1):H=!0,le(R,me-X))):(G.sortIndex=we,l(m,G),D||x||(D=!0,V||(V=!0,I()))),G},o.unstable_shouldYield=B,o.unstable_wrapCallback=function(G){var j=C;return function(){var me=C;C=j;try{return G.apply(this,arguments)}finally{C=me}}}})(jf)),jf}var u1;function y2(){return u1||(u1=1,Lf.exports=g2()),Lf.exports}var qf={exports:{}},Fa={};/**
 * @license React
 * react-dom.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var f1;function x2(){if(f1)return Fa;f1=1;var o=mh();function l(m){var p="https://react.dev/errors/"+m;if(1<arguments.length){p+="?args[]="+encodeURIComponent(arguments[1]);for(var b=2;b<arguments.length;b++)p+="&args[]="+encodeURIComponent(arguments[b])}return"Minified React error #"+m+"; visit "+p+" for the full message or use the non-minified dev environment for full errors and additional helpful warnings."}function r(){}var u={d:{f:r,r:function(){throw Error(l(522))},D:r,C:r,L:r,m:r,X:r,S:r,M:r},p:0,findDOMNode:null},i=Symbol.for("react.portal");function f(m,p,b){var S=3<arguments.length&&arguments[3]!==void 0?arguments[3]:null;return{$$typeof:i,key:S==null?null:""+S,children:m,containerInfo:p,implementation:b}}var h=o.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE;function v(m,p){if(m==="font")return"";if(typeof p=="string")return p==="use-credentials"?p:""}return Fa.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE=u,Fa.createPortal=function(m,p){var b=2<arguments.length&&arguments[2]!==void 0?arguments[2]:null;if(!p||p.nodeType!==1&&p.nodeType!==9&&p.nodeType!==11)throw Error(l(299));return f(m,p,null,b)},Fa.flushSync=function(m){var p=h.T,b=u.p;try{if(h.T=null,u.p=2,m)return m()}finally{h.T=p,u.p=b,u.d.f()}},Fa.preconnect=function(m,p){typeof m=="string"&&(p?(p=p.crossOrigin,p=typeof p=="string"?p==="use-credentials"?p:"":void 0):p=null,u.d.C(m,p))},Fa.prefetchDNS=function(m){typeof m=="string"&&u.d.D(m)},Fa.preinit=function(m,p){if(typeof m=="string"&&p&&typeof p.as=="string"){var b=p.as,S=v(b,p.crossOrigin),C=typeof p.integrity=="string"?p.integrity:void 0,x=typeof p.fetchPriority=="string"?p.fetchPriority:void 0;b==="style"?u.d.S(m,typeof p.precedence=="string"?p.precedence:void 0,{crossOrigin:S,integrity:C,fetchPriority:x}):b==="script"&&u.d.X(m,{crossOrigin:S,integrity:C,fetchPriority:x,nonce:typeof p.nonce=="string"?p.nonce:void 0})}},Fa.preinitModule=function(m,p){if(typeof m=="string")if(typeof p=="object"&&p!==null){if(p.as==null||p.as==="script"){var b=v(p.as,p.crossOrigin);u.d.M(m,{crossOrigin:b,integrity:typeof p.integrity=="string"?p.integrity:void 0,nonce:typeof p.nonce=="string"?p.nonce:void 0})}}else p==null&&u.d.M(m)},Fa.preload=function(m,p){if(typeof m=="string"&&typeof p=="object"&&p!==null&&typeof p.as=="string"){var b=p.as,S=v(b,p.crossOrigin);u.d.L(m,b,{crossOrigin:S,integrity:typeof p.integrity=="string"?p.integrity:void 0,nonce:typeof p.nonce=="string"?p.nonce:void 0,type:typeof p.type=="string"?p.type:void 0,fetchPriority:typeof p.fetchPriority=="string"?p.fetchPriority:void 0,referrerPolicy:typeof p.referrerPolicy=="string"?p.referrerPolicy:void 0,imageSrcSet:typeof p.imageSrcSet=="string"?p.imageSrcSet:void 0,imageSizes:typeof p.imageSizes=="string"?p.imageSizes:void 0,media:typeof p.media=="string"?p.media:void 0})}},Fa.preloadModule=function(m,p){if(typeof m=="string")if(p){var b=v(p.as,p.crossOrigin);u.d.m(m,{as:typeof p.as=="string"&&p.as!=="script"?p.as:void 0,crossOrigin:b,integrity:typeof p.integrity=="string"?p.integrity:void 0})}else u.d.m(m)},Fa.requestFormReset=function(m){u.d.r(m)},Fa.unstable_batchedUpdates=function(m,p){return m(p)},Fa.useFormState=function(m,p,b){return h.H.useFormState(m,p,b)},Fa.useFormStatus=function(){return h.H.useHostTransitionStatus()},Fa.version="19.2.8",Fa}var h1;function w2(){if(h1)return qf.exports;h1=1;function o(){if(!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__>"u"||typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE!="function"))try{__REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(o)}catch(l){console.error(l)}}return o(),qf.exports=x2(),qf.exports}/**
 * @license React
 * react-dom-client.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var d1;function b2(){if(d1)return mc;d1=1;var o=y2(),l=mh(),r=w2();function u(e){var t="https://react.dev/errors/"+e;if(1<arguments.length){t+="?args[]="+encodeURIComponent(arguments[1]);for(var a=2;a<arguments.length;a++)t+="&args[]="+encodeURIComponent(arguments[a])}return"Minified React error #"+e+"; visit "+t+" for the full message or use the non-minified dev environment for full errors and additional helpful warnings."}function i(e){return!(!e||e.nodeType!==1&&e.nodeType!==9&&e.nodeType!==11)}function f(e){var t=e,a=e;if(e.alternate)for(;t.return;)t=t.return;else{e=t;do t=e,(t.flags&4098)!==0&&(a=t.return),e=t.return;while(e)}return t.tag===3?a:null}function h(e){if(e.tag===13){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function v(e){if(e.tag===31){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function m(e){if(f(e)!==e)throw Error(u(188))}function p(e){var t=e.alternate;if(!t){if(t=f(e),t===null)throw Error(u(188));return t!==e?null:e}for(var a=e,n=t;;){var s=a.return;if(s===null)break;var c=s.alternate;if(c===null){if(n=s.return,n!==null){a=n;continue}break}if(s.child===c.child){for(c=s.child;c;){if(c===a)return m(s),e;if(c===n)return m(s),t;c=c.sibling}throw Error(u(188))}if(a.return!==n.return)a=s,n=c;else{for(var d=!1,w=s.child;w;){if(w===a){d=!0,a=s,n=c;break}if(w===n){d=!0,n=s,a=c;break}w=w.sibling}if(!d){for(w=c.child;w;){if(w===a){d=!0,a=c,n=s;break}if(w===n){d=!0,n=c,a=s;break}w=w.sibling}if(!d)throw Error(u(189))}}if(a.alternate!==n)throw Error(u(190))}if(a.tag!==3)throw Error(u(188));return a.stateNode.current===a?e:t}function b(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e;for(e=e.child;e!==null;){if(t=b(e),t!==null)return t;e=e.sibling}return null}var S=Object.assign,C=Symbol.for("react.element"),x=Symbol.for("react.transitional.element"),D=Symbol.for("react.portal"),H=Symbol.for("react.fragment"),q=Symbol.for("react.strict_mode"),P=Symbol.for("react.profiler"),K=Symbol.for("react.consumer"),U=Symbol.for("react.context"),z=Symbol.for("react.forward_ref"),R=Symbol.for("react.suspense"),V=Symbol.for("react.suspense_list"),J=Symbol.for("react.memo"),Q=Symbol.for("react.lazy"),ne=Symbol.for("react.activity"),B=Symbol.for("react.memo_cache_sentinel"),ae=Symbol.iterator;function I(e){return e===null||typeof e!="object"?null:(e=ae&&e[ae]||e["@@iterator"],typeof e=="function"?e:null)}var ge=Symbol.for("react.client.reference");function Re(e){if(e==null)return null;if(typeof e=="function")return e.$$typeof===ge?null:e.displayName||e.name||null;if(typeof e=="string")return e;switch(e){case H:return"Fragment";case P:return"Profiler";case q:return"StrictMode";case R:return"Suspense";case V:return"SuspenseList";case ne:return"Activity"}if(typeof e=="object")switch(e.$$typeof){case D:return"Portal";case U:return e.displayName||"Context";case K:return(e._context.displayName||"Context")+".Consumer";case z:var t=e.render;return e=e.displayName,e||(e=t.displayName||t.name||"",e=e!==""?"ForwardRef("+e+")":"ForwardRef"),e;case J:return t=e.displayName||null,t!==null?t:Re(e.type)||"Memo";case Q:t=e._payload,e=e._init;try{return Re(e(t))}catch{}}return null}var le=Array.isArray,G=l.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,j=r.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,me={pending:!1,data:null,method:null,action:null},X=[],we=-1;function E(e){return{current:e}}function Z(e){0>we||(e.current=X[we],X[we]=null,we--)}function T(e,t){we++,X[we]=e.current,e.current=t}var F=E(null),ce=E(null),ue=E(null),Ee=E(null);function Te(e,t){switch(T(ue,t),T(ce,e),T(F,null),t.nodeType){case 9:case 11:e=(e=t.documentElement)&&(e=e.namespaceURI)?wm(e):0;break;default:if(e=t.tagName,t=t.namespaceURI)t=wm(t),e=bm(t,e);else switch(e){case"svg":e=1;break;case"math":e=2;break;default:e=0}}Z(F),T(F,e)}function Ue(){Z(F),Z(ce),Z(ue)}function oe(e){e.memoizedState!==null&&T(Ee,e);var t=F.current,a=bm(t,e.type);t!==a&&(T(ce,e),T(F,a))}function _e(e){ce.current===e&&(Z(F),Z(ce)),Ee.current===e&&(Z(Ee),cc._currentValue=me)}var Ze,Ye;function Xe(e){if(Ze===void 0)try{throw Error()}catch(a){var t=a.stack.trim().match(/\n( *(at )?)/);Ze=t&&t[1]||"",Ye=-1<a.stack.indexOf(`
    at`)?" (<anonymous>)":-1<a.stack.indexOf("@")?"@unknown:0:0":""}return`
`+Ze+e+Ye}var tt=!1;function ot(e,t){if(!e||tt)return"";tt=!0;var a=Error.prepareStackTrace;Error.prepareStackTrace=void 0;try{var n={DetermineComponentFrameRoot:function(){try{if(t){var Ce=function(){throw Error()};if(Object.defineProperty(Ce.prototype,"props",{set:function(){throw Error()}}),typeof Reflect=="object"&&Reflect.construct){try{Reflect.construct(Ce,[])}catch(ve){var he=ve}Reflect.construct(e,[],Ce)}else{try{Ce.call()}catch(ve){he=ve}e.call(Ce.prototype)}}else{try{throw Error()}catch(ve){he=ve}(Ce=e())&&typeof Ce.catch=="function"&&Ce.catch(function(){})}}catch(ve){if(ve&&he&&typeof ve.stack=="string")return[ve.stack,he.stack]}return[null,null]}};n.DetermineComponentFrameRoot.displayName="DetermineComponentFrameRoot";var s=Object.getOwnPropertyDescriptor(n.DetermineComponentFrameRoot,"name");s&&s.configurable&&Object.defineProperty(n.DetermineComponentFrameRoot,"name",{value:"DetermineComponentFrameRoot"});var c=n.DetermineComponentFrameRoot(),d=c[0],w=c[1];if(d&&w){var L=d.split(`
`),re=w.split(`
`);for(s=n=0;n<L.length&&!L[n].includes("DetermineComponentFrameRoot");)n++;for(;s<re.length&&!re[s].includes("DetermineComponentFrameRoot");)s++;if(n===L.length||s===re.length)for(n=L.length-1,s=re.length-1;1<=n&&0<=s&&L[n]!==re[s];)s--;for(;1<=n&&0<=s;n--,s--)if(L[n]!==re[s]){if(n!==1||s!==1)do if(n--,s--,0>s||L[n]!==re[s]){var be=`
`+L[n].replace(" at new "," at ");return e.displayName&&be.includes("<anonymous>")&&(be=be.replace("<anonymous>",e.displayName)),be}while(1<=n&&0<=s);break}}}finally{tt=!1,Error.prepareStackTrace=a}return(a=e?e.displayName||e.name:"")?Xe(a):""}function Ut(e,t){switch(e.tag){case 26:case 27:case 5:return Xe(e.type);case 16:return Xe("Lazy");case 13:return e.child!==t&&t!==null?Xe("Suspense Fallback"):Xe("Suspense");case 19:return Xe("SuspenseList");case 0:case 15:return ot(e.type,!1);case 11:return ot(e.type.render,!1);case 1:return ot(e.type,!0);case 31:return Xe("Activity");default:return""}}function fa(e){try{var t="",a=null;do t+=Ut(e,a),a=e,e=e.return;while(e);return t}catch(n){return`
Error generating stack: `+n.message+`
`+n.stack}}var Ca=Object.prototype.hasOwnProperty,ta=o.unstable_scheduleCallback,Ha=o.unstable_cancelCallback,Xa=o.unstable_shouldYield,an=o.unstable_requestPaint,qt=o.unstable_now,to=o.unstable_getCurrentPriorityLevel,La=o.unstable_ImmediatePriority,ao=o.unstable_UserBlockingPriority,Pa=o.unstable_NormalPriority,ut=o.unstable_LowPriority,xa=o.unstable_IdlePriority,nn=o.log,on=o.unstable_setDisableYieldValue,ln=null,wa=null;function sn(e){if(typeof nn=="function"&&on(e),wa&&typeof wa.setStrictMode=="function")try{wa.setStrictMode(ln,e)}catch{}}var Vt=Math.clz32?Math.clz32:Jo,Qo=Math.log,Io=Math.LN2;function Jo(e){return e>>>=0,e===0?32:31-(Qo(e)/Io|0)|0}var yn=256,xn=262144,Hl=4194304;function _a(e){var t=e&42;if(t!==0)return t;switch(e&-e){case 1:return 1;case 2:return 2;case 4:return 4;case 8:return 8;case 16:return 16;case 32:return 32;case 64:return 64;case 128:return 128;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:return e&261888;case 262144:case 524288:case 1048576:case 2097152:return e&3932160;case 4194304:case 8388608:case 16777216:case 33554432:return e&62914560;case 67108864:return 67108864;case 134217728:return 134217728;case 268435456:return 268435456;case 536870912:return 536870912;case 1073741824:return 0;default:return e}}function Ll(e,t,a){var n=e.pendingLanes;if(n===0)return 0;var s=0,c=e.suspendedLanes,d=e.pingedLanes;e=e.warmLanes;var w=n&134217727;return w!==0?(n=w&~c,n!==0?s=_a(n):(d&=w,d!==0?s=_a(d):a||(a=w&~e,a!==0&&(s=_a(a))))):(w=n&~c,w!==0?s=_a(w):d!==0?s=_a(d):a||(a=n&~e,a!==0&&(s=_a(a)))),s===0?0:t!==0&&t!==s&&(t&c)===0&&(c=s&-s,a=t&-t,c>=a||c===32&&(a&4194048)!==0)?t:s}function So(e,t){return(e.pendingLanes&~(e.suspendedLanes&~e.pingedLanes)&t)===0}function Sc(e,t){switch(e){case 1:case 2:case 4:case 8:case 64:return t+250;case 16:case 32:case 128:case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:return t+5e3;case 4194304:case 8388608:case 16777216:case 33554432:return-1;case 67108864:case 134217728:case 268435456:case 536870912:case 1073741824:return-1;default:return-1}}function ur(){var e=Hl;return Hl<<=1,(Hl&62914560)===0&&(Hl=4194304),e}function zs(e){for(var t=[],a=0;31>a;a++)t.push(e);return t}function $o(e,t){e.pendingLanes|=t,t!==268435456&&(e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0)}function fr(e,t,a,n,s,c){var d=e.pendingLanes;e.pendingLanes=a,e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0,e.expiredLanes&=a,e.entangledLanes&=a,e.errorRecoveryDisabledLanes&=a,e.shellSuspendCounter=0;var w=e.entanglements,L=e.expirationTimes,re=e.hiddenUpdates;for(a=d&~a;0<a;){var be=31-Vt(a),Ce=1<<be;w[be]=0,L[be]=-1;var he=re[be];if(he!==null)for(re[be]=null,be=0;be<he.length;be++){var ve=he[be];ve!==null&&(ve.lane&=-536870913)}a&=~Ce}n!==0&&Cs(e,n,0),c!==0&&s===0&&e.tag!==0&&(e.suspendedLanes|=c&~(d&~t))}function Cs(e,t,a){e.pendingLanes|=t,e.suspendedLanes&=~t;var n=31-Vt(t);e.entangledLanes|=t,e.entanglements[n]=e.entanglements[n]|1073741824|a&261930}function gi(e,t){var a=e.entangledLanes|=t;for(e=e.entanglements;a;){var n=31-Vt(a),s=1<<n;s&t|e[n]&t&&(e[n]|=t),a&=~s}}function hr(e,t){var a=t&-t;return a=(a&42)!==0?1:jl(a),(a&(e.suspendedLanes|t))!==0?0:a}function jl(e){switch(e){case 2:e=1;break;case 8:e=4;break;case 32:e=16;break;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:case 4194304:case 8388608:case 16777216:case 33554432:e=128;break;case 268435456:e=134217728;break;default:e=0}return e}function _s(e){return e&=-e,2<e?8<e?(e&134217727)!==0?32:268435456:8:2}function yi(){var e=j.p;return e!==0?e:(e=window.event,e===void 0?32:Km(e.type))}function xi(e,t){var a=j.p;try{return j.p=e,t()}finally{j.p=a}}var On=Math.random().toString(36).slice(2),ra="__reactFiber$"+On,ba="__reactProps$"+On,To="__reactContainer$"+On,As="__reactEvents$"+On,dr="__reactListeners$"+On,mr="__reactHandles$"+On,wi="__reactResources$"+On,el="__reactMarker$"+On;function Es(e){delete e[ra],delete e[ba],delete e[As],delete e[dr],delete e[mr]}function no(e){var t=e[ra];if(t)return t;for(var a=e.parentNode;a;){if(t=a[To]||a[ra]){if(a=t.alternate,t.child!==null||a!==null&&a.child!==null)for(e=Am(e);e!==null;){if(a=e[ra])return a;e=Am(e)}return t}e=a,a=e.parentNode}return null}function zo(e){if(e=e[ra]||e[To]){var t=e.tag;if(t===5||t===6||t===13||t===31||t===26||t===27||t===3)return e}return null}function tl(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e.stateNode;throw Error(u(33))}function Co(e){var t=e[wi];return t||(t=e[wi]={hoistableStyles:new Map,hoistableScripts:new Map}),t}function aa(e){e[el]=!0}var pr=new Set,rn={};function wn(e,t){bn(e,t),bn(e+"Capture",t)}function bn(e,t){for(rn[e]=t,e=0;e<t.length;e++)pr.add(t[e])}var ql=RegExp("^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$"),Tc={},zc={};function zt(e){return Ca.call(zc,e)?!0:Ca.call(Tc,e)?!1:ql.test(e)?zc[e]=!0:(Tc[e]=!0,!1)}function dt(e,t,a){if(zt(t))if(a===null)e.removeAttribute(t);else{switch(typeof a){case"undefined":case"function":case"symbol":e.removeAttribute(t);return;case"boolean":var n=t.toLowerCase().slice(0,5);if(n!=="data-"&&n!=="aria-"){e.removeAttribute(t);return}}e.setAttribute(t,""+a)}}function Za(e,t,a){if(a===null)e.removeAttribute(t);else{switch(typeof a){case"undefined":case"function":case"symbol":case"boolean":e.removeAttribute(t);return}e.setAttribute(t,""+a)}}function Qa(e,t,a,n){if(n===null)e.removeAttribute(a);else{switch(typeof n){case"undefined":case"function":case"symbol":case"boolean":e.removeAttribute(a);return}e.setAttributeNS(t,a,""+n)}}function pa(e){switch(typeof e){case"bigint":case"boolean":case"number":case"string":case"undefined":return e;case"object":return e;default:return""}}function bi(e){var t=e.type;return(e=e.nodeName)&&e.toLowerCase()==="input"&&(t==="checkbox"||t==="radio")}function Cc(e,t,a){var n=Object.getOwnPropertyDescriptor(e.constructor.prototype,t);if(!e.hasOwnProperty(t)&&typeof n<"u"&&typeof n.get=="function"&&typeof n.set=="function"){var s=n.get,c=n.set;return Object.defineProperty(e,t,{configurable:!0,get:function(){return s.call(this)},set:function(d){a=""+d,c.call(this,d)}}),Object.defineProperty(e,t,{enumerable:n.enumerable}),{getValue:function(){return a},setValue:function(d){a=""+d},stopTracking:function(){e._valueTracker=null,delete e[t]}}}}function _o(e){if(!e._valueTracker){var t=bi(e)?"checked":"value";e._valueTracker=Cc(e,t,""+e[t])}}function vr(e){if(!e)return!1;var t=e._valueTracker;if(!t)return!0;var a=t.getValue(),n="";return e&&(n=bi(e)?e.checked?"true":"false":e.value),e=n,e!==a?(t.setValue(e),!0):!1}function Rs(e){if(e=e||(typeof document<"u"?document:void 0),typeof e>"u")return null;try{return e.activeElement||e.body}catch{return e.body}}var Gl=/[\n"\\]/g;function Aa(e){return e.replace(Gl,function(t){return"\\"+t.charCodeAt(0).toString(16)+" "})}function Mi(e,t,a,n,s,c,d,w){e.name="",d!=null&&typeof d!="function"&&typeof d!="symbol"&&typeof d!="boolean"?e.type=d:e.removeAttribute("type"),t!=null?d==="number"?(t===0&&e.value===""||e.value!=t)&&(e.value=""+pa(t)):e.value!==""+pa(t)&&(e.value=""+pa(t)):d!=="submit"&&d!=="reset"||e.removeAttribute("value"),t!=null?Fl(e,d,pa(t)):a!=null?Fl(e,d,pa(a)):n!=null&&e.removeAttribute("value"),s==null&&c!=null&&(e.defaultChecked=!!c),s!=null&&(e.checked=s&&typeof s!="function"&&typeof s!="symbol"),w!=null&&typeof w!="function"&&typeof w!="symbol"&&typeof w!="boolean"?e.name=""+pa(w):e.removeAttribute("name")}function _c(e,t,a,n,s,c,d,w){if(c!=null&&typeof c!="function"&&typeof c!="symbol"&&typeof c!="boolean"&&(e.type=c),t!=null||a!=null){if(!(c!=="submit"&&c!=="reset"||t!=null)){_o(e);return}a=a!=null?""+pa(a):"",t=t!=null?""+pa(t):a,w||t===e.value||(e.value=t),e.defaultValue=t}n=n??s,n=typeof n!="function"&&typeof n!="symbol"&&!!n,e.checked=w?e.checked:!!n,e.defaultChecked=!!n,d!=null&&typeof d!="function"&&typeof d!="symbol"&&typeof d!="boolean"&&(e.name=d),_o(e)}function Fl(e,t,a){t==="number"&&Rs(e.ownerDocument)===e||e.defaultValue===""+a||(e.defaultValue=""+a)}function oo(e,t,a,n){if(e=e.options,t){t={};for(var s=0;s<a.length;s++)t["$"+a[s]]=!0;for(a=0;a<e.length;a++)s=t.hasOwnProperty("$"+e[a].value),e[a].selected!==s&&(e[a].selected=s),s&&n&&(e[a].defaultSelected=!0)}else{for(a=""+pa(a),t=null,s=0;s<e.length;s++){if(e[s].value===a){e[s].selected=!0,n&&(e[s].defaultSelected=!0);return}t!==null||e[s].disabled||(t=e[s])}t!==null&&(t.selected=!0)}}function Ac(e,t,a){if(t!=null&&(t=""+pa(t),t!==e.value&&(e.value=t),a==null)){e.defaultValue!==t&&(e.defaultValue=t);return}e.defaultValue=a!=null?""+pa(a):""}function at(e,t,a,n){if(t==null){if(n!=null){if(a!=null)throw Error(u(92));if(le(n)){if(1<n.length)throw Error(u(93));n=n[0]}a=n}a==null&&(a=""),t=a}a=pa(t),e.defaultValue=a,n=e.textContent,n===a&&n!==""&&n!==null&&(e.value=n),_o(e)}function Ie(e,t){if(t){var a=e.firstChild;if(a&&a===e.lastChild&&a.nodeType===3){a.nodeValue=t;return}}e.textContent=t}var s0=new Set("animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp".split(" "));function Si(e,t,a){var n=t.indexOf("--")===0;a==null||typeof a=="boolean"||a===""?n?e.setProperty(t,""):t==="float"?e.cssFloat="":e[t]="":n?e.setProperty(t,a):typeof a!="number"||a===0||s0.has(t)?t==="float"?e.cssFloat=a:e[t]=(""+a).trim():e[t]=a+"px"}function gr(e,t,a){if(t!=null&&typeof t!="object")throw Error(u(62));if(e=e.style,a!=null){for(var n in a)!a.hasOwnProperty(n)||t!=null&&t.hasOwnProperty(n)||(n.indexOf("--")===0?e.setProperty(n,""):n==="float"?e.cssFloat="":e[n]="");for(var s in t)n=t[s],t.hasOwnProperty(s)&&a[s]!==n&&Si(e,s,n)}else for(var c in t)t.hasOwnProperty(c)&&Si(e,c,t[c])}function Ns(e){if(e.indexOf("-")===-1)return!1;switch(e){case"annotation-xml":case"color-profile":case"font-face":case"font-face-src":case"font-face-uri":case"font-face-format":case"font-face-name":case"missing-glyph":return!1;default:return!0}}var i0=new Map([["acceptCharset","accept-charset"],["htmlFor","for"],["httpEquiv","http-equiv"],["crossOrigin","crossorigin"],["accentHeight","accent-height"],["alignmentBaseline","alignment-baseline"],["arabicForm","arabic-form"],["baselineShift","baseline-shift"],["capHeight","cap-height"],["clipPath","clip-path"],["clipRule","clip-rule"],["colorInterpolation","color-interpolation"],["colorInterpolationFilters","color-interpolation-filters"],["colorProfile","color-profile"],["colorRendering","color-rendering"],["dominantBaseline","dominant-baseline"],["enableBackground","enable-background"],["fillOpacity","fill-opacity"],["fillRule","fill-rule"],["floodColor","flood-color"],["floodOpacity","flood-opacity"],["fontFamily","font-family"],["fontSize","font-size"],["fontSizeAdjust","font-size-adjust"],["fontStretch","font-stretch"],["fontStyle","font-style"],["fontVariant","font-variant"],["fontWeight","font-weight"],["glyphName","glyph-name"],["glyphOrientationHorizontal","glyph-orientation-horizontal"],["glyphOrientationVertical","glyph-orientation-vertical"],["horizAdvX","horiz-adv-x"],["horizOriginX","horiz-origin-x"],["imageRendering","image-rendering"],["letterSpacing","letter-spacing"],["lightingColor","lighting-color"],["markerEnd","marker-end"],["markerMid","marker-mid"],["markerStart","marker-start"],["overlinePosition","overline-position"],["overlineThickness","overline-thickness"],["paintOrder","paint-order"],["panose-1","panose-1"],["pointerEvents","pointer-events"],["renderingIntent","rendering-intent"],["shapeRendering","shape-rendering"],["stopColor","stop-color"],["stopOpacity","stop-opacity"],["strikethroughPosition","strikethrough-position"],["strikethroughThickness","strikethrough-thickness"],["strokeDasharray","stroke-dasharray"],["strokeDashoffset","stroke-dashoffset"],["strokeLinecap","stroke-linecap"],["strokeLinejoin","stroke-linejoin"],["strokeMiterlimit","stroke-miterlimit"],["strokeOpacity","stroke-opacity"],["strokeWidth","stroke-width"],["textAnchor","text-anchor"],["textDecoration","text-decoration"],["textRendering","text-rendering"],["transformOrigin","transform-origin"],["underlinePosition","underline-position"],["underlineThickness","underline-thickness"],["unicodeBidi","unicode-bidi"],["unicodeRange","unicode-range"],["unitsPerEm","units-per-em"],["vAlphabetic","v-alphabetic"],["vHanging","v-hanging"],["vIdeographic","v-ideographic"],["vMathematical","v-mathematical"],["vectorEffect","vector-effect"],["vertAdvY","vert-adv-y"],["vertOriginX","vert-origin-x"],["vertOriginY","vert-origin-y"],["wordSpacing","word-spacing"],["writingMode","writing-mode"],["xmlnsXlink","xmlns:xlink"],["xHeight","x-height"]]),yr=/^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;function lo(e){return yr.test(""+e)?"javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')":e}function cn(){}var Ti=null;function zi(e){return e=e.target||e.srcElement||window,e.correspondingUseElement&&(e=e.correspondingUseElement),e.nodeType===3?e.parentNode:e}var kn=null,so=null;function Ec(e){var t=zo(e);if(t&&(e=t.stateNode)){var a=e[ba]||null;e:switch(e=t.stateNode,t.type){case"input":if(Mi(e,a.value,a.defaultValue,a.defaultValue,a.checked,a.defaultChecked,a.type,a.name),t=a.name,a.type==="radio"&&t!=null){for(a=e;a.parentNode;)a=a.parentNode;for(a=a.querySelectorAll('input[name="'+Aa(""+t)+'"][type="radio"]'),t=0;t<a.length;t++){var n=a[t];if(n!==e&&n.form===e.form){var s=n[ba]||null;if(!s)throw Error(u(90));Mi(n,s.value,s.defaultValue,s.defaultValue,s.checked,s.defaultChecked,s.type,s.name)}}for(t=0;t<a.length;t++)n=a[t],n.form===e.form&&vr(n)}break e;case"textarea":Ac(e,a.value,a.defaultValue);break e;case"select":t=a.value,t!=null&&oo(e,!!a.multiple,t,!1)}}}var Ot=!1;function io(e,t,a){if(Ot)return e(t,a);Ot=!0;try{var n=e(t);return n}finally{if(Ot=!1,(kn!==null||so!==null)&&(hu(),kn&&(t=kn,e=so,so=kn=null,Ec(t),e)))for(t=0;t<e.length;t++)Ec(e[t])}}function Ds(e,t){var a=e.stateNode;if(a===null)return null;var n=a[ba]||null;if(n===null)return null;a=n[t];e:switch(t){case"onClick":case"onClickCapture":case"onDoubleClick":case"onDoubleClickCapture":case"onMouseDown":case"onMouseDownCapture":case"onMouseMove":case"onMouseMoveCapture":case"onMouseUp":case"onMouseUpCapture":case"onMouseEnter":(n=!n.disabled)||(e=e.type,n=!(e==="button"||e==="input"||e==="select"||e==="textarea")),e=!n;break e;default:e=!1}if(e)return null;if(a&&typeof a!="function")throw Error(u(231,t,typeof a));return a}var Bn=!(typeof window>"u"||typeof window.document>"u"||typeof window.document.createElement>"u"),Vl=!1;if(Bn)try{var Us={};Object.defineProperty(Us,"passive",{get:function(){Vl=!0}}),window.addEventListener("test",Us,Us),window.removeEventListener("test",Us,Us)}catch{Vl=!1}var Ao=null,xr=null,al=null;function Rc(){if(al)return al;var e,t=xr,a=t.length,n,s="value"in Ao?Ao.value:Ao.textContent,c=s.length;for(e=0;e<a&&t[e]===s[e];e++);var d=a-e;for(n=1;n<=d&&t[a-n]===s[c-n];n++);return al=s.slice(e,1<n?1-n:void 0)}function Os(e){var t=e.keyCode;return"charCode"in e?(e=e.charCode,e===0&&t===13&&(e=13)):e=t,e===10&&(e=13),32<=e||e===13?e:0}function Ci(){return!0}function Mn(){return!1}function ea(e){function t(a,n,s,c,d){this._reactName=a,this._targetInst=s,this.type=n,this.nativeEvent=c,this.target=d,this.currentTarget=null;for(var w in e)e.hasOwnProperty(w)&&(a=e[w],this[w]=a?a(c):c[w]);return this.isDefaultPrevented=(c.defaultPrevented!=null?c.defaultPrevented:c.returnValue===!1)?Ci:Mn,this.isPropagationStopped=Mn,this}return S(t.prototype,{preventDefault:function(){this.defaultPrevented=!0;var a=this.nativeEvent;a&&(a.preventDefault?a.preventDefault():typeof a.returnValue!="unknown"&&(a.returnValue=!1),this.isDefaultPrevented=Ci)},stopPropagation:function(){var a=this.nativeEvent;a&&(a.stopPropagation?a.stopPropagation():typeof a.cancelBubble!="unknown"&&(a.cancelBubble=!0),this.isPropagationStopped=Ci)},persist:function(){},isPersistent:Ci}),t}var Sn={eventPhase:0,bubbles:0,cancelable:0,timeStamp:function(e){return e.timeStamp||Date.now()},defaultPrevented:0,isTrusted:0},Hn=ea(Sn),nl=S({},Sn,{view:0,detail:0}),Nc=ea(nl),_i,wr,ks,Ai=S({},nl,{screenX:0,screenY:0,clientX:0,clientY:0,pageX:0,pageY:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,getModifierState:Ls,button:0,buttons:0,relatedTarget:function(e){return e.relatedTarget===void 0?e.fromElement===e.srcElement?e.toElement:e.fromElement:e.relatedTarget},movementX:function(e){return"movementX"in e?e.movementX:(e!==ks&&(ks&&e.type==="mousemove"?(_i=e.screenX-ks.screenX,wr=e.screenY-ks.screenY):wr=_i=0,ks=e),_i)},movementY:function(e){return"movementY"in e?e.movementY:wr}}),Bs=ea(Ai),Eo=S({},Ai,{dataTransfer:0}),Dc=ea(Eo),r0=S({},nl,{relatedTarget:0}),br=ea(r0),Uc=S({},Sn,{animationName:0,elapsedTime:0,pseudoElement:0}),c0=ea(Uc),u0=S({},Sn,{clipboardData:function(e){return"clipboardData"in e?e.clipboardData:window.clipboardData}}),Hs=ea(u0),Oc=S({},Sn,{data:0}),Mr=ea(Oc),f0={Esc:"Escape",Spacebar:" ",Left:"ArrowLeft",Up:"ArrowUp",Right:"ArrowRight",Down:"ArrowDown",Del:"Delete",Win:"OS",Menu:"ContextMenu",Apps:"ContextMenu",Scroll:"ScrollLock",MozPrintableKey:"Unidentified"},h0={8:"Backspace",9:"Tab",12:"Clear",13:"Enter",16:"Shift",17:"Control",18:"Alt",19:"Pause",20:"CapsLock",27:"Escape",32:" ",33:"PageUp",34:"PageDown",35:"End",36:"Home",37:"ArrowLeft",38:"ArrowUp",39:"ArrowRight",40:"ArrowDown",45:"Insert",46:"Delete",112:"F1",113:"F2",114:"F3",115:"F4",116:"F5",117:"F6",118:"F7",119:"F8",120:"F9",121:"F10",122:"F11",123:"F12",144:"NumLock",145:"ScrollLock",224:"Meta"},Kl={Alt:"altKey",Control:"ctrlKey",Meta:"metaKey",Shift:"shiftKey"};function Sr(e){var t=this.nativeEvent;return t.getModifierState?t.getModifierState(e):(e=Kl[e])?!!t[e]:!1}function Ls(){return Sr}var Tr=S({},nl,{key:function(e){if(e.key){var t=f0[e.key]||e.key;if(t!=="Unidentified")return t}return e.type==="keypress"?(e=Os(e),e===13?"Enter":String.fromCharCode(e)):e.type==="keydown"||e.type==="keyup"?h0[e.keyCode]||"Unidentified":""},code:0,location:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,repeat:0,locale:0,getModifierState:Ls,charCode:function(e){return e.type==="keypress"?Os(e):0},keyCode:function(e){return e.type==="keydown"||e.type==="keyup"?e.keyCode:0},which:function(e){return e.type==="keypress"?Os(e):e.type==="keydown"||e.type==="keyup"?e.keyCode:0}}),zr=ea(Tr),d0=S({},Ai,{pointerId:0,width:0,height:0,pressure:0,tangentialPressure:0,tiltX:0,tiltY:0,twist:0,pointerType:0,isPrimary:0}),kc=ea(d0),m0=S({},nl,{touches:0,targetTouches:0,changedTouches:0,altKey:0,metaKey:0,ctrlKey:0,shiftKey:0,getModifierState:Ls}),js=ea(m0),Cr=S({},Sn,{propertyName:0,elapsedTime:0,pseudoElement:0}),Ei=ea(Cr),p0=S({},Ai,{deltaX:function(e){return"deltaX"in e?e.deltaX:"wheelDeltaX"in e?-e.wheelDeltaX:0},deltaY:function(e){return"deltaY"in e?e.deltaY:"wheelDeltaY"in e?-e.wheelDeltaY:"wheelDelta"in e?-e.wheelDelta:0},deltaZ:0,deltaMode:0}),_r=ea(p0),v0=S({},Sn,{newState:0,oldState:0}),Bc=ea(v0),g0=[9,13,27,32],qs=Bn&&"CompositionEvent"in window,ol=null;Bn&&"documentMode"in document&&(ol=document.documentMode);var Ar=Bn&&"TextEvent"in window&&!ol,Ri=Bn&&(!qs||ol&&8<ol&&11>=ol),Ni=" ",Gs=!1;function Fs(e,t){switch(e){case"keyup":return g0.indexOf(t.keyCode)!==-1;case"keydown":return t.keyCode!==229;case"keypress":case"mousedown":case"focusout":return!0;default:return!1}}function Er(e){return e=e.detail,typeof e=="object"&&"data"in e?e.data:null}var Ro=!1;function ro(e,t){switch(e){case"compositionend":return Er(t);case"keypress":return t.which!==32?null:(Gs=!0,Ni);case"textInput":return e=t.data,e===Ni&&Gs?null:e;default:return null}}function ja(e,t){if(Ro)return e==="compositionend"||!qs&&Fs(e,t)?(e=Rc(),al=xr=Ao=null,Ro=!1,e):null;switch(e){case"paste":return null;case"keypress":if(!(t.ctrlKey||t.altKey||t.metaKey)||t.ctrlKey&&t.altKey){if(t.char&&1<t.char.length)return t.char;if(t.which)return String.fromCharCode(t.which)}return null;case"compositionend":return Ri&&t.locale!=="ko"?null:t.data;default:return null}}var Rr={color:!0,date:!0,datetime:!0,"datetime-local":!0,email:!0,month:!0,number:!0,password:!0,range:!0,search:!0,tel:!0,text:!0,time:!0,url:!0,week:!0};function Ln(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t==="input"?!!Rr[e.type]:t==="textarea"}function Yl(e,t,a,n){kn?so?so.push(n):so=[n]:kn=n,t=xu(t,"onChange"),0<t.length&&(a=new Hn("onChange","change",null,a,n),e.push({event:a,listeners:t}))}var jn=null,Vs=null;function Nr(e){mm(e,0)}function Ea(e){var t=tl(e);if(vr(t))return e}function Dr(e,t){if(e==="change")return t}var Di=!1;if(Bn){var Wl;if(Bn){var Ks="oninput"in document;if(!Ks){var Ur=document.createElement("div");Ur.setAttribute("oninput","return;"),Ks=typeof Ur.oninput=="function"}Wl=Ks}else Wl=!1;Di=Wl&&(!document.documentMode||9<document.documentMode)}function Or(){jn&&(jn.detachEvent("onpropertychange",Ui),Vs=jn=null)}function Ui(e){if(e.propertyName==="value"&&Ea(Vs)){var t=[];Yl(t,Vs,e,zi(e)),io(Nr,t)}}function kr(e,t,a){e==="focusin"?(Or(),jn=t,Vs=a,jn.attachEvent("onpropertychange",Ui)):e==="focusout"&&Or()}function No(e){if(e==="selectionchange"||e==="keyup"||e==="keydown")return Ea(Vs)}function Br(e,t){if(e==="click")return Ea(t)}function Ys(e,t){if(e==="input"||e==="change")return Ea(t)}function Ws(e,t){return e===t&&(e!==0||1/e===1/t)||e!==e&&t!==t}var Ra=typeof Object.is=="function"?Object.is:Ws;function ll(e,t){if(Ra(e,t))return!0;if(typeof e!="object"||e===null||typeof t!="object"||t===null)return!1;var a=Object.keys(e),n=Object.keys(t);if(a.length!==n.length)return!1;for(n=0;n<a.length;n++){var s=a[n];if(!Ca.call(t,s)||!Ra(e[s],t[s]))return!1}return!0}function Oi(e){for(;e&&e.firstChild;)e=e.firstChild;return e}function co(e,t){var a=Oi(e);e=0;for(var n;a;){if(a.nodeType===3){if(n=e+a.textContent.length,e<=t&&n>=t)return{node:a,offset:t-e};e=n}e:{for(;a;){if(a.nextSibling){a=a.nextSibling;break e}a=a.parentNode}a=void 0}a=Oi(a)}}function Hc(e,t){return e&&t?e===t?!0:e&&e.nodeType===3?!1:t&&t.nodeType===3?Hc(e,t.parentNode):"contains"in e?e.contains(t):e.compareDocumentPosition?!!(e.compareDocumentPosition(t)&16):!1:!1}function Lc(e){e=e!=null&&e.ownerDocument!=null&&e.ownerDocument.defaultView!=null?e.ownerDocument.defaultView:window;for(var t=Rs(e.document);t instanceof e.HTMLIFrameElement;){try{var a=typeof t.contentWindow.location.href=="string"}catch{a=!1}if(a)e=t.contentWindow;else break;t=Rs(e.document)}return t}function Xs(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t&&(t==="input"&&(e.type==="text"||e.type==="search"||e.type==="tel"||e.type==="url"||e.type==="password")||t==="textarea"||e.contentEditable==="true")}var jc=Bn&&"documentMode"in document&&11>=document.documentMode,sl=null,Hr=null,uo=null,Lr=!1;function jr(e,t,a){var n=a.window===a?a.document:a.nodeType===9?a:a.ownerDocument;Lr||sl==null||sl!==Rs(n)||(n=sl,"selectionStart"in n&&Xs(n)?n={start:n.selectionStart,end:n.selectionEnd}:(n=(n.ownerDocument&&n.ownerDocument.defaultView||window).getSelection(),n={anchorNode:n.anchorNode,anchorOffset:n.anchorOffset,focusNode:n.focusNode,focusOffset:n.focusOffset}),uo&&ll(uo,n)||(uo=n,n=xu(Hr,"onSelect"),0<n.length&&(t=new Hn("onSelect","select",null,t,a),e.push({event:t,listeners:n}),t.target=sl)))}function Do(e,t){var a={};return a[e.toLowerCase()]=t.toLowerCase(),a["Webkit"+e]="webkit"+t,a["Moz"+e]="moz"+t,a}var Uo={animationend:Do("Animation","AnimationEnd"),animationiteration:Do("Animation","AnimationIteration"),animationstart:Do("Animation","AnimationStart"),transitionrun:Do("Transition","TransitionRun"),transitionstart:Do("Transition","TransitionStart"),transitioncancel:Do("Transition","TransitionCancel"),transitionend:Do("Transition","TransitionEnd")},qr={},Gr={};Bn&&(Gr=document.createElement("div").style,"AnimationEvent"in window||(delete Uo.animationend.animation,delete Uo.animationiteration.animation,delete Uo.animationstart.animation),"TransitionEvent"in window||delete Uo.transitionend.transition);function Oo(e){if(qr[e])return qr[e];if(!Uo[e])return e;var t=Uo[e],a;for(a in t)if(t.hasOwnProperty(a)&&a in Gr)return qr[e]=t[a];return e}var qc=Oo("animationend"),ki=Oo("animationiteration"),Ps=Oo("animationstart"),Gc=Oo("transitionrun"),Fc=Oo("transitionstart"),Fr=Oo("transitioncancel"),Bi=Oo("transitionend"),Zs=new Map,ko="abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel".split(" ");ko.push("scrollEnd");function un(e,t){Zs.set(e,t),wn(t,[e])}var Xl=typeof reportError=="function"?reportError:function(e){if(typeof window=="object"&&typeof window.ErrorEvent=="function"){var t=new window.ErrorEvent("error",{bubbles:!0,cancelable:!0,message:typeof e=="object"&&e!==null&&typeof e.message=="string"?String(e.message):String(e),error:e});if(!window.dispatchEvent(t))return}else if(typeof process=="object"&&typeof process.emit=="function"){process.emit("uncaughtException",e);return}console.error(e)},Na=[],il=0,Pl=0;function rl(){for(var e=il,t=Pl=il=0;t<e;){var a=Na[t];Na[t++]=null;var n=Na[t];Na[t++]=null;var s=Na[t];Na[t++]=null;var c=Na[t];if(Na[t++]=null,n!==null&&s!==null){var d=n.pending;d===null?s.next=s:(s.next=d.next,d.next=s),n.pending=s}c!==0&&Bo(a,s,c)}}function Zl(e,t,a,n){Na[il++]=e,Na[il++]=t,Na[il++]=a,Na[il++]=n,Pl|=n,e.lanes|=n,e=e.alternate,e!==null&&(e.lanes|=n)}function Qs(e,t,a,n){return Zl(e,t,a,n),Ql(e)}function qn(e,t){return Zl(e,null,null,t),Ql(e)}function Bo(e,t,a){e.lanes|=a;var n=e.alternate;n!==null&&(n.lanes|=a);for(var s=!1,c=e.return;c!==null;)c.childLanes|=a,n=c.alternate,n!==null&&(n.childLanes|=a),c.tag===22&&(e=c.stateNode,e===null||e._visibility&1||(s=!0)),e=c,c=c.return;return e.tag===3?(c=e.stateNode,s&&t!==null&&(s=31-Vt(a),e=c.hiddenUpdates,n=e[s],n===null?e[s]=[t]:n.push(t),t.lane=a|536870912),c):null}function Ql(e){if(50<ac)throw ac=0,of=null,Error(u(185));for(var t=e.return;t!==null;)e=t,t=e.return;return e.tag===3?e.stateNode:null}var cl={};function y0(e,t,a,n){this.tag=e,this.key=a,this.sibling=this.child=this.return=this.stateNode=this.type=this.elementType=null,this.index=0,this.refCleanup=this.ref=null,this.pendingProps=t,this.dependencies=this.memoizedState=this.updateQueue=this.memoizedProps=null,this.mode=n,this.subtreeFlags=this.flags=0,this.deletions=null,this.childLanes=this.lanes=0,this.alternate=null}function va(e,t,a,n){return new y0(e,t,a,n)}function Is(e){return e=e.prototype,!(!e||!e.isReactComponent)}function fo(e,t){var a=e.alternate;return a===null?(a=va(e.tag,t,e.key,e.mode),a.elementType=e.elementType,a.type=e.type,a.stateNode=e.stateNode,a.alternate=e,e.alternate=a):(a.pendingProps=t,a.type=e.type,a.flags=0,a.subtreeFlags=0,a.deletions=null),a.flags=e.flags&65011712,a.childLanes=e.childLanes,a.lanes=e.lanes,a.child=e.child,a.memoizedProps=e.memoizedProps,a.memoizedState=e.memoizedState,a.updateQueue=e.updateQueue,t=e.dependencies,a.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext},a.sibling=e.sibling,a.index=e.index,a.ref=e.ref,a.refCleanup=e.refCleanup,a}function Hi(e,t){e.flags&=65011714;var a=e.alternate;return a===null?(e.childLanes=0,e.lanes=t,e.child=null,e.subtreeFlags=0,e.memoizedProps=null,e.memoizedState=null,e.updateQueue=null,e.dependencies=null,e.stateNode=null):(e.childLanes=a.childLanes,e.lanes=a.lanes,e.child=a.child,e.subtreeFlags=0,e.deletions=null,e.memoizedProps=a.memoizedProps,e.memoizedState=a.memoizedState,e.updateQueue=a.updateQueue,e.type=a.type,t=a.dependencies,e.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext}),e}function ul(e,t,a,n,s,c){var d=0;if(n=e,typeof e=="function")Is(e)&&(d=1);else if(typeof e=="string")d=xv(e,a,F.current)?26:e==="html"||e==="head"||e==="body"?27:5;else e:switch(e){case ne:return e=va(31,a,t,s),e.elementType=ne,e.lanes=c,e;case H:return ho(a.children,s,c,t);case q:d=8,s|=24;break;case P:return e=va(12,a,t,s|2),e.elementType=P,e.lanes=c,e;case R:return e=va(13,a,t,s),e.elementType=R,e.lanes=c,e;case V:return e=va(19,a,t,s),e.elementType=V,e.lanes=c,e;default:if(typeof e=="object"&&e!==null)switch(e.$$typeof){case U:d=10;break e;case K:d=9;break e;case z:d=11;break e;case J:d=14;break e;case Q:d=16,n=null;break e}d=29,a=Error(u(130,e===null?"null":typeof e,"")),n=null}return t=va(d,a,t,s),t.elementType=e,t.type=n,t.lanes=c,t}function ho(e,t,a,n){return e=va(7,e,n,t),e.lanes=a,e}function Js(e,t,a){return e=va(6,e,null,t),e.lanes=a,e}function Vc(e){var t=va(18,null,null,0);return t.stateNode=e,t}function $s(e,t,a){return t=va(4,e.children!==null?e.children:[],e.key,t),t.lanes=a,t.stateNode={containerInfo:e.containerInfo,pendingChildren:null,implementation:e.implementation},t}var Li=new WeakMap;function qa(e,t){if(typeof e=="object"&&e!==null){var a=Li.get(e);return a!==void 0?a:(t={value:e,source:t,stack:fa(t)},Li.set(e,t),t)}return{value:e,source:t,stack:fa(t)}}var fl=[],hl=0,ei=null,Il=0,Ia=[],fn=0,Ho=null,Gn=1,Fn="";function Tn(e,t){fl[hl++]=Il,fl[hl++]=ei,ei=e,Il=t}function ti(e,t,a){Ia[fn++]=Gn,Ia[fn++]=Fn,Ia[fn++]=Ho,Ho=e;var n=Gn;e=Fn;var s=32-Vt(n)-1;n&=~(1<<s),a+=1;var c=32-Vt(t)+s;if(30<c){var d=s-s%5;c=(n&(1<<d)-1).toString(32),n>>=d,s-=d,Gn=1<<32-Vt(t)+s|a<<s|n,Fn=c+e}else Gn=1<<c|a<<s|n,Fn=e}function Kt(e){e.return!==null&&(Tn(e,1),ti(e,1,0))}function Jl(e){for(;e===ei;)ei=fl[--hl],fl[hl]=null,Il=fl[--hl],fl[hl]=null;for(;e===Ho;)Ho=Ia[--fn],Ia[fn]=null,Fn=Ia[--fn],Ia[fn]=null,Gn=Ia[--fn],Ia[fn]=null}function Vr(e,t){Ia[fn++]=Gn,Ia[fn++]=Fn,Ia[fn++]=Ho,Gn=t.id,Fn=t.overflow,Ho=e}var Yt=null,kt=null,mt=!1,Vn=null,Da=!1,ai=Error(u(519));function mo(e){var t=Error(u(418,1<arguments.length&&arguments[1]!==void 0&&arguments[1]?"text":"HTML",""));throw ts(qa(t,e)),ai}function $l(e){var t=e.stateNode,a=e.type,n=e.memoizedProps;switch(t[ra]=e,t[ba]=n,a){case"dialog":yt("cancel",t),yt("close",t);break;case"iframe":case"object":case"embed":yt("load",t);break;case"video":case"audio":for(a=0;a<oc.length;a++)yt(oc[a],t);break;case"source":yt("error",t);break;case"img":case"image":case"link":yt("error",t),yt("load",t);break;case"details":yt("toggle",t);break;case"input":yt("invalid",t),_c(t,n.value,n.defaultValue,n.checked,n.defaultChecked,n.type,n.name,!0);break;case"select":yt("invalid",t);break;case"textarea":yt("invalid",t),at(t,n.value,n.defaultValue,n.children)}a=n.children,typeof a!="string"&&typeof a!="number"&&typeof a!="bigint"||t.textContent===""+a||n.suppressHydrationWarning===!0||ym(t.textContent,a)?(n.popover!=null&&(yt("beforetoggle",t),yt("toggle",t)),n.onScroll!=null&&yt("scroll",t),n.onScrollEnd!=null&&yt("scrollend",t),n.onClick!=null&&(t.onclick=cn),t=!0):t=!1,t||mo(e,!0)}function es(e){for(Yt=e.return;Yt;)switch(Yt.tag){case 5:case 31:case 13:Da=!1;return;case 27:case 3:Da=!0;return;default:Yt=Yt.return}}function zn(e){if(e!==Yt)return!1;if(!mt)return es(e),mt=!0,!1;var t=e.tag,a;if((a=t!==3&&t!==27)&&((a=t===5)&&(a=e.type,a=!(a!=="form"&&a!=="button")||wf(e.type,e.memoizedProps)),a=!a),a&&kt&&mo(e),es(e),t===13){if(e=e.memoizedState,e=e!==null?e.dehydrated:null,!e)throw Error(u(317));kt=_m(e)}else if(t===31){if(e=e.memoizedState,e=e!==null?e.dehydrated:null,!e)throw Error(u(317));kt=_m(e)}else t===27?(t=kt,hs(e.type)?(e=zf,zf=null,kt=e):kt=t):kt=Yt?Qn(e.stateNode.nextSibling):null;return!0}function dl(){kt=Yt=null,mt=!1}function ni(){var e=Vn;return e!==null&&(vn===null?vn=e:vn.push.apply(vn,e),Vn=null),e}function ts(e){Vn===null?Vn=[e]:Vn.push(e)}var ji=E(null),Ga=null,Ma=null;function Kn(e,t,a){T(ji,t._currentValue),t._currentValue=a}function Yn(e){e._currentValue=ji.current,Z(ji)}function as(e,t,a){for(;e!==null;){var n=e.alternate;if((e.childLanes&t)!==t?(e.childLanes|=t,n!==null&&(n.childLanes|=t)):n!==null&&(n.childLanes&t)!==t&&(n.childLanes|=t),e===a)break;e=e.return}}function oi(e,t,a,n){var s=e.child;for(s!==null&&(s.return=e);s!==null;){var c=s.dependencies;if(c!==null){var d=s.child;c=c.firstContext;e:for(;c!==null;){var w=c;c=s;for(var L=0;L<t.length;L++)if(w.context===t[L]){c.lanes|=a,w=c.alternate,w!==null&&(w.lanes|=a),as(c.return,a,e),n||(d=null);break e}c=w.next}}else if(s.tag===18){if(d=s.return,d===null)throw Error(u(341));d.lanes|=a,c=d.alternate,c!==null&&(c.lanes|=a),as(d,a,e),d=null}else d=s.child;if(d!==null)d.return=s;else for(d=s;d!==null;){if(d===e){d=null;break}if(s=d.sibling,s!==null){s.return=d.return,d=s;break}d=d.return}s=d}}function Cn(e,t,a,n){e=null;for(var s=t,c=!1;s!==null;){if(!c){if((s.flags&524288)!==0)c=!0;else if((s.flags&262144)!==0)break}if(s.tag===10){var d=s.alternate;if(d===null)throw Error(u(387));if(d=d.memoizedProps,d!==null){var w=s.type;Ra(s.pendingProps.value,d.value)||(e!==null?e.push(w):e=[w])}}else if(s===Ee.current){if(d=s.alternate,d===null)throw Error(u(387));d.memoizedState.memoizedState!==s.memoizedState.memoizedState&&(e!==null?e.push(cc):e=[cc])}s=s.return}e!==null&&oi(t,e,a,n),t.flags|=262144}function li(e){for(e=e.firstContext;e!==null;){if(!Ra(e.context._currentValue,e.memoizedValue))return!0;e=e.next}return!1}function Lo(e){Ga=e,Ma=null,e=e.dependencies,e!==null&&(e.firstContext=null)}function na(e){return Kc(Ga,e)}function si(e,t){return Ga===null&&Lo(e),Kc(e,t)}function Kc(e,t){var a=t._currentValue;if(t={context:t,memoizedValue:a,next:null},Ma===null){if(e===null)throw Error(u(308));Ma=t,e.dependencies={lanes:0,firstContext:t},e.flags|=524288}else Ma=Ma.next=t;return a}var qi=typeof AbortController<"u"?AbortController:function(){var e=[],t=this.signal={aborted:!1,addEventListener:function(a,n){e.push(n)}};this.abort=function(){t.aborted=!0,e.forEach(function(a){return a()})}},Yc=o.unstable_scheduleCallback,Wc=o.unstable_NormalPriority,Wt={$$typeof:U,Consumer:null,Provider:null,_currentValue:null,_currentValue2:null,_threadCount:0};function Gi(){return{controller:new qi,data:new Map,refCount:0}}function ns(e){e.refCount--,e.refCount===0&&Yc(Wc,function(){e.controller.abort()})}var ii=null,Fi=0,Ja=0,ml=null;function Kr(e,t){if(ii===null){var a=ii=[];Fi=0,Ja=ff(),ml={status:"pending",value:void 0,then:function(n){a.push(n)}}}return Fi++,t.then(hn,hn),t}function hn(){if(--Fi===0&&ii!==null){ml!==null&&(ml.status="fulfilled");var e=ii;ii=null,Ja=0,ml=null;for(var t=0;t<e.length;t++)(0,e[t])()}}function Xc(e,t){var a=[],n={status:"pending",value:null,reason:null,then:function(s){a.push(s)}};return e.then(function(){n.status="fulfilled",n.value=t;for(var s=0;s<a.length;s++)(0,a[s])(t)},function(s){for(n.status="rejected",n.reason=s,s=0;s<a.length;s++)(0,a[s])(void 0)}),n}var Pc=G.S;G.S=function(e,t){Fd=qt(),typeof t=="object"&&t!==null&&typeof t.then=="function"&&Kr(e,t),Pc!==null&&Pc(e,t)};var jo=E(null);function Vi(){var e=jo.current;return e!==null?e:Ft.pooledCache}function g(e,t){t===null?T(jo,jo.current):T(jo,t.pool)}function y(){var e=Vi();return e===null?null:{parent:Wt._currentValue,pool:e}}var M=Error(u(460)),_=Error(u(474)),N=Error(u(542)),k={then:function(){}};function A(e){return e=e.status,e==="fulfilled"||e==="rejected"}function O(e,t,a){switch(a=e[a],a===void 0?e.push(t):a!==t&&(t.then(cn,cn),t=a),t.status){case"fulfilled":return t.value;case"rejected":throw e=t.reason,de(e),e;default:if(typeof t.status=="string")t.then(cn,cn);else{if(e=Ft,e!==null&&100<e.shellSuspendCounter)throw Error(u(482));e=t,e.status="pending",e.then(function(n){if(t.status==="pending"){var s=t;s.status="fulfilled",s.value=n}},function(n){if(t.status==="pending"){var s=t;s.status="rejected",s.reason=n}})}switch(t.status){case"fulfilled":return t.value;case"rejected":throw e=t.reason,de(e),e}throw ee=t,M}}function Y(e){try{var t=e._init;return t(e._payload)}catch(a){throw a!==null&&typeof a=="object"&&typeof a.then=="function"?(ee=a,M):a}}var ee=null;function te(){if(ee===null)throw Error(u(459));var e=ee;return ee=null,e}function de(e){if(e===M||e===N)throw Error(u(483))}var pe=null,fe=0;function ye(e){var t=fe;return fe+=1,pe===null&&(pe=[]),O(pe,e,t)}function Se(e,t){t=t.props.ref,e.ref=t!==void 0?t:null}function xe(e,t){throw t.$$typeof===C?Error(u(525)):(e=Object.prototype.toString.call(t),Error(u(31,e==="[object Object]"?"object with keys {"+Object.keys(t).join(", ")+"}":e)))}function Ne(e){function t($,W){if(e){var ie=$.deletions;ie===null?($.deletions=[W],$.flags|=16):ie.push(W)}}function a($,W){if(!e)return null;for(;W!==null;)t($,W),W=W.sibling;return null}function n($){for(var W=new Map;$!==null;)$.key!==null?W.set($.key,$):W.set($.index,$),$=$.sibling;return W}function s($,W){return $=fo($,W),$.index=0,$.sibling=null,$}function c($,W,ie){return $.index=ie,e?(ie=$.alternate,ie!==null?(ie=ie.index,ie<W?($.flags|=67108866,W):ie):($.flags|=67108866,W)):($.flags|=1048576,W)}function d($){return e&&$.alternate===null&&($.flags|=67108866),$}function w($,W,ie,Me){return W===null||W.tag!==6?(W=Js(ie,$.mode,Me),W.return=$,W):(W=s(W,ie),W.return=$,W)}function L($,W,ie,Me){var Pe=ie.type;return Pe===H?be($,W,ie.props.children,Me,ie.key):W!==null&&(W.elementType===Pe||typeof Pe=="object"&&Pe!==null&&Pe.$$typeof===Q&&Y(Pe)===W.type)?(W=s(W,ie.props),Se(W,ie),W.return=$,W):(W=ul(ie.type,ie.key,ie.props,null,$.mode,Me),Se(W,ie),W.return=$,W)}function re($,W,ie,Me){return W===null||W.tag!==4||W.stateNode.containerInfo!==ie.containerInfo||W.stateNode.implementation!==ie.implementation?(W=$s(ie,$.mode,Me),W.return=$,W):(W=s(W,ie.children||[]),W.return=$,W)}function be($,W,ie,Me,Pe){return W===null||W.tag!==7?(W=ho(ie,$.mode,Me,Pe),W.return=$,W):(W=s(W,ie),W.return=$,W)}function Ce($,W,ie){if(typeof W=="string"&&W!==""||typeof W=="number"||typeof W=="bigint")return W=Js(""+W,$.mode,ie),W.return=$,W;if(typeof W=="object"&&W!==null){switch(W.$$typeof){case x:return ie=ul(W.type,W.key,W.props,null,$.mode,ie),Se(ie,W),ie.return=$,ie;case D:return W=$s(W,$.mode,ie),W.return=$,W;case Q:return W=Y(W),Ce($,W,ie)}if(le(W)||I(W))return W=ho(W,$.mode,ie,null),W.return=$,W;if(typeof W.then=="function")return Ce($,ye(W),ie);if(W.$$typeof===U)return Ce($,si($,W),ie);xe($,W)}return null}function he($,W,ie,Me){var Pe=W!==null?W.key:null;if(typeof ie=="string"&&ie!==""||typeof ie=="number"||typeof ie=="bigint")return Pe!==null?null:w($,W,""+ie,Me);if(typeof ie=="object"&&ie!==null){switch(ie.$$typeof){case x:return ie.key===Pe?L($,W,ie,Me):null;case D:return ie.key===Pe?re($,W,ie,Me):null;case Q:return ie=Y(ie),he($,W,ie,Me)}if(le(ie)||I(ie))return Pe!==null?null:be($,W,ie,Me,null);if(typeof ie.then=="function")return he($,W,ye(ie),Me);if(ie.$$typeof===U)return he($,W,si($,ie),Me);xe($,ie)}return null}function ve($,W,ie,Me,Pe){if(typeof Me=="string"&&Me!==""||typeof Me=="number"||typeof Me=="bigint")return $=$.get(ie)||null,w(W,$,""+Me,Pe);if(typeof Me=="object"&&Me!==null){switch(Me.$$typeof){case x:return $=$.get(Me.key===null?ie:Me.key)||null,L(W,$,Me,Pe);case D:return $=$.get(Me.key===null?ie:Me.key)||null,re(W,$,Me,Pe);case Q:return Me=Y(Me),ve($,W,ie,Me,Pe)}if(le(Me)||I(Me))return $=$.get(ie)||null,be(W,$,Me,Pe,null);if(typeof Me.then=="function")return ve($,W,ie,ye(Me),Pe);if(Me.$$typeof===U)return ve($,W,ie,si(W,Me),Pe);xe(W,Me)}return null}function Fe($,W,ie,Me){for(var Pe=null,_t=null,We=W,ht=W=0,Mt=null;We!==null&&ht<ie.length;ht++){We.index>ht?(Mt=We,We=null):Mt=We.sibling;var At=he($,We,ie[ht],Me);if(At===null){We===null&&(We=Mt);break}e&&We&&At.alternate===null&&t($,We),W=c(At,W,ht),_t===null?Pe=At:_t.sibling=At,_t=At,We=Mt}if(ht===ie.length)return a($,We),mt&&Tn($,ht),Pe;if(We===null){for(;ht<ie.length;ht++)We=Ce($,ie[ht],Me),We!==null&&(W=c(We,W,ht),_t===null?Pe=We:_t.sibling=We,_t=We);return mt&&Tn($,ht),Pe}for(We=n(We);ht<ie.length;ht++)Mt=ve(We,$,ht,ie[ht],Me),Mt!==null&&(e&&Mt.alternate!==null&&We.delete(Mt.key===null?ht:Mt.key),W=c(Mt,W,ht),_t===null?Pe=Mt:_t.sibling=Mt,_t=Mt);return e&&We.forEach(function(gs){return t($,gs)}),mt&&Tn($,ht),Pe}function Je($,W,ie,Me){if(ie==null)throw Error(u(151));for(var Pe=null,_t=null,We=W,ht=W=0,Mt=null,At=ie.next();We!==null&&!At.done;ht++,At=ie.next()){We.index>ht?(Mt=We,We=null):Mt=We.sibling;var gs=he($,We,At.value,Me);if(gs===null){We===null&&(We=Mt);break}e&&We&&gs.alternate===null&&t($,We),W=c(gs,W,ht),_t===null?Pe=gs:_t.sibling=gs,_t=gs,We=Mt}if(At.done)return a($,We),mt&&Tn($,ht),Pe;if(We===null){for(;!At.done;ht++,At=ie.next())At=Ce($,At.value,Me),At!==null&&(W=c(At,W,ht),_t===null?Pe=At:_t.sibling=At,_t=At);return mt&&Tn($,ht),Pe}for(We=n(We);!At.done;ht++,At=ie.next())At=ve(We,$,ht,At.value,Me),At!==null&&(e&&At.alternate!==null&&We.delete(At.key===null?ht:At.key),W=c(At,W,ht),_t===null?Pe=At:_t.sibling=At,_t=At);return e&&We.forEach(function(Rv){return t($,Rv)}),mt&&Tn($,ht),Pe}function jt($,W,ie,Me){if(typeof ie=="object"&&ie!==null&&ie.type===H&&ie.key===null&&(ie=ie.props.children),typeof ie=="object"&&ie!==null){switch(ie.$$typeof){case x:e:{for(var Pe=ie.key;W!==null;){if(W.key===Pe){if(Pe=ie.type,Pe===H){if(W.tag===7){a($,W.sibling),Me=s(W,ie.props.children),Me.return=$,$=Me;break e}}else if(W.elementType===Pe||typeof Pe=="object"&&Pe!==null&&Pe.$$typeof===Q&&Y(Pe)===W.type){a($,W.sibling),Me=s(W,ie.props),Se(Me,ie),Me.return=$,$=Me;break e}a($,W);break}else t($,W);W=W.sibling}ie.type===H?(Me=ho(ie.props.children,$.mode,Me,ie.key),Me.return=$,$=Me):(Me=ul(ie.type,ie.key,ie.props,null,$.mode,Me),Se(Me,ie),Me.return=$,$=Me)}return d($);case D:e:{for(Pe=ie.key;W!==null;){if(W.key===Pe)if(W.tag===4&&W.stateNode.containerInfo===ie.containerInfo&&W.stateNode.implementation===ie.implementation){a($,W.sibling),Me=s(W,ie.children||[]),Me.return=$,$=Me;break e}else{a($,W);break}else t($,W);W=W.sibling}Me=$s(ie,$.mode,Me),Me.return=$,$=Me}return d($);case Q:return ie=Y(ie),jt($,W,ie,Me)}if(le(ie))return Fe($,W,ie,Me);if(I(ie)){if(Pe=I(ie),typeof Pe!="function")throw Error(u(150));return ie=Pe.call(ie),Je($,W,ie,Me)}if(typeof ie.then=="function")return jt($,W,ye(ie),Me);if(ie.$$typeof===U)return jt($,W,si($,ie),Me);xe($,ie)}return typeof ie=="string"&&ie!==""||typeof ie=="number"||typeof ie=="bigint"?(ie=""+ie,W!==null&&W.tag===6?(a($,W.sibling),Me=s(W,ie),Me.return=$,$=Me):(a($,W),Me=Js(ie,$.mode,Me),Me.return=$,$=Me),d($)):a($,W)}return function($,W,ie,Me){try{fe=0;var Pe=jt($,W,ie,Me);return pe=null,Pe}catch(We){if(We===M||We===N)throw We;var _t=va(29,We,null,$.mode);return _t.lanes=Me,_t.return=$,_t}finally{}}}var Oe=Ne(!0),ze=Ne(!1),Ae=!1;function He(e){e.updateQueue={baseState:e.memoizedState,firstBaseUpdate:null,lastBaseUpdate:null,shared:{pending:null,lanes:0,hiddenCallbacks:null},callbacks:null}}function ke(e,t){e=e.updateQueue,t.updateQueue===e&&(t.updateQueue={baseState:e.baseState,firstBaseUpdate:e.firstBaseUpdate,lastBaseUpdate:e.lastBaseUpdate,shared:e.shared,callbacks:null})}function Le(e){return{lane:e,tag:0,payload:null,callback:null,next:null}}function je(e,t,a){var n=e.updateQueue;if(n===null)return null;if(n=n.shared,(Dt&2)!==0){var s=n.pending;return s===null?t.next=t:(t.next=s.next,s.next=t),n.pending=t,t=Ql(e),Bo(e,null,a),t}return Zl(e,n,t,a),Ql(e)}function De(e,t,a){if(t=t.updateQueue,t!==null&&(t=t.shared,(a&4194048)!==0)){var n=t.lanes;n&=e.pendingLanes,a|=n,t.lanes=a,gi(e,a)}}function Be(e,t){var a=e.updateQueue,n=e.alternate;if(n!==null&&(n=n.updateQueue,a===n)){var s=null,c=null;if(a=a.firstBaseUpdate,a!==null){do{var d={lane:a.lane,tag:a.tag,payload:a.payload,callback:null,next:null};c===null?s=c=d:c=c.next=d,a=a.next}while(a!==null);c===null?s=c=t:c=c.next=t}else s=c=t;a={baseState:n.baseState,firstBaseUpdate:s,lastBaseUpdate:c,shared:n.shared,callbacks:n.callbacks},e.updateQueue=a;return}e=a.lastBaseUpdate,e===null?a.firstBaseUpdate=t:e.next=t,a.lastBaseUpdate=t}var Ve=!1;function qe(){if(Ve){var e=ml;if(e!==null)throw e}}function Qe(e,t,a,n){Ve=!1;var s=e.updateQueue;Ae=!1;var c=s.firstBaseUpdate,d=s.lastBaseUpdate,w=s.shared.pending;if(w!==null){s.shared.pending=null;var L=w,re=L.next;L.next=null,d===null?c=re:d.next=re,d=L;var be=e.alternate;be!==null&&(be=be.updateQueue,w=be.lastBaseUpdate,w!==d&&(w===null?be.firstBaseUpdate=re:w.next=re,be.lastBaseUpdate=L))}if(c!==null){var Ce=s.baseState;d=0,be=re=L=null,w=c;do{var he=w.lane&-536870913,ve=he!==w.lane;if(ve?(bt&he)===he:(n&he)===he){he!==0&&he===Ja&&(Ve=!0),be!==null&&(be=be.next={lane:0,tag:w.tag,payload:w.payload,callback:null,next:null});e:{var Fe=e,Je=w;he=t;var jt=a;switch(Je.tag){case 1:if(Fe=Je.payload,typeof Fe=="function"){Ce=Fe.call(jt,Ce,he);break e}Ce=Fe;break e;case 3:Fe.flags=Fe.flags&-65537|128;case 0:if(Fe=Je.payload,he=typeof Fe=="function"?Fe.call(jt,Ce,he):Fe,he==null)break e;Ce=S({},Ce,he);break e;case 2:Ae=!0}}he=w.callback,he!==null&&(e.flags|=64,ve&&(e.flags|=8192),ve=s.callbacks,ve===null?s.callbacks=[he]:ve.push(he))}else ve={lane:he,tag:w.tag,payload:w.payload,callback:w.callback,next:null},be===null?(re=be=ve,L=Ce):be=be.next=ve,d|=he;if(w=w.next,w===null){if(w=s.shared.pending,w===null)break;ve=w,w=ve.next,ve.next=null,s.lastBaseUpdate=ve,s.shared.pending=null}}while(!0);be===null&&(L=Ce),s.baseState=L,s.firstBaseUpdate=re,s.lastBaseUpdate=be,c===null&&(s.shared.lanes=0),is|=d,e.lanes=d,e.memoizedState=Ce}}function it(e,t){if(typeof e!="function")throw Error(u(191,e));e.call(t)}function $e(e,t){var a=e.callbacks;if(a!==null)for(e.callbacks=null,e=0;e<a.length;e++)it(a[e],t)}var nt=E(null),vt=E(0);function Ct(e,t){e=Tl,T(vt,e),T(nt,t),Tl=e|t.baseLanes}function Rt(){T(vt,Tl),T(nt,nt.current)}function It(){Tl=vt.current,Z(nt),Z(vt)}var wt=E(null),St=null;function Xt(e){var t=e.alternate;T(Tt,Tt.current&1),T(wt,e),St===null&&(t===null||nt.current!==null||t.memoizedState!==null)&&(St=e)}function $a(e){T(Tt,Tt.current),T(wt,e),St===null&&(St=e)}function en(e){e.tag===22?(T(Tt,Tt.current),T(wt,e),St===null&&(St=e)):oa()}function oa(){T(Tt,Tt.current),T(wt,wt.current)}function Gt(e){Z(wt),St===e&&(St=null),Z(Tt)}var Tt=E(0);function Wn(e){for(var t=e;t!==null;){if(t.tag===13){var a=t.memoizedState;if(a!==null&&(a=a.dehydrated,a===null||Sf(a)||Tf(a)))return t}else if(t.tag===19&&(t.memoizedProps.revealOrder==="forwards"||t.memoizedProps.revealOrder==="backwards"||t.memoizedProps.revealOrder==="unstable_legacy-backwards"||t.memoizedProps.revealOrder==="together")){if((t.flags&128)!==0)return t}else if(t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return null;t=t.return}t.sibling.return=t.return,t=t.sibling}return null}var Sa=0,et=null,ft=null,Nt=null,pl=!1,vl=!1,Xn=!1,po=0,Pn=0,_n=null,os=0;function Pt(){throw Error(u(321))}function Ki(e,t){if(t===null)return!1;for(var a=0;a<t.length&&a<e.length;a++)if(!Ra(e[a],t[a]))return!1;return!0}function Zn(e,t,a,n,s,c){return Sa=c,et=t,t.memoizedState=null,t.updateQueue=null,t.lanes=0,G.H=e===null||e.memoizedState===null?$h:U0,Xn=!1,c=a(n,s),Xn=!1,vl&&(c=Wi(t,a,n,s)),Yi(e),c}function Yi(e){G.H=Xr;var t=ft!==null&&ft.next!==null;if(Sa=0,Nt=ft=et=null,pl=!1,Pn=0,_n=null,t)throw Error(u(300));e===null||ha||(e=e.dependencies,e!==null&&li(e)&&(ha=!0))}function Wi(e,t,a,n){et=e;var s=0;do{if(vl&&(_n=null),Pn=0,vl=!1,25<=s)throw Error(u(301));if(s+=1,Nt=ft=null,e.updateQueue!=null){var c=e.updateQueue;c.lastEffect=null,c.events=null,c.stores=null,c.memoCache!=null&&(c.memoCache.index=0)}G.H=ed,c=t(a,n)}while(vl);return c}function bp(){var e=G.H,t=e.useState()[0];return t=typeof t.then=="function"?Yr(t):t,e=e.useState()[0],(ft!==null?ft.memoizedState:null)!==e&&(et.flags|=1024),t}function x0(){var e=po!==0;return po=0,e}function w0(e,t,a){t.updateQueue=e.updateQueue,t.flags&=-2053,e.lanes&=~a}function b0(e){if(pl){for(e=e.memoizedState;e!==null;){var t=e.queue;t!==null&&(t.pending=null),e=e.next}pl=!1}Sa=0,Nt=ft=et=null,vl=!1,Pn=po=0,_n=null}function tn(){var e={memoizedState:null,baseState:null,baseQueue:null,queue:null,next:null};return Nt===null?et.memoizedState=Nt=e:Nt=Nt.next=e,Nt}function ca(){if(ft===null){var e=et.alternate;e=e!==null?e.memoizedState:null}else e=ft.next;var t=Nt===null?et.memoizedState:Nt.next;if(t!==null)Nt=t,ft=e;else{if(e===null)throw et.alternate===null?Error(u(467)):Error(u(310));ft=e,e={memoizedState:ft.memoizedState,baseState:ft.baseState,baseQueue:ft.baseQueue,queue:ft.queue,next:null},Nt===null?et.memoizedState=Nt=e:Nt=Nt.next=e}return Nt}function Zc(){return{lastEffect:null,events:null,stores:null,memoCache:null}}function Yr(e){var t=Pn;return Pn+=1,_n===null&&(_n=[]),e=O(_n,e,t),t=et,(Nt===null?t.memoizedState:Nt.next)===null&&(t=t.alternate,G.H=t===null||t.memoizedState===null?$h:U0),e}function Qc(e){if(e!==null&&typeof e=="object"){if(typeof e.then=="function")return Yr(e);if(e.$$typeof===U)return na(e)}throw Error(u(438,String(e)))}function M0(e){var t=null,a=et.updateQueue;if(a!==null&&(t=a.memoCache),t==null){var n=et.alternate;n!==null&&(n=n.updateQueue,n!==null&&(n=n.memoCache,n!=null&&(t={data:n.data.map(function(s){return s.slice()}),index:0})))}if(t==null&&(t={data:[],index:0}),a===null&&(a=Zc(),et.updateQueue=a),a.memoCache=t,a=t.data[t.index],a===void 0)for(a=t.data[t.index]=Array(e),n=0;n<e;n++)a[n]=B;return t.index++,a}function gl(e,t){return typeof t=="function"?t(e):t}function Ic(e){var t=ca();return S0(t,ft,e)}function S0(e,t,a){var n=e.queue;if(n===null)throw Error(u(311));n.lastRenderedReducer=a;var s=e.baseQueue,c=n.pending;if(c!==null){if(s!==null){var d=s.next;s.next=c.next,c.next=d}t.baseQueue=s=c,n.pending=null}if(c=e.baseState,s===null)e.memoizedState=c;else{t=s.next;var w=d=null,L=null,re=t,be=!1;do{var Ce=re.lane&-536870913;if(Ce!==re.lane?(bt&Ce)===Ce:(Sa&Ce)===Ce){var he=re.revertLane;if(he===0)L!==null&&(L=L.next={lane:0,revertLane:0,gesture:null,action:re.action,hasEagerState:re.hasEagerState,eagerState:re.eagerState,next:null}),Ce===Ja&&(be=!0);else if((Sa&he)===he){re=re.next,he===Ja&&(be=!0);continue}else Ce={lane:0,revertLane:re.revertLane,gesture:null,action:re.action,hasEagerState:re.hasEagerState,eagerState:re.eagerState,next:null},L===null?(w=L=Ce,d=c):L=L.next=Ce,et.lanes|=he,is|=he;Ce=re.action,Xn&&a(c,Ce),c=re.hasEagerState?re.eagerState:a(c,Ce)}else he={lane:Ce,revertLane:re.revertLane,gesture:re.gesture,action:re.action,hasEagerState:re.hasEagerState,eagerState:re.eagerState,next:null},L===null?(w=L=he,d=c):L=L.next=he,et.lanes|=Ce,is|=Ce;re=re.next}while(re!==null&&re!==t);if(L===null?d=c:L.next=w,!Ra(c,e.memoizedState)&&(ha=!0,be&&(a=ml,a!==null)))throw a;e.memoizedState=c,e.baseState=d,e.baseQueue=L,n.lastRenderedState=c}return s===null&&(n.lanes=0),[e.memoizedState,n.dispatch]}function T0(e){var t=ca(),a=t.queue;if(a===null)throw Error(u(311));a.lastRenderedReducer=e;var n=a.dispatch,s=a.pending,c=t.memoizedState;if(s!==null){a.pending=null;var d=s=s.next;do c=e(c,d.action),d=d.next;while(d!==s);Ra(c,t.memoizedState)||(ha=!0),t.memoizedState=c,t.baseQueue===null&&(t.baseState=c),a.lastRenderedState=c}return[c,n]}function xh(e,t,a){var n=et,s=ca(),c=mt;if(c){if(a===void 0)throw Error(u(407));a=a()}else a=t();var d=!Ra((ft||s).memoizedState,a);if(d&&(s.memoizedState=a,ha=!0),s=s.queue,_0(Mh.bind(null,n,s,e),[e]),s.getSnapshot!==t||d||Nt!==null&&Nt.memoizedState.tag&1){if(n.flags|=2048,Xi(9,{destroy:void 0},bh.bind(null,n,s,a,t),null),Ft===null)throw Error(u(349));c||(Sa&127)!==0||wh(n,t,a)}return a}function wh(e,t,a){e.flags|=16384,e={getSnapshot:t,value:a},t=et.updateQueue,t===null?(t=Zc(),et.updateQueue=t,t.stores=[e]):(a=t.stores,a===null?t.stores=[e]:a.push(e))}function bh(e,t,a,n){t.value=a,t.getSnapshot=n,Sh(t)&&Th(e)}function Mh(e,t,a){return a(function(){Sh(t)&&Th(e)})}function Sh(e){var t=e.getSnapshot;e=e.value;try{var a=t();return!Ra(e,a)}catch{return!0}}function Th(e){var t=qn(e,2);t!==null&&gn(t,e,2)}function z0(e){var t=tn();if(typeof e=="function"){var a=e;if(e=a(),Xn){sn(!0);try{a()}finally{sn(!1)}}}return t.memoizedState=t.baseState=e,t.queue={pending:null,lanes:0,dispatch:null,lastRenderedReducer:gl,lastRenderedState:e},t}function zh(e,t,a,n){return e.baseState=a,S0(e,ft,typeof n=="function"?n:gl)}function Mp(e,t,a,n,s){if(eu(e))throw Error(u(485));if(e=t.action,e!==null){var c={payload:s,action:e,next:null,isTransition:!0,status:"pending",value:null,reason:null,listeners:[],then:function(d){c.listeners.push(d)}};G.T!==null?a(!0):c.isTransition=!1,n(c),a=t.pending,a===null?(c.next=t.pending=c,Ch(t,c)):(c.next=a.next,t.pending=a.next=c)}}function Ch(e,t){var a=t.action,n=t.payload,s=e.state;if(t.isTransition){var c=G.T,d={};G.T=d;try{var w=a(s,n),L=G.S;L!==null&&L(d,w),_h(e,t,w)}catch(re){C0(e,t,re)}finally{c!==null&&d.types!==null&&(c.types=d.types),G.T=c}}else try{c=a(s,n),_h(e,t,c)}catch(re){C0(e,t,re)}}function _h(e,t,a){a!==null&&typeof a=="object"&&typeof a.then=="function"?a.then(function(n){Ah(e,t,n)},function(n){return C0(e,t,n)}):Ah(e,t,a)}function Ah(e,t,a){t.status="fulfilled",t.value=a,Eh(t),e.state=a,t=e.pending,t!==null&&(a=t.next,a===t?e.pending=null:(a=a.next,t.next=a,Ch(e,a)))}function C0(e,t,a){var n=e.pending;if(e.pending=null,n!==null){n=n.next;do t.status="rejected",t.reason=a,Eh(t),t=t.next;while(t!==n)}e.action=null}function Eh(e){e=e.listeners;for(var t=0;t<e.length;t++)(0,e[t])()}function Rh(e,t){return t}function Nh(e,t){if(mt){var a=Ft.formState;if(a!==null){e:{var n=et;if(mt){if(kt){t:{for(var s=kt,c=Da;s.nodeType!==8;){if(!c){s=null;break t}if(s=Qn(s.nextSibling),s===null){s=null;break t}}c=s.data,s=c==="F!"||c==="F"?s:null}if(s){kt=Qn(s.nextSibling),n=s.data==="F!";break e}}mo(n)}n=!1}n&&(t=a[0])}}return a=tn(),a.memoizedState=a.baseState=t,n={pending:null,lanes:0,dispatch:null,lastRenderedReducer:Rh,lastRenderedState:t},a.queue=n,a=Qh.bind(null,et,n),n.dispatch=a,n=z0(!1),c=D0.bind(null,et,!1,n.queue),n=tn(),s={state:t,dispatch:null,action:e,pending:null},n.queue=s,a=Mp.bind(null,et,s,c,a),s.dispatch=a,n.memoizedState=e,[t,a,!1]}function Dh(e){var t=ca();return Uh(t,ft,e)}function Uh(e,t,a){if(t=S0(e,t,Rh)[0],e=Ic(gl)[0],typeof t=="object"&&t!==null&&typeof t.then=="function")try{var n=Yr(t)}catch(d){throw d===M?N:d}else n=t;t=ca();var s=t.queue,c=s.dispatch;return a!==t.memoizedState&&(et.flags|=2048,Xi(9,{destroy:void 0},Sp.bind(null,s,a),null)),[n,c,e]}function Sp(e,t){e.action=t}function Oh(e){var t=ca(),a=ft;if(a!==null)return Uh(t,a,e);ca(),t=t.memoizedState,a=ca();var n=a.queue.dispatch;return a.memoizedState=e,[t,n,!1]}function Xi(e,t,a,n){return e={tag:e,create:a,deps:n,inst:t,next:null},t=et.updateQueue,t===null&&(t=Zc(),et.updateQueue=t),a=t.lastEffect,a===null?t.lastEffect=e.next=e:(n=a.next,a.next=e,e.next=n,t.lastEffect=e),e}function kh(){return ca().memoizedState}function Jc(e,t,a,n){var s=tn();et.flags|=e,s.memoizedState=Xi(1|t,{destroy:void 0},a,n===void 0?null:n)}function $c(e,t,a,n){var s=ca();n=n===void 0?null:n;var c=s.memoizedState.inst;ft!==null&&n!==null&&Ki(n,ft.memoizedState.deps)?s.memoizedState=Xi(t,c,a,n):(et.flags|=e,s.memoizedState=Xi(1|t,c,a,n))}function Bh(e,t){Jc(8390656,8,e,t)}function _0(e,t){$c(2048,8,e,t)}function Tp(e){et.flags|=4;var t=et.updateQueue;if(t===null)t=Zc(),et.updateQueue=t,t.events=[e];else{var a=t.events;a===null?t.events=[e]:a.push(e)}}function Hh(e){var t=ca().memoizedState;return Tp({ref:t,nextImpl:e}),function(){if((Dt&2)!==0)throw Error(u(440));return t.impl.apply(void 0,arguments)}}function Lh(e,t){return $c(4,2,e,t)}function jh(e,t){return $c(4,4,e,t)}function qh(e,t){if(typeof t=="function"){e=e();var a=t(e);return function(){typeof a=="function"?a():t(null)}}if(t!=null)return e=e(),t.current=e,function(){t.current=null}}function Gh(e,t,a){a=a!=null?a.concat([e]):null,$c(4,4,qh.bind(null,t,e),a)}function A0(){}function Fh(e,t){var a=ca();t=t===void 0?null:t;var n=a.memoizedState;return t!==null&&Ki(t,n[1])?n[0]:(a.memoizedState=[e,t],e)}function Vh(e,t){var a=ca();t=t===void 0?null:t;var n=a.memoizedState;if(t!==null&&Ki(t,n[1]))return n[0];if(n=e(),Xn){sn(!0);try{e()}finally{sn(!1)}}return a.memoizedState=[n,t],n}function E0(e,t,a){return a===void 0||(Sa&1073741824)!==0&&(bt&261930)===0?e.memoizedState=t:(e.memoizedState=a,e=Kd(),et.lanes|=e,is|=e,a)}function Kh(e,t,a,n){return Ra(a,t)?a:nt.current!==null?(e=E0(e,a,n),Ra(e,t)||(ha=!0),e):(Sa&42)===0||(Sa&1073741824)!==0&&(bt&261930)===0?(ha=!0,e.memoizedState=a):(e=Kd(),et.lanes|=e,is|=e,t)}function Yh(e,t,a,n,s){var c=j.p;j.p=c!==0&&8>c?c:8;var d=G.T,w={};G.T=w,D0(e,!1,t,a);try{var L=s(),re=G.S;if(re!==null&&re(w,L),L!==null&&typeof L=="object"&&typeof L.then=="function"){var be=Xc(L,n);Wr(e,t,be,Rn(e))}else Wr(e,t,n,Rn(e))}catch(Ce){Wr(e,t,{then:function(){},status:"rejected",reason:Ce},Rn())}finally{j.p=c,d!==null&&w.types!==null&&(d.types=w.types),G.T=d}}function zp(){}function R0(e,t,a,n){if(e.tag!==5)throw Error(u(476));var s=Wh(e).queue;Yh(e,s,t,me,a===null?zp:function(){return Xh(e),a(n)})}function Wh(e){var t=e.memoizedState;if(t!==null)return t;t={memoizedState:me,baseState:me,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:gl,lastRenderedState:me},next:null};var a={};return t.next={memoizedState:a,baseState:a,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:gl,lastRenderedState:a},next:null},e.memoizedState=t,e=e.alternate,e!==null&&(e.memoizedState=t),t}function Xh(e){var t=Wh(e);t.next===null&&(t=e.alternate.memoizedState),Wr(e,t.next.queue,{},Rn())}function N0(){return na(cc)}function Ph(){return ca().memoizedState}function Zh(){return ca().memoizedState}function Cp(e){for(var t=e.return;t!==null;){switch(t.tag){case 24:case 3:var a=Rn();e=Le(a);var n=je(t,e,a);n!==null&&(gn(n,t,a),De(n,t,a)),t={cache:Gi()},e.payload=t;return}t=t.return}}function _p(e,t,a){var n=Rn();a={lane:n,revertLane:0,gesture:null,action:a,hasEagerState:!1,eagerState:null,next:null},eu(e)?Ih(t,a):(a=Qs(e,t,a,n),a!==null&&(gn(a,e,n),Jh(a,t,n)))}function Qh(e,t,a){var n=Rn();Wr(e,t,a,n)}function Wr(e,t,a,n){var s={lane:n,revertLane:0,gesture:null,action:a,hasEagerState:!1,eagerState:null,next:null};if(eu(e))Ih(t,s);else{var c=e.alternate;if(e.lanes===0&&(c===null||c.lanes===0)&&(c=t.lastRenderedReducer,c!==null))try{var d=t.lastRenderedState,w=c(d,a);if(s.hasEagerState=!0,s.eagerState=w,Ra(w,d))return Zl(e,t,s,0),Ft===null&&rl(),!1}catch{}finally{}if(a=Qs(e,t,s,n),a!==null)return gn(a,e,n),Jh(a,t,n),!0}return!1}function D0(e,t,a,n){if(n={lane:2,revertLane:ff(),gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null},eu(e)){if(t)throw Error(u(479))}else t=Qs(e,a,n,2),t!==null&&gn(t,e,2)}function eu(e){var t=e.alternate;return e===et||t!==null&&t===et}function Ih(e,t){vl=pl=!0;var a=e.pending;a===null?t.next=t:(t.next=a.next,a.next=t),e.pending=t}function Jh(e,t,a){if((a&4194048)!==0){var n=t.lanes;n&=e.pendingLanes,a|=n,t.lanes=a,gi(e,a)}}var Xr={readContext:na,use:Qc,useCallback:Pt,useContext:Pt,useEffect:Pt,useImperativeHandle:Pt,useLayoutEffect:Pt,useInsertionEffect:Pt,useMemo:Pt,useReducer:Pt,useRef:Pt,useState:Pt,useDebugValue:Pt,useDeferredValue:Pt,useTransition:Pt,useSyncExternalStore:Pt,useId:Pt,useHostTransitionStatus:Pt,useFormState:Pt,useActionState:Pt,useOptimistic:Pt,useMemoCache:Pt,useCacheRefresh:Pt};Xr.useEffectEvent=Pt;var $h={readContext:na,use:Qc,useCallback:function(e,t){return tn().memoizedState=[e,t===void 0?null:t],e},useContext:na,useEffect:Bh,useImperativeHandle:function(e,t,a){a=a!=null?a.concat([e]):null,Jc(4194308,4,qh.bind(null,t,e),a)},useLayoutEffect:function(e,t){return Jc(4194308,4,e,t)},useInsertionEffect:function(e,t){Jc(4,2,e,t)},useMemo:function(e,t){var a=tn();t=t===void 0?null:t;var n=e();if(Xn){sn(!0);try{e()}finally{sn(!1)}}return a.memoizedState=[n,t],n},useReducer:function(e,t,a){var n=tn();if(a!==void 0){var s=a(t);if(Xn){sn(!0);try{a(t)}finally{sn(!1)}}}else s=t;return n.memoizedState=n.baseState=s,e={pending:null,lanes:0,dispatch:null,lastRenderedReducer:e,lastRenderedState:s},n.queue=e,e=e.dispatch=_p.bind(null,et,e),[n.memoizedState,e]},useRef:function(e){var t=tn();return e={current:e},t.memoizedState=e},useState:function(e){e=z0(e);var t=e.queue,a=Qh.bind(null,et,t);return t.dispatch=a,[e.memoizedState,a]},useDebugValue:A0,useDeferredValue:function(e,t){var a=tn();return E0(a,e,t)},useTransition:function(){var e=z0(!1);return e=Yh.bind(null,et,e.queue,!0,!1),tn().memoizedState=e,[!1,e]},useSyncExternalStore:function(e,t,a){var n=et,s=tn();if(mt){if(a===void 0)throw Error(u(407));a=a()}else{if(a=t(),Ft===null)throw Error(u(349));(bt&127)!==0||wh(n,t,a)}s.memoizedState=a;var c={value:a,getSnapshot:t};return s.queue=c,Bh(Mh.bind(null,n,c,e),[e]),n.flags|=2048,Xi(9,{destroy:void 0},bh.bind(null,n,c,a,t),null),a},useId:function(){var e=tn(),t=Ft.identifierPrefix;if(mt){var a=Fn,n=Gn;a=(n&~(1<<32-Vt(n)-1)).toString(32)+a,t="_"+t+"R_"+a,a=po++,0<a&&(t+="H"+a.toString(32)),t+="_"}else a=os++,t="_"+t+"r_"+a.toString(32)+"_";return e.memoizedState=t},useHostTransitionStatus:N0,useFormState:Nh,useActionState:Nh,useOptimistic:function(e){var t=tn();t.memoizedState=t.baseState=e;var a={pending:null,lanes:0,dispatch:null,lastRenderedReducer:null,lastRenderedState:null};return t.queue=a,t=D0.bind(null,et,!0,a),a.dispatch=t,[e,t]},useMemoCache:M0,useCacheRefresh:function(){return tn().memoizedState=Cp.bind(null,et)},useEffectEvent:function(e){var t=tn(),a={impl:e};return t.memoizedState=a,function(){if((Dt&2)!==0)throw Error(u(440));return a.impl.apply(void 0,arguments)}}},U0={readContext:na,use:Qc,useCallback:Fh,useContext:na,useEffect:_0,useImperativeHandle:Gh,useInsertionEffect:Lh,useLayoutEffect:jh,useMemo:Vh,useReducer:Ic,useRef:kh,useState:function(){return Ic(gl)},useDebugValue:A0,useDeferredValue:function(e,t){var a=ca();return Kh(a,ft.memoizedState,e,t)},useTransition:function(){var e=Ic(gl)[0],t=ca().memoizedState;return[typeof e=="boolean"?e:Yr(e),t]},useSyncExternalStore:xh,useId:Ph,useHostTransitionStatus:N0,useFormState:Dh,useActionState:Dh,useOptimistic:function(e,t){var a=ca();return zh(a,ft,e,t)},useMemoCache:M0,useCacheRefresh:Zh};U0.useEffectEvent=Hh;var ed={readContext:na,use:Qc,useCallback:Fh,useContext:na,useEffect:_0,useImperativeHandle:Gh,useInsertionEffect:Lh,useLayoutEffect:jh,useMemo:Vh,useReducer:T0,useRef:kh,useState:function(){return T0(gl)},useDebugValue:A0,useDeferredValue:function(e,t){var a=ca();return ft===null?E0(a,e,t):Kh(a,ft.memoizedState,e,t)},useTransition:function(){var e=T0(gl)[0],t=ca().memoizedState;return[typeof e=="boolean"?e:Yr(e),t]},useSyncExternalStore:xh,useId:Ph,useHostTransitionStatus:N0,useFormState:Oh,useActionState:Oh,useOptimistic:function(e,t){var a=ca();return ft!==null?zh(a,ft,e,t):(a.baseState=e,[e,a.queue.dispatch])},useMemoCache:M0,useCacheRefresh:Zh};ed.useEffectEvent=Hh;function O0(e,t,a,n){t=e.memoizedState,a=a(n,t),a=a==null?t:S({},t,a),e.memoizedState=a,e.lanes===0&&(e.updateQueue.baseState=a)}var k0={enqueueSetState:function(e,t,a){e=e._reactInternals;var n=Rn(),s=Le(n);s.payload=t,a!=null&&(s.callback=a),t=je(e,s,n),t!==null&&(gn(t,e,n),De(t,e,n))},enqueueReplaceState:function(e,t,a){e=e._reactInternals;var n=Rn(),s=Le(n);s.tag=1,s.payload=t,a!=null&&(s.callback=a),t=je(e,s,n),t!==null&&(gn(t,e,n),De(t,e,n))},enqueueForceUpdate:function(e,t){e=e._reactInternals;var a=Rn(),n=Le(a);n.tag=2,t!=null&&(n.callback=t),t=je(e,n,a),t!==null&&(gn(t,e,a),De(t,e,a))}};function td(e,t,a,n,s,c,d){return e=e.stateNode,typeof e.shouldComponentUpdate=="function"?e.shouldComponentUpdate(n,c,d):t.prototype&&t.prototype.isPureReactComponent?!ll(a,n)||!ll(s,c):!0}function ad(e,t,a,n){e=t.state,typeof t.componentWillReceiveProps=="function"&&t.componentWillReceiveProps(a,n),typeof t.UNSAFE_componentWillReceiveProps=="function"&&t.UNSAFE_componentWillReceiveProps(a,n),t.state!==e&&k0.enqueueReplaceState(t,t.state,null)}function ri(e,t){var a=t;if("ref"in t){a={};for(var n in t)n!=="ref"&&(a[n]=t[n])}if(e=e.defaultProps){a===t&&(a=S({},a));for(var s in e)a[s]===void 0&&(a[s]=e[s])}return a}function nd(e){Xl(e)}function od(e){console.error(e)}function ld(e){Xl(e)}function tu(e,t){try{var a=e.onUncaughtError;a(t.value,{componentStack:t.stack})}catch(n){setTimeout(function(){throw n})}}function sd(e,t,a){try{var n=e.onCaughtError;n(a.value,{componentStack:a.stack,errorBoundary:t.tag===1?t.stateNode:null})}catch(s){setTimeout(function(){throw s})}}function B0(e,t,a){return a=Le(a),a.tag=3,a.payload={element:null},a.callback=function(){tu(e,t)},a}function id(e){return e=Le(e),e.tag=3,e}function rd(e,t,a,n){var s=a.type.getDerivedStateFromError;if(typeof s=="function"){var c=n.value;e.payload=function(){return s(c)},e.callback=function(){sd(t,a,n)}}var d=a.stateNode;d!==null&&typeof d.componentDidCatch=="function"&&(e.callback=function(){sd(t,a,n),typeof s!="function"&&(rs===null?rs=new Set([this]):rs.add(this));var w=n.stack;this.componentDidCatch(n.value,{componentStack:w!==null?w:""})})}function Ap(e,t,a,n,s){if(a.flags|=32768,n!==null&&typeof n=="object"&&typeof n.then=="function"){if(t=a.alternate,t!==null&&Cn(t,a,s,!0),a=wt.current,a!==null){switch(a.tag){case 31:case 13:return St===null?du():a.alternate===null&&la===0&&(la=3),a.flags&=-257,a.flags|=65536,a.lanes=s,n===k?a.flags|=16384:(t=a.updateQueue,t===null?a.updateQueue=new Set([n]):t.add(n),rf(e,n,s)),!1;case 22:return a.flags|=65536,n===k?a.flags|=16384:(t=a.updateQueue,t===null?(t={transitions:null,markerInstances:null,retryQueue:new Set([n])},a.updateQueue=t):(a=t.retryQueue,a===null?t.retryQueue=new Set([n]):a.add(n)),rf(e,n,s)),!1}throw Error(u(435,a.tag))}return rf(e,n,s),du(),!1}if(mt)return t=wt.current,t!==null?((t.flags&65536)===0&&(t.flags|=256),t.flags|=65536,t.lanes=s,n!==ai&&(e=Error(u(422),{cause:n}),ts(qa(e,a)))):(n!==ai&&(t=Error(u(423),{cause:n}),ts(qa(t,a))),e=e.current.alternate,e.flags|=65536,s&=-s,e.lanes|=s,n=qa(n,a),s=B0(e.stateNode,n,s),Be(e,s),la!==4&&(la=2)),!1;var c=Error(u(520),{cause:n});if(c=qa(c,a),tc===null?tc=[c]:tc.push(c),la!==4&&(la=2),t===null)return!0;n=qa(n,a),a=t;do{switch(a.tag){case 3:return a.flags|=65536,e=s&-s,a.lanes|=e,e=B0(a.stateNode,n,e),Be(a,e),!1;case 1:if(t=a.type,c=a.stateNode,(a.flags&128)===0&&(typeof t.getDerivedStateFromError=="function"||c!==null&&typeof c.componentDidCatch=="function"&&(rs===null||!rs.has(c))))return a.flags|=65536,s&=-s,a.lanes|=s,s=id(s),rd(s,e,a,n),Be(a,s),!1}a=a.return}while(a!==null);return!1}var H0=Error(u(461)),ha=!1;function Ua(e,t,a,n){t.child=e===null?ze(t,null,a,n):Oe(t,e.child,a,n)}function cd(e,t,a,n,s){a=a.render;var c=t.ref;if("ref"in n){var d={};for(var w in n)w!=="ref"&&(d[w]=n[w])}else d=n;return Lo(t),n=Zn(e,t,a,d,c,s),w=x0(),e!==null&&!ha?(w0(e,t,s),yl(e,t,s)):(mt&&w&&Kt(t),t.flags|=1,Ua(e,t,n,s),t.child)}function ud(e,t,a,n,s){if(e===null){var c=a.type;return typeof c=="function"&&!Is(c)&&c.defaultProps===void 0&&a.compare===null?(t.tag=15,t.type=c,fd(e,t,c,n,s)):(e=ul(a.type,null,n,t,t.mode,s),e.ref=t.ref,e.return=t,t.child=e)}if(c=e.child,!Y0(e,s)){var d=c.memoizedProps;if(a=a.compare,a=a!==null?a:ll,a(d,n)&&e.ref===t.ref)return yl(e,t,s)}return t.flags|=1,e=fo(c,n),e.ref=t.ref,e.return=t,t.child=e}function fd(e,t,a,n,s){if(e!==null){var c=e.memoizedProps;if(ll(c,n)&&e.ref===t.ref)if(ha=!1,t.pendingProps=n=c,Y0(e,s))(e.flags&131072)!==0&&(ha=!0);else return t.lanes=e.lanes,yl(e,t,s)}return L0(e,t,a,n,s)}function hd(e,t,a,n){var s=n.children,c=e!==null?e.memoizedState:null;if(e===null&&t.stateNode===null&&(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),n.mode==="hidden"){if((t.flags&128)!==0){if(c=c!==null?c.baseLanes|a:a,e!==null){for(n=t.child=e.child,s=0;n!==null;)s=s|n.lanes|n.childLanes,n=n.sibling;n=s&~c}else n=0,t.child=null;return dd(e,t,c,a,n)}if((a&536870912)!==0)t.memoizedState={baseLanes:0,cachePool:null},e!==null&&g(t,c!==null?c.cachePool:null),c!==null?Ct(t,c):Rt(),en(t);else return n=t.lanes=536870912,dd(e,t,c!==null?c.baseLanes|a:a,a,n)}else c!==null?(g(t,c.cachePool),Ct(t,c),oa(),t.memoizedState=null):(e!==null&&g(t,null),Rt(),oa());return Ua(e,t,s,a),t.child}function Pr(e,t){return e!==null&&e.tag===22||t.stateNode!==null||(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),t.sibling}function dd(e,t,a,n,s){var c=Vi();return c=c===null?null:{parent:Wt._currentValue,pool:c},t.memoizedState={baseLanes:a,cachePool:c},e!==null&&g(t,null),Rt(),en(t),e!==null&&Cn(e,t,n,!0),t.childLanes=s,null}function au(e,t){return t=ou({mode:t.mode,children:t.children},e.mode),t.ref=e.ref,e.child=t,t.return=e,t}function md(e,t,a){return Oe(t,e.child,null,a),e=au(t,t.pendingProps),e.flags|=2,Gt(t),t.memoizedState=null,e}function Ep(e,t,a){var n=t.pendingProps,s=(t.flags&128)!==0;if(t.flags&=-129,e===null){if(mt){if(n.mode==="hidden")return e=au(t,n),t.lanes=536870912,Pr(null,e);if($a(t),(e=kt)?(e=Cm(e,Da),e=e!==null&&e.data==="&"?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:Ho!==null?{id:Gn,overflow:Fn}:null,retryLane:536870912,hydrationErrors:null},a=Vc(e),a.return=t,t.child=a,Yt=t,kt=null)):e=null,e===null)throw mo(t);return t.lanes=536870912,null}return au(t,n)}var c=e.memoizedState;if(c!==null){var d=c.dehydrated;if($a(t),s)if(t.flags&256)t.flags&=-257,t=md(e,t,a);else if(t.memoizedState!==null)t.child=e.child,t.flags|=128,t=null;else throw Error(u(558));else if(ha||Cn(e,t,a,!1),s=(a&e.childLanes)!==0,ha||s){if(n=Ft,n!==null&&(d=hr(n,a),d!==0&&d!==c.retryLane))throw c.retryLane=d,qn(e,d),gn(n,e,d),H0;du(),t=md(e,t,a)}else e=c.treeContext,kt=Qn(d.nextSibling),Yt=t,mt=!0,Vn=null,Da=!1,e!==null&&Vr(t,e),t=au(t,n),t.flags|=4096;return t}return e=fo(e.child,{mode:n.mode,children:n.children}),e.ref=t.ref,t.child=e,e.return=t,e}function nu(e,t){var a=t.ref;if(a===null)e!==null&&e.ref!==null&&(t.flags|=4194816);else{if(typeof a!="function"&&typeof a!="object")throw Error(u(284));(e===null||e.ref!==a)&&(t.flags|=4194816)}}function L0(e,t,a,n,s){return Lo(t),a=Zn(e,t,a,n,void 0,s),n=x0(),e!==null&&!ha?(w0(e,t,s),yl(e,t,s)):(mt&&n&&Kt(t),t.flags|=1,Ua(e,t,a,s),t.child)}function pd(e,t,a,n,s,c){return Lo(t),t.updateQueue=null,a=Wi(t,n,a,s),Yi(e),n=x0(),e!==null&&!ha?(w0(e,t,c),yl(e,t,c)):(mt&&n&&Kt(t),t.flags|=1,Ua(e,t,a,c),t.child)}function vd(e,t,a,n,s){if(Lo(t),t.stateNode===null){var c=cl,d=a.contextType;typeof d=="object"&&d!==null&&(c=na(d)),c=new a(n,c),t.memoizedState=c.state!==null&&c.state!==void 0?c.state:null,c.updater=k0,t.stateNode=c,c._reactInternals=t,c=t.stateNode,c.props=n,c.state=t.memoizedState,c.refs={},He(t),d=a.contextType,c.context=typeof d=="object"&&d!==null?na(d):cl,c.state=t.memoizedState,d=a.getDerivedStateFromProps,typeof d=="function"&&(O0(t,a,d,n),c.state=t.memoizedState),typeof a.getDerivedStateFromProps=="function"||typeof c.getSnapshotBeforeUpdate=="function"||typeof c.UNSAFE_componentWillMount!="function"&&typeof c.componentWillMount!="function"||(d=c.state,typeof c.componentWillMount=="function"&&c.componentWillMount(),typeof c.UNSAFE_componentWillMount=="function"&&c.UNSAFE_componentWillMount(),d!==c.state&&k0.enqueueReplaceState(c,c.state,null),Qe(t,n,c,s),qe(),c.state=t.memoizedState),typeof c.componentDidMount=="function"&&(t.flags|=4194308),n=!0}else if(e===null){c=t.stateNode;var w=t.memoizedProps,L=ri(a,w);c.props=L;var re=c.context,be=a.contextType;d=cl,typeof be=="object"&&be!==null&&(d=na(be));var Ce=a.getDerivedStateFromProps;be=typeof Ce=="function"||typeof c.getSnapshotBeforeUpdate=="function",w=t.pendingProps!==w,be||typeof c.UNSAFE_componentWillReceiveProps!="function"&&typeof c.componentWillReceiveProps!="function"||(w||re!==d)&&ad(t,c,n,d),Ae=!1;var he=t.memoizedState;c.state=he,Qe(t,n,c,s),qe(),re=t.memoizedState,w||he!==re||Ae?(typeof Ce=="function"&&(O0(t,a,Ce,n),re=t.memoizedState),(L=Ae||td(t,a,L,n,he,re,d))?(be||typeof c.UNSAFE_componentWillMount!="function"&&typeof c.componentWillMount!="function"||(typeof c.componentWillMount=="function"&&c.componentWillMount(),typeof c.UNSAFE_componentWillMount=="function"&&c.UNSAFE_componentWillMount()),typeof c.componentDidMount=="function"&&(t.flags|=4194308)):(typeof c.componentDidMount=="function"&&(t.flags|=4194308),t.memoizedProps=n,t.memoizedState=re),c.props=n,c.state=re,c.context=d,n=L):(typeof c.componentDidMount=="function"&&(t.flags|=4194308),n=!1)}else{c=t.stateNode,ke(e,t),d=t.memoizedProps,be=ri(a,d),c.props=be,Ce=t.pendingProps,he=c.context,re=a.contextType,L=cl,typeof re=="object"&&re!==null&&(L=na(re)),w=a.getDerivedStateFromProps,(re=typeof w=="function"||typeof c.getSnapshotBeforeUpdate=="function")||typeof c.UNSAFE_componentWillReceiveProps!="function"&&typeof c.componentWillReceiveProps!="function"||(d!==Ce||he!==L)&&ad(t,c,n,L),Ae=!1,he=t.memoizedState,c.state=he,Qe(t,n,c,s),qe();var ve=t.memoizedState;d!==Ce||he!==ve||Ae||e!==null&&e.dependencies!==null&&li(e.dependencies)?(typeof w=="function"&&(O0(t,a,w,n),ve=t.memoizedState),(be=Ae||td(t,a,be,n,he,ve,L)||e!==null&&e.dependencies!==null&&li(e.dependencies))?(re||typeof c.UNSAFE_componentWillUpdate!="function"&&typeof c.componentWillUpdate!="function"||(typeof c.componentWillUpdate=="function"&&c.componentWillUpdate(n,ve,L),typeof c.UNSAFE_componentWillUpdate=="function"&&c.UNSAFE_componentWillUpdate(n,ve,L)),typeof c.componentDidUpdate=="function"&&(t.flags|=4),typeof c.getSnapshotBeforeUpdate=="function"&&(t.flags|=1024)):(typeof c.componentDidUpdate!="function"||d===e.memoizedProps&&he===e.memoizedState||(t.flags|=4),typeof c.getSnapshotBeforeUpdate!="function"||d===e.memoizedProps&&he===e.memoizedState||(t.flags|=1024),t.memoizedProps=n,t.memoizedState=ve),c.props=n,c.state=ve,c.context=L,n=be):(typeof c.componentDidUpdate!="function"||d===e.memoizedProps&&he===e.memoizedState||(t.flags|=4),typeof c.getSnapshotBeforeUpdate!="function"||d===e.memoizedProps&&he===e.memoizedState||(t.flags|=1024),n=!1)}return c=n,nu(e,t),n=(t.flags&128)!==0,c||n?(c=t.stateNode,a=n&&typeof a.getDerivedStateFromError!="function"?null:c.render(),t.flags|=1,e!==null&&n?(t.child=Oe(t,e.child,null,s),t.child=Oe(t,null,a,s)):Ua(e,t,a,s),t.memoizedState=c.state,e=t.child):e=yl(e,t,s),e}function gd(e,t,a,n){return dl(),t.flags|=256,Ua(e,t,a,n),t.child}var j0={dehydrated:null,treeContext:null,retryLane:0,hydrationErrors:null};function q0(e){return{baseLanes:e,cachePool:y()}}function G0(e,t,a){return e=e!==null?e.childLanes&~a:0,t&&(e|=En),e}function yd(e,t,a){var n=t.pendingProps,s=!1,c=(t.flags&128)!==0,d;if((d=c)||(d=e!==null&&e.memoizedState===null?!1:(Tt.current&2)!==0),d&&(s=!0,t.flags&=-129),d=(t.flags&32)!==0,t.flags&=-33,e===null){if(mt){if(s?Xt(t):oa(),(e=kt)?(e=Cm(e,Da),e=e!==null&&e.data!=="&"?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:Ho!==null?{id:Gn,overflow:Fn}:null,retryLane:536870912,hydrationErrors:null},a=Vc(e),a.return=t,t.child=a,Yt=t,kt=null)):e=null,e===null)throw mo(t);return Tf(e)?t.lanes=32:t.lanes=536870912,null}var w=n.children;return n=n.fallback,s?(oa(),s=t.mode,w=ou({mode:"hidden",children:w},s),n=ho(n,s,a,null),w.return=t,n.return=t,w.sibling=n,t.child=w,n=t.child,n.memoizedState=q0(a),n.childLanes=G0(e,d,a),t.memoizedState=j0,Pr(null,n)):(Xt(t),F0(t,w))}var L=e.memoizedState;if(L!==null&&(w=L.dehydrated,w!==null)){if(c)t.flags&256?(Xt(t),t.flags&=-257,t=V0(e,t,a)):t.memoizedState!==null?(oa(),t.child=e.child,t.flags|=128,t=null):(oa(),w=n.fallback,s=t.mode,n=ou({mode:"visible",children:n.children},s),w=ho(w,s,a,null),w.flags|=2,n.return=t,w.return=t,n.sibling=w,t.child=n,Oe(t,e.child,null,a),n=t.child,n.memoizedState=q0(a),n.childLanes=G0(e,d,a),t.memoizedState=j0,t=Pr(null,n));else if(Xt(t),Tf(w)){if(d=w.nextSibling&&w.nextSibling.dataset,d)var re=d.dgst;d=re,n=Error(u(419)),n.stack="",n.digest=d,ts({value:n,source:null,stack:null}),t=V0(e,t,a)}else if(ha||Cn(e,t,a,!1),d=(a&e.childLanes)!==0,ha||d){if(d=Ft,d!==null&&(n=hr(d,a),n!==0&&n!==L.retryLane))throw L.retryLane=n,qn(e,n),gn(d,e,n),H0;Sf(w)||du(),t=V0(e,t,a)}else Sf(w)?(t.flags|=192,t.child=e.child,t=null):(e=L.treeContext,kt=Qn(w.nextSibling),Yt=t,mt=!0,Vn=null,Da=!1,e!==null&&Vr(t,e),t=F0(t,n.children),t.flags|=4096);return t}return s?(oa(),w=n.fallback,s=t.mode,L=e.child,re=L.sibling,n=fo(L,{mode:"hidden",children:n.children}),n.subtreeFlags=L.subtreeFlags&65011712,re!==null?w=fo(re,w):(w=ho(w,s,a,null),w.flags|=2),w.return=t,n.return=t,n.sibling=w,t.child=n,Pr(null,n),n=t.child,w=e.child.memoizedState,w===null?w=q0(a):(s=w.cachePool,s!==null?(L=Wt._currentValue,s=s.parent!==L?{parent:L,pool:L}:s):s=y(),w={baseLanes:w.baseLanes|a,cachePool:s}),n.memoizedState=w,n.childLanes=G0(e,d,a),t.memoizedState=j0,Pr(e.child,n)):(Xt(t),a=e.child,e=a.sibling,a=fo(a,{mode:"visible",children:n.children}),a.return=t,a.sibling=null,e!==null&&(d=t.deletions,d===null?(t.deletions=[e],t.flags|=16):d.push(e)),t.child=a,t.memoizedState=null,a)}function F0(e,t){return t=ou({mode:"visible",children:t},e.mode),t.return=e,e.child=t}function ou(e,t){return e=va(22,e,null,t),e.lanes=0,e}function V0(e,t,a){return Oe(t,e.child,null,a),e=F0(t,t.pendingProps.children),e.flags|=2,t.memoizedState=null,e}function xd(e,t,a){e.lanes|=t;var n=e.alternate;n!==null&&(n.lanes|=t),as(e.return,t,a)}function K0(e,t,a,n,s,c){var d=e.memoizedState;d===null?e.memoizedState={isBackwards:t,rendering:null,renderingStartTime:0,last:n,tail:a,tailMode:s,treeForkCount:c}:(d.isBackwards=t,d.rendering=null,d.renderingStartTime=0,d.last=n,d.tail=a,d.tailMode=s,d.treeForkCount=c)}function wd(e,t,a){var n=t.pendingProps,s=n.revealOrder,c=n.tail;n=n.children;var d=Tt.current,w=(d&2)!==0;if(w?(d=d&1|2,t.flags|=128):d&=1,T(Tt,d),Ua(e,t,n,a),n=mt?Il:0,!w&&e!==null&&(e.flags&128)!==0)e:for(e=t.child;e!==null;){if(e.tag===13)e.memoizedState!==null&&xd(e,a,t);else if(e.tag===19)xd(e,a,t);else if(e.child!==null){e.child.return=e,e=e.child;continue}if(e===t)break e;for(;e.sibling===null;){if(e.return===null||e.return===t)break e;e=e.return}e.sibling.return=e.return,e=e.sibling}switch(s){case"forwards":for(a=t.child,s=null;a!==null;)e=a.alternate,e!==null&&Wn(e)===null&&(s=a),a=a.sibling;a=s,a===null?(s=t.child,t.child=null):(s=a.sibling,a.sibling=null),K0(t,!1,s,a,c,n);break;case"backwards":case"unstable_legacy-backwards":for(a=null,s=t.child,t.child=null;s!==null;){if(e=s.alternate,e!==null&&Wn(e)===null){t.child=s;break}e=s.sibling,s.sibling=a,a=s,s=e}K0(t,!0,a,null,c,n);break;case"together":K0(t,!1,null,null,void 0,n);break;default:t.memoizedState=null}return t.child}function yl(e,t,a){if(e!==null&&(t.dependencies=e.dependencies),is|=t.lanes,(a&t.childLanes)===0)if(e!==null){if(Cn(e,t,a,!1),(a&t.childLanes)===0)return null}else return null;if(e!==null&&t.child!==e.child)throw Error(u(153));if(t.child!==null){for(e=t.child,a=fo(e,e.pendingProps),t.child=a,a.return=t;e.sibling!==null;)e=e.sibling,a=a.sibling=fo(e,e.pendingProps),a.return=t;a.sibling=null}return t.child}function Y0(e,t){return(e.lanes&t)!==0?!0:(e=e.dependencies,!!(e!==null&&li(e)))}function Rp(e,t,a){switch(t.tag){case 3:Te(t,t.stateNode.containerInfo),Kn(t,Wt,e.memoizedState.cache),dl();break;case 27:case 5:oe(t);break;case 4:Te(t,t.stateNode.containerInfo);break;case 10:Kn(t,t.type,t.memoizedProps.value);break;case 31:if(t.memoizedState!==null)return t.flags|=128,$a(t),null;break;case 13:var n=t.memoizedState;if(n!==null)return n.dehydrated!==null?(Xt(t),t.flags|=128,null):(a&t.child.childLanes)!==0?yd(e,t,a):(Xt(t),e=yl(e,t,a),e!==null?e.sibling:null);Xt(t);break;case 19:var s=(e.flags&128)!==0;if(n=(a&t.childLanes)!==0,n||(Cn(e,t,a,!1),n=(a&t.childLanes)!==0),s){if(n)return wd(e,t,a);t.flags|=128}if(s=t.memoizedState,s!==null&&(s.rendering=null,s.tail=null,s.lastEffect=null),T(Tt,Tt.current),n)break;return null;case 22:return t.lanes=0,hd(e,t,a,t.pendingProps);case 24:Kn(t,Wt,e.memoizedState.cache)}return yl(e,t,a)}function bd(e,t,a){if(e!==null)if(e.memoizedProps!==t.pendingProps)ha=!0;else{if(!Y0(e,a)&&(t.flags&128)===0)return ha=!1,Rp(e,t,a);ha=(e.flags&131072)!==0}else ha=!1,mt&&(t.flags&1048576)!==0&&ti(t,Il,t.index);switch(t.lanes=0,t.tag){case 16:e:{var n=t.pendingProps;if(e=Y(t.elementType),t.type=e,typeof e=="function")Is(e)?(n=ri(e,n),t.tag=1,t=vd(null,t,e,n,a)):(t.tag=0,t=L0(null,t,e,n,a));else{if(e!=null){var s=e.$$typeof;if(s===z){t.tag=11,t=cd(null,t,e,n,a);break e}else if(s===J){t.tag=14,t=ud(null,t,e,n,a);break e}}throw t=Re(e)||e,Error(u(306,t,""))}}return t;case 0:return L0(e,t,t.type,t.pendingProps,a);case 1:return n=t.type,s=ri(n,t.pendingProps),vd(e,t,n,s,a);case 3:e:{if(Te(t,t.stateNode.containerInfo),e===null)throw Error(u(387));n=t.pendingProps;var c=t.memoizedState;s=c.element,ke(e,t),Qe(t,n,null,a);var d=t.memoizedState;if(n=d.cache,Kn(t,Wt,n),n!==c.cache&&oi(t,[Wt],a,!0),qe(),n=d.element,c.isDehydrated)if(c={element:n,isDehydrated:!1,cache:d.cache},t.updateQueue.baseState=c,t.memoizedState=c,t.flags&256){t=gd(e,t,n,a);break e}else if(n!==s){s=qa(Error(u(424)),t),ts(s),t=gd(e,t,n,a);break e}else{switch(e=t.stateNode.containerInfo,e.nodeType){case 9:e=e.body;break;default:e=e.nodeName==="HTML"?e.ownerDocument.body:e}for(kt=Qn(e.firstChild),Yt=t,mt=!0,Vn=null,Da=!0,a=ze(t,null,n,a),t.child=a;a;)a.flags=a.flags&-3|4096,a=a.sibling}else{if(dl(),n===s){t=yl(e,t,a);break e}Ua(e,t,n,a)}t=t.child}return t;case 26:return nu(e,t),e===null?(a=Dm(t.type,null,t.pendingProps,null))?t.memoizedState=a:mt||(a=t.type,e=t.pendingProps,n=wu(ue.current).createElement(a),n[ra]=t,n[ba]=e,Oa(n,a,e),aa(n),t.stateNode=n):t.memoizedState=Dm(t.type,e.memoizedProps,t.pendingProps,e.memoizedState),null;case 27:return oe(t),e===null&&mt&&(n=t.stateNode=Em(t.type,t.pendingProps,ue.current),Yt=t,Da=!0,s=kt,hs(t.type)?(zf=s,kt=Qn(n.firstChild)):kt=s),Ua(e,t,t.pendingProps.children,a),nu(e,t),e===null&&(t.flags|=4194304),t.child;case 5:return e===null&&mt&&((s=n=kt)&&(n=sv(n,t.type,t.pendingProps,Da),n!==null?(t.stateNode=n,Yt=t,kt=Qn(n.firstChild),Da=!1,s=!0):s=!1),s||mo(t)),oe(t),s=t.type,c=t.pendingProps,d=e!==null?e.memoizedProps:null,n=c.children,wf(s,c)?n=null:d!==null&&wf(s,d)&&(t.flags|=32),t.memoizedState!==null&&(s=Zn(e,t,bp,null,null,a),cc._currentValue=s),nu(e,t),Ua(e,t,n,a),t.child;case 6:return e===null&&mt&&((e=a=kt)&&(a=iv(a,t.pendingProps,Da),a!==null?(t.stateNode=a,Yt=t,kt=null,e=!0):e=!1),e||mo(t)),null;case 13:return yd(e,t,a);case 4:return Te(t,t.stateNode.containerInfo),n=t.pendingProps,e===null?t.child=Oe(t,null,n,a):Ua(e,t,n,a),t.child;case 11:return cd(e,t,t.type,t.pendingProps,a);case 7:return Ua(e,t,t.pendingProps,a),t.child;case 8:return Ua(e,t,t.pendingProps.children,a),t.child;case 12:return Ua(e,t,t.pendingProps.children,a),t.child;case 10:return n=t.pendingProps,Kn(t,t.type,n.value),Ua(e,t,n.children,a),t.child;case 9:return s=t.type._context,n=t.pendingProps.children,Lo(t),s=na(s),n=n(s),t.flags|=1,Ua(e,t,n,a),t.child;case 14:return ud(e,t,t.type,t.pendingProps,a);case 15:return fd(e,t,t.type,t.pendingProps,a);case 19:return wd(e,t,a);case 31:return Ep(e,t,a);case 22:return hd(e,t,a,t.pendingProps);case 24:return Lo(t),n=na(Wt),e===null?(s=Vi(),s===null&&(s=Ft,c=Gi(),s.pooledCache=c,c.refCount++,c!==null&&(s.pooledCacheLanes|=a),s=c),t.memoizedState={parent:n,cache:s},He(t),Kn(t,Wt,s)):((e.lanes&a)!==0&&(ke(e,t),Qe(t,null,null,a),qe()),s=e.memoizedState,c=t.memoizedState,s.parent!==n?(s={parent:n,cache:n},t.memoizedState=s,t.lanes===0&&(t.memoizedState=t.updateQueue.baseState=s),Kn(t,Wt,n)):(n=c.cache,Kn(t,Wt,n),n!==s.cache&&oi(t,[Wt],a,!0))),Ua(e,t,t.pendingProps.children,a),t.child;case 29:throw t.pendingProps}throw Error(u(156,t.tag))}function xl(e){e.flags|=4}function W0(e,t,a,n,s){if((t=(e.mode&32)!==0)&&(t=!1),t){if(e.flags|=16777216,(s&335544128)===s)if(e.stateNode.complete)e.flags|=8192;else if(Pd())e.flags|=8192;else throw ee=k,_}else e.flags&=-16777217}function Md(e,t){if(t.type!=="stylesheet"||(t.state.loading&4)!==0)e.flags&=-16777217;else if(e.flags|=16777216,!Hm(t))if(Pd())e.flags|=8192;else throw ee=k,_}function lu(e,t){t!==null&&(e.flags|=4),e.flags&16384&&(t=e.tag!==22?ur():536870912,e.lanes|=t,Ii|=t)}function Zr(e,t){if(!mt)switch(e.tailMode){case"hidden":t=e.tail;for(var a=null;t!==null;)t.alternate!==null&&(a=t),t=t.sibling;a===null?e.tail=null:a.sibling=null;break;case"collapsed":a=e.tail;for(var n=null;a!==null;)a.alternate!==null&&(n=a),a=a.sibling;n===null?t||e.tail===null?e.tail=null:e.tail.sibling=null:n.sibling=null}}function Qt(e){var t=e.alternate!==null&&e.alternate.child===e.child,a=0,n=0;if(t)for(var s=e.child;s!==null;)a|=s.lanes|s.childLanes,n|=s.subtreeFlags&65011712,n|=s.flags&65011712,s.return=e,s=s.sibling;else for(s=e.child;s!==null;)a|=s.lanes|s.childLanes,n|=s.subtreeFlags,n|=s.flags,s.return=e,s=s.sibling;return e.subtreeFlags|=n,e.childLanes=a,t}function Np(e,t,a){var n=t.pendingProps;switch(Jl(t),t.tag){case 16:case 15:case 0:case 11:case 7:case 8:case 12:case 9:case 14:return Qt(t),null;case 1:return Qt(t),null;case 3:return a=t.stateNode,n=null,e!==null&&(n=e.memoizedState.cache),t.memoizedState.cache!==n&&(t.flags|=2048),Yn(Wt),Ue(),a.pendingContext&&(a.context=a.pendingContext,a.pendingContext=null),(e===null||e.child===null)&&(zn(t)?xl(t):e===null||e.memoizedState.isDehydrated&&(t.flags&256)===0||(t.flags|=1024,ni())),Qt(t),null;case 26:var s=t.type,c=t.memoizedState;return e===null?(xl(t),c!==null?(Qt(t),Md(t,c)):(Qt(t),W0(t,s,null,n,a))):c?c!==e.memoizedState?(xl(t),Qt(t),Md(t,c)):(Qt(t),t.flags&=-16777217):(e=e.memoizedProps,e!==n&&xl(t),Qt(t),W0(t,s,e,n,a)),null;case 27:if(_e(t),a=ue.current,s=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==n&&xl(t);else{if(!n){if(t.stateNode===null)throw Error(u(166));return Qt(t),null}e=F.current,zn(t)?$l(t):(e=Em(s,n,a),t.stateNode=e,xl(t))}return Qt(t),null;case 5:if(_e(t),s=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==n&&xl(t);else{if(!n){if(t.stateNode===null)throw Error(u(166));return Qt(t),null}if(c=F.current,zn(t))$l(t);else{var d=wu(ue.current);switch(c){case 1:c=d.createElementNS("http://www.w3.org/2000/svg",s);break;case 2:c=d.createElementNS("http://www.w3.org/1998/Math/MathML",s);break;default:switch(s){case"svg":c=d.createElementNS("http://www.w3.org/2000/svg",s);break;case"math":c=d.createElementNS("http://www.w3.org/1998/Math/MathML",s);break;case"script":c=d.createElement("div"),c.innerHTML="<script><\/script>",c=c.removeChild(c.firstChild);break;case"select":c=typeof n.is=="string"?d.createElement("select",{is:n.is}):d.createElement("select"),n.multiple?c.multiple=!0:n.size&&(c.size=n.size);break;default:c=typeof n.is=="string"?d.createElement(s,{is:n.is}):d.createElement(s)}}c[ra]=t,c[ba]=n;e:for(d=t.child;d!==null;){if(d.tag===5||d.tag===6)c.appendChild(d.stateNode);else if(d.tag!==4&&d.tag!==27&&d.child!==null){d.child.return=d,d=d.child;continue}if(d===t)break e;for(;d.sibling===null;){if(d.return===null||d.return===t)break e;d=d.return}d.sibling.return=d.return,d=d.sibling}t.stateNode=c;e:switch(Oa(c,s,n),s){case"button":case"input":case"select":case"textarea":n=!!n.autoFocus;break e;case"img":n=!0;break e;default:n=!1}n&&xl(t)}}return Qt(t),W0(t,t.type,e===null?null:e.memoizedProps,t.pendingProps,a),null;case 6:if(e&&t.stateNode!=null)e.memoizedProps!==n&&xl(t);else{if(typeof n!="string"&&t.stateNode===null)throw Error(u(166));if(e=ue.current,zn(t)){if(e=t.stateNode,a=t.memoizedProps,n=null,s=Yt,s!==null)switch(s.tag){case 27:case 5:n=s.memoizedProps}e[ra]=t,e=!!(e.nodeValue===a||n!==null&&n.suppressHydrationWarning===!0||ym(e.nodeValue,a)),e||mo(t,!0)}else e=wu(e).createTextNode(n),e[ra]=t,t.stateNode=e}return Qt(t),null;case 31:if(a=t.memoizedState,e===null||e.memoizedState!==null){if(n=zn(t),a!==null){if(e===null){if(!n)throw Error(u(318));if(e=t.memoizedState,e=e!==null?e.dehydrated:null,!e)throw Error(u(557));e[ra]=t}else dl(),(t.flags&128)===0&&(t.memoizedState=null),t.flags|=4;Qt(t),e=!1}else a=ni(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=a),e=!0;if(!e)return t.flags&256?(Gt(t),t):(Gt(t),null);if((t.flags&128)!==0)throw Error(u(558))}return Qt(t),null;case 13:if(n=t.memoizedState,e===null||e.memoizedState!==null&&e.memoizedState.dehydrated!==null){if(s=zn(t),n!==null&&n.dehydrated!==null){if(e===null){if(!s)throw Error(u(318));if(s=t.memoizedState,s=s!==null?s.dehydrated:null,!s)throw Error(u(317));s[ra]=t}else dl(),(t.flags&128)===0&&(t.memoizedState=null),t.flags|=4;Qt(t),s=!1}else s=ni(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=s),s=!0;if(!s)return t.flags&256?(Gt(t),t):(Gt(t),null)}return Gt(t),(t.flags&128)!==0?(t.lanes=a,t):(a=n!==null,e=e!==null&&e.memoizedState!==null,a&&(n=t.child,s=null,n.alternate!==null&&n.alternate.memoizedState!==null&&n.alternate.memoizedState.cachePool!==null&&(s=n.alternate.memoizedState.cachePool.pool),c=null,n.memoizedState!==null&&n.memoizedState.cachePool!==null&&(c=n.memoizedState.cachePool.pool),c!==s&&(n.flags|=2048)),a!==e&&a&&(t.child.flags|=8192),lu(t,t.updateQueue),Qt(t),null);case 4:return Ue(),e===null&&pf(t.stateNode.containerInfo),Qt(t),null;case 10:return Yn(t.type),Qt(t),null;case 19:if(Z(Tt),n=t.memoizedState,n===null)return Qt(t),null;if(s=(t.flags&128)!==0,c=n.rendering,c===null)if(s)Zr(n,!1);else{if(la!==0||e!==null&&(e.flags&128)!==0)for(e=t.child;e!==null;){if(c=Wn(e),c!==null){for(t.flags|=128,Zr(n,!1),e=c.updateQueue,t.updateQueue=e,lu(t,e),t.subtreeFlags=0,e=a,a=t.child;a!==null;)Hi(a,e),a=a.sibling;return T(Tt,Tt.current&1|2),mt&&Tn(t,n.treeForkCount),t.child}e=e.sibling}n.tail!==null&&qt()>uu&&(t.flags|=128,s=!0,Zr(n,!1),t.lanes=4194304)}else{if(!s)if(e=Wn(c),e!==null){if(t.flags|=128,s=!0,e=e.updateQueue,t.updateQueue=e,lu(t,e),Zr(n,!0),n.tail===null&&n.tailMode==="hidden"&&!c.alternate&&!mt)return Qt(t),null}else 2*qt()-n.renderingStartTime>uu&&a!==536870912&&(t.flags|=128,s=!0,Zr(n,!1),t.lanes=4194304);n.isBackwards?(c.sibling=t.child,t.child=c):(e=n.last,e!==null?e.sibling=c:t.child=c,n.last=c)}return n.tail!==null?(e=n.tail,n.rendering=e,n.tail=e.sibling,n.renderingStartTime=qt(),e.sibling=null,a=Tt.current,T(Tt,s?a&1|2:a&1),mt&&Tn(t,n.treeForkCount),e):(Qt(t),null);case 22:case 23:return Gt(t),It(),n=t.memoizedState!==null,e!==null?e.memoizedState!==null!==n&&(t.flags|=8192):n&&(t.flags|=8192),n?(a&536870912)!==0&&(t.flags&128)===0&&(Qt(t),t.subtreeFlags&6&&(t.flags|=8192)):Qt(t),a=t.updateQueue,a!==null&&lu(t,a.retryQueue),a=null,e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(a=e.memoizedState.cachePool.pool),n=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(n=t.memoizedState.cachePool.pool),n!==a&&(t.flags|=2048),e!==null&&Z(jo),null;case 24:return a=null,e!==null&&(a=e.memoizedState.cache),t.memoizedState.cache!==a&&(t.flags|=2048),Yn(Wt),Qt(t),null;case 25:return null;case 30:return null}throw Error(u(156,t.tag))}function Dp(e,t){switch(Jl(t),t.tag){case 1:return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 3:return Yn(Wt),Ue(),e=t.flags,(e&65536)!==0&&(e&128)===0?(t.flags=e&-65537|128,t):null;case 26:case 27:case 5:return _e(t),null;case 31:if(t.memoizedState!==null){if(Gt(t),t.alternate===null)throw Error(u(340));dl()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 13:if(Gt(t),e=t.memoizedState,e!==null&&e.dehydrated!==null){if(t.alternate===null)throw Error(u(340));dl()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 19:return Z(Tt),null;case 4:return Ue(),null;case 10:return Yn(t.type),null;case 22:case 23:return Gt(t),It(),e!==null&&Z(jo),e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 24:return Yn(Wt),null;case 25:return null;default:return null}}function Sd(e,t){switch(Jl(t),t.tag){case 3:Yn(Wt),Ue();break;case 26:case 27:case 5:_e(t);break;case 4:Ue();break;case 31:t.memoizedState!==null&&Gt(t);break;case 13:Gt(t);break;case 19:Z(Tt);break;case 10:Yn(t.type);break;case 22:case 23:Gt(t),It(),e!==null&&Z(jo);break;case 24:Yn(Wt)}}function Qr(e,t){try{var a=t.updateQueue,n=a!==null?a.lastEffect:null;if(n!==null){var s=n.next;a=s;do{if((a.tag&e)===e){n=void 0;var c=a.create,d=a.inst;n=c(),d.destroy=n}a=a.next}while(a!==s)}}catch(w){Ht(t,t.return,w)}}function ls(e,t,a){try{var n=t.updateQueue,s=n!==null?n.lastEffect:null;if(s!==null){var c=s.next;n=c;do{if((n.tag&e)===e){var d=n.inst,w=d.destroy;if(w!==void 0){d.destroy=void 0,s=t;var L=a,re=w;try{re()}catch(be){Ht(s,L,be)}}}n=n.next}while(n!==c)}}catch(be){Ht(t,t.return,be)}}function Td(e){var t=e.updateQueue;if(t!==null){var a=e.stateNode;try{$e(t,a)}catch(n){Ht(e,e.return,n)}}}function zd(e,t,a){a.props=ri(e.type,e.memoizedProps),a.state=e.memoizedState;try{a.componentWillUnmount()}catch(n){Ht(e,t,n)}}function Ir(e,t){try{var a=e.ref;if(a!==null){switch(e.tag){case 26:case 27:case 5:var n=e.stateNode;break;case 30:n=e.stateNode;break;default:n=e.stateNode}typeof a=="function"?e.refCleanup=a(n):a.current=n}}catch(s){Ht(e,t,s)}}function qo(e,t){var a=e.ref,n=e.refCleanup;if(a!==null)if(typeof n=="function")try{n()}catch(s){Ht(e,t,s)}finally{e.refCleanup=null,e=e.alternate,e!=null&&(e.refCleanup=null)}else if(typeof a=="function")try{a(null)}catch(s){Ht(e,t,s)}else a.current=null}function Cd(e){var t=e.type,a=e.memoizedProps,n=e.stateNode;try{e:switch(t){case"button":case"input":case"select":case"textarea":a.autoFocus&&n.focus();break e;case"img":a.src?n.src=a.src:a.srcSet&&(n.srcset=a.srcSet)}}catch(s){Ht(e,e.return,s)}}function X0(e,t,a){try{var n=e.stateNode;ev(n,e.type,a,t),n[ba]=t}catch(s){Ht(e,e.return,s)}}function _d(e){return e.tag===5||e.tag===3||e.tag===26||e.tag===27&&hs(e.type)||e.tag===4}function P0(e){e:for(;;){for(;e.sibling===null;){if(e.return===null||_d(e.return))return null;e=e.return}for(e.sibling.return=e.return,e=e.sibling;e.tag!==5&&e.tag!==6&&e.tag!==18;){if(e.tag===27&&hs(e.type)||e.flags&2||e.child===null||e.tag===4)continue e;e.child.return=e,e=e.child}if(!(e.flags&2))return e.stateNode}}function Z0(e,t,a){var n=e.tag;if(n===5||n===6)e=e.stateNode,t?(a.nodeType===9?a.body:a.nodeName==="HTML"?a.ownerDocument.body:a).insertBefore(e,t):(t=a.nodeType===9?a.body:a.nodeName==="HTML"?a.ownerDocument.body:a,t.appendChild(e),a=a._reactRootContainer,a!=null||t.onclick!==null||(t.onclick=cn));else if(n!==4&&(n===27&&hs(e.type)&&(a=e.stateNode,t=null),e=e.child,e!==null))for(Z0(e,t,a),e=e.sibling;e!==null;)Z0(e,t,a),e=e.sibling}function su(e,t,a){var n=e.tag;if(n===5||n===6)e=e.stateNode,t?a.insertBefore(e,t):a.appendChild(e);else if(n!==4&&(n===27&&hs(e.type)&&(a=e.stateNode),e=e.child,e!==null))for(su(e,t,a),e=e.sibling;e!==null;)su(e,t,a),e=e.sibling}function Ad(e){var t=e.stateNode,a=e.memoizedProps;try{for(var n=e.type,s=t.attributes;s.length;)t.removeAttributeNode(s[0]);Oa(t,n,a),t[ra]=e,t[ba]=a}catch(c){Ht(e,e.return,c)}}var wl=!1,da=!1,Q0=!1,Ed=typeof WeakSet=="function"?WeakSet:Set,Ta=null;function Up(e,t){if(e=e.containerInfo,yf=_u,e=Lc(e),Xs(e)){if("selectionStart"in e)var a={start:e.selectionStart,end:e.selectionEnd};else e:{a=(a=e.ownerDocument)&&a.defaultView||window;var n=a.getSelection&&a.getSelection();if(n&&n.rangeCount!==0){a=n.anchorNode;var s=n.anchorOffset,c=n.focusNode;n=n.focusOffset;try{a.nodeType,c.nodeType}catch{a=null;break e}var d=0,w=-1,L=-1,re=0,be=0,Ce=e,he=null;t:for(;;){for(var ve;Ce!==a||s!==0&&Ce.nodeType!==3||(w=d+s),Ce!==c||n!==0&&Ce.nodeType!==3||(L=d+n),Ce.nodeType===3&&(d+=Ce.nodeValue.length),(ve=Ce.firstChild)!==null;)he=Ce,Ce=ve;for(;;){if(Ce===e)break t;if(he===a&&++re===s&&(w=d),he===c&&++be===n&&(L=d),(ve=Ce.nextSibling)!==null)break;Ce=he,he=Ce.parentNode}Ce=ve}a=w===-1||L===-1?null:{start:w,end:L}}else a=null}a=a||{start:0,end:0}}else a=null;for(xf={focusedElem:e,selectionRange:a},_u=!1,Ta=t;Ta!==null;)if(t=Ta,e=t.child,(t.subtreeFlags&1028)!==0&&e!==null)e.return=t,Ta=e;else for(;Ta!==null;){switch(t=Ta,c=t.alternate,e=t.flags,t.tag){case 0:if((e&4)!==0&&(e=t.updateQueue,e=e!==null?e.events:null,e!==null))for(a=0;a<e.length;a++)s=e[a],s.ref.impl=s.nextImpl;break;case 11:case 15:break;case 1:if((e&1024)!==0&&c!==null){e=void 0,a=t,s=c.memoizedProps,c=c.memoizedState,n=a.stateNode;try{var Fe=ri(a.type,s);e=n.getSnapshotBeforeUpdate(Fe,c),n.__reactInternalSnapshotBeforeUpdate=e}catch(Je){Ht(a,a.return,Je)}}break;case 3:if((e&1024)!==0){if(e=t.stateNode.containerInfo,a=e.nodeType,a===9)Mf(e);else if(a===1)switch(e.nodeName){case"HEAD":case"HTML":case"BODY":Mf(e);break;default:e.textContent=""}}break;case 5:case 26:case 27:case 6:case 4:case 17:break;default:if((e&1024)!==0)throw Error(u(163))}if(e=t.sibling,e!==null){e.return=t.return,Ta=e;break}Ta=t.return}}function Rd(e,t,a){var n=a.flags;switch(a.tag){case 0:case 11:case 15:Ml(e,a),n&4&&Qr(5,a);break;case 1:if(Ml(e,a),n&4)if(e=a.stateNode,t===null)try{e.componentDidMount()}catch(d){Ht(a,a.return,d)}else{var s=ri(a.type,t.memoizedProps);t=t.memoizedState;try{e.componentDidUpdate(s,t,e.__reactInternalSnapshotBeforeUpdate)}catch(d){Ht(a,a.return,d)}}n&64&&Td(a),n&512&&Ir(a,a.return);break;case 3:if(Ml(e,a),n&64&&(e=a.updateQueue,e!==null)){if(t=null,a.child!==null)switch(a.child.tag){case 27:case 5:t=a.child.stateNode;break;case 1:t=a.child.stateNode}try{$e(e,t)}catch(d){Ht(a,a.return,d)}}break;case 27:t===null&&n&4&&Ad(a);case 26:case 5:Ml(e,a),t===null&&n&4&&Cd(a),n&512&&Ir(a,a.return);break;case 12:Ml(e,a);break;case 31:Ml(e,a),n&4&&Ud(e,a);break;case 13:Ml(e,a),n&4&&Od(e,a),n&64&&(e=a.memoizedState,e!==null&&(e=e.dehydrated,e!==null&&(a=Fp.bind(null,a),rv(e,a))));break;case 22:if(n=a.memoizedState!==null||wl,!n){t=t!==null&&t.memoizedState!==null||da,s=wl;var c=da;wl=n,(da=t)&&!c?Sl(e,a,(a.subtreeFlags&8772)!==0):Ml(e,a),wl=s,da=c}break;case 30:break;default:Ml(e,a)}}function Nd(e){var t=e.alternate;t!==null&&(e.alternate=null,Nd(t)),e.child=null,e.deletions=null,e.sibling=null,e.tag===5&&(t=e.stateNode,t!==null&&Es(t)),e.stateNode=null,e.return=null,e.dependencies=null,e.memoizedProps=null,e.memoizedState=null,e.pendingProps=null,e.stateNode=null,e.updateQueue=null}var Jt=null,dn=!1;function bl(e,t,a){for(a=a.child;a!==null;)Dd(e,t,a),a=a.sibling}function Dd(e,t,a){if(wa&&typeof wa.onCommitFiberUnmount=="function")try{wa.onCommitFiberUnmount(ln,a)}catch{}switch(a.tag){case 26:da||qo(a,t),bl(e,t,a),a.memoizedState?a.memoizedState.count--:a.stateNode&&(a=a.stateNode,a.parentNode.removeChild(a));break;case 27:da||qo(a,t);var n=Jt,s=dn;hs(a.type)&&(Jt=a.stateNode,dn=!1),bl(e,t,a),sc(a.stateNode),Jt=n,dn=s;break;case 5:da||qo(a,t);case 6:if(n=Jt,s=dn,Jt=null,bl(e,t,a),Jt=n,dn=s,Jt!==null)if(dn)try{(Jt.nodeType===9?Jt.body:Jt.nodeName==="HTML"?Jt.ownerDocument.body:Jt).removeChild(a.stateNode)}catch(c){Ht(a,t,c)}else try{Jt.removeChild(a.stateNode)}catch(c){Ht(a,t,c)}break;case 18:Jt!==null&&(dn?(e=Jt,Tm(e.nodeType===9?e.body:e.nodeName==="HTML"?e.ownerDocument.body:e,a.stateNode),lr(e)):Tm(Jt,a.stateNode));break;case 4:n=Jt,s=dn,Jt=a.stateNode.containerInfo,dn=!0,bl(e,t,a),Jt=n,dn=s;break;case 0:case 11:case 14:case 15:ls(2,a,t),da||ls(4,a,t),bl(e,t,a);break;case 1:da||(qo(a,t),n=a.stateNode,typeof n.componentWillUnmount=="function"&&zd(a,t,n)),bl(e,t,a);break;case 21:bl(e,t,a);break;case 22:da=(n=da)||a.memoizedState!==null,bl(e,t,a),da=n;break;default:bl(e,t,a)}}function Ud(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null))){e=e.dehydrated;try{lr(e)}catch(a){Ht(t,t.return,a)}}}function Od(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null&&(e=e.dehydrated,e!==null))))try{lr(e)}catch(a){Ht(t,t.return,a)}}function Op(e){switch(e.tag){case 31:case 13:case 19:var t=e.stateNode;return t===null&&(t=e.stateNode=new Ed),t;case 22:return e=e.stateNode,t=e._retryCache,t===null&&(t=e._retryCache=new Ed),t;default:throw Error(u(435,e.tag))}}function iu(e,t){var a=Op(e);t.forEach(function(n){if(!a.has(n)){a.add(n);var s=Vp.bind(null,e,n);n.then(s,s)}})}function mn(e,t){var a=t.deletions;if(a!==null)for(var n=0;n<a.length;n++){var s=a[n],c=e,d=t,w=d;e:for(;w!==null;){switch(w.tag){case 27:if(hs(w.type)){Jt=w.stateNode,dn=!1;break e}break;case 5:Jt=w.stateNode,dn=!1;break e;case 3:case 4:Jt=w.stateNode.containerInfo,dn=!0;break e}w=w.return}if(Jt===null)throw Error(u(160));Dd(c,d,s),Jt=null,dn=!1,c=s.alternate,c!==null&&(c.return=null),s.return=null}if(t.subtreeFlags&13886)for(t=t.child;t!==null;)kd(t,e),t=t.sibling}var vo=null;function kd(e,t){var a=e.alternate,n=e.flags;switch(e.tag){case 0:case 11:case 14:case 15:mn(t,e),pn(e),n&4&&(ls(3,e,e.return),Qr(3,e),ls(5,e,e.return));break;case 1:mn(t,e),pn(e),n&512&&(da||a===null||qo(a,a.return)),n&64&&wl&&(e=e.updateQueue,e!==null&&(n=e.callbacks,n!==null&&(a=e.shared.hiddenCallbacks,e.shared.hiddenCallbacks=a===null?n:a.concat(n))));break;case 26:var s=vo;if(mn(t,e),pn(e),n&512&&(da||a===null||qo(a,a.return)),n&4){var c=a!==null?a.memoizedState:null;if(n=e.memoizedState,a===null)if(n===null)if(e.stateNode===null){e:{n=e.type,a=e.memoizedProps,s=s.ownerDocument||s;t:switch(n){case"title":c=s.getElementsByTagName("title")[0],(!c||c[el]||c[ra]||c.namespaceURI==="http://www.w3.org/2000/svg"||c.hasAttribute("itemprop"))&&(c=s.createElement(n),s.head.insertBefore(c,s.querySelector("head > title"))),Oa(c,n,a),c[ra]=e,aa(c),n=c;break e;case"link":var d=km("link","href",s).get(n+(a.href||""));if(d){for(var w=0;w<d.length;w++)if(c=d[w],c.getAttribute("href")===(a.href==null||a.href===""?null:a.href)&&c.getAttribute("rel")===(a.rel==null?null:a.rel)&&c.getAttribute("title")===(a.title==null?null:a.title)&&c.getAttribute("crossorigin")===(a.crossOrigin==null?null:a.crossOrigin)){d.splice(w,1);break t}}c=s.createElement(n),Oa(c,n,a),s.head.appendChild(c);break;case"meta":if(d=km("meta","content",s).get(n+(a.content||""))){for(w=0;w<d.length;w++)if(c=d[w],c.getAttribute("content")===(a.content==null?null:""+a.content)&&c.getAttribute("name")===(a.name==null?null:a.name)&&c.getAttribute("property")===(a.property==null?null:a.property)&&c.getAttribute("http-equiv")===(a.httpEquiv==null?null:a.httpEquiv)&&c.getAttribute("charset")===(a.charSet==null?null:a.charSet)){d.splice(w,1);break t}}c=s.createElement(n),Oa(c,n,a),s.head.appendChild(c);break;default:throw Error(u(468,n))}c[ra]=e,aa(c),n=c}e.stateNode=n}else Bm(s,e.type,e.stateNode);else e.stateNode=Om(s,n,e.memoizedProps);else c!==n?(c===null?a.stateNode!==null&&(a=a.stateNode,a.parentNode.removeChild(a)):c.count--,n===null?Bm(s,e.type,e.stateNode):Om(s,n,e.memoizedProps)):n===null&&e.stateNode!==null&&X0(e,e.memoizedProps,a.memoizedProps)}break;case 27:mn(t,e),pn(e),n&512&&(da||a===null||qo(a,a.return)),a!==null&&n&4&&X0(e,e.memoizedProps,a.memoizedProps);break;case 5:if(mn(t,e),pn(e),n&512&&(da||a===null||qo(a,a.return)),e.flags&32){s=e.stateNode;try{Ie(s,"")}catch(Fe){Ht(e,e.return,Fe)}}n&4&&e.stateNode!=null&&(s=e.memoizedProps,X0(e,s,a!==null?a.memoizedProps:s)),n&1024&&(Q0=!0);break;case 6:if(mn(t,e),pn(e),n&4){if(e.stateNode===null)throw Error(u(162));n=e.memoizedProps,a=e.stateNode;try{a.nodeValue=n}catch(Fe){Ht(e,e.return,Fe)}}break;case 3:if(Su=null,s=vo,vo=bu(t.containerInfo),mn(t,e),vo=s,pn(e),n&4&&a!==null&&a.memoizedState.isDehydrated)try{lr(t.containerInfo)}catch(Fe){Ht(e,e.return,Fe)}Q0&&(Q0=!1,Bd(e));break;case 4:n=vo,vo=bu(e.stateNode.containerInfo),mn(t,e),pn(e),vo=n;break;case 12:mn(t,e),pn(e);break;case 31:mn(t,e),pn(e),n&4&&(n=e.updateQueue,n!==null&&(e.updateQueue=null,iu(e,n)));break;case 13:mn(t,e),pn(e),e.child.flags&8192&&e.memoizedState!==null!=(a!==null&&a.memoizedState!==null)&&(cu=qt()),n&4&&(n=e.updateQueue,n!==null&&(e.updateQueue=null,iu(e,n)));break;case 22:s=e.memoizedState!==null;var L=a!==null&&a.memoizedState!==null,re=wl,be=da;if(wl=re||s,da=be||L,mn(t,e),da=be,wl=re,pn(e),n&8192)e:for(t=e.stateNode,t._visibility=s?t._visibility&-2:t._visibility|1,s&&(a===null||L||wl||da||ci(e)),a=null,t=e;;){if(t.tag===5||t.tag===26){if(a===null){L=a=t;try{if(c=L.stateNode,s)d=c.style,typeof d.setProperty=="function"?d.setProperty("display","none","important"):d.display="none";else{w=L.stateNode;var Ce=L.memoizedProps.style,he=Ce!=null&&Ce.hasOwnProperty("display")?Ce.display:null;w.style.display=he==null||typeof he=="boolean"?"":(""+he).trim()}}catch(Fe){Ht(L,L.return,Fe)}}}else if(t.tag===6){if(a===null){L=t;try{L.stateNode.nodeValue=s?"":L.memoizedProps}catch(Fe){Ht(L,L.return,Fe)}}}else if(t.tag===18){if(a===null){L=t;try{var ve=L.stateNode;s?zm(ve,!0):zm(L.stateNode,!1)}catch(Fe){Ht(L,L.return,Fe)}}}else if((t.tag!==22&&t.tag!==23||t.memoizedState===null||t===e)&&t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break e;for(;t.sibling===null;){if(t.return===null||t.return===e)break e;a===t&&(a=null),t=t.return}a===t&&(a=null),t.sibling.return=t.return,t=t.sibling}n&4&&(n=e.updateQueue,n!==null&&(a=n.retryQueue,a!==null&&(n.retryQueue=null,iu(e,a))));break;case 19:mn(t,e),pn(e),n&4&&(n=e.updateQueue,n!==null&&(e.updateQueue=null,iu(e,n)));break;case 30:break;case 21:break;default:mn(t,e),pn(e)}}function pn(e){var t=e.flags;if(t&2){try{for(var a,n=e.return;n!==null;){if(_d(n)){a=n;break}n=n.return}if(a==null)throw Error(u(160));switch(a.tag){case 27:var s=a.stateNode,c=P0(e);su(e,c,s);break;case 5:var d=a.stateNode;a.flags&32&&(Ie(d,""),a.flags&=-33);var w=P0(e);su(e,w,d);break;case 3:case 4:var L=a.stateNode.containerInfo,re=P0(e);Z0(e,re,L);break;default:throw Error(u(161))}}catch(be){Ht(e,e.return,be)}e.flags&=-3}t&4096&&(e.flags&=-4097)}function Bd(e){if(e.subtreeFlags&1024)for(e=e.child;e!==null;){var t=e;Bd(t),t.tag===5&&t.flags&1024&&t.stateNode.reset(),e=e.sibling}}function Ml(e,t){if(t.subtreeFlags&8772)for(t=t.child;t!==null;)Rd(e,t.alternate,t),t=t.sibling}function ci(e){for(e=e.child;e!==null;){var t=e;switch(t.tag){case 0:case 11:case 14:case 15:ls(4,t,t.return),ci(t);break;case 1:qo(t,t.return);var a=t.stateNode;typeof a.componentWillUnmount=="function"&&zd(t,t.return,a),ci(t);break;case 27:sc(t.stateNode);case 26:case 5:qo(t,t.return),ci(t);break;case 22:t.memoizedState===null&&ci(t);break;case 30:ci(t);break;default:ci(t)}e=e.sibling}}function Sl(e,t,a){for(a=a&&(t.subtreeFlags&8772)!==0,t=t.child;t!==null;){var n=t.alternate,s=e,c=t,d=c.flags;switch(c.tag){case 0:case 11:case 15:Sl(s,c,a),Qr(4,c);break;case 1:if(Sl(s,c,a),n=c,s=n.stateNode,typeof s.componentDidMount=="function")try{s.componentDidMount()}catch(re){Ht(n,n.return,re)}if(n=c,s=n.updateQueue,s!==null){var w=n.stateNode;try{var L=s.shared.hiddenCallbacks;if(L!==null)for(s.shared.hiddenCallbacks=null,s=0;s<L.length;s++)it(L[s],w)}catch(re){Ht(n,n.return,re)}}a&&d&64&&Td(c),Ir(c,c.return);break;case 27:Ad(c);case 26:case 5:Sl(s,c,a),a&&n===null&&d&4&&Cd(c),Ir(c,c.return);break;case 12:Sl(s,c,a);break;case 31:Sl(s,c,a),a&&d&4&&Ud(s,c);break;case 13:Sl(s,c,a),a&&d&4&&Od(s,c);break;case 22:c.memoizedState===null&&Sl(s,c,a),Ir(c,c.return);break;case 30:break;default:Sl(s,c,a)}t=t.sibling}}function I0(e,t){var a=null;e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(a=e.memoizedState.cachePool.pool),e=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(e=t.memoizedState.cachePool.pool),e!==a&&(e!=null&&e.refCount++,a!=null&&ns(a))}function J0(e,t){e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&ns(e))}function go(e,t,a,n){if(t.subtreeFlags&10256)for(t=t.child;t!==null;)Hd(e,t,a,n),t=t.sibling}function Hd(e,t,a,n){var s=t.flags;switch(t.tag){case 0:case 11:case 15:go(e,t,a,n),s&2048&&Qr(9,t);break;case 1:go(e,t,a,n);break;case 3:go(e,t,a,n),s&2048&&(e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&ns(e)));break;case 12:if(s&2048){go(e,t,a,n),e=t.stateNode;try{var c=t.memoizedProps,d=c.id,w=c.onPostCommit;typeof w=="function"&&w(d,t.alternate===null?"mount":"update",e.passiveEffectDuration,-0)}catch(L){Ht(t,t.return,L)}}else go(e,t,a,n);break;case 31:go(e,t,a,n);break;case 13:go(e,t,a,n);break;case 23:break;case 22:c=t.stateNode,d=t.alternate,t.memoizedState!==null?c._visibility&2?go(e,t,a,n):Jr(e,t):c._visibility&2?go(e,t,a,n):(c._visibility|=2,Pi(e,t,a,n,(t.subtreeFlags&10256)!==0||!1)),s&2048&&I0(d,t);break;case 24:go(e,t,a,n),s&2048&&J0(t.alternate,t);break;default:go(e,t,a,n)}}function Pi(e,t,a,n,s){for(s=s&&((t.subtreeFlags&10256)!==0||!1),t=t.child;t!==null;){var c=e,d=t,w=a,L=n,re=d.flags;switch(d.tag){case 0:case 11:case 15:Pi(c,d,w,L,s),Qr(8,d);break;case 23:break;case 22:var be=d.stateNode;d.memoizedState!==null?be._visibility&2?Pi(c,d,w,L,s):Jr(c,d):(be._visibility|=2,Pi(c,d,w,L,s)),s&&re&2048&&I0(d.alternate,d);break;case 24:Pi(c,d,w,L,s),s&&re&2048&&J0(d.alternate,d);break;default:Pi(c,d,w,L,s)}t=t.sibling}}function Jr(e,t){if(t.subtreeFlags&10256)for(t=t.child;t!==null;){var a=e,n=t,s=n.flags;switch(n.tag){case 22:Jr(a,n),s&2048&&I0(n.alternate,n);break;case 24:Jr(a,n),s&2048&&J0(n.alternate,n);break;default:Jr(a,n)}t=t.sibling}}var $r=8192;function Zi(e,t,a){if(e.subtreeFlags&$r)for(e=e.child;e!==null;)Ld(e,t,a),e=e.sibling}function Ld(e,t,a){switch(e.tag){case 26:Zi(e,t,a),e.flags&$r&&e.memoizedState!==null&&wv(a,vo,e.memoizedState,e.memoizedProps);break;case 5:Zi(e,t,a);break;case 3:case 4:var n=vo;vo=bu(e.stateNode.containerInfo),Zi(e,t,a),vo=n;break;case 22:e.memoizedState===null&&(n=e.alternate,n!==null&&n.memoizedState!==null?(n=$r,$r=16777216,Zi(e,t,a),$r=n):Zi(e,t,a));break;default:Zi(e,t,a)}}function jd(e){var t=e.alternate;if(t!==null&&(e=t.child,e!==null)){t.child=null;do t=e.sibling,e.sibling=null,e=t;while(e!==null)}}function ec(e){var t=e.deletions;if((e.flags&16)!==0){if(t!==null)for(var a=0;a<t.length;a++){var n=t[a];Ta=n,Gd(n,e)}jd(e)}if(e.subtreeFlags&10256)for(e=e.child;e!==null;)qd(e),e=e.sibling}function qd(e){switch(e.tag){case 0:case 11:case 15:ec(e),e.flags&2048&&ls(9,e,e.return);break;case 3:ec(e);break;case 12:ec(e);break;case 22:var t=e.stateNode;e.memoizedState!==null&&t._visibility&2&&(e.return===null||e.return.tag!==13)?(t._visibility&=-3,ru(e)):ec(e);break;default:ec(e)}}function ru(e){var t=e.deletions;if((e.flags&16)!==0){if(t!==null)for(var a=0;a<t.length;a++){var n=t[a];Ta=n,Gd(n,e)}jd(e)}for(e=e.child;e!==null;){switch(t=e,t.tag){case 0:case 11:case 15:ls(8,t,t.return),ru(t);break;case 22:a=t.stateNode,a._visibility&2&&(a._visibility&=-3,ru(t));break;default:ru(t)}e=e.sibling}}function Gd(e,t){for(;Ta!==null;){var a=Ta;switch(a.tag){case 0:case 11:case 15:ls(8,a,t);break;case 23:case 22:if(a.memoizedState!==null&&a.memoizedState.cachePool!==null){var n=a.memoizedState.cachePool.pool;n!=null&&n.refCount++}break;case 24:ns(a.memoizedState.cache)}if(n=a.child,n!==null)n.return=a,Ta=n;else e:for(a=e;Ta!==null;){n=Ta;var s=n.sibling,c=n.return;if(Nd(n),n===a){Ta=null;break e}if(s!==null){s.return=c,Ta=s;break e}Ta=c}}}var kp={getCacheForType:function(e){var t=na(Wt),a=t.data.get(e);return a===void 0&&(a=e(),t.data.set(e,a)),a},cacheSignal:function(){return na(Wt).controller.signal}},Bp=typeof WeakMap=="function"?WeakMap:Map,Dt=0,Ft=null,gt=null,bt=0,Bt=0,An=null,ss=!1,Qi=!1,$0=!1,Tl=0,la=0,is=0,ui=0,ef=0,En=0,Ii=0,tc=null,vn=null,tf=!1,cu=0,Fd=0,uu=1/0,fu=null,rs=null,ga=0,cs=null,Ji=null,zl=0,af=0,nf=null,Vd=null,ac=0,of=null;function Rn(){return(Dt&2)!==0&&bt!==0?bt&-bt:G.T!==null?ff():yi()}function Kd(){if(En===0)if((bt&536870912)===0||mt){var e=xn;xn<<=1,(xn&3932160)===0&&(xn=262144),En=e}else En=536870912;return e=wt.current,e!==null&&(e.flags|=32),En}function gn(e,t,a){(e===Ft&&(Bt===2||Bt===9)||e.cancelPendingCommit!==null)&&($i(e,0),us(e,bt,En,!1)),$o(e,a),((Dt&2)===0||e!==Ft)&&(e===Ft&&((Dt&2)===0&&(ui|=a),la===4&&us(e,bt,En,!1)),Go(e))}function Yd(e,t,a){if((Dt&6)!==0)throw Error(u(327));var n=!a&&(t&127)===0&&(t&e.expiredLanes)===0||So(e,t),s=n?jp(e,t):sf(e,t,!0),c=n;do{if(s===0){Qi&&!n&&us(e,t,0,!1);break}else{if(a=e.current.alternate,c&&!Hp(a)){s=sf(e,t,!1),c=!1;continue}if(s===2){if(c=t,e.errorRecoveryDisabledLanes&c)var d=0;else d=e.pendingLanes&-536870913,d=d!==0?d:d&536870912?536870912:0;if(d!==0){t=d;e:{var w=e;s=tc;var L=w.current.memoizedState.isDehydrated;if(L&&($i(w,d).flags|=256),d=sf(w,d,!1),d!==2){if($0&&!L){w.errorRecoveryDisabledLanes|=c,ui|=c,s=4;break e}c=vn,vn=s,c!==null&&(vn===null?vn=c:vn.push.apply(vn,c))}s=d}if(c=!1,s!==2)continue}}if(s===1){$i(e,0),us(e,t,0,!0);break}e:{switch(n=e,c=s,c){case 0:case 1:throw Error(u(345));case 4:if((t&4194048)!==t)break;case 6:us(n,t,En,!ss);break e;case 2:vn=null;break;case 3:case 5:break;default:throw Error(u(329))}if((t&62914560)===t&&(s=cu+300-qt(),10<s)){if(us(n,t,En,!ss),Ll(n,0,!0)!==0)break e;zl=t,n.timeoutHandle=Mm(Wd.bind(null,n,a,vn,fu,tf,t,En,ui,Ii,ss,c,"Throttled",-0,0),s);break e}Wd(n,a,vn,fu,tf,t,En,ui,Ii,ss,c,null,-0,0)}}break}while(!0);Go(e)}function Wd(e,t,a,n,s,c,d,w,L,re,be,Ce,he,ve){if(e.timeoutHandle=-1,Ce=t.subtreeFlags,Ce&8192||(Ce&16785408)===16785408){Ce={stylesheets:null,count:0,imgCount:0,imgBytes:0,suspenseyImages:[],waitingForImages:!0,waitingForViewTransition:!1,unsuspend:cn},Ld(t,c,Ce);var Fe=(c&62914560)===c?cu-qt():(c&4194048)===c?Fd-qt():0;if(Fe=bv(Ce,Fe),Fe!==null){zl=c,e.cancelPendingCommit=Fe(em.bind(null,e,t,c,a,n,s,d,w,L,be,Ce,null,he,ve)),us(e,c,d,!re);return}}em(e,t,c,a,n,s,d,w,L)}function Hp(e){for(var t=e;;){var a=t.tag;if((a===0||a===11||a===15)&&t.flags&16384&&(a=t.updateQueue,a!==null&&(a=a.stores,a!==null)))for(var n=0;n<a.length;n++){var s=a[n],c=s.getSnapshot;s=s.value;try{if(!Ra(c(),s))return!1}catch{return!1}}if(a=t.child,t.subtreeFlags&16384&&a!==null)a.return=t,t=a;else{if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return!0;t=t.return}t.sibling.return=t.return,t=t.sibling}}return!0}function us(e,t,a,n){t&=~ef,t&=~ui,e.suspendedLanes|=t,e.pingedLanes&=~t,n&&(e.warmLanes|=t),n=e.expirationTimes;for(var s=t;0<s;){var c=31-Vt(s),d=1<<c;n[c]=-1,s&=~d}a!==0&&Cs(e,a,t)}function hu(){return(Dt&6)===0?(nc(0),!1):!0}function lf(){if(gt!==null){if(Bt===0)var e=gt.return;else e=gt,Ma=Ga=null,b0(e),pe=null,fe=0,e=gt;for(;e!==null;)Sd(e.alternate,e),e=e.return;gt=null}}function $i(e,t){var a=e.timeoutHandle;a!==-1&&(e.timeoutHandle=-1,nv(a)),a=e.cancelPendingCommit,a!==null&&(e.cancelPendingCommit=null,a()),zl=0,lf(),Ft=e,gt=a=fo(e.current,null),bt=t,Bt=0,An=null,ss=!1,Qi=So(e,t),$0=!1,Ii=En=ef=ui=is=la=0,vn=tc=null,tf=!1,(t&8)!==0&&(t|=t&32);var n=e.entangledLanes;if(n!==0)for(e=e.entanglements,n&=t;0<n;){var s=31-Vt(n),c=1<<s;t|=e[s],n&=~c}return Tl=t,rl(),a}function Xd(e,t){et=null,G.H=Xr,t===M||t===N?(t=te(),Bt=3):t===_?(t=te(),Bt=4):Bt=t===H0?8:t!==null&&typeof t=="object"&&typeof t.then=="function"?6:1,An=t,gt===null&&(la=1,tu(e,qa(t,e.current)))}function Pd(){var e=wt.current;return e===null?!0:(bt&4194048)===bt?St===null:(bt&62914560)===bt||(bt&536870912)!==0?e===St:!1}function Zd(){var e=G.H;return G.H=Xr,e===null?Xr:e}function Qd(){var e=G.A;return G.A=kp,e}function du(){la=4,ss||(bt&4194048)!==bt&&wt.current!==null||(Qi=!0),(is&134217727)===0&&(ui&134217727)===0||Ft===null||us(Ft,bt,En,!1)}function sf(e,t,a){var n=Dt;Dt|=2;var s=Zd(),c=Qd();(Ft!==e||bt!==t)&&(fu=null,$i(e,t)),t=!1;var d=la;e:do try{if(Bt!==0&&gt!==null){var w=gt,L=An;switch(Bt){case 8:lf(),d=6;break e;case 3:case 2:case 9:case 6:wt.current===null&&(t=!0);var re=Bt;if(Bt=0,An=null,er(e,w,L,re),a&&Qi){d=0;break e}break;default:re=Bt,Bt=0,An=null,er(e,w,L,re)}}Lp(),d=la;break}catch(be){Xd(e,be)}while(!0);return t&&e.shellSuspendCounter++,Ma=Ga=null,Dt=n,G.H=s,G.A=c,gt===null&&(Ft=null,bt=0,rl()),d}function Lp(){for(;gt!==null;)Id(gt)}function jp(e,t){var a=Dt;Dt|=2;var n=Zd(),s=Qd();Ft!==e||bt!==t?(fu=null,uu=qt()+500,$i(e,t)):Qi=So(e,t);e:do try{if(Bt!==0&&gt!==null){t=gt;var c=An;t:switch(Bt){case 1:Bt=0,An=null,er(e,t,c,1);break;case 2:case 9:if(A(c)){Bt=0,An=null,Jd(t);break}t=function(){Bt!==2&&Bt!==9||Ft!==e||(Bt=7),Go(e)},c.then(t,t);break e;case 3:Bt=7;break e;case 4:Bt=5;break e;case 7:A(c)?(Bt=0,An=null,Jd(t)):(Bt=0,An=null,er(e,t,c,7));break;case 5:var d=null;switch(gt.tag){case 26:d=gt.memoizedState;case 5:case 27:var w=gt;if(d?Hm(d):w.stateNode.complete){Bt=0,An=null;var L=w.sibling;if(L!==null)gt=L;else{var re=w.return;re!==null?(gt=re,mu(re)):gt=null}break t}}Bt=0,An=null,er(e,t,c,5);break;case 6:Bt=0,An=null,er(e,t,c,6);break;case 8:lf(),la=6;break e;default:throw Error(u(462))}}qp();break}catch(be){Xd(e,be)}while(!0);return Ma=Ga=null,G.H=n,G.A=s,Dt=a,gt!==null?0:(Ft=null,bt=0,rl(),la)}function qp(){for(;gt!==null&&!Xa();)Id(gt)}function Id(e){var t=bd(e.alternate,e,Tl);e.memoizedProps=e.pendingProps,t===null?mu(e):gt=t}function Jd(e){var t=e,a=t.alternate;switch(t.tag){case 15:case 0:t=pd(a,t,t.pendingProps,t.type,void 0,bt);break;case 11:t=pd(a,t,t.pendingProps,t.type.render,t.ref,bt);break;case 5:b0(t);default:Sd(a,t),t=gt=Hi(t,Tl),t=bd(a,t,Tl)}e.memoizedProps=e.pendingProps,t===null?mu(e):gt=t}function er(e,t,a,n){Ma=Ga=null,b0(t),pe=null,fe=0;var s=t.return;try{if(Ap(e,s,t,a,bt)){la=1,tu(e,qa(a,e.current)),gt=null;return}}catch(c){if(s!==null)throw gt=s,c;la=1,tu(e,qa(a,e.current)),gt=null;return}t.flags&32768?(mt||n===1?e=!0:Qi||(bt&536870912)!==0?e=!1:(ss=e=!0,(n===2||n===9||n===3||n===6)&&(n=wt.current,n!==null&&n.tag===13&&(n.flags|=16384))),$d(t,e)):mu(t)}function mu(e){var t=e;do{if((t.flags&32768)!==0){$d(t,ss);return}e=t.return;var a=Np(t.alternate,t,Tl);if(a!==null){gt=a;return}if(t=t.sibling,t!==null){gt=t;return}gt=t=e}while(t!==null);la===0&&(la=5)}function $d(e,t){do{var a=Dp(e.alternate,e);if(a!==null){a.flags&=32767,gt=a;return}if(a=e.return,a!==null&&(a.flags|=32768,a.subtreeFlags=0,a.deletions=null),!t&&(e=e.sibling,e!==null)){gt=e;return}gt=e=a}while(e!==null);la=6,gt=null}function em(e,t,a,n,s,c,d,w,L){e.cancelPendingCommit=null;do pu();while(ga!==0);if((Dt&6)!==0)throw Error(u(327));if(t!==null){if(t===e.current)throw Error(u(177));if(c=t.lanes|t.childLanes,c|=Pl,fr(e,a,c,d,w,L),e===Ft&&(gt=Ft=null,bt=0),Ji=t,cs=e,zl=a,af=c,nf=s,Vd=n,(t.subtreeFlags&10256)!==0||(t.flags&10256)!==0?(e.callbackNode=null,e.callbackPriority=0,Kp(Pa,function(){return lm(),null})):(e.callbackNode=null,e.callbackPriority=0),n=(t.flags&13878)!==0,(t.subtreeFlags&13878)!==0||n){n=G.T,G.T=null,s=j.p,j.p=2,d=Dt,Dt|=4;try{Up(e,t,a)}finally{Dt=d,j.p=s,G.T=n}}ga=1,tm(),am(),nm()}}function tm(){if(ga===1){ga=0;var e=cs,t=Ji,a=(t.flags&13878)!==0;if((t.subtreeFlags&13878)!==0||a){a=G.T,G.T=null;var n=j.p;j.p=2;var s=Dt;Dt|=4;try{kd(t,e);var c=xf,d=Lc(e.containerInfo),w=c.focusedElem,L=c.selectionRange;if(d!==w&&w&&w.ownerDocument&&Hc(w.ownerDocument.documentElement,w)){if(L!==null&&Xs(w)){var re=L.start,be=L.end;if(be===void 0&&(be=re),"selectionStart"in w)w.selectionStart=re,w.selectionEnd=Math.min(be,w.value.length);else{var Ce=w.ownerDocument||document,he=Ce&&Ce.defaultView||window;if(he.getSelection){var ve=he.getSelection(),Fe=w.textContent.length,Je=Math.min(L.start,Fe),jt=L.end===void 0?Je:Math.min(L.end,Fe);!ve.extend&&Je>jt&&(d=jt,jt=Je,Je=d);var $=co(w,Je),W=co(w,jt);if($&&W&&(ve.rangeCount!==1||ve.anchorNode!==$.node||ve.anchorOffset!==$.offset||ve.focusNode!==W.node||ve.focusOffset!==W.offset)){var ie=Ce.createRange();ie.setStart($.node,$.offset),ve.removeAllRanges(),Je>jt?(ve.addRange(ie),ve.extend(W.node,W.offset)):(ie.setEnd(W.node,W.offset),ve.addRange(ie))}}}}for(Ce=[],ve=w;ve=ve.parentNode;)ve.nodeType===1&&Ce.push({element:ve,left:ve.scrollLeft,top:ve.scrollTop});for(typeof w.focus=="function"&&w.focus(),w=0;w<Ce.length;w++){var Me=Ce[w];Me.element.scrollLeft=Me.left,Me.element.scrollTop=Me.top}}_u=!!yf,xf=yf=null}finally{Dt=s,j.p=n,G.T=a}}e.current=t,ga=2}}function am(){if(ga===2){ga=0;var e=cs,t=Ji,a=(t.flags&8772)!==0;if((t.subtreeFlags&8772)!==0||a){a=G.T,G.T=null;var n=j.p;j.p=2;var s=Dt;Dt|=4;try{Rd(e,t.alternate,t)}finally{Dt=s,j.p=n,G.T=a}}ga=3}}function nm(){if(ga===4||ga===3){ga=0,an();var e=cs,t=Ji,a=zl,n=Vd;(t.subtreeFlags&10256)!==0||(t.flags&10256)!==0?ga=5:(ga=0,Ji=cs=null,om(e,e.pendingLanes));var s=e.pendingLanes;if(s===0&&(rs=null),_s(a),t=t.stateNode,wa&&typeof wa.onCommitFiberRoot=="function")try{wa.onCommitFiberRoot(ln,t,void 0,(t.current.flags&128)===128)}catch{}if(n!==null){t=G.T,s=j.p,j.p=2,G.T=null;try{for(var c=e.onRecoverableError,d=0;d<n.length;d++){var w=n[d];c(w.value,{componentStack:w.stack})}}finally{G.T=t,j.p=s}}(zl&3)!==0&&pu(),Go(e),s=e.pendingLanes,(a&261930)!==0&&(s&42)!==0?e===of?ac++:(ac=0,of=e):ac=0,nc(0)}}function om(e,t){(e.pooledCacheLanes&=t)===0&&(t=e.pooledCache,t!=null&&(e.pooledCache=null,ns(t)))}function pu(){return tm(),am(),nm(),lm()}function lm(){if(ga!==5)return!1;var e=cs,t=af;af=0;var a=_s(zl),n=G.T,s=j.p;try{j.p=32>a?32:a,G.T=null,a=nf,nf=null;var c=cs,d=zl;if(ga=0,Ji=cs=null,zl=0,(Dt&6)!==0)throw Error(u(331));var w=Dt;if(Dt|=4,qd(c.current),Hd(c,c.current,d,a),Dt=w,nc(0,!1),wa&&typeof wa.onPostCommitFiberRoot=="function")try{wa.onPostCommitFiberRoot(ln,c)}catch{}return!0}finally{j.p=s,G.T=n,om(e,t)}}function sm(e,t,a){t=qa(a,t),t=B0(e.stateNode,t,2),e=je(e,t,2),e!==null&&($o(e,2),Go(e))}function Ht(e,t,a){if(e.tag===3)sm(e,e,a);else for(;t!==null;){if(t.tag===3){sm(t,e,a);break}else if(t.tag===1){var n=t.stateNode;if(typeof t.type.getDerivedStateFromError=="function"||typeof n.componentDidCatch=="function"&&(rs===null||!rs.has(n))){e=qa(a,e),a=id(2),n=je(t,a,2),n!==null&&(rd(a,n,t,e),$o(n,2),Go(n));break}}t=t.return}}function rf(e,t,a){var n=e.pingCache;if(n===null){n=e.pingCache=new Bp;var s=new Set;n.set(t,s)}else s=n.get(t),s===void 0&&(s=new Set,n.set(t,s));s.has(a)||($0=!0,s.add(a),e=Gp.bind(null,e,t,a),t.then(e,e))}function Gp(e,t,a){var n=e.pingCache;n!==null&&n.delete(t),e.pingedLanes|=e.suspendedLanes&a,e.warmLanes&=~a,Ft===e&&(bt&a)===a&&(la===4||la===3&&(bt&62914560)===bt&&300>qt()-cu?(Dt&2)===0&&$i(e,0):ef|=a,Ii===bt&&(Ii=0)),Go(e)}function im(e,t){t===0&&(t=ur()),e=qn(e,t),e!==null&&($o(e,t),Go(e))}function Fp(e){var t=e.memoizedState,a=0;t!==null&&(a=t.retryLane),im(e,a)}function Vp(e,t){var a=0;switch(e.tag){case 31:case 13:var n=e.stateNode,s=e.memoizedState;s!==null&&(a=s.retryLane);break;case 19:n=e.stateNode;break;case 22:n=e.stateNode._retryCache;break;default:throw Error(u(314))}n!==null&&n.delete(t),im(e,a)}function Kp(e,t){return ta(e,t)}var vu=null,tr=null,cf=!1,gu=!1,uf=!1,fs=0;function Go(e){e!==tr&&e.next===null&&(tr===null?vu=tr=e:tr=tr.next=e),gu=!0,cf||(cf=!0,Wp())}function nc(e,t){if(!uf&&gu){uf=!0;do for(var a=!1,n=vu;n!==null;){if(e!==0){var s=n.pendingLanes;if(s===0)var c=0;else{var d=n.suspendedLanes,w=n.pingedLanes;c=(1<<31-Vt(42|e)+1)-1,c&=s&~(d&~w),c=c&201326741?c&201326741|1:c?c|2:0}c!==0&&(a=!0,fm(n,c))}else c=bt,c=Ll(n,n===Ft?c:0,n.cancelPendingCommit!==null||n.timeoutHandle!==-1),(c&3)===0||So(n,c)||(a=!0,fm(n,c));n=n.next}while(a);uf=!1}}function Yp(){rm()}function rm(){gu=cf=!1;var e=0;fs!==0&&av()&&(e=fs);for(var t=qt(),a=null,n=vu;n!==null;){var s=n.next,c=cm(n,t);c===0?(n.next=null,a===null?vu=s:a.next=s,s===null&&(tr=a)):(a=n,(e!==0||(c&3)!==0)&&(gu=!0)),n=s}ga!==0&&ga!==5||nc(e),fs!==0&&(fs=0)}function cm(e,t){for(var a=e.suspendedLanes,n=e.pingedLanes,s=e.expirationTimes,c=e.pendingLanes&-62914561;0<c;){var d=31-Vt(c),w=1<<d,L=s[d];L===-1?((w&a)===0||(w&n)!==0)&&(s[d]=Sc(w,t)):L<=t&&(e.expiredLanes|=w),c&=~w}if(t=Ft,a=bt,a=Ll(e,e===t?a:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),n=e.callbackNode,a===0||e===t&&(Bt===2||Bt===9)||e.cancelPendingCommit!==null)return n!==null&&n!==null&&Ha(n),e.callbackNode=null,e.callbackPriority=0;if((a&3)===0||So(e,a)){if(t=a&-a,t===e.callbackPriority)return t;switch(n!==null&&Ha(n),_s(a)){case 2:case 8:a=ao;break;case 32:a=Pa;break;case 268435456:a=xa;break;default:a=Pa}return n=um.bind(null,e),a=ta(a,n),e.callbackPriority=t,e.callbackNode=a,t}return n!==null&&n!==null&&Ha(n),e.callbackPriority=2,e.callbackNode=null,2}function um(e,t){if(ga!==0&&ga!==5)return e.callbackNode=null,e.callbackPriority=0,null;var a=e.callbackNode;if(pu()&&e.callbackNode!==a)return null;var n=bt;return n=Ll(e,e===Ft?n:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),n===0?null:(Yd(e,n,t),cm(e,qt()),e.callbackNode!=null&&e.callbackNode===a?um.bind(null,e):null)}function fm(e,t){if(pu())return null;Yd(e,t,!0)}function Wp(){ov(function(){(Dt&6)!==0?ta(La,Yp):rm()})}function ff(){if(fs===0){var e=Ja;e===0&&(e=yn,yn<<=1,(yn&261888)===0&&(yn=256)),fs=e}return fs}function hm(e){return e==null||typeof e=="symbol"||typeof e=="boolean"?null:typeof e=="function"?e:lo(""+e)}function dm(e,t){var a=t.ownerDocument.createElement("input");return a.name=t.name,a.value=t.value,e.id&&a.setAttribute("form",e.id),t.parentNode.insertBefore(a,t),e=new FormData(e),a.parentNode.removeChild(a),e}function Xp(e,t,a,n,s){if(t==="submit"&&a&&a.stateNode===s){var c=hm((s[ba]||null).action),d=n.submitter;d&&(t=(t=d[ba]||null)?hm(t.formAction):d.getAttribute("formAction"),t!==null&&(c=t,d=null));var w=new Hn("action","action",null,n,s);e.push({event:w,listeners:[{instance:null,listener:function(){if(n.defaultPrevented){if(fs!==0){var L=d?dm(s,d):new FormData(s);R0(a,{pending:!0,data:L,method:s.method,action:c},null,L)}}else typeof c=="function"&&(w.preventDefault(),L=d?dm(s,d):new FormData(s),R0(a,{pending:!0,data:L,method:s.method,action:c},c,L))},currentTarget:s}]})}}for(var hf=0;hf<ko.length;hf++){var df=ko[hf],Pp=df.toLowerCase(),Zp=df[0].toUpperCase()+df.slice(1);un(Pp,"on"+Zp)}un(qc,"onAnimationEnd"),un(ki,"onAnimationIteration"),un(Ps,"onAnimationStart"),un("dblclick","onDoubleClick"),un("focusin","onFocus"),un("focusout","onBlur"),un(Gc,"onTransitionRun"),un(Fc,"onTransitionStart"),un(Fr,"onTransitionCancel"),un(Bi,"onTransitionEnd"),bn("onMouseEnter",["mouseout","mouseover"]),bn("onMouseLeave",["mouseout","mouseover"]),bn("onPointerEnter",["pointerout","pointerover"]),bn("onPointerLeave",["pointerout","pointerover"]),wn("onChange","change click focusin focusout input keydown keyup selectionchange".split(" ")),wn("onSelect","focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange".split(" ")),wn("onBeforeInput",["compositionend","keypress","textInput","paste"]),wn("onCompositionEnd","compositionend focusout keydown keypress keyup mousedown".split(" ")),wn("onCompositionStart","compositionstart focusout keydown keypress keyup mousedown".split(" ")),wn("onCompositionUpdate","compositionupdate focusout keydown keypress keyup mousedown".split(" "));var oc="abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting".split(" "),Qp=new Set("beforetoggle cancel close invalid load scroll scrollend toggle".split(" ").concat(oc));function mm(e,t){t=(t&4)!==0;for(var a=0;a<e.length;a++){var n=e[a],s=n.event;n=n.listeners;e:{var c=void 0;if(t)for(var d=n.length-1;0<=d;d--){var w=n[d],L=w.instance,re=w.currentTarget;if(w=w.listener,L!==c&&s.isPropagationStopped())break e;c=w,s.currentTarget=re;try{c(s)}catch(be){Xl(be)}s.currentTarget=null,c=L}else for(d=0;d<n.length;d++){if(w=n[d],L=w.instance,re=w.currentTarget,w=w.listener,L!==c&&s.isPropagationStopped())break e;c=w,s.currentTarget=re;try{c(s)}catch(be){Xl(be)}s.currentTarget=null,c=L}}}}function yt(e,t){var a=t[As];a===void 0&&(a=t[As]=new Set);var n=e+"__bubble";a.has(n)||(pm(t,e,2,!1),a.add(n))}function mf(e,t,a){var n=0;t&&(n|=4),pm(a,e,n,t)}var yu="_reactListening"+Math.random().toString(36).slice(2);function pf(e){if(!e[yu]){e[yu]=!0,pr.forEach(function(a){a!=="selectionchange"&&(Qp.has(a)||mf(a,!1,e),mf(a,!0,e))});var t=e.nodeType===9?e:e.ownerDocument;t===null||t[yu]||(t[yu]=!0,mf("selectionchange",!1,t))}}function pm(e,t,a,n){switch(Km(t)){case 2:var s=Tv;break;case 8:s=zv;break;default:s=Rf}a=s.bind(null,t,a,e),s=void 0,!Vl||t!=="touchstart"&&t!=="touchmove"&&t!=="wheel"||(s=!0),n?s!==void 0?e.addEventListener(t,a,{capture:!0,passive:s}):e.addEventListener(t,a,!0):s!==void 0?e.addEventListener(t,a,{passive:s}):e.addEventListener(t,a,!1)}function vf(e,t,a,n,s){var c=n;if((t&1)===0&&(t&2)===0&&n!==null)e:for(;;){if(n===null)return;var d=n.tag;if(d===3||d===4){var w=n.stateNode.containerInfo;if(w===s)break;if(d===4)for(d=n.return;d!==null;){var L=d.tag;if((L===3||L===4)&&d.stateNode.containerInfo===s)return;d=d.return}for(;w!==null;){if(d=no(w),d===null)return;if(L=d.tag,L===5||L===6||L===26||L===27){n=c=d;continue e}w=w.parentNode}}n=n.return}io(function(){var re=c,be=zi(a),Ce=[];e:{var he=Zs.get(e);if(he!==void 0){var ve=Hn,Fe=e;switch(e){case"keypress":if(Os(a)===0)break e;case"keydown":case"keyup":ve=zr;break;case"focusin":Fe="focus",ve=br;break;case"focusout":Fe="blur",ve=br;break;case"beforeblur":case"afterblur":ve=br;break;case"click":if(a.button===2)break e;case"auxclick":case"dblclick":case"mousedown":case"mousemove":case"mouseup":case"mouseout":case"mouseover":case"contextmenu":ve=Bs;break;case"drag":case"dragend":case"dragenter":case"dragexit":case"dragleave":case"dragover":case"dragstart":case"drop":ve=Dc;break;case"touchcancel":case"touchend":case"touchmove":case"touchstart":ve=js;break;case qc:case ki:case Ps:ve=c0;break;case Bi:ve=Ei;break;case"scroll":case"scrollend":ve=Nc;break;case"wheel":ve=_r;break;case"copy":case"cut":case"paste":ve=Hs;break;case"gotpointercapture":case"lostpointercapture":case"pointercancel":case"pointerdown":case"pointermove":case"pointerout":case"pointerover":case"pointerup":ve=kc;break;case"toggle":case"beforetoggle":ve=Bc}var Je=(t&4)!==0,jt=!Je&&(e==="scroll"||e==="scrollend"),$=Je?he!==null?he+"Capture":null:he;Je=[];for(var W=re,ie;W!==null;){var Me=W;if(ie=Me.stateNode,Me=Me.tag,Me!==5&&Me!==26&&Me!==27||ie===null||$===null||(Me=Ds(W,$),Me!=null&&Je.push(lc(W,Me,ie))),jt)break;W=W.return}0<Je.length&&(he=new ve(he,Fe,null,a,be),Ce.push({event:he,listeners:Je}))}}if((t&7)===0){e:{if(he=e==="mouseover"||e==="pointerover",ve=e==="mouseout"||e==="pointerout",he&&a!==Ti&&(Fe=a.relatedTarget||a.fromElement)&&(no(Fe)||Fe[To]))break e;if((ve||he)&&(he=be.window===be?be:(he=be.ownerDocument)?he.defaultView||he.parentWindow:window,ve?(Fe=a.relatedTarget||a.toElement,ve=re,Fe=Fe?no(Fe):null,Fe!==null&&(jt=f(Fe),Je=Fe.tag,Fe!==jt||Je!==5&&Je!==27&&Je!==6)&&(Fe=null)):(ve=null,Fe=re),ve!==Fe)){if(Je=Bs,Me="onMouseLeave",$="onMouseEnter",W="mouse",(e==="pointerout"||e==="pointerover")&&(Je=kc,Me="onPointerLeave",$="onPointerEnter",W="pointer"),jt=ve==null?he:tl(ve),ie=Fe==null?he:tl(Fe),he=new Je(Me,W+"leave",ve,a,be),he.target=jt,he.relatedTarget=ie,Me=null,no(be)===re&&(Je=new Je($,W+"enter",Fe,a,be),Je.target=ie,Je.relatedTarget=jt,Me=Je),jt=Me,ve&&Fe)t:{for(Je=Ip,$=ve,W=Fe,ie=0,Me=$;Me;Me=Je(Me))ie++;Me=0;for(var Pe=W;Pe;Pe=Je(Pe))Me++;for(;0<ie-Me;)$=Je($),ie--;for(;0<Me-ie;)W=Je(W),Me--;for(;ie--;){if($===W||W!==null&&$===W.alternate){Je=$;break t}$=Je($),W=Je(W)}Je=null}else Je=null;ve!==null&&vm(Ce,he,ve,Je,!1),Fe!==null&&jt!==null&&vm(Ce,jt,Fe,Je,!0)}}e:{if(he=re?tl(re):window,ve=he.nodeName&&he.nodeName.toLowerCase(),ve==="select"||ve==="input"&&he.type==="file")var _t=Dr;else if(Ln(he))if(Di)_t=Ys;else{_t=No;var We=kr}else ve=he.nodeName,!ve||ve.toLowerCase()!=="input"||he.type!=="checkbox"&&he.type!=="radio"?re&&Ns(re.elementType)&&(_t=Dr):_t=Br;if(_t&&(_t=_t(e,re))){Yl(Ce,_t,a,be);break e}We&&We(e,he,re),e==="focusout"&&re&&he.type==="number"&&re.memoizedProps.value!=null&&Fl(he,"number",he.value)}switch(We=re?tl(re):window,e){case"focusin":(Ln(We)||We.contentEditable==="true")&&(sl=We,Hr=re,uo=null);break;case"focusout":uo=Hr=sl=null;break;case"mousedown":Lr=!0;break;case"contextmenu":case"mouseup":case"dragend":Lr=!1,jr(Ce,a,be);break;case"selectionchange":if(jc)break;case"keydown":case"keyup":jr(Ce,a,be)}var ht;if(qs)e:{switch(e){case"compositionstart":var Mt="onCompositionStart";break e;case"compositionend":Mt="onCompositionEnd";break e;case"compositionupdate":Mt="onCompositionUpdate";break e}Mt=void 0}else Ro?Fs(e,a)&&(Mt="onCompositionEnd"):e==="keydown"&&a.keyCode===229&&(Mt="onCompositionStart");Mt&&(Ri&&a.locale!=="ko"&&(Ro||Mt!=="onCompositionStart"?Mt==="onCompositionEnd"&&Ro&&(ht=Rc()):(Ao=be,xr="value"in Ao?Ao.value:Ao.textContent,Ro=!0)),We=xu(re,Mt),0<We.length&&(Mt=new Mr(Mt,e,null,a,be),Ce.push({event:Mt,listeners:We}),ht?Mt.data=ht:(ht=Er(a),ht!==null&&(Mt.data=ht)))),(ht=Ar?ro(e,a):ja(e,a))&&(Mt=xu(re,"onBeforeInput"),0<Mt.length&&(We=new Mr("onBeforeInput","beforeinput",null,a,be),Ce.push({event:We,listeners:Mt}),We.data=ht)),Xp(Ce,e,re,a,be)}mm(Ce,t)})}function lc(e,t,a){return{instance:e,listener:t,currentTarget:a}}function xu(e,t){for(var a=t+"Capture",n=[];e!==null;){var s=e,c=s.stateNode;if(s=s.tag,s!==5&&s!==26&&s!==27||c===null||(s=Ds(e,a),s!=null&&n.unshift(lc(e,s,c)),s=Ds(e,t),s!=null&&n.push(lc(e,s,c))),e.tag===3)return n;e=e.return}return[]}function Ip(e){if(e===null)return null;do e=e.return;while(e&&e.tag!==5&&e.tag!==27);return e||null}function vm(e,t,a,n,s){for(var c=t._reactName,d=[];a!==null&&a!==n;){var w=a,L=w.alternate,re=w.stateNode;if(w=w.tag,L!==null&&L===n)break;w!==5&&w!==26&&w!==27||re===null||(L=re,s?(re=Ds(a,c),re!=null&&d.unshift(lc(a,re,L))):s||(re=Ds(a,c),re!=null&&d.push(lc(a,re,L)))),a=a.return}d.length!==0&&e.push({event:t,listeners:d})}var Jp=/\r\n?/g,$p=/\u0000|\uFFFD/g;function gm(e){return(typeof e=="string"?e:""+e).replace(Jp,`
`).replace($p,"")}function ym(e,t){return t=gm(t),gm(e)===t}function Lt(e,t,a,n,s,c){switch(a){case"children":typeof n=="string"?t==="body"||t==="textarea"&&n===""||Ie(e,n):(typeof n=="number"||typeof n=="bigint")&&t!=="body"&&Ie(e,""+n);break;case"className":Za(e,"class",n);break;case"tabIndex":Za(e,"tabindex",n);break;case"dir":case"role":case"viewBox":case"width":case"height":Za(e,a,n);break;case"style":gr(e,n,c);break;case"data":if(t!=="object"){Za(e,"data",n);break}case"src":case"href":if(n===""&&(t!=="a"||a!=="href")){e.removeAttribute(a);break}if(n==null||typeof n=="function"||typeof n=="symbol"||typeof n=="boolean"){e.removeAttribute(a);break}n=lo(""+n),e.setAttribute(a,n);break;case"action":case"formAction":if(typeof n=="function"){e.setAttribute(a,"javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')");break}else typeof c=="function"&&(a==="formAction"?(t!=="input"&&Lt(e,t,"name",s.name,s,null),Lt(e,t,"formEncType",s.formEncType,s,null),Lt(e,t,"formMethod",s.formMethod,s,null),Lt(e,t,"formTarget",s.formTarget,s,null)):(Lt(e,t,"encType",s.encType,s,null),Lt(e,t,"method",s.method,s,null),Lt(e,t,"target",s.target,s,null)));if(n==null||typeof n=="symbol"||typeof n=="boolean"){e.removeAttribute(a);break}n=lo(""+n),e.setAttribute(a,n);break;case"onClick":n!=null&&(e.onclick=cn);break;case"onScroll":n!=null&&yt("scroll",e);break;case"onScrollEnd":n!=null&&yt("scrollend",e);break;case"dangerouslySetInnerHTML":if(n!=null){if(typeof n!="object"||!("__html"in n))throw Error(u(61));if(a=n.__html,a!=null){if(s.children!=null)throw Error(u(60));e.innerHTML=a}}break;case"multiple":e.multiple=n&&typeof n!="function"&&typeof n!="symbol";break;case"muted":e.muted=n&&typeof n!="function"&&typeof n!="symbol";break;case"suppressContentEditableWarning":case"suppressHydrationWarning":case"defaultValue":case"defaultChecked":case"innerHTML":case"ref":break;case"autoFocus":break;case"xlinkHref":if(n==null||typeof n=="function"||typeof n=="boolean"||typeof n=="symbol"){e.removeAttribute("xlink:href");break}a=lo(""+n),e.setAttributeNS("http://www.w3.org/1999/xlink","xlink:href",a);break;case"contentEditable":case"spellCheck":case"draggable":case"value":case"autoReverse":case"externalResourcesRequired":case"focusable":case"preserveAlpha":n!=null&&typeof n!="function"&&typeof n!="symbol"?e.setAttribute(a,""+n):e.removeAttribute(a);break;case"inert":case"allowFullScreen":case"async":case"autoPlay":case"controls":case"default":case"defer":case"disabled":case"disablePictureInPicture":case"disableRemotePlayback":case"formNoValidate":case"hidden":case"loop":case"noModule":case"noValidate":case"open":case"playsInline":case"readOnly":case"required":case"reversed":case"scoped":case"seamless":case"itemScope":n&&typeof n!="function"&&typeof n!="symbol"?e.setAttribute(a,""):e.removeAttribute(a);break;case"capture":case"download":n===!0?e.setAttribute(a,""):n!==!1&&n!=null&&typeof n!="function"&&typeof n!="symbol"?e.setAttribute(a,n):e.removeAttribute(a);break;case"cols":case"rows":case"size":case"span":n!=null&&typeof n!="function"&&typeof n!="symbol"&&!isNaN(n)&&1<=n?e.setAttribute(a,n):e.removeAttribute(a);break;case"rowSpan":case"start":n==null||typeof n=="function"||typeof n=="symbol"||isNaN(n)?e.removeAttribute(a):e.setAttribute(a,n);break;case"popover":yt("beforetoggle",e),yt("toggle",e),dt(e,"popover",n);break;case"xlinkActuate":Qa(e,"http://www.w3.org/1999/xlink","xlink:actuate",n);break;case"xlinkArcrole":Qa(e,"http://www.w3.org/1999/xlink","xlink:arcrole",n);break;case"xlinkRole":Qa(e,"http://www.w3.org/1999/xlink","xlink:role",n);break;case"xlinkShow":Qa(e,"http://www.w3.org/1999/xlink","xlink:show",n);break;case"xlinkTitle":Qa(e,"http://www.w3.org/1999/xlink","xlink:title",n);break;case"xlinkType":Qa(e,"http://www.w3.org/1999/xlink","xlink:type",n);break;case"xmlBase":Qa(e,"http://www.w3.org/XML/1998/namespace","xml:base",n);break;case"xmlLang":Qa(e,"http://www.w3.org/XML/1998/namespace","xml:lang",n);break;case"xmlSpace":Qa(e,"http://www.w3.org/XML/1998/namespace","xml:space",n);break;case"is":dt(e,"is",n);break;case"innerText":case"textContent":break;default:(!(2<a.length)||a[0]!=="o"&&a[0]!=="O"||a[1]!=="n"&&a[1]!=="N")&&(a=i0.get(a)||a,dt(e,a,n))}}function gf(e,t,a,n,s,c){switch(a){case"style":gr(e,n,c);break;case"dangerouslySetInnerHTML":if(n!=null){if(typeof n!="object"||!("__html"in n))throw Error(u(61));if(a=n.__html,a!=null){if(s.children!=null)throw Error(u(60));e.innerHTML=a}}break;case"children":typeof n=="string"?Ie(e,n):(typeof n=="number"||typeof n=="bigint")&&Ie(e,""+n);break;case"onScroll":n!=null&&yt("scroll",e);break;case"onScrollEnd":n!=null&&yt("scrollend",e);break;case"onClick":n!=null&&(e.onclick=cn);break;case"suppressContentEditableWarning":case"suppressHydrationWarning":case"innerHTML":case"ref":break;case"innerText":case"textContent":break;default:if(!rn.hasOwnProperty(a))e:{if(a[0]==="o"&&a[1]==="n"&&(s=a.endsWith("Capture"),t=a.slice(2,s?a.length-7:void 0),c=e[ba]||null,c=c!=null?c[a]:null,typeof c=="function"&&e.removeEventListener(t,c,s),typeof n=="function")){typeof c!="function"&&c!==null&&(a in e?e[a]=null:e.hasAttribute(a)&&e.removeAttribute(a)),e.addEventListener(t,n,s);break e}a in e?e[a]=n:n===!0?e.setAttribute(a,""):dt(e,a,n)}}}function Oa(e,t,a){switch(t){case"div":case"span":case"svg":case"path":case"a":case"g":case"p":case"li":break;case"img":yt("error",e),yt("load",e);var n=!1,s=!1,c;for(c in a)if(a.hasOwnProperty(c)){var d=a[c];if(d!=null)switch(c){case"src":n=!0;break;case"srcSet":s=!0;break;case"children":case"dangerouslySetInnerHTML":throw Error(u(137,t));default:Lt(e,t,c,d,a,null)}}s&&Lt(e,t,"srcSet",a.srcSet,a,null),n&&Lt(e,t,"src",a.src,a,null);return;case"input":yt("invalid",e);var w=c=d=s=null,L=null,re=null;for(n in a)if(a.hasOwnProperty(n)){var be=a[n];if(be!=null)switch(n){case"name":s=be;break;case"type":d=be;break;case"checked":L=be;break;case"defaultChecked":re=be;break;case"value":c=be;break;case"defaultValue":w=be;break;case"children":case"dangerouslySetInnerHTML":if(be!=null)throw Error(u(137,t));break;default:Lt(e,t,n,be,a,null)}}_c(e,c,w,L,re,d,s,!1);return;case"select":yt("invalid",e),n=d=c=null;for(s in a)if(a.hasOwnProperty(s)&&(w=a[s],w!=null))switch(s){case"value":c=w;break;case"defaultValue":d=w;break;case"multiple":n=w;default:Lt(e,t,s,w,a,null)}t=c,a=d,e.multiple=!!n,t!=null?oo(e,!!n,t,!1):a!=null&&oo(e,!!n,a,!0);return;case"textarea":yt("invalid",e),c=s=n=null;for(d in a)if(a.hasOwnProperty(d)&&(w=a[d],w!=null))switch(d){case"value":n=w;break;case"defaultValue":s=w;break;case"children":c=w;break;case"dangerouslySetInnerHTML":if(w!=null)throw Error(u(91));break;default:Lt(e,t,d,w,a,null)}at(e,n,s,c);return;case"option":for(L in a)if(a.hasOwnProperty(L)&&(n=a[L],n!=null))switch(L){case"selected":e.selected=n&&typeof n!="function"&&typeof n!="symbol";break;default:Lt(e,t,L,n,a,null)}return;case"dialog":yt("beforetoggle",e),yt("toggle",e),yt("cancel",e),yt("close",e);break;case"iframe":case"object":yt("load",e);break;case"video":case"audio":for(n=0;n<oc.length;n++)yt(oc[n],e);break;case"image":yt("error",e),yt("load",e);break;case"details":yt("toggle",e);break;case"embed":case"source":case"link":yt("error",e),yt("load",e);case"area":case"base":case"br":case"col":case"hr":case"keygen":case"meta":case"param":case"track":case"wbr":case"menuitem":for(re in a)if(a.hasOwnProperty(re)&&(n=a[re],n!=null))switch(re){case"children":case"dangerouslySetInnerHTML":throw Error(u(137,t));default:Lt(e,t,re,n,a,null)}return;default:if(Ns(t)){for(be in a)a.hasOwnProperty(be)&&(n=a[be],n!==void 0&&gf(e,t,be,n,a,void 0));return}}for(w in a)a.hasOwnProperty(w)&&(n=a[w],n!=null&&Lt(e,t,w,n,a,null))}function ev(e,t,a,n){switch(t){case"div":case"span":case"svg":case"path":case"a":case"g":case"p":case"li":break;case"input":var s=null,c=null,d=null,w=null,L=null,re=null,be=null;for(ve in a){var Ce=a[ve];if(a.hasOwnProperty(ve)&&Ce!=null)switch(ve){case"checked":break;case"value":break;case"defaultValue":L=Ce;default:n.hasOwnProperty(ve)||Lt(e,t,ve,null,n,Ce)}}for(var he in n){var ve=n[he];if(Ce=a[he],n.hasOwnProperty(he)&&(ve!=null||Ce!=null))switch(he){case"type":c=ve;break;case"name":s=ve;break;case"checked":re=ve;break;case"defaultChecked":be=ve;break;case"value":d=ve;break;case"defaultValue":w=ve;break;case"children":case"dangerouslySetInnerHTML":if(ve!=null)throw Error(u(137,t));break;default:ve!==Ce&&Lt(e,t,he,ve,n,Ce)}}Mi(e,d,w,L,re,be,c,s);return;case"select":ve=d=w=he=null;for(c in a)if(L=a[c],a.hasOwnProperty(c)&&L!=null)switch(c){case"value":break;case"multiple":ve=L;default:n.hasOwnProperty(c)||Lt(e,t,c,null,n,L)}for(s in n)if(c=n[s],L=a[s],n.hasOwnProperty(s)&&(c!=null||L!=null))switch(s){case"value":he=c;break;case"defaultValue":w=c;break;case"multiple":d=c;default:c!==L&&Lt(e,t,s,c,n,L)}t=w,a=d,n=ve,he!=null?oo(e,!!a,he,!1):!!n!=!!a&&(t!=null?oo(e,!!a,t,!0):oo(e,!!a,a?[]:"",!1));return;case"textarea":ve=he=null;for(w in a)if(s=a[w],a.hasOwnProperty(w)&&s!=null&&!n.hasOwnProperty(w))switch(w){case"value":break;case"children":break;default:Lt(e,t,w,null,n,s)}for(d in n)if(s=n[d],c=a[d],n.hasOwnProperty(d)&&(s!=null||c!=null))switch(d){case"value":he=s;break;case"defaultValue":ve=s;break;case"children":break;case"dangerouslySetInnerHTML":if(s!=null)throw Error(u(91));break;default:s!==c&&Lt(e,t,d,s,n,c)}Ac(e,he,ve);return;case"option":for(var Fe in a)if(he=a[Fe],a.hasOwnProperty(Fe)&&he!=null&&!n.hasOwnProperty(Fe))switch(Fe){case"selected":e.selected=!1;break;default:Lt(e,t,Fe,null,n,he)}for(L in n)if(he=n[L],ve=a[L],n.hasOwnProperty(L)&&he!==ve&&(he!=null||ve!=null))switch(L){case"selected":e.selected=he&&typeof he!="function"&&typeof he!="symbol";break;default:Lt(e,t,L,he,n,ve)}return;case"img":case"link":case"area":case"base":case"br":case"col":case"embed":case"hr":case"keygen":case"meta":case"param":case"source":case"track":case"wbr":case"menuitem":for(var Je in a)he=a[Je],a.hasOwnProperty(Je)&&he!=null&&!n.hasOwnProperty(Je)&&Lt(e,t,Je,null,n,he);for(re in n)if(he=n[re],ve=a[re],n.hasOwnProperty(re)&&he!==ve&&(he!=null||ve!=null))switch(re){case"children":case"dangerouslySetInnerHTML":if(he!=null)throw Error(u(137,t));break;default:Lt(e,t,re,he,n,ve)}return;default:if(Ns(t)){for(var jt in a)he=a[jt],a.hasOwnProperty(jt)&&he!==void 0&&!n.hasOwnProperty(jt)&&gf(e,t,jt,void 0,n,he);for(be in n)he=n[be],ve=a[be],!n.hasOwnProperty(be)||he===ve||he===void 0&&ve===void 0||gf(e,t,be,he,n,ve);return}}for(var $ in a)he=a[$],a.hasOwnProperty($)&&he!=null&&!n.hasOwnProperty($)&&Lt(e,t,$,null,n,he);for(Ce in n)he=n[Ce],ve=a[Ce],!n.hasOwnProperty(Ce)||he===ve||he==null&&ve==null||Lt(e,t,Ce,he,n,ve)}function xm(e){switch(e){case"css":case"script":case"font":case"img":case"image":case"input":case"link":return!0;default:return!1}}function tv(){if(typeof performance.getEntriesByType=="function"){for(var e=0,t=0,a=performance.getEntriesByType("resource"),n=0;n<a.length;n++){var s=a[n],c=s.transferSize,d=s.initiatorType,w=s.duration;if(c&&w&&xm(d)){for(d=0,w=s.responseEnd,n+=1;n<a.length;n++){var L=a[n],re=L.startTime;if(re>w)break;var be=L.transferSize,Ce=L.initiatorType;be&&xm(Ce)&&(L=L.responseEnd,d+=be*(L<w?1:(w-re)/(L-re)))}if(--n,t+=8*(c+d)/(s.duration/1e3),e++,10<e)break}}if(0<e)return t/e/1e6}return navigator.connection&&(e=navigator.connection.downlink,typeof e=="number")?e:5}var yf=null,xf=null;function wu(e){return e.nodeType===9?e:e.ownerDocument}function wm(e){switch(e){case"http://www.w3.org/2000/svg":return 1;case"http://www.w3.org/1998/Math/MathML":return 2;default:return 0}}function bm(e,t){if(e===0)switch(t){case"svg":return 1;case"math":return 2;default:return 0}return e===1&&t==="foreignObject"?0:e}function wf(e,t){return e==="textarea"||e==="noscript"||typeof t.children=="string"||typeof t.children=="number"||typeof t.children=="bigint"||typeof t.dangerouslySetInnerHTML=="object"&&t.dangerouslySetInnerHTML!==null&&t.dangerouslySetInnerHTML.__html!=null}var bf=null;function av(){var e=window.event;return e&&e.type==="popstate"?e===bf?!1:(bf=e,!0):(bf=null,!1)}var Mm=typeof setTimeout=="function"?setTimeout:void 0,nv=typeof clearTimeout=="function"?clearTimeout:void 0,Sm=typeof Promise=="function"?Promise:void 0,ov=typeof queueMicrotask=="function"?queueMicrotask:typeof Sm<"u"?function(e){return Sm.resolve(null).then(e).catch(lv)}:Mm;function lv(e){setTimeout(function(){throw e})}function hs(e){return e==="head"}function Tm(e,t){var a=t,n=0;do{var s=a.nextSibling;if(e.removeChild(a),s&&s.nodeType===8)if(a=s.data,a==="/$"||a==="/&"){if(n===0){e.removeChild(s),lr(t);return}n--}else if(a==="$"||a==="$?"||a==="$~"||a==="$!"||a==="&")n++;else if(a==="html")sc(e.ownerDocument.documentElement);else if(a==="head"){a=e.ownerDocument.head,sc(a);for(var c=a.firstChild;c;){var d=c.nextSibling,w=c.nodeName;c[el]||w==="SCRIPT"||w==="STYLE"||w==="LINK"&&c.rel.toLowerCase()==="stylesheet"||a.removeChild(c),c=d}}else a==="body"&&sc(e.ownerDocument.body);a=s}while(a);lr(t)}function zm(e,t){var a=e;e=0;do{var n=a.nextSibling;if(a.nodeType===1?t?(a._stashedDisplay=a.style.display,a.style.display="none"):(a.style.display=a._stashedDisplay||"",a.getAttribute("style")===""&&a.removeAttribute("style")):a.nodeType===3&&(t?(a._stashedText=a.nodeValue,a.nodeValue=""):a.nodeValue=a._stashedText||""),n&&n.nodeType===8)if(a=n.data,a==="/$"){if(e===0)break;e--}else a!=="$"&&a!=="$?"&&a!=="$~"&&a!=="$!"||e++;a=n}while(a)}function Mf(e){var t=e.firstChild;for(t&&t.nodeType===10&&(t=t.nextSibling);t;){var a=t;switch(t=t.nextSibling,a.nodeName){case"HTML":case"HEAD":case"BODY":Mf(a),Es(a);continue;case"SCRIPT":case"STYLE":continue;case"LINK":if(a.rel.toLowerCase()==="stylesheet")continue}e.removeChild(a)}}function sv(e,t,a,n){for(;e.nodeType===1;){var s=a;if(e.nodeName.toLowerCase()!==t.toLowerCase()){if(!n&&(e.nodeName!=="INPUT"||e.type!=="hidden"))break}else if(n){if(!e[el])switch(t){case"meta":if(!e.hasAttribute("itemprop"))break;return e;case"link":if(c=e.getAttribute("rel"),c==="stylesheet"&&e.hasAttribute("data-precedence"))break;if(c!==s.rel||e.getAttribute("href")!==(s.href==null||s.href===""?null:s.href)||e.getAttribute("crossorigin")!==(s.crossOrigin==null?null:s.crossOrigin)||e.getAttribute("title")!==(s.title==null?null:s.title))break;return e;case"style":if(e.hasAttribute("data-precedence"))break;return e;case"script":if(c=e.getAttribute("src"),(c!==(s.src==null?null:s.src)||e.getAttribute("type")!==(s.type==null?null:s.type)||e.getAttribute("crossorigin")!==(s.crossOrigin==null?null:s.crossOrigin))&&c&&e.hasAttribute("async")&&!e.hasAttribute("itemprop"))break;return e;default:return e}}else if(t==="input"&&e.type==="hidden"){var c=s.name==null?null:""+s.name;if(s.type==="hidden"&&e.getAttribute("name")===c)return e}else return e;if(e=Qn(e.nextSibling),e===null)break}return null}function iv(e,t,a){if(t==="")return null;for(;e.nodeType!==3;)if((e.nodeType!==1||e.nodeName!=="INPUT"||e.type!=="hidden")&&!a||(e=Qn(e.nextSibling),e===null))return null;return e}function Cm(e,t){for(;e.nodeType!==8;)if((e.nodeType!==1||e.nodeName!=="INPUT"||e.type!=="hidden")&&!t||(e=Qn(e.nextSibling),e===null))return null;return e}function Sf(e){return e.data==="$?"||e.data==="$~"}function Tf(e){return e.data==="$!"||e.data==="$?"&&e.ownerDocument.readyState!=="loading"}function rv(e,t){var a=e.ownerDocument;if(e.data==="$~")e._reactRetry=t;else if(e.data!=="$?"||a.readyState!=="loading")t();else{var n=function(){t(),a.removeEventListener("DOMContentLoaded",n)};a.addEventListener("DOMContentLoaded",n),e._reactRetry=n}}function Qn(e){for(;e!=null;e=e.nextSibling){var t=e.nodeType;if(t===1||t===3)break;if(t===8){if(t=e.data,t==="$"||t==="$!"||t==="$?"||t==="$~"||t==="&"||t==="F!"||t==="F")break;if(t==="/$"||t==="/&")return null}}return e}var zf=null;function _m(e){e=e.nextSibling;for(var t=0;e;){if(e.nodeType===8){var a=e.data;if(a==="/$"||a==="/&"){if(t===0)return Qn(e.nextSibling);t--}else a!=="$"&&a!=="$!"&&a!=="$?"&&a!=="$~"&&a!=="&"||t++}e=e.nextSibling}return null}function Am(e){e=e.previousSibling;for(var t=0;e;){if(e.nodeType===8){var a=e.data;if(a==="$"||a==="$!"||a==="$?"||a==="$~"||a==="&"){if(t===0)return e;t--}else a!=="/$"&&a!=="/&"||t++}e=e.previousSibling}return null}function Em(e,t,a){switch(t=wu(a),e){case"html":if(e=t.documentElement,!e)throw Error(u(452));return e;case"head":if(e=t.head,!e)throw Error(u(453));return e;case"body":if(e=t.body,!e)throw Error(u(454));return e;default:throw Error(u(451))}}function sc(e){for(var t=e.attributes;t.length;)e.removeAttributeNode(t[0]);Es(e)}var In=new Map,Rm=new Set;function bu(e){return typeof e.getRootNode=="function"?e.getRootNode():e.nodeType===9?e:e.ownerDocument}var Cl=j.d;j.d={f:cv,r:uv,D:fv,C:hv,L:dv,m:mv,X:vv,S:pv,M:gv};function cv(){var e=Cl.f(),t=hu();return e||t}function uv(e){var t=zo(e);t!==null&&t.tag===5&&t.type==="form"?Xh(t):Cl.r(e)}var ar=typeof document>"u"?null:document;function Nm(e,t,a){var n=ar;if(n&&typeof t=="string"&&t){var s=Aa(t);s='link[rel="'+e+'"][href="'+s+'"]',typeof a=="string"&&(s+='[crossorigin="'+a+'"]'),Rm.has(s)||(Rm.add(s),e={rel:e,crossOrigin:a,href:t},n.querySelector(s)===null&&(t=n.createElement("link"),Oa(t,"link",e),aa(t),n.head.appendChild(t)))}}function fv(e){Cl.D(e),Nm("dns-prefetch",e,null)}function hv(e,t){Cl.C(e,t),Nm("preconnect",e,t)}function dv(e,t,a){Cl.L(e,t,a);var n=ar;if(n&&e&&t){var s='link[rel="preload"][as="'+Aa(t)+'"]';t==="image"&&a&&a.imageSrcSet?(s+='[imagesrcset="'+Aa(a.imageSrcSet)+'"]',typeof a.imageSizes=="string"&&(s+='[imagesizes="'+Aa(a.imageSizes)+'"]')):s+='[href="'+Aa(e)+'"]';var c=s;switch(t){case"style":c=nr(e);break;case"script":c=or(e)}In.has(c)||(e=S({rel:"preload",href:t==="image"&&a&&a.imageSrcSet?void 0:e,as:t},a),In.set(c,e),n.querySelector(s)!==null||t==="style"&&n.querySelector(ic(c))||t==="script"&&n.querySelector(rc(c))||(t=n.createElement("link"),Oa(t,"link",e),aa(t),n.head.appendChild(t)))}}function mv(e,t){Cl.m(e,t);var a=ar;if(a&&e){var n=t&&typeof t.as=="string"?t.as:"script",s='link[rel="modulepreload"][as="'+Aa(n)+'"][href="'+Aa(e)+'"]',c=s;switch(n){case"audioworklet":case"paintworklet":case"serviceworker":case"sharedworker":case"worker":case"script":c=or(e)}if(!In.has(c)&&(e=S({rel:"modulepreload",href:e},t),In.set(c,e),a.querySelector(s)===null)){switch(n){case"audioworklet":case"paintworklet":case"serviceworker":case"sharedworker":case"worker":case"script":if(a.querySelector(rc(c)))return}n=a.createElement("link"),Oa(n,"link",e),aa(n),a.head.appendChild(n)}}}function pv(e,t,a){Cl.S(e,t,a);var n=ar;if(n&&e){var s=Co(n).hoistableStyles,c=nr(e);t=t||"default";var d=s.get(c);if(!d){var w={loading:0,preload:null};if(d=n.querySelector(ic(c)))w.loading=5;else{e=S({rel:"stylesheet",href:e,"data-precedence":t},a),(a=In.get(c))&&Cf(e,a);var L=d=n.createElement("link");aa(L),Oa(L,"link",e),L._p=new Promise(function(re,be){L.onload=re,L.onerror=be}),L.addEventListener("load",function(){w.loading|=1}),L.addEventListener("error",function(){w.loading|=2}),w.loading|=4,Mu(d,t,n)}d={type:"stylesheet",instance:d,count:1,state:w},s.set(c,d)}}}function vv(e,t){Cl.X(e,t);var a=ar;if(a&&e){var n=Co(a).hoistableScripts,s=or(e),c=n.get(s);c||(c=a.querySelector(rc(s)),c||(e=S({src:e,async:!0},t),(t=In.get(s))&&_f(e,t),c=a.createElement("script"),aa(c),Oa(c,"link",e),a.head.appendChild(c)),c={type:"script",instance:c,count:1,state:null},n.set(s,c))}}function gv(e,t){Cl.M(e,t);var a=ar;if(a&&e){var n=Co(a).hoistableScripts,s=or(e),c=n.get(s);c||(c=a.querySelector(rc(s)),c||(e=S({src:e,async:!0,type:"module"},t),(t=In.get(s))&&_f(e,t),c=a.createElement("script"),aa(c),Oa(c,"link",e),a.head.appendChild(c)),c={type:"script",instance:c,count:1,state:null},n.set(s,c))}}function Dm(e,t,a,n){var s=(s=ue.current)?bu(s):null;if(!s)throw Error(u(446));switch(e){case"meta":case"title":return null;case"style":return typeof a.precedence=="string"&&typeof a.href=="string"?(t=nr(a.href),a=Co(s).hoistableStyles,n=a.get(t),n||(n={type:"style",instance:null,count:0,state:null},a.set(t,n)),n):{type:"void",instance:null,count:0,state:null};case"link":if(a.rel==="stylesheet"&&typeof a.href=="string"&&typeof a.precedence=="string"){e=nr(a.href);var c=Co(s).hoistableStyles,d=c.get(e);if(d||(s=s.ownerDocument||s,d={type:"stylesheet",instance:null,count:0,state:{loading:0,preload:null}},c.set(e,d),(c=s.querySelector(ic(e)))&&!c._p&&(d.instance=c,d.state.loading=5),In.has(e)||(a={rel:"preload",as:"style",href:a.href,crossOrigin:a.crossOrigin,integrity:a.integrity,media:a.media,hrefLang:a.hrefLang,referrerPolicy:a.referrerPolicy},In.set(e,a),c||yv(s,e,a,d.state))),t&&n===null)throw Error(u(528,""));return d}if(t&&n!==null)throw Error(u(529,""));return null;case"script":return t=a.async,a=a.src,typeof a=="string"&&t&&typeof t!="function"&&typeof t!="symbol"?(t=or(a),a=Co(s).hoistableScripts,n=a.get(t),n||(n={type:"script",instance:null,count:0,state:null},a.set(t,n)),n):{type:"void",instance:null,count:0,state:null};default:throw Error(u(444,e))}}function nr(e){return'href="'+Aa(e)+'"'}function ic(e){return'link[rel="stylesheet"]['+e+"]"}function Um(e){return S({},e,{"data-precedence":e.precedence,precedence:null})}function yv(e,t,a,n){e.querySelector('link[rel="preload"][as="style"]['+t+"]")?n.loading=1:(t=e.createElement("link"),n.preload=t,t.addEventListener("load",function(){return n.loading|=1}),t.addEventListener("error",function(){return n.loading|=2}),Oa(t,"link",a),aa(t),e.head.appendChild(t))}function or(e){return'[src="'+Aa(e)+'"]'}function rc(e){return"script[async]"+e}function Om(e,t,a){if(t.count++,t.instance===null)switch(t.type){case"style":var n=e.querySelector('style[data-href~="'+Aa(a.href)+'"]');if(n)return t.instance=n,aa(n),n;var s=S({},a,{"data-href":a.href,"data-precedence":a.precedence,href:null,precedence:null});return n=(e.ownerDocument||e).createElement("style"),aa(n),Oa(n,"style",s),Mu(n,a.precedence,e),t.instance=n;case"stylesheet":s=nr(a.href);var c=e.querySelector(ic(s));if(c)return t.state.loading|=4,t.instance=c,aa(c),c;n=Um(a),(s=In.get(s))&&Cf(n,s),c=(e.ownerDocument||e).createElement("link"),aa(c);var d=c;return d._p=new Promise(function(w,L){d.onload=w,d.onerror=L}),Oa(c,"link",n),t.state.loading|=4,Mu(c,a.precedence,e),t.instance=c;case"script":return c=or(a.src),(s=e.querySelector(rc(c)))?(t.instance=s,aa(s),s):(n=a,(s=In.get(c))&&(n=S({},a),_f(n,s)),e=e.ownerDocument||e,s=e.createElement("script"),aa(s),Oa(s,"link",n),e.head.appendChild(s),t.instance=s);case"void":return null;default:throw Error(u(443,t.type))}else t.type==="stylesheet"&&(t.state.loading&4)===0&&(n=t.instance,t.state.loading|=4,Mu(n,a.precedence,e));return t.instance}function Mu(e,t,a){for(var n=a.querySelectorAll('link[rel="stylesheet"][data-precedence],style[data-precedence]'),s=n.length?n[n.length-1]:null,c=s,d=0;d<n.length;d++){var w=n[d];if(w.dataset.precedence===t)c=w;else if(c!==s)break}c?c.parentNode.insertBefore(e,c.nextSibling):(t=a.nodeType===9?a.head:a,t.insertBefore(e,t.firstChild))}function Cf(e,t){e.crossOrigin==null&&(e.crossOrigin=t.crossOrigin),e.referrerPolicy==null&&(e.referrerPolicy=t.referrerPolicy),e.title==null&&(e.title=t.title)}function _f(e,t){e.crossOrigin==null&&(e.crossOrigin=t.crossOrigin),e.referrerPolicy==null&&(e.referrerPolicy=t.referrerPolicy),e.integrity==null&&(e.integrity=t.integrity)}var Su=null;function km(e,t,a){if(Su===null){var n=new Map,s=Su=new Map;s.set(a,n)}else s=Su,n=s.get(a),n||(n=new Map,s.set(a,n));if(n.has(e))return n;for(n.set(e,null),a=a.getElementsByTagName(e),s=0;s<a.length;s++){var c=a[s];if(!(c[el]||c[ra]||e==="link"&&c.getAttribute("rel")==="stylesheet")&&c.namespaceURI!=="http://www.w3.org/2000/svg"){var d=c.getAttribute(t)||"";d=e+d;var w=n.get(d);w?w.push(c):n.set(d,[c])}}return n}function Bm(e,t,a){e=e.ownerDocument||e,e.head.insertBefore(a,t==="title"?e.querySelector("head > title"):null)}function xv(e,t,a){if(a===1||t.itemProp!=null)return!1;switch(e){case"meta":case"title":return!0;case"style":if(typeof t.precedence!="string"||typeof t.href!="string"||t.href==="")break;return!0;case"link":if(typeof t.rel!="string"||typeof t.href!="string"||t.href===""||t.onLoad||t.onError)break;switch(t.rel){case"stylesheet":return e=t.disabled,typeof t.precedence=="string"&&e==null;default:return!0}case"script":if(t.async&&typeof t.async!="function"&&typeof t.async!="symbol"&&!t.onLoad&&!t.onError&&t.src&&typeof t.src=="string")return!0}return!1}function Hm(e){return!(e.type==="stylesheet"&&(e.state.loading&3)===0)}function wv(e,t,a,n){if(a.type==="stylesheet"&&(typeof n.media!="string"||matchMedia(n.media).matches!==!1)&&(a.state.loading&4)===0){if(a.instance===null){var s=nr(n.href),c=t.querySelector(ic(s));if(c){t=c._p,t!==null&&typeof t=="object"&&typeof t.then=="function"&&(e.count++,e=Tu.bind(e),t.then(e,e)),a.state.loading|=4,a.instance=c,aa(c);return}c=t.ownerDocument||t,n=Um(n),(s=In.get(s))&&Cf(n,s),c=c.createElement("link"),aa(c);var d=c;d._p=new Promise(function(w,L){d.onload=w,d.onerror=L}),Oa(c,"link",n),a.instance=c}e.stylesheets===null&&(e.stylesheets=new Map),e.stylesheets.set(a,t),(t=a.state.preload)&&(a.state.loading&3)===0&&(e.count++,a=Tu.bind(e),t.addEventListener("load",a),t.addEventListener("error",a))}}var Af=0;function bv(e,t){return e.stylesheets&&e.count===0&&Cu(e,e.stylesheets),0<e.count||0<e.imgCount?function(a){var n=setTimeout(function(){if(e.stylesheets&&Cu(e,e.stylesheets),e.unsuspend){var c=e.unsuspend;e.unsuspend=null,c()}},6e4+t);0<e.imgBytes&&Af===0&&(Af=62500*tv());var s=setTimeout(function(){if(e.waitingForImages=!1,e.count===0&&(e.stylesheets&&Cu(e,e.stylesheets),e.unsuspend)){var c=e.unsuspend;e.unsuspend=null,c()}},(e.imgBytes>Af?50:800)+t);return e.unsuspend=a,function(){e.unsuspend=null,clearTimeout(n),clearTimeout(s)}}:null}function Tu(){if(this.count--,this.count===0&&(this.imgCount===0||!this.waitingForImages)){if(this.stylesheets)Cu(this,this.stylesheets);else if(this.unsuspend){var e=this.unsuspend;this.unsuspend=null,e()}}}var zu=null;function Cu(e,t){e.stylesheets=null,e.unsuspend!==null&&(e.count++,zu=new Map,t.forEach(Mv,e),zu=null,Tu.call(e))}function Mv(e,t){if(!(t.state.loading&4)){var a=zu.get(e);if(a)var n=a.get(null);else{a=new Map,zu.set(e,a);for(var s=e.querySelectorAll("link[data-precedence],style[data-precedence]"),c=0;c<s.length;c++){var d=s[c];(d.nodeName==="LINK"||d.getAttribute("media")!=="not all")&&(a.set(d.dataset.precedence,d),n=d)}n&&a.set(null,n)}s=t.instance,d=s.getAttribute("data-precedence"),c=a.get(d)||n,c===n&&a.set(null,s),a.set(d,s),this.count++,n=Tu.bind(this),s.addEventListener("load",n),s.addEventListener("error",n),c?c.parentNode.insertBefore(s,c.nextSibling):(e=e.nodeType===9?e.head:e,e.insertBefore(s,e.firstChild)),t.state.loading|=4}}var cc={$$typeof:U,Provider:null,Consumer:null,_currentValue:me,_currentValue2:me,_threadCount:0};function Sv(e,t,a,n,s,c,d,w,L){this.tag=1,this.containerInfo=e,this.pingCache=this.current=this.pendingChildren=null,this.timeoutHandle=-1,this.callbackNode=this.next=this.pendingContext=this.context=this.cancelPendingCommit=null,this.callbackPriority=0,this.expirationTimes=zs(-1),this.entangledLanes=this.shellSuspendCounter=this.errorRecoveryDisabledLanes=this.expiredLanes=this.warmLanes=this.pingedLanes=this.suspendedLanes=this.pendingLanes=0,this.entanglements=zs(0),this.hiddenUpdates=zs(null),this.identifierPrefix=n,this.onUncaughtError=s,this.onCaughtError=c,this.onRecoverableError=d,this.pooledCache=null,this.pooledCacheLanes=0,this.formState=L,this.incompleteTransitions=new Map}function Lm(e,t,a,n,s,c,d,w,L,re,be,Ce){return e=new Sv(e,t,a,d,L,re,be,Ce,w),t=1,c===!0&&(t|=24),c=va(3,null,null,t),e.current=c,c.stateNode=e,t=Gi(),t.refCount++,e.pooledCache=t,t.refCount++,c.memoizedState={element:n,isDehydrated:a,cache:t},He(c),e}function jm(e){return e?(e=cl,e):cl}function qm(e,t,a,n,s,c){s=jm(s),n.context===null?n.context=s:n.pendingContext=s,n=Le(t),n.payload={element:a},c=c===void 0?null:c,c!==null&&(n.callback=c),a=je(e,n,t),a!==null&&(gn(a,e,t),De(a,e,t))}function Gm(e,t){if(e=e.memoizedState,e!==null&&e.dehydrated!==null){var a=e.retryLane;e.retryLane=a!==0&&a<t?a:t}}function Ef(e,t){Gm(e,t),(e=e.alternate)&&Gm(e,t)}function Fm(e){if(e.tag===13||e.tag===31){var t=qn(e,67108864);t!==null&&gn(t,e,67108864),Ef(e,67108864)}}function Vm(e){if(e.tag===13||e.tag===31){var t=Rn();t=jl(t);var a=qn(e,t);a!==null&&gn(a,e,t),Ef(e,t)}}var _u=!0;function Tv(e,t,a,n){var s=G.T;G.T=null;var c=j.p;try{j.p=2,Rf(e,t,a,n)}finally{j.p=c,G.T=s}}function zv(e,t,a,n){var s=G.T;G.T=null;var c=j.p;try{j.p=8,Rf(e,t,a,n)}finally{j.p=c,G.T=s}}function Rf(e,t,a,n){if(_u){var s=Nf(n);if(s===null)vf(e,t,n,Au,a),Ym(e,n);else if(_v(s,e,t,a,n))n.stopPropagation();else if(Ym(e,n),t&4&&-1<Cv.indexOf(e)){for(;s!==null;){var c=zo(s);if(c!==null)switch(c.tag){case 3:if(c=c.stateNode,c.current.memoizedState.isDehydrated){var d=_a(c.pendingLanes);if(d!==0){var w=c;for(w.pendingLanes|=2,w.entangledLanes|=2;d;){var L=1<<31-Vt(d);w.entanglements[1]|=L,d&=~L}Go(c),(Dt&6)===0&&(uu=qt()+500,nc(0))}}break;case 31:case 13:w=qn(c,2),w!==null&&gn(w,c,2),hu(),Ef(c,2)}if(c=Nf(n),c===null&&vf(e,t,n,Au,a),c===s)break;s=c}s!==null&&n.stopPropagation()}else vf(e,t,n,null,a)}}function Nf(e){return e=zi(e),Df(e)}var Au=null;function Df(e){if(Au=null,e=no(e),e!==null){var t=f(e);if(t===null)e=null;else{var a=t.tag;if(a===13){if(e=h(t),e!==null)return e;e=null}else if(a===31){if(e=v(t),e!==null)return e;e=null}else if(a===3){if(t.stateNode.current.memoizedState.isDehydrated)return t.tag===3?t.stateNode.containerInfo:null;e=null}else t!==e&&(e=null)}}return Au=e,null}function Km(e){switch(e){case"beforetoggle":case"cancel":case"click":case"close":case"contextmenu":case"copy":case"cut":case"auxclick":case"dblclick":case"dragend":case"dragstart":case"drop":case"focusin":case"focusout":case"input":case"invalid":case"keydown":case"keypress":case"keyup":case"mousedown":case"mouseup":case"paste":case"pause":case"play":case"pointercancel":case"pointerdown":case"pointerup":case"ratechange":case"reset":case"resize":case"seeked":case"submit":case"toggle":case"touchcancel":case"touchend":case"touchstart":case"volumechange":case"change":case"selectionchange":case"textInput":case"compositionstart":case"compositionend":case"compositionupdate":case"beforeblur":case"afterblur":case"beforeinput":case"blur":case"fullscreenchange":case"focus":case"hashchange":case"popstate":case"select":case"selectstart":return 2;case"drag":case"dragenter":case"dragexit":case"dragleave":case"dragover":case"mousemove":case"mouseout":case"mouseover":case"pointermove":case"pointerout":case"pointerover":case"scroll":case"touchmove":case"wheel":case"mouseenter":case"mouseleave":case"pointerenter":case"pointerleave":return 8;case"message":switch(to()){case La:return 2;case ao:return 8;case Pa:case ut:return 32;case xa:return 268435456;default:return 32}default:return 32}}var Uf=!1,ds=null,ms=null,ps=null,uc=new Map,fc=new Map,vs=[],Cv="mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset".split(" ");function Ym(e,t){switch(e){case"focusin":case"focusout":ds=null;break;case"dragenter":case"dragleave":ms=null;break;case"mouseover":case"mouseout":ps=null;break;case"pointerover":case"pointerout":uc.delete(t.pointerId);break;case"gotpointercapture":case"lostpointercapture":fc.delete(t.pointerId)}}function hc(e,t,a,n,s,c){return e===null||e.nativeEvent!==c?(e={blockedOn:t,domEventName:a,eventSystemFlags:n,nativeEvent:c,targetContainers:[s]},t!==null&&(t=zo(t),t!==null&&Fm(t)),e):(e.eventSystemFlags|=n,t=e.targetContainers,s!==null&&t.indexOf(s)===-1&&t.push(s),e)}function _v(e,t,a,n,s){switch(t){case"focusin":return ds=hc(ds,e,t,a,n,s),!0;case"dragenter":return ms=hc(ms,e,t,a,n,s),!0;case"mouseover":return ps=hc(ps,e,t,a,n,s),!0;case"pointerover":var c=s.pointerId;return uc.set(c,hc(uc.get(c)||null,e,t,a,n,s)),!0;case"gotpointercapture":return c=s.pointerId,fc.set(c,hc(fc.get(c)||null,e,t,a,n,s)),!0}return!1}function Wm(e){var t=no(e.target);if(t!==null){var a=f(t);if(a!==null){if(t=a.tag,t===13){if(t=h(a),t!==null){e.blockedOn=t,xi(e.priority,function(){Vm(a)});return}}else if(t===31){if(t=v(a),t!==null){e.blockedOn=t,xi(e.priority,function(){Vm(a)});return}}else if(t===3&&a.stateNode.current.memoizedState.isDehydrated){e.blockedOn=a.tag===3?a.stateNode.containerInfo:null;return}}}e.blockedOn=null}function Eu(e){if(e.blockedOn!==null)return!1;for(var t=e.targetContainers;0<t.length;){var a=Nf(e.nativeEvent);if(a===null){a=e.nativeEvent;var n=new a.constructor(a.type,a);Ti=n,a.target.dispatchEvent(n),Ti=null}else return t=zo(a),t!==null&&Fm(t),e.blockedOn=a,!1;t.shift()}return!0}function Xm(e,t,a){Eu(e)&&a.delete(t)}function Av(){Uf=!1,ds!==null&&Eu(ds)&&(ds=null),ms!==null&&Eu(ms)&&(ms=null),ps!==null&&Eu(ps)&&(ps=null),uc.forEach(Xm),fc.forEach(Xm)}function Ru(e,t){e.blockedOn===t&&(e.blockedOn=null,Uf||(Uf=!0,o.unstable_scheduleCallback(o.unstable_NormalPriority,Av)))}var Nu=null;function Pm(e){Nu!==e&&(Nu=e,o.unstable_scheduleCallback(o.unstable_NormalPriority,function(){Nu===e&&(Nu=null);for(var t=0;t<e.length;t+=3){var a=e[t],n=e[t+1],s=e[t+2];if(typeof n!="function"){if(Df(n||a)===null)continue;break}var c=zo(a);c!==null&&(e.splice(t,3),t-=3,R0(c,{pending:!0,data:s,method:a.method,action:n},n,s))}}))}function lr(e){function t(L){return Ru(L,e)}ds!==null&&Ru(ds,e),ms!==null&&Ru(ms,e),ps!==null&&Ru(ps,e),uc.forEach(t),fc.forEach(t);for(var a=0;a<vs.length;a++){var n=vs[a];n.blockedOn===e&&(n.blockedOn=null)}for(;0<vs.length&&(a=vs[0],a.blockedOn===null);)Wm(a),a.blockedOn===null&&vs.shift();if(a=(e.ownerDocument||e).$$reactFormReplay,a!=null)for(n=0;n<a.length;n+=3){var s=a[n],c=a[n+1],d=s[ba]||null;if(typeof c=="function")d||Pm(a);else if(d){var w=null;if(c&&c.hasAttribute("formAction")){if(s=c,d=c[ba]||null)w=d.formAction;else if(Df(s)!==null)continue}else w=d.action;typeof w=="function"?a[n+1]=w:(a.splice(n,3),n-=3),Pm(a)}}}function Zm(){function e(c){c.canIntercept&&c.info==="react-transition"&&c.intercept({handler:function(){return new Promise(function(d){return s=d})},focusReset:"manual",scroll:"manual"})}function t(){s!==null&&(s(),s=null),n||setTimeout(a,20)}function a(){if(!n&&!navigation.transition){var c=navigation.currentEntry;c&&c.url!=null&&navigation.navigate(c.url,{state:c.getState(),info:"react-transition",history:"replace"})}}if(typeof navigation=="object"){var n=!1,s=null;return navigation.addEventListener("navigate",e),navigation.addEventListener("navigatesuccess",t),navigation.addEventListener("navigateerror",t),setTimeout(a,100),function(){n=!0,navigation.removeEventListener("navigate",e),navigation.removeEventListener("navigatesuccess",t),navigation.removeEventListener("navigateerror",t),s!==null&&(s(),s=null)}}}function Of(e){this._internalRoot=e}Du.prototype.render=Of.prototype.render=function(e){var t=this._internalRoot;if(t===null)throw Error(u(409));var a=t.current,n=Rn();qm(a,n,e,t,null,null)},Du.prototype.unmount=Of.prototype.unmount=function(){var e=this._internalRoot;if(e!==null){this._internalRoot=null;var t=e.containerInfo;qm(e.current,2,null,e,null,null),hu(),t[To]=null}};function Du(e){this._internalRoot=e}Du.prototype.unstable_scheduleHydration=function(e){if(e){var t=yi();e={blockedOn:null,target:e,priority:t};for(var a=0;a<vs.length&&t!==0&&t<vs[a].priority;a++);vs.splice(a,0,e),a===0&&Wm(e)}};var Qm=l.version;if(Qm!=="19.2.8")throw Error(u(527,Qm,"19.2.8"));j.findDOMNode=function(e){var t=e._reactInternals;if(t===void 0)throw typeof e.render=="function"?Error(u(188)):(e=Object.keys(e).join(","),Error(u(268,e)));return e=p(t),e=e!==null?b(e):null,e=e===null?null:e.stateNode,e};var Ev={bundleType:0,version:"19.2.8",rendererPackageName:"react-dom",currentDispatcherRef:G,reconcilerVersion:"19.2.8"};if(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__<"u"){var Uu=__REACT_DEVTOOLS_GLOBAL_HOOK__;if(!Uu.isDisabled&&Uu.supportsFiber)try{ln=Uu.inject(Ev),wa=Uu}catch{}}return mc.createRoot=function(e,t){if(!i(e))throw Error(u(299));var a=!1,n="",s=nd,c=od,d=ld;return t!=null&&(t.unstable_strictMode===!0&&(a=!0),t.identifierPrefix!==void 0&&(n=t.identifierPrefix),t.onUncaughtError!==void 0&&(s=t.onUncaughtError),t.onCaughtError!==void 0&&(c=t.onCaughtError),t.onRecoverableError!==void 0&&(d=t.onRecoverableError)),t=Lm(e,1,!1,null,null,a,n,null,s,c,d,Zm),e[To]=t.current,pf(e),new Of(t)},mc.hydrateRoot=function(e,t,a){if(!i(e))throw Error(u(299));var n=!1,s="",c=nd,d=od,w=ld,L=null;return a!=null&&(a.unstable_strictMode===!0&&(n=!0),a.identifierPrefix!==void 0&&(s=a.identifierPrefix),a.onUncaughtError!==void 0&&(c=a.onUncaughtError),a.onCaughtError!==void 0&&(d=a.onCaughtError),a.onRecoverableError!==void 0&&(w=a.onRecoverableError),a.formState!==void 0&&(L=a.formState)),t=Lm(e,1,!0,t,a??null,n,s,L,c,d,w,Zm),t.context=jm(null),a=t.current,n=Rn(),n=jl(n),s=Le(n),s.callback=null,je(a,s,n),a=n,t.current.lanes=a,$o(t,a),Go(t),e[To]=t.current,pf(e),new Du(t)},mc.version="19.2.8",mc}var m1;function M2(){if(m1)return Hf.exports;m1=1;function o(){if(!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__>"u"||typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE!="function"))try{__REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(o)}catch(l){console.error(l)}}return o(),Hf.exports=b2(),Hf.exports}var S2=M2();const T2="modulepreload",z2=function(o,l){return new URL(o,l).href},p1={},eo=function(l,r,u){let i=Promise.resolve();if(r&&r.length>0){let h=function(b){return Promise.all(b.map(S=>Promise.resolve(S).then(C=>({status:"fulfilled",value:C}),C=>({status:"rejected",reason:C}))))};const v=document.getElementsByTagName("link"),m=document.querySelector("meta[property=csp-nonce]"),p=m?.nonce||m?.getAttribute("nonce");i=h(r.map(b=>{if(b=z2(b,u),b in p1)return;p1[b]=!0;const S=b.endsWith(".css"),C=S?'[rel="stylesheet"]':"";if(!!u)for(let H=v.length-1;H>=0;H--){const q=v[H];if(q.href===b&&(!S||q.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${b}"]${C}`))return;const D=document.createElement("link");if(D.rel=S?"stylesheet":T2,S||(D.as="script"),D.crossOrigin="",D.href=b,p&&D.setAttribute("nonce",p),document.head.appendChild(D),S)return new Promise((H,q)=>{D.addEventListener("load",H),D.addEventListener("error",()=>q(new Error(`Unable to preload CSS for ${b}`)))})}))}function f(h){const v=new Event("vite:preloadError",{cancelable:!0});if(v.payload=h,window.dispatchEvent(v),!v.defaultPrevented)throw h}return i.then(h=>{for(const v of h||[])v.status==="rejected"&&f(v.reason);return l().catch(f)})},Bl=()=>typeof window<"u"&&"__TAURI_INTERNALS__"in window;async function pi(o,l){if(Bl()){const{invoke:u}=await eo(async()=>{const{invoke:i}=await Promise.resolve().then(()=>Z3);return{invoke:i}},void 0,import.meta.url);return u(o,l)}const{webInvoke:r}=await eo(async()=>{const{webInvoke:u}=await import("./webBackend-sNFyw_1v.js");return{webInvoke:u}},[],import.meta.url);return r(o,l)}const C2=Object.freeze(Object.defineProperty({__proto__:null,invoke:pi,isDesktop:Bl},Symbol.toStringTag,{value:"Module"}));async function Q1(o,l){if(Bl()){const{listen:u}=await eo(async()=>{const{listen:i}=await import("./event-BZ79yHFs.js");return{listen:i}},__vite__mapDeps([0,1]),import.meta.url);return u(o,l)}const{onProgress:r}=await eo(async()=>{const{onProgress:u}=await import("./webBackend-sNFyw_1v.js");return{onProgress:u}},[],import.meta.url);return r(u=>l({payload:u}))}async function oh(o){if(Bl()){const{save:l}=await eo(async()=>{const{save:r}=await import("./index-BVTacFom.js");return{save:r}},__vite__mapDeps([2,1]),import.meta.url);return l(o)}return o.defaultPath??"download"}const I1=new Map;async function J1(o){if(Bl()){const{open:l}=await eo(async()=>{const{open:u}=await import("./index-BVTacFom.js");return{open:u}},__vite__mapDeps([2,1]),import.meta.url),r=await l(o);return Array.isArray(r)?r[0]??null:r}return new Promise(l=>{const r=document.createElement("input");r.type="file";const u=o.filters?.flatMap(i=>i.extensions.map(f=>`.${f}`))??[];u.length&&(r.accept=u.join(",")),r.onchange=()=>{const i=r.files?.[0];if(!i)return l(null);const f=`browser-file:${i.name}`;I1.set(f,i),l(f)},r.oncancel=()=>l(null),r.click()})}const ia=(()=>{try{const o=new URLSearchParams(window.location.search).get("mobile");if(o==="0")return!1;if(o==="1")return!0;const l=navigator.userAgent;return/iPhone|iPad|iPod|Android/i.test(l)||/Macintosh/.test(l)&&navigator.maxTouchPoints>1}catch{return!1}})(),Zu="该设备内存有限，已使用轻量模式",bc="terrainLab.mounting",Gf="terrainLab.safeMode",ph=()=>{try{return window.localStorage}catch{return null}},$1=(()=>{try{const o=ph(),l=new URLSearchParams(window.location.search).get("safe");return l==="0"?(o?.removeItem(Gf),o?.removeItem(bc),!1):l==="1"?!0:!ia||!o?!1:(o.getItem(bc)&&(o.removeItem(bc),o.setItem(Gf,"1")),o.getItem(Gf)==="1")}catch{return!1}})(),xs=$1;function ep(){ia&&ph()?.setItem(bc,String(Date.now()))}function lh(){ph()?.removeItem(bc)}const _2=Object.freeze(Object.defineProperty({__proto__:null,LIGHT_MODE_NOTICE:Zu,crashedLastTime:$1,isConstrainedDevice:ia,isSafeMode:xs,listen:Q1,markMountFinished:lh,markMountStarted:ep,open:J1,pickedFiles:I1,save:oh},Symbol.toStringTag,{value:"Module"}));let Ff=[];async function A2(o){const l=await pi("generate_terrain",{config:o});return ia&&await E2(l),l}async function E2(o){for(const r of Ff)URL.revokeObjectURL(r);Ff=[];const l=o.analysisPreviews;for(const r of Object.keys(l)){const u=l[r];if(!(typeof u!="string"||!u.startsWith("data:")))try{const i=await(await fetch(u)).blob(),f=URL.createObjectURL(i);Ff.push(f),l[r]=f}catch{}}}async function R2(o,l,r){try{const{decodeCityBinary:u}=await eo(async()=>{const{decodeCityBinary:h}=await Promise.resolve().then(()=>H2);return{decodeCityBinary:h}},void 0,import.meta.url),i=await pi("city_tile_bin",{index:o,column:l,row:r}),f=i instanceof ArrayBuffer?i:new Uint8Array(i).buffer;return u(f)}catch(u){const i=await fetch(`/city-tiles/${o}-${l}-${r}.json`);if(!i.ok)throw u;const[f]=await i.json();return f}}async function N2(o){return pi("city_scene",{index:o})}async function D2(o,l,r){if(!Bl()){const{exportPng:u}=await eo(async()=>{const{exportPng:i}=await import("./webBackend-sNFyw_1v.js");return{exportPng:i}},[],import.meta.url);return u(o,l,r)}return pi("export_terrain",{config:o,outputPath:l,outputSize:r})}async function U2(o,l){if(!Bl()){const{projectToFile:r}=await eo(async()=>{const{projectToFile:u}=await import("./webBackend-sNFyw_1v.js");return{projectToFile:u}},[],import.meta.url);return r(o,l)}return pi("save_project",{project:o,outputPath:l})}async function O2(o){if(!Bl()){const{pickedFiles:l}=await eo(async()=>{const{pickedFiles:i}=await Promise.resolve().then(()=>_2);return{pickedFiles:i}},void 0,import.meta.url),r=l.get(o);if(!r)throw new Error("the chosen project file is no longer available");const{projectFromBytes:u}=await eo(async()=>{const{projectFromBytes:i}=await import("./webBackend-sNFyw_1v.js");return{projectFromBytes:i}},[],import.meta.url);return u(new Uint8Array(await r.arrayBuffer()))}return pi("load_project",{inputPath:o})}const rr=(()=>{try{return typeof window<"u"&&new URLSearchParams(window.location.search).get("perf")==="1"}catch{return!1}})(),Zt={frames:0,frameMs:0,worstFrameMs:0,renderCpuMs:0,drawCalls:0,triangles:0,geometries:0,textures:0,timings:{}};rr&&typeof window<"u"&&(window.__PERF__=Zt);function Va(o,l){rr&&(o.endsWith("MaxMs")?Zt.timings[o]=Math.max(Zt.timings[o]??0,l):Zt.timings[o]=l)}function k2(o,l){if(!rr)return l();const r=performance.now();try{return l()}finally{Va(o,performance.now()-r)}}function B2(o){if(!rr)return{begin(){},end(){},dispose(){}};o.info.autoReset=!1;const l=document.createElement("div");l.style.cssText="position:fixed;left:8px;top:8px;z-index:99999;padding:4px 8px;background:rgba(0,0,0,.65);color:#9f9;font:11px/1.35 ui-monospace,monospace;white-space:pre;pointer-events:none;border-radius:4px",document.body.appendChild(l);let r=0,u=0;return{begin(){o.info.reset(),u=performance.now()},end(){const i=performance.now(),f=i-u;if(Zt.renderCpuMs=Zt.renderCpuMs===0?f:Zt.renderCpuMs*.9+f*.1,r>0){const h=i-r;Zt.frameMs=Zt.frameMs===0?h:Zt.frameMs*.9+h*.1,Zt.worstFrameMs=Math.max(Zt.worstFrameMs*.995,h)}if(r=i,Zt.frames+=1,Zt.drawCalls=o.info.render.calls,Zt.triangles=o.info.render.triangles,Zt.geometries=o.info.memory.geometries,Zt.textures=o.info.memory.textures,Zt.frames%15===0){const h=Zt.timings;l.textContent=`frame ${Zt.frameMs.toFixed(1)} ms (worst ${Zt.worstFrameMs.toFixed(0)})  render ${Zt.renderCpuMs.toFixed(1)}
draws ${Zt.drawCalls}  tris ${(Zt.triangles/1e3).toFixed(0)}k  geo ${Zt.geometries}  tex ${Zt.textures}
`+Object.entries(h).map(([v,m])=>`${v} ${m.toFixed(0)}`).join("  ")}},dispose(){l.remove()}}}function tp(o){const l=new DataView(o,0,8),r=String.fromCharCode(l.getUint8(0),l.getUint8(1),l.getUint8(2),l.getUint8(3));if(r!=="CSB1")throw new Error(`not a city scene container (magic ${JSON.stringify(r)})`);const u=l.getUint32(4,!0),i=JSON.parse(new TextDecoder().decode(new Uint8Array(o,8,u))),f=Math.ceil((8+u)/4)*4,h=([p,b])=>new Float32Array(o,f+p,b/4),v=([p,b])=>new Uint32Array(o,f+p,b/4),m=([p,b])=>new Uint8Array(o,f+p,b);return{...i,meshes:i.meshes.map(p=>({...p,positions:h(p.positions),normals:m(p.normals),colors:p.colors?m(p.colors):void 0,uvs:p.uvs?h(p.uvs):void 0,indices:v(p.indices)})),instances:i.instances.map(p=>({...p,data:h(p.data)})),textures:i.textures.map(p=>({...p,data:m(p.data)}))}}async function ap(o){const{invoke:l}=await eo(async()=>{const{invoke:f}=await Promise.resolve().then(()=>C2);return{invoke:f}},void 0,import.meta.url),r=performance.now(),u=await l("city_scene_bin",{index:o});Va("cityFetchMs",performance.now()-r);const i=u instanceof ArrayBuffer?u:new Uint8Array(u).buffer;return k2("cityDecodeMs",()=>tp(i))}const H2=Object.freeze(Object.defineProperty({__proto__:null,decodeCityBinary:tp,loadCityScene:ap},Symbol.toStringTag,{value:"Module"})),Ya={treeDetail:450,treeCoreNear:150,treeCoreFar:500,hedge:350,tuft:180,tallProp:380,lowProp:160,fineStatic:700,flatStatic:1500,moveEpsilon:2.5,turnCosine:.9997,frustumMarginM:90,shadowTrees:700,shadowAll:2200},L2=/^(trim\.|wire|sign\/|signal\.|awning|balcony)/,j2=/^(kerb|marking\.|barrier|median)/;function q2(o){const{key:l,part:r}=o;return l.startsWith("tree/")?Number.POSITIVE_INFINITY:l==="hedge"?Ya.hedge:l==="tuft"?Ya.tuft:l.startsWith("furniture/")?/lamp|pole|gantry|sign\.guide|shelter/.test(l)?Ya.tallProp:Ya.lowProp:Number.POSITIVE_INFINITY}function G2(o){return L2.test(o)?Ya.fineStatic:j2.test(o)?Ya.flatStatic:Number.POSITIVE_INFINITY}class F2{constructor(l){this.handles=l;for(const r of l.statics){const u=r.geometry.boundingBox??new fh().setFromObject(r);this.statics.push({object:r,box:u,radius:G2(r.name),cast:r.castShadow})}this.sets=l.lodSets,this.setRadius=this.sets.map(q2);for(const r of this.sets)r.mesh.userData.cast=r.mesh.castShadow}statics=[];sets;setRadius;lastCamera=new rt(Number.NaN,0,0);lastForward=new rt(0,0,-1);frustum=null;shadowTier=-1;keptInstances=0;hiddenStatics=0;update(l,r,u,i){const f=i<Ya.shadowTrees?0:i<Ya.shadowAll?1:2;let h=!1;if(f!==this.shadowTier){this.shadowTier=f,h=!0;for(const m of this.statics)m.object.castShadow=m.cast&&f<2;for(const m of this.sets)m.mesh.castShadow=!!m.mesh.userData.cast&&f<1}return!Number.isNaN(this.lastCamera.x)&&l.distanceToSquared(this.lastCamera)<=Ya.moveEpsilon*Ya.moveEpsilon&&r.dot(this.lastForward)>=Ya.turnCosine||(this.lastCamera.copy(l),this.lastForward.copy(r),this.frustum=u,this.rebuild(l)),h}rebuild(l){let r=0;for(const i of this.statics){const f=i.radius===Number.POSITIVE_INFINITY||i.box.distanceToPoint(l)<=i.radius;i.object.visible!==f&&(i.object.visible=f),f||(r+=1)}this.hiddenStatics=r;let u=0;for(let i=0;i<this.sets.length;i+=1){const f=this.sets[i],h=this.setRadius[i],v=f.key.startsWith("tree/"),m=v&&f.part==="mass",p=f.mesh,b=p.instanceMatrix.array,S=p.instanceColor?p.instanceColor.array:null,C=f.far,x=C?C.instanceMatrix.array:null,D=C&&C.instanceColor?C.instanceColor.array:null,H=f.bounds,q=this.frustum?this.frustum.planes:null,P=Ya.frustumMarginM,K=f.matrices,U=f.colors;let z=0,R=0;for(let V=0;V<f.count;V+=1){const J=V*4,Q=H[J]-l.x,ne=H[J+1]-l.y,B=H[J+2]-l.z,ae=Math.sqrt(Q*Q+ne*ne+B*B)-H[J+3];if(q){let Re=!1;for(let le=0;le<6;le+=1){const G=q[le],j=G.normal;if(j.x*H[J]+j.y*H[J+1]+j.z*H[J+2]+G.constant+P+H[J+3]<0){Re=!0;break}}if(Re)continue}let I=!1;if(v){if(m){if(ae<Ya.treeCoreNear)continue;I=ae>=Ya.treeCoreFar}else if(ae>=Ya.treeDetail)continue}else if(ae>=h)continue;const ge=V*16;if(I&&x){const Re=R*16;for(let le=0;le<16;le+=1)x[Re+le]=K[ge+le];D&&U&&(D[R*3]=U[V*3],D[R*3+1]=U[V*3+1],D[R*3+2]=U[V*3+2]),R+=1}else{const Re=z*16;for(let le=0;le<16;le+=1)b[Re+le]=K[ge+le];S&&U&&(S[z*3]=U[V*3],S[z*3+1]=U[V*3+1],S[z*3+2]=U[V*3+2]),z+=1}}this.commit(p,z,S!==null),C&&this.commit(C,R,D!==null),u+=z+R}this.keptInstances=u}commit(l,r,u){l.count=r,l.visible=r>0,r!==0&&(l.instanceMatrix.clearUpdateRanges(),l.instanceMatrix.addUpdateRange(0,r*16),l.instanceMatrix.needsUpdate=!0,u&&l.instanceColor&&(l.instanceColor.clearUpdateRanges(),l.instanceColor.addUpdateRange(0,r*3),l.instanceColor.needsUpdate=!0),l.computeBoundingSphere())}}const np=`
varying vec3 vFurRel;
// Cell hashes are taken on wrapped coordinates: a lawn is thousands of metres from the
// origin and a hash of a million loses every bit it has. The noise is periodic over a
// few hundred cells, which nobody can see.
float furHash(vec2 p) {
  p = mod(p, vec2(251.0, 241.0));
  vec3 q = fract(vec3(p.xyx) * 0.1031);
  q += dot(q, q.yzx + 33.33);
  return fract((q.x + q.y) * q.z);
}
float furNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(furHash(i), furHash(i + vec2(1, 0)), f.x), mix(furHash(i + vec2(0, 1)), furHash(i + vec2(1, 1)), f.x), f.y);
}
`,V2=`
${np}
uniform vec3 uFurTip;
uniform vec3 uFurDry;
`,K2=`
{
  vec2 p = (vFurRel + cameraPosition).xz;
  // How big one pixel is on the ground, metres: detail below it must not be drawn.
  // The footprint of one pixel on the ground, metres. At a grazing angle it is long
  // one way and short the other; a layer must be gone when its feature is smaller
  // than the *long* side, or it smears into streaks along the view direction.
  vec2 fdx = dFdx(p);
  vec2 fdy = dFdy(p);
  float px = max(length(fdx), length(fdy));
  float blade = furNoise(p * 70.0);
  float blade2 = furNoise(p * 31.0 + 17.0);
  float clump = furNoise(p * 9.0 + 3.0);
  float lushNoise = furNoise(p * 0.55 + 11.0);
  float macro = furNoise(p * 0.07 + 5.0);
  float fBlade = 1.0 - smoothstep(0.003, 0.014, px);
  float fClump = 1.0 - smoothstep(0.02, 0.10, px);
  float fPatch = 1.0 - smoothstep(0.25, 1.0, px);
  float fMacro = 1.0 - smoothstep(3.0, 10.0, px);
  float fine = mix(0.5, 0.5 * (blade + blade2), fBlade);
  float lum = 0.62 + 0.76 * fine;
  lum *= mix(1.0, 0.78 + 0.44 * clump, fClump);
  lum *= mix(1.0, 0.86 + 0.28 * lushNoise, fPatch);
  // Lush to dry across the lawn, a metres-wide drift.
  vec3 col = diffuseColor.rgb * lum;
  col = mix(col, col * uFurDry, smoothstep(0.35, 0.8, macro) * 0.55 * fMacro);
  // Sun-bleached tips where the blade is tall.
  col = mix(col, col * uFurTip, smoothstep(0.55, 1.0, fine) * 0.5);
  diffuseColor.rgb = col;
}
`,Y2=`
{
  vec2 p = (vFurRel + cameraPosition).xz;
  float px = max(length(dFdx(p)), length(dFdy(p)));
  float fc = 1.0 - smoothstep(0.02, 0.10, px);
  float e = 0.12;
  float c0 = furNoise(p * 9.0 + 3.0);
  vec2 g = vec2(furNoise(p * 9.0 + 3.0 + vec2(e, 0.0)) - c0, furNoise(p * 9.0 + 3.0 + vec2(0.0, e)) - c0) / e;
  vec3 tilt = vec3(-g.x, 0.0, -g.y) * 0.16 * fc;
  normal = normalize(normal + normalize(mat3(viewMatrix) * tilt + 1e-5) * length(tilt));
}
`,W2=`
#if NUM_DIR_LIGHTS > 0
{
  vec3 L = directionalLights[0].direction;
  vec3 V = normalize(vViewPosition);
  float ndv = clamp(dot(normalize(vNormal), V), 0.0, 1.0);
  // A pile of thin blades is lighter across the grain...
  float rim = pow(1.0 - ndv, 3.0);
  // ...and glows when the sun is behind it.
  float back = pow(max(dot(-V, L), 0.0), 3.0) * 0.5 + 0.25;
  vec3 pile = diffuseColor.rgb * directionalLights[0].color;
  totalEmissiveRadiance += pile * (rim * 0.55 * back + 0.05);
}
#endif
`;function X2(o){return o.replace("#include <common>",`#include <common>
varying vec3 vFurRel;`).replace("#include <project_vertex>",`#include <project_vertex>
#ifdef USE_INSTANCING
  vFurRel = (modelMatrix * instanceMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;
#else
  vFurRel = (modelMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;
#endif`)}function P2(o,l=14208906,r=12037482){const u=o.onBeforeCompile,i=o.customProgramCacheKey?.bind(o);o.customProgramCacheKey=()=>`${i?i():""}|fur`;const f=new $t(l),h=new $t(r);return f.multiplyScalar(1/.85),h.multiplyScalar(1/.72),o.onBeforeCompile=(v,m)=>{u?.(v,m),v.uniforms.uFurTip={value:f},v.uniforms.uFurDry={value:h},v.vertexShader=X2(v.vertexShader),v.fragmentShader=v.fragmentShader.replace("void main() {",`${V2}
void main() {`).replace("#include <color_fragment>",`#include <color_fragment>
${K2}`).replace("#include <normal_fragment_maps>",`#include <normal_fragment_maps>
${Y2}`).replace("#include <emissivemap_fragment>",`#include <emissivemap_fragment>
${W2}`)},o.roughness=1,o.needsUpdate=!0,o}const yc=16,Z2=.11;function Q2(o){const l=new Ke({color:7044167,roughness:1,metalness:0});return l.customProgramCacheKey=()=>"grass-shell",l.onBeforeCompile=r=>{r.uniforms.uShells={value:yc},r.uniforms.uShellHeight={value:Z2},r.vertexShader=r.vertexShader.replace("#include <common>",`#include <common>
varying vec3 vFurRel;
varying float vShell;
uniform float uShells;
uniform float uShellHeight;`).replace("#include <begin_vertex>",`#include <begin_vertex>
float shell = float(gl_InstanceID + 1) / uShells;
vShell = shell;
transformed += normalize(normal) * (uShellHeight * shell);`).replace("#include <project_vertex>",`#include <project_vertex>
vFurRel = (modelMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;`),r.fragmentShader=r.fragmentShader.replace("void main() {",`${np}
varying float vShell;
void main() {
  {
    vec2 p = (vFurRel + cameraPosition).xz;
    float px = max(length(dFdx(p)), length(dFdy(p)));
    // Blades a centimetre across, one per cell; the cell's own height decides how
    // many shells it reaches, and its width tapers toward the tip.
    float density = 90.0;
    vec2 cell = floor(p * density);
    vec2 local = fract(p * density) - 0.5;
    float h = 0.35 + 0.65 * furHash(cell);
    vec2 offset = (vec2(furHash(cell + 7.0), furHash(cell + 13.0)) - 0.5) * 0.45;
    float r = length(local - offset);
    float width = 0.46 * (1.0 - vShell / max(h, 0.05));
    // Fade to nothing as blades get thinner than a pixel: the base lawn takes over.
    float far = smoothstep(0.012, 0.05, px);
    // The blades thin out with distance and are gone at the edge of the carpet, so the
    // pile melts into the lawn the surface shader draws beyond it.
    float edge = smoothstep(14.0, 34.0, length(vFurRel.xz));
    if (vShell > h * (1.0 - edge) || r > width || far > 0.97) discard;
  }`).replace("#include <color_fragment>",`#include <color_fragment>
{
  float h = 0.35 + 0.65 * furHash(floor((vFurRel + cameraPosition).xz * 90.0));
  float t = vShell / max(h, 0.05);
  // Dark and damp at the root, bleached at the tip, one blade to the next.
  float tone = 0.55 + 0.9 * furHash(floor((vFurRel + cameraPosition).xz * 90.0) + 3.0);
  diffuseColor.rgb *= mix(0.62, 1.25, vShell) * (0.72 + 0.4 * tone);
  diffuseColor.rgb = mix(diffuseColor.rgb, diffuseColor.rgb * vec3(1.25, 1.18, 0.75), t * 0.45);
}`)},l}class I2{constructor(l,r,u=36){this.reachM=u,this.material=Q2();for(const i of l){const f=i.geometry.boundingBox??(i.geometry.computeBoundingBox(),i.geometry.boundingBox);this.tracked.push({object:i,box:f,shells:null})}}tracked=[];material;identity=new Mo;carpetMesh=null;carpet(l,r,u=40){const i=new gc(u*2,u*2,1,1);i.rotateX(-Math.PI/2);const f=new Ka(i,this.material,yc);for(let h=0;h<yc;h+=1)f.setMatrixAt(h,this.identity);f.instanceMatrix.needsUpdate=!0,f.position.y=l,f.castShadow=!1,f.receiveShadow=!0,f.frustumCulled=!1,r.add(f),this.carpetMesh=f}update(l){this.carpetMesh&&(this.carpetMesh.position.x=Math.round(l.x),this.carpetMesh.position.z=Math.round(l.z));for(const r of this.tracked){const u=r.object.visible&&r.box.distanceToPoint(l)<this.reachM;if(u&&!r.shells){const i=new Ka(r.object.geometry,this.material,yc);for(let f=0;f<yc;f+=1)i.setMatrixAt(f,this.identity);i.instanceMatrix.needsUpdate=!0,i.castShadow=!1,i.receiveShadow=!0,i.frustumCulled=!1,i.position.copy(r.object.position),i.quaternion.copy(r.object.quaternion),i.scale.copy(r.object.scale),r.object.parent?.add(i),r.shells=i}r.shells&&(r.shells.visible=u)}}dispose(){this.carpetMesh&&(this.carpetMesh.removeFromParent(),this.carpetMesh.geometry.dispose(),this.carpetMesh.dispose());for(const l of this.tracked)l.shells&&(l.shells.removeFromParent(),l.shells.dispose());this.material.dispose()}}const J2=32;class vh{exports;speciesKeys;constructor(l){this.exports=l;const r=[],u=new TextDecoder;for(let i=0;i<l.tree_species_count();i+=1){const f=l.tree_species_key(i);r.push(u.decode(new Uint8Array(l.memory.buffer,l.tree_buffer_ptr(),f)))}this.speciesKeys=r}static async load(l="./worldgen_trees.wasm"){const r=await fetch(l);if(!r.ok)throw new Error(`${l}: ${r.status} ${r.statusText}`);const u=await r.arrayBuffer(),{instance:i}=await WebAssembly.instantiate(u,{});return new vh(i.exports)}speciesIndex(l){return this.speciesKeys.indexOf(l)}grow(l,r){const u=Math.floor(l.seed),i=this.exports.tree_generate(l.species,u>>>0,Math.floor(u/4294967296)>>>0,l.height,l.openness??.7,l.age??.7,l.season??.5,l.health??1,l.lift??0,r),f=new Uint8Array(this.exports.memory.buffer,this.exports.tree_buffer_ptr(),i).slice().buffer,h=new DataView(f),v=U=>h.getUint32(U*4,!0),m=U=>h.getFloat32(U*4,!0);if(v(0)!==1162170964)throw new Error("not a tree buffer");const p=v(2),b=v(3),S=U=>[m(U),m(U+1),m(U+2)];let C=J2*4;const x=new Float32Array(f,C,p*8);C+=p*32;const D=new Float32Array(f,C,b*4);C+=b*16;const H=new Int8Array(f,C,b*4);C+=b*4;const q=new Int8Array(f,C,b*4);C+=b*4;const P=new Uint8Array(f,C,b*4);C+=b*4;const K=new Uint8Array(f,C,b*4);return{species:v(15),leafForm:v(16),lod:v(28),height:m(4),crownRadius:m(5),crownBase:m(6),trunkRadius:m(7),bark:S(8),cover:m(11),autumn:m(12),flush:m(13),bloom:m(14),foliage:S(17),autumnColour:S(20),bloomColour:S(23),leafAspect:m(26),fissure:m(27),segments:x,segmentCount:p,leafPlacement:D,leafDirection:H,leafNormal:q,leafShape:P,leafTint:K,leafCount:b}}}const $2=16;class gh{workers=[];busy=[];waiting=new Map;next=1;static create(l=Math.max(1,Math.min(3,(navigator.hardwareConcurrency??4)-1)),r="./worldgen_trees.wasm"){if(typeof Worker>"u")return null;try{const u=new gh,i=new URL(r,location.href).href;for(let f=0;f<l;f+=1){const h=new Worker(new URL(""+new URL("growWorker-B1vhxpPZ.js",import.meta.url).href,import.meta.url),{type:"module"});h.postMessage({init:i}),h.onmessage=v=>{const m=u.waiting.get(v.data.id);m&&(u.waiting.delete(v.data.id),u.busy[m.worker]-=1,v.data.data?m.resolve(v.data.data):m.reject(new Error(v.data.error??"worker failed")))},u.workers.push(h),u.busy.push(0)}return u}catch{return null}}get inFlight(){return this.busy.reduce((l,r)=>l+r,0)}get capacity(){return this.workers.length*$2}grow(l,r){let u=0;for(let f=1;f<this.busy.length;f+=1)this.busy[f]<this.busy[u]&&(u=f);const i=this.next++;return this.busy[u]+=1,new Promise((f,h)=>{this.waiting.set(i,{resolve:f,reject:h,worker:u}),this.workers[u].postMessage({id:i,spec:l,lod:r})})}dispose(){for(const l of this.workers)l.terminate();this.workers.length=0}}const hi=7,Ko=256,eg=typeof location<"u"&&new URLSearchParams(location.search).get("treedebug")==="nobump"?"true":"false",tg=typeof location<"u"&&new URLSearchParams(location.search).get("treedebug")==="nodisp"?"0.0":"1.0",ag=`
#ifndef USE_INSTANCING
attribute float aTreeKind;
attribute vec4 aSegA;
attribute vec4 aSegB;
attribute vec4 aLeafPos;
attribute vec4 aLeafDir;
attribute vec4 aLeafNor;
attribute vec4 aLeafShape;
attribute vec4 aLeafTint;
attribute float aTreeId;
#endif
uniform sampler2D uTreeTable;
varying float vTreeKind;
varying vec4 vTreeColour;
varying vec4 vTreeA;
varying vec4 vTreeB;
varying vec2 vBarkAR;
varying float vAbove;

#ifndef USE_INSTANCING
vec4 treeRow(float id, int k) {
  int i = int(id + 0.5);
  return texelFetch(uTreeTable, ivec2((i & ${Ko-1}) * ${hi} + k, i >> 8), 0);
}

float treeHash(vec3 p) {
  return fract(sin(dot(p, vec3(12.9898, 78.233, 37.719))) * 43758.5453);
}

// One vertex of a branch or a leaf: where it is, which way it faces.
void treeShape(out vec3 pos, out vec3 nor) {
  vTreeKind = aTreeKind;
  if (aTreeKind < 1.5) {
    vec3 axis = aSegB.xyz - aSegA.xyz;
    float len = max(length(axis), 1e-4);
    vec3 w = axis / len;
    vec3 refv = abs(w.y) < 0.9 ? vec3(0.0, 1.0, 0.0) : vec3(1.0, 0.0, 0.0);
    vec3 u = normalize(cross(refv, w));
    vec3 v = cross(w, u);
    vec4 bark = treeRow(aTreeId, 3);
    float t = clamp(position.y, 0.0, 1.0);
    // Each tube runs a little past both ends, so where a branch bends the joint is
    // overlapped wood and not a crack.
    float ext = min(0.8 * max(aSegA.w, aSegB.w), 0.35 * len) / len;
    float tt = mix(-ext, 1.0 + ext, position.y);
    // The taper carries on through the extension, so it meets the next tube's surface.
    float r = max(mix(aSegA.w, aSegB.w, tt), 0.002);
    // Bark is not a cylinder: ridges and furrows run along it and shift the surface.
    vec3 radialW = u * position.x + v * position.z;
    // The angle about the axis in the world's own frame, so the relief is the same on
    // both tubes where two overlap, whatever way each happens to be oriented.
    float ang = atan(radialW.z, radialW.x);
    vec3 centre = mix(aSegA.xyz, aSegB.xyz, tt);
    float hsh = treeHash(vec3(float(aTreeId), 3.7, 1.3));
    float ridge = sin(ang * 5.0 + centre.y * 0.8 + hsh * 6.0) + 0.6 * sin(ang * 11.0 - centre.y * 1.9 + hsh * 11.0) + 0.35 * sin(ang * 23.0 + centre.y * 3.1);
    float flare = r > 0.05 ? 1.0 : 0.0;
    r *= 1.0 + flare * bark.w * 0.11 * ridge * ${tg};
    vec3 radial = u * position.x + v * position.z;
    pos = centre + radial * r;
    nor = normalize(radial * len + w * (aSegA.w - aSegB.w));
    vTreeColour = vec4(bark.rgb, 1.0);
    vTreeA = vec4(w, bark.w);
    vTreeB = vec4(pos, 0.0);
    vBarkAR = vec2(ang, r);
    vAbove = pos.y - treeRow(aTreeId, 6).y;
  } else {
    float form = treeRow(aTreeId, 0).w;
    vec4 foliage = treeRow(aTreeId, 0);
    vec4 autumn = treeRow(aTreeId, 1);
    vec4 bloom = treeRow(aTreeId, 2);
    vec4 extra = treeRow(aTreeId, 4);
    vec4 crown = treeRow(aTreeId, 5);
    float len = aLeafPos.w;
    vec3 d = normalize(aLeafDir.xyz);
    vec3 n = normalize(aLeafNor.xyz - d * dot(aLeafNor.xyz, d));
    vec3 sd = cross(d, n);
    vec4 sh = aLeafShape;
    vec4 tn = aLeafTint;

    // A flower where the tree is in bloom, a leaf elsewhere.
    float lot = treeHash(aLeafPos.xyz * 7.3 + tn.xyz);
    bool flower = lot < bloom.w;
    float f = flower ? 10.0 + form : form;

    float hw = 0.5;
    if (flower) hw = 0.5;
    else if (form < 0.5 || (form > 1.5 && form < 2.5) || form > 4.5 && form < 5.5) hw = clamp(0.5 * extra.y * (0.9 + 0.25 * sh.z), 0.14, 0.62);
    else if (form < 1.5) hw = 0.62;
    else if (form < 3.5) hw = 0.62;
    else if (form < 4.5) hw = 0.38;
    else hw = 0.6;

    float x = position.x;
    float y = position.y;
    float curl = (sh.w - 0.5) * 2.0;
    pos = aLeafPos.xyz + d * (len * y) + sd * (len * hw * x);
    pos += n * len * (curl * 0.30 * y * y + 0.16 * hw * x * x * (1.0 - 0.5 * y));
    pos -= d * len * abs(curl) * 0.08 * y * y;
    nor = normalize(n - sd * 0.32 * x - d * curl * 0.5 * y);

    // Colour: the tree's foliage, this leaf's own hue, shade and age, and whether
    // it has turned yet.
    float turned = clamp((autumn.w * 1.25 - tn.w * 0.5) / 0.35, 0.0, 1.0);
    vec3 col = foliage.rgb * (0.80 + 0.40 * tn.x);
    col.g *= 0.94 + 0.12 * tn.x;
    col *= mix(0.66, 1.08, tn.y);
    col = mix(col, vec3(0.34, 0.26, 0.12), 0.55 * pow(tn.z, 3.0));
    vec3 aut = autumn.rgb * (0.75 + 0.5 * tn.x) * mix(0.8, 1.05, tn.y);
    col = mix(col, aut, turned);
    col = mix(col, col * vec3(1.22, 1.28, 0.62), extra.x);
    if (flower) col = bloom.rgb * (0.88 + 0.24 * tn.x);

    // Where this leaf sits in the crown, as a (squashed) ellipsoid: 1 at the lit outer
    // surface, 0 on the axis. Light reaching a leaf falls off steeply with depth, and
    // faster under the crown than on top of it, so the inside is deep shade, the underside
    // dark, and the sunlit skin bright: the contrast that makes a crown read as a volume.
    vec3 rel = (aLeafPos.xyz - crown.xyz) / vec3(max(extra.w, 0.5), max(crown.w, 0.5), max(extra.w, 0.5));
    float nrm = length(rel);
    float depthIn = clamp(1.0 - nrm, 0.0, 1.0);
    float under = smoothstep(-0.2, -0.9, rel.y);
    float ao = mix(1.0, 0.42, smoothstep(0.0, 0.55, depthIn));
    ao *= 1.0 - 0.45 * under;
    // The same noise that breaks a crown into clumps: lobes of light and dark.
    float lobe = treeHash(floor(aLeafPos.xyz * 0.9)) * 0.5 + 0.5;
    ao *= mix(0.78, 1.12, lobe);
    // Shade is cooler and bluer (skylight). The sun's own warmth comes from the
    // light; doubling it here as well took the blue out of every sunlit crown.
    vec3 shaded = col * vec3(0.55, 0.78, 0.82);
    vec3 sunlit = col * vec3(1.08, 1.06, 0.97);
    col = mix(shaded, sunlit, clamp(ao, 0.0, 1.0)) * clamp(0.38 + 0.80 * ao, 0.38, 1.15);
    vTreeColour = vec4(col, tn.y * ao);
    vTreeA = sh;
    vTreeB = vec4(x * hw, y, f, hw);
    vBarkAR = vec2(0.0);
    vAbove = 99.0;
  }
}
#endif
`,ng=`
vec3 objectNormal;
{
#ifndef USE_INSTANCING
  if (aTreeKind > 0.5) {
    vec3 tp;
    treeShape(tp, objectNormal);
  } else
#endif
  {
    vTreeKind = 0.0;
    vTreeColour = vec4(0.0);
    vTreeA = vec4(0.0);
    vTreeB = vec4(0.0);
    objectNormal = vec3(normal);
  }
}
#ifdef USE_TANGENT
  vec3 objectTangent = vec3(tangent.xyz);
#endif
`,og=`
vec3 transformed;
{
#ifndef USE_INSTANCING
  if (aTreeKind > 0.5) {
    vec3 tn;
    treeShape(transformed, tn);
  } else
#endif
  {
    vTreeKind = 0.0;
    vTreeColour = vec4(0.0);
    vTreeA = vec4(0.0);
    vTreeB = vec4(0.0);
    transformed = vec3(position);
  }
}
#ifdef USE_ALPHAHASH
  vPosition = vec3(position);
#endif
`,lg=`
varying float vTreeKind;
varying vec4 vTreeColour;
varying vec4 vTreeA;
varying vec4 vTreeB;
varying vec2 vBarkAR;
varying float vAbove;

float tHash(vec3 p) {
  // Wrapped, then a Hoskins-style hash: stable far from the origin and not degenerate at it.
  p = mod(p, vec3(251.0, 241.0, 239.0));
  p = fract(p * 0.1031);
  p += dot(p, p.zyx + 31.32);
  return fract((p.x + p.y) * p.z);
}

float tNoise(vec3 x) {
  vec3 i = floor(x);
  vec3 f = fract(x);
  f = f * f * (3.0 - 2.0 * f);
  return mix(
    mix(mix(tHash(i), tHash(i + vec3(1, 0, 0)), f.x), mix(tHash(i + vec3(0, 1, 0)), tHash(i + vec3(1, 1, 0)), f.x), f.y),
    mix(mix(tHash(i + vec3(0, 0, 1)), tHash(i + vec3(1, 0, 1)), f.x), mix(tHash(i + vec3(0, 1, 1)), tHash(i + vec3(1, 1, 1)), f.x), f.y),
    f.z);
}

// Worley cells in space: F1, F2 and a per-cell id. F2 - F1 is the network of edges
// between cells, which is what the furrows between bark plates are. In world space, so
// it is seamless over every joint and every branch.
vec3 tWorley(vec3 p) {
  vec3 ip = floor(p);
  vec3 fp = fract(p);
  float f1 = 8.0;
  float f2 = 8.0;
  float id = 0.0;
  for (int k = -1; k <= 1; k++) {
    for (int j = -1; j <= 1; j++) {
      for (int i = -1; i <= 1; i++) {
        vec3 g = vec3(float(i), float(j), float(k));
        vec3 cell = ip + g;
        vec3 o = vec3(tHash(cell + 1.7), tHash(cell + 5.3), tHash(cell + 9.1));
        float d = length(g + o - fp);
        if (d < f1) { f2 = f1; f1 = d; id = tHash(cell + 13.0); }
        else if (d < f2) { f2 = d; }
      }
    }
  }
  return vec3(f1, f2, id);
}

float sdSeg(vec2 p, vec2 a, vec2 b) {
  vec2 pa = p - a;
  vec2 ba = b - a;
  float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
  return length(pa - ba * h);
}

// Not an exact distance, but zero on the outline and right in sign and scale.
float sdEllipse(vec2 p, vec2 c, vec2 ab, float ang) {
  vec2 q = p - c;
  float cs = cos(ang);
  float sn = sin(ang);
  q = vec2(cs * q.x + sn * q.y, -sn * q.x + cs * q.y);
  return (length(q / ab) - 1.0) * min(ab.x, ab.y);
}

// The outline of one leaf, in units of its own length; negative inside. q.x is
// across the blade from the midrib, q.y along it from the stalk. 'vein' is how
// much of a vein this spot is.
float leafOutline(vec2 q, float form, float hw, vec4 sh, out float vein) {
  vein = 0.0;
  float t = q.y;
  float asym = sh.z - 0.5;
  if (form > 9.5) {
    // A flower: five petals about a centre.
    vec2 v = q - vec2(0.0, 0.5);
    float r = length(v);
    float th = atan(v.x, v.y);
    float R = 0.34 * (0.72 + 0.28 * abs(cos(2.5 * th + sh.z * 6.0)));
    vein = 1.0 - smoothstep(0.0, 0.07, r);
    return r - R;
  }
  if (form < 0.5 || (form > 1.5 && form < 2.5) || (form > 4.5 && form < 5.5)) {
    float a = form < 0.5 ? 0.62 + 0.38 * sh.z : (form < 2.5 ? 0.9 : 0.8);
    float b = form < 0.5 ? mix(0.95, 0.55, sh.y) : (form < 2.5 ? mix(0.8, 0.5, sh.y) : mix(1.3, 0.9, sh.y));
    float qx = q.x - asym * 0.14 * sin(3.14159 * t);
    float wn = pow(max(sin(3.14159 * pow(clamp(t, 0.0, 1.0), a)), 0.0), b);
    if (form > 1.5 && form < 2.5) {
      wn *= 1.0 + 0.09 * sh.x * (abs(fract(t * (9.0 + 9.0 * sh.x)) - 0.5) * 2.0 - 0.5);
    }
    float side = hw * 0.96 * wn * (1.0 + asym * 0.5 * sign(qx));
    float d = abs(qx) - side;
    float mid = 1.0 - smoothstep(0.004, 0.010 + 0.006 * (1.0 - t), abs(qx));
    float nv = 5.0 + 5.0 * sh.x + (form > 4.5 ? 2.0 : 0.0);
    float ph = t * nv - abs(qx) / max(hw, 0.1) * nv * 0.55;
    float lines = smoothstep(0.40, 0.5, abs(fract(ph) - 0.5));
    lines *= 1.0 - smoothstep(0.7, 0.95, abs(qx) / max(side, 1e-3));
    lines *= smoothstep(0.04, 0.12, t);
    vein = max(mid * step(t, 0.97), lines * 0.8);
    // The stalk, below the blade.
    d = min(d, sdSeg(q, vec2(0.0), vec2(0.0, 0.05)) - 0.010);
    return d;
  }
  if (form < 1.5) {
    // Palmately lobed: plane, maple.
    vec2 v = q - vec2(0.0, 0.40);
    float r = length(v);
    float th = atan(v.x, v.y);
    float lobe = 0.5 + 0.5 * cos(6.2832 * th);
    float E = (0.30 + 0.55 * sh.x) * (1.0 - smoothstep(2.1, 2.8, abs(th)));
    float R = 0.30 * (1.0 + E * pow(lobe, 1.4 + 1.2 * sh.y)) * (1.0 + asym * 0.25 * sin(th));
    float d = r - R;
    vein = pow(1.0 - abs(sin(3.14159 * th)), 14.0) * (1.0 - smoothstep(0.75, 0.98, r / R)) * step(abs(th), 2.3);
    vein = max(vein, (1.0 - smoothstep(0.0, 0.015, abs(q.x))) * step(q.y, 0.4) * step(0.03, q.y));
    d = min(d, sdSeg(q, vec2(0.0), vec2(0.0, 0.4)) - 0.010);
    return d;
  }
  if (form < 3.5) {
    // A shoot of needles: a flat feathery fan, a rachis down the middle with needles combed
    // out on both sides, so the spray reads as a soft filled shape and not a few hairs.
    vec2 v = q - vec2(0.0, 0.04);
    float r = length(v);
    float th = atan(v.x, v.y);
    float spread = 0.62 + 0.30 * sh.x;
    float teeth = 0.80 + 0.20 * abs(sin(th * (9.0 + 6.0 * sh.y) + sh.z * 6.0));
    float R = 0.96 * teeth * (1.0 - 0.18 * abs(th) / spread);
    float d = max(r - R, (abs(th) - spread) * max(r, 0.12));
    vein = (1.0 - smoothstep(0.0, 0.018, abs(q.x))) * step(0.04, q.y) * 0.7;
    return d;
  }
  if (form < 4.5) {
    // A compound leaf: leaflets in pairs on a rachis, one at the end.
    float d = sdSeg(q, vec2(0.0), vec2(0.0, 1.0)) - 0.007;
    vein = 1.0 - smoothstep(0.0, 0.012, d);
    for (int i = 0; i < 6; i++) {
      float fi = float(i);
      float ty = 0.15 + 0.125 * fi;
      float sz = 1.0 - 0.07 * fi;
      for (int s = 0; s < 2; s++) {
        float sg = s == 0 ? -1.0 : 1.0;
        vec2 c = vec2(sg * 0.14 * sz, ty + 0.06);
        float de = sdEllipse(q, c, vec2(0.15, 0.058) * sz, -sg * 0.95 + (sh.z - 0.5) * 0.3);
        d = min(d, de);
      }
    }
    d = min(d, sdEllipse(q, vec2(0.0, 0.93), vec2(0.05, 0.12), 1.5708));
    return d;
  }
  // A fan: ginkgo.
  vec2 v = q - vec2(0.0, 0.12);
  float r = length(v);
  float th = atan(v.x, v.y);
  float R = 0.86 * (1.0 + 0.05 * cos(th * 9.0 + sh.x * 5.0)) * (1.0 - 0.30 * exp(-th * th / 0.012));
  float d = max(r - R, (abs(th) - 1.05) * max(r, 0.1));
  vein = pow(abs(cos(th * 8.0)), 24.0) * step(0.06, r) * (1.0 - smoothstep(0.8, 1.0, r / R));
  d = min(d, sdSeg(q, vec2(0.0), vec2(0.0, 0.14)) - 0.010);
  return d;
}

// Coverage of the blade at this fragment, 0 outside the outline and 1 inside.
float treeLeafCoverage(out float vein, out float inner) {
  float d = leafOutline(vTreeB.xy, vTreeB.z, vTreeB.w, vTreeA, vein);
  float aa = max(fwidth(d), 1e-5);
  inner = clamp(-d / 0.08, 0.0, 1.0);
  // A feature thinner than a pixel (a needle, a twig of leaflets, a distant leaf)
  // must fade towards the mass it belongs to, not vanish: grow the outline by half
  // a pixel, so nothing is ever drawn thinner than one.
  d -= 0.5 * aa;
  return clamp(0.5 - d / aa, 0.0, 1.0);
}
`,sg=`
float tBark = 0.0;
if (vTreeKind > 0.5) {
  if (vTreeKind < 1.5) {
    vec3 axis = normalize(vTreeA.xyz);
    float fissure = vTreeA.w;
    float vert = smoothstep(0.5, 0.9, abs(axis.y));
    vec3 q = vTreeB.xyz;
    float R = max(vBarkAR.y, 0.004);
    // Plates about ten centimetres across the grain and several times that along it.
    vec3 sq = vec3(1.0, 1.0, 1.0);
    vec3 qa = q * 10.0;
    vec3 qb = q * 20.0;
    float vv = vert;
    qa.y = mix(qa.y, qa.y * 0.13, vv);
    qb.y = mix(qb.y, qb.y * 0.38, vv);
    vec3 wv = vec3(tNoise(qa * 0.35), tNoise(qa * 0.35 + 7.0), tNoise(qa * 0.35 + 17.0)) - 0.5;
    qa += wv * 1.6;
    vec3 wa = tWorley(qa);
    // Ragged edges: the furrow wanders, and pinches shut in places.
    float edge = wa.y - wa.x + (tNoise(qa * 2.3 + 3.0) - 0.5) * 0.22 + (tNoise(qa * 7.0) - 0.5) * 0.08;
    float furrow = 1.0 - smoothstep(0.0, 0.045 + 0.06 * fissure, edge);
    float rounding = smoothstep(0.0, 0.42, edge);
    vec3 wb = tWorley(qb + wv * 1.5);
    float crack = (1.0 - smoothstep(0.0, 0.05, wb.y - wb.x + (tNoise(qb * 1.5) - 0.5) * 0.12)) * (1.0 - furrow) * 0.6;
    float grain = tNoise(vec3(q.x * 40.0 + q.z * 40.0, q.y * 6.0, 0.5));
    float fine = tNoise(q * 200.0);
    float along = q.y;
    float plateId = wa.z;
    float dome = 1.0 - smoothstep(0.0, 0.75, wa.x);
    float plate = 0.35 + 0.65 * plateId;
    // Height in metres: each plate is a low dome, split by deep furrows, cracked across and fibrous.
    float h = (1.0 - furrow) * (0.020 + 0.040 * dome) * (0.7 + 0.8 * fissure) - crack * 0.004 + grain * 0.003;
    // Plates that have flaked off show bare wood.
    float flake = smoothstep(0.88, 0.95, plateId) * smoothstep(0.30, 0.55, tNoise(q * 5.0));
    float wornMask = 0.0;
    tBark = h * smoothstep(0.025, 0.07, R);
    float woodGrain = 0.55 + 0.45 * sin(q.x * 90.0 + q.z * 90.0 + 9.0 * tNoise(q * vec3(18.0, 4.0, 18.0)));
    vec3 woodCol = mix(vec3(0.30, 0.13, 0.045), vec3(0.78, 0.43, 0.15), woodGrain) * (0.8 + 0.4 * fine);
    // Bark itself: warm grey-brown, each plate its own tone, silvered on the high ones, near-black in the furrows.
    // Colour by age of the wood: thin young twigs are warm tan-brown, old thick limbs
    // dark brown. Only smooth-barked species (plane, ginkgo) stay pale, and mottled.
    float thick = smoothstep(0.015, 0.14, R);
    vec3 youngC = vec3(0.42, 0.27, 0.165);
    vec3 oldC = vec3(0.30, 0.200, 0.130);
    vec3 woody = mix(youngC, oldC, thick);
    float smoothBark = 1.0 - smoothstep(0.18, 0.40, fissure);
    vec3 paleC = mix(vTreeColour.rgb, vec3(0.30, 0.22, 0.15), 0.35) * vec3(1.25, 1.0, 0.72);
    vec3 base = mix(woody, mix(paleC, woody, 0.15 * thick), smoothBark);
    base *= mix(vec3(1.0), vTreeColour.rgb / max(dot(vTreeColour.rgb, vec3(0.33)), 0.02) * 0.9, 0.18);
    vec3 plateCol = base * (0.90 + 0.16 * plate) * (0.93 + 0.14 * grain) * (0.94 + 0.12 * dome);
    // Mottling on smooth bark: flaked patches of olive-grey and cream.
    float mott = tNoise(q * 6.0 + 21.0);
    plateCol = mix(plateCol, plateCol * vec3(0.92, 0.95, 0.88), smoothBark * smoothstep(0.5, 0.75, mott) * 0.25);
    vec3 creviceCol = vec3(0.045, 0.030, 0.021);
    // Real bark is matte and low in saturation, and varies a great deal in value over a
    // metre: weathering, damp, where rain runs down it. Plastic is uniform and clean.
    float lum0 = dot(plateCol, vec3(0.30, 0.59, 0.11));
    plateCol = mix(vec3(lum0), plateCol, 1.28);
    float weather = tNoise(q * vec3(1.6, 0.35, 1.6) + 5.0);
    float streak = tNoise(vec3(q.x * 9.0 + q.z * 9.0, q.y * 0.8, 3.0));
    plateCol *= 0.95 + 0.55 * weather;
    plateCol *= 0.88 + 0.24 * streak;
    // Fine pitting and a dusting of grit, so no surface is perfectly clean.
    plateCol *= 0.92 + 0.16 * tNoise(q * 320.0);
    // Young wood is smooth: bark only fissures into plates once a limb is a few
    // centimetres thick. On a twig a centimetre across, plates ten centimetres wide
    // put its whole surface in a furrow, and every twig drew near-black.
    float fissured = smoothstep(0.025, 0.07, R);
    vec3 col = mix(plateCol, creviceCol, clamp((furrow * 0.90 + crack * 0.30) * fissured, 0.0, 1.0));
    // Planted in the ground: soil splashed up the foot, damp and dark where the wood meets
    // the earth, and less light reaching a surface so close to it.
    float foot = 1.0 - smoothstep(-0.2, 0.9, vAbove);
    float splash = (1.0 - smoothstep(0.0, 0.9, vAbove)) * (0.55 + 0.45 * tNoise(q * 14.0));
    vec3 soil = vec3(0.060, 0.043, 0.030);
    col = mix(col, col * 0.45 + soil * 0.9, clamp(splash * 0.9 + foot * 0.7, 0.0, 0.95));
    col *= mix(0.40, 1.0, smoothstep(0.0, 1.8, vAbove));
    col = mix(col, woodCol, wornMask);
    // Lichen and damp in patches, on the shaded faces.
    float lichen = smoothstep(0.66, 0.84, tNoise(q * 4.5 + 9.0));
    col = mix(col, col * vec3(0.78, 1.18, 0.62) + vec3(0.01, 0.02, 0.0), lichen * 0.0);
    diffuseColor.rgb = col;
  } else {
    float vein;
    float inner;
    float cover = treeLeafCoverage(vein, inner);
    vec3 col = vTreeColour.rgb;
    col = mix(col, col * 1.32 + vec3(0.035, 0.045, 0.0), vein * 0.6);
    col *= 0.88 + 0.12 * inner;
    diffuseColor.rgb = col;
    diffuseColor.a = cover;
  }
}
`,Bu=typeof location<"u"?new URLSearchParams(location.search).get("treedebug"):null,ig=`
if (vTreeKind > 0.5 && vTreeKind < 1.5 && !${eg}) {
  // Bump from the bark height (metres), by screen-space derivatives.
  vec3 sx = dFdx(-vViewPosition);
  vec3 sy = dFdy(-vViewPosition);
  float hx = dFdx(tBark);
  float hy = dFdy(tBark);
  vec3 r1 = cross(sy, normal);
  vec3 r2 = cross(normal, sx);
  float det = dot(sx, r1);
  vec3 grad = sign(det) * (hx * r1 + hy * r2);
  normal = normalize(abs(det) * normal - grad);
}
`,rg=`
${Bu==="normal"?"if (vTreeKind > 0.5) { totalEmissiveRadiance = normal * 0.5 + 0.5; diffuseColor.rgb = vec3(0.0); }":""}
${Bu==="albedo"?"if (vTreeKind > 0.5) { totalEmissiveRadiance = diffuseColor.rgb; diffuseColor.rgb = vec3(0.0); }":""}
${Bu==="sun"?"if (vTreeKind > 0.5) { totalEmissiveRadiance = vec3(max(dot(normal, directionalLights[0].direction), 0.0)); diffuseColor.rgb = vec3(0.0); }":""}
${Bu==="front"?"if (vTreeKind > 0.5) { totalEmissiveRadiance = gl_FrontFacing ? vec3(0.0,1.0,0.0) : vec3(1.0,0.0,0.0); diffuseColor.rgb = vec3(0.0); }":""}
#if NUM_DIR_LIGHTS > 0
if (vTreeKind > 1.5) {
  // Light through a leaf. Chlorophyll absorbs red and blue and passes yellow-green, so
  // what comes out the far side is brighter than the leaf's own reflected colour, and
  // yellower. It needs the sun behind the leaf (seen from the front), and it is strongest
  // when we look toward the sun through the foliage. Scaled by the leaf's exposure to
  // the sun, so leaves deep in the crown do not glow.
  vec3 L = directionalLights[0].direction;
  vec3 V = normalize(vViewPosition);
  float back = max(dot(-normal, L), 0.0);
  float toSun = pow(max(dot(-V, L), 0.0), 3.0);
  float t = 0.26 * pow(back, 0.7) + 0.50 * toSun * (0.35 + 0.65 * back);
  float seen = smoothstep(0.08, 0.60, vTreeColour.a);
  vec3 a = max(diffuseColor.rgb, vec3(1e-3));
  float m = max(max(a.r, a.g), a.b);
  vec3 hue = a / m;
  // A leaf transmits about what it reflects in the green (5 - 10 %), and a
  // quarter of that in the blue; at twice that, every back-lit leaf glowed lime.
  vec3 through = pow(hue, vec3(0.80, 0.42, 1.6)) * vec3(0.85, 1.0, 0.5);
  totalEmissiveRadiance += through * (m * 0.9 + 0.006) * directionalLights[0].color * t * seen;
}
#endif
`;class cg{uniform;data;capacity;free=[];constructor(l=16){this.capacity=l*Ko,this.data=new Float32Array(Ko*hi*l*4);const r=new ma(this.data,Ko*hi,l,$n,V1);r.minFilter=Ts,r.magFilter=Ts,r.generateMipmaps=!1,r.needsUpdate=!0,this.uniform={value:r},this.free.push([0,this.capacity])}allocate(l){for(;;){const r=this.free.findIndex(([,u])=>u>=l);if(r>=0){const[u,i]=this.free[r];return i===l?this.free.splice(r,1):this.free[r]=[u+l,i-l],u}this.grow(Math.max(l,this.capacity))}}release(l,r){this.free.push([l,r]),this.free.sort((i,f)=>i[0]-f[0]);const u=[];for(const i of this.free){const f=u[u.length-1];f&&f[0]+f[1]===i[0]?f[1]+=i[1]:u.push([i[0],i[1]])}this.free=u}grow(l){const r=Math.ceil((this.capacity+l)/Ko),u=new Float32Array(Ko*hi*r*4);u.set(this.data);const i=this.capacity;this.data=u,this.capacity=r*Ko;const f=this.uniform.value;f.dispose(),f.image={data:u,width:Ko*hi,height:r},f.needsUpdate=!0,this.release(i,this.capacity-i)}put(l,r){const u=((l>>8)*Ko*hi+(l&Ko-1)*hi)*4;r.forEach((i,f)=>this.data.set(i,u+f*4)),this.uniform.value.needsUpdate=!0}}const Qu=new cg;function ug(o){let l=o.replace("#include <common>",`#include <common>
${ag}`);return l.includes("#include <beginnormal_vertex>")&&(l=l.replace("#include <beginnormal_vertex>",ng)),l.replace("#include <begin_vertex>",og)}function xc(o,l){const r=o.onBeforeCompile,u=`tree-${l}`,i=o.customProgramCacheKey?.bind(o);o.customProgramCacheKey=()=>`${i?i():""}${u}`,o.onBeforeCompile=(f,h)=>{r?.(f,h),f.uniforms.uTreeTable=Qu.uniform,f.vertexShader=ug(f.vertexShader);let v=f.fragmentShader;v=v.replace("void main() {",`${lg}
void main() {`),l==="lit"?(v=v.replace("#include <color_fragment>",`#include <color_fragment>
${sg}`),v=v.replace("#include <normal_fragment_maps>",`#include <normal_fragment_maps>
${ig}`),v=v.replace("#include <emissivemap_fragment>",`#include <emissivemap_fragment>
${rg}`)):l==="depth"?v=v.replace("#include <alphamap_fragment>",`#include <alphamap_fragment>
if (vTreeKind > 1.5) { float tv; float ti; diffuseColor.a = treeLeafCoverage(tv, ti); }`):v=v.replace("#include <normal_fragment_begin>",`#include <normal_fragment_begin>
if (vTreeKind > 1.5) { float tv; float ti; if (treeLeafCoverage(tv, ti) < 0.5) discard; }`),f.fragmentShader=v},o.needsUpdate=!0}function fg(){const o=new Ke({color:16777215,roughness:1,metalness:0,envMapIntensity:.1});xc(o,"lit");const l=new Ke({color:16777215,roughness:.62,metalness:0,side:ct,alphaTest:.5,alphaToCoverage:!1});xc(l,"lit");const r=new Im({depthPacking:Jm});xc(r,"depth");const u=new Im({depthPacking:Jm,alphaTest:.5});return u.side=ct,xc(u,"depth"),{wood:o,leaf:l,woodDepth:r,leafDepth:u,dispose(){o.dispose(),l.dispose(),r.dispose(),u.dispose()}}}let Hu=null;function hg(){if(Hu)return Hu;const o=64,l=document.createElement("canvas");l.width=o,l.height=o;const r=l.getContext("2d");if(r){const i=r.createRadialGradient(o/2,o/2,0,o/2,o/2,o/2);i.addColorStop(0,"rgba(255,255,255,0.95)"),i.addColorStop(.35,"rgba(255,255,255,0.55)"),i.addColorStop(1,"rgba(255,255,255,0)"),r.fillStyle=i,r.fillRect(0,0,o,o)}const u=new hh(l);return Hu=new Dn({map:u,color:723205,transparent:!0,opacity:.6,depthWrite:!1,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),Hu}const dg=[16,10,7,5];function mg(o){const l=o+1,r=new Float32Array(l*2*3),u=new Float32Array(l*2*3),i=new Float32Array(l*2).fill(1);for(let v=0;v<l;v+=1){const m=v/o*Math.PI*2,p=Math.cos(m),b=Math.sin(m);for(let S=0;S<2;S+=1){const C=(v*2+S)*3;r[C]=p,r[C+1]=S,r[C+2]=b,u[C]=p,u[C+2]=b}}const f=[];for(let v=0;v<o;v+=1){const m=v*2,p=m+1,b=m+2,S=m+3;f.push(m,b,p,p,b,S)}const h=new K1;return h.setAttribute("position",new ya(r,3)),h.setAttribute("normal",new ya(u,3)),h.setAttribute("aTreeKind",new ya(i,1)),h.setIndex(f),h}function pg(o){const l=o===0?2:1,r=o===0||o===1?2:1,u=[],i=[],f=[];for(let p=0;p<=l;p+=1)for(let b=0;b<=r;b+=1)u.push(b/r*2-1,p/l,0),i.push(0,0,1),f.push(2);const h=[],v=r+1;for(let p=0;p<l;p+=1)for(let b=0;b<r;b+=1){const S=p*v+b;h.push(S,S+1,S+v,S+1,S+v+1,S+v)}const m=new K1;return m.setAttribute("position",new st(u,3)),m.setAttribute("normal",new st(i,3)),m.setAttribute("aTreeKind",new st(f,1)),m.setIndex(h),m}class vg{constructor(l,r,u={},i=null){this.grower=l,this.pool=i,this.trees=r,this.lodDistances=u.lodDistancesM??[22,60,160],this.farM=u.farM??1500,this.season=u.season??.5,this.rowBase=Qu.allocate(r.length),this.materials=fg(),this.rowDone=new Uint8Array(r.length),this.group.name="unique-forest";const f=u.chunkM??64,h=new Map;r.forEach((v,m)=>{const p=Math.floor(v.x/f),b=Math.floor(v.z/f),S=`${p},${b}`;let C=h.get(S);C||(C={key:S,trees:[],min:new rt(1/0,1/0,1/0),max:new rt(-1/0,-1/0,-1/0),centre:new Et,lod:-1,wanted:-1,distance:1/0,wood:null,thin:null,leaf:null,decal:null,job:null},h.set(S,C),this.chunks.push(C)),C.trees.push(m);const x=v.height*.7;C.min.min(new rt(v.x-x,v.y,v.z-x)),C.max.max(new rt(v.x+x,v.y+v.height*1.05,v.z+x))});for(const v of this.chunks)v.centre.set((v.min.x+v.max.x)/2,(v.min.z+v.max.z)/2);this.stats.chunks=this.chunks.length}group=new ka;chunks=[];trees;rowBase;materials;rowDone;lodDistances;farM;season;casting=!0;generation=0;stats={chunks:0,built:0,trees:0,segments:0,leaves:0,growMs:0,jobs:0};get pending(){let l=0;for(const r of this.chunks)r.wanted>=0&&r.lod!==r.wanted&&(l+=1);return l}get treeCount(){return this.trees.length}setSeason(l){if(!(Math.abs(l-this.season)<1e-4)){this.season=l,this.generation+=1,this.rowDone.fill(0);for(const r of this.chunks)r.job=null,r.lod=-1}}setCasting(l){if(l!==this.casting){this.casting=l;for(const r of this.chunks)this.applyCasting(r)}}applyCasting(l){const r=this.casting&&l.lod>=0&&l.lod<=2;l.wood&&(l.wood.castShadow=r),l.thin&&(l.thin.castShadow=!1),l.leaf&&(l.leaf.castShadow=r)}wantedLevel(l,r){if(l>this.farM)return-2;const[u,i,f]=this.lodDistances,h=v=>r>=0&&r<=v?1.1:.92;return l<u*h(0)?0:l<i*h(1)?1:l<f*h(2)?2:3}update(l,r){const u=performance.now();let i=!1;const f=[];for(const h of this.chunks){const v=Math.max(h.min.x-l.x,0,l.x-h.max.x),m=Math.max(h.min.y-l.y,0,l.y-h.max.y),p=Math.max(h.min.z-l.z,0,l.z-h.max.z);if(h.distance=Math.sqrt(v*v+m*m+p*p),h.wanted=this.wantedLevel(h.distance,h.lod),h.wanted===-2){h.job=null,(h.wood?.visible||h.leaf?.visible)&&(this.hide(h),i=!0);continue}h.lod!==h.wanted?(h.job&&h.job.lod!==h.wanted&&(h.job=null),f.push(h)):h.wood&&!h.wood.visible&&(h.wood.visible=!0,h.thin&&(h.thin.visible=!0),h.leaf&&(h.leaf.visible=!0),i=!0)}this.stats.jobs=f.length,f.sort((h,v)=>h.distance-v.distance);for(const h of f){if(performance.now()-u>=r)break;h.job??={chunk:h,lod:h.wanted,next:0,grown:[],requested:0,arrived:0,generation:this.generation,failed:!1},this.advance(h.job,u+r)&&(i=!0)}return i}hide(l){l.wood&&(l.wood.visible=!1),l.thin&&(l.thin.visible=!1),l.leaf&&(l.leaf.visible=!1)}advance(l,r){if(this.pool)return this.advanceAsync(l);const{chunk:u}=l,i=this.generation;for(;l.next<u.trees.length;){const f=u.trees[l.next],h=this.trees[f],v=performance.now(),m=this.grower.grow({species:h.species,seed:h.seed,height:h.height,openness:h.openness,age:h.age,season:this.season,health:h.health,lift:h.lift},l.lod);if(this.stats.growMs+=performance.now()-v,this.rowDone[f]||this.writeRow(f,m),l.grown.push(m),l.next+=1,performance.now()>=r&&l.next<u.trees.length)return!1}return i!==this.generation?!1:(this.finish(l),!0)}advanceAsync(l){const{chunk:r}=l,u=this.pool;for(;l.requested<r.trees.length&&u.inFlight<u.capacity;){const i=l.requested;l.requested+=1;const f=this.trees[r.trees[i]],h=performance.now();u.grow({species:f.species,seed:f.seed,height:f.height,openness:f.openness,age:f.age,season:this.season,health:f.health,lift:f.lift},l.lod).then(v=>{this.stats.growMs+=performance.now()-h,l.grown[i]=v,l.arrived+=1}).catch(()=>{l.failed=!0,l.arrived+=1})}return l.arrived<r.trees.length?!1:l.failed||l.generation!==this.generation||r.job!==l?(r.job=null,!1):(r.trees.forEach((i,f)=>{this.rowDone[i]||this.writeRow(i,l.grown[f])}),this.finish(l),!0)}writeRow(l,r){const u=i=>[...i];Qu.put(this.rowBase+l,[[...u(r.foliage),r.leafForm],[...u(r.autumnColour),r.autumn],[...u(r.bloomColour),r.bloom],[...u(r.bark).map(i=>i*1.3),r.fissure],[r.flush,r.leafAspect,r.cover,r.crownRadius],[this.trees[l].x,this.trees[l].y+r.crownBase+.5*(r.height-r.crownBase),this.trees[l].z,.5*(r.height-r.crownBase)],[this.trees[l].x,this.trees[l].y,this.trees[l].z,r.crownBase]]),this.rowDone[l]=1}finish(l){const{chunk:r,lod:u,grown:i}=l;let f=0,h=0;for(const j of i)f+=j.segmentCount,h+=j.leafCount;const v=new Float32Array(f*4),m=new Float32Array(f*4),p=new Uint16Array(f),b=new Float32Array(h*4),S=new Int8Array(h*4),C=new Int8Array(h*4),x=new Uint8Array(h*4),D=new Uint8Array(h*4),H=new Uint16Array(h);let q=0,P=0;i.forEach((j,me)=>{const X=r.trees[me],we=this.trees[X],{x:E,y:Z,z:T}=we;for(let F=0;F<j.segmentCount;F+=1){const ce=F*8,ue=(q+F)*4;v[ue]=j.segments[ce]+E,v[ue+1]=j.segments[ce+1]+Z,v[ue+2]=j.segments[ce+2]+T,v[ue+3]=j.segments[ce+3],m[ue]=j.segments[ce+4]+E,m[ue+1]=j.segments[ce+5]+Z,m[ue+2]=j.segments[ce+6]+T,m[ue+3]=j.segments[ce+7],p[q+F]=this.rowBase+X}q+=j.segmentCount;for(let F=0;F<j.leafCount;F+=1){const ce=F*4,ue=(P+F)*4;b[ue]=j.leafPlacement[ce]+E,b[ue+1]=j.leafPlacement[ce+1]+Z,b[ue+2]=j.leafPlacement[ce+2]+T,b[ue+3]=j.leafPlacement[ce+3],H[P+F]=this.rowBase+X}S.set(j.leafDirection,P*4),C.set(j.leafNormal,P*4),x.set(j.leafShape,P*4),D.set(j.leafTint,P*4),P+=j.leafCount});const K=new Nv;new fh(r.min,r.max).getBoundingSphere(K),K.radius+=2;const U=[],z=[];for(let j=0;j<f;j+=1)(Math.max(v[j*4+3],m[j*4+3])<.02?z:U).push(j);const R=(j,me)=>{const X=new Float32Array(j.length*4),we=new Float32Array(j.length*4),E=new Uint16Array(j.length);j.forEach((T,F)=>{X.set(v.subarray(T*4,T*4+4),F*4),we.set(m.subarray(T*4,T*4+4),F*4),E[F]=p[T]});const Z=mg(me);return Z.setAttribute("aSegA",new _l(X,4)),Z.setAttribute("aSegB",new _l(we,4)),Z.setAttribute("aTreeId",new _l(E,1)),Z.instanceCount=j.length,Z.boundingSphere=K.clone(),Z},V=R(U,dg[u]),J=R(z,3),Q=pg(u);Q.setAttribute("aLeafPos",new _l(b,4)),Q.setAttribute("aLeafDir",new _l(S,4,!0)),Q.setAttribute("aLeafNor",new _l(C,4,!0)),Q.setAttribute("aLeafShape",new _l(x,4,!0)),Q.setAttribute("aLeafTint",new _l(D,4,!0)),Q.setAttribute("aTreeId",new _l(H,1)),Q.instanceCount=h,Q.boundingSphere=K.clone();const ne=new xt(V,this.materials.wood);ne.customDepthMaterial=this.materials.woodDepth,ne.receiveShadow=!0,ne.name=`forest/${r.key}/wood`;const B=new xt(J,this.materials.wood);B.receiveShadow=!1,B.name=`forest/${r.key}/twigs`;const ae=new xt(Q,this.materials.leaf);ae.customDepthMaterial=this.materials.leafDepth,ae.receiveShadow=!0,ae.name=`forest/${r.key}/leaf`;const I=[],ge=[];r.trees.forEach((j,me)=>{const X=this.trees[j],we=Math.max(1.6,i[me].trunkRadius*7+.8),E=X.y+.03;I.push(X.x-we,E,X.z-we,X.x+we,E,X.z-we,X.x+we,E,X.z+we,X.x-we,E,X.z+we),ge.push(0,0,1,0,1,1,0,1)});const Re=new sa;Re.setAttribute("position",new st(I,3)),Re.setAttribute("uv",new st(ge,2));const le=[];for(let j=0;j<r.trees.length;j+=1)le.push(j*4,j*4+2,j*4+1,j*4,j*4+3,j*4+2);Re.setIndex(le);const G=new xt(Re,hg());G.renderOrder=1,G.frustumCulled=!1,G.name=`forest/${r.key}/contact`,G.userData.noOcclusion=!0,this.release(r),r.decal=G,this.group.add(G),r.wood=ne,r.thin=B,this.group.add(B),r.leaf=ae,r.lod=u,r.job=null,this.group.add(ne,ae),this.applyCasting(r),this.recount()}release(l){for(const r of[l.wood,l.thin,l.leaf])r&&(r.removeFromParent(),r.geometry.dispose());l.wood=null,l.thin=null,l.leaf=null,l.decal&&(l.decal.removeFromParent(),l.decal.geometry.dispose(),l.decal=null)}recount(){let l=0,r=0,u=0,i=0;for(const f of this.chunks)!f.wood||!f.leaf||(l+=1,r+=f.trees.length,u+=f.wood.geometry.instanceCount+(f.thin?f.thin.geometry.instanceCount:0),i+=f.leaf.geometry.instanceCount);Object.assign(this.stats,{built:l,trees:r,segments:u,leaves:i})}dispose(){for(const l of this.chunks)this.release(l);this.materials.dispose(),Qu.release(this.rowBase,this.trees.length),this.group.removeFromParent()}}let v1=null,op=null;function gg(o){return v1??=vh.load(o).then(l=>(op=l,l)).catch(l=>(console.warn("tree grower unavailable:",l),null)),v1}function yg(){return op}function xg(...o){let l=2654435769,r=2246822507;for(const u of o){const i=Math.round(u*8)|0;l=Math.imul(l^i,2246822507),l^=l>>>13,r=Math.imul(r+l+668265263,3266489909),r^=r>>>16}return l=Math.imul(l^l>>>15,739982445),r=Math.imul(r^r>>>12,695872825),(r>>>0&2097151)*4294967296+(l>>>0)}let Vf;function wg(){return Vf===void 0&&(Vf=new URLSearchParams(location.search).get("workers")==="0"?null:gh.create()),Vf}function Kf(o,l){return(Math.sin(o*12.9898+l*78.233)*43758.5453%1+1)%1}function bg(o,l,r){const u=[],i=[],f=[],h=new Set,v=new Map;for(const p of o){if(!p.key.startsWith("tree/"))continue;i.push(p.mesh),p.far&&i.push(p.far),f.push(p);const b=p.mesh.geometry;b.boundingBox||b.computeBoundingBox();const S=b.boundingBox?.max.y??0;v.set(p.key,Math.max(v.get(p.key)??0,S))}const m=Number(r)%1000003;for(const p of f){if(h.has(p.key))continue;h.add(p.key);const b=p.key.split("/")[1],S=l.speciesIndex(b),C=S>=0?S:Math.max(0,l.speciesIndex("xiang-zhang")),x=v.get(p.key)||10,D=p.matrices;for(let H=0;H<p.count;H+=1){const q=H*16,P=Math.hypot(D[q+4],D[q+5],D[q+6]),K=D[q+12],U=D[q+13],z=D[q+14],R=xg(m,K,z,C);u.push({x:K,y:U,z,species:C,height:Math.max(2,x*P),seed:R,openness:.55+.4*Kf(K,z),age:.55+.45*Kf(z,K+3.1),health:.85+.15*Kf(K+1.7,z-.4),lift:0})}}return{trees:u,replaced:i,sets:f}}function Mg(o,l){const r=yg();if(!r)return null;const u=o.lodSets.length>0?o.lodSets:[...o.instanced.entries()].map(([m,p])=>({mesh:p,key:m,count:p.count,matrices:p.instanceMatrix.array.slice(0,p.count*16),far:null})),i=bg(u,r,l.seed);if(i.trees.length===0)return null;const f=new vg(r,i.trees,{},wg());for(const m of i.replaced)m.removeFromParent();const h=new Set(i.sets);o.lodSets=o.lodSets.filter(m=>!h.has(m));const v=[];o.group.traverse(m=>{m.isInstancedMesh&&m.name.startsWith("tree/")&&v.push(m)});for(const m of v)m.removeFromParent();for(const m of[...o.instanced.keys()])m.startsWith("tree/")&&o.instanced.delete(m);return f}const Wo={yard:4,parking:5,asphaltLot:6,plaza:7,lawn:8},lp=`
float gHash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
float gNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(gHash(i), gHash(i + vec2(1.0, 0.0)), f.x),
             mix(gHash(i + vec2(0.0, 1.0)), gHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
// Coverage of a line w metres wide every period metres, fading to its mean
// share once the line is under a pixel.
float gLine(float c, float period, float w, float fw) {
  float d = abs(fract(c / period + 0.5) - 0.5) * period;
  float cov = clamp((w * 0.5 + fw * 0.5 - d) / max(fw, 1e-4), 0.0, 1.0);
  float mean = min(1.0, w / period);
  return mix(cov, mean, smoothstep(0.3 * period, 0.7 * period, fw));
}
float gBox(vec2 p, vec2 lo, vec2 hi) {
  vec2 s = step(lo, p) * step(p, hi);
  return s.x * s.y;
}
vec3 gContainer(float h) {
  if (h < 0.18) return vec3(0.46, 0.17, 0.12);
  if (h < 0.34) return vec3(0.12, 0.22, 0.38);
  if (h < 0.48) return vec3(0.16, 0.33, 0.24);
  if (h < 0.60) return vec3(0.62, 0.36, 0.10);
  if (h < 0.74) return vec3(0.50, 0.52, 0.52);
  if (h < 0.86) return vec3(0.40, 0.30, 0.20);
  return vec3(0.70, 0.68, 0.62);
}
vec3 groundSurface(float kind, vec3 base, vec2 p, float fw, float seed) {
  vec2 q = p + seed * 97.0;
  float low = gNoise(q / 31.0);
  float mid = gNoise(q / 6.3 + 7.0);
  float fine = gNoise(q / 0.45);
  float grain = 1.0 - smoothstep(0.15, 0.6, fw);
  vec3 c = base * (0.88 + 0.24 * low) * (0.94 + 0.12 * mid);
  c *= mix(1.0, 0.92 + 0.16 * fine, grain);
  if (kind < 4.5) {
    // Concrete yard: slab tone, joints, wheel wear, oil, container stacks.
    vec2 cell = floor(q / 5.0);
    float tone = 0.92 + 0.16 * gHash(cell + 3.0);
    float joint = max(gLine(q.x, 5.0, 0.03, fw), gLine(q.y, 5.0, 0.03, fw));
    c *= tone * (1.0 - 0.5 * joint);
    float lane = smoothstep(0.35, 0.5, gNoise(vec2(q.x / 55.0, q.y / 2.2)));
    c *= 1.0 - 0.10 * lane * (1.0 - smoothstep(0.5, 3.0, fw));
    float oil = smoothstep(0.62, 0.9, gNoise(q / 2.6 + 11.0));
    c *= 1.0 - 0.14 * oil * (1.0 - smoothstep(0.8, 3.0, fw));
    vec2 blockSize = vec2(42.0, 30.0);
    vec2 block = floor(q / blockSize);
    vec2 local = q - block * blockSize;
    float stacked = step(gHash(block + seed * 13.0), 0.33);
    if (stacked > 0.5) {
      vec2 rel = vec2((local.x - 8.0) / 12.19, (local.y - 7.0) / 2.59);
      float inside = gBox(rel, vec2(0.0, 0.0), vec2(2.0, 6.0));
      vec2 box = floor(rel);
      float h = gHash(box + block * 7.0);
      vec2 inBox = fract(rel);
      float gap = 1.0 - gBox(inBox, vec2(0.012, 0.03), vec2(0.988, 0.97));
      vec3 box3 = gContainer(h) * (0.85 + 0.3 * gHash(box + 5.0));
      // Corrugation ribs along the side, 0.26 m pitch.
      box3 *= 1.0 - 0.10 * grain * step(0.5, fract(q.y / 0.26));
      box3 *= 1.0 - 0.25 * gap;
      float showBoxes = 1.0 - smoothstep(0.4, 2.4, fw);
      vec3 meanC = mix(c, vec3(0.42, 0.34, 0.28), 0.09);
      c = mix(meanC, mix(c, box3, inside), showBoxes);
    }
  } else if (kind < 5.5) {
    // Car park: 16.6 m double-loaded module along z, 2.5 m bays along x.
    float v = mod(q.y, 16.6);
    float inStall = 1.0 - step(5.3, v) * (1.0 - step(11.3, v));
    float depth = v < 5.3 ? v : 16.6 - v;
    float bayX = q.x / 2.5;
    float paint = gLine(q.x, 2.5, 0.1, fw) * inStall;
    float endA = 1.0 - smoothstep(0.05, 0.05 + max(fw, 0.02), abs(v - 5.3));
    float endB = 1.0 - smoothstep(0.05, 0.05 + max(fw, 0.02), abs(v - 11.3));
    paint = max(paint, max(endA, endB) * 0.9);
    c *= 1.0 - 0.10 * (1.0 - inStall) * gNoise(vec2(q.x / 9.0, q.y / 2.0));
    float row = floor(q.y / 16.6) + (v > 8.3 ? 0.5 : 0.0);
    float oil = smoothstep(0.62, 0.78, gNoise(vec2(floor(bayX), row) * 3.1 + q / 1.6)) * inStall;
    c *= 1.0 - 0.28 * oil * (1.0 - smoothstep(0.5, 3.0, fw));
    float sid = gHash(vec2(floor(bayX), row) + seed * 5.0);
    float occupied = step(sid, 0.55) * inStall;
    float across = fract(bayX);
    float car = occupied * gBox(vec2(across, depth), vec2(0.14, 0.45), vec2(0.86, 4.85));
    float tint = gHash(vec2(floor(bayX), row) + 9.0);
    vec3 paintC = tint < 0.30 ? vec3(0.62) : tint < 0.50 ? vec3(0.05) : tint < 0.72 ? vec3(0.32, 0.33, 0.35) : tint < 0.84 ? vec3(0.42, 0.10, 0.08) : vec3(0.10, 0.16, 0.30);
    float glass = gBox(vec2(across, depth), vec2(0.2, 1.3), vec2(0.8, 3.6));
    vec3 carC = mix(paintC, vec3(0.05, 0.06, 0.07), glass * 0.65);
    float showCar = 1.0 - smoothstep(0.5, 2.2, fw);
    vec3 meanCar = mix(c, vec3(0.32), 0.19 * inStall);
    c = mix(meanCar, mix(c, carC, car), showCar);
    c = mix(c, vec3(0.78, 0.77, 0.70), paint * 0.8);
  } else if (kind < 6.5) {
    // Asphalt lot: oil, patches, cracks, painted lane lines.
    // Oil drips collect in rounded stains, soft at the edge, darkest at the core.
    float oil = smoothstep(0.58, 0.95, gNoise(q / 3.1 + 31.0)) * (0.6 + 0.4 * gNoise(q / 0.5));
    c *= 1.0 - 0.20 * oil * (1.0 - smoothstep(0.8, 4.0, fw));
    vec2 pc = floor(q / 14.0);
    float repair = step(gHash(pc + 41.0), 0.14) * gBox(fract(q / 14.0), vec2(0.15, 0.25), vec2(0.85, 0.7));
    c *= 1.0 - 0.18 * repair;
    float crack = smoothstep(0.46, 0.5, gNoise(vec2(q.x / 0.9, q.y / 14.0) + 21.0));
    c *= 1.0 - 0.06 * crack * grain;
    float lane = max(gLine(q.x + 4.0, 28.0, 0.12, fw), gLine(q.y + 9.0, 28.0, 0.12, fw)) * step(0.35, gHash(floor(q / 28.0) + seed));
    c = mix(c, vec3(0.62, 0.52, 0.18), lane * 0.55);
  } else if (kind < 7.5) {
    // Plaza: granite pavers, expansion joints, planting beds.
    vec2 pv = floor(q / 0.6);
    float paver = 0.95 + 0.10 * gHash(pv);
    float joints = max(gLine(q.x, 0.6, 0.012, fw), gLine(q.y, 0.6, 0.012, fw));
    float expansion = max(gLine(q.x, 6.0, 0.04, fw), gLine(q.y, 6.0, 0.04, fw));
    c *= paver * (1.0 - 0.18 * joints) * (1.0 - 0.28 * expansion);
    vec2 block = floor(q / vec2(34.0, 26.0));
    vec2 local = q - block * vec2(34.0, 26.0);
    float bed = step(gHash(block + seed * 3.0), 0.30) * gBox(local, vec2(6.0, 9.0), vec2(26.0, 14.0));
    vec3 green = vec3(0.17, 0.27, 0.11) * (0.8 + 0.4 * gNoise(q / 0.7));
    c = mix(c, green, bed);
  } else {
    // Lawn: mown bands and patchiness.
    float band = step(0.5, fract(q.x / 6.0));
    c *= 1.0 + 0.05 * (band - 0.5) * (1.0 - smoothstep(1.0, 4.0, fw));
    c *= 0.9 + 0.2 * gNoise(q / 2.4 + 3.0);
  }
  return c;
}
`;function Yf(o,l){const r=o.onBeforeCompile;return o.onBeforeCompile=(u,i)=>{r?.call(o,u,i),u.vertexShader=u.vertexShader.replace("#include <common>",`#include <common>
varying vec2 vGroundP;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vGroundP = position.xz;`),u.fragmentShader=u.fragmentShader.replace("#include <common>",`#include <common>
varying vec2 vGroundP;
`+lp).replace("#include <color_fragment>",`#include <color_fragment>
        diffuseColor.rgb = groundSurface(${l.toFixed(1)}, diffuseColor.rgb, vGroundP, length(fwidth(vGroundP)), 0.0);`)},o.customProgramCacheKey=()=>`ground-${l}`,o}const Sg=`
varying vec3 vBarkPos;
`,Tg=`
varying vec3 vBarkPos;
float bkHash(vec2 p) {
  vec3 q = fract(vec3(p.xyx) * vec3(0.1031, 0.1030, 0.0973));
  q += dot(q, q.yxz + 33.33);
  return fract((q.x + q.y) * q.z);
}
float bkNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(bkHash(i), bkHash(i + vec2(1.0, 0.0)), f.x),
             mix(bkHash(i + vec2(0.0, 1.0)), bkHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
`,zg=`
#include <color_fragment>
{
  // Round the trunk by arc length, up it by height.
  float r = max(length(vBarkPos.xz), 0.04);
  vec2 q = vec2(atan(vBarkPos.x, vBarkPos.z) * r, vBarkPos.y);
  float fw = max(length(fwidth(q)), 1e-4);
  // Fissures: ridged noise stretched along the trunk, one plate every ~6 cm.
  float plate = bkNoise(q * vec2(16.0, 2.6) + 3.7);
  float ridge = 1.0 - abs(plate * 2.0 - 1.0);
  float fissure = smoothstep(0.55, 0.95, ridge);
  float crack = bkNoise(q * vec2(34.0, 15.0) + 11.0);
  float detail = 1.0 - smoothstep(0.02, 0.12, fw);
  float coarse = 1.0 - smoothstep(0.004, 0.02, fw);
  float shade = 1.0 - 0.42 * fissure * detail - 0.12 * (crack - 0.5) * detail;
  // Pale lichen and damp: slow patches over the whole trunk.
  float patch = bkNoise(q * vec2(2.2, 1.1) + 29.0);
  vec3 lichen = vec3(1.12, 1.14, 1.0);
  vec3 damp = vec3(0.82, 0.80, 0.78);
  vec3 tone = mix(vec3(1.0), lichen, smoothstep(0.62, 0.85, patch) * 0.5);
  tone = mix(tone, damp, smoothstep(0.30, 0.12, patch) * 0.45);
  diffuseColor.rgb *= mix(1.0, shade, 0.55 + 0.45 * coarse) * tone;
}
`;function Cg(o){const l=o.onBeforeCompile;o.onBeforeCompile=(u,i)=>{l?.call(o,u,i),u.vertexShader=u.vertexShader.replace("#include <common>",`#include <common>
${Sg}`).replace("#include <begin_vertex>",`#include <begin_vertex>
vBarkPos = position;`),u.fragmentShader=u.fragmentShader.replace("#include <common>",`#include <common>
${Tg}`).replace("#include <color_fragment>",zg)};const r=o.customProgramCacheKey.bind(o);return o.customProgramCacheKey=()=>`${r()}|bark`,o.needsUpdate=!0,o}const _g=`
float atHash(vec2 p) { p = fract(p * vec2(123.34, 456.21)); p += dot(p, p + 45.32); return fract(p.x * p.y); }
float atNoise(vec2 p) {
  vec2 i = floor(p), f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(atHash(i), atHash(i + vec2(1.0, 0.0)), f.x), mix(atHash(i + vec2(0.0, 1.0)), atHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
// Two samples of the same texture, the second rotated and rescaled, blended by
// noise so that neighbouring tiles differ. \`restore\` puts back the contrast the
// blend loses (it peaks where both samples are equally weighted).
vec4 antiTileSample(sampler2D t, vec2 uv, float restore) {
  vec4 a = texture2D(t, uv);
#ifdef AT_ALIGNED
  // Laid paving has its joints on one grid that runs with the kerb: the second
  // sample is the same grid moved by whole slabs, never turned or rescaled,
  // or the blend shows a second set of joints at an angle.
  vec4 b = texture2D(t, uv + vec2(0.25, 0.5));
#else
  vec4 b = texture2D(t, mat2(0.8, -0.6, 0.6, 0.8) * uv * 0.61 + vec2(0.37, 0.11));
#endif
  float w = smoothstep(0.30, 0.70, atNoise(uv * 0.17 + 3.1));
  vec4 c = mix(a, b, w);
  // The contrast is restored around the texture's own mean (its top mip level),
  // not around mid grey: ground textures are dark, and stretching them about 0.5
  // would push their colours out of range.
  vec4 mean = textureLod(t, vec2(0.5), 20.0);
  float boost = 1.0 + restore * (1.0 - abs(2.0 * w - 1.0));
  return vec4(max(mean.rgb + (c.rgb - mean.rgb) * boost, vec3(0.0)), c.a);
}
`,Ag={asphalt:`
vec3 antiTone(vec2 uv) {
  // uv is in 4 m tiles; world metres = uv * 4.
  vec2 m = uv * 4.0;
  float lo = atNoise(uv * 0.06 + 9.0);
  float mid = atNoise(uv * 0.35);
  float tone = 0.80 + 0.40 * lo + 0.14 * (mid - 0.5);
  // Damp / sun-bleached zones a few tens of metres across: wet-dark patches
  // beside oxidised pale ones (asphalt greys and lightens as the binder weathers).
  float damp = smoothstep(0.58, 0.82, atNoise(uv * 0.11 + 31.0));
  float bleach = smoothstep(0.62, 0.9, atNoise(uv * 0.07 + 57.0));
  tone *= 1.0 - 0.20 * damp + 0.12 * bleach;
  // Patched repairs: straight-edged rectangles of newer, blacker surface.
  vec2 cell = floor(uv * 0.5);
  vec2 f = fract(uv * 0.5);
  float inside = step(0.15, f.x) * step(f.x, 0.85) * step(0.20, f.y) * step(f.y, 0.70);
  float patched = step(0.90, atHash(cell)) * inside;
  tone *= 1.0 - 0.26 * patched;
  // ...and the old surface just outside a patch is a touch paler, with a
  // hairline of sealant round the cut.
  vec2 edge = min(f - vec2(0.15, 0.20), vec2(0.85, 0.70) - f) * vec2(8.0, 8.0);
  float cut = (1.0 - smoothstep(0.0, 0.035, abs(min(edge.x, edge.y)))) * step(0.90, atHash(cell)) * step(0.0, min(edge.x, edge.y) + 0.035);
  tone *= 1.0 - 0.30 * cut;
  // Sealed cracks: a few long, thin, darker lines (two orientations).
  float crack = 1.0 - smoothstep(0.0, 0.035, abs(atNoise(uv * vec2(0.5, 0.045) + 4.0) - 0.5));
  tone *= 1.0 - 0.22 * crack * step(0.55, atNoise(uv * 0.08));
  float crack2 = 1.0 - smoothstep(0.0, 0.03, abs(atNoise(uv * vec2(0.05, 0.6) + 17.0) - 0.5));
  tone *= 1.0 - 0.20 * crack2 * step(0.6, atNoise(uv * 0.09 + 6.0));
  // Alligator cracking in worn zones: a polygon network, only where it is bleached.
  vec2 g = m * 1.7 + 20.0 * atNoise(m * 0.3);
  float net = abs(atNoise(g) - 0.5);
  tone *= 1.0 - 0.18 * (1.0 - smoothstep(0.0, 0.025, net)) * smoothstep(0.55, 0.8, atNoise(uv * 0.13 + 71.0));
  // Oil drips: dark soft blobs ~0.4-1 m, one in about every few cells, each
  // with a rainbow-free dark centre and a faint halo.
  vec2 oc = floor(m / 9.0);
  vec2 of = fract(m / 9.0);
  vec2 centre = vec2(atHash(oc + 3.7), atHash(oc + 8.1));
  float od = length((of - centre) * 9.0 * vec2(1.0, 1.6));
  float oil = (1.0 - smoothstep(0.15, 0.7 + atHash(oc) * 0.5, od)) * step(0.55, atHash(oc + 1.3));
  tone *= 1.0 - 0.32 * oil;
  // Utility covers: a round iron lid, 0.7 m, one cell in ~18 (cells of 31 m).
  vec2 mc = floor(m / 31.0);
  vec2 mf = fract(m / 31.0);
  vec2 lid = vec2(atHash(mc + 19.3), atHash(mc + 44.7)) * 0.8 + 0.1;
  float ld = length((mf - lid) * 31.0);
  float lidOn = step(0.94, atHash(mc + 5.5));
  float ring = lidOn * (1.0 - smoothstep(0.34, 0.37, ld));
  float rim = lidOn * (1.0 - smoothstep(0.36, 0.40, ld)) * smoothstep(0.30, 0.34, ld);
  tone = mix(tone, 0.62 * (0.85 + 0.3 * atHash(floor(m * 14.0))), ring);
  tone *= 1.0 - 0.45 * rim;
  // A hint of warm/cool: aged binder is neutral to slightly warm, never blue.
  return vec3(tone * 1.01, tone, tone * 0.985);
}`,grass:`
vec3 antiTone(vec2 uv) {
  float lo = atNoise(uv * 0.045 + 2.0);
  float mid = atNoise(uv * 0.22);
  vec3 lush = vec3(0.90, 1.07, 0.86);
  vec3 dry = vec3(1.22, 1.05, 0.66);
  vec3 c = mix(lush, dry, smoothstep(0.45, 0.80, lo));
  float bare = smoothstep(0.76, 0.90, atNoise(uv * 0.09 + 7.0));
  c = mix(c, vec3(1.40, 1.02, 0.72), bare * 0.55);
  // Mottling at a metre or two: thicker and thinner sward, the mower's
  // overlap, a damp hollow. A lawn seen from a few storeys up is never one
  // flat green.
  float mottle = atNoise(uv * 0.8 + 11.0) * 0.6 + atNoise(uv * 2.3 + 5.0) * 0.4;
  return c * (0.76 + 0.30 * mid) * (0.86 + 0.28 * mottle);
}`,paving:`
vec3 antiTone(vec2 uv) {
  // Slab to slab: each 2 m slab is a slightly different batch of concrete.
  float slab = atHash(floor(uv * 2.0));
  float lo = atNoise(uv * 0.07 + 5.0);
  float stain = smoothstep(0.62, 0.85, atNoise(uv * 0.28 + 1.7));
  float tone = (0.90 + 0.18 * slab) * (0.86 + 0.28 * lo) * (1.0 - 0.18 * stain);
  return vec3(tone * 1.01, tone, tone * 0.98);
}`};function Lu(o,l){const r=l==="grass"?.3:.22;return o.onBeforeCompile=u=>{u.fragmentShader=u.fragmentShader.replace("#include <common>",`#include <common>
${l==="paving"?`#define AT_ALIGNED
`:""}${_g}
${Ag[l]}`).replace("#include <map_fragment>",th.map_fragment.replace("texture2D( map, vMapUv )",`antiTileSample( map, vMapUv, ${r.toFixed(2)} )`).replace("diffuseColor *= sampledDiffuseColor;",`diffuseColor *= sampledDiffuseColor;
	diffuseColor.rgb *= antiTone(vMapUv);`)).replace("#include <normal_fragment_maps>",th.normal_fragment_maps.split("texture2D( normalMap, vNormalMapUv )").join("antiTileSample( normalMap, vNormalMapUv, 0.0 )"))},o.customProgramCacheKey=()=>`antitile-${l}`,o.needsUpdate=!0,o}const Eg=`
uniform float uElevTileW;
float elevHash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}
`,Rg=`
float elevGlass = 0.0;
float elevPaint = 0.0;
vec3 elevF0 = vec3(0.04);
vec3 elevEmit = vec3(0.0);
#ifdef USE_MAP
{
  float cls = sampledDiffuseColor.a * 255.0;
  elevPaint = smoothstep(220.0, 250.0, cls);
  elevGlass = 1.0 - smoothstep(128.0, 168.0, cls);
  vec3 tint = vec3(1.0);
  #if defined( USE_COLOR ) || defined( USE_COLOR_ALPHA )
    tint = vColor.rgb;
  #endif
  vec3 solid = mix(sampledDiffuseColor.rgb, sampledDiffuseColor.rgb * tint, elevPaint);

  // Which window this is: its id in the tile, which tile along and up the wall,
  // and which building (its paint colour).
  vec2 tileIx = floor(vMapUv);
  float wid = floor(cls + 0.5);
  vec3 seed = vec3(wid, tileIx.x * 7.13 + tileIx.y * 131.7, dot(tint, vec3(311.0, 173.0, 59.0)));
  float h1 = elevHash(seed);
  float h2 = elevHash(seed + 17.0);
  float h3 = elevHash(seed + 41.0);
  float h4 = elevHash(seed + 73.0);
  float h5 = elevHash(seed + 97.0);
  float u = sampledDiffuseColor.r;
  float v = sampledDiffuseColor.g;
  float widthM = max(sampledDiffuseColor.b * 5.0, 0.4);
  // Metres of wall per pixel.  A window a few pixels across cannot show a
  // curtain's folds or its edge, and drawing them anyway is what makes a
  // distant tower sparkle; past ~6 cm a pixel the interior fades to what a
  // window averages to.
  float mpp = max(length(dFdx(vMapUv)), length(dFdy(vMapUv))) * uElevTileW;
  float detail = 1.0 - smoothstep(0.05, 0.22, mpp);
  float du = max(fwidth(u), 1e-4) * 1.5;

#if defined( ELEV_SHOP )
  // A shop: a pale back wall, shelves of goods in a run of colours, panel
  // lights in the ceiling, and it lights itself — in daylight a shop interior
  // is about as bright as the shaded pavement outside it.  One in eight is
  // shut and dark.
  // Shelf cells 0.45 m wide and a seventh of the glass high.  Their edges are
  // filtered by the pixel footprint, so a shelf does not sparkle at a grazing
  // angle, and a dark divider between cells makes them read as shelving.
  float cellW = u * widthM / 0.45;
  float fu = max(fwidth(cellW), 1e-4);
  float fv = max(fwidth(v), 1e-4);
  // The goods are smooth value noise over the same grid: a hash at each
  // lattice corner, blended with a smoothstep.  A hard cell index flickers from
  // one pixel to the next at a cell edge; this does not.
  vec2 gp = vec2(cellW, v * 7.0);
  vec2 gi = floor(gp);
  vec2 gf = gp - gi;
  gf = gf * gf * (3.0 - 2.0 * gf);
  float n00 = elevHash(seed + gi.x * 3.1 + gi.y * 7.7);
  float n10 = elevHash(seed + (gi.x + 1.0) * 3.1 + gi.y * 7.7);
  float n01 = elevHash(seed + gi.x * 3.1 + (gi.y + 1.0) * 7.7);
  float n11 = elevHash(seed + (gi.x + 1.0) * 3.1 + (gi.y + 1.0) * 7.7);
  float n = mix(mix(n00, n10, gf.x), mix(n01, n11, gf.x), gf.y);
  // Goods seen through glass are mostly neutral: grey and warm, not a rainbow.
  vec3 goods = vec3(0.42 + 0.28 * n, 0.41 + 0.27 * n, 0.38 + 0.26 * n);
  float edgeM = min(fract(cellW), 1.0 - fract(cellW));
  goods *= 1.0 - 0.35 * (1.0 - smoothstep(0.067 - fu, 0.067 + fu, edgeM));
  vec3 backWall = mix(vec3(0.55, 0.52, 0.46), vec3(0.62, 0.62, 0.60), h3);
  float inRows = smoothstep(0.06 - fv, 0.06 + fv, v) * (1.0 - smoothstep(0.74 - fv, 0.74 + fv, v));
  float board = smoothstep(0.16 - fv * 7.0, 0.16 + fv * 7.0, fract(v * 7.0));
  float shelf = inRows * board * step(0.30, h2);
  vec3 inside = mix(backWall * 0.7, goods, shelf);
  inside = mix(inside, vec3(0.85), step(0.92, v) * step(fract(u * widthM / 1.5 + h4), 0.32));
  float lit = step(0.12, h1);
  inside = mix(vec3(0.52, 0.50, 0.46), inside, detail);
  elevEmit = inside * 0.42 * mix(0.88, lit, detail);
  inside = inside * 0.10 + vec3(0.008);
#elif defined( ELEV_OFFICE )
  // An office floor: a dark plate under a ceiling of lit panels, half the
  // floors behind venetian blinds; an opaque spandrel at the slab.
  vec3 inside = vec3(0.022, 0.024, 0.027) * (0.6 + 0.8 * h1);
  inside = mix(inside, vec3(0.09, 0.09, 0.09), step(0.86, v));
  float lamp = step(fract(u * widthM / 1.2 + h3), 0.24) * step(0.90, v) * step(h4, 0.8);
  inside += lamp * vec3(0.50, 0.52, 0.50);
  if (h2 < 0.45) {
    float f = 0.15 + 0.85 * h5;
    float slat = 0.75 + 0.25 * step(0.4, fract(v * 55.0));
    inside = mix(inside, vec3(0.42, 0.42, 0.40) * slat, step(1.0 - f, v));
  }
  if (sampledDiffuseColor.b > 0.97) inside = vec3(0.016, 0.017, 0.018);
  // Coated glass passes well under half the light.
  inside *= 0.42;
  inside = mix(vec3(0.030, 0.031, 0.033), inside, detail);
#else
  // The room: dark, a little warmer and lighter towards the ceiling, where the
  // window lights it.
  vec3 room = vec3(0.034, 0.031, 0.028) * (0.55 + 0.9 * h1) * mix(0.7, 1.25, v);
  vec3 inside = room;
  float foldFreq = widthM / 0.11;
  float folds = 1.0 - 0.14 * (1.0 - smoothstep(0.25, 0.6, fwidth(u) * foldFreq))
    * (0.5 - 0.5 * sin(u * foldFreq * 6.2831));
  if (h2 < 0.32) {
    // No curtain: the room, and now and then a lit lamp or a pale wall.
    inside = room * (h3 < 0.12 ? 3.5 : 1.0);
  } else if (h2 < 0.70) {
    // Sheer white curtains, drawn from one side.
    float f = 0.25 + 0.75 * h3;
    float covered = h4 < 0.5 ? smoothstep(f + du, f - du, u) : smoothstep(1.0 - f - du, 1.0 - f + du, u);
    inside = mix(room, vec3(0.50, 0.49, 0.45) * folds, covered * 0.82);
  } else if (h2 < 0.90) {
    // A pair of heavy curtains, partly open.
    vec3 cloth = h5 < 0.25 ? vec3(0.36, 0.28, 0.18)
      : h5 < 0.45 ? vec3(0.18, 0.21, 0.27)
      : h5 < 0.70 ? vec3(0.46, 0.42, 0.34)
      : h5 < 0.85 ? vec3(0.20, 0.13, 0.09)
      : vec3(0.27, 0.31, 0.25);
    float f = 0.30 + 0.70 * h3;
    float covered = smoothstep(f * 0.5 + du, f * 0.5 - du, u) + smoothstep(1.0 - f * 0.5 - du, 1.0 - f * 0.5 + du, u);
    inside = mix(room, cloth * folds, clamp(covered, 0.0, 1.0));
  } else {
    // A roller blind, lowered part way.
    float f = 0.2 + 0.8 * h3;
    inside = mix(room, vec3(0.46, 0.44, 0.40), step(1.0 - f, v));
  }
  // Two panes of float glass take a third of the light each way.
  inside *= 0.62;
  // What a window averages to: a room with a curtain over part of it.
  inside = mix(vec3(0.075, 0.071, 0.064), inside, detail);
#endif
  diffuseColor.rgb = diffuse * mix(solid, inside, elevGlass);
#ifdef ELEV_TINT_GLASS
  // A curtain wall's vertex colour is its coating, which is what it reflects.
  elevF0 = mix(vec3(0.04), tint, 0.30);
#elif defined( ELEV_OFFICE )
  elevF0 = vec3(0.10, 0.12, 0.13);
#endif
}
#endif
diffuseColor.a = 1.0;
`,Ng=`
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, elevF0, elevGlass);
material.specularColorBlended = mix(material.specularColorBlended, elevF0, elevGlass);
`,Dg=`
#include <emissivemap_fragment>
totalEmissiveRadiance += elevEmit * elevGlass;
`,Ug=`
#include <roughnessmap_fragment>
roughnessFactor = mix(mix(0.62, 0.90, elevPaint), 0.045, elevGlass);
`,Og=`
#include <normal_fragment_maps>
#ifdef USE_MAP
if (elevGlass > 0.0) {
  // Every pane is glazed a fraction of a degree off true, which is what makes
  // the sky in one window a different value from the sky in the next.
  float panes = max(1.0, floor(max(sampledDiffuseColor.b * 5.0, 0.4) / 0.85 + 0.5));
  float pane = floor(sampledDiffuseColor.r * panes);
  vec3 ps = vec3(floor(sampledDiffuseColor.a * 255.0 + 0.5), pane, floor(vMapUv.x) * 3.1 + floor(vMapUv.y) * 17.3);
  vec2 tilt = vec2(elevHash(ps + 5.0), elevHash(ps + 11.0)) - 0.5;
  normal = normalize(normal + elevGlass * vec3(tilt * 0.07, 0.0));
}
#endif
`;function kg(o,l,r){const u={};u.ELEV_SHOP="",o.defines={...o.defines??{},...u},o.onBeforeCompile=i=>{i.uniforms.uElevTileW={value:r},i.fragmentShader=i.fragmentShader.replace("void main() {",`${Eg}
void main() {`).replace("#include <color_fragment>",Rg).replace("#include <roughnessmap_fragment>",Ug).replace("#include <lights_physical_fragment>",Ng).replace("#include <emissivemap_fragment>",Dg).replace("#include <normal_fragment_maps>",Og)},o.customProgramCacheKey=()=>`cn-elevation-${l}`,o.needsUpdate=!0}const Bg=`
uniform sampler2D uFacadeTable;
uniform float uInteriorLight;
uniform float uInteriorMap;
uniform float uFacadeFlat;
varying vec3 vFacadeWorld;

vec4 fTex(int x, int row) { return texelFetch(uFacadeTable, ivec2(x, row), 0); }
float fU16(vec2 hl) { return (floor(hl.x * 255.0 + 0.5) * 256.0 + floor(hl.y * 255.0 + 0.5)) / 1000.0; }
int fByte(float v) { return int(v * 255.0 + 0.5); }

float fHash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}
float fHash2(vec2 p) { return fHash(vec3(p, 7.13)); }
float fNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(fHash2(i), fHash2(i + vec2(1.0, 0.0)), f.x),
             mix(fHash2(i + vec2(0.0, 1.0)), fHash2(i + vec2(1.0, 1.0)), f.x), f.y);
}

// 1 inside [a, b], filtered over a pixel of width w.
float fBox(float x, float a, float b, float w) {
  return clamp((x - a) / w + 0.5, 0.0, 1.0) * clamp((b - x) / w + 0.5, 0.0, 1.0);
}
// Thin lines of half-width hw every 'period', filtered; fades to the lines'
// mean coverage once they are finer than a pixel.
float fLines(float x, float period, float hw, float w) {
  float d = abs(fract(x / period + 0.5) - 0.5) * period;
  float line = 1.0 - smoothstep(hw - w * 0.5, hw + w * 0.5, d);
  float mean = min(1.0, 2.0 * hw / period);
  return mix(line, mean, smoothstep(hw, hw * 4.0, w));
}

// A room behind a pane, traced from the glass.  Returns radiance.
// p: entry point (x along the room from its left, y above its floor),
// rd: ray in room space (x along, y up, z into the room), size: (w, h, d).
vec3 fRoom(vec2 p, vec3 rd, vec3 size, float seed, float office) {
  rd.z = max(rd.z, 1e-3);
  if (abs(rd.x) < 1e-4) rd.x = 1e-4;
  if (abs(rd.y) < 1e-4) rd.y = 1e-4;
  float tx = rd.x > 0.0 ? (size.x - p.x) / max(rd.x, 1e-4) : -p.x / min(rd.x, -1e-4);
  float ty = rd.y > 0.0 ? (size.y - p.y) / max(rd.y, 1e-4) : -p.y / min(rd.y, -1e-4);
  float tz = size.z / rd.z;
  float t = min(min(tx, ty), tz);
  vec3 hit = vec3(p, 0.0) + rd * t;

  float h1 = fHash(vec3(seed, 1.0, 3.0));
  float h2 = fHash(vec3(seed, 5.0, 7.0));
  float h3 = fHash(vec3(seed, 11.0, 13.0));
  vec3 wallC = h1 < 0.45 ? vec3(0.72, 0.70, 0.66)
    : h1 < 0.65 ? vec3(0.70, 0.64, 0.52)
    : h1 < 0.80 ? vec3(0.58, 0.64, 0.66)
    : h1 < 0.92 ? vec3(0.66, 0.66, 0.60)
    : vec3(0.52, 0.40, 0.30);
  vec3 floorC = h2 < 0.55 ? vec3(0.34, 0.22, 0.13) : h2 < 0.8 ? vec3(0.60, 0.58, 0.54) : vec3(0.42, 0.40, 0.38);
  vec3 c;
  if (t == tz) {
    c = wallC;
    // A door or a wardrobe on the back wall, and a picture.
    float door = fBox(hit.x, size.x * (0.15 + 0.5 * h3), size.x * (0.15 + 0.5 * h3) + 0.9, 0.02) * step(hit.y, 2.05);
    c = mix(c, office > 0.5 ? vec3(0.30) : vec3(0.40, 0.28, 0.18), door);
  } else if (t == ty) {
    if (rd.y < 0.0) {
      c = floorC * (0.9 + 0.1 * step(0.5, fract(hit.z / 0.15)));
    } else {
      c = vec3(0.80, 0.80, 0.78);
      if (office > 0.5) {
        // Ceiling tiles with light panels.
        c *= 0.85 + 0.15 * step(0.04, fract(hit.x / 0.6)) * step(0.04, fract(hit.z / 0.6));
      }
    }
  } else {
    c = wallC * 0.9;
  }
  // Furniture against the back wall: a sofa, a bed, a desk row, a cabinet.
  vec3 bmin = vec3(size.x * (0.1 + 0.2 * h2), 0.0, size.z - (office > 0.5 ? 1.4 : 0.9));
  vec3 bmax = vec3(bmin.x + size.x * (0.35 + 0.3 * h3), office > 0.5 ? 0.75 : 0.85 + 0.9 * step(0.7, h1), size.z);
  vec3 o = vec3(p, 0.0);
  vec3 inv = 1.0 / rd;
  vec3 t0 = (bmin - o) * inv;
  vec3 t1 = (bmax - o) * inv;
  vec3 tn = min(t0, t1);
  vec3 tf = max(t0, t1);
  float enter = max(max(tn.x, tn.y), tn.z);
  float leave = min(min(tf.x, tf.y), tf.z);
  if (enter > 0.0 && enter < leave && enter < t) {
    t = enter;
    hit = o + rd * t;
    vec3 cloth = vec3(fHash(vec3(seed, 17.0, 1.0)), fHash(vec3(seed, 19.0, 2.0)), fHash(vec3(seed, 23.0, 3.0)));
    c = office > 0.5 ? vec3(0.35, 0.34, 0.33) : mix(vec3(0.30), cloth, 0.5);
    c *= tn.y == enter ? 1.0 : 0.75;
  }
  // Daylight from the window falls off into the room; a lamp is on in some.
  float daylight = uInteriorLight * mix(1.0, 0.35, clamp(hit.z / size.z, 0.0, 1.0));
  float lamp = step(office > 0.5 ? 0.3 : 0.8, fHash(vec3(seed, 29.0, 31.0)));
  vec3 light = vec3(daylight) + lamp * uInteriorLight * vec3(1.1, 0.95, 0.75)
    * smoothstep(size.y * 1.6, 0.0, length(hit - vec3(size.x * 0.5, size.y, size.z * 0.5)));
  vec3 radiance = c * light;
  // The lamp itself.
  if (t == ty && rd.y > 0.0) {
    float disc = 1.0 - smoothstep(0.18, 0.22, length(hit.xz - vec2(size.x * 0.5, size.z * 0.5)));
    radiance += lamp * disc * vec3(3.0, 2.7, 2.2);
  }
  return radiance;
}
`,Hg=`
varying vec3 vFacadeWorld;
`,Lg=`
#include <worldpos_vertex>
vFacadeWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;
`,jg=`
float fGlass = 0.0;
float fRough = 0.9;
vec3 fEmit = vec3(0.0);
vec3 fF0 = vec3(0.04);
vec2 fTilt = vec2(0.0);
{
  int row = fByte(vColor.a);
  vec3 tint = vColor.rgb;
  vec4 h0 = fTex(0, row);
  vec4 h1 = fTex(1, row);
  vec4 h2 = fTex(2, row);
  vec4 h3 = fTex(3, row);
  vec4 h4 = fTex(4, row);
  float W = max(fU16(h0.rg), 0.5);
  float H = max(h0.b * 255.0 / 50.0, 2.0);
  int n = fByte(h0.a);
  int skin = fByte(h1.r);
  float tileModule = h1.g * 255.0 / 100.0;
  int glassKind = fByte(h1.b);
  float grilles = h1.a * 255.0;
  vec3 frameC = h2.rgb;
  vec3 trimC = h3.rgb;
  int flags = fByte(h3.a);
  vec3 coatC = h4.rgb;
  float weather = h4.a;

  float U = vUv.x;
  float V = vUv.y;
  float k = floor(U / W);
  bool odd = mod(k, 2.0) > 0.5;
  float x = U - k * W;
  if (odd) x = W - x;
  float s = floor(V);
  float ml = (V - s) * H;
  // Metres of wall per pixel, across and up.
  float fw = max(fwidth(U), 1e-4);
  float fh = max(fwidth(V) * H, 1e-4);

  // The span under the fragment.
  int kind = 0;
  float x0 = 0.0;
  float x1 = W;
  float p0 = 0.0;
  float p1 = 0.0;
  int spanBits = 0;
  int spanIndex = 0;
  for (int i = 0; i < 20; i++) {
    if (i >= n || uFacadeFlat > 0.5) break;
    vec4 a = fTex(8 + 2 * i, row);
    float sx1 = fU16(a.ba);
    if (x < sx1 || i == n - 1) {
      vec4 b = fTex(9 + 2 * i, row);
      x0 = fU16(a.rg);
      x1 = sx1;
      kind = fByte(b.r);
      p0 = b.g * 255.0 / 100.0;
      p1 = b.b * 255.0 / 50.0;
      spanBits = fByte(b.a);
      spanIndex = i;
      break;
    }
  }
  int panes = spanBits - (spanBits / 16) * 16;
  int style = spanBits / 16;

  // --- the skin -------------------------------------------------------------
  vec2 wp = vec2(U, V * H);
  float blotch = fNoise(vFacadeWorld.xz * 0.31 + vFacadeWorld.y * 0.27) - 0.5;
  float fine = fNoise(wp * 2.7) - 0.5;
  float streak = max(0.0, fNoise(vec2(U * 9.0, V * H * 0.35)) - 0.45) / 0.55;
  // Patch repairs and fading leave the paint a few per cent off from bay to bay.
  float patchN = fNoise(vec2(U * 0.23 + float(row), V * H * 0.11)) - 0.5;
  float paint = 0.86 * (1.0 + (0.06 + 0.08 * weather) * blotch + 0.03 * fine + 0.08 * patchN);
  paint *= 1.0 - (0.04 + 0.10 * weather) * streak * smoothstep(0.0, 0.9, ml / H + 0.2);
  vec3 skinC = tint * paint;
  float skinRough = 0.9;
  if (skin == 1) {
    // 面砖: small glazed tiles, a joint every module up and every 0.24 m across.
    float jointH = fLines(wp.y, max(tileModule, 0.04), 0.004, fh);
    float jointV = fLines(wp.x + 0.12 * mod(floor(wp.y / max(tileModule, 0.04)), 2.0), 0.24, 0.004, fw);
    float joint = max(jointH, jointV);
    vec2 cell = floor(vec2(wp.x / 0.24, wp.y / max(tileModule, 0.04)));
    float tile = 1.0 + 0.05 * (fHash2(cell) - 0.5) * smoothstep(0.06, 0.02, max(fw, fh));
    skinC = mix(tint * paint * tile, vec3(0.55, 0.54, 0.52) * paint, joint * 0.6);
    skinRough = 0.5;
  } else if (skin == 2) {
    float m = max(tileModule, 0.3);
    float joint = max(fLines(wp.y, m, 0.005, fh), fLines(wp.x, m * 2.0, 0.005, fw));
    vec2 cell = floor(vec2(wp.x / (m * 2.0), wp.y / m));
    skinC = tint * paint * (1.0 + 0.06 * (fHash2(cell) - 0.5)) * (1.0 - 0.25 * joint);
    skinRough = 0.7;
  } else if (skin == 3) {
    // Running-bond brick in a pale mortar.
    float course = floor(wp.y / 0.0625);
    float bx = wp.x + 0.12 * mod(course, 2.0);
    float mortar = max(fLines(wp.y, 0.0625, 0.005, fh), fLines(bx, 0.24, 0.005, fw));
    float brick = 1.0 + 0.16 * (fHash2(vec2(floor(bx / 0.24), course)) - 0.5) * smoothstep(0.05, 0.015, max(fw, fh));
    skinC = mix(tint * paint * brick, vec3(0.50, 0.48, 0.45), mortar * 0.7);
  } else if (skin == 4) {
    float joint = max(fLines(wp.y, 1.2, 0.006, fh), fLines(wp.x, 1.2, 0.006, fw));
    skinC = tint * paint * (1.0 - 0.3 * joint);
    skinRough = 0.45;
  } else if (skin == 5) {
    skinC = tint * 0.86 * (1.0 + 0.12 * blotch + 0.05 * fine)
      * (1.0 - (0.08 + 0.12 * weather) * streak);
  }
  // A band at every slab.
  if ((flags / 2) - (flags / 4) * 2 == 1) {
    float band = fBox(ml, -0.05, 0.22, fh) + fBox(ml, H - 0.02, H + 0.05, fh);
    skinC = mix(skinC, trimC * (1.0 - 0.3 * fBox(ml, -0.05, 0.02, fh)), clamp(band, 0.0, 1.0));
  }
  // Grime under the slab line, a soft shadow the projecting slab edge leaves.
  skinC *= 1.0 - 0.10 * fBox(ml, H - 0.12, H + 0.01, fh) * (0.5 + weather);

  vec3 colour = skinC;
  float rough = skinRough;

  // --- the openings --------------------------------------------------------
  // Opening rectangle, in the span's own x and in metres above the floor.
  float ox0 = x0;
  float ox1 = x1;
  float oy0 = 0.0;
  float oy1 = 0.0;
  float opening = 0.0;
  int sash = style;
  if (kind == 2) {
    oy0 = p0; oy1 = p1; opening = 1.0;
  } else if (kind == 3) {
    ox0 += 0.06; ox1 -= 0.06; oy0 = p0; oy1 = p1; opening = 1.0; panes = 3;
  } else if (kind == 4 || kind == 10) {
    ox0 += 0.3; ox1 -= 0.3; oy0 = 0.02; oy1 = p1; opening = 1.0;
    panes = int(clamp(floor((ox1 - ox0) / 0.9 + 0.5), 2.0, 6.0));
    sash = 0;
  } else if (kind == 6) {
    oy0 = p0; oy1 = p1; opening = 1.0; panes = 1; sash = 1;
  }
  bool surround = (flags - (flags / 2) * 2) == 1 && kind == 2;
  if (surround) { ox0 += 0.14; ox1 -= 0.14; }

  float glassCov = 0.0;
  float gu = 0.0;
  float gv = 0.0;
  float ow = max(ox1 - ox0, 0.1);
  float oh = max(oy1 - oy0, 0.1);
  if (kind == 1) {
    colour = trimC * (1.0 - 0.25 * (fBox(x, x0 - 0.1, x0 + 0.025, fw) + fBox(x, x1 - 0.025, x1 + 0.1, fw)));
    rough = 0.7;
  } else if (kind == 5) {
    // AC louvre screen.
    float screen = fBox(ml, p0, p1, fh) * fBox(x, x0, x1, fw);
    float slats = fLines(ml, 0.09, 0.022, fh);
    vec3 louvre = mix(trimC * 0.95, vec3(0.08), slats);
    louvre = mix(louvre, trimC, fBox(x, x0, x0 + 0.04, fw) + fBox(x, x1 - 0.04, x1, fw));
    colour = mix(colour, louvre, screen);
    rough = mix(rough, 0.6, screen);
  } else if (kind == 7) {
    // Curtain wall: mullions, transom at the spandrel, glass everywhere else.
    float m = max(p0, 0.6);
    float mull = max(fLines(x, m, 0.035, fw), fLines(ml, H, 0.035, fh));
    mull = max(mull, fLines(ml - p1, H * 4.0, 0.03, fh));
    glassCov = 1.0 - mull;
    gu = fract(x / m);
    gv = ml < p1 ? 0.0 : (ml - p1) / max(H - p1, 0.5);
    ow = m;
    oh = H - p1;
    colour = frameC;
    rough = 0.4;
  } else if (kind == 8) {
    float joint = max(fLines(ml, max(tileModule, 0.3), 0.005, fh), fBox(x, (x0 + x1) * 0.5 - 0.004, (x0 + x1) * 0.5 + 0.004, fw));
    colour = skinC * (1.0 - 0.2 * joint);
  } else if (kind == 9) {
    colour = skinC * (1.0 - 0.25 * fLines(ml, 1.2, 0.006, fh));
    rough = 0.45;
  }

  if (opening > 0.5) {
    float inOpen = fBox(x, ox0, ox1, fw) * fBox(ml, oy0, oy1, fh);
    float f = 0.055;
    float inGlass = fBox(x, ox0 + f, ox1 - f, fw) * fBox(ml, oy0 + f, oy1 - f, fh);
    float pw = (ow - 2.0 * f) / float(max(panes, 1));
    float gx = x - ox0 - f;
    float mull = panes > 1 ? fLines(gx, pw, 0.022, fw) * fBox(gx, pw * 0.5, ow - 2.0 * f - pw * 0.5, fw) : 0.0;
    float transom = 0.0;
    if (sash == 1 && oh > 1.5) transom = fBox(ml, oy1 - 0.55, oy1 - 0.51, fh);
    if (sash == 2) transom = fBox(ml, oy1 - 0.62, oy1 - 0.58, fh);
    if (sash == 0 && panes >= 2) mull = max(mull, fBox(gx, pw - 0.07, pw - 0.04, fw));
    float lattice = sash == 3 ? fLines(gx, 0.12, 0.012, fw) * 0.85 : 0.0;
    float glass = inGlass * (1.0 - mull) * (1.0 - transom);
    // Security grilles over the lowest storeys.
    float grille = 0.0;
    if (s < grilles) grille = max(fLines(gx, 0.11, 0.008, fw), fLines(ml, 0.55, 0.01, fh)) * inOpen;
    vec3 frameCol = mix(frameC, vec3(0.20, 0.18, 0.16), grille);
    colour = mix(colour, frameCol, clamp(inOpen - glass + grille * glass, 0.0, 1.0));
    rough = mix(rough, 0.45, inOpen);
    glassCov = glass * (1.0 - grille) * (1.0 - lattice);
    colour = mix(colour, trimC * 0.35, lattice * glass);
    gu = clamp((x - ox0) / ow, 0.0, 1.0);
    gv = clamp((ml - oy0) / oh, 0.0, 1.0);
    // The sill, its drip, and the shadowed head reveal.
    float sill = fBox(x, ox0 - 0.05, ox1 + 0.05, fw) * fBox(ml, oy0 - 0.06, oy0, fh);
    colour = mix(colour, trimC, sill * step(0.5, float(kind == 2)));
    float head = fBox(x, ox0, ox1, fw) * fBox(ml, oy1, oy1 + 0.09, fh);
    colour *= 1.0 - 0.32 * head;
    if (surround) {
      float ring = fBox(x, ox0 - 0.14, ox1 + 0.14, fw) * fBox(ml, oy0 - 0.16, oy1 + 0.18, fh) * (1.0 - inOpen);
      colour = mix(colour, trimC, ring);
    }
    if (kind == 4) {
      // Under the balcony above: the soffit's shadow on the door wall.
      colour *= 1.0 - 0.25 * fBox(ml, oy1 + 0.05, H + 0.01, fh);
    }
    if (kind == 10) {
      // A loggia: the whole span is a recess in shadow, its rail in front.
      float recess = fBox(x, x0, x1, fw) * fBox(ml, 0.0, H - 0.16, fh) * (1.0 - inOpen);
      colour = mix(colour, skinC * 0.42, recess);
      float rail = fBox(x, x0, x1, fw) * fBox(ml, 0.0, 1.1, fh);
      int railKind = style;
      vec3 railC = railKind == 0 ? skinC : railKind == 3 ? trimC : vec3(0.10, 0.13, 0.14);
      float railCov = railKind == 3 ? max(fLines(x, 0.12, 0.015, fw), fBox(ml, 1.02, 1.1, fh)) : 1.0;
      colour = mix(colour, railC, rail * railCov);
      glassCov *= 1.0 - rail * railCov;
      if (railKind == 1) { fGlass = rail; }
    }
  }

  // --- weathering ----------------------------------------------------------
  // What a photograph of any Chinese block shows and a clean render does not:
  // dirt washed down the wall in vertical streaks, darker under each sill where
  // the runoff leaves the drip, and a splash zone at the foot.  The strengths are
  // read off photographs of 1990s-2010s plaster and tile blocks: a streak takes
  // 10-30 % off the wall, a tear stain 15-35 %.  Each pattern averages to its
  // mean once it is finer than a pixel.
  {
    float wall = 1.0 - glassCov;
    float wfine = smoothstep(0.04, 0.2, fw);
    // Streak columns: a few per metre, each of its own strength, long up the wall.
    float colN = fNoise(vec2(U * 1.9 + float(row) * 3.7, V * H * 0.035));
    float colF = fNoise(vec2(U * 7.0, V * H * 0.12));
    float rain = smoothstep(0.42, 0.85, colN) * mix(0.55 + 0.45 * colF, 0.75, wfine);
    rain = mix(rain, 0.18, smoothstep(0.15, 0.6, fw));
    float tear = 0.0;
    if (opening > 0.5 && kind == 2) {
      // Under the sill, as far as the floor, fading as it runs.
      float drop = oy0 - 0.06 - ml;
      float run = smoothstep(0.0, 0.05, drop) * (1.0 - smoothstep(0.0, max(oy0 - 0.06, 0.3), drop));
      float drip = mix(0.35 + 0.65 * fNoise(vec2(x * 16.0, float(s) * 5.1)), 0.65, wfine);
      tear = fBox(x, ox0 - 0.04, ox1 + 0.04, fw) * run * drip;
    }
    // Splash zone: the bottom 0.6 m of the lowest storey.
    float splash = (s < 0.5 ? 1.0 - smoothstep(0.1, 0.6, ml) : 0.0);
    float dirt = (0.10 + 0.20 * weather) * rain + (0.15 + 0.20 * weather) * tear + 0.18 * splash;
    // Grime is brown-grey, so it takes more blue than red off the wall.
    colour *= vec3(1.0) - vec3(0.85, 0.92, 1.0) * clamp(dirt, 0.0, 0.6) * wall;
    // Repainted patches: a maintenance crew rolls a new coat on a rectangle of
    // wall, and the new coat never quite matches (a few % lighter or warmer,
    // with a straight edge). Cells ~4.5 m x 6 m, one in six repainted.
    vec2 rpC = vec2(U / 4.5 + float(row) * 1.7, V * H / 6.0);
    float rpH = fHash2(floor(rpC) + 11.0);
    vec2 rpF = fract(rpC);
    float rpIn = step(0.08, rpF.x) * step(rpF.x, 0.92) * step(0.1, rpF.y) * step(rpF.y, 0.9);
    float rp = step(0.84, rpH) * rpIn * wall * (1.0 - wfine * 0.6);
    colour *= mix(vec3(1.0), vec3(1.07, 1.05, 0.99) + (rpH - 0.92) * vec3(0.4, 0.2, -0.2), rp);
    // Efflorescence and rising damp: pale lime blooms low on the wall, a damp
    // darkening just above the pavement.
    float lowZone = (s < 0.5 ? 1.0 - smoothstep(0.4, 2.2, ml) : 0.0);
    float bloom = smoothstep(0.55, 0.8, fNoise(vec2(U * 2.3, ml * 3.1 + 7.0))) * lowZone * (0.4 + 0.6 * weather);
    colour = mix(colour, colour * 0.45 + vec3(0.5, 0.5, 0.47), bloom * 0.45 * wall * (1.0 - wfine * 0.5));
    colour *= 1.0 - 0.10 * lowZone * smoothstep(0.35, 0.65, fNoise(vec2(U * 0.8, 3.0))) * wall;
    // Wet / dirty paint is glossier than dry chalky render in streaks.
    rough = mix(rough, rough - 0.12, clamp(rain * 0.6, 0.0, 1.0) * wall);
  }

  // --- glass ----------------------------------------------------------------
  if (glassCov > 0.001) {
    float spandrel = kind == 7 && ml < p1 ? 1.0 : 0.0;
    float office = glassKind >= 2 ? 1.0 : 0.0;
    // Which window this is: the row, the span, the flat, the storey, and on a
    // curtain wall the tileModule.
    float cell = kind == 7 ? floor(U / max(p0, 0.6)) : 0.0;
    float seed = fHash(vec3(float(row) * 1.37 + float(spanIndex) * 0.61, k * 3.1 + s * 17.3, cell * 0.73)) * 1000.0;
    float g2 = fHash(vec3(seed, 3.0, 4.0));
    float g3 = fHash(vec3(seed, 5.0, 6.0));
    float g4 = fHash(vec3(seed, 7.0, 8.0));
    float detail = 1.0 - smoothstep(0.03, 0.10, max(fw, fh));
    float far = smoothstep(0.08, 0.25, max(fw, fh));

    // Curtains in the plane of the glass, by window.
    float du = fw / ow * 1.5;
    float curtain = 0.0;
    vec3 curtainC = vec3(0.0);
    if (office > 0.5) {
      if (g2 < 0.45) {
        float f = 0.15 + 0.85 * g3;
        curtain = smoothstep(1.0 - f - 0.01, 1.0 - f + 0.01, gv);
        curtainC = vec3(0.45, 0.45, 0.43) * (0.8 + 0.2 * fLines(gv * oh, 0.05, 0.012, fh));
      }
    } else if (g2 < 0.30) {
      curtain = 0.0;
    } else if (g2 < 0.68) {
      float f = 0.25 + 0.75 * g3;
      curtain = 0.82 * (g4 < 0.5 ? smoothstep(f + du, f - du, gu) : smoothstep(1.0 - f - du, 1.0 - f + du, gu));
      curtainC = vec3(0.62, 0.60, 0.55);
    } else if (g2 < 0.9) {
      float f = 0.3 + 0.7 * g3;
      curtain = smoothstep(f * 0.5 + du, f * 0.5 - du, gu) + smoothstep(1.0 - f * 0.5 - du, 1.0 - f * 0.5 + du, gu);
      curtainC = g4 < 0.3 ? vec3(0.42, 0.33, 0.22) : g4 < 0.55 ? vec3(0.22, 0.26, 0.33) : g4 < 0.8 ? vec3(0.55, 0.50, 0.40) : vec3(0.26, 0.16, 0.12);
    } else {
      float f = 0.2 + 0.8 * g3;
      curtain = smoothstep(1.0 - f - 0.01, 1.0 - f + 0.01, gv);
      curtainC = vec3(0.56, 0.54, 0.50);
    }
    curtain = clamp(curtain, 0.0, 1.0);
    float folds = 1.0 - 0.12 * (1.0 - smoothstep(0.02, 0.05, fw)) * (0.5 - 0.5 * sin(gu * ow / 0.11 * 6.2831));
    vec3 curtainLit = curtainC * folds * uInteriorLight * 1.6;

    // The room: traced near, averaged far.
    vec3 room = vec3(0.0);
    vec3 roomMean = vec3(0.55, 0.52, 0.47) * uInteriorLight * 0.55;
    if (detail > 0.001 && uInteriorMap > 0.5 && spandrel < 0.5) {
      vec3 nV = normalize(vNormal) * (gl_FrontFacing ? 1.0 : -1.0);
      vec3 nW = normalize((vec4(nV, 0.0) * viewMatrix).xyz);
      vec3 tW = normalize(cross(vec3(0.0, 1.0, 0.0), nW));
      vec3 dpx = dFdx(vFacadeWorld);
      vec3 dpy = dFdy(vFacadeWorld);
      if (dot(tW, dpx) * dFdx(U) + dot(tW, dpy) * dFdy(U) < 0.0) tW = -tW;
      vec3 rdW = normalize(vFacadeWorld - cameraPosition);
      vec3 rd = vec3(dot(rdW, tW), rdW.y, -dot(rdW, nW));
      // The room spans the opening and a little more, centred on it, in
      // global U so mirrored flats need no special case.
      float uo0 = kind == 7 ? cell * max(p0, 0.6) : odd ? k * W + (W - ox1) : k * W + ox0;
      float roomW = max(ow + 1.4, office > 0.5 ? 4.0 : 3.0);
      float px = U - (uo0 + ow * 0.5) + roomW * 0.5;
      vec3 size = vec3(roomW, H - 0.15, office > 0.5 ? 7.0 : 4.2);
      room = fRoom(vec2(px, ml), rd, size, seed, office);
    }
    vec3 inside = mix(roomMean, room, detail);
    inside = mix(inside, curtainLit, curtain);
    if (spandrel > 0.5) inside = vec3(0.012);
    float transmit = office > 0.5 ? 0.45 : glassKind == 1 ? 0.6 : 0.8;
    vec3 glassTint = glassKind == 1 ? mix(vec3(1.0), coatC * 1.5, 0.5) : glassKind == 3 ? mix(vec3(1.0), coatC * 1.4, 0.6) : vec3(1.0);
    inside *= transmit * glassTint;
    inside = mix(inside, vec3(0.55, 0.52, 0.47) * uInteriorLight * 0.45 * transmit, far);

    fEmit = inside * glassCov;
    fGlass = max(fGlass, glassCov);
    colour = mix(colour, vec3(0.0), glassCov);
    // Dirty glass: dust and rain-borne grime dry in vertical runs, the pane is
    // rougher and greyer there and reflects less crisply.
    float gDirt = smoothstep(0.45, 0.9, fNoise(vec2(U * 3.3 + seed, V * H * 0.05))) * (0.55 + 0.45 * fNoise(vec2(U * 11.0, V * H * 0.2)));
    gDirt = mix(gDirt, 0.25, smoothstep(0.1, 0.5, fw)) * (0.25 + 0.75 * weather);
    rough = mix(rough, mix(0.035, 0.34, gDirt), glassCov);
    colour += vec3(0.30, 0.29, 0.26) * gDirt * 0.35 * glassCov;
    fF0 = glassKind == 3 ? mix(vec3(0.04), coatC, 0.3) : glassKind == 2 ? vec3(0.10, 0.11, 0.12) : glassKind == 1 ? mix(vec3(0.05), coatC * 0.12, 0.5) : vec3(0.045);
    // Each pane is glazed a fraction of a degree off true.
    float pane = floor(gu * float(max(panes, 1)));
    fTilt = (vec2(fHash(vec3(seed, pane, 41.0)), fHash(vec3(seed, pane, 43.0))) - 0.5) * 0.06;
  }
  fRough = rough;
  diffuseColor.rgb = diffuse * colour;
  diffuseColor.a = 1.0;
}
`,qg=`
#include <roughnessmap_fragment>
roughnessFactor = fRough;
`,Gg=`
#include <normal_fragment_maps>
normal = normalize(normal + fGlass * vec3(fTilt, 0.0));
`,Fg=`
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, fF0, fGlass);
material.specularColorBlended = mix(material.specularColorBlended, fF0, fGlass);
`,Vg=`
#include <emissivemap_fragment>
totalEmissiveRadiance += fEmit;
`,sp=.16;function Kg(o){const l=new Ke({color:16777215,roughness:.9,metalness:0,envMapIntensity:1,vertexColors:!0});return l.defines={...l.defines??{},USE_UV:""},l.onBeforeCompile=r=>{r.uniforms.uFacadeTable={value:o},r.uniforms.uInteriorLight={value:sp},r.uniforms.uFacadeFlat={value:new URLSearchParams(window.location.search).get("facade")==="flat"?1:0},r.uniforms.uInteriorMap={value:new URLSearchParams(window.location.search).get("interior")==="0"?0:1},r.vertexShader=r.vertexShader.replace("#include <common>",`#include <common>
${Hg}`).replace("#include <worldpos_vertex>",Lg),r.fragmentShader=r.fragmentShader.replace("#include <common>",`#include <common>
${Bg}`).replace("#include <color_fragment>",jg).replace("#include <roughnessmap_fragment>",qg).replace("#include <normal_fragment_maps>",Gg).replace("#include <lights_physical_fragment>",Fg).replace("#include <emissivemap_fragment>",Vg)},l.customProgramCacheKey=()=>"cn-facade",l}function Yg(o,l,r){const u=new ma(o,l,r,$n);return u.colorSpace=Y1,u.magFilter=Ts,u.minFilter=Ts,u.generateMipmaps=!1,u.flipY=!1,u.needsUpdate=!0,u}function g1(o,l,r,u){o.defines={...o.defines??{},USE_UV:""},o.onBeforeCompile=f=>{f.uniforms.uRibPeriod={value:l},f.uniforms.uRibDepth={value:r},f.fragmentShader=f.fragmentShader.replace("#include <common>",`#include <common>
uniform float uRibPeriod;
uniform float uRibDepth;`).replace("#include <normal_fragment_maps>",`#include <normal_fragment_maps>
        {
          float t = vUv[${u}] / uRibPeriod;
          float w = fwidth(t);
          // A trapezoid: flat crown, sloping web, flat pan. The slope of the
          // profile is what tilts the normal.
          float f = fract(t);
          float slope = (smoothstep(0.15, 0.25, f) - smoothstep(0.55, 0.65, f)) * 2.0 - 1.0;
          float web = 1.0 - smoothstep(0.0, 0.08, abs(abs(f - 0.4) - 0.2));
          float tilt = slope * web * uRibDepth * (1.0 - smoothstep(0.25, 0.6, w));
          // The tangent along the UV axis, from screen derivatives.
          vec3 dp1 = dFdx(-vViewPosition);
          vec3 dp2 = dFdy(-vViewPosition);
          vec2 duv1 = dFdx(vUv);
          vec2 duv2 = dFdy(vUv);
          vec3 tangent = ${u===0?"dp1 * duv2.y - dp2 * duv1.y":"dp2 * duv1.x - dp1 * duv2.x"};
          float tl = length(tangent);
          if (tl > 1e-6) normal = normalize(normal + (tangent / tl) * tilt);
        }`)};const i=o.customProgramCacheKey.bind(o);o.customProgramCacheKey=()=>`${i()}|ribs${u}`,o.needsUpdate=!0}const Wg=`
varying vec3 vWxPos;
varying vec3 vWxNormal;
`,Xg=`
uniform float uWxStrength;
varying vec3 vWxPos;
varying vec3 vWxNormal;
float wxRain = 0.0;
float wxHash(vec2 p) {
  vec3 q = fract(vec3(p.xyx) * vec3(0.1031, 0.1030, 0.0973));
  q += dot(q, q.yxz + 33.33);
  return fract((q.x + q.y) * q.z);
}
float wxNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(wxHash(i), wxHash(i + vec2(1.0, 0.0)), f.x),
             mix(wxHash(i + vec2(0.0, 1.0)), wxHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
`,Pg=`
#include <color_fragment>
{
  float wallness = 1.0 - smoothstep(0.35, 0.7, abs(vWxNormal.y));
  vec2 along = vec2(-vWxNormal.z, vWxNormal.x);
  along /= max(length(along), 1e-4);
  float u = dot(vWxPos.xz, along);
  float fw = max(fwidth(u), 1e-4);
  // How weathered this block is: a slow field across the city, so neighbours
  // differ but one building is of a piece.
  float weather = wxNoise(vWxPos.xz * 0.013);
  float colN = wxNoise(vec2(u * 1.9, vWxPos.y * 0.035));
  float colF = wxNoise(vec2(u * 7.0, vWxPos.y * 0.12));
  float rain = smoothstep(0.42, 0.85, colN) * mix(0.55 + 0.45 * colF, 0.75, smoothstep(0.04, 0.2, fw));
  rain = mix(rain, 0.18, smoothstep(0.15, 0.6, fw));
  float patchN = wxNoise(vec2(u * 0.23, vWxPos.y * 0.11)) - 0.5;
  diffuseColor.rgb *= 1.0 + 0.08 * patchN * wallness * uWxStrength;
  float dirt = clamp((0.10 + 0.20 * weather) * rain * uWxStrength, 0.0, 0.5) * wallness;
  diffuseColor.rgb *= vec3(1.0) - vec3(0.85, 0.92, 1.0) * dirt;
  // Repainted patches (a rectangle of newer coat, a few % lighter and warmer),
  // pale lime efflorescence and damp darkening low on the wall.
  vec2 rpC = vec2(u / 4.5, vWxPos.y / 6.0);
  float rpH = wxHash(floor(rpC) + 11.0);
  vec2 rpF = fract(rpC);
  float rpIn = step(0.08, rpF.x) * step(rpF.x, 0.92) * step(0.1, rpF.y) * step(rpF.y, 0.9);
  float rp = step(0.84, rpH) * rpIn * wallness * (1.0 - 0.6 * smoothstep(0.15, 0.6, fw)) * uWxStrength;
  diffuseColor.rgb *= mix(vec3(1.0), vec3(1.07, 1.05, 0.99) + (rpH - 0.92) * vec3(0.4, 0.2, -0.2), rp);
  float lowZone = 1.0 - smoothstep(0.4, 2.2, vWxPos.y);
  float bloom = smoothstep(0.55, 0.8, wxNoise(vec2(u * 2.3, vWxPos.y * 3.1 + 7.0))) * lowZone * (0.4 + 0.6 * weather);
  diffuseColor.rgb = mix(diffuseColor.rgb, diffuseColor.rgb * 0.45 + vec3(0.5, 0.5, 0.47), bloom * 0.4 * wallness * uWxStrength * (1.0 - 0.5 * smoothstep(0.15, 0.6, fw)));
  diffuseColor.rgb *= 1.0 - 0.10 * lowZone * wallness * uWxStrength * smoothstep(0.35, 0.65, wxNoise(vec2(u * 0.8, 3.0)));
  wxRain = rain;
}
`;function Al(o,l=1){const r=o.onBeforeCompile;o.onBeforeCompile=(i,f)=>{r?.call(o,i,f),i.uniforms.uWxStrength={value:l},i.vertexShader=i.vertexShader.replace("#include <common>",`#include <common>
${Wg}`).replace("#include <begin_vertex>",`#include <begin_vertex>
vWxPos = position;
vWxNormal = normal;`),i.fragmentShader=i.fragmentShader.replace("#include <common>",`#include <common>
${Xg}`).replace("#include <color_fragment>",Pg).replace("#include <roughnessmap_fragment>",`#include <roughnessmap_fragment>
roughnessFactor = clamp(roughnessFactor - 0.10 * wxRain * uWxStrength, 0.3, 1.0);`)};const u=o.customProgramCacheKey.bind(o);o.customProgramCacheKey=()=>`${u()}|weathered`,o.needsUpdate=!0}const Zg='"Microsoft YaHei","PingFang SC","Noto Sans CJK SC","Source Han Sans SC","WenQuanYi Zen Hei","Noto Sans SC",sans-serif',Qg='"SimHei","Heiti SC","Microsoft YaHei","PingFang SC","Noto Sans CJK SC","WenQuanYi Zen Hei",sans-serif',Ig='"SimSun","Songti SC","Noto Serif CJK SC","Source Han Serif SC","STSong","WenQuanYi Zen Hei",serif',Jg='"KaiTi","STKaiti","Kaiti SC","AR PL UKai CN","DFKai-SB","WenQuanYi Zen Hei",serif',$g='"Microsoft YaHei UI","PingFang SC","HarmonyOS Sans SC","Noto Sans CJK SC","WenQuanYi Zen Hei",sans-serif',e0=[[800,Zg,0],[900,Qg,.035],[700,Ig,.02],[400,Jg,.045],[700,$g,.02]],Po=o=>`#${o}`;function ey(o){return{cols:+o[2],rows:+o[3],cw:+o[4],ch:+o[5]}}function Iu(o,l){if(typeof document>"u")return null;const r=document.createElement("canvas");return r.width=o,r.height=l,r.getContext("2d")}function ty(o,l,r,u){return[...l].map(i=>o.measureText(i).width+r*u)}function sh(o,l,r,u,i){const[f,h,v]=e0[i.font%e0.length];o.save(),o.font=`${f} ${i.size}px ${h}`,o.textBaseline="middle",o.textAlign="left";const m=[...l].length,p=i.gap??(m<=2?.28:m===3?.06:0),b=ty(o,l,p,i.size),S=b.reduce((D,H)=>D+H,0)-p*i.size,C=S>i.maxW?i.maxW/S:1;o.translate(r,u),o.scale(C,1);let x=i.align==="left"?0:-S/2;if(i.shadow){o.fillStyle=i.shadow[2];let D=x;[...l].forEach((H,q)=>{o.fillText(H,D+i.shadow[0],i.shadow[1]),D+=b[q]})}i.glow&&(o.shadowColor=i.glow,o.shadowBlur=i.size*.25),o.lineJoin="round",[...l].forEach((D,H)=>{i.stroke&&i.strokeW&&(o.lineWidth=i.strokeW,o.strokeStyle=i.stroke,o.strokeText(D,x,0)),o.fillStyle=i.fill,o.fillText(D,x,0),v>0&&(o.lineWidth=i.size*v,o.strokeStyle=i.fill,o.strokeText(D,x,0)),x+=b[H]}),o.restore()}function pc(o,l,r,u,i,f){o.beginPath(),o.moveTo(l+f,r),o.arcTo(l+u,r,l+u,r+i,f),o.arcTo(l+u,r+i,l,r+i,f),o.arcTo(l,r+i,l,r,f),o.arcTo(l,r,l+u,r,f),o.closePath()}function ay(o,l,r,u,i,f,h,v){o.save(),o.translate(r,u),o.lineCap="round",o.lineJoin="round";const m=(p,b,S,C)=>{o.fillStyle=C,o.beginPath(),o.arc(p,b,S,0,Math.PI*2),o.fill()};switch(l){case 1:{m(0,i*.42,i*.58,f),m(0,-i*.3,i*.42,f),o.fillStyle=v,o.beginPath(),o.moveTo(-i*.36,-i*.55),o.lineTo(-i*.3,-i*.95),o.lineTo(-i*.1,-i*.7),o.lineTo(0,-i),o.lineTo(i*.1,-i*.7),o.lineTo(i*.3,-i*.95),o.lineTo(i*.36,-i*.55),o.closePath(),o.fill(),m(-i*.14,-i*.32,i*.06,h),m(i*.14,-i*.32,i*.06,h),o.fillStyle=h,o.fillRect(-i*.4,i*0,i*.8,i*.1);break}case 2:{o.strokeStyle=f,o.lineWidth=i*.4,o.beginPath(),o.moveTo(-i*.88,i*.85),o.bezierCurveTo(-i*.88,-i*1.1,-i*.02,-i*1.1,-i*.02,i*.3),o.moveTo(i*.88,i*.85),o.bezierCurveTo(i*.88,-i*1.1,i*.02,-i*1.1,i*.02,i*.3),o.stroke();break}case 3:{pc(o,-i*.85,-i*.85,i*1.7,i*1.7,i*.3),o.fillStyle=f,o.fill(),o.fillStyle=v;for(let p=-2;p<=2;p++)o.fillRect(p*i*.3-i*.07,-i*.5,i*.14,i*1.1);o.fillRect(-i*.6,-i*.7,i*1.2,i*.16);break}case 4:{m(0,0,i,f),m(0,0,i*.84,v),m(0,0,i*.74,f),m(0,0,i*.66,v),o.fillStyle=f,o.beginPath();for(let p=0;p<10;p++){const b=-Math.PI/2+p*Math.PI/5,S=p%2===0?i*.5:i*.2;o.lineTo(Math.cos(b)*S,Math.sin(b)*S)}o.closePath(),o.fill();break}case 5:{m(0,i*.3,i*.62,f),o.strokeStyle=f,o.lineWidth=i*.14;for(const p of[-1,1])o.beginPath(),o.moveTo(p*i*.3,-i*.2),o.lineTo(p*i*.55,-i*.8),o.moveTo(p*i*.45,-i*.5),o.lineTo(p*i*.85,-i*.55),o.stroke();m(-i*.2,i*.25,i*.07,h),m(i*.2,i*.25,i*.07,h);break}case 6:{o.strokeStyle=f,o.lineWidth=i*.17;for(let p=1;p<=3;p++)o.beginPath(),o.arc(-i*.4,i*.5,i*.45*p,-Math.PI*.45,0),o.stroke();m(-i*.4,i*.5,i*.12,f);break}case 7:{o.strokeStyle=f,o.lineWidth=i*.2;for(const p of[-1,1])o.beginPath(),o.arc(p*i*.36,0,i*.52,0,Math.PI*2),o.stroke();break}case 8:{o.strokeStyle=f,o.lineWidth=i*.24,o.beginPath(),o.arc(0,0,i*.72,.35,Math.PI*2-.05),o.stroke(),o.fillStyle=v,o.fillRect(-i*.72,-i*.08,i*1.2,i*.17);break}case 9:{m(0,0,i,f),o.fillStyle=h,o.fillRect(-i*.36,-i*.36,i*.72,i*.72),o.fillStyle=f,o.fillRect(-i*.22,-i*.22,i*.44,i*.44),o.fillStyle=h,o.fillRect(-i*.1,-i*.1,i*.2,i*.2);break}case 10:{m(0,0,i,f),o.fillStyle=h,o.fillRect(-i*.5,-i*.5,i*1,i*.18),o.fillRect(-i*.09,-i*.5,i*.18,i*1),o.fillRect(-i*.5,i*.32,i*1,i*.18);break}case 11:{pc(o,-i*.9,-i*.9,i*1.8,i*1.8,i*.28),o.fillStyle=f,o.fill(),o.strokeStyle=h,o.lineWidth=i*.16,o.strokeRect(-i*.45,-i*.45,i*.9,i*.9),o.fillStyle=h,o.fillRect(-i*.9,-i*.08,i*1.8,i*.16);break}case 12:{o.fillStyle=f,pc(o,-i*.3,-i*.9,i*.6,i*1.8,i*.1),o.fill(),pc(o,-i*.9,-i*.3,i*1.8,i*.6,i*.1),o.fill();break}case 13:{o.fillStyle=f,o.beginPath();for(let p=0;p<6;p++){const b=Math.PI/6+p*Math.PI/3;o.lineTo(Math.cos(b)*i*.95,Math.sin(b)*i*.95)}o.closePath(),o.fill(),o.fillStyle=h,o.fillRect(-i*.7,-i*.1,i*1.4,i*.2);break}case 14:{o.fillStyle=f,o.beginPath(),o.moveTo(-i*.9,i*.8),o.lineTo(0,-i*.85),o.lineTo(i*.9,i*.8),o.lineTo(i*.45,i*.8),o.lineTo(0,-i*.05),o.lineTo(-i*.45,i*.8),o.closePath(),o.fill();break}case 15:{o.fillStyle=f,o.beginPath(),o.moveTo(-i*.95,-i*.1),o.lineTo(-i*.5,-i*.35),o.lineTo(-i*.12,i*.25),o.lineTo(i*.7,-i*.8),o.lineTo(i*.95,-i*.7),o.lineTo(-i*.1,i*.85),o.closePath(),o.fill();break}case 16:{o.fillStyle=f,pc(o,-i*.5,-i*.1,i*1,i*1,i*.2),o.fill(),o.fillRect(-i*.26,-i*.7,i*.52,i*.7),o.fillStyle=v,o.fillRect(-i*.32,-i*.9,i*.64,i*.2),o.fillStyle=h,o.fillRect(-i*.5,i*.2,i*1,i*.14);break}case 17:{o.fillStyle=f,o.beginPath(),o.arc(0,0,i*.85,0,Math.PI),o.closePath(),o.fill(),o.strokeStyle=f,o.lineWidth=i*.1;for(const p of[-1,0,1])o.beginPath(),o.moveTo(p*i*.4,-i*.1),o.bezierCurveTo(p*i*.4-i*.2,-i*.35,p*i*.4+i*.2,-i*.5,p*i*.4,-i*.8),o.stroke();break}case 18:{o.fillStyle=f,o.beginPath(),o.ellipse(0,i*.1,i*.95,i*.5,0,Math.PI,0),o.fill(),o.fillRect(-i*.9,i*.05,i*1.8,i*.28);break}case 19:{o.fillStyle=f,o.beginPath(),o.moveTo(-i*.8,i*.8),o.bezierCurveTo(-i*.9,-i*.6,i*.2,-i*1,i*.85,-i*.8),o.bezierCurveTo(i*.9,-i*0,i*.3,i*.8,-i*.8,i*.8),o.fill(),o.strokeStyle=h,o.lineWidth=i*.08,o.beginPath(),o.moveTo(-i*.7,i*.7),o.lineTo(i*.5,-i*.5),o.stroke();break}case 22:{o.fillStyle=f,o.beginPath(),o.moveTo(-i*.6,-i*.3),o.lineTo(i*.6,-i*.3),o.lineTo(i*.42,i*.9),o.lineTo(-i*.42,i*.9),o.closePath(),o.fill(),o.fillRect(-i*.7,-i*.5,i*1.4,i*.2),o.strokeStyle=f,o.lineWidth=i*.1,o.beginPath(),o.moveTo(i*.1,-i*.5),o.lineTo(i*.3,-i*1),o.stroke();break}case 23:{o.fillStyle=f,o.beginPath(),o.moveTo(0,-i*.9),o.lineTo(i*.7,0),o.lineTo(0,i*.9),o.lineTo(-i*.7,0),o.closePath(),o.fill(),o.fillStyle=h,o.beginPath(),o.moveTo(0,-i*.45),o.lineTo(i*.35,0),o.lineTo(0,i*.45),o.lineTo(-i*.35,0),o.closePath(),o.fill();break}}o.restore()}function ny(o,l,r,u,i,f){o.save(),o.translate(l,r),o.beginPath(),o.rect(0,0,u,i),o.clip();const h=Po(f.bg),v=Po(f.fg),m=Po(f.acc);o.fillStyle=h,o.fillRect(0,0,u,i);let p=u/2,b=u*.9,S=i*.5,C=i*.62;f.sub&&(S=i*.42,C=i*.5);const x=i*.84,D=i*.17,H=v;switch(f.style){case 1:{const P=o.createLinearGradient(0,0,0,i);P.addColorStop(0,"rgba(255,255,255,0.16)"),P.addColorStop(.5,"rgba(255,255,255,0.0)"),P.addColorStop(1,"rgba(0,0,0,0.12)"),o.fillStyle=P,o.fillRect(0,0,u,i),o.strokeStyle="rgba(200,205,210,0.9)",o.lineWidth=5,o.strokeRect(2.5,2.5,u-5,i-5),o.strokeStyle=m,o.globalAlpha=.7,o.lineWidth=2,o.strokeRect(9,9,u-18,i-18),o.globalAlpha=1;break}case 2:{o.fillStyle=m,o.globalAlpha=.9,o.fillRect(0,i*.8,u,i*.2),o.globalAlpha=1,C=f.sub?i*.56:i*.72,S=f.sub?i*.42:i*.46;break}case 3:{const P=i*.36,K=i*.62;f.mark,ay(o,f.mark,K,i*.5,P,v,h,m),p=(K+P+u)/2+2,b=u-(K+P)-i*.28,C=f.sub?i*.46:i*.54,S=f.sub?i*.4:i*.5;break}case 5:{o.fillStyle="#101114",o.fillRect(0,0,u,i),o.strokeStyle=v,o.globalAlpha=.55,o.lineWidth=3,o.strokeRect(6,6,u-12,i-12),o.globalAlpha=1;break}case 6:{const P=f.mark===20?["#f07a1a","#00833e","#e03a2c"]:[Po(f.acc),"#1b8a43",Po(f.acc)],K=i*.075;P.forEach((U,z)=>{o.fillStyle=U,o.fillRect(0,z*K+2,u,K*.8),o.fillRect(0,i-(z+1)*K-2,u,K*.8)}),S=f.sub?i*.46:i*.5,C=f.sub?i*.4:i*.46;break}default:o.strokeStyle=m,o.globalAlpha=.55,o.lineWidth=2,o.strokeRect(6,6,u-12,i-12),o.globalAlpha=1}const q=f.style===2;sh(o,f.text,p,S,{fill:v,size:C,font:f.font,maxW:b,stroke:q?m:void 0,strokeW:q?C*.09:0,shadow:q?[C*.045,C*.06,"rgba(0,0,0,0.55)"]:void 0,glow:f.style===5?v:void 0}),f.sub&&sh(o,f.sub,p,x,{fill:H,size:D,font:0,maxW:b*.95,gap:.04}),o.restore()}function oy(o,l,r,u,i,f){o.save(),o.translate(l,r),o.beginPath(),o.rect(0,0,u,i),o.clip(),o.fillStyle=Po(f.bg),o.fillRect(0,0,u,i),o.strokeStyle=Po(f.acc),o.globalAlpha=.8,o.lineWidth=3,o.strokeRect(8,8,u-16,i-16),o.globalAlpha=1;const h=[...f.text],v=h.length,m=Math.min(u*.62,(i-44)/v*.92),p=(i-44)/v;h.forEach((b,S)=>{sh(o,b,u/2,22+p*(S+.5),{fill:Po(f.fg),size:m,font:f.font,maxW:u*.8,stroke:f.style===2?Po(f.acc):void 0,strokeW:f.style===2?m*.08:0})}),o.restore()}function ly(o,l,r,u,i){const h=i.mark>0?120:0,v=h+[...i.name].length*100,m=(r-v)/2,p=Po(i.colour);if(o.save(),o.translate(0,l),i.mark>0){const C=m+50,x=u/2;o.fillStyle="rgba(0,0,0,0.55)",o.strokeStyle="rgba(0,0,0,0.55)",o.save(),o.translate(3,4),y1(o,i.mark,C,x,40,"rgba(0,0,0,0.55)"),o.restore(),y1(o,i.mark,C,x,40,p)}const[b,S]=e0[i.font%e0.length];o.font=`${b} 92px ${S}`,o.textBaseline="middle",o.textAlign="center",o.lineJoin="round",[...i.name].forEach((C,x)=>{const D=m+h+100*(x+.5),H=u/2+2;o.fillStyle="rgba(0,0,0,0.5)",o.fillText(C,D+4,H+5),o.lineWidth=6,o.strokeStyle="rgba(20,22,26,0.75)",o.strokeText(C,D,H),o.fillStyle=p,o.fillText(C,D,H),i.style===1&&(o.lineWidth=2,o.strokeStyle="rgba(255,255,255,0.35)",o.strokeText(C,D,H))}),o.restore()}function y1(o,l,r,u,i,f){switch(o.save(),o.translate(r,u),o.fillStyle=f,o.strokeStyle=f,o.lineWidth=i*.28,o.lineJoin="round",l){case 1:o.beginPath(),o.arc(0,0,i*.85,0,Math.PI*2),o.stroke(),o.beginPath(),o.arc(0,0,i*.3,0,Math.PI*2),o.fill();break;case 2:o.beginPath(),o.moveTo(0,-i),o.lineTo(i*.8,0),o.lineTo(0,i),o.lineTo(-i*.8,0),o.closePath(),o.fill();break;case 3:o.beginPath(),o.moveTo(0,-i*.95),o.lineTo(i*.95,i*.8),o.lineTo(-i*.95,i*.8),o.closePath(),o.fill();break;case 4:o.beginPath();for(let h=0;h<6;h++){const v=h*Math.PI/3;o.lineTo(Math.cos(v)*i,Math.sin(v)*i)}o.closePath(),o.stroke();break;case 5:o.beginPath(),o.moveTo(-i,i*.2),o.bezierCurveTo(-i*.5,-i*.9,0,i*.9,i*.5,-i*.2),o.lineTo(i,-i*.5),o.stroke();break;default:o.strokeRect(-i*.8,-i*.8,i*1.6,i*1.6),o.fillRect(-i*.3,-i*.3,i*.6,i*.6)}o.restore()}function sy(){const l=Iu(128,128);if(!l)return null;const r=l.createImageData(128,128),u=[[200,28,36],[244,244,240],[34,74,166],[244,244,240]];for(let f=0;f<128;f++)for(let h=0;h<128;h++){const v=(h+f)%128/128*u.length,m=Math.floor(v)%u.length,p=v-Math.floor(v),b=(m+1)%u.length,S=p>.94?(p-.94)/.06:0,C=(f*128+h)*4;for(let x=0;x<3;x++)r.data[C+x]=u[m][x]*(1-S)+u[b][x]*S;r.data[C+3]=255}l.putImageData(r,0,0);const i=new hh(l.canvas);return i.wrapS=Ba,i.wrapT=Ba,i.colorSpace=wo,i.flipY=!1,i.anisotropy=4,i}function Wf(o){const l=new hh(o.canvas);return l.colorSpace=wo,l.flipY=!1,l.generateMipmaps=!0,l.minFilter=Yo,l.magFilter=ua,l.anisotropy=8,l.needsUpdate=!0,l}function iy(o){if(typeof document>"u")return null;const l=new TextDecoder("utf-8").decode(o).replace(/\0+$/,""),r={},u=[],i=[],f=[];for(const x of l.split(`
`)){if(!x)continue;const D=x.split("	");switch(D[0]){case"SHEET":r[D[1]]=ey(D);break;case"B":u.push({cell:+D[1],bays:+D[2],style:+D[3],font:+D[4],bg:D[5],fg:D[6],acc:D[7],mark:+D[8],text:D[9],sub:D[10]??""});break;case"V":i.push({cell:+D[1],style:+D[2],font:+D[3],bg:D[4],fg:D[5],acc:D[6],text:D[7]});break;case"C":f.push({cell:+D[1],style:+D[2],font:+D[3],colour:D[4],mark:+D[5],name:D[6]});break}}const h=r.board,v=r.vert,m=r.crown;if(!h||!v||!m)return null;const p=Iu(h.cols*h.cw,h.rows*h.ch),b=Iu(v.cols*v.cw,v.rows*v.ch),S=Iu(m.cw,m.rows*m.ch),C=sy();if(!p||!b||!S||!C)return null;for(const x of u)ny(p,x.cell%h.cols*h.cw,Math.floor(x.cell/h.cols)*h.ch,h.cw*x.bays,h.ch,x);for(const x of i)oy(b,x.cell%v.cols*v.cw,Math.floor(x.cell/v.cols)*v.ch,v.cw,v.ch,x);for(const x of f)ly(S,x.cell*m.ch,m.cw,m.ch,x);return{board:Wf(p),vert:Wf(b),crown:Wf(S),pole:C}}function Ss(o){if(typeof o!="string")return new Uint8Array(o.buffer,o.byteOffset,o.byteLength);const r=atob(o),u=new Uint8Array(r.length);for(let i=0;i<r.length;i+=1)u[i]=r.charCodeAt(i);return u}function Ju(o,l){if(o instanceof Float32Array){if(o.length!==l)throw new Error(`buffer is ${o.length} floats, expected ${l}`);return o}const r=Ss(o);if(r.length!==l*4)throw new Error(`buffer is ${r.length} bytes, expected ${l*4}`);const u=r.slice();return new Float32Array(u.buffer,0,l)}function ry(o,l){if(o instanceof Uint32Array){if(o.length!==l)throw new Error(`index buffer is ${o.length} entries, expected ${l}`);return o}const r=Ss(o);if(r.length!==l*4)throw new Error(`index buffer is ${r.length} bytes, expected ${l*4}`);const u=r.slice();return new Uint32Array(u.buffer,0,l)}const cy=[16722456,16757760,1638234],uy=[2755080,2761224,666134],fy=24;function hy(o,l=64){const r=new Map;for(const B of o)r.set(B.name,B);const u=[],i=[],f=new Map,h=new Set,v=new Set,m=new Map,p=B=>{const ae=f.get(B);if(ae)return ae;const I=r.get(B);if(!I)return null;const ge=new ma(Ss(I.data),I.width,I.height,$n);ge.wrapS=Ba,ge.wrapT=Ba,ge.anisotropy=16,ge.magFilter=ua,ge.minFilter=Yo,ge.generateMipmaps=!0,ge.colorSpace=wo,ge.flipY=!1;const Re=B.startsWith("facade/")||B.startsWith("cnshop/");return ge.repeat.set(1/I.tileWidthM,Re?1:1/I.tileHeightM),ge.needsUpdate=!0,f.set(B,ge),i.push(ge),ge},b=[],S=(B,ae)=>{const I=r.get(B);if(!I?.hasNormalSource)return null;const ge=`${B}#normal`,Re=f.get(ge);if(Re)return Re;const le=p(B);if(!le)return null;const{width:G,height:j}=I,me=new Uint8Array(G*j*4);for(let Z=0;Z<me.length;Z+=4)me[Z]=128,me[Z+1]=128,me[Z+2]=255,me[Z+3]=255;const X=new ma(me,G,j,$n);X.wrapS=Ba,X.wrapT=Ba,X.flipY=!1,X.anisotropy=8,X.minFilter=Yo,X.generateMipmaps=!0,X.repeat.copy(le.repeat),X.needsUpdate=!0,f.set(ge,X),i.push(X);let we=null;b.push(()=>{const Z=le.image.data;let T=255,F=0;for(let ue=3;ue<Z.length;ue+=4)T=Math.min(T,Z[ue]),F=Math.max(F,Z[ue]);const ce=F-T>8;we=new Float32Array(G*j);for(let ue=0;ue<we.length;ue+=1){const Ee=ue*4;we[ue]=ce?Z[Ee+3]/255:(Z[Ee]*.2126+Z[Ee+1]*.7152+Z[Ee+2]*.0722)/255}});const E=4;for(let Z=0;Z<E;Z+=1)b.push(()=>{const T=we;for(let F=Math.floor(j*Z/E);F<Math.floor(j*(Z+1)/E);F+=1){const ce=(F+j-1)%j*G,ue=F*G,Ee=(F+1)%j*G;for(let Te=0;Te<G;Te+=1){const Ue=(Te+G-1)%G,oe=(Te+1)%G,_e=(T[ue+Ue]-T[ue+oe])*ae,Ze=(T[ce+Te]-T[Ee+Te])*ae,Ye=1/Math.sqrt(_e*_e+Ze*Ze+1),Xe=(ue+Te)*4;me[Xe]=(_e*Ye*.5+.5)*255,me[Xe+1]=(Ze*Ye*.5+.5)*255,me[Xe+2]=(Ye*.5+.5)*255,me[Xe+3]=255}}Z===E-1&&(X.needsUpdate=!0,we=null)});return X},C=(B,ae)=>(m.set(B,ae),u.push(ae),ae),x=(B,ae,I)=>C(B,new Ke({color:I,...ae})),D=p("ground/asphalt");Lu(x("asphalt",{map:D??void 0,normalMap:S("ground/asphalt",2.4)??void 0,roughness:.88,metalness:0},D?16777215:6578782),"asphalt"),x("asphalt.pavement",{roughness:.95},7302246);const H=p("ground/paving");Lu(x("sidewalk",{map:H??void 0,normalMap:S("ground/paving",1.6)??void 0,roughness:.9},H?16777215:7237224),"paving"),Lu(x("parcel.paving",{map:H??void 0,roughness:.92},16777215),"paving"),Yf(x("parcel.yard",{roughness:.93},9209724),Wo.yard),Yf(x("parcel.lot",{roughness:.9},6447196),Wo.asphaltLot),Yf(x("parcel.parking",{roughness:.9},7302505),Wo.parking),x("kerb",{roughness:.86,polygonOffset:!0,polygonOffsetFactor:0,polygonOffsetUnits:-2},9341826),x("tactile.guide",{map:p("tactile/guide")??void 0,normalMap:S("tactile/guide",2)??void 0,roughness:.8},16777215),x("tactile.warn",{map:p("tactile/warn")??void 0,normalMap:S("tactile/warn",2)??void 0,roughness:.8,polygonOffset:!0,polygonOffsetFactor:0,polygonOffsetUnits:-2},16777215);const q={polygonOffset:!0,polygonOffsetFactor:0,polygonOffsetUnits:-2};x("treepit.edge",{roughness:.78,...q},10591894),x("treepit.grate",{roughness:.6,metalness:.25,...q},4539967),x("treepit.gravel",{roughness:1,...q},12170409),x("treepit.planter",{roughness:.35,metalness:.05},4935248);const P=p("ground/grass");for(const B of["block.ground","parcel.green","median.plant"])P2(Lu(x(B,{map:P??void 0,normalMap:S("ground/grass",1.2)??void 0,roughness:.95,...B==="block.ground"?{polygonOffset:!0,polygonOffsetFactor:2,polygonOffsetUnits:2}:{}},P?16777215:3029538),"grass"));{const B=x("water",{roughness:.2,metalness:0,envMapIntensity:.45,vertexColors:!0},16777215),ae={value:0};B.userData.uTime=ae,B.onBeforeCompile=I=>{I.uniforms.uWaterTime=ae,I.vertexShader=I.vertexShader.replace("#include <common>",`#include <common>
varying vec2 vWaterP;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vWaterP = position.xz;`),I.fragmentShader=I.fragmentShader.replace("#include <common>",`#include <common>
varying vec2 vWaterP;
uniform float uWaterTime;
float waterHash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
float waterNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(waterHash(i), waterHash(i + vec2(1.0, 0.0)), f.x),
             mix(waterHash(i + vec2(0.0, 1.0)), waterHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
// One travelling wave's slope, faded out once its phase aliases.
vec2 waterWave(vec2 p, vec2 dir, float k, float speed, float amp) {
  float phase = dot(p, dir) * k + uWaterTime * speed;
  float fw = max(abs(dFdx(phase)), abs(dFdy(phase)));
  float keep = 1.0 - smoothstep(0.7, 2.4, fw);
  return dir * (cos(phase) * amp * keep);
}`).replace("#include <roughnessmap_fragment>",`#include <roughnessmap_fragment>
float waterDist = length(vViewPosition);
roughnessFactor = clamp(roughnessFactor + smoothstep(60.0, 700.0, waterDist) * 0.22, 0.0, 1.0);`).replace("#include <normal_fragment_maps>",`#include <normal_fragment_maps>
{
  vec2 p = vWaterP;
  // Warp the lookup with low-frequency noise so no wave front is a straight,
  // grid-aligned line and the pattern does not repeat.
  p += 3.5 * (vec2(waterNoise(p * 0.031 + 3.1), waterNoise(p * 0.029 - 7.7)) - 0.5);
  vec2 g = vec2(0.0);
  g += waterWave(p, vec2(0.80, 0.60), 0.31, 0.62, 0.85);
  g += waterWave(p, vec2(-0.50, 0.86), 0.57, -0.83, 0.70);
  g += waterWave(p, vec2(0.96, -0.28), 0.93, 1.07, 0.48);
  g += waterWave(p, vec2(-0.31, -0.95), 1.61, -1.31, 0.30);
  g += waterWave(p, vec2(0.62, -0.78), 2.70, 1.67, 0.16);
  float far = 1.0 - smoothstep(120.0, 900.0, waterDist);
  vec3 gw = vec3(g.x, 0.0, g.y) * 0.032 * (0.25 + 0.75 * far);
  normal = normalize(normal - normalize(mat3(viewMatrix) * gw) * length(gw));
}`)}}Al(x("bank.earth",{roughness:.96,side:ct,vertexColors:!0},16777215),.5);const K=p("foliage/hedge");for(const B of["hedge","hedge.clipped"])x(B,{map:K??void 0,normalMap:S("foliage/hedge",2.6)??void 0,roughness:.88,flatShading:!1},K?16777215:2771487);for(const B of["red","pink","yellow"]){const ae=p(`flowers/${B}`);x(`flowers/${B}`,{map:ae??void 0,normalMap:S(`flowers/${B}`,1.8)??void 0,roughness:.8},ae?16777215:9054752)}x("barrier.concrete",{roughness:.9},10131598),x("rail.steel",{roughness:.42,metalness:.75,envMapIntensity:.9},9409688),x("bridge.concrete",{roughness:.9},8157810),Al(x("viaduct.concrete",{roughness:.86,vertexColors:!0},16777215),.7),x("viaduct.glass",{roughness:.12,metalness:.1,transparent:!0,opacity:.42,depthWrite:!1,side:ct,envMapIntensity:1.2},7317922),x("viaduct.planter",{roughness:.9,vertexColors:!0},16777215),x("bridge.stone",{roughness:.88,side:ct,vertexColors:!0},11840672),x("quay.stone",{map:H??void 0,normalMap:S("ground/paving",1.4)??void 0,roughness:.9,side:ct,vertexColors:!0},H?15920354:12893360);{const B=new Ke({color:15660274,roughness:1,metalness:0,transparent:!0,opacity:.2,depthWrite:!1,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2});C("water.foam",B)}x("bridge.steel",{roughness:.5,metalness:.6,side:ct,vertexColors:!0},10133670),x("wire",{roughness:.55,metalness:.3,side:ct},1316378),x("rail.ballast",{roughness:1},8091503),x("rail.sleeper",{roughness:.9},10197135),x("rail.platform",{roughness:.88,side:ct},11841960),x("station.wall",{roughness:.85},13486527),x("station.roof",{roughness:.6,metalness:.2,side:ct},13883096),x("station.glass",{roughness:.1,metalness:.1,envMapIntensity:1.2},5666956),x("train.body",{roughness:.32,metalness:.3,side:ct,vertexColors:!0},16777215),x("train.glass",{roughness:.08,metalness:.1,side:ct,envMapIntensity:1.3},1384231),x("train.stripe",{roughness:.4,side:ct},2051994);const U=(B,ae,I,ge=!1)=>C(B,new Ke({color:I?16777215:ae,map:I??void 0,alphaTest:ge?.5:0,transparent:!1,roughness:.45,metalness:0,emissive:new $t(I?0:ae).multiplyScalar(.06),polygonOffset:!0,polygonOffsetFactor:-4,polygonOffsetUnits:-4,envMapIntensity:.5}));U("marking.white",15263452,null),U("marking.yellow",14197788,null);for(const B of["marking.crosswalk","marking.dashed-3-5","marking.dashed-6-9"])U(B,15263452,p(B.replace(".","/")),!0);const z=p("roof");x("roof",{map:z??void 0,normalMap:S("roof",1.4)??void 0,roughness:.94},z?16777215:6973538);for(const B of["wheat","rice","rapeseed","corn","vegetables","fallow","orchard"])for(let ae=0;ae<3;ae+=1){const I=`field/${B}.${ae}`,ge=p(I);x(I,{map:ge??void 0,normalMap:S(I,1.6)??void 0,roughness:.96},ge?16777215:7039546)}x("field.ridge",{roughness:1},7035455);for(const[B,ae,I]of[["roof.tile",.82,0],["roof.terracotta",.78,0],["roof.steel",.45,.35]]){const ge=p(B);x(B,{map:ge??void 0,normalMap:S(B,2)??void 0,roughness:ae,metalness:I},ge?16777215:5917252)}for(let B=0;B<fy;B+=1){const ae=`facade/${B.toString().padStart(2,"0")}`,I=p(ae),ge=B>=8&&B<=15;if(x(ae,{map:I??void 0,roughness:ge?.28:.85,metalness:ge?.1:0,envMapIntensity:ge?1.25:.5,vertexColors:!0},I?16777215:11052188),ge||Al(m.get(ae)),ge){const Re=m.get(ae);Re.onBeforeCompile=le=>{le.vertexShader=le.vertexShader.replace("#include <common>",`#include <common>
varying vec3 vGlassW;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vGlassW = (modelMatrix * vec4(transformed, 1.0)).xyz;`),le.fragmentShader=le.fragmentShader.replace("#include <common>",`#include <common>
varying vec3 vGlassW;`).replace("#include <emissivemap_fragment>",`#include <emissivemap_fragment>
{
  float lum = dot(diffuseColor.rgb, vec3(0.3, 0.55, 0.15));
  float glassMask = smoothstep(0.012, 0.03, lum) * (1.0 - smoothstep(0.12, 0.3, lum));
  float floorId = floor(vGlassW.y / 3.6);
  float h1 = fract(sin(floorId * 12.9898 + floor((vGlassW.x + vGlassW.z) / 27.0) * 78.233) * 43758.5453);
  float h2 = fract(sin(dot(vec2(floorId, floor((vGlassW.x + vGlassW.z) / 3.9)), vec2(127.1, 311.7))) * 43758.5453);
  float variation = 0.72 + 0.34 * h1 + 0.16 * h2;
  float sky = 0.75 + 0.5 * smoothstep(0.0, 140.0, vGlassW.y);
  vec3 tint = pow(vColor.rgb, vec3(1.6));
  float fres = pow(1.0 - clamp(abs(dot(normalize(vViewPosition), normal)), 0.0, 1.0), 2.5);
  totalEmissiveRadiance += vec3(0.78, 0.82, 0.86) * tint * glassMask * variation * sky * (0.13 + 0.30 * fres);
}`)},Re.needsUpdate=!0}}const R=r.get("cnfacade/table");if(R){const B=Yg(Ss(R.data),R.width,R.height);i.push(B),C("cnfacade",Kg(B))}for(const B of o){if(!B.name.startsWith("cnshop/"))continue;const ae=x(B.name,{map:p(B.name)??void 0,roughness:.85,metalness:0,vertexColors:!0},16777215);kg(ae,"shop",B.tileWidthM)}Al(x("cnres.paint",{roughness:.9,vertexColors:!0},16777215)),Al(x("cnres.trim",{roughness:.6},14342869),.6),x("cnres.glass",{roughness:.06,metalness:0,envMapIntensity:1.4},11123910),x("solar.tubes",{roughness:.18,metalness:.2,envMapIntensity:1},1712944);for(const B of["shop","lobby","home"])x(`ground/${B}`,{map:p(`ground/${B}`)??void 0,roughness:.72},16777215);C("sign/shop",new Ke({map:p("sign/shop")??void 0,roughness:.4,emissiveMap:p("sign/shop")??void 0,emissive:16777215,emissiveIntensity:.55}));{const B=r.get("sign/manifest"),ae=B?iy(Ss(B.data)):null;if(ae){i.push(ae.board,ae.vert,ae.crown,ae.pole),C("sign/board",new Ke({map:ae.board,roughness:.45,emissiveMap:ae.board,emissive:16777215,emissiveIntensity:.22})),C("sign/vert",new Ke({map:ae.vert,roughness:.5,emissiveMap:ae.vert,emissive:16777215,emissiveIntensity:.22,side:ct})),C("sign/crown",new Ke({map:ae.crown,alphaTest:.4,roughness:.35,metalness:.3,emissiveMap:ae.crown,emissive:16777215,emissiveIntensity:.3}));const I=C("sign/pole",new Ke({map:ae.pole,roughness:.25,emissiveMap:ae.pole,emissive:16777215,emissiveIntensity:.35}));I.userData.poleMap=ae.pole}else for(const I of["sign/board","sign/vert","sign/crown","sign/pole"])x(I,{roughness:.6},13617856)}Al(x("trim.light",{roughness:.72},9341052),.6),x("trim.dark",{roughness:.45,metalness:.2},2764596),x("balcony.slab",{roughness:.82},8288880),Al(x("metal.ac",{roughness:.5,metalness:.05},12105390),.5),x("awning",{roughness:.55},1844780),Al(x("wall.render",{roughness:.88},10262154));const V=x("metal.clad",{roughness:.42,metalness:.3,vertexColors:!0},16777215);g1(V,.25,.55,0),Al(V,.5),g1(x("metal.door",{roughness:.5,metalness:.35},9409430),.11,.6,1),x("roof.skylight",{roughness:.3,metalness:0,emissive:2764592},13226966),x("pole.concrete",{roughness:.9},8420468),x("steel",{roughness:.38,metalness:.72,envMapIntensity:.9},10133670),x("furniture/lamp",{roughness:.55,metalness:.35},7633276),x("furniture/lamp.double",{roughness:.55,metalness:.35},7633276),x("furniture/lamp.deco",{roughness:.42,metalness:.55},2896179),x("furniture/lamp.globe",{roughness:.2,emissive:16773592,emissiveIntensity:.12},15855592),x("furniture/lantern",{roughness:.65,emissive:3802628,emissiveIntensity:.4},11804184),x("furniture/pole",{roughness:.9},8420468),x("furniture/bollard",{roughness:.5,metalness:.2},12106934),x("furniture/railing",{roughness:.45,metalness:.5},11054512),x("furniture/shelter",{roughness:.55,metalness:.25},7238778),x("furniture/sign.guide",{roughness:.45},1002646),x("furniture/sign.crossing",{roughness:.45},1002646),x("furniture/sign.roundabout",{roughness:.45},1002646),x("furniture/sign.glyph",{roughness:.4},15921902),x("signal.body",{roughness:.55,metalness:.35},1711649),x("asphalt.cycle",{roughness:.9,polygonOffset:!0,polygonOffsetFactor:0,polygonOffsetUnits:-2},7104868),x("furniture/bin",{roughness:.6,metalness:.05},10133666),x("furniture/kiosk",{roughness:.5,metalness:.2},9080982),x("furniture/gantry.steel",{roughness:.42,metalness:.7,envMapIntensity:.9},9607328),x("furniture/gantry.board",{roughness:.4,metalness:.1},1002646),x("car/body",{roughness:.3,metalness:.35,envMapIntensity:1},16777215),x("car/glass",{roughness:.12,metalness:.5,envMapIntensity:1.1},1844265),x("car/wheel",{roughness:.9},1052690),Cg(x("bark",{roughness:.95,vertexColors:!0},16777215));const J=(B,ae)=>{const I=p(ae);return C(B,new Ke({map:I??void 0,alphaTest:I?.42:0,side:ct,vertexColors:!0,roughness:.86,metalness:0,emissive:1451023}))};J("leaf","vegetation/leaf");const Q=p("vegetation/tuft");C("tuft",new Ke({map:Q??void 0,alphaTest:Q?.45:0,side:ct,vertexColors:!0,roughness:.92}));const ne=[];for(let B=0;B<3;B+=1){const ae=new ws(.17,10,8),I=new Ke({color:uy[B],emissive:cy[B],emissiveIntensity:0,roughness:.25});u.push(I);const ge=new Ka(ae,I,Math.max(1,l));ge.count=0,ge.frustumCulled=!1,ne.push(ge)}return{get(B){const ae=m.get(B);if(ae)return ae;const I=B.indexOf("#");if(I>0){const Re=B.slice(0,I).split("/"),le=B.slice(I+1),G=Re.length>=2?Re[1]:"";if(le==="leaf"&&p(`vegetation/leaf/${G}`))return J(B,`vegetation/leaf/${G}`);if(le==="mass")return C(B,new Ke({color:16777215,roughness:1,metalness:0,vertexColors:!0,emissive:1451023,emissiveIntensity:.5,alphaHash:!0,opacity:1}));if(le==="bark")return C(B,new Ke({color:16777215,roughness:.95,metalness:0,flatShading:!0,vertexColors:!0}))}h.add(B);const ge=new Dn({color:16711935});return u.push(ge),m.set(B,ge),ge},textures:i,lamps:ne,missing:h,uvMismatch:v,pending:b,dispose(){for(const B of u)B.dispose();for(const B of i)B.dispose();for(const B of ne)B.geometry.dispose()}}}function dy(o){const l=o.find(u=>u.name.startsWith("vegetation/leaf"));if(!l)return null;const r=new ma(Ss(l.data),l.width,l.height,$n);return r.magFilter=ua,r.minFilter=ua,r.generateMipmaps=!1,r.needsUpdate=!0,r}function my(o){if(typeof o!="string")return o.byteLength;const l=o.endsWith("==")?2:o.endsWith("=")?1:0;return Math.floor(o.length*3/4)-l}function*py(o,l){const r=Ju(o.positions,o.vertexCount*3);yield;let u;if(o.normalsSnorm){const v=Ss(o.normals),m=new Int8Array(v.buffer,v.byteOffset,o.vertexCount*3);u=new Float32Array(o.vertexCount*3);for(let p=0;p<u.length;p+=1)u[p]=Math.max(-1,m[p]/127)}else u=Ju(o.normals,o.vertexCount*3);yield;const i=o.colors?Ss(o.colors):null;let f=null;if(o.uvs){const v=o.vertexCount*2,m=my(o.uvs);Math.abs(m-v*4)>2?l.add(`${o.material}: ${o.vertexCount} verts need ${v*4} B of UV, got ${m} B`):f=Ju(o.uvs,v),yield}const h=ry(o.indices,o.triangleCount*3);return yield,{positions:r,normals:u,uvs:f,colors:i,indices:h}}function x1(o,l){const r=new sa;return r.setAttribute("position",new ya(o.positions,3)),l.alphaCutout&&(l.material==="tuft"||l.material.startsWith("leaf"))&&r.setAttribute("aLeafCard",new ya(new Float32Array(o.positions.length/3).fill(1),1)),r.setAttribute("normal",new ya(o.normals,3)),o.colors&&r.setAttribute("color",new ya(o.colors,4,!0)),o.uvs&&r.setAttribute("uv",new ya(o.uvs,2)),r.setIndex(new ya(o.indices,1)),r.computeBoundingSphere(),r.computeBoundingBox(),r}function*vy(o,l){const{positions:r,indices:u}=o,i=u.length/3,f=new Map;for(let p=0;p<i;p+=1){const b=u[p*3]*3,S=u[p*3+1]*3,C=u[p*3+2]*3,x=Math.floor((r[b]+r[S]+r[C])/(3*l)),D=Math.floor((r[b+2]+r[S+2]+r[C+2])/(3*l)),H=(x+2048)*4096+(D+2048);let q=f.get(H);q||(q=[],f.set(H,q)),q.push(p),(p&65535)===65535&&(yield)}if(yield,f.size<=1)return[o];const h=r.length/3,v=new Int32Array(h).fill(-1),m=[];for(const p of f.values()){const b=[],S=new Uint32Array(p.length*3);let C=0;for(const K of p)for(let U=0;U<3;U+=1){const z=u[K*3+U];let R=v[z];R<0&&(R=b.length,v[z]=R,b.push(z)),S[C]=R,C+=1}const x=b.length,D=new Float32Array(x*3),H=new Float32Array(x*3),q=o.uvs?new Float32Array(x*2):null,P=o.colors?new Uint8Array(x*4):null;for(let K=0;K<x;K+=1){const U=b[K];D[K*3]=r[U*3],D[K*3+1]=r[U*3+1],D[K*3+2]=r[U*3+2],H[K*3]=o.normals[U*3],H[K*3+1]=o.normals[U*3+1],H[K*3+2]=o.normals[U*3+2],q&&o.uvs&&(q[K*2]=o.uvs[U*2],q[K*2+1]=o.uvs[U*2+1]),P&&o.colors&&(P[K*4]=o.colors[U*4],P[K*4+1]=o.colors[U*4+1],P[K*4+2]=o.colors[U*4+2],P[K*4+3]=o.colors[U*4+3])}for(let K=0;K<x;K+=1)v[b[K]]=-1;m.push({positions:D,normals:H,uvs:q,colors:P,indices:S}),yield}return m}function gy(o,l){const r=o.boundingBox??new fh().setFromBufferAttribute(o.attributes.position),u=r.getCenter(new rt),i=r.getSize(new rt),f=new Dv(.5,1);f.scale(i.x,i.y,i.z),f.translate(u.x,u.y,u.z);const h=f.attributes.position.count;if(l.uvs&&f.setAttribute("uv",new ya(new Float32Array(h*2),2)),l.colors){let v=0,m=0,p=0;const b=l.colors.length/4,S=Math.max(1,Math.floor(b/512));let C=0;for(let H=0;H<b;H+=S)v+=l.colors[H*4],m+=l.colors[H*4+1],p+=l.colors[H*4+2],C+=1;const x=[Math.round(v/C),Math.round(m/C),Math.round(p/C)],D=new Uint8Array(h*4);for(let H=0;H<h;H+=1)D[H*4]=x[0],D[H*4+1]=x[1],D[H*4+2]=x[2],D[H*4+3]=255;f.setAttribute("color",new ya(D,4,!0))}return f.computeBoundingSphere(),f.computeBoundingBox(),f}const yy=new Set(["water","bank.earth","quay.stone"]),ip=-20;let w1=null;function xy(){return w1??=new Wa({vertexShader:"void main() { gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0); gl_Position.z = gl_Position.w; }",fragmentShader:"void main() { gl_FragColor = vec4(0.0); }",colorWrite:!1,depthWrite:!0,depthTest:!0,depthFunc:Uv,side:ct}),w1}function wy(o,l,r={}){const u=new ka;u.name="city";const i=[],f=[],h=new Map,v=new Map,m=[],p=[],b=r.chunkCellM??0,S=r.chunkMinTriangles??6e3;let C,x,D=!1,H=!1,q=0;const P=()=>{for(const R of f)R.dispose();for(const R of i)R.dispose();u.clear()},K={group:u,materials:l,instanced:h,prototypes:v,lamps:l.lamps,vehicleCount:o.traffic.agents.length,statics:m,lodSets:p,dispose:P};function*U(){const R=new Map;for(const B of o.instances)R.set(B.key,{count:B.count,floats:Ju(B.data,B.count*10)}),yield;for(;l.pending.length>0;)l.pending.shift()(),yield;const V=new W1,J=new $t;for(const B of o.meshes){if(q+=1,!B.vertexCount||!B.triangleCount)continue;const ae=yield*py(B,l.uvMismatch),I=l.get(B.material),ge=B.instanceOf;if(ge){const le=R.get(ge);if(!le){l.missing.add(`instance-list:${ge}`);continue}const G=x1(ae,B),j=new Ka(G,I,Math.max(1,le.count));j.name=`${ge}`,j.count=le.count,j.castShadow=B.castShadow,j.receiveShadow=B.receiveShadow;const me=B.material.includes("#")?B.material.split("#")[1]:"";j.userData.part=me;const X=!!I.vertexColors||ge.startsWith("car/"),we=le.floats,Z=r.lod===!0?new Float32Array(le.count*4):null,T=G.boundingSphere,F=4096;for(let ce=0;ce<le.count;ce+=F){const ue=Math.min(le.count,ce+F);for(let Ee=ce;Ee<ue;Ee+=1){const Te=Ee*10;if(V.position.set(we[Te],we[Te+1],we[Te+2]),V.rotation.set(0,we[Te+3],0),V.scale.set(we[Te+4],we[Te+5],we[Te+6]),V.updateMatrix(),j.setMatrixAt(Ee,V.matrix),X&&(J.setRGB(we[Te+7],we[Te+8],we[Te+9]),j.setColorAt(Ee,J)),Z){const Ue=Math.max(we[Te+4],we[Te+5],we[Te+6]);Z[Ee*4]=we[Te]+T.center.x*we[Te+4],Z[Ee*4+1]=we[Te+1]+T.center.y*we[Te+5],Z[Ee*4+2]=we[Te+2]+T.center.z*we[Te+6],Z[Ee*4+3]=T.radius*Ue}}ue<le.count&&(yield)}if(j.instanceMatrix.needsUpdate=!0,j.instanceColor&&(j.instanceColor.needsUpdate=!0),j.computeBoundingSphere(),j.computeBoundingBox(),h.set(ge,j),v.set(ge,{geometry:G,material:I}),f.push(j),u.add(j),Z){let ce=null;if(me==="mass"){const ue=gy(G,ae);i.push(ue),ce=new Ka(ue,I,Math.max(1,le.count)),ce.name=`${ge}#far`,ce.count=0,ce.castShadow=!1,ce.receiveShadow=B.receiveShadow,ce.visible=!1,f.push(ce),u.add(ce)}p.push({mesh:j,key:ge,part:me,count:le.count,matrices:j.instanceMatrix.array.slice(0,le.count*16),colors:j.instanceColor?j.instanceColor.array.slice(0,le.count*3):null,bounds:Z,far:ce})}yield;continue}const Re=b>0&&B.triangleCount>=S?yield*vy(ae,b):[ae];for(const le of Re){const G=x1(le,B),j=new xt(G,I);if(j.name=B.material,j.castShadow=B.castShadow,j.receiveShadow=B.receiveShadow,i.push(G),m.push(j),u.add(j),yy.has(B.material)){const me=new xt(G,xy());me.name="river.mask",me.renderOrder=ip+5,me.frustumCulled=j.frustumCulled,u.add(me)}yield}}const Q=h.get("car/body"),ne=h.get("car/glass");C=Q?Q.geometry.clone():void 0,x=ne?ne.geometry.clone():void 0,C&&i.push(C),x&&i.push(x),K.carBody=C,K.carGlass=x}const z=U();return{handles:K,get done(){return D},get progress(){return D?1:Math.min(.99,q/Math.max(1,o.meshes.length))},step(R){if(D||H)return D;const V=performance.now()+R;for(;;){if(z.next().done)return D=!0,!0;if(performance.now()>=V)return!1}},cancel(){H||D||(H=!0,z.return(void 0),P())}}}gg();function rp(o){let l=0,r=null,u=null,i=null,f=!1,h=0;return{get layer(){return i},get done(){return i!==null},get progress(){return i?1:l===0?0:.05+.9*(u?.progress??0)},step(v){if(i||f)return i!==null;const m=performance.now(),p=m+v;if(l===0){let S=0;for(const C of o.signals)S+=C.lamps.length;r=hy(o.textures,S),Va("cityMaterialsMs",performance.now()-m),l=1,u=wy(o,r,{chunkCellM:320,chunkMinTriangles:8e3,lod:!0})}l===1&&u&&u.step(Math.max(0,p-performance.now()))&&(l=2),l===2&&u&&r&&(i=by(o,r,u.handles));const b=performance.now()-m;return h+=b,Va("cityBuildSliceMaxMs",b),Va("cityBuildCpuMs",h),i!==null},cancel(){f||i||(f=!0,u?.cancel(),r?.dispose())}}}function by(o,l,r){const u=[],i=new ka;i.name="city-layer",i.add(r.group);const f=0,h=new W1,v=[];r.carBody&&f>0;const m=[[],[],[]];for(const K of o.signals)for(const U of K.lamps)(m[U.aspect]??m[0]).push(...U.position);r.lamps.forEach((K,U)=>{const z=m[U]??[],R=z.length/3;R>K.instanceMatrix.count&&u.push(`lamps of aspect ${U} exceed capacity`),K.count=Math.min(R,K.instanceMatrix.count);for(let J=0;J<K.count;J+=1)h.position.set(z[J*3],z[J*3+1],z[J*3+2]),h.rotation.set(0,0,0),h.updateMatrix(),K.setMatrixAt(J,h.matrix);K.instanceMatrix.needsUpdate=!0;const V=K.material;V.emissiveIntensity=U===3?0:3.2,i.add(K)});const p=Mg(r,o);p&&i.add(p.group);const b=r.statics.filter(K=>/^(parcel\.green|block\.ground)$/.test(K.name)),S=b.length>0?new I2(b,l.get("parcel.green")):null,C=new F2(r),x=new Mo,D=new Mo,H=new Ov,q=new rt,P=new rt;return{group:i,scene:o,problems:u,update(K,U=0,z){const R=l.get("water").userData.uTime;R&&(R.value=K);const V=l.get("sign/pole").userData?.poleMap;if(V&&(V.offset.y=-(K/3%1)),!z)return!1;z.updateMatrixWorld(),x.copy(i.matrixWorld).invert(),q.setFromMatrixPosition(z.matrixWorld).applyMatrix4(x),z.getWorldDirection(P).transformDirection(x),D.copy(z.matrixWorld).invert(),D.premultiply(z.projectionMatrix).multiply(i.matrixWorld),H.setFromProjectionMatrix(D);let J=C.update(q,P,H,U);return S?.update(q),p&&(p.setCasting(U<Ya.shadowTrees),J=p.update(q,6)||J),J},lodStats(){return{hiddenStatics:C.hiddenStatics,instances:C.keptInstances}},dispose(){i.removeFromParent(),p?.dispose(),S?.dispose();for(const K of v)K.dispose();r.dispose(),l.dispose()}}}const mi={name:"CopyShader",uniforms:{tDiffuse:{value:null},opacity:{value:1}},vertexShader:`

		varying vec2 vUv;

		void main() {

			vUv = uv;
			gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );

		}`,fragmentShader:`

		uniform float opacity;

		uniform sampler2D tDiffuse;

		varying vec2 vUv;

		void main() {

			vec4 texel = texture2D( tDiffuse, vUv );
			gl_FragColor = opacity * texel;


		}`};class vi{constructor(){this.isPass=!0,this.enabled=!0,this.needsSwap=!0,this.clear=!1,this.renderToScreen=!1}setSize(){}render(){console.error("THREE.Pass: .render() must be implemented in derived pass.")}dispose(){}}const My=new ah(-1,1,1,-1,0,1);class Sy extends sa{constructor(){super(),this.setAttribute("position",new st([-1,3,0,-1,-1,0,3,-1,0],3)),this.setAttribute("uv",new st([0,2,0,0,2,0],2))}}const Ty=new Sy;class l0{constructor(l){this._mesh=new xt(Ty,l)}dispose(){this._mesh.geometry.dispose()}render(l){l.render(this._mesh,My)}get material(){return this._mesh.material}set material(l){this._mesh.material=l}}class cp extends vi{constructor(l,r="tDiffuse"){super(),this.textureID=r,this.uniforms=null,this.material=null,l instanceof Wa?(this.uniforms=l.uniforms,this.material=l):l&&(this.uniforms=Ol.clone(l.uniforms),this.material=new Wa({name:l.name!==void 0?l.name:"unspecified",defines:Object.assign({},l.defines),uniforms:this.uniforms,vertexShader:l.vertexShader,fragmentShader:l.fragmentShader})),this._fsQuad=new l0(this.material)}render(l,r,u){this.uniforms[this.textureID]&&(this.uniforms[this.textureID].value=u.texture),this._fsQuad.material=this.material,this.renderToScreen?(l.setRenderTarget(null),this._fsQuad.render(l)):(l.setRenderTarget(r),this.clear&&l.clear(l.autoClearColor,l.autoClearDepth,l.autoClearStencil),this._fsQuad.render(l))}dispose(){this.material.dispose(),this._fsQuad.dispose()}}class b1 extends vi{constructor(l,r){super(),this.scene=l,this.camera=r,this.clear=!0,this.needsSwap=!1,this.inverse=!1}render(l,r,u){const i=l.getContext(),f=l.state;f.buffers.color.setMask(!1),f.buffers.depth.setMask(!1),f.buffers.color.setLocked(!0),f.buffers.depth.setLocked(!0);let h,v;this.inverse?(h=0,v=1):(h=1,v=0),f.buffers.stencil.setTest(!0),f.buffers.stencil.setOp(i.REPLACE,i.REPLACE,i.REPLACE),f.buffers.stencil.setFunc(i.ALWAYS,h,4294967295),f.buffers.stencil.setClear(v),f.buffers.stencil.setLocked(!0),l.setRenderTarget(u),this.clear&&l.clear(),l.render(this.scene,this.camera),l.setRenderTarget(r),this.clear&&l.clear(),l.render(this.scene,this.camera),f.buffers.color.setLocked(!1),f.buffers.depth.setLocked(!1),f.buffers.color.setMask(!0),f.buffers.depth.setMask(!0),f.buffers.stencil.setLocked(!1),f.buffers.stencil.setFunc(i.EQUAL,1,4294967295),f.buffers.stencil.setOp(i.KEEP,i.KEEP,i.KEEP),f.buffers.stencil.setLocked(!0)}}class zy extends vi{constructor(){super(),this.needsSwap=!1}render(l){l.state.buffers.stencil.setLocked(!1),l.state.buffers.stencil.setTest(!1)}}class Cy{constructor(l,r){if(this.renderer=l,this._pixelRatio=l.getPixelRatio(),r===void 0){const u=l.getSize(new Et);this._width=u.width,this._height=u.height,r=new Ms(this._width*this._pixelRatio,this._height*this._pixelRatio,{type:kl}),r.texture.name="EffectComposer.rt1"}else this._width=r.width,this._height=r.height;this.renderTarget1=r,this.renderTarget2=r.clone(),this.renderTarget2.texture.name="EffectComposer.rt2",this.writeBuffer=this.renderTarget1,this.readBuffer=this.renderTarget2,this.renderToScreen=!0,this.passes=[],this.copyPass=new cp(mi),this.copyPass.material.blending=Rl,this.timer=new kv}swapBuffers(){const l=this.readBuffer;this.readBuffer=this.writeBuffer,this.writeBuffer=l}addPass(l){this.passes.push(l),l.setSize(this._width*this._pixelRatio,this._height*this._pixelRatio)}insertPass(l,r){this.passes.splice(r,0,l),l.setSize(this._width*this._pixelRatio,this._height*this._pixelRatio)}removePass(l){const r=this.passes.indexOf(l);r!==-1&&this.passes.splice(r,1)}isLastEnabledPass(l){for(let r=l+1;r<this.passes.length;r++)if(this.passes[r].enabled)return!1;return!0}render(l){this.timer.update(),l===void 0&&(l=this.timer.getDelta());const r=this.renderer.getRenderTarget();let u=!1;for(let i=0,f=this.passes.length;i<f;i++){const h=this.passes[i];if(h.enabled!==!1){if(h.renderToScreen=this.renderToScreen&&this.isLastEnabledPass(i),h.render(this.renderer,this.writeBuffer,this.readBuffer,l,u),h.needsSwap){if(u){const v=this.renderer.getContext(),m=this.renderer.state.buffers.stencil;m.setFunc(v.NOTEQUAL,1,4294967295),this.copyPass.render(this.renderer,this.writeBuffer,this.readBuffer,l),m.setFunc(v.EQUAL,1,4294967295)}this.swapBuffers()}b1!==void 0&&(h instanceof b1?u=!0:h instanceof zy&&(u=!1))}}this.renderer.setRenderTarget(r)}reset(l){if(l===void 0){const r=this.renderer.getSize(new Et);this._pixelRatio=this.renderer.getPixelRatio(),this._width=r.width,this._height=r.height,l=this.renderTarget1.clone(),l.setSize(this._width*this._pixelRatio,this._height*this._pixelRatio)}this.renderTarget1.dispose(),this.renderTarget2.dispose(),this.renderTarget1=l,this.renderTarget2=l.clone(),this.writeBuffer=this.renderTarget1,this.readBuffer=this.renderTarget2}setSize(l,r){this._width=l,this._height=r;const u=this._width*this._pixelRatio,i=this._height*this._pixelRatio;this.renderTarget1.setSize(u,i),this.renderTarget2.setSize(u,i);for(let f=0;f<this.passes.length;f++)this.passes[f].setSize(u,i)}setPixelRatio(l){this._pixelRatio=l,this.setSize(this._width,this._height)}dispose(){this.renderTarget1.dispose(),this.renderTarget2.dispose(),this.copyPass.dispose()}}const ju={defines:{PERSPECTIVE_CAMERA:1,SAMPLES:16,NORMAL_VECTOR_TYPE:1,DEPTH_SWIZZLING:"x",SCREEN_SPACE_RADIUS:0,SCREEN_SPACE_RADIUS_SCALE:100,SCENE_CLIP_BOX:0},uniforms:{tNormal:{value:null},tDepth:{value:null},tNoise:{value:null},resolution:{value:new Et},cameraNear:{value:null},cameraFar:{value:null},cameraProjectionMatrix:{value:new Mo},cameraProjectionMatrixInverse:{value:new Mo},cameraWorldMatrix:{value:new Mo},radius:{value:.25},distanceExponent:{value:1},thickness:{value:1},distanceFallOff:{value:1},scale:{value:1},sceneBoxMin:{value:new rt(-1,-1,-1)},sceneBoxMax:{value:new rt(1,1,1)}},vertexShader:`

		varying vec2 vUv;

		void main() {
			vUv = uv;
			gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
		}`,fragmentShader:`
		varying vec2 vUv;
		uniform highp sampler2D tNormal;
		uniform highp sampler2D tDepth;
		uniform sampler2D tNoise;
		uniform vec2 resolution;
		uniform float cameraNear;
		uniform float cameraFar;
		uniform mat4 cameraProjectionMatrix;
		uniform mat4 cameraProjectionMatrixInverse;
		uniform mat4 cameraWorldMatrix;
		uniform float radius;
		uniform float distanceExponent;
		uniform float thickness;
		uniform float distanceFallOff;
		uniform float scale;
		#if SCENE_CLIP_BOX == 1
			uniform vec3 sceneBoxMin;
			uniform vec3 sceneBoxMax;
		#endif

		#include <common>
		#include <packing>

		#ifndef FRAGMENT_OUTPUT
		#define FRAGMENT_OUTPUT vec4(vec3(ao), 1.)
		#endif

		vec3 getViewPosition( const in vec2 screenPosition, const in float depth ) {
			#ifdef USE_REVERSED_DEPTH_BUFFER
				vec4 clipSpacePosition = vec4( vec2( screenPosition ) * 2.0 - 1.0, depth, 1.0 );
			#else
				vec4 clipSpacePosition = vec4( vec3( screenPosition, depth ) * 2.0 - 1.0, 1.0 );
			#endif
			vec4 viewSpacePosition = cameraProjectionMatrixInverse * clipSpacePosition;
			return viewSpacePosition.xyz / viewSpacePosition.w;
		}

		float getDepth(const vec2 uv) {
			return textureLod(tDepth, uv.xy, 0.0).DEPTH_SWIZZLING;
		}

		float fetchDepth(const ivec2 uv) {
			return texelFetch(tDepth, uv.xy, 0).DEPTH_SWIZZLING;
		}

		float getViewZ(const in float depth) {
			#if PERSPECTIVE_CAMERA == 1
				return perspectiveDepthToViewZ(depth, cameraNear, cameraFar);
			#else
				return orthographicDepthToViewZ(depth, cameraNear, cameraFar);
			#endif
		}

		vec3 computeNormalFromDepth(const vec2 uv) {
			vec2 size = vec2(textureSize(tDepth, 0));
			ivec2 p = ivec2(uv * size);
			float c0 = fetchDepth(p);
			float l2 = fetchDepth(p - ivec2(2, 0));
			float l1 = fetchDepth(p - ivec2(1, 0));
			float r1 = fetchDepth(p + ivec2(1, 0));
			float r2 = fetchDepth(p + ivec2(2, 0));
			float b2 = fetchDepth(p - ivec2(0, 2));
			float b1 = fetchDepth(p - ivec2(0, 1));
			float t1 = fetchDepth(p + ivec2(0, 1));
			float t2 = fetchDepth(p + ivec2(0, 2));
			float dl = abs((2.0 * l1 - l2) - c0);
			float dr = abs((2.0 * r1 - r2) - c0);
			float db = abs((2.0 * b1 - b2) - c0);
			float dt = abs((2.0 * t1 - t2) - c0);
			vec3 ce = getViewPosition(uv, c0).xyz;
			vec3 dpdx = (dl < dr) ? ce - getViewPosition((uv - vec2(1.0 / size.x, 0.0)), l1).xyz : -ce + getViewPosition((uv + vec2(1.0 / size.x, 0.0)), r1).xyz;
			vec3 dpdy = (db < dt) ? ce - getViewPosition((uv - vec2(0.0, 1.0 / size.y)), b1).xyz : -ce + getViewPosition((uv + vec2(0.0, 1.0 / size.y)), t1).xyz;
			return normalize(cross(dpdx, dpdy));
		}

		vec3 getViewNormal(const vec2 uv) {
			#if NORMAL_VECTOR_TYPE == 2
				return normalize(textureLod(tNormal, uv, 0.).rgb);
			#elif NORMAL_VECTOR_TYPE == 1
				return unpackRGBToNormal(textureLod(tNormal, uv, 0.).rgb);
			#else
				return computeNormalFromDepth(uv);
			#endif
		}

		vec3 getSceneUvAndDepth(vec3 sampleViewPos) {
			vec4 sampleClipPos = cameraProjectionMatrix * vec4(sampleViewPos, 1.);
			vec2 sampleUv = sampleClipPos.xy / sampleClipPos.w * 0.5 + 0.5;
			float sampleSceneDepth = getDepth(sampleUv);
			return vec3(sampleUv, sampleSceneDepth);
		}

		void main() {
			float depth = getDepth(vUv.xy);

			#ifdef USE_REVERSED_DEPTH_BUFFER
				if (depth <= 0.0) {
					discard;
					return;
				}
			#else
				if (depth >= 1.0) {
					discard;
					return;
				}
			#endif
			
			vec3 viewPos = getViewPosition(vUv, depth);
			vec3 viewNormal = getViewNormal(vUv);

			float radiusToUse = radius;
			float distanceFalloffToUse = thickness;
			#if SCREEN_SPACE_RADIUS == 1
				float radiusScale = getViewPosition(vec2(0.5 + float(SCREEN_SPACE_RADIUS_SCALE) / resolution.x, 0.0), depth).x;
				radiusToUse *= radiusScale;
				distanceFalloffToUse *= radiusScale;
			#endif

			#if SCENE_CLIP_BOX == 1
				vec3 worldPos = (cameraWorldMatrix * vec4(viewPos, 1.0)).xyz;
				float boxDistance = length(max(vec3(0.0), max(sceneBoxMin - worldPos, worldPos - sceneBoxMax)));
				if (boxDistance > radiusToUse) {
					discard;
					return;
				}
			#endif

			vec2 noiseResolution = vec2(textureSize(tNoise, 0));
			vec2 noiseUv = vUv * resolution / noiseResolution;
			vec4 noiseTexel = textureLod(tNoise, noiseUv, 0.0);
			vec3 randomVec = noiseTexel.xyz * 2.0 - 1.0;
			vec3 tangent = normalize(vec3(randomVec.xy, 0.));
			vec3 bitangent = vec3(-tangent.y, tangent.x, 0.);
			mat3 kernelMatrix = mat3(tangent, bitangent, vec3(0., 0., 1.));

			const int DIRECTIONS = SAMPLES < 30 ? 3 : 5;
			const int STEPS = (SAMPLES + DIRECTIONS - 1) / DIRECTIONS;
			float ao = 0.0;
			for (int i = 0; i < DIRECTIONS; ++i) {

				float angle = float(i) / float(DIRECTIONS) * PI;
				vec4 sampleDir = vec4(cos(angle), sin(angle), 0., 0.5 + 0.5 * noiseTexel.w);
				sampleDir.xyz = normalize(kernelMatrix * sampleDir.xyz);

				vec3 viewDir = normalize(-viewPos.xyz);
				vec3 sliceBitangent = normalize(cross(sampleDir.xyz, viewDir));
				vec3 sliceTangent = cross(sliceBitangent, viewDir);
				vec3 normalInSlice = normalize(viewNormal - sliceBitangent * dot(viewNormal, sliceBitangent));

				vec3 tangentToNormalInSlice = cross(normalInSlice, sliceBitangent);
				vec2 cosHorizons = vec2(dot(viewDir, tangentToNormalInSlice), dot(viewDir, -tangentToNormalInSlice));

				for (int j = 0; j < STEPS; ++j) {
					vec3 sampleViewOffset = sampleDir.xyz * radiusToUse * sampleDir.w * pow(float(j + 1) / float(STEPS), distanceExponent);

					vec3 sampleSceneUvDepth = getSceneUvAndDepth(viewPos + sampleViewOffset);
					vec3 sampleSceneViewPos = getViewPosition(sampleSceneUvDepth.xy, sampleSceneUvDepth.z);
					vec3 viewDelta = sampleSceneViewPos - viewPos;
					if (abs(viewDelta.z) < thickness) {
						float sampleCosHorizon = dot(viewDir, normalize(viewDelta));
						cosHorizons.x += max(0., (sampleCosHorizon - cosHorizons.x) * mix(1., 2. / float(j + 2), distanceFallOff));
					}

					sampleSceneUvDepth = getSceneUvAndDepth(viewPos - sampleViewOffset);
					sampleSceneViewPos = getViewPosition(sampleSceneUvDepth.xy, sampleSceneUvDepth.z);
					viewDelta = sampleSceneViewPos - viewPos;
					if (abs(viewDelta.z) < thickness) {
						float sampleCosHorizon = dot(viewDir, normalize(viewDelta));
						cosHorizons.y += max(0., (sampleCosHorizon - cosHorizons.y) * mix(1., 2. / float(j + 2), distanceFallOff));
					}
				}

				vec2 sinHorizons = sqrt(1. - cosHorizons * cosHorizons);
				float nx = dot(normalInSlice, sliceTangent);
				float ny = dot(normalInSlice, viewDir);
				float nxb = 1. / 2. * (acos(cosHorizons.y) - acos(cosHorizons.x) + sinHorizons.x * cosHorizons.x - sinHorizons.y * cosHorizons.y);
				float nyb = 1. / 2. * (2. - cosHorizons.x * cosHorizons.x - cosHorizons.y * cosHorizons.y);
				float occlusion = nx * nxb + ny * nyb;
				ao += occlusion;
			}

			ao = clamp(ao / float(DIRECTIONS), 0., 1.);
		#if SCENE_CLIP_BOX == 1
			ao = mix(ao, 1., smoothstep(0., radiusToUse, boxDistance));
		#endif
			ao = pow(ao, scale);

			gl_FragColor = FRAGMENT_OUTPUT;
		}`},qu={defines:{PERSPECTIVE_CAMERA:1},uniforms:{tDepth:{value:null},cameraNear:{value:null},cameraFar:{value:null}},vertexShader:`
		varying vec2 vUv;

		void main() {
			vUv = uv;
			gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
		}`,fragmentShader:`
		uniform sampler2D tDepth;
		uniform float cameraNear;
		uniform float cameraFar;
		varying vec2 vUv;

		#include <packing>

		float getLinearDepth( const in vec2 screenPosition ) {
			#if PERSPECTIVE_CAMERA == 1
				float fragCoordZ = texture2D( tDepth, screenPosition ).x;
				float viewZ = perspectiveDepthToViewZ( fragCoordZ, cameraNear, cameraFar );
				return viewZToOrthographicDepth( viewZ, cameraNear, cameraFar );
			#else
				return texture2D( tDepth, screenPosition ).x;
			#endif
		}

		void main() {
			float depth = getLinearDepth( vUv );
			gl_FragColor = vec4( vec3( 1.0 - depth ), 1.0 );

		}`},Xf={uniforms:{tDiffuse:{value:null},intensity:{value:1}},vertexShader:`
		varying vec2 vUv;

		void main() {
			vUv = uv;
			gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
		}`,fragmentShader:`
		uniform float intensity;
		uniform sampler2D tDiffuse;
		varying vec2 vUv;

		void main() {
			vec4 texel = texture2D( tDiffuse, vUv );
			gl_FragColor = vec4(mix(vec3(1.), texel.rgb, intensity), texel.a);
		}`};function _y(o=5){const l=Math.floor(o)%2===0?Math.floor(o)+1:Math.floor(o),r=Ay(l),u=r.length,i=new Uint8Array(u*4);for(let h=0;h<u;++h){const v=r[h],m=2*Math.PI*v/u,p=new rt(Math.cos(m),Math.sin(m),0).normalize();i[h*4]=(p.x*.5+.5)*255,i[h*4+1]=(p.y*.5+.5)*255,i[h*4+2]=127,i[h*4+3]=255}const f=new ma(i,l,l);return f.wrapS=Ba,f.wrapT=Ba,f.needsUpdate=!0,f}function Ay(o){const l=Math.floor(o)%2===0?Math.floor(o)+1:Math.floor(o),r=l*l,u=Array(r).fill(0);let i=Math.floor(l/2),f=l-1;for(let h=1;h<=r;){if(i===-1&&f===l?(f=l-2,i=0):(f===l&&(f=0),i<0&&(i=l-1)),u[i*l+f]!==0){f-=2,i++;continue}else u[i*l+f]=h++;f++,i--}return u}const Gu={defines:{SAMPLES:16,SAMPLE_VECTORS:up(16,2,1),NORMAL_VECTOR_TYPE:1,DEPTH_VALUE_SOURCE:0},uniforms:{tDiffuse:{value:null},tNormal:{value:null},tDepth:{value:null},tNoise:{value:null},resolution:{value:new Et},cameraProjectionMatrixInverse:{value:new Mo},lumaPhi:{value:5},depthPhi:{value:5},normalPhi:{value:5},radius:{value:4},index:{value:0}},vertexShader:`

		varying vec2 vUv;

		void main() {
			vUv = uv;
			gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
		}`,fragmentShader:`

		varying vec2 vUv;

		uniform sampler2D tDiffuse;
		uniform sampler2D tNormal;
		uniform sampler2D tDepth;
		uniform sampler2D tNoise;
		uniform vec2 resolution;
		uniform mat4 cameraProjectionMatrixInverse;
		uniform float lumaPhi;
		uniform float depthPhi;
		uniform float normalPhi;
		uniform float radius;
		uniform int index;

		#include <common>
		#include <packing>

		#ifndef SAMPLE_LUMINANCE
		#define SAMPLE_LUMINANCE dot(vec3(0.2125, 0.7154, 0.0721), a)
		#endif

		#ifndef FRAGMENT_OUTPUT
		#define FRAGMENT_OUTPUT vec4(denoised, 1.)
		#endif

		float getLuminance(const in vec3 a) {
			return SAMPLE_LUMINANCE;
		}

		const vec3 poissonDisk[SAMPLES] = SAMPLE_VECTORS;

		vec3 getViewPosition( const in vec2 screenPosition, const in float depth ) {
			#ifdef USE_REVERSED_DEPTH_BUFFER
				vec4 clipSpacePosition = vec4( vec2( screenPosition ) * 2.0 - 1.0, depth, 1.0 );
			#else
				vec4 clipSpacePosition = vec4( vec3( screenPosition, depth ) * 2.0 - 1.0, 1.0 );
			#endif
			vec4 viewSpacePosition = cameraProjectionMatrixInverse * clipSpacePosition;
			return viewSpacePosition.xyz / viewSpacePosition.w;
		}

		float getDepth(const vec2 uv) {
		#if DEPTH_VALUE_SOURCE == 1
			return textureLod(tDepth, uv.xy, 0.0).a;
		#else
			return textureLod(tDepth, uv.xy, 0.0).r;
		#endif
		}

		float fetchDepth(const ivec2 uv) {
			#if DEPTH_VALUE_SOURCE == 1
				return texelFetch(tDepth, uv.xy, 0).a;
			#else
				return texelFetch(tDepth, uv.xy, 0).r;
			#endif
		}

		vec3 computeNormalFromDepth(const vec2 uv) {
			vec2 size = vec2(textureSize(tDepth, 0));
			ivec2 p = ivec2(uv * size);
			float c0 = fetchDepth(p);
			float l2 = fetchDepth(p - ivec2(2, 0));
			float l1 = fetchDepth(p - ivec2(1, 0));
			float r1 = fetchDepth(p + ivec2(1, 0));
			float r2 = fetchDepth(p + ivec2(2, 0));
			float b2 = fetchDepth(p - ivec2(0, 2));
			float b1 = fetchDepth(p - ivec2(0, 1));
			float t1 = fetchDepth(p + ivec2(0, 1));
			float t2 = fetchDepth(p + ivec2(0, 2));
			float dl = abs((2.0 * l1 - l2) - c0);
			float dr = abs((2.0 * r1 - r2) - c0);
			float db = abs((2.0 * b1 - b2) - c0);
			float dt = abs((2.0 * t1 - t2) - c0);
			vec3 ce = getViewPosition(uv, c0).xyz;
			vec3 dpdx = (dl < dr) ?  ce - getViewPosition((uv - vec2(1.0 / size.x, 0.0)), l1).xyz
									: -ce + getViewPosition((uv + vec2(1.0 / size.x, 0.0)), r1).xyz;
			vec3 dpdy = (db < dt) ?  ce - getViewPosition((uv - vec2(0.0, 1.0 / size.y)), b1).xyz
									: -ce + getViewPosition((uv + vec2(0.0, 1.0 / size.y)), t1).xyz;
			return normalize(cross(dpdx, dpdy));
		}

		vec3 getViewNormal(const vec2 uv) {
		#if NORMAL_VECTOR_TYPE == 2
			return normalize(textureLod(tNormal, uv, 0.).rgb);
		#elif NORMAL_VECTOR_TYPE == 1
			return unpackRGBToNormal(textureLod(tNormal, uv, 0.).rgb);
		#else
			return computeNormalFromDepth(uv);
		#endif
		}

		void denoiseSample(in vec3 center, in vec3 viewNormal, in vec3 viewPos, in vec2 sampleUv, inout vec3 denoised, inout float totalWeight) {
			vec4 sampleTexel = textureLod(tDiffuse, sampleUv, 0.0);
			float sampleDepth = getDepth(sampleUv);
			vec3 sampleNormal = getViewNormal(sampleUv);
			vec3 neighborColor = sampleTexel.rgb;
			vec3 viewPosSample = getViewPosition(sampleUv, sampleDepth);

			float normalDiff = dot(viewNormal, sampleNormal);
			float normalSimilarity = pow(max(normalDiff, 0.), normalPhi);
			float lumaDiff = abs(getLuminance(neighborColor) - getLuminance(center));
			float lumaSimilarity = max(1.0 - lumaDiff / lumaPhi, 0.0);
			float depthDiff = abs(dot(viewPos - viewPosSample, viewNormal));
			float depthSimilarity = max(1. - depthDiff / depthPhi, 0.);
			float w = lumaSimilarity * depthSimilarity * normalSimilarity;

			denoised += w * neighborColor;
			totalWeight += w;
		}

		void main() {
			float depth = getDepth(vUv.xy);
			vec3 viewNormal = getViewNormal(vUv);
			if (depth == 1. || dot(viewNormal, viewNormal) == 0.) {
				discard;
				return;
			}
			vec4 texel = textureLod(tDiffuse, vUv, 0.0);
			vec3 center = texel.rgb;
			vec3 viewPos = getViewPosition(vUv, depth);

			vec2 noiseResolution = vec2(textureSize(tNoise, 0));
			vec2 noiseUv = vUv * resolution / noiseResolution;
			vec4 noiseTexel = textureLod(tNoise, noiseUv, 0.0);
      		vec2 noiseVec = vec2(sin(noiseTexel[index % 4] * 2. * PI), cos(noiseTexel[index % 4] * 2. * PI));
    		mat2 rotationMatrix = mat2(noiseVec.x, -noiseVec.y, noiseVec.x, noiseVec.y);

			float totalWeight = 1.0;
			vec3 denoised = texel.rgb;
			for (int i = 0; i < SAMPLES; i++) {
				vec3 sampleDir = poissonDisk[i];
				vec2 offset = rotationMatrix * (sampleDir.xy * (1. + sampleDir.z * (radius - 1.)) / resolution);
				vec2 sampleUv = vUv + offset;
				denoiseSample(center, viewNormal, viewPos, sampleUv, denoised, totalWeight);
			}

			if (totalWeight > 0.) {
				denoised /= totalWeight;
			}
			gl_FragColor = FRAGMENT_OUTPUT;
		}`};function up(o,l,r){const u=Ey(o,l,r);let i="vec3[SAMPLES](";for(let f=0;f<o;f++){const h=u[f];i+=`vec3(${h.x}, ${h.y}, ${h.z})${f<o-1?",":")"}`}return i}function Ey(o,l,r){const u=[];for(let i=0;i<o;i++){const f=2*Math.PI*l*i/o,h=Math.pow(i/(o-1),r);u.push(new rt(Math.cos(f),Math.sin(f),h))}return u}class Ry{constructor(l=Math){this.grad3=[[1,1,0],[-1,1,0],[1,-1,0],[-1,-1,0],[1,0,1],[-1,0,1],[1,0,-1],[-1,0,-1],[0,1,1],[0,-1,1],[0,1,-1],[0,-1,-1]],this.grad4=[[0,1,1,1],[0,1,1,-1],[0,1,-1,1],[0,1,-1,-1],[0,-1,1,1],[0,-1,1,-1],[0,-1,-1,1],[0,-1,-1,-1],[1,0,1,1],[1,0,1,-1],[1,0,-1,1],[1,0,-1,-1],[-1,0,1,1],[-1,0,1,-1],[-1,0,-1,1],[-1,0,-1,-1],[1,1,0,1],[1,1,0,-1],[1,-1,0,1],[1,-1,0,-1],[-1,1,0,1],[-1,1,0,-1],[-1,-1,0,1],[-1,-1,0,-1],[1,1,1,0],[1,1,-1,0],[1,-1,1,0],[1,-1,-1,0],[-1,1,1,0],[-1,1,-1,0],[-1,-1,1,0],[-1,-1,-1,0]],this.p=[];for(let r=0;r<256;r++)this.p[r]=Math.floor(l.random()*256);this.perm=[];for(let r=0;r<512;r++)this.perm[r]=this.p[r&255];this.simplex=[[0,1,2,3],[0,1,3,2],[0,0,0,0],[0,2,3,1],[0,0,0,0],[0,0,0,0],[0,0,0,0],[1,2,3,0],[0,2,1,3],[0,0,0,0],[0,3,1,2],[0,3,2,1],[0,0,0,0],[0,0,0,0],[0,0,0,0],[1,3,2,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[1,2,0,3],[0,0,0,0],[1,3,0,2],[0,0,0,0],[0,0,0,0],[0,0,0,0],[2,3,0,1],[2,3,1,0],[1,0,2,3],[1,0,3,2],[0,0,0,0],[0,0,0,0],[0,0,0,0],[2,0,3,1],[0,0,0,0],[2,1,3,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[2,0,1,3],[0,0,0,0],[0,0,0,0],[0,0,0,0],[3,0,1,2],[3,0,2,1],[0,0,0,0],[3,1,2,0],[2,1,0,3],[0,0,0,0],[0,0,0,0],[0,0,0,0],[3,1,0,2],[0,0,0,0],[3,2,0,1],[3,2,1,0]]}noise(l,r){let u,i,f;const h=.5*(Math.sqrt(3)-1),v=(l+r)*h,m=Math.floor(l+v),p=Math.floor(r+v),b=(3-Math.sqrt(3))/6,S=(m+p)*b,C=m-S,x=p-S,D=l-C,H=r-x;let q,P;D>H?(q=1,P=0):(q=0,P=1);const K=D-q+b,U=H-P+b,z=D-1+2*b,R=H-1+2*b,V=m&255,J=p&255,Q=this.perm[V+this.perm[J]]%12,ne=this.perm[V+q+this.perm[J+P]]%12,B=this.perm[V+1+this.perm[J+1]]%12;let ae=.5-D*D-H*H;ae<0?u=0:(ae*=ae,u=ae*ae*this._dot(this.grad3[Q],D,H));let I=.5-K*K-U*U;I<0?i=0:(I*=I,i=I*I*this._dot(this.grad3[ne],K,U));let ge=.5-z*z-R*R;return ge<0?f=0:(ge*=ge,f=ge*ge*this._dot(this.grad3[B],z,R)),70*(u+i+f)}noise3d(l,r,u){let i,f,h,v;const p=(l+r+u)*.3333333333333333,b=Math.floor(l+p),S=Math.floor(r+p),C=Math.floor(u+p),x=1/6,D=(b+S+C)*x,H=b-D,q=S-D,P=C-D,K=l-H,U=r-q,z=u-P;let R,V,J,Q,ne,B;K>=U?U>=z?(R=1,V=0,J=0,Q=1,ne=1,B=0):K>=z?(R=1,V=0,J=0,Q=1,ne=0,B=1):(R=0,V=0,J=1,Q=1,ne=0,B=1):U<z?(R=0,V=0,J=1,Q=0,ne=1,B=1):K<z?(R=0,V=1,J=0,Q=0,ne=1,B=1):(R=0,V=1,J=0,Q=1,ne=1,B=0);const ae=K-R+x,I=U-V+x,ge=z-J+x,Re=K-Q+2*x,le=U-ne+2*x,G=z-B+2*x,j=K-1+3*x,me=U-1+3*x,X=z-1+3*x,we=b&255,E=S&255,Z=C&255,T=this.perm[we+this.perm[E+this.perm[Z]]]%12,F=this.perm[we+R+this.perm[E+V+this.perm[Z+J]]]%12,ce=this.perm[we+Q+this.perm[E+ne+this.perm[Z+B]]]%12,ue=this.perm[we+1+this.perm[E+1+this.perm[Z+1]]]%12;let Ee=.6-K*K-U*U-z*z;Ee<0?i=0:(Ee*=Ee,i=Ee*Ee*this._dot3(this.grad3[T],K,U,z));let Te=.6-ae*ae-I*I-ge*ge;Te<0?f=0:(Te*=Te,f=Te*Te*this._dot3(this.grad3[F],ae,I,ge));let Ue=.6-Re*Re-le*le-G*G;Ue<0?h=0:(Ue*=Ue,h=Ue*Ue*this._dot3(this.grad3[ce],Re,le,G));let oe=.6-j*j-me*me-X*X;return oe<0?v=0:(oe*=oe,v=oe*oe*this._dot3(this.grad3[ue],j,me,X)),32*(i+f+h+v)}noise4d(l,r,u,i){const f=this.grad4,h=this.simplex,v=this.perm,m=(Math.sqrt(5)-1)/4,p=(5-Math.sqrt(5))/20;let b,S,C,x,D;const H=(l+r+u+i)*m,q=Math.floor(l+H),P=Math.floor(r+H),K=Math.floor(u+H),U=Math.floor(i+H),z=(q+P+K+U)*p,R=q-z,V=P-z,J=K-z,Q=U-z,ne=l-R,B=r-V,ae=u-J,I=i-Q,ge=ne>B?32:0,Re=ne>ae?16:0,le=B>ae?8:0,G=ne>I?4:0,j=B>I?2:0,me=ae>I?1:0,X=ge+Re+le+G+j+me,we=h[X][0]>=3?1:0,E=h[X][1]>=3?1:0,Z=h[X][2]>=3?1:0,T=h[X][3]>=3?1:0,F=h[X][0]>=2?1:0,ce=h[X][1]>=2?1:0,ue=h[X][2]>=2?1:0,Ee=h[X][3]>=2?1:0,Te=h[X][0]>=1?1:0,Ue=h[X][1]>=1?1:0,oe=h[X][2]>=1?1:0,_e=h[X][3]>=1?1:0,Ze=ne-we+p,Ye=B-E+p,Xe=ae-Z+p,tt=I-T+p,ot=ne-F+2*p,Ut=B-ce+2*p,fa=ae-ue+2*p,Ca=I-Ee+2*p,ta=ne-Te+3*p,Ha=B-Ue+3*p,Xa=ae-oe+3*p,an=I-_e+3*p,qt=ne-1+4*p,to=B-1+4*p,La=ae-1+4*p,ao=I-1+4*p,Pa=q&255,ut=P&255,xa=K&255,nn=U&255,on=v[Pa+v[ut+v[xa+v[nn]]]]%32,ln=v[Pa+we+v[ut+E+v[xa+Z+v[nn+T]]]]%32,wa=v[Pa+F+v[ut+ce+v[xa+ue+v[nn+Ee]]]]%32,sn=v[Pa+Te+v[ut+Ue+v[xa+oe+v[nn+_e]]]]%32,Vt=v[Pa+1+v[ut+1+v[xa+1+v[nn+1]]]]%32;let Qo=.6-ne*ne-B*B-ae*ae-I*I;Qo<0?b=0:(Qo*=Qo,b=Qo*Qo*this._dot4(f[on],ne,B,ae,I));let Io=.6-Ze*Ze-Ye*Ye-Xe*Xe-tt*tt;Io<0?S=0:(Io*=Io,S=Io*Io*this._dot4(f[ln],Ze,Ye,Xe,tt));let Jo=.6-ot*ot-Ut*Ut-fa*fa-Ca*Ca;Jo<0?C=0:(Jo*=Jo,C=Jo*Jo*this._dot4(f[wa],ot,Ut,fa,Ca));let yn=.6-ta*ta-Ha*Ha-Xa*Xa-an*an;yn<0?x=0:(yn*=yn,x=yn*yn*this._dot4(f[sn],ta,Ha,Xa,an));let xn=.6-qt*qt-to*to-La*La-ao*ao;return xn<0?D=0:(xn*=xn,D=xn*xn*this._dot4(f[Vt],qt,to,La,ao)),27*(b+S+C+x+D)}_dot(l,r,u){return l[0]*r+l[1]*u}_dot3(l,r,u,i){return l[0]*r+l[1]*u+l[2]*i}_dot4(l,r,u,i,f){return l[0]*r+l[1]*u+l[2]*i+l[3]*f}}class bo extends vi{constructor(l,r,u=512,i=512,f,h,v){super(),this.width=u,this.height=i,this.clear=!0,this.camera=r,this.scene=l,this.output=0,this._renderGBuffer=!0,this._visibilityCache=[],this.blendIntensity=1,this.pdRings=2,this.pdRadiusExponent=2,this.pdSamples=16,this.gtaoNoiseTexture=_y(),this.pdNoiseTexture=this._generateNoise(),this.gtaoRenderTarget=new Ms(this.width,this.height,{type:kl}),this.pdRenderTarget=this.gtaoRenderTarget.clone(),this.gtaoMaterial=new Wa({defines:Object.assign({},ju.defines),uniforms:Ol.clone(ju.uniforms),vertexShader:ju.vertexShader,fragmentShader:ju.fragmentShader,blending:Rl,depthTest:!1,depthWrite:!1}),this.gtaoMaterial.defines.PERSPECTIVE_CAMERA=this.camera.isPerspectiveCamera?1:0,this.gtaoMaterial.uniforms.tNoise.value=this.gtaoNoiseTexture,this.gtaoMaterial.uniforms.resolution.value.set(this.width,this.height),this.gtaoMaterial.uniforms.cameraNear.value=this.camera.near,this.gtaoMaterial.uniforms.cameraFar.value=this.camera.far,this.normalMaterial=new Bv,this.normalMaterial.blending=Rl,this.pdMaterial=new Wa({defines:Object.assign({},Gu.defines),uniforms:Ol.clone(Gu.uniforms),vertexShader:Gu.vertexShader,fragmentShader:Gu.fragmentShader,depthTest:!1,depthWrite:!1}),this.pdMaterial.uniforms.tDiffuse.value=this.gtaoRenderTarget.texture,this.pdMaterial.uniforms.tNoise.value=this.pdNoiseTexture,this.pdMaterial.uniforms.resolution.value.set(this.width,this.height),this.pdMaterial.uniforms.lumaPhi.value=10,this.pdMaterial.uniforms.depthPhi.value=2,this.pdMaterial.uniforms.normalPhi.value=3,this.pdMaterial.uniforms.radius.value=8,this.depthRenderMaterial=new Wa({defines:Object.assign({},qu.defines),uniforms:Ol.clone(qu.uniforms),vertexShader:qu.vertexShader,fragmentShader:qu.fragmentShader,blending:Rl}),this.depthRenderMaterial.uniforms.cameraNear.value=this.camera.near,this.depthRenderMaterial.uniforms.cameraFar.value=this.camera.far,this.copyMaterial=new Wa({uniforms:Ol.clone(mi.uniforms),vertexShader:mi.vertexShader,fragmentShader:mi.fragmentShader,transparent:!0,depthTest:!1,depthWrite:!1,blendSrc:e1,blendDst:ku,blendEquation:Ou,blendSrcAlpha:$m,blendDstAlpha:ku,blendEquationAlpha:Ou}),this.blendMaterial=new Wa({uniforms:Ol.clone(Xf.uniforms),vertexShader:Xf.vertexShader,fragmentShader:Xf.fragmentShader,transparent:!0,depthTest:!1,depthWrite:!1,blending:Hv,blendSrc:e1,blendDst:ku,blendEquation:Ou,blendSrcAlpha:$m,blendDstAlpha:ku,blendEquationAlpha:Ou}),this._fsQuad=new l0(null),this._originalClearColor=new $t,this.setGBuffer(f?f.depthTexture:void 0,f?f.normalTexture:void 0),h!==void 0&&this.updateGtaoMaterial(h),v!==void 0&&this.updatePdMaterial(v)}setSize(l,r){this.width=l,this.height=r,this.gtaoRenderTarget.setSize(l,r),this.normalRenderTarget.setSize(l,r),this.pdRenderTarget.setSize(l,r),this.gtaoMaterial.uniforms.resolution.value.set(l,r),this.gtaoMaterial.uniforms.cameraProjectionMatrix.value.copy(this.camera.projectionMatrix),this.gtaoMaterial.uniforms.cameraProjectionMatrixInverse.value.copy(this.camera.projectionMatrixInverse),this.pdMaterial.uniforms.resolution.value.set(l,r),this.pdMaterial.uniforms.cameraProjectionMatrixInverse.value.copy(this.camera.projectionMatrixInverse)}dispose(){this.gtaoNoiseTexture.dispose(),this.pdNoiseTexture.dispose(),this.normalRenderTarget.dispose(),this.gtaoRenderTarget.dispose(),this.pdRenderTarget.dispose(),this.normalMaterial.dispose(),this.pdMaterial.dispose(),this.copyMaterial.dispose(),this.depthRenderMaterial.dispose(),this._fsQuad.dispose()}get gtaoMap(){return this.pdRenderTarget.texture}setGBuffer(l,r){l!==void 0?(this.depthTexture=l,this.normalTexture=r,this._renderGBuffer=!1):(this.depthTexture=new Lv,this.depthTexture.format=jv,this.depthTexture.type=qv,this.normalRenderTarget=new Ms(this.width,this.height,{minFilter:Ts,magFilter:Ts,type:kl,depthTexture:this.depthTexture}),this.normalTexture=this.normalRenderTarget.texture,this._renderGBuffer=!0);const u=this.normalTexture?1:0,i=this.depthTexture===this.normalTexture?"w":"x";this.gtaoMaterial.defines.NORMAL_VECTOR_TYPE=u,this.gtaoMaterial.defines.DEPTH_SWIZZLING=i,this.gtaoMaterial.uniforms.tNormal.value=this.normalTexture,this.gtaoMaterial.uniforms.tDepth.value=this.depthTexture,this.pdMaterial.defines.NORMAL_VECTOR_TYPE=u,this.pdMaterial.defines.DEPTH_SWIZZLING=i,this.pdMaterial.uniforms.tNormal.value=this.normalTexture,this.pdMaterial.uniforms.tDepth.value=this.depthTexture,this.depthRenderMaterial.uniforms.tDepth.value=this.normalRenderTarget.depthTexture}setSceneClipBox(l){l?(this.gtaoMaterial.needsUpdate=this.gtaoMaterial.defines.SCENE_CLIP_BOX!==1,this.gtaoMaterial.defines.SCENE_CLIP_BOX=1,this.gtaoMaterial.uniforms.sceneBoxMin.value.copy(l.min),this.gtaoMaterial.uniforms.sceneBoxMax.value.copy(l.max)):(this.gtaoMaterial.needsUpdate=this.gtaoMaterial.defines.SCENE_CLIP_BOX===0,this.gtaoMaterial.defines.SCENE_CLIP_BOX=0)}updateGtaoMaterial(l){l.radius!==void 0&&(this.gtaoMaterial.uniforms.radius.value=l.radius),l.distanceExponent!==void 0&&(this.gtaoMaterial.uniforms.distanceExponent.value=l.distanceExponent),l.thickness!==void 0&&(this.gtaoMaterial.uniforms.thickness.value=l.thickness),l.distanceFallOff!==void 0&&(this.gtaoMaterial.uniforms.distanceFallOff.value=l.distanceFallOff,this.gtaoMaterial.needsUpdate=!0),l.scale!==void 0&&(this.gtaoMaterial.uniforms.scale.value=l.scale),l.samples!==void 0&&l.samples!==this.gtaoMaterial.defines.SAMPLES&&(this.gtaoMaterial.defines.SAMPLES=l.samples,this.gtaoMaterial.needsUpdate=!0),l.screenSpaceRadius!==void 0&&(l.screenSpaceRadius?1:0)!==this.gtaoMaterial.defines.SCREEN_SPACE_RADIUS&&(this.gtaoMaterial.defines.SCREEN_SPACE_RADIUS=l.screenSpaceRadius?1:0,this.gtaoMaterial.needsUpdate=!0)}updatePdMaterial(l){let r=!1;l.lumaPhi!==void 0&&(this.pdMaterial.uniforms.lumaPhi.value=l.lumaPhi),l.depthPhi!==void 0&&(this.pdMaterial.uniforms.depthPhi.value=l.depthPhi),l.normalPhi!==void 0&&(this.pdMaterial.uniforms.normalPhi.value=l.normalPhi),l.radius!==void 0&&l.radius!==this.radius&&(this.pdMaterial.uniforms.radius.value=l.radius),l.radiusExponent!==void 0&&l.radiusExponent!==this.pdRadiusExponent&&(this.pdRadiusExponent=l.radiusExponent,r=!0),l.rings!==void 0&&l.rings!==this.pdRings&&(this.pdRings=l.rings,r=!0),l.samples!==void 0&&l.samples!==this.pdSamples&&(this.pdSamples=l.samples,r=!0),r&&(this.pdMaterial.defines.SAMPLES=this.pdSamples,this.pdMaterial.defines.SAMPLE_VECTORS=up(this.pdSamples,this.pdRings,this.pdRadiusExponent),this.pdMaterial.needsUpdate=!0)}render(l,r,u){switch(this._renderGBuffer&&(this._overrideVisibility(),this._renderOverride(l,this.normalMaterial,this.normalRenderTarget,7829503,1),this._restoreVisibility()),this.gtaoMaterial.uniforms.cameraNear.value=this.camera.near,this.gtaoMaterial.uniforms.cameraFar.value=this.camera.far,this.gtaoMaterial.uniforms.cameraProjectionMatrix.value.copy(this.camera.projectionMatrix),this.gtaoMaterial.uniforms.cameraProjectionMatrixInverse.value.copy(this.camera.projectionMatrixInverse),this.gtaoMaterial.uniforms.cameraWorldMatrix.value.copy(this.camera.matrixWorld),this._renderPass(l,this.gtaoMaterial,this.gtaoRenderTarget,16777215,1),this.pdMaterial.uniforms.cameraProjectionMatrixInverse.value.copy(this.camera.projectionMatrixInverse),this._renderPass(l,this.pdMaterial,this.pdRenderTarget,16777215,1),this.output){case bo.OUTPUT.Off:break;case bo.OUTPUT.Diffuse:this.copyMaterial.uniforms.tDiffuse.value=u.texture,this.copyMaterial.blending=Rl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r);break;case bo.OUTPUT.AO:this.copyMaterial.uniforms.tDiffuse.value=this.gtaoRenderTarget.texture,this.copyMaterial.blending=Rl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r);break;case bo.OUTPUT.Denoise:this.copyMaterial.uniforms.tDiffuse.value=this.pdRenderTarget.texture,this.copyMaterial.blending=Rl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r);break;case bo.OUTPUT.Depth:this.depthRenderMaterial.uniforms.cameraNear.value=this.camera.near,this.depthRenderMaterial.uniforms.cameraFar.value=this.camera.far,this._renderPass(l,this.depthRenderMaterial,this.renderToScreen?null:r);break;case bo.OUTPUT.Normal:this.copyMaterial.uniforms.tDiffuse.value=this.normalRenderTarget.texture,this.copyMaterial.blending=Rl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r);break;case bo.OUTPUT.Default:this.copyMaterial.uniforms.tDiffuse.value=u.texture,this.copyMaterial.blending=Rl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r),this.blendMaterial.uniforms.intensity.value=this.blendIntensity,this.blendMaterial.uniforms.tDiffuse.value=this.pdRenderTarget.texture,this._renderPass(l,this.blendMaterial,this.renderToScreen?null:r);break;default:console.warn("THREE.GTAOPass: Unknown output type.")}}_renderPass(l,r,u,i,f){l.getClearColor(this._originalClearColor);const h=l.getClearAlpha(),v=l.autoClear;l.setRenderTarget(u),l.autoClear=!1,i!=null&&(l.setClearColor(i),l.setClearAlpha(f||0),l.clear()),this._fsQuad.material=r,this._fsQuad.render(l),l.autoClear=v,l.setClearColor(this._originalClearColor),l.setClearAlpha(h)}_renderOverride(l,r,u,i,f){l.getClearColor(this._originalClearColor);const h=l.getClearAlpha(),v=l.autoClear;l.setRenderTarget(u),l.autoClear=!1,i=r.clearColor||i,f=r.clearAlpha||f,i!=null&&(l.setClearColor(i),l.setClearAlpha(f||0),l.clear()),this.scene.overrideMaterial=r,l.render(this.scene,this.camera),this.scene.overrideMaterial=null,l.autoClear=v,l.setClearColor(this._originalClearColor),l.setClearAlpha(h)}_overrideVisibility(){const l=this.scene,r=this._visibilityCache;l.traverse(function(u){(u.isPoints||u.isLine||u.isLine2)&&u.visible&&(u.visible=!1,r.push(u))})}_restoreVisibility(){const l=this._visibilityCache;for(let r=0;r<l.length;r++)l[r].visible=!0;l.length=0}_generateNoise(l=64){const r=new Ry,u=l*l*4,i=new Uint8Array(u);for(let h=0;h<l;h++)for(let v=0;v<l;v++){const m=h,p=v;i[(h*l+v)*4]=(r.noise(m,p)*.5+.5)*255,i[(h*l+v)*4+1]=(r.noise(m+l,p)*.5+.5)*255,i[(h*l+v)*4+2]=(r.noise(m,p+l)*.5+.5)*255,i[(h*l+v)*4+3]=(r.noise(m+l,p+l)*.5+.5)*255}const f=new ma(i,l,l,$n,nh);return f.wrapS=Ba,f.wrapT=Ba,f.needsUpdate=!0,f}}bo.OUTPUT={Off:-1,Default:0,Diffuse:1,Depth:2,Normal:3,AO:4,Denoise:5};const Fu={name:"OutputShader",uniforms:{tDiffuse:{value:null},toneMappingExposure:{value:1}},vertexShader:`
		precision highp float;

		uniform mat4 modelViewMatrix;
		uniform mat4 projectionMatrix;

		attribute vec3 position;
		attribute vec2 uv;

		varying vec2 vUv;

		void main() {

			vUv = uv;
			gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );

		}`,fragmentShader:`

		precision highp float;

		uniform sampler2D tDiffuse;

		#include <tonemapping_pars_fragment>
		#include <colorspace_pars_fragment>

		varying vec2 vUv;

		void main() {

			gl_FragColor = texture2D( tDiffuse, vUv );

			// tone mapping

			#ifdef LINEAR_TONE_MAPPING

				gl_FragColor.rgb = LinearToneMapping( gl_FragColor.rgb );

			#elif defined( REINHARD_TONE_MAPPING )

				gl_FragColor.rgb = ReinhardToneMapping( gl_FragColor.rgb );

			#elif defined( CINEON_TONE_MAPPING )

				gl_FragColor.rgb = CineonToneMapping( gl_FragColor.rgb );

			#elif defined( ACES_FILMIC_TONE_MAPPING )

				gl_FragColor.rgb = ACESFilmicToneMapping( gl_FragColor.rgb );

			#elif defined( AGX_TONE_MAPPING )

				gl_FragColor.rgb = AgXToneMapping( gl_FragColor.rgb );

			#elif defined( NEUTRAL_TONE_MAPPING )

				gl_FragColor.rgb = NeutralToneMapping( gl_FragColor.rgb );

			#elif defined( CUSTOM_TONE_MAPPING )

				gl_FragColor.rgb = CustomToneMapping( gl_FragColor.rgb );

			#endif

			// color space

			#ifdef SRGB_TRANSFER

				gl_FragColor = sRGBTransferOETF( gl_FragColor );

			#endif

		}`};class Ny extends vi{constructor(){super(),this.isOutputPass=!0,this.uniforms=Ol.clone(Fu.uniforms),this.material=new Gv({name:Fu.name,uniforms:this.uniforms,vertexShader:Fu.vertexShader,fragmentShader:Fu.fragmentShader}),this._fsQuad=new l0(this.material),this._outputColorSpace=null,this._toneMapping=null}render(l,r,u){this.uniforms.tDiffuse.value=u.texture,this.uniforms.toneMappingExposure.value=l.toneMappingExposure,(this._outputColorSpace!==l.outputColorSpace||this._toneMapping!==l.toneMapping)&&(this._outputColorSpace=l.outputColorSpace,this._toneMapping=l.toneMapping,this.material.defines={},Fv.getTransfer(this._outputColorSpace)===Vv&&(this.material.defines.SRGB_TRANSFER=""),this._toneMapping===Kv?this.material.defines.LINEAR_TONE_MAPPING="":this._toneMapping===Yv?this.material.defines.REINHARD_TONE_MAPPING="":this._toneMapping===Wv?this.material.defines.CINEON_TONE_MAPPING="":this._toneMapping===X1?this.material.defines.ACES_FILMIC_TONE_MAPPING="":this._toneMapping===Xv?this.material.defines.AGX_TONE_MAPPING="":this._toneMapping===Pv?this.material.defines.NEUTRAL_TONE_MAPPING="":this._toneMapping===Zv&&(this.material.defines.CUSTOM_TONE_MAPPING=""),this.material.needsUpdate=!0),this.renderToScreen===!0?(l.setRenderTarget(null),this._fsQuad.render(l)):(l.setRenderTarget(r),this.clear&&l.clear(l.autoClearColor,l.autoClearDepth,l.autoClearStencil),this._fsQuad.render(l))}dispose(){this.material.dispose(),this._fsQuad.dispose()}}class Dy extends vi{constructor(l,r,u=null,i=null,f=null){super(),this.scene=l,this.camera=r,this.overrideMaterial=u,this.clearColor=i,this.clearAlpha=f,this.clear=!0,this.clearDepth=!1,this.needsSwap=!1,this.isRenderPass=!0,this._oldClearColor=new $t}render(l,r,u){const i=l.autoClear;l.autoClear=!1;let f,h;this.overrideMaterial!==null&&(h=this.scene.overrideMaterial,this.scene.overrideMaterial=this.overrideMaterial),this.clearColor!==null&&(l.getClearColor(this._oldClearColor),l.setClearColor(this.clearColor,l.getClearAlpha())),this.clearAlpha!==null&&(f=l.getClearAlpha(),l.setClearAlpha(this.clearAlpha)),this.clearDepth==!0&&l.clearDepth(),l.setRenderTarget(this.renderToScreen?null:u),this.clear===!0&&l.clear(l.autoClearColor,l.autoClearDepth,l.autoClearStencil),l.render(this.scene,this.camera),this.clearColor!==null&&l.setClearColor(this._oldClearColor),this.clearAlpha!==null&&l.setClearAlpha(f),this.overrideMaterial!==null&&(this.scene.overrideMaterial=h),l.autoClear=i}}const Uy={uniforms:{tDiffuse:{value:null},luminosityThreshold:{value:1},smoothWidth:{value:1},defaultColor:{value:new $t(0)},defaultOpacity:{value:0}},vertexShader:`

		varying vec2 vUv;

		void main() {

			vUv = uv;

			gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );

		}`,fragmentShader:`

		uniform sampler2D tDiffuse;
		uniform vec3 defaultColor;
		uniform float defaultOpacity;
		uniform float luminosityThreshold;
		uniform float smoothWidth;

		varying vec2 vUv;

		void main() {

			vec4 texel = texture2D( tDiffuse, vUv );

			float v = luminance( texel.xyz );

			vec4 outputColor = vec4( defaultColor.rgb, defaultOpacity );

			float alpha = smoothstep( luminosityThreshold, luminosityThreshold + smoothWidth, v );

			gl_FragColor = mix( outputColor, texel, alpha );

		}`};class cr extends vi{constructor(l,r=1,u,i){super(),this.strength=r,this.radius=u,this.threshold=i,this.resolution=l!==void 0?new Et(l.x,l.y):new Et(256,256),this.clearColor=new $t(0,0,0),this.needsSwap=!1,this.renderTargetsHorizontal=[],this.renderTargetsVertical=[],this.nMips=5;let f=Math.round(this.resolution.x/2),h=Math.round(this.resolution.y/2);this.renderTargetBright=new Ms(f,h,{type:kl}),this.renderTargetBright.texture.name="UnrealBloomPass.bright",this.renderTargetBright.texture.generateMipmaps=!1;for(let b=0;b<this.nMips;b++){const S=new Ms(f,h,{type:kl});S.texture.name="UnrealBloomPass.h"+b,S.texture.generateMipmaps=!1,this.renderTargetsHorizontal.push(S);const C=new Ms(f,h,{type:kl});C.texture.name="UnrealBloomPass.v"+b,C.texture.generateMipmaps=!1,this.renderTargetsVertical.push(C),f=Math.round(f/2),h=Math.round(h/2)}const v=Uy;this.highPassUniforms=Ol.clone(v.uniforms),this.highPassUniforms.luminosityThreshold.value=i,this.highPassUniforms.smoothWidth.value=.01,this.materialHighPassFilter=new Wa({uniforms:this.highPassUniforms,vertexShader:v.vertexShader,fragmentShader:v.fragmentShader}),this.separableBlurMaterials=[];const m=[6,10,14,18,22];f=Math.round(this.resolution.x/2),h=Math.round(this.resolution.y/2);for(let b=0;b<this.nMips;b++)this.separableBlurMaterials.push(this._getSeparableBlurMaterial(m[b])),this.separableBlurMaterials[b].uniforms.invSize.value=new Et(1/f,1/h),f=Math.round(f/2),h=Math.round(h/2);this.compositeMaterial=this._getCompositeMaterial(this.nMips),this.compositeMaterial.uniforms.blurTexture1.value=this.renderTargetsVertical[0].texture,this.compositeMaterial.uniforms.blurTexture2.value=this.renderTargetsVertical[1].texture,this.compositeMaterial.uniforms.blurTexture3.value=this.renderTargetsVertical[2].texture,this.compositeMaterial.uniforms.blurTexture4.value=this.renderTargetsVertical[3].texture,this.compositeMaterial.uniforms.blurTexture5.value=this.renderTargetsVertical[4].texture,this.compositeMaterial.uniforms.bloomStrength.value=r,this.compositeMaterial.uniforms.bloomRadius.value=.1;const p=[1,.8,.6,.4,.2];this.compositeMaterial.uniforms.bloomFactors.value=p,this.bloomTintColors=[new rt(1,1,1),new rt(1,1,1),new rt(1,1,1),new rt(1,1,1),new rt(1,1,1)],this.compositeMaterial.uniforms.bloomTintColors.value=this.bloomTintColors,this.copyUniforms=Ol.clone(mi.uniforms),this.blendMaterial=new Wa({uniforms:this.copyUniforms,vertexShader:mi.vertexShader,fragmentShader:mi.fragmentShader,premultipliedAlpha:!0,blending:Qv,depthTest:!1,depthWrite:!1,transparent:!0}),this._oldClearColor=new $t,this._oldClearAlpha=1,this._basic=new Dn,this._fsQuad=new l0(null)}dispose(){for(let l=0;l<this.renderTargetsHorizontal.length;l++)this.renderTargetsHorizontal[l].dispose();for(let l=0;l<this.renderTargetsVertical.length;l++)this.renderTargetsVertical[l].dispose();this.renderTargetBright.dispose();for(let l=0;l<this.separableBlurMaterials.length;l++)this.separableBlurMaterials[l].dispose();this.compositeMaterial.dispose(),this.blendMaterial.dispose(),this._basic.dispose(),this._fsQuad.dispose()}setSize(l,r){let u=Math.round(l/2),i=Math.round(r/2);this.renderTargetBright.setSize(u,i);for(let f=0;f<this.nMips;f++)this.renderTargetsHorizontal[f].setSize(u,i),this.renderTargetsVertical[f].setSize(u,i),this.separableBlurMaterials[f].uniforms.invSize.value=new Et(1/u,1/i),u=Math.round(u/2),i=Math.round(i/2)}render(l,r,u,i,f){l.getClearColor(this._oldClearColor),this._oldClearAlpha=l.getClearAlpha();const h=l.autoClear;l.autoClear=!1,l.setClearColor(this.clearColor,0),f&&l.state.buffers.stencil.setTest(!1),this.renderToScreen&&(this._fsQuad.material=this._basic,this._basic.map=u.texture,l.setRenderTarget(null),l.clear(),this._fsQuad.render(l)),this.highPassUniforms.tDiffuse.value=u.texture,this.highPassUniforms.luminosityThreshold.value=this.threshold,this._fsQuad.material=this.materialHighPassFilter,l.setRenderTarget(this.renderTargetBright),l.clear(),this._fsQuad.render(l);let v=this.renderTargetBright;for(let m=0;m<this.nMips;m++)this._fsQuad.material=this.separableBlurMaterials[m],this.separableBlurMaterials[m].uniforms.colorTexture.value=v.texture,this.separableBlurMaterials[m].uniforms.direction.value=cr.BlurDirectionX,l.setRenderTarget(this.renderTargetsHorizontal[m]),l.clear(),this._fsQuad.render(l),this.separableBlurMaterials[m].uniforms.colorTexture.value=this.renderTargetsHorizontal[m].texture,this.separableBlurMaterials[m].uniforms.direction.value=cr.BlurDirectionY,l.setRenderTarget(this.renderTargetsVertical[m]),l.clear(),this._fsQuad.render(l),v=this.renderTargetsVertical[m];this._fsQuad.material=this.compositeMaterial,this.compositeMaterial.uniforms.bloomStrength.value=this.strength,this.compositeMaterial.uniforms.bloomRadius.value=this.radius,this.compositeMaterial.uniforms.bloomTintColors.value=this.bloomTintColors,l.setRenderTarget(this.renderTargetsHorizontal[0]),l.clear(),this._fsQuad.render(l),this._fsQuad.material=this.blendMaterial,this.copyUniforms.tDiffuse.value=this.renderTargetsHorizontal[0].texture,f&&l.state.buffers.stencil.setTest(!0),this.renderToScreen?(l.setRenderTarget(null),this._fsQuad.render(l)):(l.setRenderTarget(u),this._fsQuad.render(l)),l.setClearColor(this._oldClearColor,this._oldClearAlpha),l.autoClear=h}_getSeparableBlurMaterial(l){const r=[],u=l/3;for(let i=0;i<l;i++)r.push(.39894*Math.exp(-.5*i*i/(u*u))/u);return new Wa({defines:{KERNEL_RADIUS:l},uniforms:{colorTexture:{value:null},invSize:{value:new Et(.5,.5)},direction:{value:new Et(.5,.5)},gaussianCoefficients:{value:r}},vertexShader:`

				varying vec2 vUv;

				void main() {

					vUv = uv;
					gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );

				}`,fragmentShader:`

				#include <common>

				varying vec2 vUv;

				uniform sampler2D colorTexture;
				uniform vec2 invSize;
				uniform vec2 direction;
				uniform float gaussianCoefficients[KERNEL_RADIUS];

				void main() {

					float weightSum = gaussianCoefficients[0];
					vec3 diffuseSum = texture2D( colorTexture, vUv ).rgb * weightSum;

					for ( int i = 1; i < KERNEL_RADIUS; i ++ ) {

						float x = float( i );
						float w = gaussianCoefficients[i];
						vec2 uvOffset = direction * invSize * x;
						vec3 sample1 = texture2D( colorTexture, vUv + uvOffset ).rgb;
						vec3 sample2 = texture2D( colorTexture, vUv - uvOffset ).rgb;
						diffuseSum += ( sample1 + sample2 ) * w;

					}

					gl_FragColor = vec4( diffuseSum, 1.0 );

				}`})}_getCompositeMaterial(l){return new Wa({defines:{NUM_MIPS:l},uniforms:{blurTexture1:{value:null},blurTexture2:{value:null},blurTexture3:{value:null},blurTexture4:{value:null},blurTexture5:{value:null},bloomStrength:{value:1},bloomFactors:{value:null},bloomTintColors:{value:null},bloomRadius:{value:0}},vertexShader:`

				varying vec2 vUv;

				void main() {

					vUv = uv;
					gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );

				}`,fragmentShader:`

				varying vec2 vUv;

				uniform sampler2D blurTexture1;
				uniform sampler2D blurTexture2;
				uniform sampler2D blurTexture3;
				uniform sampler2D blurTexture4;
				uniform sampler2D blurTexture5;
				uniform float bloomStrength;
				uniform float bloomRadius;
				uniform float bloomFactors[NUM_MIPS];
				uniform vec3 bloomTintColors[NUM_MIPS];

				float lerpBloomFactor( const in float factor ) {

					float mirrorFactor = 1.2 - factor;
					return mix( factor, mirrorFactor, bloomRadius );

				}

				void main() {

					// 3.0 for backwards compatibility with previous alpha-based intensity
					vec3 bloom = 3.0 * bloomStrength * (
						lerpBloomFactor( bloomFactors[ 0 ] ) * bloomTintColors[ 0 ] * texture2D( blurTexture1, vUv ).rgb +
						lerpBloomFactor( bloomFactors[ 1 ] ) * bloomTintColors[ 1 ] * texture2D( blurTexture2, vUv ).rgb +
						lerpBloomFactor( bloomFactors[ 2 ] ) * bloomTintColors[ 2 ] * texture2D( blurTexture3, vUv ).rgb +
						lerpBloomFactor( bloomFactors[ 3 ] ) * bloomTintColors[ 3 ] * texture2D( blurTexture4, vUv ).rgb +
						lerpBloomFactor( bloomFactors[ 4 ] ) * bloomTintColors[ 4 ] * texture2D( blurTexture5, vUv ).rgb
					);

					float bloomAlpha = max( bloom.r, max( bloom.g, bloom.b ) );
					gl_FragColor = vec4( bloom, bloomAlpha );

				}`})}}cr.BlurDirectionX=new Et(1,0);cr.BlurDirectionY=new Et(0,1);function Oy(){return new cp({name:"SanitizeShader",uniforms:{tDiffuse:{value:null}},vertexShader:`
      varying vec2 vUv;
      void main() {
        vUv = uv;
        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
      }`,fragmentShader:`
      uniform sampler2D tDiffuse;
      varying vec2 vUv;
      // By the exponent bits: GLSL ES does not require isnan or isinf to work,
      // and a compiler that assumes no NaNs folds both to false.
      bool bad(float v) { return (floatBitsToUint(v) & 0x7f800000u) == 0x7f800000u || abs(v) > 1.0e4; }
      void main() {
        vec4 c = texture2D(tDiffuse, vUv);
        if (bad(c.r) || bad(c.g) || bad(c.b) || bad(c.a)) c = vec4(0.0, 0.0, 0.0, 1.0);
        gl_FragColor = vec4(min(c.rgb, vec3(64.0)), c.a);
      }`})}const t0={value:null};function fp(o){t0.value||(t0.value=dy(o))}function ky(o){const l=o,r=l._overrideVisibility.bind(o);l._overrideVisibility=()=>{r(),l.scene.traverse(u=>{u.userData.noOcclusion&&u.visible&&(u.visible=!1,l._visibilityCache.push(u))})}}function By(o){const l={get value(){return t0.value?1:0}};o.onBeforeCompile=r=>{r.uniforms.foliageMask=t0,r.uniforms.foliageMaskOn=l,r.vertexShader=r.vertexShader.replace("#include <common>",`#include <common>
attribute float aLeafCard;
varying vec2 vCardUv;
varying float vLeafCard;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vCardUv = uv; vLeafCard = aLeafCard;`),r.fragmentShader=r.fragmentShader.replace("void main() {",`uniform sampler2D foliageMask;
uniform float foliageMaskOn;
varying vec2 vCardUv;
varying float vLeafCard;
void main() {`).replace("#include <normal_fragment_begin>",`#include <normal_fragment_begin>
if (foliageMaskOn > 0.5 && vLeafCard > 0.5 && texture2D(foliageMask, vCardUv).a < 0.45) discard;`)},o.needsUpdate=!0}const Zo=500,Hy=600,Ly=300,jy=2;function ys(o,l){const r=Math.sin(o.rotationRadians),u=Math.cos(o.rotationRadians),i=(l.x_km-o.origin.x_km)*1e3,f=(l.y_km-o.origin.y_km)*1e3,h=i*u+f*r,v=-i*r+f*u;return[Math.floor(h/Zo),Math.floor(v/Zo)]}function a0(o,l){return`${o},${l}`}function Pf(o,l,r){const u=Math.max(o*Zo-r.x,0,r.x-(o+1)*Zo),i=Math.max(l*Zo-r.z,0,r.z-(l+1)*Zo);return Math.hypot(u,i)}const ih={log:!1};class qy{constructor(l,r,u,i=Hy){this.index=l,this.load=r,this.onDetail=u,this.loadM=i}group=new ka;tiles=new Map;fetching=0;disposed=!1;lastCamera=null;update(l,r){if(this.disposed)return;this.lastCamera=l.clone();const u=performance.now(),i=Math.ceil((this.loadM+Zo)/Zo),f=Math.floor(l.x/Zo),h=Math.floor(l.z/Zo);for(let v=h-i;v<=h+i;v+=1)for(let m=f-i;m<=f+i;m+=1){if(Pf(m,v,l)>this.loadM)continue;const p=a0(m,v);this.tiles.has(p)||this.tiles.set(p,{column:m,row:v,state:"idle",scene:null,build:null,layer:null})}for(const v of this.tiles.values()){const m=Pf(v.column,v.row,l);if(m>this.loadM+Ly){this.drop(v);continue}if(v.state==="idle"&&m<=this.loadM&&this.fetching<jy&&this.fetch(v),v.state==="ready"){const p=performance.now();v.build=rp(v.scene),ih.log&&console.log(`[tiles] begin ${v.column},${v.row} ${(performance.now()-p).toFixed(0)}ms`),v.scene=null,v.state="building"}if(v.state==="building"&&v.build){const p=r-(performance.now()-u);if(p>0){const b=performance.now();v.build.step(p),ih.log&&console.log(`[tiles] step ${v.column},${v.row} ${(performance.now()-b).toFixed(1)}ms progress ${v.build.progress.toFixed(2)}`)}v.build.done&&this.mount(v,v.build.layer)}}}tick(l,r,u){for(const i of this.tiles.values())i.layer?.update(l,r,u)}summary(){const l={};for(const r of this.tiles.values())l[r.state]=(l[r.state]??0)+1;return l}pending(){let l=0;for(const r of this.tiles.values())(r.state==="fetching"||r.state==="ready"||r.state==="building"||r.state==="idle"&&this.lastCamera&&Pf(r.column,r.row,this.lastCamera)<=this.loadM)&&(l+=1);return l}dispose(){this.disposed=!0;for(const l of this.tiles.values())this.drop(l);this.tiles.clear()}fetch(l){l.state="fetching",this.fetching+=1,this.load(this.index,l.column,l.row).then(r=>{if(this.disposed)return;const u=r.meshes.length===0&&r.instances.every(i=>i.count===0);u||fp(r.textures),l.scene=u?null:r,l.state=u?"empty":"ready"}).catch(()=>{this.disposed||(l.state="failed")}).finally(()=>{this.fetching-=1})}mount(l,r){if(!r){l.state="empty",l.build=null;return}l.layer=r,l.build=null,l.state="mounted",this.group.add(r.group),this.onDetail(l.column,l.row,!0)}drop(l){l.build&&(l.build.cancel(),l.build=null),l.layer&&(this.group.remove(l.layer.group),l.layer.dispose(),l.layer=null,this.onDetail(l.column,l.row,!1)),(l.state==="mounted"||l.state==="building"||l.state==="ready")&&(l.state="idle"),l.scene=null}}const Mc={"xiang-zhang":{form:"Rounded",leaf:4218688,bark:7302505,height:14,crown:6.8,clear:.3},"yin-xing":{form:"Fan",leaf:6254918,bark:8157556,height:16,crown:6.2,clear:.44},"wu-tong":{form:"Open",leaf:6648159,bark:9078915,height:16,crown:7.5,clear:.45},"huai-shu":{form:"Rounded",leaf:5992013,bark:7828591,height:13,crown:6.8,clear:.34},"yu-shu":{form:"Rounded",leaf:5992276,bark:7433318,height:16,crown:7.2,clear:.36},"gui-hua":{form:"Oval",leaf:3626048,bark:7236456,height:6,crown:2.8,clear:.26},"xue-song":{form:"Layered",leaf:5991012,bark:6972767,height:16,crown:5.5,clear:.18},"liu-shu":{form:"Weeping",leaf:7438424,bark:6907230,height:11,crown:6.2,clear:.28},"song-shu":{form:"Irregular",leaf:5071686,bark:7103579,height:12.5,crown:5.2,clear:.35},"shui-shan":{form:"Conical",leaf:6909539,bark:7630185,height:24,crown:3,clear:.62},"ci-huai":{form:"Rounded",leaf:6320993,bark:7631212,height:13,crown:5.5,clear:.4},"rong-shu":{form:"Banyan",leaf:4613450,bark:7630955,height:14,crown:10,clear:.24},"tao-shu":{form:"Vase",leaf:5401688,bark:7959404,height:5,crown:3.5,clear:.3},"sha-shu":{form:"Conical",leaf:4349017,bark:6643804,height:18,crown:2.9,clear:.4},"he-huan":{form:"Umbrella",leaf:6976088,bark:7433575,height:11,crown:6.2,clear:.42},"yang-shu":{form:"Fastigiate",leaf:6583131,bark:7828591,height:21,crown:3,clear:.18},"bai-la":{form:"Rounded",leaf:5863760,bark:7565163,height:15,crown:6,clear:.36},"long-bai":{form:"Fastigiate",leaf:4549728,bark:7103582,height:10,crown:1.8,clear:.14},"zi-wei":{form:"Vase",leaf:5071676,bark:7564648,height:5.5,crown:2.8,clear:.2},"luan-shu":{form:"Rounded",leaf:7108928,bark:7827818,height:11,crown:5.8,clear:.32},"yu-lan":{form:"Oval",leaf:5336132,bark:9012866,height:8.5,crown:4.1,clear:.24},"bai-hua":{form:"Open",leaf:5796436,bark:9671052,height:10.5,crown:3.9,clear:.38},"hu-yang":{form:"Open",leaf:6779216,bark:8156009,height:11.5,crown:5.5,clear:.3},"sha-zao":{form:"Rounded",leaf:7107935,bark:7103837,height:6,crown:3.4,clear:.2},"zong-lu":{form:"Palm",leaf:5073989,bark:6641481,height:6,crown:3.1,clear:.8},"pu-kui":{form:"Palm",leaf:5600069,bark:7564387,height:9,crown:4.8,clear:.82},"da-wang-ye":{form:"Palm",leaf:5405003,bark:9012866,height:16,crown:8,clear:.84},"ye-zi":{form:"Palm",leaf:6127173,bark:7959402,height:12.5,crown:7,clear:.86},"mu-ma-huang":{form:"Conical",leaf:4940118,bark:7563615,height:17,crown:4,clear:.4},zhu:{form:"Bamboo",leaf:5402948,bark:7110733,height:8,crown:2.3,clear:.35},"lu-wei":{form:"Reed",leaf:6123339,bark:9011297,height:2.9,crown:1.2,clear:.1},"hong-shu":{form:"Shrub",leaf:3955012,bark:6904656,height:3.8,crown:2.2,clear:.25},"sha-ji":{form:"Shrub",leaf:6779227,bark:6642514,height:2.9,crown:1.8,clear:.12},"suo-suo":{form:"Shrub",leaf:6122071,bark:7828329,height:2.8,crown:2.1,clear:.1},"dian-zhuang":{form:"Cushion",leaf:4940621,bark:7761765,height:.4,crown:.5,clear:0}},M1=[["huai-shu",32],["wu-tong",28],["xiang-zhang",14],["yin-xing",9],["bai-la",9],["yang-shu",8]],Gy={ChinesePlane:"wu-tong",Ginkgo:"yin-xing",Cedar:"xue-song",Bamboo:"zhu",Willow:"liu-shu",chinesePlane:"wu-tong",ginkgo:"yin-xing",cedar:"xue-song",bamboo:"zhu",willow:"liu-shu"};function za(o,l=0,r=0){const u=Math.sin(o*12.9898+l*78.233+r*37.719)*43758.5453;return u-Math.floor(u)}function Fy(o){return o&&Gy[o]||"huai-shu"}function Vy(o,l,r){const u=o&&o.length?o.filter(([v])=>Mc[v]):M1,i=u.length?u:M1,f=i.reduce((v,[,m])=>v+m,0);let h=za(l,r,5.1)*f;for(const[v,m]of i){if(h<m)return v;h-=m}return i[i.length-1][0]}function S1(o){const l=Mc[o]??Mc["huai-shu"];return{height:l.height,crown:l.crown*.5}}function Xo(o,l,r){const u=o>>16&255,i=o>>8&255,f=o&255,h=v=>Math.max(0,Math.min(255,Math.round(v)));return h(u*l*(1+.1*r))<<16|h(i*l)<<8|h(f*l*(1-.16*r))}const Jn=(1+Math.sqrt(5))/2,hp=[[-1,Jn,0],[1,Jn,0],[-1,-Jn,0],[1,-Jn,0],[0,-1,Jn],[0,1,Jn],[0,-1,-Jn],[0,1,-Jn],[Jn,0,-1],[Jn,0,1],[-Jn,0,-1],[-Jn,0,1]].map(o=>{const l=Math.hypot(o[0],o[1],o[2]);return[o[0]/l,o[1]/l,o[2]/l]}),dp=[[0,11,5],[0,5,1],[0,1,7],[0,7,10],[0,10,11],[1,5,9],[5,11,4],[11,10,2],[10,7,6],[7,1,8],[3,9,4],[3,4,2],[3,2,6],[3,6,8],[3,8,9],[4,9,5],[2,4,11],[6,2,10],[8,6,7],[9,8,1]];function n0(o){const l=Math.hypot(o[0],o[1],o[2])||1;return[o[0]/l,o[1]/l,o[2]/l]}const Nl=hp.map(o=>o.slice()),mp=[];{const o=new Map,l=(r,u)=>{const i=r<u?`${r}_${u}`:`${u}_${r}`;let f=o.get(i);return f===void 0&&(Nl.push(n0([(Nl[r][0]+Nl[u][0])/2,(Nl[r][1]+Nl[u][1])/2,(Nl[r][2]+Nl[u][2])/2])),f=Nl.length-1,o.set(i,f)),f};for(const[r,u,i]of dp){const f=l(r,u),h=l(u,i),v=l(i,r);mp.push([r,f,v],[f,u,h],[v,h,i],[f,h,v])}}function yo(o,l,r,u,i,f,h,v,m,p,b,S=!0){const C=S?Nl:hp,x=S?mp:dp,D=C.map((P,K)=>{const U=za(v,K,1.7)*2-1,z=za(v,K,8.3)*2-1,R=1+b*(.7*U+(S?.35*z:0));return[l+P[0]*i*R,r+P[1]*f*R,u+P[2]*h*R]}),H=Math.max(i,f,h),q=C.map(P=>n0([P[0]/i*H,P[1]/f*H,P[2]/h*H]));for(let P=0;P<x.length;P+=1){const[K,U,z]=x[P],R=[(C[K][0]+C[U][0]+C[z][0])/3,(C[K][1]+C[U][1]+C[z][1])/3,(C[K][2]+C[U][2]+C[z][2])/3],V=n0([R[0]/i*H,R[1]/f*H,R[2]/h*H]),J=.46+.54*(.5+.5*V[1]),Q=.96+.08*za(v,P,3.3),ne=Xo(m,Q,(za(v,P,9.1)-.5)*.25);o.triSmooth?o.triSmooth(D[K],D[U],D[z],q[K],q[U],q[z],ne,p*J):o.tri(D[K],D[U],D[z],V,ne,p*J)}}function T1(o,l,r,u,i,f,h,v,m){const p=[];for(let C=0;C<m;C+=1){const x=C/m*Math.PI*2,D=f*(.78+.44*za(h,C,2.2));p.push([l+Math.cos(x)*D,r+(za(h,C,4.4)-.5)*(i-r)*.18,u+Math.sin(x)*D])}const b=[l,i,u],S=[l,r-(i-r)*.1,u];for(let C=0;C<m;C+=1){const x=p[C],D=p[(C+1)%m],H=(C+.5)/m*Math.PI*2,q=n0([Math.cos(H)*.7,.55,Math.sin(H)*.7]),P=.92+.16*za(h,C,6.6);o.tri(b,D,x,q,Xo(v,P*.78,.6),1),o.tri(S,x,D,[q[0],-.7,q[2]],Xo(v,.5,.6),.55)}}function z1(o,l,r,u,i,f,h){const m=[],p=[];for(let b=0;b<4;b+=1){const S=b/4*Math.PI*2+.4;m.push([l+Math.cos(S)*f,r,u+Math.sin(S)*f]),p.push([l+Math.cos(S)*f*.72,i,u+Math.sin(S)*f*.72])}for(let b=0;b<4;b+=1){const S=(b+1)%4,C=(b+.5)/4*Math.PI*2+.4,x=[Math.cos(C),0,Math.sin(C)];o.tri(m[b],m[S],p[S],x,h,.9),o.tri(m[b],p[S],p[b],x,h,.9)}}function Ky(o,l,r,u,i,f,h,v){const m=Mc[h]??Mc["huai-shu"],p=.84+.3*za(v,1.3,7.7),b=za(v,2.9,1.1)*2-1,S=Xo(m.leaf,1*p,b),C=Xo(m.bark,.9,0),x=Math.floor(v*1e6),D=Math.min(.7,Math.max(.05,m.clear)),H=r+i*D,q=i*(1-D),P=Math.max(f*.09,i*.022),K=z=>z1(o,l,r,u,z,P,C),U=m.form;if(U==="Conical"||U==="Layered"){const z=U==="Layered"?4:3,R=U==="Layered"?1.2:.8;K(H+q*.15);for(let V=0;V<z;V+=1){const J=V/z,Q=(V+1.35)/z,ne=f*R*(1-J*.82);T1(o,l,H+q*J*.92,u,H+q*Math.min(1,Q),ne,x+V,S,7)}}else if(U==="Fastigiate")K(H),yo(o,l,H+q*.3,u,f*.62,q*.32,f*.62,x,S,1,.16),yo(o,l,H+q*.7,u,f*.52,q*.32,f*.52,x+1,S,1,.16);else if(U==="Umbrella")K(H+q*.3),yo(o,l,H+q*.68,u,f*1.12,q*.26,f*1.12,x,S,1,.14),yo(o,l+f*.4,H+q*.55,u-f*.2,f*.6,q*.2,f*.6,x+1,S,1,.14);else if(U==="Weeping")K(H+q*.2),yo(o,l,H+q*.58,u,f*.95,q*.34,f*.95,x,S,1,.14),yo(o,l,H+q*.28,u,f*.86,q*.34,f*.86,x+1,Xo(S,.94,.2),.95,.2);else if(U==="Palm"){const z=r+i*.84;z1(o,l,r,u,z,Math.max(i*.017,.08*f),Xo(C,1.5,0));const R=f*1.05;for(let V=0;V<2;V+=1){const J=V===0?9:6,Q=R*(V===0?1:.62),ne=V===0?-i*.075:i*.03;for(let B=0;B<J;B+=1){const ae=B/J*Math.PI*2+za(x,B,1.1)+V*.4,I=Q*(V===0?.26:.2),ge=[l+Math.cos(ae)*Q,z+ne,u+Math.sin(ae)*Q],Re=[l+Math.cos(ae)*Q*.5,z+i*.035,u+Math.sin(ae)*Q*.5],le=[Re[0]+Math.cos(ae+1.5708)*I,Re[1],Re[2]+Math.sin(ae+1.5708)*I],G=[Re[0]+Math.cos(ae-1.5708)*I,Re[1],Re[2]+Math.sin(ae-1.5708)*I],j=[l,z+i*.01,u],me=Xo(S,.85+.2*za(x,B,2.2),0);o.tri(j,le,Re,[0,1,.2],me,1),o.tri(j,Re,G,[0,1,.2],me,1),o.tri(Re,le,ge,[0,.8,.3],me,.95),o.tri(Re,ge,G,[0,.8,.3],me,.95)}}}else if(U==="Bamboo")for(let z=0;z<3;z+=1){const R=za(x,z,8.1)*Math.PI*2;yo(o,l+Math.cos(R)*f*.35,r+i*(.55+.1*z),u+Math.sin(R)*f*.35,f*.55,i*.36,f*.55,x+z,S,1,.2)}else if(U==="Reed")for(let z=0;z<2;z+=1)T1(o,l+(z-.5)*f*.5,r,u,r+i*(.9-.1*z),f*.7,x+z,Xo(S,1.1,.3),5);else if(U==="Shrub"||U==="Cushion")yo(o,l,r+i*.42,u,f*1,i*.42,f*1,x,S,1,.2),U==="Shrub"&&yo(o,l+f*.5,r+i*.35,u+f*.3,f*.7,i*.35,f*.7,x+1,S,1,.2);else{const z=U==="Open"||U==="Irregular",R=U==="Oval"?1.25:U==="Fan"?1.1:1,V=U==="Banyan"?1.15:1;K(H+q*.3),yo(o,l,H+q*.52,u,f*1.05*V,q*.44*R,f*1.05*V,x,S,1,z?.22:.17);const J=z?3:2;for(let Q=0;Q<J;Q+=1){const ne=za(x,Q,3.9)*Math.PI*2,B=f*(.45+.25*za(x,Q,4.1)),ae=H+q*(.4+.35*za(x,Q,5.7)),I=f*(.48+.2*za(x,Q,6.3));yo(o,l+Math.cos(ne)*B,ae,u+Math.sin(ne)*B,I,q*.26,I,x+3+Q,Xo(S,.94+.12*za(x,Q,2.5),0),1,.2,!1)}}}const Yy=0,Wy=1,C1=2,Xy=3,rh=4,Zf=8;function pp(o){return{roofM:0,startM:0,storeyM:3,shopM:0,wall:o,baseBand:o,baseN:0,topBand:o,topN:0,unitW:3.6,kind:rh,window:[0,0,0,0],balcony:[0,0,0],moduleM:0,glass:[.1,.12,.14],glassKind:0,frame:[.3,.3,.3],trim:o,skin:0,flags:0,weather:.4,crown:0,terracotta:!1,roof:[.17,.165,.152],solar:!1,podiumFloors:0,podiumStoreyM:3,setbackM:0,podiumUnitW:3.6,podiumKind:rh,podiumWindow:[0,0,0,0]}}function Py(o){return{...o,storeyM:o.podiumStoreyM,unitW:o.podiumUnitW,kind:o.podiumKind,window:o.podiumWindow,balcony:[0,0,0],podiumFloors:0}}function _1(o){return{...o,shopM:0}}function Zy(o){const l=Math.max(o.length,1),r=new Float32Array(l*Zf*4);o.forEach(({look:i,storeys:f},h)=>{const v=h*Zf*4,m=(p,b,S,C,x)=>{r.set([b,S,C,x],v+p*4)};m(0,i.wall[0],i.wall[1],i.wall[2],i.storeyM),m(1,i.baseBand[0],i.baseBand[1],i.baseBand[2],i.baseN+256*f),m(2,i.topBand[0],i.topBand[1],i.topBand[2],i.topN),m(3,i.unitW,i.window[0],i.window[1],i.kind),m(4,i.window[2],i.window[3],i.balcony[0],i.balcony[1]),m(5,i.glass[0],i.glass[1],i.glass[2],i.moduleM),m(6,i.frame[0],i.frame[1],i.frame[2],i.skin*8+i.flags),m(7,i.shopM,i.weather,i.glassKind,i.balcony[2])});const u=new ma(r,Zf,l,$n,V1);return u.colorSpace=Y1,u.magFilter=Ts,u.minFilter=Ts,u.generateMipmaps=!1,u.flipY=!1,u.needsUpdate=!0,u}const Qy=`
attribute float aBld;
attribute vec2 aFace;
varying float vBld;
varying vec2 vFace;
`,Iy=`
vBld = aBld;
vFace = aFace;
`,Jy=`
uniform sampler2D uMassLook;
varying float vBld;
varying vec2 vFace;

vec4 mLook(int row, int texel) { return texelFetch(uMassLook, ivec2(texel, row), 0); }
float mHash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}
float mNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(mHash(vec3(i, 1.0)), mHash(vec3(i + vec2(1.0, 0.0), 1.0)), f.x),
             mix(mHash(vec3(i + vec2(0.0, 1.0), 1.0)), mHash(vec3(i + vec2(1.0, 1.0), 1.0)), f.x), f.y);
}
// 1 inside [a, b], integrated over a pixel of width w.
float mBox(float x, float a, float b, float w) {
  return clamp((x - a) / w + 0.5, 0.0, 1.0) * clamp((b - x) / w + 0.5, 0.0, 1.0);
}
// The same box repeating every 'period': once a pixel spans a good part of the
// period the position inside it is noise, so it fades to the box's share.
float mBoxP(float x, float a, float b, float w, float period) {
  float mean = clamp((b - a) / period, 0.0, 1.0);
  return mix(mBox(x, a, b, w), mean, smoothstep(0.2 * period, 0.8 * period, w));
}
// Lines of half-width hw every 'period', fading to their share.
float mLines(float x, float period, float hw, float w) {
  float d = abs(fract(x / period + 0.5) - 0.5) * period;
  float line = clamp((hw + w * 0.5 - d) / max(w, 1e-4), 0.0, 1.0);
  return mix(line, min(1.0, 2.0 * hw / period), smoothstep(0.25 * period, 0.7 * period, w));
}

// What is seen through one window: the room's mean, or a curtain or blind of
// this window's own. 'pix' is the pixel's size relative to the window, 0..1+.
vec3 mWindowInside(vec2 cell, float gu, float gv, float office, float pix, vec3 roomMean) {
  float g2 = mHash(vec3(cell, 3.0));
  float g3 = mHash(vec3(cell, 5.0));
  float g4 = mHash(vec3(cell, 7.0));
  float du = 0.04 + pix;
  float curtain = 0.0;
  vec3 curtainC = vec3(0.0);
  if (office > 0.5) {
    if (g2 < 0.45) {
      float f = 0.15 + 0.85 * g3;
      curtain = smoothstep(1.0 - f - du, 1.0 - f + du, gv);
      curtainC = vec3(0.45, 0.45, 0.43);
    }
  } else if (g2 < 0.30) {
    curtain = 0.0;
  } else if (g2 < 0.68) {
    float f = 0.25 + 0.75 * g3;
    curtain = 0.82 * (g4 < 0.5 ? smoothstep(f + du, f - du, gu) : smoothstep(1.0 - f - du, 1.0 - f + du, gu));
    curtainC = vec3(0.62, 0.60, 0.55);
  } else if (g2 < 0.9) {
    float f = 0.3 + 0.7 * g3;
    curtain = smoothstep(f * 0.5 + du, f * 0.5 - du, gu) + smoothstep(1.0 - f * 0.5 - du, 1.0 - f * 0.5 + du, gu);
    curtainC = g4 < 0.3 ? vec3(0.42, 0.33, 0.22) : g4 < 0.55 ? vec3(0.22, 0.26, 0.33) : g4 < 0.8 ? vec3(0.55, 0.50, 0.40) : vec3(0.26, 0.16, 0.12);
  } else {
    float f = 0.2 + 0.8 * g3;
    curtain = smoothstep(1.0 - f - du, 1.0 - f + du, gv);
    curtainC = vec3(0.56, 0.54, 0.50);
  }
  curtain = clamp(curtain, 0.0, 1.0);
  return mix(roomMean, curtainC * uInteriorLightM * 1.6, curtain);
}
// The share of a window a curtain covers, and its mean colour, over all windows.
vec3 mWindowMean(float office, vec3 roomMean) {
  float cover = office > 0.5 ? 0.26 : 0.40;
  vec3 curtainMean = (office > 0.5 ? vec3(0.45, 0.45, 0.43) : vec3(0.50, 0.46, 0.40)) * uInteriorLightM * 1.6;
  return mix(roomMean, curtainMean, cover);
}
`,$y=`
float massGlass = 0.0;
float massRough = 0.92;
vec3 massEmit = vec3(0.0);
vec3 massF0 = vec3(0.04);
float massTone = 1.0;
#include <color_fragment>
float wallness = 1.0 - smoothstep(0.35, 0.7, abs(vMassNormal.y));
float hM = (vMassWorld.y - vBase) / uMetre;
float seedKind = floor(vSeed);
float seed = fract(vSeed);
if (vBld > -0.5 && wallness > 0.01) {
  int bi = int(vBld + 0.5);
  vec4 t0 = mLook(bi, 0);
  vec4 t1 = mLook(bi, 1);
  vec4 t2 = mLook(bi, 2);
  vec4 t3 = mLook(bi, 3);
  vec4 t4 = mLook(bi, 4);
  vec4 t5 = mLook(bi, 5);
  vec4 t6 = mLook(bi, 6);
  vec4 t7 = mLook(bi, 7);
  float storeyM = max(t0.a, 2.4);
  float baseN = mod(t1.a, 256.0);
  float total = floor(t1.a / 256.0);
  float topN = t2.a;
  float unitW = max(t3.x, 1.0);
  int lookKind = int(t3.w + 0.5);
  float shopM = t7.x;
  float weather = t7.y;
  int glassKind = int(t7.z + 0.5);
  int skinFlags = int(t6.a + 0.5);
  int flags = skinFlags - (skinFlags / 8) * 8;
  bool surround = (flags - (flags / 2) * 2) == 1;
  bool slabBand = ((flags / 2) - (flags / 4) * 2) == 1;
  bool office = glassKind >= 2;
  vec3 trimC = mix(t0.rgb, vec3(0.70, 0.70, 0.67), 0.35);

  // Which flat, and where in it: the wall is cut into whole flats, mirrored at
  // every second, as the detail cuts it.
  float L = max(vFace.y / uMetre, 0.5);
  float u = vFace.x / uMetre;
  float units = max(floor(L / unitW + 0.5), 1.0);
  float U = u * units * unitW / L;
  float k = floor(U / unitW);
  float x = U - k * unitW;
  if (mod(k, 2.0) > 0.5) x = unitW - x;
  float fw = max(fwidth(U), 1e-4);
  float fh = max(fwidth(hM), 1e-4);

  bool shop = shopM > 0.5 && hM < shopM;
  float hh = shopM > 0.5 ? hM - shopM : hM;
  float fl = floor(hh / storeyM);
  float ml = hh - fl * storeyM;
  float fadeCell = smoothstep(0.12 * unitW, 0.4 * unitW, fw);
  float fadeCellY = smoothstep(0.12 * storeyM, 0.4 * storeyM, fh);
  float fadeAny = max(fadeCell, fadeCellY);

  // The body colour, by storey: the base band, the body, the top band.
  vec3 wallC = t0.rgb;
  float inBase = (1.0 - smoothstep(baseN - 0.02 - fh / storeyM, baseN + 0.02 + fh / storeyM, hh / storeyM)) * step(0.5, baseN);
  float inTop = smoothstep(total - topN - 0.02 - fh / storeyM, total - topN + 0.02 + fh / storeyM, hh / storeyM) * step(0.5, topN);
  wallC = mix(wallC, t1.rgb, inBase);
  wallC = mix(wallC, t2.rgb, inTop);
  if (shop) wallC = t1.rgb;
  // Plant, parapets and gables have no design: they keep their own colour.
  if (lookKind == 4) wallC = diffuseColor.rgb;
  // Patch repairs and fading leave the paint a few per cent off from flat to flat.
  float patchN = mNoise(vec2(U * 0.23 + t0.a * 3.1, hh * 0.11 + t3.x)) - 0.5;
  float blotch = mNoise(vMassWorld.xz / uMetre * 0.31 + hM * 0.27) - 0.5;
  float wallTone = 1.0 + (0.05 + 0.07 * weather) * blotch + 0.07 * patchN;
  vec3 colour = wallC * wallTone;
  float rough = 0.9;

  float glassCov = 0.0;     // reflective glass, any kind
  float emitCov = 0.0;      // glass with a room behind it
  vec3 inside = vec3(0.0);
  vec3 roomMean = vec3(0.55, 0.52, 0.47) * uInteriorLightM * 0.55;

  if (lookKind == 4) {
    // No windows: shed, plant, parapet. Profiled cladding gets its ribs.
    float ribs = mLines(U, 0.3, 0.02, fw);
    colour *= 1.0 - 0.10 * ribs * step(1.5, seedKind) * (1.0 - step(2.5, seedKind));
  } else if (shop) {
    // The shop storey: piers, a lit shop window, a fascia sign, a stall riser,
    // and a roller shutter down in some bays.
    float cellId = mHash(vec3(k, fl, t0.a * 7.0));
    float shut = step(0.80, cellId);
    float pierW = 0.45;
    float win = mBoxP(x, pierW, unitW - pierW, fw, unitW) * mBoxP(hM, 0.55, min(3.35, shopM - 0.9), fh, shopM);
    float fascia = mBoxP(hM, min(3.45, shopM - 0.85), shopM - 0.12, fh, shopM);
    vec3 signC = mix(vec3(0.62, 0.10, 0.08), vec3(0.08, 0.20, 0.45), step(0.5, mHash(vec3(k, 3.0, t0.a))));
    signC = mix(signC, vec3(0.74, 0.70, 0.60), step(0.7, mHash(vec3(k, 5.0, t0.a * 3.0))));
    colour = mix(colour, mix(signC, wallC, fadeCell) * 0.9, fascia * (1.0 - 0.5 * fadeCell));
    vec3 shutter = vec3(0.42, 0.43, 0.42);
    colour = mix(colour, mix(shutter, wallC, fadeCell), win * shut);
    float lit = mix(0.35 + 0.65 * mHash(vec3(k, 11.0, t0.a)), 0.62, fadeCell);
    inside = vec3(1.0, 0.88, 0.68) * uInteriorLightM * 1.5 * lit;
    glassCov = win * (1.0 - shut);
    emitCov = glassCov;
  } else if (lookKind == 1) {
    // Curtain wall: mullions at the module, a transom at every slab, a dark
    // spandrel under the vision glass.
    float module = max(t5.w, 0.6);
    float mull = max(mLines(U, module, 0.035, fw), mLines(hh, storeyM, 0.035, fh));
    glassCov = 1.0 - mull;
    colour = mix(colour, t6.rgb, mull);
    float vision = mBoxP(ml, 0.9, storeyM - 0.04, fh, storeyM);
    emitCov = glassCov * vision;
    float pix = fw / module;
    vec2 cell = vec2(floor(U / module), fl) + t0.a * 31.0;
    float gu = fract(U / module);
    float gv = clamp((ml - 0.9) / max(storeyM - 0.9, 0.5), 0.0, 1.0);
    inside = mix(mWindowInside(cell, gu, gv, 1.0, pix, roomMean), mWindowMean(1.0, roomMean), smoothstep(0.15, 0.6, pix));
    // Pane to pane the coating reads a little differently: a tilt, a sky patch.
    massTone = mix(0.80 + 0.40 * mHash(vec3(cell, 13.0)), 1.0, smoothstep(0.15, 0.6, pix));
    // Upper floors take more sky.
    massTone *= mix(0.9, 1.12, smoothstep(0.0, 60.0, hM));
  } else {
    // Punched windows: grid, ribbon and the residential window all one rule.
    float sill = t4.x;
    float head = t4.y;
    float winX = mBoxP(x, t3.y, t3.z, fw, unitW);
    float winY = mBoxP(ml, sill, head, fh, storeyM);
    glassCov = winX * winY;
    emitCov = glassCov;
    float gu = clamp((x - t3.y) / max(t3.z - t3.y, 0.3), 0.0, 1.0);
    float gv = clamp((ml - sill) / max(head - sill, 0.3), 0.0, 1.0);
    float pix = fw / max(t3.z - t3.y, 0.3);
    vec2 cell = vec2(k, fl) + t0.a * 31.0;
    inside = mix(mWindowInside(cell, gu, gv, office ? 1.0 : 0.0, pix, roomMean), mWindowMean(office ? 1.0 : 0.0, roomMean), smoothstep(0.15, 0.6, pix));
    massTone = mix(0.85 + 0.30 * mHash(vec3(cell, 13.0)), 1.0, smoothstep(0.15, 0.6, pix));
    // Moulded surround and the shadowed head reveal.
    if (surround) {
      float ring = mBoxP(x, t3.y - 0.14, t3.z + 0.14, fw, unitW) * mBoxP(ml, sill - 0.16, head + 0.18, fh, storeyM);
      colour = mix(colour, trimC, clamp(ring - glassCov, 0.0, 1.0) * (1.0 - 0.4 * fadeAny));
    }
    colour *= 1.0 - 0.30 * mBoxP(x, t3.y, t3.z, fw, unitW) * mBoxP(ml, head, head + 0.1, fh, storeyM);
    // A balcony or loggia door in the flat.
    if (t4.w > 0.1) {
      float bx0 = t4.z + 0.3;
      float bx1 = t4.w - 0.3;
      float bhead = t7.w;
      float recess = mBoxP(x, t4.z, t4.w, fw, unitW) * mBoxP(ml, 0.0, bhead, fh, storeyM);
      colour *= 1.0 - 0.18 * recess;
      float door = mBoxP(x, bx0, bx1, fw, unitW) * mBoxP(ml, 1.1, bhead - 0.03, fh, storeyM);
      float dgu = clamp((x - bx0) / max(bx1 - bx0, 0.3), 0.0, 1.0);
      vec3 doorIn = mix(mWindowInside(cell + 5.0, dgu, 0.5, 0.0, fw / max(bx1 - bx0, 0.3), roomMean), mWindowMean(0.0, roomMean), smoothstep(0.15, 0.6, fw / max(bx1 - bx0, 0.3)));
      float gcOld = glassCov;
      glassCov = clamp(glassCov + door, 0.0, 1.0);
      inside = (inside * gcOld + doorIn * door) / max(glassCov, 1e-3);
      emitCov = glassCov;
    }
  }

  // The slab: a band of trim at the edge and the grime under it.
  if (lookKind != 4 && !shop) {
    if (slabBand) {
      float band = mBoxP(ml, -0.05, 0.22, fh, storeyM);
      colour = mix(colour, trimC, band * (1.0 - glassCov) * 0.8);
    }
    colour *= 1.0 - 0.10 * mBoxP(ml, storeyM - 0.12, storeyM + 0.01, fh, storeyM) * (0.5 + weather) * (1.0 - glassCov);
  }

  // Glass: the room's radiance and the coating, in place of the wall.
  float transmit = office ? 0.45 : glassKind == 1 ? 0.6 : 0.8;
  vec3 glassTint = glassKind == 1 ? mix(vec3(1.0), t5.rgb * 1.5, 0.5) : glassKind == 3 ? mix(vec3(1.0), t5.rgb * 1.4, 0.6) : vec3(1.0);
  if (lookKind == 1) transmit = 0.6;
  massEmit = inside * transmit * glassTint * emitCov * massTone;
  colour = mix(colour, vec3(0.0), glassCov);
  vec3 f0 = glassKind == 3 ? mix(vec3(0.04), t5.rgb, 0.3) : glassKind == 2 ? vec3(0.10, 0.11, 0.12) : glassKind == 1 ? mix(vec3(0.05), t5.rgb * 0.12, 0.5) : vec3(0.045);
  massF0 = f0;
  rough = mix(rough, 0.07, glassCov);
  massGlass = glassCov;

  // Dirt washed down the wall in columns: the detail's streaks, as their mean
  // once a column is under a pixel.
  float column = floor(U / 0.8);
  float columnR = fract(sin(column * 12.9898 + t0.a * 78.233 + t3.x) * 43758.5453);
  float runR = fract(sin(column * 4.13 + floor(hM / 9.0) * 7.71 + t3.x * 3.3) * 43758.5453);
  float streak = smoothstep(0.55, 0.95, columnR) * (0.55 + 0.45 * runR);
  streak = mix(streak, 0.13, smoothstep(0.1, 0.6, fw));
  colour *= 1.0 - vec3(0.85, 0.92, 1.0) * (0.10 + 0.20 * weather) * streak * (1.0 - glassCov);
  // A foot that is darker where the street's dirt and the neighbour's shade meet it.
  float foot = 1.0 - smoothstep(0.0, 6.0, hM);
  if (lookKind != 4) colour *= 1.0 - 0.22 * foot * (1.0 - glassCov * 0.5);

  diffuseColor.rgb = mix(diffuseColor.rgb, colour, wallness);
  massRough = mix(0.92, rough, wallness);
  massGlass *= wallness;
  massEmit *= wallness;
}
if (vBld > -0.5 && wallness <= 0.01 && vMassNormal.y > 0.5) {
  // A roof deck: screed, membrane or tile, patched and stained, never one flat grey.
  vec2 rp = vMassWorld.xz / uMetre;
  float rfw = length(fwidth(rp));
  float rn = mNoise(rp * 0.35) - 0.5;
  float rf = mNoise(rp * 1.9) - 0.5;
  float coarse = 1.0 + 0.10 * rn;
  float fineN = 1.0 + 0.06 * rf * (1.0 - smoothstep(0.3, 1.2, rfw));
  float joints = 1.0 - 0.07 * max(mLines(rp.x, 1.2, 0.012, rfw), mLines(rp.y, 3.0, 0.012, rfw));
  diffuseColor.rgb *= coarse * fineN * joints;
}
`,e3=`
#include <roughnessmap_fragment>
roughnessFactor = massRough;
`,t3=`
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, massF0, massGlass);
material.specularColorBlended = mix(material.specularColorBlended, massF0, massGlass);
`,a3=`
#include <emissivemap_fragment>
{
  // The room behind the glass, and a sky patch where there is no environment map.
  float facing = clamp(abs(dot(normalize(vViewPosition), normal)), 0.0, 1.0);
  float fresnel = pow(1.0 - facing, 3.0);
  totalEmissiveRadiance += massEmit;
  totalEmissiveRadiance += massGlass * massTone * vec3(0.030, 0.037, 0.048) * (0.4 + 3.0 * fresnel);
}
`,n3=`#define uInteriorLightM ${sp.toFixed(4)}
`,ch=256;function Un(o,l){let r=o*2654435761^l*2246822507;return r^=r>>>16,r=Math.imul(r,2146121005),r^=r>>>15,(r>>>0)/4294967295}const vp=10246986,A1=[13222838,12169892,13815233,11051928,12563871,10462115,14207920,vp,15131871],o3=[599e4,7174016,9081755,12106944,5069670],E1=[7565681,7368816,7697266],l3=[14474973,12962252,11451595,14077885,12567487],R1=[6061470,5205903,9081753,10857648,7244448],s3=[13546914,13546914,13546914,13546914,13546914,12623502,12623502,12623502,12623502,11308676,11308676,11308676,11308676,14140853,14140853,14140853,14140853,10647923,10647923,10647923,14538443,14538443,14538443,12894396,12894396,12894396,12894396,10657433,10657433,10657433,14802907,14802907,9333089,9333089,12618102,12618102,7113109,11320516,11191475],i3=6258591,r3=7239036,N1=14211280,c3=9737364,D1=9067077,u3=6578782,f3=9144706,h3=10131343,d3={arterial:5,collector:4,local:3},m3=2,o0=3;function p3(o,l,r,u,i,f,h,v){let m=1,p=0,b=0;for(let Q=0;Q<r.length;Q+=1){const[ne,B]=r[Q],[ae,I]=r[(Q+1)%r.length],ge=Math.hypot(ae-ne,I-B);ge>b&&(b=ge,m=(ae-ne)/ge,p=(I-B)/ge)}const S=r.reduce((Q,ne)=>Q+ne[0],0)/r.length,C=r.reduce((Q,ne)=>Q+ne[1],0)/r.length;let x=1/0,D=-1/0,H=1/0,q=-1/0;for(const[Q,ne]of r){const B=(Q-S)*m+(ne-C)*p,ae=-(Q-S)*p+(ne-C)*m;x=Math.min(x,B),D=Math.max(D,B),H=Math.min(H,ae),q=Math.max(q,ae)}const P=D-x,K=q-H,U=(Q,ne)=>[S+m*Q-p*ne,C+p*Q+m*ne],z=Q=>Un(l.id,Q);if(f){if(P>24*h){const[Q,ne]=U((x+D)*.5,(H+q)*.5);o.orientedBox(Q,ne,m,p,P*.4,.8*h,u,u+.7*h,13159631)}return}if(l.useType==="village"){if(z(71)<.45&&P>6*h){const Q=.35+.35*z(72),[ne,B]=U(D-P*Q*.5,(H+q)*.5);o.orientedBox(ne,B,m,p,P*Q*.5,K*.45,u,u+2.6*h,i3)}if(z(73)<.6){const[Q,ne]=U(x+1.5*h,H+1.5*h);o.orientedBox(Q,ne,m,p,.8*h,.8*h,u,u+1.6*h,N1)}return}if(l.heightMetres<9)return;if(l.useType==="commercial"){const Q=gp(r,3.5*h);Q&&x3(Q)>60*h*h&&o.parapet(Q,u,3.2*h,c3);const ne=1+Math.floor(z(77)*2);for(let B=0;B<ne;B+=1){const[ae,I]=U(x+P*(.15+.2*B),q-K*.22);o.orientedBox(ae,I,m,p,2*h,2*h,u,u+3.5*h,N1)}return}const R=(16+4*z(78))*h,V=Math.max(1,Math.round(P/R)),J=l.floors>=8;for(let Q=0;Q<V;Q+=1){const ne=x+(Q+.5)*(P/V),[B,ae]=U(ne,H+K*.62);o.orientedBox(B,ae,m,p,(J?3.2:2)*h,(J?3:1.8)*h,u,u+(J?3.6:2.6)*h,i)}if(v??(l.floors<=7&&z(79)<.75)){const Q=Math.max(1,Math.floor((K-3*h)/(2.6*h)));for(let ne=0;ne<Q;ne+=1){const B=H+1.6*h+ne*2.6*h,[ae,I]=U((x+D)*.5,B);o.orientedBox(ae,I,m,p,P*(.3+.12*z(80+ne)),.7*h,u,u+1.3*h,r3)}}}class U1{positions=[];normals=[];colors=[];tileIds=[];bases=[];seeds=[];blds=[];faces=[];bld=-1;plainBld=-1;base=0;seed=0;tileIndex=new Map;current=-1;colour=new $t;select(l,r){const u=a0(l,r);let i=this.tileIndex.get(u);i===void 0&&(i=this.tileIndex.size,this.tileIndex.set(u,i)),this.current=i<ch?i:-1}triangles(){return this.positions.length/9}push(l,r,u,i,f,h,v){this.colour.setHex(f),[l,r,u].forEach((m,p)=>{this.positions.push(m[0],m[1],m[2]),this.normals.push(i[0],i[1],i[2]),this.colors.push(this.colour.r*h,this.colour.g*h,this.colour.b*h),this.tileIds.push(this.current),this.bases.push(this.base),this.seeds.push(this.seed),this.blds.push(this.bld),this.faces.push(v?v.u[p]:0,v?v.length:1)})}tri(l,r,u,i,f,h=1,v){const m=r[0]-l[0],p=r[1]-l[1],b=r[2]-l[2],S=u[0]-l[0],C=u[1]-l[1],x=u[2]-l[2],D=p*x-b*C,H=b*S-m*x,q=m*C-p*S;D*i[0]+H*i[1]+q*i[2]>=0?this.push(l,r,u,i,f,h,v):this.push(l,u,r,i,f,h,v&&{u:[v.u[0],v.u[2],v.u[1]],length:v.length})}triSmooth(l,r,u,i,f,h,v,m=1){const p=r[0]-l[0],b=r[1]-l[1],S=r[2]-l[2],C=u[0]-l[0],x=u[1]-l[1],D=u[2]-l[2],H=b*D-S*x,q=S*C-p*D,P=p*x-b*C,K=i[0]+f[0]+h[0],U=i[1]+f[1]+h[1],z=i[2]+f[2]+h[2],R=H*K+q*U+P*z<0,V=R?[l,u,r]:[l,r,u],J=R?[i,h,f]:[i,f,h];this.colour.setHex(v);for(let Q=0;Q<3;Q+=1)this.positions.push(V[Q][0],V[Q][1],V[Q][2]),this.normals.push(J[Q][0],J[Q][1],J[Q][2]),this.colors.push(this.colour.r*m,this.colour.g*m,this.colour.b*m),this.tileIds.push(this.current),this.bases.push(this.base),this.seeds.push(this.seed),this.blds.push(this.bld),this.faces.push(0,1)}prism(l,r,u,i,f,h=r,v=!0){const m=l.length;if(m<3||!(u>r))return;this.base=h;let p=0,b=0;for(const[C,x]of l)p+=C,b+=x;p/=m,b/=m;for(let C=0;C<m;C+=1){const[x,D]=l[C],[H,q]=l[(C+1)%m],P=H-x,K=q-D,U=Math.hypot(P,K);if(U<1e-9)continue;let z=K/U,R=-P/U;z*((x+H)*.5-p)+R*((D+q)*.5-b)<0&&(z=-z,R=-R);const V=[z,0,R];this.tri([x,r,D],[H,r,q],[H,u,q],V,i,1,{u:[0,U,U],length:U}),this.tri([x,r,D],[H,u,q],[x,u,D],V,i,1,{u:[0,U,0],length:U})}if(!v)return;const S=l.map(([C,x])=>new Et(C,x));for(const[C,x,D]of t1.triangulateShape(S,[]))this.tri([l[C][0],u,l[C][1]],[l[x][0],u,l[x][1]],[l[D][0],u,l[D][1]],[0,1,0],f,1)}parapet(l,r,u,i){const f=l.length;if(f<3||!(u>0))return;let h=0,v=0;for(const[m,p]of l)h+=m,v+=p;h/=f,v/=f,this.base=r;for(let m=0;m<f;m+=1){const[p,b]=l[m],[S,C]=l[(m+1)%f],x=S-p,D=C-b,H=Math.hypot(x,D);if(H<1e-9)continue;let q=D/H,P=-x/H;q*((p+S)*.5-h)+P*((b+C)*.5-v)<0&&(q=-q,P=-P);for(const K of[1,-1]){const U=[q*K,0,P*K];this.tri([p,r,b],[S,r,C],[S,r+u,C],U,i,1,{u:[0,H,H],length:H}),this.tri([p,r,b],[S,r+u,C],[p,r+u,b],U,i,1,{u:[0,H,0],length:H})}}}pitchedRoof(l,r,u,i,f,h,v){let m=1,p=0,b=0;for(let I=0;I<l.length;I+=1){const[ge,Re]=l[I],[le,G]=l[(I+1)%l.length],j=Math.hypot(le-ge,G-Re);j>b&&(b=j,m=(le-ge)/j,p=(G-Re)/j)}const S=l.reduce((I,ge)=>I+ge[0],0)/l.length,C=l.reduce((I,ge)=>I+ge[1],0)/l.length;let x=1/0,D=-1/0,H=1/0,q=-1/0;for(const[I,ge]of l){const Re=(I-S)*m+(ge-C)*p,le=-(I-S)*p+(ge-C)*m;x=Math.min(x,Re),D=Math.max(D,Re),H=Math.min(H,le),q=Math.max(q,le)}const P=(I,ge,Re)=>[S+m*I-p*ge,Re,C+p*I+m*ge],U=((q-H)*.5+u)*i,z=r,R=r+U,V=(H+q)*.5,J=x-u,Q=D+u,ne=f?Math.min(x+(q-H)*.5,(x+D)*.5):J+u,B=f?Math.max(D-(q-H)*.5,(x+D)*.5):Q-u,ae=(I,ge)=>{const Re=(I[1][1]-I[0][1])*(I[2][2]-I[0][2])-(I[1][2]-I[0][2])*(I[2][1]-I[0][1]),le=(I[1][2]-I[0][2])*(I[2][0]-I[0][0])-(I[1][0]-I[0][0])*(I[2][2]-I[0][2]),G=(I[1][0]-I[0][0])*(I[2][1]-I[0][1])-(I[1][1]-I[0][1])*(I[2][0]-I[0][0]),j=Math.hypot(Re,le,G)||1,me=le<0?-1:1,X=[Re/j*me,le/j*me,G/j*me];this.tri(I[0],I[1],I[2],X,ge),I.length===4&&this.tri(I[0],I[2],I[3],X,ge)};ae([P(J,H-u,z),P(Q,H-u,z),P(B,V,R),P(ne,V,R)],h),ae([P(J,q+u,z),P(Q,q+u,z),P(B,V,R),P(ne,V,R)],h),f?(ae([P(J,H-u,z),P(J,q+u,z),P(ne,V,R)],h),ae([P(Q,H-u,z),P(Q,q+u,z),P(B,V,R)],h)):(ae([P(x,H,z),P(x,q,z),P(x,V,R)],v),ae([P(D,H,z),P(D,q,z),P(D,V,R)],v))}box(l,r,u,i,f,h){this.orientedBox(l,r,1,0,u,u,i,f,h)}orientedBox(l,r,u,i,f,h,v,m,p){const b=-i,S=u,C=(q,P)=>[l+u*q+b*P,r+i*q+S*P],x=[C(-f,-h),C(f,-h),C(f,h),C(-f,h)],D=this.seed,H=this.bld;this.seed=o0+D%1,this.bld=this.plainBld,this.prism(x,v,m,p,p),this.seed=D,this.bld=H}flat(l,r,u,i){if(l.length<3)return;const f=l.map(([v,m])=>new Et(v,m)),h=v=>[l[v][0],r(l[v][0],l[v][1])+u,l[v][1]];for(const[v,m,p]of t1.triangulateShape(f,[]))this.tri(h(v),h(m),h(p),[0,1,0],i)}smoothTri(l,r,u,i,f,h){const m=(u[0]-l[0])*(f[2]-l[2])-(u[2]-l[2])*(f[0]-l[0])<0?[[l,r],[u,i],[f,h]]:[[l,r],[f,h],[u,i]];for(const[p,b]of m)this.positions.push(p[0],p[1],p[2]),this.normals.push(0,1,0),this.colors.push(b.r,b.g,b.b),this.tileIds.push(this.current),this.bases.push(0),this.seeds.push(o0),this.blds.push(-1),this.faces.push(0,1)}ribbon(l,r,u){for(let i=0;i+1<l.length;i+=1){const[f,h,v]=l[i],[m,p,b]=l[i+1],S=m-f,C=b-v,x=Math.hypot(S,C);if(x<1e-9)continue;const D=-C/x*r*.5,H=S/x*r*.5,q=[f+D,h,v+H],P=[f-D,h,v-H],K=[m+D,p,b+H],U=[m-D,p,b-H];this.tri(q,P,U,[0,1,0],u),this.tri(q,U,K,[0,1,0],u)}}geometry(){const l=new sa;return l.setAttribute("position",new st(this.positions,3)),l.setAttribute("normal",new st(this.normals,3)),l.setAttribute("color",new st(this.colors,3)),l.setAttribute("aTile",new st(this.tileIds,1)),l.setAttribute("aBase",new st(this.bases,1)),l.setAttribute("aSeed",new st(this.seeds,1)),l.setAttribute("aBld",new st(this.blds,1)),l.setAttribute("aFace",new st(this.faces,2)),l.computeBoundingSphere(),l.computeBoundingBox(),l}}function O1(o,l){return o.map((r,u)=>{const i=o[Math.max(0,u-1)],f=o[Math.min(o.length-1,u+1)],h=f[0]-i[0],v=f[2]-i[2],m=Math.hypot(h,v)||1;return[r[0]-v/m*l,r[1],r[2]+h/m*l]})}function k1(o,l,r,u,i,f,h){Ky(o,l,r,u,i,f,h,Math.abs(Math.sin(l*12.9898+u*78.233)*43758.5453)%1)}function Vu(o){const l=Math.max(o.length,1);return{x_km:o.reduce((r,u)=>r+u.x_km,0)/l,y_km:o.reduce((r,u)=>r+u.y_km,0)/l}}const v3=[{at:-1,colour:5069374},{at:-.9,colour:7306079},{at:-.62,colour:5204064},{at:-.25,colour:3625820},{at:0,colour:3099222},{at:.25,colour:3625820},{at:.62,colour:5204064},{at:.9,colour:7306079},{at:1,colour:5069374}];function g3(o){if(!o)return v3;const l=Math.min(1,Math.max(0,o.moisture)),r=Math.min(1,Math.max(0,.3+1.2*o.sediment+.25*(1-l))),u=(p,b,S)=>new $t(p).lerp(new $t(b),S).getHex(),i=u(2902610,4868656,r),f=u(3627100,5920058,r),h=u(5204064,8024136,r),v=u(5069374,10129528,Math.min(1,(1-l)*1.4)),m=u(7306079,11576196,Math.min(1,(1-l)*1.4));return[{at:-1,colour:v},{at:-.9,colour:m},{at:-.62,colour:h},{at:-.25,colour:f},{at:0,colour:i},{at:.25,colour:f},{at:.62,colour:h},{at:.9,colour:m},{at:1,colour:v}]}function B1(o,l,r,u,i,f){const h=g3(f),v=u.metresToScene;let m=l.map(P=>u.toScene(P.x_km,P.y_km));if(m.length<2)return;for(let P=0;P<3&&m.length<600;P+=1){const K=[m[0]];for(let U=0;U+1<m.length;U+=1){const[z,R]=m[U],[V,J]=m[U+1];K.push([z*.75+V*.25,R*.75+J*.25],[z*.25+V*.75,R*.25+J*.75])}K.push(m[m.length-1]),m=K}const p=r*.5*v,b=p+Math.max(2*v,p*.1),S=m.length,C=m.map((P,K)=>{const U=m[Math.max(0,K-1)],z=m[Math.min(S-1,K+1)],R=z[0]-U[0],V=z[1]-U[1],J=Math.hypot(R,V)||1;return[-V/J,R/J]}),x=m.map(([P,K],U)=>{let z=u.heightAt(P,K);for(const R of[-1,1])z=Math.max(z,u.heightAt(P+C[U][0]*p*R,K+C[U][1]*p*R));return z+.22*v}),D=h.map(P=>new $t(P.colour)),H=new $t(6253130);let q=0;for(let P=0;P+1<S;P+=1){i(l[Math.min(l.length-1,Math.round(P/(S-1)*(l.length-1)))]),q+=Math.hypot(m[P+1][0]-m[P][0],m[P+1][1]-m[P][1]);const K=.5+.5*Math.sin(q/v*.011+1.3)*Math.sin(q/v*.031),U=(z,R)=>{const V=h[R].at,J=V*(Math.abs(V)===1?b:p),Q=D[R].clone();return R>0&&R<h.length-1&&Q.lerp(H,K*.28),{p:[m[z][0]+C[z][0]*J,x[z],m[z][1]+C[z][1]*J],colour:Q}};for(let z=0;z+1<h.length;z+=1){const R=U(P,z),V=U(P,z+1),J=U(P+1,z),Q=U(P+1,z+1);o.smoothTri(R.p,R.colour,V.p,V.colour,Q.p,Q.colour),o.smoothTri(R.p,R.colour,Q.p,Q.colour,J.p,J.colour)}}}const wc=new $t;function Qf(o){return wc.setRGB(o[0],o[1],o[2]).getHex()}function y3(o){return wc.setHex(o),[wc.r,wc.g,wc.b]}function x3(o){let l=0;for(let r=0;r<o.length;r+=1){const[u,i]=o[r],[f,h]=o[(r+1)%o.length];l+=u*h-f*i}return Math.abs(l)*.5}function gp(o,l){const r=o.length;if(r<3||l<=0)return null;let u=0;for(let m=0;m<r;m+=1){const[p,b]=o[m],[S,C]=o[(m+1)%r];u+=p*C-S*b}const i=u>=0?1:-1,f=o.map((m,p)=>{const b=o[(p+1)%r],S=Math.hypot(b[0]-m[0],b[1]-m[1])||1,C=-(b[1]-m[1])/S*i,x=(b[0]-m[0])/S*i;return{px:m[0]+C*l,pz:m[1]+x*l,dx:(b[0]-m[0])/S,dz:(b[1]-m[1])/S}}),h=[];for(let m=0;m<r;m+=1){const p=f[(m+r-1)%r],b=f[m],S=p.dx*b.dz-p.dz*b.dx;if(Math.abs(S)<1e-6){h.push([b.px,b.pz]);continue}const C=((b.px-p.px)*b.dz-(b.pz-p.pz)*b.dx)/S;h.push([p.px+p.dx*C,p.pz+p.dz*C])}let v=0;for(let m=0;m<r;m+=1)v+=h[m][0]*h[(m+1)%r][1]-h[(m+1)%r][0]*h[m][1];return v*i<=0||Math.abs(v)<Math.abs(u)/3?null:h}function w3(o,l){const r=o.heightMetres>=40&&o.useType==="commercial",u=l?l3:o.useType==="village"?s3:r?o3:A1;let i=u[Math.floor(Un(o.id,1)*u.length)];i===vp&&o.heightMetres>25&&(i=A1[0]);const f=y3(i),h=pp(f),m=(o.useType==="commercial"||o.useType==="mixedUse")&&o.floors>=2?4.5:0,p=Math.max(o.floors-(m>0?1:0),1);h.shopM=m,h.roofM=Math.max(o.heightMetres,3),h.storeyM=Math.min(Math.max((h.roofM-m)/p,2.7),4.8),h.weather=.2+.4*Un(o.id,5),h.baseBand=[f[0]*.7,f[1]*.7,f[2]*.7],h.topBand=f,h.trim=f,h.glassKind=o.useType==="commercial"?2:Un(o.id,7)<.4?1:0;const b=Un(o.id,8);h.glass=b<.5?[.3,.55,.52]:[.28,.42,.58],h.frame=[.2,.2,.2],h.flags=Un(o.id,9)<.35?2:0;const S=o.heightMetres>=24,C=Math.max(...o.footprint.map(x=>Math.hypot(x.x_km-o.footprint[0].x_km,x.y_km-o.footprint[0].y_km)))*1e3<34;if(l)h.kind=rh;else if(o.facade==="curtainWall")h.kind=Wy,h.moduleM=1.2+.6*Un(o.id,10),h.unitW=h.moduleM*3,h.glassKind=3,h.glass=[[.42,.66,.66],[.38,.52,.74],[.66,.7,.72],[.3,.33,.36]][Math.floor(Un(o.id,11)*4)],h.frame=[.075,.08,.085],h.window=[0,h.unitW,.9,h.storeyM];else if(o.facade==="concreteGlass"||o.facade==="stoneCivic"){h.kind=o.facade==="stoneCivic"?C1:Xy,h.unitW=(o.facade==="stoneCivic"?4.9:4.6)*(C?.78:1);const x=h.kind===C1?.9:.3;h.window=[x,h.unitW-x,.8,h.storeyM-.5],h.glassKind=2}else{h.kind=Yy,h.unitW=S?3.3:3.6;const x=!!o.balconyBays&&o.balconyBays>0&&o.useType!=="village";h.window=x?[.35,1.55,.9,Math.min(2.4,h.storeyM-.45)]:[h.unitW*.5-.9,h.unitW*.5+.9,.9,Math.min(2.4,h.storeyM-.45)],x&&(h.balcony=[1.75,h.unitW-.35,h.storeyM-.45])}return h}function b3(o,l){const r=l.metresToScene,u=new U1,i=new Map;for(const z of o.parcels)i.set(z.id,ys(o.frame,Vu(z.ring)));const f=new Float32Array(ch),h=z=>z.map(R=>l.toScene(R.x_km,R.y_km));for(const z of o.blocks)u.select(...ys(o.frame,Vu(z.boundary))),u.seed=Wo.lawn+Un(z.boundary.length,9)*.999,u.flat(h(z.boundary),l.heightAt,.15*r,7109198);const v={park:5139254,residential:6716492,villa:6256454,farmstead:7042119,village:8026722,commercial:7302505,mixedUse:7631469,civic:10131084,industrial:9209724},m=(z,R)=>{const V=R*7919%100/100;switch(z){case"industrial":return V<.55?{colour:v.industrial,kind:Wo.yard}:{colour:6447196,kind:Wo.asphaltLot};case"commercial":case"mixedUse":return V<.7?{colour:v[z],kind:Wo.parking}:{colour:v.civic,kind:Wo.plaza};case"civic":return{colour:v.civic,kind:Wo.plaza};case"park":case"residential":case"villa":case"farmstead":return{colour:v[z],kind:Wo.lawn};default:return{colour:v[z]??f3,kind:o0}}};for(const z of o.parcels){u.select(...ys(o.frame,Vu(z.ring)));const R=m(z.useType,z.id);u.seed=R.kind+Un(z.id,6)*.999,u.flat(h(z.ring),l.heightAt,.25*r,R.colour)}const p=[];for(const z of o.hdRoads){const R=z.centreline[Math.floor(z.centreline.length/2)];R&&u.select(...ys(o.frame,R));const V=z.centreline.map(ne=>{const[B,ae]=l.toScene(ne.x_km,ne.y_km);return[B,l.heightAt(B,ae),ae]});if(V.length<2)continue;if(z.bridge){const ne=V[0],B=V[V.length-1];V.forEach((ae,I)=>{ae[1]=ne[1]+(B[1]-ne[1])*(I/(V.length-1))})}for(const ne of V)ne[1]+=.35*r;const J=Math.max(z.widthMetres,6),Q=z.layer?0:d3[z.class]??0;if(u.ribbon(V,(J-2*Q)*r,u3),Q>0)for(const ne of[-1,1])u.ribbon(O1(V,ne*(J-Q)*.5*r),Q*r,h3);Q>=2.4&&p.push({pts:V,offset:(J*.5-Q+1.05)*r,tile:R?ys(o.frame,R):null})}u.seed=o0;for(const z of o.trees??[]){if(!z.point)continue;u.select(...ys(o.frame,z.point));const[R,V]=l.toScene(z.point.x_km,z.point.y_km),J=l.heightAt(R,V),Q=Fy(z.species),ne=S1(Q);k1(u,R,J,V,(z.heightMetres??ne.height)*r,Math.max(z.crownRadiusMetres??ne.crown,1.5)*r,Q)}for(const z of p){z.tile&&u.select(...z.tile);for(const R of[-1,1]){const V=O1(z.pts,R*z.offset);let J=4;for(let Q=0;Q+1<V.length;Q+=1){const[ne,B,ae]=V[Q],[I,ge,Re]=V[Q+1],le=Math.hypot(I-ne,Re-ae);for(;J<le;){const G=J/le,j=ne+(I-ne)*G,me=ae+(Re-ae)*G,X=Vy(o.streetMix,j,me),we=S1(X);k1(u,j,B+(ge-B)*G-.35*r,me,we.height*r,we.crown*r,X),J+=8*r}J-=le}}}u.seed=0;const b=[],S=(z,R)=>{const V=Math.max(1,Math.round((R-z.shopM)/Math.max(z.storeyM,2.4)));return b.push({look:z,storeys:V}),b.length-1};u.plainBld=S(pp([.5,.5,.5]),3);for(const z of o.buildings){u.select(...i.get(z.parcelId)??ys(o.frame,Vu(z.footprint)));const R=h(z.footprint);if(R.length<3)continue;let V=1/0;for(const[ce,ue]of R)V=Math.min(V,l.heightAt(ce,ue));const J=z.farLook?.startM??0,Q=J>0?V+J*r:V-1.5*r,ne=z.useType==="industrial"&&z.floors<=1,B=z.farLook??w3(z,ne),ae=z.farLook?B.roofM:Math.max(z.heightMetres,3),I=ae*r,ge=Qf(B.wall),Re=z.farLook?Qf(B.roof):ne?R1[Math.floor(Un(z.id,2)*R1.length)]:z.roof==="terracotta"?D1:E1[Math.floor(Un(z.id,2)*E1.length)],le=V+I;u.seed=Un(z.id,61)*.999+(ne?m3:0);const G=Qf(B.topN>0||B.baseN>0?B.topBand:B.wall),j=ne||ae<9?0:z.useType==="commercial"?z.floors>=12?1.6:1.1:1.2,me=z.floors>=19,X=z.farLook?B.crown===2&&!me||B.crown===4&&!me&&z.floors<=7||B.crown===3&&!me:!1,we=!z.farLook&&(z.roof==="gable"||z.roof==="hip")&&R.length===4,E=(X||we)&&R.length===4,Z=z.farLook?Re:z.roof==="terracotta"?D1:8215114,T=B.podiumFloors>0?(B.shopM+B.podiumFloors*B.podiumStoreyM)*r:0,F=J===0&&T>0&&T<I-6*r?gp(R,B.setbackM*r):null;if(F)u.bld=S(Py(B),T/r),u.prism(R,Q,V+T,ge,Re,V,!0),u.bld=u.plainBld,u.parapet(R,V+T,1.1*r,G),u.bld=S(_1(B),(I-T)/r),u.prism(F,V+T,le,ge,Re,V+T,!0);else if(z.tierRing&&z.tierRing.length>=3&&(z.podiumHeightMetres??0)>0&&(z.podiumHeightMetres??0)*r<I){const ce=(z.podiumHeightMetres??0)*r;u.bld=S(B,z.podiumHeightMetres??0),u.prism(R,Q,V+ce,ge,Re,V,!0),u.bld=S(_1(B),ae-(z.podiumHeightMetres??0)),u.prism(h(z.tierRing),V+ce,le,ge,Re,V+ce,!0)}else if(u.bld=S(B,ae-J),u.prism(R,Q,le,ge,Re,J>0?Q:V,!E),E){u.bld=u.plainBld;const ce=X||z.roof==="hip";u.pitchedRoof(R,le,.6*r,B.crown===3?.7:X?.5:.62,ce,Z,ge)}u.bld=u.plainBld,!E&&j>0&&u.parapet(F??R,le,j*r,G),E||p3(u,z,F??R,le,ge,ne,r,z.farLook?B.solar:void 0),u.bld=-1}const C=new U1,x=z=>{const[R,V]=ys(o.frame,z),J=a0(R,V);let Q=u.tileIndex.get(J);Q===void 0&&(Q=u.tileIndex.size,u.tileIndex.set(J,Q)),C.tileIndex.set(J,Q),C.select(R,V)};o.river&&o.river.length>=2&&B1(C,o.river,o.riverWidthMetres??60,l,x,o.riverSetting);for(const z of o.tributaries??[])z.line.length>=2&&B1(C,z.line,z.widthMetres,l,x,o.riverSetting);let D=null,H=null;if(C.triangles()>0){H=new Ke({vertexColors:!0,roughness:.32,metalness:0,envMapIntensity:.4,polygonOffset:!0,polygonOffsetFactor:-3,polygonOffsetUnits:-3}),H.onBeforeCompile=R=>{R.uniforms.uHidden={value:f},R.vertexShader=R.vertexShader.replace("#include <common>",`#include <common>
attribute float aTile;
varying float vTile;
varying vec3 vWaterWorld;`).replace("#include <worldpos_vertex>",`#include <worldpos_vertex>
vTile = aTile;
vWaterWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;`),R.fragmentShader=R.fragmentShader.replace("#include <common>",`#include <common>
uniform float uHidden[256];
varying float vTile;
varying vec3 vWaterWorld;`).replace("#include <color_fragment>",`if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;
#include <color_fragment>`).replace("#include <emissivemap_fragment>",`#include <emissivemap_fragment>
          {
            // Sky on the surface: a little face-on, most of it at a grazing angle.
            float facing = clamp(abs(dot(normalize(vViewPosition), normal)), 0.0, 1.0);
            float fresnel = pow(1.0 - facing, 3.0);
            vec3 sky = vec3(0.30, 0.44, 0.60);
            // A broad, slow, non-repeating swell moves the reflection about; it is
            // value noise on world position, averaged out once a pixel spans it, so
            // from the air the river is a smooth sky reflection.
            vec2 wp = vWaterWorld.xz / 0.0016;
            vec2 wi = floor(wp); vec2 wf = fract(wp); wf = wf * wf * (3.0 - 2.0 * wf);
            float h00 = fract(sin(dot(wi, vec2(127.1, 311.7))) * 43758.5453);
            float h10 = fract(sin(dot(wi + vec2(1.0, 0.0), vec2(127.1, 311.7))) * 43758.5453);
            float h01 = fract(sin(dot(wi + vec2(0.0, 1.0), vec2(127.1, 311.7))) * 43758.5453);
            float h11 = fract(sin(dot(wi + vec2(1.0, 1.0), vec2(127.1, 311.7))) * 43758.5453);
            float swell = mix(mix(h00, h10, wf.x), mix(h01, h11, wf.x), wf.y);
            float pix = max(length(dFdx(wp)), length(dFdy(wp)));
            float ripple = mix(swell, 0.5, smoothstep(0.35, 1.2, pix));
            totalEmissiveRadiance += sky * (0.02 + 0.55 * fresnel) * (0.88 + 0.24 * ripple);
          }`)};const z=C.geometry();D=new xt(z,H),D.name="cityWater",D.receiveShadow=!1,D.castShadow=!1,D.renderOrder=3}const q=u.geometry(),P=Zy(b),K=new Ke({vertexColors:!0,roughness:.92,metalness:0,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2});K.onBeforeCompile=z=>{z.uniforms.uMetre={value:r},z.uniforms.uHidden={value:f},z.uniforms.uMassLook={value:P},z.vertexShader=z.vertexShader.replace("#include <common>",`#include <common>
attribute float aTile;
attribute float aBase;
attribute float aSeed;
varying float vTile;
varying float vBase;
varying float vSeed;
varying vec3 vMassWorld;
varying vec3 vMassNormal;
`+Qy).replace("#include <worldpos_vertex>",`#include <worldpos_vertex>
vTile = aTile;
vBase = aBase;
vSeed = aSeed;
vMassWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;
vMassNormal = normalize(mat3(modelMatrix) * objectNormal);
`+Iy),z.fragmentShader=z.fragmentShader.replace("#include <common>",`#include <common>
`+n3+`uniform float uMetre;
uniform float uHidden[256];
varying float vTile;
varying float vBase;
varying float vSeed;
varying vec3 vMassWorld;
varying vec3 vMassNormal;
`+lp+Jy).replace("#include <color_fragment>",`if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;
        ${$y}
        // Open ground of a parcel: joints, bays, containers, stains by use.
        if (seedKind > 3.5 && vMassNormal.y > 0.9) {
          vec2 gp = vMassWorld.xz / uMetre;
          diffuseColor.rgb = groundSurface(seedKind, diffuseColor.rgb, gp, length(fwidth(gp)), fract(vSeed));
        }
        `).replace("#include <roughnessmap_fragment>",e3).replace("#include <lights_physical_fragment>",t3).replace("#include <emissivemap_fragment>",a3)},K.customProgramCacheKey=()=>"city-mass-look";const U=new xt(q,K);return U.name="cityMass",U.castShadow=!0,U.receiveShadow=!0,{mesh:U,water:D,triangles:u.triangles()+C.triangles(),setTileHidden(z,R,V){const J=u.tileIndex.get(a0(z,R));J!==void 0&&J<ch&&(f[J]=V?1:0)},dispose(){q.dispose(),K.dispose(),P.dispose(),D?.geometry.dispose(),H?.dispose()}}}const H1=1.7,M3=4.5,S3=4,T3=9e3,z3=.9,C3=new Set(["KeyW","KeyA","KeyS","KeyD","KeyQ","KeyE","Space","ArrowUp","ArrowDown","ArrowLeft","ArrowRight","ShiftLeft","ShiftRight","ControlLeft","ControlRight"]);class _3{constructor(l){this.host=l;const r=(m,p)=>{window.addEventListener(m,p),this.cleanup.push(()=>window.removeEventListener(m,p))};r("keydown",m=>{!this.active||this.typing(m)||C3.has(m.code)&&(this.keys.add(m.code),m.preventDefault())}),r("keyup",m=>{this.keys.delete(m.code)}),r("blur",()=>this.keys.clear());const u=l.dom,i=m=>{this.active&&(this.dragging=!0,this.pointer=m.pointerId,u.setPointerCapture?.(m.pointerId))},f=m=>{!this.active||!this.dragging||m.pointerId!==this.pointer||(this.yaw-=m.movementX*.0026,this.pitch=Ge.clamp(this.pitch-m.movementY*.0026,-1.5,1.5))},h=m=>{m.pointerId===this.pointer&&(this.dragging=!1,this.pointer=-1,u.releasePointerCapture?.(m.pointerId))},v=m=>{this.active&&(m.preventDefault(),this.speedScale=Ge.clamp(this.speedScale*Math.exp(-m.deltaY*.0015),.05,20))};u.addEventListener("pointerdown",i),u.addEventListener("pointermove",f),u.addEventListener("pointerup",h),u.addEventListener("pointercancel",h),u.addEventListener("wheel",v,{passive:!1}),this.cleanup.push(()=>{u.removeEventListener("pointerdown",i),u.removeEventListener("pointermove",f),u.removeEventListener("pointerup",h),u.removeEventListener("pointercancel",h),u.removeEventListener("wheel",v)})}active=!1;walking=!1;groundTarget=new rt;speedMs=0;onChange=null;yaw=0;pitch=0;speedScale=1;keys=new Set;dragging=!1;pointer=-1;forward=new rt;right=new rt;euler=new dh(0,0,0,"YXZ");cleanup=[];typing(l){const r=l.target;return!!(r&&/^(INPUT|SELECT|TEXTAREA)$/.test(r.tagName))}enter(l=!1){const r=this.host.camera;r.up.set(0,1,0),r.getWorldDirection(this.forward),this.yaw=Math.atan2(-this.forward.x,-this.forward.z),this.pitch=Math.asin(Ge.clamp(this.forward.y,-1,1)),this.active=!0,this.walking=l,this.keys.clear(),l&&this.snapToEye(),this.onChange?.()}exit(){this.active&&(this.active=!1,this.dragging=!1,this.keys.clear(),this.onChange?.())}setWalking(l){this.walking!==l&&(this.walking=l,l&&(this.pitch=Math.max(this.pitch,-.35),this.snapToEye()),this.onChange?.())}snapToEye(){const{camera:l,metresToScene:r}=this.host;l.position.y=this.host.groundHeight(l.position.x,l.position.z)+H1*r}update(l){if(!this.active)return;const{camera:r,metresToScene:u,halfExtent:i}=this.host,f=Math.min(l,.1);this.euler.set(this.pitch,this.yaw,0),r.quaternion.setFromEuler(this.euler);const h=this.host.groundHeight(r.position.x,r.position.z),v=Math.max(0,(r.position.y-h)/u),m=this.keys.has("ShiftLeft")||this.keys.has("ShiftRight")?4:1,p=this.walking?M3:Ge.clamp(v*z3,S3,T3);this.speedMs=p*m*this.speedScale;const b=this.speedMs*u*f,S=this.keys,C=(S.has("KeyW")||S.has("ArrowUp")?1:0)-(S.has("KeyS")||S.has("ArrowDown")?1:0),x=(S.has("KeyD")||S.has("ArrowRight")?1:0)-(S.has("KeyA")||S.has("ArrowLeft")?1:0),D=(S.has("Space")||S.has("KeyE")?1:0)-(S.has("KeyQ")||S.has("ControlLeft")||S.has("ControlRight")?1:0);this.walking?this.forward.set(-Math.sin(this.yaw),0,-Math.cos(this.yaw)):r.getWorldDirection(this.forward),this.right.set(-this.forward.z,0,this.forward.x),this.right.lengthSq()>1e-9&&this.right.normalize(),r.position.addScaledVector(this.forward,C*b),r.position.addScaledVector(this.right,x*b),this.walking||(r.position.y+=D*b);const H=i*.998;r.position.x=Ge.clamp(r.position.x,-H,H),r.position.z=Ge.clamp(r.position.z,-H,H);const q=this.host.groundHeight(r.position.x,r.position.z),P=H1*u;if(this.walking){const K=q+P;r.position.y+=(K-r.position.y)*Math.min(1,f*14)}else r.position.y<q+P&&(r.position.y=q+P);r.updateMatrixWorld(),this.updateGroundTarget(q,v)}updateGroundTarget(l,r){const{camera:u,metresToScene:i}=this.host;u.getWorldDirection(this.forward);const h=Math.max(30,r*14)*i;let v=h;if(this.forward.y<-1e-4){let m=0,p=Math.max(2*i,h/64),b=0;for(let S=0;S<96&&m<h;S+=1){m+=p;const C=u.position.x+this.forward.x*m,x=u.position.z+this.forward.z*m;if(u.position.y+this.forward.y*m<=this.host.groundHeight(C,x)){let H=b,q=m;for(let P=0;P<6;P+=1){const K=(H+q)*.5,U=u.position.x+this.forward.x*K,z=u.position.z+this.forward.z*K;u.position.y+this.forward.y*K<=this.host.groundHeight(U,z)?q=K:H=K}v=Math.min(h,q);break}b=m,p*=1.08}}this.groundTarget.copy(u.position).addScaledVector(this.forward,Math.max(v,1.2*i))}dispose(){for(const l of this.cleanup)l();this.cleanup.length=0}}new rt(-.58,.55,-.6).normalize();const A3=new $t().setRGB(.2,.195,.185,Iv);function E3(o,l){const r=new Jv(o),u=new P1,i=new xt(new ws(100,32,20),l);i.frustumCulled=!1,u.add(i);const f=new xt(new ws(90,32,10,0,Math.PI*2,Math.PI*.515,Math.PI*.485),new Dn({color:A3,side:Z1,fog:!1}));f.renderOrder=1,f.frustumCulled=!1,u.add(f);const h=r.fromScene(u,.04).texture;return i.geometry.dispose(),f.geometry.dispose(),f.material.dispose(),r.dispose(),h}function Nn(o,l=!1){const r=o[0].index!==null,u=new Set(Object.keys(o[0].attributes)),i=new Set(Object.keys(o[0].morphAttributes)),f={},h={},v=o[0].morphTargetsRelative,m=new sa;let p=0;for(let b=0;b<o.length;++b){const S=o[b];let C=0;if(r!==(S.index!==null))return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+b+". All geometries must have compatible attributes; make sure index attribute exists among all geometries, or in none of them."),null;for(const x in S.attributes){if(!u.has(x))return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+b+'. All geometries must have compatible attributes; make sure "'+x+'" attribute exists among all geometries, or in none of them.'),null;f[x]===void 0&&(f[x]=[]),f[x].push(S.attributes[x]),C++}if(C!==u.size)return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+b+". Make sure all geometries have the same number of attributes."),null;if(v!==S.morphTargetsRelative)return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+b+". .morphTargetsRelative must be consistent throughout all geometries."),null;for(const x in S.morphAttributes){if(!i.has(x))return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+b+".  .morphAttributes must be consistent throughout all geometries."),null;h[x]===void 0&&(h[x]=[]),h[x].push(S.morphAttributes[x])}if(l){let x;if(r)x=S.index.count;else if(S.attributes.position!==void 0)x=S.attributes.position.count;else return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+b+". The geometry must have either an index or a position attribute"),null;m.addGroup(p,x,b),p+=x}}if(r){let b=0;const S=[];for(let C=0;C<o.length;++C){const x=o[C].index;for(let D=0;D<x.count;++D)S.push(x.getX(D)+b);b+=o[C].attributes.position.count}m.setIndex(S)}for(const b in f){const S=L1(f[b]);if(!S)return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed while trying to merge the "+b+" attribute."),null;m.setAttribute(b,S)}for(const b in h){const S=h[b][0].length;if(S!==0){m.morphAttributes=m.morphAttributes||{},m.morphAttributes[b]=[];for(let C=0;C<S;++C){const x=[];for(let H=0;H<h[b].length;++H)x.push(h[b][H][C]);const D=L1(x);if(!D)return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed while trying to merge the "+b+" morphAttribute."),null;m.morphAttributes[b].push(D)}}}return m}function L1(o){let l,r,u,i=-1,f=0;for(let p=0;p<o.length;++p){const b=o[p];if(l===void 0&&(l=b.array.constructor),l!==b.array.constructor)return console.error("THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.array must be of consistent array types across matching attributes."),null;if(r===void 0&&(r=b.itemSize),r!==b.itemSize)return console.error("THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.itemSize must be consistent across matching attributes."),null;if(u===void 0&&(u=b.normalized),u!==b.normalized)return console.error("THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.normalized must be consistent across matching attributes."),null;if(i===-1&&(i=b.gpuType),i!==b.gpuType)return console.error("THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.gpuType must be consistent across matching attributes."),null;f+=b.count*r}const h=new l(f),v=new ya(h,r,u);let m=0;for(let p=0;p<o.length;++p){const b=o[p];if(b.isInterleavedBufferAttribute){const S=m/r;for(let C=0,x=b.count;C<x;C++)for(let D=0;D<r;D++){const H=b.getComponent(C,D);v.setComponent(C+S,D,H)}}else h.set(b.array,m);m+=b.count*r}return i!==void 0&&(v.gpuType=i),v}function yp(o){const l=atob(o),r=new Uint8Array(l.length);for(let u=0;u<l.length;u+=1)r[u]=l.charCodeAt(u);return r}function If(o){const l=yp(o);return new Float32Array(l.buffer,l.byteOffset,l.byteLength>>2)}function Jf(o,l,r){const u=[];for(const f of o??[]){if(f.lod!==l)continue;const h=If(f.positions),v=f.heightMetres*r,m=new Float32Array(h.length);for(let S=0;S<h.length;S+=1)m[S]=h[S]*v;const p=yp(f.indices),b=new sa;b.setAttribute("position",new ya(m,3)),b.setAttribute("normal",new ya(If(f.normals).slice(),3)),b.setAttribute("color",new ya(If(f.colors).slice(),3)),b.setIndex(new ya(new Uint16Array(p.buffer.slice(p.byteOffset,p.byteOffset+p.byteLength)),1)),b.computeBoundingSphere(),u.push(b)}if(!u.length){const f=new $v(3.8*r,18*r,5,1);f.translate(0,9*r,0);const h=new Float32Array(f.attributes.position.count*3);for(let v=0;v<h.length;v+=3)h[v]=.02,h[v+1]=.05,h[v+2]=.02;f.setAttribute("color",new ya(h,3)),u.push(f)}const i=new Ke({vertexColors:!0,roughness:.94,metalness:0});return{geometries:u,material:i,dispose(){for(const f of u)f.dispose();i.dispose()}}}function R3(o){let l=o>>>0;return()=>(l=Math.imul(l,1664525)+1013904223>>>0,l/4294967296)}function N3(o){const l=R3(2135587861),r=[],u=[],i=[],f=9;for(let v=0;v<f;v+=1){const m=v/f*Math.PI*2+l()*.6,p=(.2+l()*.24)*o,b=(.011+l()*.008)*o,S=(.05+l()*.16)*o,C=l()*.05*o,x=Math.cos(m),D=Math.sin(m),H=-D,q=x,P=x*C,K=D*C,U=.85+l()*.3,z=3,R=[];for(let J=0;J<=z;J+=1){const Q=J/z;R.push({x:P+x*S*Q*Q,y:p*Q-p*.1*Q*Q,z:K+D*S*Q*Q,w:b*(1-Q*Q*.92),v:(.38+.62*Q)*U})}const V=(J,Q)=>{r.push(J.x+H*J.w*Q,J.y,J.z+q*J.w*Q);const ne=new rt(x*.35+H*Q*.25,1,D*.35+q*Q*.25).normalize();u.push(ne.x,ne.y,ne.z),i.push(J.v*.9,J.v,J.v*(.62+.2*(1-J.v)))};for(let J=0;J<z;J+=1){const Q=R[J],ne=R[J+1];J===z-1?(V(Q,-1),V(Q,1),V(ne,0)):(V(Q,-1),V(Q,1),V(ne,1),V(Q,-1),V(ne,1),V(ne,-1))}}const h=new sa;return h.setAttribute("position",new st(r,3)),h.setAttribute("normal",new st(u,3)),h.setAttribute("color",new st(i,3)),h}function D3(o){const l=new ws(.5*o,11,8),r=l.attributes.position,u=new Float32Array(r.count*3);for(let i=0;i<r.count;i+=1){const f=r.getX(i)/o,h=r.getY(i)/o,v=r.getZ(i)/o,m=.86+.2*Math.sin(f*19+v*7)*Math.cos(v*17-h*9)+.1*Math.sin(h*31+f*23);r.setXYZ(i,f*m*1.15*o,Math.max(-.05,h*m*.78+.3)*o,v*m*1.15*o);const p=Ge.clamp(h+.5,0,1),b=.5+.5*Math.sin(f*57+h*43)*Math.sin(v*61-h*37),S=.42+.5*p+.3*b;u[i*3]=S*.82,u[i*3+1]=S,u[i*3+2]=S*.55}return l.setAttribute("color",new ya(u,3)),l.computeVertexNormals(),l}function U3(o){const l=[],r=[],u=.24*o,i=.006*o;for(const[m,p]of[[1,0],[0,1]]){l.push(-m*i,0,-p*i,m*i,0,p*i,m*i*.6,u,p*i*.6),l.push(-m*i,0,-p*i,m*i*.6,u,p*i*.6,-m*i*.6,u,-p*i*.6);for(let b=0;b<6;b+=1)r.push(.16,.42,.06)}const f=.034*o;for(let m=0;m<5;m+=1){const p=m/5*Math.PI*2,b=(m+.5)/5*Math.PI*2,S=(m+1)/5*Math.PI*2;l.push(0,u+.004*o,0),l.push(Math.cos(p)*f*.6,u+.01*o,Math.sin(p)*f*.6),l.push(Math.cos(b)*f,u+.012*o,Math.sin(b)*f),l.push(0,u+.004*o,0),l.push(Math.cos(b)*f,u+.012*o,Math.sin(b)*f),l.push(Math.cos(S)*f*.6,u+.01*o,Math.sin(S)*f*.6);for(let C=0;C<6;C+=1)r.push(C%3===0?.55:1,C%3===0?.42:1,C%3===0?.1:1)}const h=new sa;h.setAttribute("position",new st(l,3)),h.setAttribute("color",new st(r,3)),h.computeVertexNormals();const v=h.attributes.normal;for(let m=0;m<v.count;m+=1)v.setXYZ(m,0,1,0);return h}function O3(o,l){const r=new ka,u=[],i=[],f=(ne,B,ae)=>{const I=new Ke({vertexColors:!0,roughness:ae,metalness:0,side:ct,transparent:!1});u.push(I),i.push(ne);const ge=new Ka(ne,I,B);return ge.instanceMatrix.setUsage(Xu),ge.count=0,ge.frustumCulled=!1,ge.castShadow=!1,ge.receiveShadow=!0,r.add(ge),{mesh:ge,capacity:B,count:0}},h=f(N3(o),4e4,1),v=f(D3(o),4e3,.95),m=f(U3(o),9e3,.8),p=[h,v,m];r.visible=!1;let b=Number.POSITIVE_INFINITY,S=Number.POSITIVE_INFINITY,C=0,x=0,D=0;const H=new Mo,q=new Pu,P=new dh,K=new rt,U=new rt,z=new $t,R=new rt(0,1,0),V=[16052454,16177226,14243215,9071568,15764028],J=(ne,B,ae)=>ne.count>=ne.capacity?!1:(ne.mesh.setMatrixAt(ne.count,B),ne.mesh.setColorAt(ne.count,ae),ne.count+=1,!0),Q=(ne,B,ae,I)=>{const ge=ne/o*B,Re=Ge.clamp(ge*.62,10,130),le=Re<=22?.42:Re<=50?.75:Re<=90?1.25:1.9,G=Math.hypot(ae-b,I-S)/o,j=Math.abs(Re-C)/Math.max(1,C);if(G<le*4&&j<.12&&x===le)return;b=ae,S=I,C=Re,x=le;for(const Te of p)Te.count=0;const me=(b+1.6)/o,X=(S+1.6)/o,we=Math.floor((me-Re)/le),E=Math.ceil((me+Re)/le),Z=Math.floor((X-Re)/le),T=Math.ceil((X+Re)/le),F=l.hash01,ue=Math.max(1.5,le)*o,Ee=34;for(let Te=Z;Te<=T;Te+=1)for(let Ue=we;Ue<=E;Ue+=1){const oe=(Ue+.08+F(Ue,Te,101)*.84)*le,_e=(Te+.08+F(Ue,Te,102)*.84)*le,Ze=oe-me,Ye=_e-X,Xe=Math.hypot(Ze,Ye);if(Xe>Re)continue;const tt=-1.6+oe*o,ot=-1.6+_e*o;if(tt<=-1.6||tt>=1.6||ot<=-1.6||ot>=1.6||l.excluded(tt,ot)>48||l.water(tt,ot)>24||l.elevation01(tt,ot)>.8)continue;const fa=l.height(tt,ot),Ca=Math.hypot(l.height(tt+ue,ot)-l.height(tt-ue,ot),l.height(tt,ot+ue)-l.height(tt,ot-ue))/(2*ue);if(Ca>.85)continue;const ta=l.forest(tt,ot)/255,Ha=Math.floor(oe/Ee),Xa=Math.floor(_e/Ee),an=F(Ha,Xa,111),qt=F(Ha,Xa,112)>.9,to=F(Ha,Xa,113)>.86,La=1-Ge.smoothstep(Xe,Re*.72,Re);if(La<=.02)continue;const ao=(qt?.1:1)*(1-Ge.smoothstep(Ca,.35,.85))*(1-ta*.45);if(F(Ue,Te,103)>ao*La)continue;const Pa=F(Ue,Te,104)*Math.PI*2,ut=(F(Ue,Te,105)-.5)*.22;q.setFromEuler(P.set(ut,Pa,ut*.7));const xa=(.7+F(Ue,Te,106)*.85)*(.6+.4*La)*(1.12-an*.32);if(K.set(tt,fa,ot),U.set(xa*(.85+F(Ue,Te,107)*.5),xa,xa*(.85+F(Ue,Te,108)*.5)),H.compose(K,q,U),z.setHSL(.245-an*.125+(F(Ue,Te,109)-.5)*.03,.34+(1-an)*.18,.25+F(Ue,Te,110)*.14+an*.05),J(h,H,z),!qt&&an<.55&&ta<.5&&F(Ue,Te,120)>.93&&La>.4){const on=F(Ue,Te,121)*Math.PI*2;q.setFromAxisAngle(R,on),K.set(tt+(F(Ue,Te,122)-.5)*le,fa,ot+(F(Ue,Te,123)-.5)*le);const ln=.8+F(Ue,Te,124)*.7;U.set(ln,ln,ln),H.compose(K,q,U),z.setHex(V[Math.floor(F(Ha,Xa,125)*V.length)%V.length]),J(m,H,z)}const nn=(to?.16:.006)+ta*.03;if(!qt&&F(Ue,Te,130)<nn&&La>.55&&Ca<.6){const on=(.7+F(Ue,Te,131)*1.5)*(to?1.1:.9);q.setFromAxisAngle(R,F(Ue,Te,132)*Math.PI*2),K.set(tt,fa-.04*o,ot),U.set(on*(.9+F(Ue,Te,133)*.4),on*(.75+F(Ue,Te,134)*.5),on*(.9+F(Ue,Te,135)*.4)),H.compose(K,q,U),z.setHSL(.27-an*.09+(F(Ue,Te,136)-.5)*.05,.4,.13+F(Ue,Te,137)*.09),J(v,H,z)}}for(const Te of p)Te.mesh.count=Te.count,Te.mesh.instanceMatrix.needsUpdate=!0,Te.mesh.instanceColor&&(Te.mesh.instanceColor.needsUpdate=!0)};return{group:r,setFade(ne){D=ne,r.visible=D>.001},update(ne,B,ae,I){D<=.001||Q(ne,B,ae,I)},dispose(){for(const ne of i)ne.dispose();for(const ne of u)ne.dispose();r.removeFromParent()}}}const $u=.21,xo=[1040,560],xp=12,uh=6,wp=2.5;var di=(o=>(o[o.Cotton=0]="Cotton",o[o.Maize=1]="Maize",o[o.Wheat=2]="Wheat",o[o.Stubble=3]="Stubble",o[o.Soil=4]="Soil",o[o.Alfalfa=5]="Alfalfa",o))(di||{});function Ku(o){return o-Math.floor(o)}function $f(o,l){let r=Ku(o*.1031),u=Ku(l*.1031),i=Ku(o*.1031);const f=r*(u+33.33)+u*(i+33.33)+i*(r+33.33);return r+=f,u+=f,i+=f,Ku((r+u)*i)}function k3(o){return o<.34?0:o<.54?1:o<.68?2:o<.78?3:o<.86?4:5}function j1(o,l){const r=Math.cos($u),u=Math.sin($u),i=r*o+u*l,f=-u*o+r*l,h=Math.floor(i/xo[0]),v=Math.floor(f/xo[1]),m=i-h*xo[0],p=f-v*xo[1],b=3+Math.floor($f(h,v)*3),S=xo[1]/b,C=Math.floor(p/S),x=$f(h+17,v-5)>.5,D=x&&m>xo[0]*.5?1:0,H=k3($f(h*8+C,v*8+D*4+1)),q=Math.min(p,xo[1]-p),P=Math.min(Math.abs(p-C*S),Math.abs(p-(C+1)*S)),K=Math.min(m,xo[0]-m,x?Math.abs(m-xo[0]*.5):1/0),U=q<xp+uh||P<wp*.5+.6||K<uh*.5+.6;return{crop:H,margin:U,rowAngle:-$u}}const B3=`
const float FARM_ANGLE = ${$u.toFixed(4)};
const vec2 FARM_BLOCK = vec2(${xo[0].toFixed(1)}, ${xo[1].toFixed(1)});
const float FARM_BELT = ${xp.toFixed(1)};
const float FARM_ROAD = ${uh.toFixed(1)};
const float FARM_BUND = ${wp.toFixed(1)};
float fHash(vec2 p) {
  vec3 p3 = fract(vec3(p.xyx) * 0.1031);
  p3 += dot(p3, p3.yzx + 33.33);
  return fract((p3.x + p3.y) * p3.z);
}
// A line \`half\` wide at distance \`d\`, antialiased by the pixel \`fw\`, and
// averaged to its share of a \`period\` once the pixel outgrows it.
float farmLine(float d, float halfW, float fw, float period) {
  float m = clamp((halfW - d) / max(fw, 1e-3) + 0.5, 0.0, 1.0);
  return mix(m, min(1.0, 2.0 * halfW / period), smoothstep(halfW, halfW * 5.0, fw));
}
vec3 farmCrop(float h, vec2 q, float foot) {
  // Rows along the block, and a plant's worth of grain within them.
  float rowsNear = 1.0 - smoothstep(0.12, 0.5, foot);
  float rows = 0.5 + 0.5 * sin(q.y * 8.27);
  float tone = 0.86 + 0.28 * dFbm(q / 55.0) + 0.10 * (dNoise(q / 6.0) - 0.5) * (1.0 - smoothstep(2.0, 8.0, foot));
  vec3 soil = vec3(0.16, 0.115, 0.075);
  vec3 c;
  if (h < 0.34) {
    c = mix(soil * 1.1, vec3(0.075, 0.115, 0.042), 0.55 + 0.45 * mix(0.8, rows, rowsNear));
  } else if (h < 0.54) {
    c = mix(soil, vec3(0.048, 0.082, 0.028), 0.75 + 0.25 * mix(0.85, rows, rowsNear));
  } else if (h < 0.68) {
    c = vec3(0.36, 0.28, 0.115) * (0.92 + 0.12 * dNoise(q / 1.5));
  } else if (h < 0.78) {
    float bales = (1.0 - smoothstep(0.0, 0.5, abs(fract(q.y / 9.0) - 0.5) * 9.0)) * rowsNear;
    c = mix(vec3(0.33, 0.28, 0.17), vec3(0.42, 0.36, 0.21), 0.5 + 0.5 * sin(q.y * 2.1)) * (1.0 + 0.2 * bales);
  } else if (h < 0.86) {
    float furrow = 0.5 + 0.5 * sin(q.y * 6.28 / 0.9);
    c = soil * (0.82 + 0.3 * mix(0.5, furrow, rowsNear));
  } else {
    c = vec3(0.085, 0.13, 0.05) * (0.9 + 0.2 * dNoise(q / 3.0));
  }
  return c * tone;
}
vec3 farmColour(vec2 p, float foot) {
  float c = cos(FARM_ANGLE), s = sin(FARM_ANGLE);
  vec2 q = vec2(c * p.x + s * p.y, -s * p.x + c * p.y);
  vec2 b = floor(q / FARM_BLOCK);
  vec2 f = q - b * FARM_BLOCK;
  float strips = 3.0 + floor(fHash(b) * 3.0);
  float width = FARM_BLOCK.y / strips;
  float strip = floor(f.y / width);
  bool halves = fHash(b + vec2(17.0, -5.0)) > 0.5;
  float split = halves && f.x > FARM_BLOCK.x * 0.5 ? 1.0 : 0.0;
  vec3 col = farmCrop(fHash(vec2(b.x * 8.0 + strip, b.y * 8.0 + split * 4.0 + 1.0)), q, foot);
  // From far off a strip is narrower than a pixel: its crops average.
  vec3 mean = vec3(0.13, 0.13, 0.07);
  col = mix(col, mean, smoothstep(width * 0.25, width * 1.2, foot));
  // Bunds between strips, the road across a halved block, block ends.
  float bund = min(abs(f.y - strip * width), abs(f.y - (strip + 1.0) * width));
  col = mix(col, vec3(0.19, 0.16, 0.115), farmLine(bund, FARM_BUND * 0.5, foot, width));
  float end = min(f.x, FARM_BLOCK.x - f.x);
  if (halves) end = min(end, abs(f.x - FARM_BLOCK.x * 0.5));
  col = mix(col, vec3(0.23, 0.205, 0.165), farmLine(end, FARM_ROAD * 0.5, foot, FARM_BLOCK.x * (halves ? 0.5 : 1.0)));
  // A block's long sides: the farm road, then the shelterbelt on the line.
  float edge = min(f.y, FARM_BLOCK.y - f.y);
  col = mix(col, vec3(0.23, 0.205, 0.165), farmLine(abs(edge - FARM_BELT * 0.5 - FARM_ROAD * 0.5), FARM_ROAD * 0.5, foot, FARM_BLOCK.y));
  float crowns = 0.55 + 0.9 * dNoise(q / vec2(4.5, 3.2)) * (1.0 - smoothstep(1.0, 4.0, foot)) + 0.2 * (dNoise(q / 40.0) - 0.5);
  vec3 belt = vec3(0.032, 0.058, 0.026) * mix(1.0, crowns, 1.0 - smoothstep(2.0, 6.0, foot));
  col = mix(col, belt, farmLine(edge, FARM_BELT * 0.5, foot, FARM_BLOCK.y));
  // The belt's shadow on the strip on its away side.
  float shade = farmLine(abs(f.y - FARM_BELT * 0.5 - 4.0), 4.0, foot, FARM_BLOCK.y) * step(FARM_BELT * 0.5, f.y);
  return col * (1.0 - 0.35 * shade);
}
`;class Dl{positions=[];indices=[];quad(l,r,u,i){const f=this.positions.length/3;this.positions.push(...l,...r,...u,...i),this.indices.push(f,f+1,f+2,f,f+2,f+3)}geometry(){const l=new sa;return l.setAttribute("position",new st(this.positions,3)),l.setIndex(this.indices),l.computeVertexNormals(),l}get empty(){return this.positions.length===0}}function Yu(o,l,r,u,i){o.quad(l,r,u,i),o.quad(i,u,r,l)}function H3(o,l){const r=new ka,u=[],i=l.worldSizeKm*.5,f=new e2({color:2828840});for(const h of o){const v=[];h.pathKm.forEach(([b,S],C)=>{const x=(b-i)*1e3*l.metresToScene,D=(S-i)*1e3*l.metresToScene;v.push(x,l.terrainHeightAt(x,D)+(Math.max(h.fillM[C],0)+6)*l.metresToScene,D)});const m=new sa;m.setAttribute("position",new st(v,3)),u.push(m);const p=new t2(m,f);p.frustumCulled=!1,r.add(p)}return{group:r,geometries:u,materials:[f]}}function L3(o,l){const r=new ka,u=[],i=[],f=l.metresToScene,h=l.worldSizeKm*.5,v=l.layer(8223346,-3),m=l.layer(7038033,-2,{side:ct}),p=l.layer(11118238,-3,{side:ct}),b=l.layer(4934992,-5,{metalness:.6,roughness:.4}),S=l.layer(12170926,-4),C=l.layer(14014682,-4,{side:ct}),x=l.layer(13618372,-3,{side:ct});i.push(v,m,p,b,S,C,x);for(const D of o){const H=D.pathKm.length;if(H<2)continue;const q=D.pathKm.map(([le,G],j)=>{const me=(le-h)*1e3*f,X=(G-h)*1e3*f,we=l.terrainHeightAt(me,X),E=Math.max(D.designM[j]-D.groundM[j],0);return{x:me,z:X,y0:we,fill:E,structure:D.structure[j],s:D.sM[j]}}),P=q.map((le,G)=>{const j=q[Math.max(0,G-1)],me=q[Math.min(H-1,G+1)],X=Math.max(1e-12,Math.hypot(me.x-j.x,me.z-j.z));return{tx:(me.x-j.x)/X,tz:(me.z-j.z)/X}}),K=(le,G,j)=>{const me=q[le],X=P[le];return[me.x+-X.tz*G*f,me.y0+(me.fill+j)*f,me.z+X.tx*G*f]},U=new Dl,z=new Dl,R=new Dl,V=new Dl,J=new Dl,Q=new Dl,ne=new Dl,B=D.formationWidthM*.5,ae=D.trackSpacingM*.5;for(let le=0;le+1<H;le+=1){const G=le+1,j=q[le].structure>=3&&q[le].structure<=4,me=j?.5:.3;U.quad(K(le,-B,me),K(le,B,me),K(G,B,me),K(G,-B,me));for(const X of[-1,1]){if(!j&&q[le].fill>.4){const we=q[le].fill*1.5+.6,E=q[G].fill*1.5+.6,Z=(T,F)=>[K(T,X*(B+F),0)[0],q[T].y0-.2*f,K(T,X*(B+F),0)[2]];z.quad(K(le,X*B,me),Z(le,we),Z(G,E),K(G,X*B,me))}if(j){const we=E=>[K(E,X*B,0)[0],q[E].y0,K(E,X*B,0)[2]];R.quad(K(le,X*B,me),K(G,X*B,me),we(G),we(le))}for(const we of[-1,1]){const E=ae*X+we*.75;V.quad(K(le,E-.06,me+.2),K(le,E+.06,me+.2),K(G,E+.06,me+.2),K(G,E-.06,me+.2))}}}const I=new Dl;let ge=Math.ceil(q[0].s/32.6)*32.6;for(let le=0;le+1<H;le+=1)for(;ge<=q[le+1].s;){if(q[le].structure>=3&&q[le].structure<=4&&q[le].fill>3){const G=(ge-q[le].s)/Math.max(1e-6,q[le+1].s-q[le].s),j=q[le].x+(q[le+1].x-q[le].x)*G,me=q[le].z+(q[le+1].z-q[le].z)*G,X=l.terrainHeightAt(j,me),we=X+(q[le].fill-.5)*f,E=P[le].tx,Z=P[le].tz,T=(F,ce,ue)=>[j+E*F*f-Z*ce*f,ue,me+Z*F*f+E*ce*f];for(const[F,ce,ue,Ee]of[[-1.3,-3,1.3,-3],[1.3,-3,1.3,3],[1.3,3,-1.3,3],[-1.3,3,-1.3,-3]])I.quad(T(F,ce,X),T(ue,Ee,X),T(ue,Ee,we),T(F,ce,we))}ge+=32.6}for(const le of D.stations){const G=le.plan.sM,j=le.plan.platformLengthM;let me=q.findIndex(Ye=>Ye.s>=G-j/2),X=q.findIndex(Ye=>Ye.s>=G+j/2);if(me<0||X<0)continue;me=Math.max(0,me),X=Math.max(me+1,X);const we=le.plan.tracks>=4,E=we?[[-16.25,-4.25],[4.25,16.25]]:[[-(ae+9.75),-(ae+1.75)],[ae+1.75,ae+9.75]];for(let Ye=me;Ye<X;Ye+=1)for(const[Xe,tt]of E)J.quad(K(Ye,Xe,1.35),K(Ye,tt,1.35),K(Ye+1,tt,1.35),K(Ye+1,Xe,1.35)),Q.quad(K(Ye,Xe-1,6.5),K(Ye,tt+1,6.5),K(Ye+1,tt+1,6.5),K(Ye+1,Xe-1,6.5));const Z=Math.floor((me+X)/2),T=le.plan.buildingSide>=0?1:-1,F=(we?38:ae+12+20)*T,ce=we?150:60,ue=we?38:20,Ee=we?17:8.5,Te=K(Z,F,0),Ue=P[Z],oe=(Ye,Xe,tt)=>[Te[0]+Ue.tx*Ye*f-Ue.tz*Xe*f,tt,Te[2]+Ue.tz*Ye*f+Ue.tx*Xe*f],_e=q[Z].y0,Ze=[[-ce/2,-ue/2],[ce/2,-ue/2],[ce/2,ue/2],[-ce/2,ue/2]];for(let Ye=0;Ye<4;Ye+=1){const[Xe,tt]=Ze[Ye],[ot,Ut]=Ze[(Ye+1)%4];ne.quad(oe(Xe,tt,_e),oe(ot,Ut,_e),oe(ot,Ut,_e+Ee*f),oe(Xe,tt,_e+Ee*f))}ne.quad(oe(-ce/2-1,-ue/2-1,_e+Ee*f),oe(ce/2+1,-ue/2-1,_e+Ee*f),oe(ce/2+1,ue/2+1,_e+Ee*f),oe(-ce/2-1,ue/2+1,_e+Ee*f))}const Re=(le,G,j)=>{if(le.empty)return;const me=le.geometry();u.push(me);const X=new xt(me,G);X.renderOrder=j,X.frustumCulled=!1,r.add(X)};Re(z,m,2),Re(U,v,3),Re(R,p,3),Re(I,p,3),Re(V,b,5),Re(J,S,4),Re(Q,C,5),Re(ne,x,4)}return{group:r,geometries:u,materials:i}}const j3={0:[0,1,2,4,5,6],1:[2,5],2:[0,2,3,4,6],3:[0,2,3,5,6],4:[1,2,3,5],5:[0,1,3,5,6],6:[0,1,3,4,5,6],7:[0,2,5],8:[0,1,2,3,4,5,6],9:[0,1,2,3,5,6]},q3=[[0,6,10.4,12],[0,1.6,6,12],[4.4,6,6,12],[0,6,5.2,6.8],[0,1.6,0,6],[4.4,6,0,6],[0,6,0,1.6]];function G3(o,l){const r=new ka,u=[],i=[],f=l.metresToScene,h=l.worldSizeKm*.5,[v,m]=o.axis,[p,b]=o.normal,S=(F,ce)=>{const ue=o.centre.x_km+(F*v+ce*p)/1e3,Ee=o.centre.y_km+(F*m+ce*b)/1e3;return[(ue-h)*1e3*f,(Ee-h)*1e3*f]},[C,x]=S(0,0),D=l.terrainHeightAt(C,x),H=new Map,q=(F,ce,ue,Ee,Te)=>{let Ue=H.get(F);if(!Ue){const oe=l.layer(ce,ue,{side:ct,...Te});i.push(oe),Ue={batch:new Dl,material:oe,order:Ee},H.set(F,Ue)}return Ue.batch},P=(F,ce,ue,Ee,Te,Ue)=>{const oe=(_e,Ze)=>{const[Ye,Xe]=S(_e,Ze);return[Ye,D+Ue*f,Xe]};Yu(F,oe(ce,Ee),oe(ue,Ee),oe(ue,Te),oe(ce,Te))},K=(F,ce,ue,Ee,Te)=>{const Ue=ue[0]-ce[0],oe=ue[1]-ce[1],_e=Math.max(1e-6,Math.hypot(Ue,oe)),Ze=-oe/_e*Ee*.5,Ye=Ue/_e*Ee*.5,Xe=(tt,ot)=>{const[Ut,fa]=S(tt,ot);return[Ut,D+Te*f,fa]};Yu(F,Xe(ce[0]-Ze,ce[1]-Ye),Xe(ce[0]+Ze,ce[1]+Ye),Xe(ue[0]+Ze,ue[1]+Ye),Xe(ue[0]-Ze,ue[1]-Ye))},U=(F,ce,ue,Ee,Te,Ue,oe=0)=>{const _e=(Ye,Xe,tt)=>{const[ot,Ut]=S(Ye,Xe);return[ot,D+(oe+tt)*f,Ut]},Ze=[[ce,Ee],[ue,Ee],[ue,Te],[ce,Te]];for(let Ye=0;Ye<4;Ye+=1){const[Xe,tt]=Ze[Ye],[ot,Ut]=Ze[(Ye+1)%4];Yu(F,_e(Xe,tt,0),_e(ot,Ut,0),_e(ot,Ut,Ue),_e(Xe,tt,Ue))}Yu(F,_e(ce,Ee,Ue),_e(ue,Ee,Ue),_e(ue,Te,Ue),_e(ce,Te,Ue))},z=o.runwayLengthM*.5,R=o.runwayWidthM,V=q("shoulder",5000528,-2,2);P(V,-z,z,-o.pavedWidthM/2,o.pavedWidthM/2,.05);const J=q("asphalt",3618875,-3,3);P(J,-z,z,-R/2,R/2,.07);const Q=q("paint",15658730,-6,6,{emissive:2236962});P(Q,-z,z,-R/2+.9,-R/2+1.8,.1),P(Q,-z,z,R/2-1.8,R/2-.9,.1);for(let F=-z+12;F+30<z-12;F+=50)P(Q,F,F+30,-.45,.45,.1);const ne=o.code.startsWith("3")?8:12;for(const F of[-1,1]){const ce=-F,ue=oe=>F*z+ce*oe;for(let oe=0;oe<ne;oe+=1){const _e=-R/2+1.8+(R-3.6)/ne*(oe+.5)-.9,Ze=6;P(Q,Math.min(ue(Ze),ue(Ze+30)),Math.max(ue(Ze),ue(Ze+30)),_e,_e+1.8,.1)}const Ee=400;for(const oe of[-1,1]){const _e=oe*9;P(Q,Math.min(ue(Ee),ue(Ee+45)),Math.max(ue(Ee),ue(Ee+45)),_e-3,_e+3,.1)}for(let oe=0;oe<3;oe+=1){const _e=150*(oe+1)+400;if(!(_e+22.5>z))for(const Ze of[-1,1])for(let Ye=0;Ye<=2-oe;Ye+=1){const Xe=Ze*(6+3.6*Ye);P(Q,Math.min(ue(_e),ue(_e+22.5)),Math.max(ue(_e),ue(_e+22.5)),Xe-.9,Xe+.9,.1)}}const Te=F<0?o.designatorHi:o.designatorLo,Ue=50;[...Te].forEach((oe,_e)=>{for(const Ze of j3[oe]??[]){const[Ye,Xe,tt,ot]=q3[Ze],Ut=Ue+tt,fa=Ue+ot,Ca=(_e-(Te.length-1)/2)*9-3+Ye,ta=(_e-(Te.length-1)/2)*9-3+Xe;P(Q,Math.min(ue(Ut),ue(fa)),Math.max(ue(Ut),ue(fa)),Ca,ta,.1)}})}const B=q("taxi",4539978,-3,3),ae=q("yellow",14202674,-6,6),I=o.taxiwayZ,ge=o.taxiwayWidthM;P(B,-z+60,z-60,I-ge/2,I+ge/2,.07),P(ae,-z+60,z-60,I-.1,I+.1,.1);for(const F of o.links)K(B,F.from,F.to,ge,.07),K(ae,F.from,F.to,.2,.1);const Re=q("apron",10197395,-3,3);P(Re,o.apron.x0,o.apron.x1,o.apron.z0,o.apron.z1,.07);for(const[F,ce]of o.stands)K(ae,[F,o.apron.z0+6],[F,ce-10],.25,.1);const le=q("carpark",6184802,-3,3);P(le,o.carPark.x0,o.carPark.x1,o.carPark.z0,o.carPark.z1,.07);const G=q("white",14605526,-1,4),j=q("glass",5205894,-1,4,{metalness:.4,roughness:.2}),me=q("grey",9409689,-1,4),X=o.terminal;U(G,X.x0,X.x1,X.z0,X.z1,X.heightM,0),U(j,X.x0-.2,X.x1+.2,X.z0-.2,X.z1+.2,X.heightM*.45,X.heightM*.25);for(const[F]of o.stands)U(me,F-1.5,F+1.5,o.apron.z1-38,X.z0,.3,4.5);const we=o.tower;U(G,we.x0+4,we.x1-4,we.z0+4,we.z1-4,we.heightM-7,0),U(j,we.x0-3,we.x1+3,we.z0-3,we.z1+3,6,we.heightM-7),U(me,we.x0-3.5,we.x1+3.5,we.z0-3.5,we.z1+3.5,.6,we.heightM-1);for(const F of o.hangars)U(me,F.x0,F.x1,F.z0,F.z1,F.heightM,0);o.stands.forEach(([F,ce],ue)=>{ue%2!==1&&(U(G,F-2,F+2,ce-18,ce+18,4,1.2),U(G,F-17.9,F+17.9,ce-4,ce-1,.4,1.8),U(G,F-.2,F+.2,ce+13,ce+18,5,1.2+2.5))});const E=q("fence",3159098,-6,6),Z=o.fence;for(const[F,ce,ue,Ee]of[[Z.x0,Z.x1,Z.z0,Z.z0],[Z.x0,Z.x1,Z.z1,Z.z1],[Z.x0,Z.x0,Z.z0,Z.z1],[Z.x1,Z.x1,Z.z0,Z.z1]])K(E,[F,ue],[ce,Ee],.5,.12);const T=q("lights",16774080,-6,6,{emissive:16774080,emissiveIntensity:.9});for(const F of[-1,1]){for(let ue=60;ue<=900;ue+=30){const Ee=F*(z+ue);P(T,Ee-.4,Ee+.4,-2.5,2.5,.5)}const ce=F*(z+300);P(T,ce-.4,ce+.4,-15,15,.5),P(T,F*(z+150)-.4,F*(z+150)+.4,-11,11,.5)}for(const{batch:F,material:ce,order:ue}of H.values()){const Ee=F.geometry();u.push(Ee);const Te=new xt(Ee,ce);Te.renderOrder=ue,Te.frustumCulled=!1,r.add(Te)}return{group:r,geometries:u,materials:i}}function q1(o){const l=atob(o),r=new Uint8Array(l.length);for(let u=0;u<l.length;u+=1)r[u]=l.charCodeAt(u);return new Float32Array(r.buffer)}function El(o){const l=atob(o),r=new Uint8Array(l.length);for(let u=0;u<l.length;u+=1)r[u]=l.charCodeAt(u);return r}const G1=`
  varying vec2 vUv;
  void main() {
    vUv = uv;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`,F1=`
  precision highp float;
  varying vec2 vUv;
  uniform float uTime;
  uniform float uCoverage;
  uniform vec2 uWind;
  uniform vec3 uSunColor;
  uniform float uShadowMode;

  float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
  }
  float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.55;
    mat2 rotation = mat2(0.82, -0.57, 0.57, 0.82);
    for (int i = 0; i < 6; i++) {
      value += noise(p) * amplitude;
      p = rotation * p * 2.03 + 9.17;
      amplitude *= 0.5;
    }
    return value;
  }
  void main() {
    vec2 drift = uWind * uTime;
    vec2 p = vUv * 4.2 + drift;
    float broad = fbm(p * 0.72);
    float detail = fbm(p * 2.1 + broad * 1.7);
    float field = broad * 0.72 + detail * 0.28;
    float threshold = mix(0.91, 0.43, uCoverage);
    float density = smoothstep(threshold, threshold + 0.16, field);
    density *= smoothstep(0.0, 0.10, vUv.x) * smoothstep(0.0, 0.10, vUv.y);
    density *= smoothstep(0.0, 0.10, 1.0 - vUv.x) * smoothstep(0.0, 0.10, 1.0 - vUv.y);
    float silver = smoothstep(threshold + 0.02, threshold + 0.19, fbm(p + vec2(-0.11, 0.08)));
    vec3 shadow = vec3(0.43, 0.49, 0.50);
    vec3 light = mix(vec3(0.88, 0.91, 0.90), uSunColor, 0.22);
    vec3 color = mix(shadow, light, 0.55 + silver * 0.45);
    if (uShadowMode > 0.5) {
      gl_FragColor = vec4(vec3(0.015, 0.025, 0.022), density * 0.34);
    } else {
      gl_FragColor = vec4(color, density * 0.76);
    }
  }
`,vc=new rt(-.55,.52,-.66).normalize(),F3=12899556,V3=`
  varying vec3 vDir;
  void main() {
    vDir = position;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`,K3=`
  varying vec3 vDir;
  uniform vec3 uSunDir;
  uniform float uTime;

  float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
  float vnoise(vec2 p) {
    vec2 i = floor(p), f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float v = 0.0, a = 0.5;
    for (int i = 0; i < 6; i++) { v += a * vnoise(p); p = p * 2.07 + vec2(19.7, 7.3); a *= 0.5; }
    return v;
  }

  void main() {
    vec3 d = normalize(vDir);
    float y = d.y;
    vec3 zenith = vec3(0.070, 0.225, 0.565);
    vec3 horizon = vec3(0.46, 0.60, 0.76);
    vec3 col = mix(horizon, zenith, pow(clamp(y, 0.0, 1.0), 0.52));

    float s = max(dot(d, uSunDir), 0.0);
    col += vec3(1.0, 0.86, 0.62) * pow(s, 6.0) * 0.14;
    col += vec3(1.0, 0.94, 0.82) * pow(s, 220.0) * 2.6;

    if (y > 0.012) {
      vec2 uv = d.xz / (y + 0.14) * 4.5;
      vec2 drift = vec2(uTime * 0.0022, uTime * 0.0009);
      float warp = fbm(uv * 0.18 + drift);
      float c = fbm(uv * 0.34 + (warp - 0.5) * 2.4 + drift * 1.7);
      float cover = smoothstep(0.50, 0.76, c + 0.14 * pow(1.0 - y, 2.0));
      float fade = smoothstep(0.012, 0.10, y);
      float lit = smoothstep(0.42, 0.92, c);
      vec3 cloud = mix(vec3(0.56, 0.60, 0.68), vec3(0.98, 0.99, 1.03), lit);
      cloud += vec3(0.30, 0.24, 0.14) * pow(s, 3.0);
      col = mix(col, cloud, cover * fade * 0.94);
    }

    col = mix(col, horizon * vec3(1.04, 1.02, 0.99), smoothstep(0.045, -0.10, y));

    gl_FragColor = vec4(col, 1.0);
    #include <tonemapping_fragment>
    #include <colorspace_fragment>
  }
`,Y3=`
  precision highp float;
  varying vec2 vUv;
  varying vec3 vWorldPosition;
  uniform float uTime;
  uniform float uMetresToScene;
  uniform vec2 uWind;
  uniform sampler2D uHydrologyMap;
  uniform sampler2D uFlowMap;

  float vertexHash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
  }
  float vertexNoise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(vertexHash(i), vertexHash(i + vec2(1.0, 0.0)), f.x),
               mix(vertexHash(i + vec2(0.0, 1.0)), vertexHash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float vertexFbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.58;
    mat2 rotation = mat2(0.81, -0.59, 0.59, 0.81);
    for (int i = 0; i < 3; i++) {
      value += vertexNoise(p) * amplitude;
      p = rotation * p * 2.03 + vec2(5.7, -3.2);
      amplitude *= 0.47;
    }
    return value;
  }

  void main() {
    vUv = uv;
    vec4 hydrology = texture2D(uHydrologyMap, uv);
    float water = hydrology.r;
    float waterKind = hydrology.g;
    float flowStrength = hydrology.b;
    vec2 flowSample = texture2D(uFlowMap, uv).rg * 2.0 - 1.0;
    vec3 displaced = position;
    float ocean = smoothstep(0.72, 0.96, waterKind);
    float lake = smoothstep(0.26, 0.48, waterKind) * (1.0 - smoothstep(0.62, 0.82, waterKind));
    float river = 1.0 - smoothstep(0.18, 0.38, waterKind);
    vec2 wind = normalize(uWind + vec2(0.0001));
    vec2 flow = normalize(flowSample + vec2(0.0001));
    float oceanWave = (vertexFbm(displaced.xy * 6.5 + wind * uTime * 0.32) - 0.50) * 7.5;
    float lakeWave = (vertexFbm(displaced.xy * 13.0 + wind * uTime * 0.20 + vec2(9.0, 4.0)) - 0.50) * 0.42;
    vec2 channel = vec2(dot(displaced.xy, flow), dot(displaced.xy, vec2(-flow.y, flow.x)));
    float riverWave = (vertexFbm(channel * vec2(18.0, 38.0) + vec2(-uTime * (0.8 + flowStrength), 0.0)) - 0.50) * 0.16;
    displaced.z += water * uMetresToScene * (ocean * oceanWave + lake * lakeWave + river * riverWave);
    vec4 world = modelMatrix * vec4(displaced, 1.0);
    vWorldPosition = world.xyz;
    gl_Position = projectionMatrix * viewMatrix * world;
  }
`,W3=`
  precision highp float;
  varying vec2 vUv;
  varying vec3 vWorldPosition;
  uniform float uTime;
  uniform vec2 uWind;
  uniform vec3 uSunDirection;
  uniform sampler2D uHydrologyMap;
  uniform sampler2D uFlowMap;

  float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
  }
  float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.54;
    mat2 rotation = mat2(0.80, -0.60, 0.60, 0.80);
    for (int i = 0; i < 4; i++) {
      value += noise(p) * amplitude;
      p = rotation * p * 2.07 + vec2(7.3, -4.1);
      amplitude *= 0.48;
    }
    return value;
  }
  float warpedSurface(vec2 p) {
    vec2 warp = vec2(fbm(p * 0.37 + vec2(11.2, -3.7)), fbm(p * 0.37 + vec2(-8.4, 6.1)));
    vec2 domain = p + (warp - 0.5) * 2.4;
    return fbm(domain) * 0.72 + fbm(domain * 2.73 + vec2(19.0, -13.0)) * 0.28;
  }
  float waterSurface(vec2 uv, float kind, vec2 wind, vec2 flow, float strength) {
    if (kind > 0.72) {
      return warpedSurface(uv * 22.0 + wind * uTime * 0.085);
    }
    if (kind > 0.24) {
      return warpedSurface(uv * 31.0 + wind * uTime * 0.045 + vec2(17.0, 9.0));
    }
    vec2 across = vec2(-flow.y, flow.x);
    vec2 channel = vec2(dot(uv, flow), dot(uv, across));
    return warpedSurface(channel * vec2(48.0, 96.0) + vec2(-uTime * (0.22 + strength * 0.34), 0.0));
  }

  void main() {
    vec4 hydrology = texture2D(uHydrologyMap, vUv);
    float water = hydrology.r;
    float waterKind = hydrology.g;
    float flowStrength = hydrology.b;
    vec2 flowSample = texture2D(uFlowMap, vUv).rg * 2.0 - 1.0;
    if (water < 0.16) discard;
    float ocean = smoothstep(0.72, 0.96, waterKind);
    float lake = smoothstep(0.26, 0.48, waterKind) * (1.0 - smoothstep(0.62, 0.82, waterKind));
    float river = 1.0 - smoothstep(0.18, 0.38, waterKind);
    vec2 wind = normalize(uWind + vec2(0.0001));
    vec2 flow = normalize(flowSample + vec2(0.0001));
    float footprint = max(length(fwidth(vUv)) * 1.5, 0.0012);
    float centre = waterSurface(vUv, waterKind, wind, flow, flowStrength);
    float sampleX = waterSurface(vUv + vec2(footprint, 0.0), waterKind, wind, flow, flowStrength);
    float sampleY = waterSurface(vUv + vec2(0.0, footprint), waterKind, wind, flow, flowStrength);
    float normalStrength = ocean * 0.00165 + lake * 0.0012 + river * (0.0009 + flowStrength * 0.0011);
    vec2 gradient = vec2(sampleX - centre, sampleY - centre) / footprint * normalStrength;
    vec3 normal = normalize(vec3(-gradient.x, 1.0, -gradient.y));
    vec3 viewDirection = normalize(cameraPosition - vWorldPosition);
    float fresnel = pow(1.0 - max(dot(normal, viewDirection), 0.0), 4.0);
    vec3 reflectedSun = reflect(-normalize(uSunDirection), normal);
    float specular = pow(max(dot(reflectedSun, viewDirection), 0.0), 38.0);
    float glint = pow(max(dot(reflectedSun, viewDirection), 0.0), 120.0);
    vec3 base = vec3(0.018, 0.105, 0.135) * ocean
      + vec3(0.035, 0.145, 0.155) * lake
      + vec3(0.075, 0.175, 0.155) * river;
    base = mix(base, vec3(0.39, 0.53, 0.56), fresnel * 0.56);
    base += vec3(1.0, 0.91, 0.72) * specular * 0.10 + vec3(1.0) * glint * 0.13;
    float edgeFoam = smoothstep(0.16, 0.42, water) * (1.0 - smoothstep(0.42, 0.72, water));
    float streamFoam = river * flowStrength * smoothstep(0.73, 0.92, centre);
    base = mix(base, vec3(0.72, 0.79, 0.75), edgeFoam * ocean * 0.38 + streamFoam * 0.14);
    // A river is opaque enough to read from kilometres off: sky on the surface,
    // not the bed's green through a veil.
    base = mix(base, vec3(0.30, 0.42, 0.46), river * (0.12 + 0.30 * fresnel));
    float alpha = mix(mix(0.52, 0.86, river * smoothstep(0.2, 0.5, water)), 0.82, ocean) * smoothstep(0.12, 0.58, water);
    alpha = max(alpha, river * 0.9 * smoothstep(0.16, 0.42, water));
    gl_FragColor = vec4(base, alpha);
  }
`;function X3({result:o,config:l,cameraMode:r,cityFocus:u,cityScene:i,onNeedCity:f,shot:h,onShotReady:v}){const m=pt.useRef(null),p=pt.useRef(null),b=pt.useRef(i??null);b.current=i??null;const S=pt.useRef(f);S.current=f,pt.useEffect(()=>{p.current?.(i??null)},[i]);const C=pt.useRef(u);C.current=u;const x=pt.useRef(h);x.current=h;const D=pt.useRef(v);D.current=v;const[H,q]=pt.useState(null),P=K=>{const U=new P1,z=.085+l.haze*.006;U.fog=new a2(F3,.01,1);const R=r==="satellite"?new ah(-1.7,1.7,1.7,-1.7,5e-6,30):new Fo(42,1,5e-6,30);r==="satellite"?(R.position.set(0,5.2,.001),R.up.set(0,0,-1)):R.position.set(3.15,1.65,3.15);const V=new n2({antialias:!ia,powerPreference:"high-performance"});V.setPixelRatio(ia?1:Math.min(window.devicePixelRatio,2)),V.outputColorSpace=wo,V.toneMapping=X1,V.shadowMap.enabled=!xs,V.shadowMap.type=o2,V.shadowMap.autoUpdate=!1,V.toneMappingExposure=1.08,K.appendChild(V.domElement);const J=V.capabilities;if(!J.isWebGL2)throw new Error("此浏览器没有 WebGL2，无法绘制 3D 地形。");if(o.meshSize>J.maxTextureSize)throw new Error(`此设备的最大纹理尺寸为 ${J.maxTextureSize}，低于网格 ${o.meshSize}；请选择更小的网格。`);const Q=V.extensions.has("EXT_color_buffer_float")||V.extensions.has("EXT_color_buffer_half_float"),ne=ia||xs;V.domElement.addEventListener("webglcontextlost",g=>{g.preventDefault(),console.error("[webgl] context lost")}),V.domElement.addEventListener("webglcontextrestored",()=>console.warn("[webgl] context restored"));const B=B2(V),ae={uSunDir:{value:vc.clone()},uTime:{value:0}},I=new Wa({vertexShader:V3,fragmentShader:K3,uniforms:ae,side:Z1,depthWrite:!1,depthTest:!1,fog:!1}),ge=new xt(new ws(9e3,48,28),I);ge.frustumCulled=!1,ge.renderOrder=-100,U.add(ge);try{if(!Q&&ne)throw new Error("no half-float render targets");U.environment=E3(V,I)}catch(g){console.warn("[terrain3d] sky environment skipped",g),U.environment=null}U.environmentIntensity=.32,V.toneMappingExposure=1.05;const Re=new a1(13950704,.1);Re.position.set(.5,.4,.6),U.add(Re);let le=null,G=null;const j=[],me=new URLSearchParams(window.location.search).get("ao")==="0";try{if(ia||!Q||new URLSearchParams(window.location.search).get("post")==="0")throw new Error("post disabled for diagnosis");const g=new Ms(2,2,{type:kl,samples:4}),y=new Cy(V,g);y.setPixelRatio(V.getPixelRatio());const M=new Dy(U,R),_=new bo(U,R,K.clientWidth||2,K.clientHeight||2);ky(_),_.output=bo.OUTPUT.Default,_.blendIntensity=.85;const N=3.2/(o.worldSizeKm*1e3);_.updateGtaoMaterial({radius:.8*N,distanceExponent:1.1,thickness:1.4*N,scale:1,samples:16}),_.updatePdMaterial({lumaPhi:10,depthPhi:2,normalPhi:3,radius:4,rings:2,samples:8}),By(_.normalMaterial),xc(_.normalMaterial,"normal");{const A=_,O=A._renderOverride.bind(_);A._renderOverride=(...Y)=>{const ee=j.map(te=>te.visible);for(const te of j)te.visible=!1;O(...Y),j.forEach((te,de)=>{te.visible=ee[de]})}}const k=new cr(new Et(960,540),.14,.5,1.05);y.addPass(M),y.addPass(_),y.addPass(Oy()),y.addPass(k),y.addPass(new Ny),y.setSize(K.clientWidth||2,K.clientHeight||2),le=y,G=_}catch{le=null,G=null}const X=new l2(R,V.domElement);X.enableDamping=!0,X.dampingFactor=.065,X.zoomToCursor=!0,X.screenSpacePanning=!0,X.target.set(0,.055,0),r==="satellite"?(X.enableRotate=!1,X.enablePan=!0,X.minZoom=.75,X.maxZoom=131072):(X.minDistance=8e-5,X.maxDistance=7.5,X.maxPolarAngle=Math.PI*.495);const we=q1(o.heightDataBase64),E=new gc(3.2,3.2,o.meshSize-1,o.meshSize-1),Z=E.attributes.position,T=3.2/(o.worldSizeKm*1e3),F=1/T,ce=1.6,ue=3.2/Math.max(2,o.meshSize-1)*.5,Ee=1.7*T,Te=.35*T;R instanceof Fo&&(X.minDistance=1.2*T,X.maxDistance=Math.max(7.5,o.worldSizeKm*1e3*T*2.4));const Ue=o.citySites??[];for(let g=0;g<we.length;g+=1){const y=Math.max(0,we[g]-o.stats.minElevation);Z.setZ(g,y*T)}Z.needsUpdate=!0,E.computeVertexNormals();const oe=new s2().load(o.previewDataUrl);oe.colorSpace=wo,oe.anisotropy=Math.min(ne?4:16,V.capabilities.getMaxAnisotropy());const _e=new Ke({map:oe,roughness:.96,metalness:0}),Ze=new Uint16Array(we.length);let Ye=1;for(let g=0;g<we.length;g+=1){const y=Math.max(0,we[g]-o.stats.minElevation);Ye=Math.max(Ye,y),Ze[g]=i2.toHalfFloat(y)}const Xe=new ma(Ze,o.meshSize,o.meshSize,n1,kl);Xe.minFilter=ua,Xe.magFilter=ua,Xe.wrapS=Xe.wrapT=fi,Xe.generateMipmaps=!1,Xe.needsUpdate=!0;const tt=o.materialDataBase64?El(o.materialDataBase64):new Uint8Array(o.meshSize*o.meshSize*4),ot=new ma(tt,o.meshSize,o.meshSize,$n,nh);ot.minFilter=ua,ot.magFilter=ua,ot.wrapS=ot.wrapT=fi,ot.generateMipmaps=!1,ot.needsUpdate=!0;const Ut=new ma(El(o.cultivatedDataBase64),o.meshSize,o.meshSize,n1,nh);Ut.minFilter=ua,Ut.magFilter=ua,Ut.wrapS=Ut.wrapT=fi,Ut.generateMipmaps=!1,Ut.needsUpdate=!0;const fa=Ge.degToRad(l.windDirection),Ca={uMaterialTex:{value:ot},uFarmTex:{value:Ut},uWindDir:{value:new Et(Math.cos(fa),Math.sin(fa))},uHeightTex:{value:Xe},uHeightN:{value:o.meshSize},uM2S:{value:T},uMaxHeightScene:{value:Ye*T},uSunWorld:{value:vc.clone()},uSunViewDir:{value:vc.clone()}};_e.onBeforeCompile=g=>{g.uniforms.uMetres={value:F},Object.assign(g.uniforms,Ca),g.vertexShader=g.vertexShader.replace("#include <common>",`#include <common>
varying vec3 vDetailP;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vDetailP = (modelMatrix * vec4(position, 1.0)).xyz;`);const y="RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );",M=th.lights_fragment_begin,_=M.lastIndexOf(y),N=_<0?M:M.slice(0,_)+`directLight.color *= mix(1.0, gSunLit, step(0.98, dot(directionalLight.direction, uSunViewDir)));
		`+M.slice(_);g.fragmentShader=g.fragmentShader.replace("#include <lights_fragment_begin>",N).replace("#include <common>",`#include <common>
varying vec3 vDetailP;
uniform float uMetres;
float dHash(vec2 p) { p = fract(p * vec2(123.34, 456.21)); p += dot(p, p + 45.32); return fract(p.x * p.y); }
float dNoise(vec2 p) {
  vec2 i = floor(p), f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(dHash(i), dHash(i + vec2(1, 0)), f.x), mix(dHash(i + vec2(0, 1)), dHash(i + vec2(1, 1)), f.x), f.y);
}
float dFbm(vec2 p) { return 0.5 * dNoise(p) + 0.3 * dNoise(p * 2.13 + 7.1) + 0.2 * dNoise(p * 4.37 + 3.7); }
uniform sampler2D uFarmTex;
${B3}
// ---- eroded relief -------------------------------------------------------
// The mesh is one sample per ~150 m, so every ridge, gully and spur finer than
// that has to be synthesised. Gradient noise with analytic derivatives, folded
// into ridges, with each octave damped by the slope the previous ones built:
// flat ground between steep flanks stays smooth and the flanks themselves break
// into branching gullies, which is what water does and plain fbm does not.
vec2 eHash(vec2 p) {
  p = vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)));
  return fract(sin(p) * 43758.5453123) * 2.0 - 1.0;
}
vec3 eNoise(vec2 x) {
  vec2 i = floor(x), f = fract(x);
  vec2 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
  vec2 du = 30.0 * f * f * (f * (f - 2.0) + 1.0);
  vec2 ga = eHash(i), gb = eHash(i + vec2(1.0, 0.0)), gc = eHash(i + vec2(0.0, 1.0)), gd = eHash(i + vec2(1.0, 1.0));
  float va = dot(ga, f), vb = dot(gb, f - vec2(1.0, 0.0)), vc = dot(gc, f - vec2(0.0, 1.0)), vd = dot(gd, f - vec2(1.0, 1.0));
  float k = va - vb - vc + vd;
  float v = va + u.x * (vb - va) + u.y * (vc - va) + u.x * u.y * k;
  vec2 d = ga + u.x * (gb - ga) + u.y * (gc - ga) + u.x * u.y * (ga - gb - gc + gd) + du * (u.yx * k + vec2(vb - va, vc - va));
  return vec3(v, d);
}
// x: relief height 0..1 (ridges high, gully floors low), yz: d(height in metres)/d(metre).
float gErodeH;
vec2 gErodeG;
uniform sampler2D uHeightTex;
uniform sampler2D uMaterialTex;
uniform vec2 uWindDir;
float gSnow = 0.0;
uniform float uHeightN;
uniform float uM2S;
uniform float uMaxHeightScene;
uniform vec3 uSunWorld;
uniform vec3 uSunViewDir;
float gSunLit = 1.0;
// Height (scene units) of the terrain at scene x/z, bilinear.
float terrainHeightScene(vec2 xz) {
  vec2 uv = ((xz + 1.6) / 3.2 * (uHeightN - 1.0) + 0.5) / uHeightN;
  return texture2D(uHeightTex, uv).r * uM2S;
}
// Wind-sculpted snow surface in metres: long drifts stretched along the wind,
// and finer ripples (sastrugi) that fade out with the pixel footprint.
float snowSurface(vec2 p, float foot) {
  vec2 w = uWindDir;
  vec2 pw = vec2(-w.y, w.x);
  vec2 q = vec2(dot(p, w) / 300.0, dot(p, pw) / 75.0);
  float drifts = dFbm(q) * 9.0 + dFbm(q * 3.3 + 5.0) * 2.4;
  float ripples = dNoise(vec2(dot(p, w) / 14.0, dot(p, pw) / 3.2) + 9.0) * 0.28 * (1.0 - smoothstep(0.8, 5.0, foot));
  return drifts + ripples;
}
// Soft cast shadow: march toward the sun and track how close the ray comes to
// the ground relative to how far it has travelled (a penumbra proportional to
// distance, as a real sun disc gives).
float sunVisibility(vec3 P) {
  float lit = 1.0;
  float stepM = 30.0;
  float t = 60.0;
  for (int i = 0; i < 36; i++) {
    vec3 q = P + uSunWorld * (t * uM2S);
    if (q.y > uMaxHeightScene || abs(q.x) > 1.6 || abs(q.z) > 1.6) break;
    // The mesh interpolates triangles while the texture interpolates bilinearly;
    // on a sharp crest they differ by tens of metres, so give the ray that much
    // slack or the ridge line shadows itself in dots.
    float clearance = q.y - terrainHeightScene(q.xz) + 14.0 * uM2S;
    lit = min(lit, clamp(clearance / (0.10 * t * uM2S) + 0.25, 0.0, 1.0));
    if (lit <= 0.0) break;
    t += stepM;
    stepM *= 1.13;
  }
  return lit * lit * (3.0 - 2.0 * lit);
}
void erodedRelief(vec2 p, float footprint, float amplitude) {
  float a = 0.0, b = 1.0, norm = 0.0;
  vec2 d = vec2(0.0), g = vec2(0.0);
  float f = 1.0 / 2400.0;
  mat2 m = mat2(0.8, -0.6, 0.6, 0.8);
  mat2 R = mat2(1.0, 0.0, 0.0, 1.0);
  vec2 q = p * f;
  for (int i = 0; i < 9; i++) {
    float lambda = 1.0 / f;
    float w = 1.0 - smoothstep(lambda * 0.08, lambda * 0.30, footprint);
    vec3 n = eNoise(q);
    float r = 1.0 - abs(n.x);
    vec2 dr = -sign(n.x) * n.yz;
    d += dr * 0.5;
    float damp = 1.0 / (1.0 + dot(d, d));
    a += b * w * r * damp;
    g += (transpose(R) * dr) * (b * w * f * damp);
    norm += b * damp;
    b *= 0.46;
    q = m * q * 2.0 + 17.3;
    R = m * R;
    f *= 2.0;
  }
  gErodeH = clamp(a / max(norm, 1e-4), 0.0, 1.0);
  gErodeG = g * amplitude;
}`).replace("#include <map_fragment>",`#include <map_fragment>
{
  vec2 pm = vDetailP.xz * uMetres;
  float dm = length(vDetailP - cameraPosition) * uMetres;
  float w0 = 1.0 - smoothstep(6.0, 60.0, dm);
  float w1 = 1.0 - smoothstep(40.0, 320.0, dm);
  float w2 = 1.0 - smoothstep(250.0, 1800.0, dm);
  float w3 = 1.0 - smoothstep(1500.0, 12000.0, dm);
  float grain = dFbm(pm * 9.0) - 0.5;
  float tufts = dFbm(pm * 1.9) - 0.5;
  float blotch = dFbm(pm * 0.21) - 0.5;
  float region = dFbm(pm * 0.021) - 0.5;
  float tone = 1.0 + w0 * grain * 0.7 + w1 * tufts * 0.55 + w2 * blotch * 0.45 + w3 * region * 0.3;
  diffuseColor.rgb *= tone;
  // Warm the dry patches and cool the lush ones a little, as real fields do.
  diffuseColor.rgb += (w1 * tufts + w2 * blotch) * vec3(0.045, 0.03, -0.02);
  float blades = dNoise(vec2(pm.x * 38.0 + pm.y * 11.0, pm.y * 70.0)) * dNoise(pm * 23.0 + 9.0);
  diffuseColor.rgb *= 1.0 + w0 * (blades - 0.25) * 0.9;
  // Vegetated ground only (roads, water and city surfaces are baked into the
  // same texture and must not get grass grain or soil): greenness gate.
  float veg = smoothstep(-0.01, 0.05, diffuseColor.g - max(diffuseColor.r, diffuseColor.b));
  // Meadow clumps: dark hollows between tufts and pale dry stalks, at a scale that
  // survives out to a few hundred metres (the blade streaks above do not).
  float clump = dNoise(pm * 3.3) * 0.6 + dNoise(pm * 8.9 + 3.0) * 0.4;
  diffuseColor.rgb *= 1.0 + veg * w1 * (clump - 0.5) * 0.55;
  diffuseColor.rgb += veg * w1 * smoothstep(0.62, 0.86, clump) * vec3(0.06, 0.05, -0.01);
  // Bare soil in scuffs and worn patches, dry straw in broad drifts: without
  // them a field is one green from horizon to boots.
  float soilShape = dFbm(pm * 0.17 + 13.0) + 0.25 * dNoise(pm * 1.3);
  float soil = smoothstep(0.735, 0.80, soilShape) * veg * (1.0 - smoothstep(600.0, 3500.0, dm));
  vec3 soilColour = vec3(0.27, 0.19, 0.12) * (0.75 + 0.5 * dNoise(pm * 7.0 + 2.0));
  diffuseColor.rgb = mix(diffuseColor.rgb, soilColour, soil * 0.72);
  float straw = smoothstep(0.52, 0.74, dFbm(pm * 0.0125 + 5.0)) * veg;
  diffuseColor.rgb = mix(diffuseColor.rgb, diffuseColor.rgb * vec3(1.32, 1.14, 0.62) + vec3(0.03, 0.02, 0.0), straw * (1.0 - smoothstep(3000.0, 14000.0, dm)) * 0.55);
  // Dark, wetter hollows where the ground gathers water.
  float wet = smoothstep(0.62, 0.80, dFbm(pm * 0.045 + 31.0)) * veg;
  diffuseColor.rgb *= 1.0 - wet * 0.22 * (1.0 - smoothstep(800.0, 6000.0, dm));
  // Eroded relief. Its strength follows the slope the mesh already has, so flat
  // ground stays flat and steep ground breaks into spurs and gullies. The height
  // and gradient are kept for the normal pass below.
  {
    vec3 wn = normalize(inverseTransformDirection(normalize(vNormal), viewMatrix));
    float slope = 1.0 - clamp(wn.y, 0.0, 1.0);
    float steep = smoothstep(0.06, 0.42, slope);
    erodedRelief(pm, dm * 0.0016, 380.0 * (0.12 + 1.9 * steep));
    float hn = smoothstep(0.18, 0.82, gErodeH);
    // ---- natural ground, built from cover and relief ----------------------
    // Colour is decided per pixel by what the ground is doing, not read from a
    // 78 m image: steep ground is bare rock banded by strata and cut by cracks,
    // scree gathers below it, soil and grass sit on the gentle ground, forest
    // follows the gully floors, and snow lies on benches and in gullies while
    // wind scours the ridges. The baked image supplies only the rock tint (the
    // lithology colour) and everything that is not natural: water, roads, fields.
    vec2 muv = ((vDetailP.xz + 1.6) / 3.2 * (uHeightN - 1.0) + 0.5) / uHeightN;
    vec4 ctrl = texture2D(uMaterialTex, muv);
    if (ctrl.a > 0.004) {
      vec3 nf = normalize(wn + vec3(-gErodeG.x, 0.0, -gErodeG.y));
      float sf = 1.0 - clamp(nf.y, 0.0, 1.0);
      float foot = dm * 0.0016;
      float altM = vDetailP.y * uMetres;
      float bl = max(dot(diffuseColor.rgb, vec3(0.299, 0.587, 0.114)), 0.05);
      vec3 tint = mix(vec3(1.0), diffuseColor.rgb / bl, 0.7);
      // Detail fades with the pixel footprint so far ground averages, not shimmers.
      float a1 = 1.0 - smoothstep(0.4, 1.6, foot);
      float a2 = 1.0 - smoothstep(2.0, 9.0, foot);
      float a3 = 1.0 - smoothstep(10.0, 45.0, foot);
      float warpS = dFbm(pm / 180.0);
      // A band is only drawn while a pixel spans well under its thickness; on a
      // cliff altitude changes fast across a pixel, so judge it by that, not by
      // distance, or the bands alias into stripes.
      float altStep = fwidth(altM);
      float bandA = 1.0 - smoothstep(38.0 * 0.06, 38.0 * 0.28, altStep);
      float bandB = 1.0 - smoothstep(7.5 * 0.06, 7.5 * 0.28, altStep);
      float strata = mix(0.5, 0.5 + 0.5 * sin(altM / 38.0 * 6.2832 + 4.0 * warpS), bandA);
      float strataF = mix(0.5, 0.5 + 0.5 * sin(altM / 7.5 * 6.2832 + 6.0 * dFbm(pm / 40.0)), bandB);
      float cr = abs(dNoise(pm / 7.0) - 0.5);
      float cracks = (1.0 - smoothstep(0.0, 0.05, cr)) * a1;
      float rockLum = 0.115 + 0.075 * strata * (0.4 + 0.6 * a3) + 0.040 * strataF * a2
        + 0.05 * (dFbm(pm / 23.0) - 0.5) * a2 + 0.05 * (dFbm(pm / 310.0) - 0.5);
      rockLum *= (1.0 - 0.45 * cracks) * (0.86 + 0.28 * hn);
      vec3 rock = tint * rockLum;
      vec3 soil = tint * 0.105 * (0.88 + 0.24 * dNoise(pm / 5.0) * a2 + 0.12 * (dFbm(pm / 90.0) - 0.5));
      vec3 scree = mix(rock, soil, 0.45) * (0.92 + 0.22 * dNoise(pm / 1.7) * a1);
      vec3 grass = vec3(0.075, 0.098, 0.034) * (0.72 + 0.55 * dFbm(pm / 36.0)) * (0.9 + 0.2 * dNoise(pm / 2.3) * a1);
      vec3 forest = vec3(0.028, 0.052, 0.024) * (0.62 + 0.75 * dNoise(pm / 9.0) * a2 + 0.3 * dFbm(pm / 260.0));
      // Old snow is grey-white, not paper: packed drifts, wind crust and a little
      // dust, in a cool tint that sky light turns bluer in shade.
      float packed = dFbm(vec2(dot(pm, uWindDir) / 420.0, dot(pm, vec2(-uWindDir.y, uWindDir.x)) / 90.0));
      float streak = dNoise(vec2(dot(pm, uWindDir) / 40.0, dot(pm, vec2(-uWindDir.y, uWindDir.x)) / 5.5)) * a2;
      vec3 snow = vec3(0.60, 0.635, 0.69) * (0.72 + 0.42 * packed + 0.10 * streak);

      // The gully pattern only means anything where the ground is sloped; on a
      // bench or a plateau it would imprint noise blotches on snow and grass.
      float hnm = mix(0.5, hn, smoothstep(0.03, 0.20, sf));
      float vegLimit = 1.0 - smoothstep(0.10, 0.24, sf);
      float rockW = smoothstep(0.15, 0.40, sf);
      float forestW = ctrl.r * vegLimit * mix(0.5, 1.0, 1.0 - hnm);
      float grassW = ctrl.g * vegLimit * mix(0.7, 1.0, 1.0 - hnm);
      float screeW = smoothstep(0.08, 0.15, sf) * (1.0 - smoothstep(0.24, 0.36, sf)) * (1.0 - 0.5 * hn) * (1.0 - ctrl.r);
      // Snow is a layer of varying depth, not a tint: it thins over ridges and
      // steep ground until rock shows through, in ragged islands and tongues.
      float depthNoise = dFbm(pm / 130.0) * 0.8 + dFbm(pm / 17.0) * 0.25 * a2;
      float depth = ctrl.b * (0.55 + depthNoise) + 0.30 * (1.0 - hnm) - 0.30;
      float snowW = smoothstep(0.26, 0.40, depth) * (1.0 - smoothstep(0.20, 0.46, sf));
      vec3 c = soil;
      c = mix(c, grass, grassW * (1.0 - rockW));
      c = mix(c, forest, forestW * (1.0 - rockW));
      c = mix(c, scree, screeW * 0.6);
      c = mix(c, rock, rockW);
      c = mix(c, snow, clamp(snowW, 0.0, 1.0));
      gSnow = clamp(snowW, 0.0, 1.0) * ctrl.a;
      diffuseColor.rgb = mix(diffuseColor.rgb, c, ctrl.a);
    }
    // Farmland: the field grid in place of the baked image's blotches, which
    // at one texel per 40 m are only patches of colour.
    float farm = smoothstep(0.07, 0.30, texture2D(uFarmTex, muv).r) * (1.0 - ctrl.a);
    if (farm > 0.004) {
      float footF = max(length(fwidth(pm)), 0.02);
      vec3 fc = farmColour(pm, footF);
      diffuseColor.rgb = mix(diffuseColor.rgb, fc * (0.94 + 0.12 * hn), farm);
    }
    // Contact shadow in gully floors and on the lee of spurs.
    diffuseColor.rgb *= mix(1.0, 0.55 + 0.65 * hn, 0.20 + 0.40 * steep);
    gSunLit = sunVisibility(vDetailP);
  }
}`).replace("#include <normal_fragment_maps>",`#include <normal_fragment_maps>
{
  vec2 pb = vDetailP.xz * uMetres;
  float db = length(vDetailP - cameraPosition) * uMetres;
  float wb = 1.0 - smoothstep(4.0, 90.0, db);
  if (wb > 0.001) {
    float e = 0.02;
    float h0 = dFbm(pb * 14.0) + 0.6 * dNoise(pb * 45.0);
    float hx = dFbm((pb + vec2(e, 0.0)) * 14.0) + 0.6 * dNoise((pb + vec2(e, 0.0)) * 45.0);
    float hz = dFbm((pb + vec2(0.0, e)) * 14.0) + 0.6 * dNoise((pb + vec2(0.0, e)) * 45.0);
    vec3 g = mat3(viewMatrix) * vec3(-(hx - h0) / e, 0.0, -(hz - h0) / e) * 0.035 * wb;
    normal = normalize(normal + g);
  }
  // Mid- and macro-relief: hummocks a few metres across out to the middle
  // distance, and long swells beyond, tilted in world space like the crumb
  // relief above. Low sun on these is what stops distant ground from reading as
  // a lit flat sheet; both fade out only where the mesh's own relief takes over.
  float wmid = 1.0 - smoothstep(30.0, 1600.0, db);
  float wmac = 1.0 - smoothstep(400.0, 9000.0, db);
  if (wmid > 0.001 || wmac > 0.001) {
    float em = 0.6;
    float m0 = dFbm(pb * 0.31) + 0.5 * dNoise(pb * 0.9 + 4.0);
    float mx = dFbm((pb + vec2(em, 0.0)) * 0.31) + 0.5 * dNoise((pb + vec2(em, 0.0)) * 0.9 + 4.0);
    float mz = dFbm((pb + vec2(0.0, em)) * 0.31) + 0.5 * dNoise((pb + vec2(0.0, em)) * 0.9 + 4.0);
    float eM = 12.0;
    float b0 = dFbm(pb * 0.021 + 17.0);
    float bx = dFbm((pb + vec2(eM, 0.0)) * 0.021 + 17.0);
    float bz = dFbm((pb + vec2(0.0, eM)) * 0.021 + 17.0);
    vec3 gm = mat3(viewMatrix) * vec3(-(mx - m0) / em * 0.20 * wmid - (bx - b0) / eM * 2.6 * wmac, 0.0, -(mz - m0) / em * 0.20 * wmid - (bz - b0) / eM * 2.6 * wmac);
    normal = normalize(normal + gm);
  }
  normal = normalize(normal + mat3(viewMatrix) * vec3(-gErodeG.x, 0.0, -gErodeG.y));
  if (gSnow > 0.01) {
    vec2 ps = vDetailP.xz * uMetres;
    float fs = length(vDetailP - cameraPosition) * uMetres * 0.0016;
    float es = max(1.5, fs * 1.2);
    float s0 = snowSurface(ps, fs);
    float sx = snowSurface(ps + vec2(es, 0.0), fs);
    float sz = snowSurface(ps + vec2(0.0, es), fs);
    normal = normalize(normal + mat3(viewMatrix) * vec3(-(sx - s0) / es, 0.0, -(sz - s0) / es) * gSnow);
  }
}`)};const ta=new xt(E,_e);ta.renderOrder=ip,ta.rotation.x=-Math.PI/2,U.add(ta);const Ha=El(o.forestDataBase64),Xa=El(o.vegetationExclusionDataBase64),an=xs?0:ia?12e3:45e3,qt=Jf(o.farTrees,2,T),to=qt.material,La=[],ao=[],Pa=[],ut=(g,y,M)=>{const _=Math.sin(g*127.1+y*311.7+M*74.7+l.seed*.013)*43758.5453;return _-Math.floor(_)};for(let g=1;g+1<o.meshSize;g+=1)for(let y=1;y+1<o.meshSize;y+=1){const M=g*o.meshSize+y,_=Ha[M]/255;if(Xa[M]>32||_<.38||ut(y,g,1)>_*.62||La.length>=an)continue;const N=(ut(y,g,2)-.5)*.86,k=(ut(y,g,3)-.5)*.86,A=-1.6+(y+N)/(o.meshSize-1)*3.2,O=-1.6+(g+k)/(o.meshSize-1)*3.2,Y=.68+ut(y,g,4)*.72,ee=new Mo;ee.compose(new rt(A,Math.max(0,we[M]-o.stats.minElevation)*T,O),new Pu().setFromAxisAngle(new rt(0,1,0),ut(y,g,5)*Math.PI*2),new rt(.75+ut(y,g,6)*.55,Y,.75+ut(y,g,7)*.55)),La.push(ee);const te=.8+ut(y,g,9)*.36;ao.push(new $t(te*(1+(ut(y,g,8)-.5)*.1),te,te*(1-(ut(y,g,8)-.5)*.1))),Pa.push(Math.floor(ut(y,g,10)*qt.geometries.length)%qt.geometries.length)}const xa=new ka;qt.geometries.forEach((g,y)=>{const M=[];if(Pa.forEach((N,k)=>{N===y&&M.push(k)}),!M.length)return;const _=new Ka(g,to,M.length);M.forEach((N,k)=>{_.setMatrixAt(k,La[N]),_.setColorAt(k,ao[N])}),_.instanceMatrix.needsUpdate=!0,_.instanceColor&&(_.instanceColor.needsUpdate=!0),xa.add(_)}),U.add(xa),j.push(xa);const nn=xs?0:ia?6e3:32e3,on=[Jf(o.farTrees,0,T),Jf(o.farTrees,1,T)],ln=on[1].material;on[0].material.dispose();const wa=on.map(g=>g.geometries.map(y=>{const M=new Ka(y,ln,nn);return M.instanceMatrix.setUsage(Xu),M.count=0,U.add(M),M})),sn=wa.flat();j.push(...sn);const Vt=(g,y,M)=>{const _=Ge.clamp((y+1.6)/3.2*(o.meshSize-1),0,o.meshSize-1),N=Ge.clamp((M+1.6)/3.2*(o.meshSize-1),0,o.meshSize-1),k=Math.floor(_),A=Math.floor(N),O=Math.min(o.meshSize-1,k+1),Y=Math.min(o.meshSize-1,A+1),ee=_-k,te=N-A,de=Ge.lerp(g[A*o.meshSize+k],g[A*o.meshSize+O],ee),pe=Ge.lerp(g[Y*o.meshSize+k],g[Y*o.meshSize+O],ee);return Ge.lerp(de,pe,te)},Qo=El(o.cultivatedDataBase64),Io={[di.Cotton]:3,[di.Maize]:2,[di.Wheat]:1,[di.Stubble]:0,[di.Soil]:0,[di.Alfalfa]:3},Jo=(g,y)=>{if(Vt(Qo,g,y)<.3*255)return 0;const M=j1(g/T,y/T);return M.margin?0:Io[M.crop]},yn=14e3,xn=new sr(1,1,1),Hl=new Ke({color:10987588,roughness:1}),_a=new Ka(xn,Hl,yn);_a.instanceMatrix.setUsage(Xu),_a.count=0,_a.frustumCulled=!1,U.add(_a);const Ll=3e4,So=14e3,Sc=24e3,ur=6e3,zs=new ir(.008*T,.012*T,.82*T,4);zs.translate(0,.41*T,0);const $o=new ir(.018*T,.028*T,.15*T,5);$o.translate(0,.88*T,0);const fr=new ir(.018*T,.028*T,2.05*T,5);fr.translate(0,1.025*T,0);const Cs=new sa;Cs.setAttribute("position",new st([0,.65,0,.34,.98,.035,0,.88,0,0,.9,0,-.31,1.28,-.025,0,1.13,0,0,1.16,0,.035,1.52,.32,0,1.38,0,0,1.38,0,-.02,1.73,-.27,0,1.57,0].map(g=>g*T),3)),Cs.computeVertexNormals();const gi=new ir(.036*T,.052*T,.2*T,8);gi.translate(.065*T,1.2*T,0);const hr=new ws(.012*T,4,3),jl=new sa;jl.setAttribute("position",new st([-.018,0,0,.018,0,0,0,.42,0,0,0,-.018,0,0,.018,0,.42,0].map(g=>g*T),3)),jl.computeVertexNormals();const _s=new Ke({color:12100680,roughness:.96}),yi=new Ke({color:13151572,roughness:.91}),xi=new Ke({color:4486700,roughness:.96}),On=new Ke({color:4027948,roughness:.98,side:ct}),ra=new Ke({color:13018409,roughness:.88}),ba=new Ke({color:15057728,roughness:.82}),To=new Ke({color:6916668,roughness:1,side:ct}),As=new Ka(zs,_s,Ll),dr=new Ka($o,yi,Ll),mr=new Ka(fr,xi,So),wi=new Ka(Cs,On,So),el=new Ka(gi,ra,So),Es=new Ka(hr,ba,ur),no=new Ka(jl,To,Sc),zo=[no,As,dr,mr,wi,el,Es];for(const g of zo)g.instanceMatrix.setUsage(Xu),g.count=0,g.frustumCulled=!1,U.add(g);let tl=Number.POSITIVE_INFINITY,Co=Number.POSITIVE_INFINITY,aa=0,pr=0;const rn=new Mo,wn=new Pu,bn=new rt,ql=new rt,Tc=new rt(0,1,0),zc=(g,y)=>{const M=g/T*y,_=M<520,N=_?Ge.clamp(M*.62,12,230):Ge.clamp(M*.58,100,1150),k=_?N<=24?.28:N<=55?.55:N<=120?1.15:2.1:N<=320?3.2:N<=650?5.5:9,A=Math.hypot(X.target.x-tl,X.target.z-Co)/T,O=Math.abs(N-aa)/Math.max(1,aa);if(A<k*3&&O<.12&&pr===k)return;tl=X.target.x,Co=X.target.z,aa=N,pr=k;const Y=(tl+1.6)/T,ee=(Co+1.6)/T;let te=0,de=0,pe=0,fe=0,ye=0;const Se=Math.floor((Y-N)/k),xe=Math.ceil((Y+N)/k),Ne=Math.floor((ee-N)/k),Oe=Math.ceil((ee+N)/k);for(let ze=Ne;ze<=Oe;ze+=1)for(let Ae=Se;Ae<=xe;Ae+=1){const He=(Ae+.12+ut(Ae,ze,72)*.76)*k,ke=(ze+.12+ut(Ae,ze,73)*.76)*k,Le=He-Y,je=ke-ee;if(Le*Le+je*je>N*N)continue;const De=-1.6+He*T,Be=-1.6+ke*T;if(De<=-1.6||De>=1.6||Be<=-1.6||Be>=1.6)continue;const Ve=Jo(De,Be);if(Ve<1||Ve>3||Vt(Xa,De,Be)>48)continue;const qe=Math.max(0,Vt(we,De,Be)-o.stats.minElevation)*T,Qe=j1(De/T,Be/T).rowAngle;if(wn.setFromAxisAngle(Tc,Qe+(ut(Ae,ze,75)-.5)*.035),!_){if(te>=yn)continue;const nt=k*(2.2+ut(Ae,ze,76)*1.4);bn.set(De,qe+(Ve===1?.34:Ve===2?.72:.18)*T,Be),ql.set(nt*T,(Ve===1?.55:Ve===2?1.25:.3)*T,.24*T),rn.compose(bn,wn,ql),_a.setMatrixAt(te,rn),_a.setColorAt(te,new $t(Ve===1?11116867:Ve===2?5208622:7440444)),te+=1;continue}const it=.86+ut(Ae,ze,77)*.26,$e=(ut(Ae,ze,78)-.5)*.08;if(wn.setFromEuler(new dh($e,Qe+ut(Ae,ze,79)*.12,$e*.6)),bn.set(De,qe,Be),ql.set(1,it,1),rn.compose(bn,wn,ql),Ve===3&&de<Sc)no.setMatrixAt(de,rn),no.setColorAt(de,new $t().setHSL(.22+ut(Ae,ze,80)*.06,.38,.28+ut(Ae,ze,81)*.12)),de+=1;else if(Ve===1&&pe<Ll)As.setMatrixAt(pe,rn),dr.setMatrixAt(pe,rn),pe+=1;else if(Ve===2&&fe<So){if(mr.setMatrixAt(fe,rn),wi.setMatrixAt(fe,rn),el.setMatrixAt(fe,rn),Le*Le+je*je<196)for(let nt=0;nt<12&&ye<ur;nt+=1){const vt=nt%6,Ct=Math.floor(nt/6),Rt=Qe+vt/6*Math.PI*2;bn.set(De+Math.cos(Rt)*.054*T,qe+(1.145+Ct*.07)*T*it,Be+Math.sin(Rt)*.054*T),ql.set(.78,1.18,.78),rn.compose(bn,wn.identity(),ql),Es.setMatrixAt(ye,rn),ye+=1}fe+=1}}_a.count=te,no.count=de,As.count=pe,dr.count=pe,mr.count=fe,wi.count=fe,el.count=fe,Es.count=ye;for(const ze of[_a,...zo])ze.instanceMatrix.needsUpdate=!0,ze.instanceColor&&(ze.instanceColor.needsUpdate=!0)},zt=(g,y)=>Math.max(0,Vt(we,g,y)-o.stats.minElevation)*T,dt=R instanceof Fo?new _3({camera:R,dom:V.domElement,groundHeight:zt,metresToScene:T,halfExtent:ce}):null,Za=dt?(()=>{const g=document.createElement("div");g.style.cssText="position:absolute;z-index:4;top:14px;right:14px;display:flex;gap:1px;border:1px solid rgba(255,255,255,.16);background:rgba(9,13,11,.78);backdrop-filter:blur(14px)";const y="padding:7px 10px;border:0;background:transparent;color:#9da8a3;font-size:11px;cursor:pointer",M=document.createElement("button");M.style.cssText=y;const _=document.createElement("button");return _.style.cssText=y,g.append(M,_),K.appendChild(g),{root:g,roamButton:M,modeButton:_}})():null,Qa=()=>{!dt||!Za||(Za.roamButton.textContent=dt.active?"轨道视角 (R)":"自由漫游 (R)",Za.roamButton.style.background=dt.active?"var(--accent, #d7ff72)":"transparent",Za.roamButton.style.color=dt.active?"#11160f":"#9da8a3",Za.modeButton.style.display=dt.active?"":"none",Za.modeButton.textContent=dt.walking?"步行 → 飞行 (G)":"飞行 → 步行 (G)")};let pa=null;const bi=g=>{!dt||!pa||(g&&!dt.active?(dt.enter(!1),pa.enabled=!1):!g&&dt.active&&(dt.exit(),pa.enabled=!0,pa.target.copy(dt.groundTarget),pa.update()),Qa())},Cc=g=>{const y=g.target;y&&/^(INPUT|SELECT|TEXTAREA)$/.test(y.tagName)||g.ctrlKey||g.metaKey||g.altKey||(g.code==="KeyR"?bi(!dt?.active):g.code==="KeyG"&&dt?.active&&(dt.setWalking(!dt.walking),Qa()))};dt&&Za&&(pa=X,Za.roamButton.onclick=()=>bi(!dt.active),Za.modeButton.onclick=()=>{dt.setWalking(!dt.walking),Qa()},window.addEventListener("keydown",Cc),Qa());const _o=new ka,vr=[],Rs=[],Gl=[],Aa=(g,y,M)=>{const N=new Uint8Array(65536);for(let A=0;A<128;A+=1)for(let O=0;O<128;O+=1){const Y=A%24,ee=O%18,te=Y>5&&Y<18&&ee>4&&ee<14&&(O+A+M)%11!==0,de=Y===0||ee===0,pe=(O*17+A*31+M*13)%17-8,fe=(A*128+O)*4,ye=te?y:g;N[fe]=Ge.clamp(ye[0]+pe+(de?7:0),0,255),N[fe+1]=Ge.clamp(ye[1]+pe+(de?7:0),0,255),N[fe+2]=Ge.clamp(ye[2]+pe+(de?7:0),0,255),N[fe+3]=255}const k=new ma(N,128,128,$n);return k.wrapS=Ba,k.wrapT=Ba,k.magFilter=ua,k.minFilter=Yo,k.generateMipmaps=!0,k.colorSpace=wo,k.repeat.set(2.8,3.8),k.needsUpdate=!0,Gl.push(k),k},Mi=new Map,_c=(o.materialTextures?.length??0)>0,Fl=(g,y,M)=>{if(!o.materialTextures?.length)return null;const _=Mi.get(`${g}|${y[0]}|${y[1]}`);if(_)return _;const N=o.materialTextures.find(Y=>Y.name===g);if(!N)return null;const k=atob(N.data),A=new Uint8Array(k.length);for(let Y=0;Y<k.length;Y+=1)A[Y]=k.charCodeAt(Y);const O=new ma(A,N.width,N.height,$n);return O.wrapS=Ba,O.wrapT=Ba,O.magFilter=ua,O.minFilter=Yo,O.generateMipmaps=!0,O.colorSpace=wo,O.repeat.set(y[0],y[1]),O.needsUpdate=!0,Mi.set(`${g}|${y[0]}|${y[1]}`,O),M.push(O),O};Fl("facade/residential",[1/22,1/30],Gl)??Aa([144,151,150],[36,66,77],3),Fl("facade/glass",[1/43,1/25],Gl)??Aa([96,122,130],[18,47,59],7),Fl("facade/stone",[1/7,1/6],Gl)??Aa([177,166,144],[48,66,69],11),Fl("facade/residential",[1/24,1/32],Gl)??Aa([146,108,91],[42,58,63],17);const oo=new Map,Ac=g=>{const y=`v${g%Si.length}`;let M=oo.get(y);return M||(M=[],oo.set(y,M)),y},at=1e3*T,Ie=o.worldSizeKm*.5,s0={generateTopUV(g,y,M,_,N,k){const A=1/T;return[new Et(y[M*3]*A,y[M*3+2]*A),new Et(y[_*3]*A,y[_*3+2]*A),new Et(y[N*3]*A,y[N*3+2]*A),new Et(y[k*3]*A,y[k*3+2]*A)]},generateSideWallUV(g,y,M,_,N,k){const A=1/T,O=y[M*3],Y=y[M*3+1],ee=y[M*3+2],te=y[_*3],de=y[_*3+1],pe=y[_*3+2],fe=y[N*3],ye=y[N*3+1],Se=y[N*3+2],xe=y[k*3],Ne=y[k*3+1],Oe=y[k*3+2];return Math.abs(Y-de)<1e-9?[new Et(O*A,ee*A),new Et(te*A,pe*A),new Et(fe*A,Se*A),new Et(xe*A,Oe*A)]:[new Et(O*A,Y*A),new Et(te*A,de*A),new Et(fe*A,ye*A),new Et(xe*A,Ne*A)]}},Si=[{base:[198,188,168],window:[96,104,112],cols:2,sill:.34},{base:[176,179,176],window:[88,97,106],cols:2,sill:.32},{base:[216,211,197],window:[112,142,154],cols:3,sill:.28},{base:[156,100,80],window:[64,56,54],cols:2,sill:.36,brick:!0},{base:[208,196,168],window:[98,118,128],cols:3,sill:.3},{base:[150,151,149],window:[80,92,100],cols:4,sill:.24,band:!0},{base:[190,148,120],window:[84,90,96],cols:2,sill:.32},{base:[132,136,142],window:[98,130,142],cols:3,sill:.3},{base:[76,90,100],window:[134,170,186],cols:4,glass:!0},{base:[66,80,88],window:[120,160,152],cols:5,glass:!0},{base:[90,94,106],window:[150,152,160],cols:4,glass:!0},{base:[112,98,90],window:[128,150,158],cols:3,glass:!0},{base:[58,62,70],window:[110,150,168],cols:6,glass:!0},{base:[168,170,172],window:[96,118,130],cols:5,glass:!0},{base:[122,96,72],window:[150,138,116],cols:4,glass:!0},{base:[70,92,96],window:[128,164,170],cols:5,glass:!0},{base:[214,212,206],window:[104,116,126],cols:4,sill:.26},{base:[142,138,130],window:[88,96,104],cols:4,sill:.26,band:!0},{base:[186,172,148],window:[96,110,118],cols:3,sill:.3},{base:[84,86,90],window:[118,128,136],cols:5,sill:.22},{base:[226,222,212],window:[100,110,118],cols:2,sill:.4},{base:[164,132,96],window:[76,64,54],cols:2,sill:.36},{base:[142,74,60],window:[70,58,52],cols:2,sill:.36,brick:!0},{base:[172,178,164],window:[92,104,110],cols:3,sill:.34}],gr=new Map,Ns=[],i0=g=>{const y=g%Si.length,M=gr.get(y);if(M)return M;const _=Si[y],N=128,k=new Uint8Array(N*N*4);let A=y*131+17;const O=()=>(A=A*1664525+1013904223>>>0,A/4294967296),Y=(pe,fe)=>{const ye=Math.sin(pe*127.1+fe*311.7+y*74.7)*43758.5453;return ye-Math.floor(ye)},ee=pe=>Y(pe*7.3,y*3.1);for(let pe=0;pe<N;pe++)for(let fe=0;fe<N;fe++){const ye=pe/N,Se=fe/N,xe=Math.min(3,Math.floor(ye*4)),Ne=ye*4-xe,Oe=Math.floor(Se*_.cols),ze=Se*_.cols-Oe,Ae=ee(xe),He=.8+(Ae-.5)*.14,ke=_.glass?.1:Math.max(.26,.32+(_.sill??.3)*.4-(Ae-.5)*.18),Le=_.glass&&ze>.92;let je;if(Oe<_.cols&&ze<(_.glass?.92:.6)&&Ne>ke&&Ne<He&&!Le){const Be=Y(Oe+y*31,xe*17.3+Math.floor(y*5.7)),Ve=Math.floor(Ne*8)%4===0?12:0;je=_.window.map(qe=>qe*(.5+Be*.85)+Ve+(O()-.5)*8)}else Ne>.9||Ne<.08?je=_.base.map(Be=>Be*.55+8):Le?je=_.base.map(Be=>Be*.45):(je=_.base.map(Be=>Be+(O()-.5)*10),_.brick&&pe%7<2&&(je=je.map(Be=>Be+16)));const De=(pe*N+fe)*4;for(let Be=0;Be<3;Be++)k[De+Be]=Ge.clamp(je[Be],0,255);k[De+3]=255}const te=new ma(k,N,N);te.wrapS=te.wrapT=Ba,te.magFilter=ua,te.minFilter=Yo,te.generateMipmaps=!0,te.colorSpace=wo,te.needsUpdate=!0,te.repeat.set(1/3,1/12.8);const de=_.glass?new Ke({map:te,roughness:.34,metalness:.45,vertexColors:!0,side:ct,envMapIntensity:.85}):new Ke({map:te,roughness:.82,vertexColors:!0,side:ct,envMapIntensity:.55});return gr.set(y,de),Ns.push(de),de},yr=[.4,.41,.4],lo=ia?[]:o.modernCities?.length?o.modernCities.map(g=>({style:"modernChinese",streets:[],blocks:g.blocks,buildings:g.buildings.map(y=>({id:y.id,parcelId:y.parcelId,footprint:y.footprint,courtyard:null,height_metres:y.heightMetres,floors:y.floors,variant:y.variant,style:y.style,pitched:y.pitched,tierRing:y.tierRing,podiumFloors:y.podiumFloors,facade:y.facade,podiumHeightMetres:y.podiumHeightMetres,windowBays:y.windowBays,balconyBays:y.balconyBays,entranceCount:y.entranceCount,roof:y.roof})),parcels:g.parcels.map(y=>({boundary:y.ring,landUse:y.useType})),sdRoads:[],hdRoads:g.hdRoads.map(y=>({id:y.id,class:y.class,widthMetres:y.widthMetres,pathKm:y.centreline.map(M=>[M.x_km,M.y_km]),lanesForward:y.lanesForward,lanesBackward:y.lanesBackward,medianMetres:y.medianMetres,layer:y.layer,structure:y.structure,lanes:y.lanes,connectors:y.connectors})),lanes:g.lanes??g.hdRoads.flatMap(y=>y.lanes??[]),connectors:g.connectors??g.hdRoads.flatMap(y=>y.connectors??[]),junctions:g.junctions??[],river:g.river?{centerline:g.river,widthMetres:g.riverWidthMetres??64}:void 0,compounds:g.compounds??[],trees:g.trees??[]})):o.cities??[],cn=[],Ti=[],zi=[];let kn=0;const so=22e4,Ec=(g,y,M)=>{if(kn>=so||g.length<3)return;let _=0;for(let A=0;A<g.length;A+=1){const O=g[(A+1)%g.length];_+=g[A].x*O.z-O.x*g[A].z}const N=_>0,k=Ge.clamp(Math.floor(M/3.25)-1,1,28);for(let A=0;A<g.length&&kn<so;A+=1){const O=g[A],Y=g[(A+1)%g.length],ee=Y.x-O.x,te=Y.z-O.z,de=Math.hypot(ee,te),pe=de*F;if(pe<7)continue;const fe=ee/Math.max(1e-9,de),ye=te/Math.max(1e-9,de),Se=N?ye:-ye,xe=N?-fe:fe,Ne=Ge.clamp(Math.floor(pe/5.4),1,30),Oe=de/Ne,ze=Math.min(.78,Oe*.31);for(let Ae=0;Ae<k&&kn<so;Ae+=1){const He=(Ae+1)*3.25+.38;if(He+1.45>M)break;const ke=y+He*T,Le=ke+1.38*T;for(let je=0;je<Ne&&kn<so;je+=1){const De=O.x+fe*Oe*(je+.5)+Se*.012*T,Be=O.z+ye*Oe*(je+.5)+xe*.012*T,Ve=ze*T,qe=De-fe*Ve,Qe=Be-ye*Ve,it=De+fe*Ve,$e=Be+ye*Ve,nt=cn.length/3;cn.push(qe,ke,Qe,it,ke,$e,it,Le,$e,qe,Le,Qe);for(let vt=0;vt<4;vt+=1)Ti.push(Se,0,xe);zi.push(nt,nt+1,nt+2,nt,nt+2,nt+3),kn+=1}}}},Ot=(g,y)=>{const M=Ge.clamp(y,0,1),_=g,N=_.transparent===!0;if(_.transparent=!0,_.opacity!==void 0&&(_.opacity=M),_.depthWrite!==void 0){const k=M>.985;_.depthWrite!==k&&(_.depthWrite=k,g.needsUpdate=!0)}N||(g.needsUpdate=!0)},io=(g,y,M)=>1-Ge.smoothstep(g,y,M),Ds=(g,y,M)=>Ge.smoothstep(g,y,M);let Bn=0;const Vl={curtain:[8,9,10,11,12,13,14,15],residential:[0,1,3,22,23],stone:[18,19,16],other:[4,5,6,7,17,20]};for(const g of lo)for(const y of g.buildings??[]){if(Bn>=8e3||y.footprint.length<3)break;const M=String(y.facade??"").toLowerCase(),_=M.includes("curtain")?Vl.curtain:M.includes("brick")?Vl.residential:M.includes("stone")?Vl.stone:Vl.other,N=_[(Number(y.id??0)*7+3)%_.length],k=Ac(N),A=y.footprint.map(ze=>({x:(ze.x_km-Ie)*at,z:(ze.y_km-Ie)*at})),O=new o1;O.moveTo(A[0].x,-A[0].z);for(const ze of A.slice(1))O.lineTo(ze.x,-ze.z);if(O.closePath(),y.courtyard&&y.courtyard.length>=3){const ze=new r2,Ae=y.courtyard.map(He=>({x:(He.x_km-Ie)*at,z:(He.y_km-Ie)*at}));ze.moveTo(Ae[0].x,-Ae[0].z);for(const He of Ae.slice(1))ze.lineTo(He.x,-He.z);ze.closePath(),O.holes.push(ze)}const Y=A.reduce((ze,Ae)=>ze+Ae.x,0)/A.length,ee=A.reduce((ze,Ae)=>ze+Ae.z,0)/A.length,te=new c2(O,{depth:y.height_metres*T,bevelEnabled:!1,curveSegments:1,steps:1,UVGenerator:s0});te.rotateX(-Math.PI/2),te.translate(0,zt(Y,ee)+.08*T,0),te.computeVertexNormals();const de=te.attributes.position,pe=new Float32Array(de.count*3).fill(1),fe=te.attributes.uv.array,ye=.82+Number(y.id??0)%37/37*.36,Se=[ye*(.92+Number(y.parcelId??1)%13/13*.16),ye*.99,ye*(.86+Number(y.id??0)%7/7*.28)],xe=new rt,Ne=new rt,Oe=new rt;for(let ze=0;ze<de.count;ze+=3)if(xe.fromBufferAttribute(de,ze),Ne.fromBufferAttribute(de,ze+1),Oe.fromBufferAttribute(de,ze+2),Ne.sub(xe),Oe.sub(xe),xe.crossVectors(Ne,Oe),Math.abs(xe.y)>.9*xe.length())for(let Ae=0;Ae<3;Ae+=1)pe[(ze+Ae)*3]=yr[0],pe[(ze+Ae)*3+1]=yr[1],pe[(ze+Ae)*3+2]=yr[2],fe[(ze+Ae)*2]=.02,fe[(ze+Ae)*2+1]=.06;else for(let Ae=0;Ae<3;Ae+=1)pe[(ze+Ae)*3]=Se[0],pe[(ze+Ae)*3+1]=Se[1],pe[(ze+Ae)*3+2]=Se[2];te.setAttribute("color",new ya(pe,3)),te.attributes.uv.needsUpdate=!0,vr.push(te),oo.get(k).push(te),_c||Ec(A,zt(Y,ee)+.1*T,y.height_metres),Bn+=1}for(const[g,y]of oo){if(y.length===0)continue;const M=Nn(y,!1);if(!M)continue;M.computeBoundingSphere(),Rs.push(M);const _=Number(g.slice(1)),N=new xt(M,i0(_));N.castShadow=!0,N.receiveShadow=!0,_o.add(N)}_o.visible=!1,U.add(_o);const Us=6500,Ao=13e3,xr=4,al=Number(new URLSearchParams(window.location.search).get("cityslice")),Rc=g=>Number.isFinite(al)&&al>0?al:g>.06?Math.min(250,g*200):xr,Os=7,Ci=4*1024*1024;let Mn=null,ea=null,Sn=null,Hn=null,nl=null;const Nc=()=>{ea?.cancel(),ea=null,Sn=null,Hn&&(Hn.layer.dispose(),Hn=null)},_i=g=>{Nc(),Mn&&(Mn.dispose(),Mn=null),g&&(fp(g.textures),ea=rp(g),Sn=g,new rt((g.origin[0]-Ie)*at,0,(g.origin[1]-Ie)*at))},wr=(g,y)=>{const M=(y.origin[0]-Ie)*at,_=(y.origin[1]-Ie)*at;g.group.scale.setScalar(T),g.group.rotation.y=-y.rotationRadians;const N=zt(M,_);g.group.position.set(M,N+.12*T,_),g.group.updateMatrixWorld(!0);const k=[];g.group.traverse(fe=>{fe!==g.group&&(fe.matrixAutoUpdate=!1);const ye=fe;ye.geometry&&(ye.isMesh||ye.isInstancedMesh)&&(k.push(ye),ye.userData.wasVisible=ye.visible,ye.layers.set(Os),ye.visible=!1)});const A=new Map;for(const fe of k){const ye=fe.material.uuid;A.has(ye)||A.set(ye,A.size)}k.sort((fe,ye)=>A.get(fe.material.uuid)-A.get(ye.material.uuid));const[O,Y,ee,te]=y.extentM,de=Math.max(ee-O,te-Y,200)*.75*T,pe=new Fo(120,1,de*.05,de*12);pe.position.set(M,N+de*1.1,_),pe.up.set(0,0,-1),pe.lookAt(M,N,_),pe.layers.set(Os),pe.updateMatrixWorld(!0),U.add(g.group),Hn={layer:g,pieces:k,next:0,camera:pe},nl??=new Ms(8,8,{type:kl})},ks=g=>{let y=0;for(const M in g.geometry.attributes)y+=g.geometry.attributes[M].array.byteLength;return g.geometry.index&&(y+=g.geometry.index.array.byteLength),y},Ai=g=>{const y=g>.06&&!(al>0);if(ea&&Sn){if(ea.step(Rc(g))&&ea.layer){const Y=ea.layer,ee=Sn;ea=null,Sn=null,wr(Y,ee)}return}if(!Hn)return;const M=Hn,_=performance.now(),N=[],k=new Set;let A=0;const O=y?24:3;for(;M.next<M.pieces.length&&A<Ci*(y?8:1);){const Y=M.pieces[M.next],ee=Y.material.uuid;if(k.size>=O&&!k.has(ee))break;k.add(ee),M.next+=1,A+=ks(Y),Y.visible=!!Y.userData.wasVisible,N.push(Y)}if(N.length>0){const Y=nl,ee=V.shadowMap.needsUpdate;V.shadowMap.needsUpdate=!1,V.setRenderTarget(Y),V.render(U,M.camera),V.setRenderTarget(null),V.shadowMap.needsUpdate=ee;for(const te of N)te.visible=!1;Va("cityWarmSliceMaxMs",performance.now()-_),Va(`cityWarm${M.next}Ms`,performance.now()-_)}if(M.next>=M.pieces.length){for(const Y of M.pieces)Y.visible=!!Y.userData.wasVisible,Y.layers.set(0);Mn=M.layer,Hn=null,console.info("[city] whole-city layer mounted"),Va("cityMountedAtS",performance.now()/1e3),V.shadowMap.needsUpdate=!0}};p.current=_i,_i(b.current);const Bs=o.modernCities??[],Eo=Bs.map(()=>null),Dc=g=>{const y=Bs[g],M=b3(y,{toScene:(_,N)=>[(_-Ie)*at,(N-Ie)*at],heightAt:zt,metresToScene:T});U.add(M.mesh),M.water&&U.add(M.water),Eo[g]=M},r0=g=>{const y=Eo[g];y&&(U.remove(y.mesh),y.water&&U.remove(y.water),y.dispose(),Eo[g]=null)},br=9e3;let Uc=0;const c0=()=>{if(Uc+=1,xs||Uc%30!==1)return;let g=-1,y=br;Bs.forEach((M,_)=>{const N=Ue[_];if(!N)return;const k=(N.xKm-Ie)*at-R.position.x,A=(N.yKm-Ie)*at-R.position.z,O=Math.hypot(k,A)*F;O<y&&(y=O,g=_)}),Eo.forEach((M,_)=>{M&&_!==g&&r0(_)}),g>=0&&!Eo[g]&&Dc(g)};ia||Bs.forEach((g,y)=>Dc(y));const u0=()=>{ia&&c0();const g=Mn?b.current:null;Eo.forEach((y,M)=>{const _=Ue[M];if(!y||!_)return;const N=g!==null&&Math.hypot(g.origin[0]-_.xKm,g.origin[1]-_.yKm)<.05;y.mesh.visible===N&&(y.mesh.visible=!N),y.water&&(y.water.visible=!N)})},Hs=(o.modernCities??[]).map(()=>null);let Oc=0;const Mr=new URLSearchParams(window.location.search).has("tilelog");ih.log=Mr;const f0=Number(new URLSearchParams(window.location.search).get("tilebudget"))||(x.current?.tileBudgetMs??6),h0=g=>{(o.modernCities??[]).forEach((y,M)=>{const _=y.frame.origin,N=(_.x_km-Ie)*at,k=(_.y_km-Ie)*at,A=Math.hypot(N-R.position.x,k-R.position.z)*F;let O=Hs[M];if(!O&&A<Us){const ee=Number(new URLSearchParams(window.location.search).get("tileload"))||(x.current?.tileLoadM??0);O=new qy(M,R2,(de,pe,fe)=>{Eo[M]?.setTileHidden(de,pe,fe)},ee>0?ee:void 0);const te=zt(N,k);O.group.scale.setScalar(T),O.group.rotation.y=-y.frame.rotationRadians,O.group.position.set(N,te+.12*T,k),O.group.updateMatrixWorld(!0),U.add(O.group),Hs[M]=O}if(!O)return;if(A>Ao){O.dispose(),U.remove(O.group),Hs[M]=null;return}const Y=O.group.worldToLocal(R.position.clone());O.update(Y,f0),O.tick(g,A,R),Oc+=1,Mr&&Oc%30===0&&console.log(`[tiles] city ${M} camera ${Y.x.toFixed(0)},${Y.z.toFixed(0)} ${JSON.stringify(O.summary())}`)})},Kl=new sa;cn.length>0&&(Kl.setAttribute("position",new st(cn,3)),Kl.setAttribute("normal",new st(Ti,3)),Kl.setIndex(zi),Kl.computeBoundingSphere());const Sr=new Ke({color:2506569,roughness:.34,metalness:.34,emissive:463126,emissiveIntensity:.08,side:ct,polygonOffset:!0,polygonOffsetFactor:-1,polygonOffsetUnits:-1}),Ls=new xt(Kl,Sr);Ls.visible=!1,_o.add(Ls);const Tr=new Map,zr=(g,y,M={})=>{const _=Tr.get(g);if(_)return _;let N=g.length*131+7;const k=()=>(N=N*1664525+1013904223>>>0,N/4294967296),A=M.size??256,O=new Uint8Array(A*A*4),Y=new Float32Array(A*A);let ee=!1;for(let pe=0;pe<A;pe++)for(let fe=0;fe<A;fe++){const ye=y(fe,pe,k),Se=(pe*A+fe)*4;for(let xe=0;xe<3;xe++)O[Se+xe]=Ge.clamp(ye[xe],0,255);O[Se+3]=255,ye.length>3&&(ee=!0,Y[pe*A+fe]=ye[3])}const te=new ma(O,A,A);te.wrapS=te.wrapT=Ba,te.magFilter=ua,te.minFilter=Yo,te.generateMipmaps=!0,te.colorSpace=wo,te.anisotropy=16,te.needsUpdate=!0;const de=new Ke({map:te,roughness:M.roughness??.95,metalness:M.metalness??0});if(ee){const pe=M.bump??2.2,fe=new Uint8Array(A*A*4);for(let Se=0;Se<A;Se++)for(let xe=0;xe<A;xe++){const Ne=Y[Se*A+(xe+A-1)%A],Oe=Y[Se*A+(xe+1)%A],ze=Y[(Se+A-1)%A*A+xe],Ae=Y[(Se+1)%A*A+xe],He=(Ne-Oe)*pe,ke=(ze-Ae)*pe,Le=Math.hypot(He,ke,1)||1,je=(Se*A+xe)*4;fe[je]=(He/Le*.5+.5)*255,fe[je+1]=(ke/Le*.5+.5)*255,fe[je+2]=(1/Le*.5+.5)*255,fe[je+3]=255}const ye=new ma(fe,A,A);ye.wrapS=ye.wrapT=Ba,ye.magFilter=ua,ye.minFilter=Yo,ye.generateMipmaps=!0,ye.needsUpdate=!0,de.normalMap=ye,de.normalScale=new Et(M.normalScale??.6,M.normalScale??.6)}return Tr.set(g,de),de},d0=()=>zr("asphalt",(g,y,M)=>{const _=Math.sin(g*.043+Math.sin(y*.031)*2.7)*Math.cos(y*.037+g*.017)*6,N=(M()-.5)*22,k=(M()-.5)*13,A=M()<.045?24:0,O=62+_+N*.5+k+A;return[O*.97,O,O*1.05,.5+(k+A)/56+(N<-9?-.14:0)]},{roughness:.97,bump:1.7,normalScale:.5}),kc=()=>zr("paving",(g,y,M)=>{const _=Math.floor(y/16),N=_%2*16,k=(g+N)%32,A=y%16,O=k<2||A<2?-30:0,Y=(Math.floor((g+N)/32)+_)%2?5:0,ee=Math.sin((g+_*11)*.021)*Math.cos(y*.019)*7,te=(M()-.5)*7,de=152+O+Y+ee+te;return[de,de*.985,de*.94,.6+(O?-.42:0)+(M()-.5)*.08]},{roughness:.88,bump:2.6,normalScale:.7}),m0=()=>zr("grass",(g,y,M)=>{const _=Math.sin(g*.09+y*.05)*Math.sin(y*.07+1.3)*10,N=Math.sin(g*.017+1.1)*Math.cos(y*.013)>.74,k=M()<.09?20:(M()-.5)*13,A=[76+_+k,98+_+k,54+_*.7+k*.8];return N&&(A[0]+=32,A[1]+=16,A[2]-=6),[...A,.5+k/60+(M()-.5)*.3]},{roughness:.98,bump:1.4,normalScale:.55}),js=new ka,Cr=[],Ei=[],p0=[],_r=d0().map,v0=_r,Bc=_r,g0=kc().map,qs={boulevard:new Ke({map:_r,color:9211789,roughness:.9,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),avenue:new Ke({map:v0,color:9474446,roughness:.93,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),street:new Ke({map:Bc,color:9737359,roughness:.95,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),service:new Ke({map:Bc,color:10066580,roughness:.98,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2})},ol=new Ke({map:g0,color:11908266,roughness:.98,polygonOffset:!0,polygonOffsetFactor:-1,polygonOffsetUnits:-1}),Ar={boulevard:[],avenue:[],street:[],service:[]},Ri=(g,y,M,_,N=0)=>{const k=(g.x_km-Ie)*at,A=(g.y_km-Ie)*at,O=(y.x_km-Ie)*at,Y=(y.y_km-Ie)*at,ee=Math.hypot(O-k,Y-A)*F,te=Math.max(1,Math.ceil(ee/90)),de=[],pe=[];for(let Se=0;Se<=te;Se+=1){const xe=Se/te,Ne=Ge.lerp(k,O,xe),Oe=Ge.lerp(A,Y,xe),ze=O-k,Ae=Y-A,He=Math.max(1e-9,Math.hypot(ze,Ae)),ke=-Ae/He,Le=ze/He,je=M*T*.5,De=zt(Ne,Oe)+(_+N)*T;de.push(Ne+ke*je,De,Oe+Le*je,Ne-ke*je,De,Oe-Le*je)}for(let Se=0;Se<te;Se+=1){const xe=Se*2;pe.push(xe,xe+2,xe+1,xe+1,xe+2,xe+3)}const fe=new sa;fe.setAttribute("position",new st(de,3));const ye=[];for(let Se=0;Se<de.length;Se+=3)ye.push(de[Se]*F/4,de[Se+2]*F/4);return fe.setAttribute("uv",new st(ye,2)),fe.setIndex(pe),fe.computeVertexNormals(),fe},Ni=(g,y,M,_,N=0)=>{if(!g||!y||!Number.isFinite(_)||(_??0)<=0)return;const k=M==="expressway"||M==="arterial"?"boulevard":M==="collector"?"avenue":M==="local"?"street":M,A=["boulevard","avenue","street","service"].includes(k??"")?k:"street",O=Ri(g,y,_+5,.045,N),Y=Ri(g,y,_,.075,N);Ei.push(O),Ar[A].push(Y)};for(const g of lo){for(const y of g.streets??[])Ni(y.from,y.to,y.class,y.widthMetres??y.width_metres);for(const y of[...g.sdRoads??[],...g.hdRoads??[]]){const M=("widthMetres"in y?y.widthMetres:void 0)??("width_metres"in y?y.width_metres:void 0),_=("pathKm"in y?y.pathKm:void 0)??("path_km"in y?y.path_km:void 0);if(_&&_.length>=2)for(let N=0;N+1<_.length;N+=1)Ni({x_km:_[N][0],y_km:_[N][1]},{x_km:_[N+1][0],y_km:_[N+1][1]},y.class,M,Number(y.layer??0)>0?6:0);else"from"in y&&"to"in y&&y.from&&y.to&&Ni(y.from,y.to,y.class,M)}}const Gs=Ei.length>0?Nn(Ei,!1):null;for(const g of Ei)g.dispose();Gs&&(Gs.computeBoundingSphere(),js.add(new xt(Gs,ol)),Cr.push(Gs));for(const g of Object.keys(Ar)){const y=Ar[g];if(y.length===0)continue;const M=Nn(y,!1);for(const _ of y)_.dispose();M&&(M.computeBoundingSphere(),js.add(new xt(M,qs[g])),Cr.push(M))}js.visible=!1,U.add(js);const Fs=new ka,Er=[],Ro=new Map,ro={polygonOffset:!0,polygonOffsetFactor:-6,polygonOffsetUnits:-6},ja={white:new Dn({color:15330276,transparent:!0,opacity:.92,side:ct,depthWrite:!1,...ro}),yellow:new Dn({color:15123291,transparent:!0,opacity:.96,side:ct,depthWrite:!1,...ro}),median:new Ke({color:8225402,roughness:.9,transparent:!0,opacity:.95,side:ct,depthWrite:!1,...ro}),paving:new Ke({color:9407872,roughness:.96,transparent:!0,opacity:.85,side:ct,depthWrite:!1,...ro}),arrow:new Dn({color:14674143,transparent:!0,opacity:.95,side:ct,depthWrite:!1,...ro}),crosswalk:new Dn({color:15921380,transparent:!0,opacity:.9,side:ct,depthWrite:!1,...ro}),connectorLeft:new Dn({color:13620428,transparent:!0,opacity:.3,side:ct,depthWrite:!1,...ro}),connectorRight:new Dn({color:13620428,transparent:!0,opacity:.3,side:ct,depthWrite:!1,...ro}),connectorThrough:new Dn({color:13620428,transparent:!0,opacity:.3,side:ct,depthWrite:!1,...ro})},Rr=(g,y)=>{const M=Ro.get(y);M?M.push(g):Ro.set(y,[g])},Ln=(g,y=0)=>{const M=g.map(_=>({..._,x:(_.x_km-Ie)*at,z:(_.y_km-Ie)*at}));return M.map((_,N)=>{const k=M[Math.max(0,N-1)],A=M[Math.min(M.length-1,N+1)],O=A.x-k.x,Y=A.z-k.z,ee=Math.max(1e-9,Math.hypot(O,Y));return{x:_.x-Y/ee*y*T,z:_.z+O/ee*y*T}})},Yl=(g,y,M)=>{if(g.length<2)return null;const _=[],N=[];for(let A=0;A<g.length;A+=1){const O=g[Math.max(0,A-1)],Y=g[Math.min(g.length-1,A+1)],ee=Y.x-O.x,te=Y.z-O.z,de=Math.max(1e-9,Math.hypot(ee,te)),pe=-te/de,fe=ee/de,ye=y*T*.5,Se=zt(g[A].x,g[A].z)+M*T;_.push(g[A].x+pe*ye,Se,g[A].z+fe*ye,g[A].x-pe*ye,Se,g[A].z-fe*ye)}for(let A=0;A+1<g.length;A+=1){const O=A*2;N.push(O,O+2,O+1,O+1,O+2,O+3)}const k=new sa;return k.setAttribute("position",new st(_,3)),k.setIndex(N),k.computeVertexNormals(),k},jn=(g,y,M,_,N)=>{const k=Yl(Ln(g,y),M,_);k&&Rr(k,N)},Vs=(g,y,M,_,N,k,A=.1)=>{const O=Ln(g,y);for(let Y=0;Y+1<O.length;Y+=1){const ee=O[Y],te=O[Y+1],pe=Math.hypot(te.x-ee.x,te.z-ee.z)*F;for(let fe=0;fe<pe;fe+=_+N){const ye=Math.min(pe,fe+_);if(ye<=fe+.05)continue;const Se=fe/Math.max(1e-9,pe),xe=ye/Math.max(1e-9,pe),Ne=Yl([{x:Ge.lerp(ee.x,te.x,Se),z:Ge.lerp(ee.z,te.z,Se)},{x:Ge.lerp(ee.x,te.x,xe),z:Ge.lerp(ee.z,te.z,xe)}],M,A);Ne&&Rr(Ne,k)}}},Nr=(g,y,M,_="straight",N=.12)=>{const k=Ln(g,y);if(k.length<2)return;let A=0;for(let qe=0;qe+1<k.length;qe+=1)A+=Math.hypot(k[qe+1].x-k[qe].x,k[qe+1].z-k[qe].z);const O=Math.max(2*T,A-16*T);let Y=0,ee=k[0],te=k[1];for(let qe=0;qe+1<k.length;qe+=1){const Qe=Math.hypot(k[qe+1].x-k[qe].x,k[qe+1].z-k[qe].z);if(Y+Qe>=O){ee=k[qe],te=k[qe+1];break}Y+=Qe}const de=te.x-ee.x,pe=te.z-ee.z,fe=Math.max(1e-9,Math.hypot(de,pe)),ye=de/fe,Se=pe/fe,xe=-Se,Ne=ye,Oe={x:ee.x+ye*Math.min(8*T,fe*.45),z:ee.z+Se*Math.min(8*T,fe*.45)},ze=4.8*T,Ae=1.35*T,He={x:Oe.x+ye*ze*.55,z:Oe.z+Se*ze*.55},ke={x:Oe.x-ye*ze*.45,z:Oe.z-Se*ze*.45},Le={x:Oe.x-ye*ze*.05,z:Oe.z-Se*ze*.05},je=_==="left"?-1:_==="right"?1:0,De=[He,{x:Le.x+xe*Ae,z:Le.z+Ne*Ae},ke,{x:Le.x-xe*Ae,z:Le.z-Ne*Ae}];if(je!==0){const qe=je*1.2*T;De[0]={x:De[0].x+xe*qe,z:De[0].z+Ne*qe}}const Be=zt(Oe.x,Oe.z)+N*T,Ve=new sa;Ve.setAttribute("position",new st(De.flatMap(qe=>[qe.x,Be,qe.z]),3)),Ve.setIndex([0,1,2,0,2,3]),Ve.computeVertexNormals(),Rr(Ve,M)};for(const g of lo){for(const y of g.hdRoads??[]){const M=y,_=M.centreline??M.pathKm?.map(([de,pe])=>({x_km:de,y_km:pe}))??M.path_km?.map(([de,pe])=>({x_km:de,y_km:pe}))??[];if(_.length<2)continue;const N=Number(M.layer??0)>0?6:0,k=M.lanes??[],A=Math.max(1,k.length?Math.max(...k.map(de=>Number(de.indexFromCurb??de.index??0)+1)):y.class==="expressway"||y.class==="arterial"?3:y.class==="collector"?2:1),O=(y.widthMetres??10)*.5;jn(_,-O+.35,.14,.11+N,ja.white),jn(_,O-.35,.14,.11+N,ja.white);const Y=(y.widthMetres??10)/Math.max(2,A*2),ee=k.length>0?[-1,1].flatMap(de=>{const pe=k.filter(fe=>Number(fe.direction??de)===de).map(fe=>Math.abs(Number(fe.offsetMetres??0))).filter(fe=>Number.isFinite(fe)).sort((fe,ye)=>fe-ye);return pe.slice(0,-1).map((fe,ye)=>de*((fe+pe[ye+1])*.5))}):[-1,1].flatMap(de=>Array.from({length:Math.max(0,A-1)},(pe,fe)=>de*(fe+1)*Y));for(const de of ee)Vs(_,de,.13,y.class==="expressway"?6:3,y.class==="expressway"?9:5,ja.white,.1+N);const te=Number(M.medianMetres??0);te>.3?jn(_,0,te*.85,.115+N,ja.median):(y.class==="arterial"||y.class==="expressway")&&jn(_,0,.16,.12+N,ja.yellow);for(const de of k){const pe=Number(de.offsetMetres??0),ye=(de.markings??[]).find(xe=>String(xe.kind).toLowerCase()==="arrow")?.arrow,Se=Number(de.direction??1)<0?_.slice().reverse():_;ye&&Nr(Se,pe,ja.arrow,String(ye).toLowerCase().includes("left")?"left":String(ye).toLowerCase().includes("right")?"right":"straight",.12+N)}}for(const y of g.lanes??[]){const M=y.path;if(M&&M.length>=2){const _=Number(y.direction??1)<0?M.slice().reverse():M;for(const N of y.markings??[])String(N.kind).toLowerCase()==="arrow"&&Nr(_,Number(y.offsetMetres??0),ja.arrow,String(N.arrow??"straight").toLowerCase().includes("left")?"left":String(N.arrow??"").toLowerCase().includes("right")?"right":"straight")}}for(const y of[...g.connectors??[],...(g.junctions??[]).flatMap(M=>M.connectors??[])]){const M=y.path??y.centreline;if(!M||M.length<2)continue;const _=String(y.movement??"through").toLowerCase(),N=_.includes("left")?ja.connectorLeft:_.includes("right")?ja.connectorRight:ja.connectorThrough;jn(M,0,y.widthMetres??.22,.14,N),Nr(M,0,N,_)}for(const y of g.junctions??[])for(const M of y.crosswalks??[]){if(M.length<2)continue;const _=M[0],N=M[M.length-1],k=N.x_km-_.x_km,A=N.y_km-_.y_km,O=Math.hypot(k,A);if(O<.001)continue;const Y=-A/O,ee=k/O;for(let te=-3;te<=3;te+=1){const de=te*.9/1e3,pe={x_km:_.x_km+Y*de-k/O*2/1e3,y_km:_.y_km+ee*de-A/O*2/1e3},fe={x_km:_.x_km+Y*de+k/O*2/1e3,y_km:_.y_km+ee*de+A/O*2/1e3};jn([pe,fe],0,.45,.15,ja.crosswalk)}}}for(const[g,y]of Ro){if(y.length===0)continue;const M=y.length===1?y[0]:Nn(y,!1);if(!M){for(const _ of y)_.dispose();continue}if(M!==y[0])for(const _ of y)_.dispose();M.computeBoundingSphere(),Er.push(M),Fs.add(new xt(M,g))}Fs.visible=!1,U.add(Fs);const Ea=new ka,Dr=[],Di=new Map,Wl=(g,y)=>{const M=Di.get(y);M?M.push(g):Di.set(y,[g])},Ks=[],Ur=new Map,Or=[],Ui=[],kr=[],No=[],Br=new Ke({color:13617078,roughness:.86,transparent:!0,opacity:.96}),Ys=new Ke({color:7828331,roughness:.74,transparent:!0,opacity:.96}),Ws=new Ke({color:3423042,roughness:.46,metalness:.12,transparent:!0,opacity:.98}),Ra=new Ke({color:6581616,roughness:.72,transparent:!0,opacity:.78}),ll=new Ke({color:9410197,roughness:.82,transparent:!0,opacity:.94}),Oi=new Ke({color:6716805,roughness:.36,metalness:.08,transparent:!0,opacity:.85}),co=(g,y,M,_,N,k,A,O=0)=>{const Y=new sr(_*T,N*T,k*T);Y.rotateY(-O),Y.translate(g,y,M),Wl(Y,A)},Hc=(g,y,M,_)=>{const N=Ln([g])[0],k=Ln([y])[0];if(!N||!k)return;const A=Math.hypot(k.x-N.x,k.z-N.z)*F;if(A<1.2)return;const O=Math.atan2(k.z-N.z,k.x-N.x),Y=(N.x+k.x)*.5,ee=(N.z+k.z)*.5,te=zt(Y,ee)+M*T*.5;co(Y,te,ee,A,M,.28,_,O),co(Y,te+M*T*.53,ee,A+.1,.12,.44,Ys,O)},Lc=g=>g?.toLowerCase().includes("curtain")?7046542:g?.toLowerCase().includes("stone")?11642255:g?.toLowerCase().includes("brick")?11109747:8752269;for(const g of lo){for(const y of g.compounds??[]){const M=y.boundary??y.wallRing;if(!M||M.length<3)continue;for(let N=0;N<M.length;N+=1)Hc(M[N],M[(N+1)%M.length],y.fenceHeightMetres??1.8,Br);const _=y.gatePoints??[];if(_.length>=2){const N=Ln([_[0]])[0],k=Ln([_[1]])[0],A=(N.x+k.x)*.5,O=(N.z+k.z)*.5,Y=Math.atan2(k.z-N.z,k.x-N.x),ee=-Math.sin(Y),te=Math.cos(Y),de=zt(A,O);co(A-ee*5.5*T,de+2.1*T,O-te*5.5*T,.9,4.2,.9,Ys,Y),co(A+ee*5.5*T,de+2.1*T,O+te*5.5*T,.9,4.2,.9,Ys,Y),co(A,de+3.4*T,O,12.4,1.15,.45,Ws,Y),co(A,de+1*T,O,.22,1.8,.18,Ws,Y)}for(const N of y.paths??[]){const k=Yl(Ln(N),(y.roadWidthMetres??5)+.3,.12);k&&Wl(k,ja.paving)}}for(const y of g.buildings??[]){if(y.footprint.length<3)continue;const M=Ln(y.footprint),_=zt(M.reduce((pe,fe)=>pe+fe.x,0)/M.length,M.reduce((pe,fe)=>pe+fe.z,0)/M.length),N=y,k=Number(N.heightMetres??N.height_metres??3.2),A=Number(y.floors??Math.max(1,Math.round(k/3.2))),O=Number(y.podiumHeightMetres??0);if(O>0){const pe=Yl([...M,M[0]],.24,0);if(pe){const fe=pe.attributes.position;for(let ye=0;ye<fe.count;ye+=1)fe.setY(ye,_+O*T);fe.needsUpdate=!0,Wl(pe,ll)}}const Y=Lc(y.facade);let ee=Ur.get(Y);ee||(ee=new Ke({color:Y,roughness:.56,metalness:y.facade?.toLowerCase().includes("curtain")?.18:.02,transparent:!0,opacity:.9}),Ur.set(Y,ee),Ks.push(ee));const te=y.balconyBays&&y.balconyBays>0?2:4;for(let pe=1;pe<A;pe+=te){const fe=Yl([...M,M[0]],y.balconyBays&&y.balconyBays>0?.34:.16,0);if(!fe)continue;const ye=fe.attributes.position,Se=_+pe*3.2*T;for(let xe=0;xe<ye.count;xe+=1)ye.setY(xe,Se);ye.needsUpdate=!0,Wl(fe,y.balconyBays&&y.balconyBays>0?Ra:ee)}const de=Math.max(1,Math.min(8,Number(y.entranceCount??1)));for(let pe=0;pe<de;pe+=1){const fe=M[pe%M.length],ye=M[(pe+1)%M.length],Se=(pe+.5)/de,xe=Ge.lerp(fe.x,ye.x,Se),Ne=Ge.lerp(fe.z,ye.z,Se),Oe=ye.x-fe.x,ze=ye.z-fe.z;co(xe,_+1.15*T,Ne,1.4,2.3,.08,Oi,Math.atan2(ze,Oe))}}for(const y of g.junctions??[])for(const M of y.signalHeads??[]){const _=M.position;if(!_)continue;const N=(_.x_km-Ie)*at,k=(_.y_km-Ie)*at,A=zt(N,k),O=Number(M.heightMetres??5.8);co(N,A+O*T*.5,k,.12,O,.12,Ws);const Y=String(M.aspects?.[0]??"red").toLowerCase(),ee=new Ke({color:Y.includes("green")?5821053:Y.includes("yellow")?15188815:14110029,emissive:Y.includes("green")?1457444:Y.includes("yellow")?4799250:4592403,emissiveIntensity:.55,roughness:.3,transparent:!0,opacity:.98});Ui.push(ee);const te=new ws(.22*T,8,6);te.translate(N,A+(O-.45)*T,k),Or.push(te),Ea.add(new xt(te,ee))}}for(const[g,y]of Di){if(y.length===0)continue;const M=y.length===1?y[0]:Nn(y,!1);if(!M){for(const _ of y)_.dispose();continue}if(M!==y[0])for(const _ of y)_.dispose();M.computeBoundingSphere(),Dr.push(M),Ea.add(new xt(M,g))}const Xs=new Ke({color:11777193,roughness:.85});No.push(Xs);const jc=[4157496,5013312,3629871,5671493],sl=new Map,Hr=g=>{let y=sl.get(g);return y||(y=new Ke({color:g,roughness:.95,flatShading:!0}),sl.set(g,y),No.push(y)),y},uo=new Map,Lr=(g,y,M,_,N,k)=>{let A=k>>>0;const O=()=>(A=A*1664525+1013904223>>>0,A/4294967296),Y=new ws(_,7,5);Y.scale(1,N,1),Y.translate(g,y+_*N*.62,M);const ee=Hr(jc[Math.floor(O()*jc.length)]),te=uo.get(ee);te?te.push(Y):uo.set(ee,[Y])},jr=(()=>{const y=new Uint8Array(16384);let M=9137;const _=()=>(M=M*1664525+1013904223>>>0,M/4294967296),N=Array.from({length:10},()=>({x:6+_()*52,w:2.1+_()*2.5,lean:(_()-.5)*20,tint:_()}));for(let A=0;A<64;A++)for(let O=0;O<64;O++){const Y=1-A/64;let ee=-1;for(const de of N){const pe=de.x+de.lean*(1-Y);if(Math.abs(O-pe)<de.w*(.3+Y*.7)){ee=de.tint;break}}const te=(A*64+O)*4;y[te]=58+ee*34,y[te+1]=ee<0?0:124+ee*58-Y*22,y[te+2]=48+ee*26,y[te+3]=ee<0?0:255}const k=new ma(y,64,64);return k.magFilter=ua,k.minFilter=Yo,k.generateMipmaps=!0,k.colorSpace=wo,k.needsUpdate=!0,new Ke({map:k,alphaTest:.5,side:ct,roughness:.95,vertexColors:!0})})();No.push(jr);{const g=[],y=[],M=[],_=[];let N=0,k=4200,A=5151;const O=()=>(A=A*1664525+1013904223>>>0,A/4294967296),Y=(ee,te,de,pe)=>{if(k--<=0)return;const fe=.75+O()*.45;for(let ye=0;ye<2;ye++){const Se=O()*Math.PI+ye*Math.PI/2,xe=Math.cos(Se)*pe/2,Ne=Math.sin(Se)*pe/2;g.push(ee-xe,de,te-Ne,ee+xe,de,te+Ne,ee+xe,de+pe*.95,te+Ne,ee-xe,de+pe*.95,te-Ne),y.push(0,0,1,0,1,1,0,1);for(let Oe=0;Oe<4;Oe++)M.push(fe,fe*(.95+O()*.12),fe*.88);_.push(N,N+1,N+2,N,N+2,N+3),N+=4}};for(const ee of o.modernCities??[]){for(const xe of ee.parcels??[]){if(!String(xe.useType??"").toLowerCase().includes("park")||!xe.ring||xe.ring.length<3)continue;let Oe=1/0,ze=-1/0,Ae=1/0,He=-1/0;for(const Le of xe.ring)Oe=Math.min(Oe,Le.x_km),ze=Math.max(ze,Le.x_km),Ae=Math.min(Ae,Le.y_km),He=Math.max(He,Le.y_km);const ke=Math.min(24,Math.max(3,Math.round((ze-Oe)*(He-Ae)*1e6*.03)));for(let Le=0;Le<ke*3&&Le<90;Le++){const je=Oe+O()*(ze-Oe),De=Ae+O()*(He-Ae),Be=(je-Ie)*at,Ve=(De-Ie)*at;Y(Be,Ve,zt(Be,Ve)+.06*T,.55*T*(.75+O()*.5)*12.5)}}for(const xe of ee.hdRoads??[]){const Ne=Number(xe.medianMetres??0);if(Ne<1.5||xe.bridge||Number(xe.layer??0)!==0)continue;const Oe=xe.centreline;if(!Oe||Oe.length<2)continue;const ze=Oe.map(He=>({x:(He.x_km-Ie)*at,z:(He.y_km-Ie)*at})),Ae=xe.class==="expressway";for(let He=0;He+1<ze.length;He++){const ke=ze[He],Le=ze[He+1],je=Math.hypot(Le.x-ke.x,Le.z-ke.z)*F,De=Math.max(1,Math.round(je/2.2));for(let Be=0;Be<De;Be++){const Ve=(Be+.5)/De,qe=ke.x+(Le.x-ke.x)*Ve,Qe=ke.z+(Le.z-ke.z)*Ve,it=zt(qe,Qe)+.1*T,$e=(O()-.5)*Math.max(.3,Ne*.42)*T,nt=-((Le.z-ke.z)/Math.max(1e-6,Math.hypot(Le.x-ke.x,Le.z-ke.z))),vt=(Le.x-ke.x)/Math.max(1e-6,Math.hypot(Le.x-ke.x,Le.z-ke.z));if(Ae){const Ct=new sr(je/De*T*1.05,.8*T,.35*T);Ct.rotateY(-Math.atan2(Le.z-ke.z,Le.x-ke.x)),Ct.translate(qe,it+.4*T,Qe);const Rt=uo.get(Xs);Rt?Rt.push(Ct):uo.set(Xs,[Ct]);continue}Lr(qe+nt*$e,it,Qe+vt*$e,(.42+O()*.4)*T*12.5*.09+.42*T,.7+O()*.3,He*31+Be*7+(xe.id??0)*977>>>0),O()<.35&&Y(qe,Qe,it,(.4+O()*.25)*T*12.5*.09+.3*T)}}}const te=[],de=[];for(const xe of ee.hdRoads??[]){if(Number(xe.layer??0)!==0||xe.class!=="arterial"&&xe.class!=="collector")continue;const Ne=xe.centreline;if(!Ne||Ne.length<2)continue;const Oe=Ne.map(Ae=>({x:(Ae.x_km-Ie)*at,z:(Ae.y_km-Ie)*at}));let ze=0;for(let Ae=0;Ae+1<Oe.length;Ae++){const He=Oe[Ae],ke=Oe[Ae+1],Le=Math.hypot(ke.x-He.x,ke.z-He.z),je=(ke.x-He.x)/Math.max(1e-9,Le),De=(ke.z-He.z)/Math.max(1e-9,Le);let Be=ze;for(;Be<Le;){const Ve=Math.round((Be+ze)/(35*T))%2===0?1:-1,qe=He.x+je*Be,Qe=He.z+De*Be,it=Math.atan2(-De,je),$e=-De*Ve,nt=je*Ve,vt=qe+$e*(xe.widthMetres*.5+4.5)*T,Ct=Qe+nt*(xe.widthMetres*.5+4.5)*T,Rt=zt(vt,Ct),It=new ir(.06*T,.1*T,7.6*T,6);It.translate(vt,Rt+3.8*T,Ct),te.push(It);const wt=2.4*T,St=new sr(wt,.09*T,.09*T);St.translate(wt/2,0,0),St.rotateY(it),St.translate(vt,Rt+7.45*T,Ct),te.push(St);const Xt=new sr(.3*T,.13*T,.8*T);Xt.translate(wt-.25*T,-.14*T,0),Xt.rotateY(it),Xt.translate(vt,Rt+7.45*T,Ct),de.push(Xt),Be+=35*T}ze+=Le}}const pe=new Ke({color:4871520,roughness:.6}),fe=new Ke({color:14213864,roughness:.4});if(No.push(pe,fe),te.length){const xe=Nn(te,!1);for(const Ne of te)Ne.dispose();if(xe){xe.computeBoundingSphere();const Ne=new xt(xe,pe);Ne.castShadow=!0,Ea.add(Ne)}}if(de.length){const xe=Nn(de,!1);for(const Ne of de)Ne.dispose();xe&&(xe.computeBoundingSphere(),Ea.add(new xt(xe,fe)))}const ye=new Ke({color:10134696,roughness:.5,metalness:.35});No.push(ye);const Se=[];for(const xe of ee.hdRoads??[]){if(Number(xe.layer??0)!==0||xe.class!=="arterial"||xe.bridge)continue;const Ne=xe.centreline;if(!Ne||Ne.length<2)continue;const Oe=Ne.map(ze=>({x:(ze.x_km-Ie)*at,z:(ze.y_km-Ie)*at}));for(const ze of[-1,1])for(let Ae=0;Ae+1<Oe.length;Ae++){const He=Oe[Ae],ke=Oe[Ae+1],Le=Math.hypot(ke.x-He.x,ke.z-He.z)*F,je=Math.max(1,Math.round(Le/3)),De=-((ke.z-He.z)/Math.max(1e-9,Math.hypot(ke.x-He.x,ke.z-He.z))),Be=(ke.x-He.x)/Math.max(1e-9,Math.hypot(ke.x-He.x,ke.z-He.z)),Ve=ze*(xe.widthMetres*.5+3.6)*T;for(let qe=0;qe<je;qe++){const Qe=qe/je,it=(qe+1)/je,$e=He.x+(ke.x-He.x)*Qe+De*Ve,nt=He.z+(ke.z-He.z)*Qe+Be*Ve,vt=He.x+(ke.x-He.x)*it+De*Ve,Ct=He.z+(ke.z-He.z)*it+Be*Ve,Rt=zt(($e+vt)/2,(nt+Ct)/2),It=Math.atan2(-(Ct-nt),vt-$e),wt=Math.max(.05,Math.hypot(vt-$e,Ct-nt));for(const Xt of[.52,1.02]){const $a=new sr(wt,.05*T,.045*T);$a.rotateY(-It),$a.translate(($e+vt)/2,Rt+Xt*T,(nt+Ct)/2),Se.push($a)}const St=new ir(.022*T,.022*T,1.06*T,5);St.translate(vt,Rt+.53*T,Ct),Se.push(St)}}}if(Se.length){const xe=Nn(Se,!1);for(const Ne of Se)Ne.dispose();if(xe){xe.computeBoundingSphere();const Ne=new xt(xe,ye);Ne.castShadow=!0,Ea.add(Ne)}}}for(const[ee,te]of uo){if(te.length===0)continue;const de=te.length===1?te[0]:Nn(te,!1);if(!de)continue;de.computeBoundingSphere();const pe=new xt(de,ee);Ea.add(pe)}if(N>0){const ee=new sa;ee.setAttribute("position",new st(g,3)),ee.setAttribute("uv",new st(y,2)),ee.setAttribute("color",new st(M,3)),ee.setIndex(_),ee.computeVertexNormals(),Ea.add(new xt(ee,jr))}}Ea.visible=!1,U.add(Ea);const Do=[6259528,7311698,5141312,8231509,9084254].map(g=>new $t(g));let Uo=null;const qr=()=>{if(Uo)return Uo;const g=64,y=new Uint8Array(g*g*4);let M=9931;const _=()=>(M=M*1664525+1013904223>>>0,M/4294967296);for(let k=0;k<26;k++){const A=6+_()*52,O=6+_()*52,Y=_()*Math.PI,ee=5.5+_()*5.5,te=2.4+_()*2.4,de=Math.cos(Y),pe=Math.sin(Y),fe=.72+_()*.36;for(let ye=0;ye<g;ye++)for(let Se=0;Se<g;Se++){const xe=Se+.5-A,Ne=ye+.5-O,Oe=xe*de+Ne*pe,ze=-xe*pe+Ne*de;if(Oe*Oe/(ee*ee)+ze*ze/(te*te)>1)continue;const Ae=(ye*g+Se)*4,He=226*fe;y[Ae]=He,y[Ae+1]=He,y[Ae+2]=He,y[Ae+3]=255}}const N=new ma(y,g,g);return N.colorSpace=wo,N.magFilter=ua,N.minFilter=Yo,N.generateMipmaps=!0,N.needsUpdate=!0,Uo=N,N},Gr=new Ke({map:qr(),color:16777215,vertexColors:!0,roughness:.88,side:ct,alphaTest:.45}),Oo=new Ke({color:7035205,roughness:.95});No.push(Gr,Oo);const qc=(g,y,M,_,N)=>{let k=N>>>0;const A=()=>(k=k*1664525+1013904223>>>0,k/4294967296),O=_*(.36+A()*.12),Y=[],ee=[],te=[];let de=0;const pe=(Be,Ve,qe,Qe,it,$e,nt,vt)=>{const Ct=Qe-Be,Rt=it-Ve,It=$e-qe,wt=Math.hypot(Ct,Rt,It)||.01,St=Ct/wt,Xt=Rt/wt,$a=It/wt;let en=-Xt,oa=St,Gt=0;const Tt=Math.hypot(en,oa,Gt);Tt<.05?(en=1,oa=0,Gt=0):(en/=Tt,oa/=Tt,Gt/=Tt);const Wn=Xt*Gt-$a*oa,Sa=$a*en-St*Gt,et=St*oa-Xt*en,ft=5;for(let Nt=0;Nt<ft;Nt++){const pl=Nt/ft*Math.PI*2,vl=(Nt+1)/ft*Math.PI*2,Xn=(Pt,Ki,Zn)=>{const Yi=Math.cos(Zn)*Ki,Wi=Math.sin(Zn)*Ki;return[Be+St*wt*Pt+en*Yi+Wn*Wi,Ve+Xt*wt*Pt+oa*Yi+Sa*Wi,qe+$a*wt*Pt+Gt*Yi+et*Wi,Math.cos(Zn)*en+Math.sin(Zn)*Wn,Math.cos(Zn)*oa+Math.sin(Zn)*Sa,Math.cos(Zn)*Gt+Math.sin(Zn)*et]},po=Xn(0,nt,pl),Pn=Xn(0,nt,vl),_n=Xn(1,vt,pl),os=Xn(1,vt,vl);Y.push(po[0],po[1],po[2],_n[0],_n[1],_n[2],os[0],os[1],os[2],Pn[0],Pn[1],Pn[2]),ee.push(po[3],po[4],po[5],_n[3],_n[4],_n[5],os[3],os[4],os[5],Pn[3],Pn[4],Pn[5]),te.push(de,de+1,de+2,de,de+2,de+3),de+=4}};pe(0,0,0,0,O,0,.07+_*.032,.05+_*.016);const fe=8+Math.floor(A()*4),ye=[],Se=[],xe=[],Ne=[],Oe=[];let ze=0;const Ae=(Be,Ve,qe,Qe,it,$e,nt,vt,Ct)=>{let Rt=-$e,It=0,wt=Qe;const St=Math.hypot(Rt,It,wt);St<.05?(Rt=1,It=0,wt=0):(Rt/=St,It/=St,wt/=St);const Xt=it*wt-$e*It,$a=$e*Rt-Qe*wt,en=Qe*It-it*Rt,oa=nt*.5,Gt=Qe*.4,Tt=it*.4+1,Wn=$e*.4,Sa=Math.hypot(Gt,Tt,Wn)||1,et=[[-1,-1],[1,-1],[1,1],[-1,1]];for(const[ft,Nt]of et)ye.push(Be+(Rt*ft+Xt*Nt)*oa,Ve+(It*ft+$a*Nt)*oa,qe+(wt*ft+en*Nt)*oa),Oe.push(Gt/Sa,Tt/Sa,Wn/Sa);Ne.push(0,0,1,0,1,1,0,1);for(let ft=0;ft<4;ft++)Se.push(vt.r*Ct,vt.g*Ct,vt.b*Ct);xe.push(ze,ze+1,ze+2,ze,ze+2,ze+3),ze+=4};for(let Be=0;Be<fe;Be++){const Ve=(Be+A()*.7)*(Math.PI*2/fe),qe=O*(.55+Be/fe*.6),Qe=_*(.16+A()*.16),it=Math.cos(Ve)*Qe,$e=Math.sin(Ve)*Qe,nt=qe+_*(.18+A()*.24);pe(0,qe,0,it,nt,$e,.05,.02);const vt=_*(.16+A()*.08),Ct=22+Math.floor(A()*10),Rt=Do[Math.floor(A()*Do.length)];for(let It=0;It<Ct;It++){const wt=A()*Math.PI*2,St=Math.acos(2*A()-1),Xt=vt*(.35+A()*.75),$a=it+Math.sin(St)*Math.cos(wt)*Xt,en=nt+Math.cos(St)*Xt*.85,oa=$e+Math.sin(St)*Math.sin(wt)*Xt,Gt=A()*Math.PI*2,Tt=Math.acos(2*A()-1),Wn=Math.sin(Tt)*Math.cos(Gt),Sa=Math.cos(Tt)*.6,et=Math.sin(Tt)*Math.sin(Gt),ft=_*(.15+A()*.13),Nt=Math.max(0,Math.min(1,(en-.4*_)/(_*.9))),pl=(.74+A()*.36)*(.78+.3*Nt);Ae($a,en,oa,Wn,Sa,et,ft,Rt,pl)}}const He=T,ke=new sa,Le=[];for(const Be of Y)Le.push(Be*He);ke.setAttribute("position",new st(Le,3)),ke.setAttribute("normal",new st(ee,3)),ke.setAttribute("uv",new st(new Array(de*2).fill(0),2)),ke.setIndex(te),ke.translate(g,y,M);const je=new sa,De=[];for(const Be of ye)De.push(Be*He);return je.setAttribute("position",new st(De,3)),je.setAttribute("color",new st(Se,3)),je.setAttribute("normal",new st(Oe,3)),je.setAttribute("uv",new st(Ne,2)),je.setIndex(xe),je.translate(g,y,M),{wood:ke,leaves:je}},ki=lo.flatMap(g=>g.trees??[]);{const y=Math.max(1,Math.ceil(ki.length/1200)),M=[],_=[];let N=0;for(let O=0;O<ki.length;O+=y){const Y=ki[O],ee=Y?.point??{x_km:Y?.x_km??0,y_km:Y?.y_km??0},te=(ee.x_km-Ie)*at,de=(ee.y_km-Ie)*at,pe=zt(te,de),fe=Ge.clamp(Number(Y?.heightMetres??10),4,26),ye=qc(te,pe,de,fe,(Y?.id??N)*7919+17>>>0);M.push(ye.wood),_.push(ye.leaves),N+=1}const k=M.length?Nn(M,!1):null;for(const O of M)O.dispose();const A=_.length?Nn(_,!1):null;for(const O of _)O.dispose();if(k){k.computeBoundingSphere();const O=new xt(k,Oo);O.castShadow=!0,Ea.add(O),kr.push(k)}if(A){A.computeBoundingSphere();const O=new xt(A,Gr);Ea.add(O),kr.push(A)}}const Ps=new ka,Gc=[],Fc=[],Fr={residential:new Ke({color:9410192,roughness:1,transparent:!0,opacity:.34,depthWrite:!1}),commercial:new Ke({color:10917753,roughness:.98,transparent:!0,opacity:.38,depthWrite:!1}),green:new Ke({color:6193500,roughness:1,transparent:!0,opacity:.42,depthWrite:!1})},Bi={residential:[],commercial:[],green:[]},Zs=new ka,ko=[],un=[],Xl=new Ke({color:2977402,roughness:.2,metalness:.08,transparent:!0,opacity:.84,depthWrite:!1});for(const g of lo){for(const M of g.parcels??[]){if(!M.boundary||M.boundary.length<3)continue;const _=M.boundary.map(te=>({x:(te.x_km-Ie)*at,z:(te.y_km-Ie)*at})),N=new o1;N.moveTo(_[0].x,-_[0].z);for(const te of _.slice(1))N.lineTo(te.x,-te.z);N.closePath();const k=new u2(N);k.rotateX(-Math.PI/2);const A=_.reduce((te,de)=>te+de.x,0)/_.length,O=_.reduce((te,de)=>te+de.z,0)/_.length;k.translate(0,zt(A,O)+.032*T,0);const Y=(("landUse"in M?M.landUse:"land_use"in M?M.land_use:void 0)??"residential").toLowerCase(),ee=Y.includes("park")||Y.includes("green")||Y.includes("water")?"green":Y.includes("commercial")||Y.includes("office")||Y.includes("mixed")?"commercial":"residential";Bi[ee].push(k),Gc.push(k)}const y=g.river;if(y){const M=("centerline_km"in y?y.centerline_km:void 0)??("path_km"in y?y.path_km:void 0),N=("centerline"in y?y.centerline:void 0)??("path"in y?y.path:void 0)??M?.map(([A,O])=>({x_km:A,y_km:O}))??[],k=("widthMetres"in y?y.widthMetres:void 0)??("width_metres"in y?y.width_metres:void 0)??24;for(let A=0;A+1<N.length;A+=1){const O=Ri(N[A],N[A+1],k,.055);ko.push(O),un.push(O)}}}for(const g of Object.keys(Bi)){const y=Bi[g];if(y.length===0)continue;const M=Nn(y,!1);M&&(M.computeBoundingSphere(),Ps.add(new xt(M,Fr[g])),Fc.push(M))}const Na=ko.length>0?Nn(ko,!1):null;Na&&(Na.computeBoundingSphere(),Zs.add(new xt(Na,Xl)),ko.length=0,ko.push(Na));for(const g of Gc)g.dispose();for(const g of un)g.dispose();Ps.visible=!1,Zs.visible=!1,U.add(Ps,Zs);const il=m0();for(const g of o.modernCities??[]){const y=g.nodes??[];if(y.length===0)continue;let M=1/0,_=-1/0,N=1/0,k=-1/0;for(const Ne of y)M=Math.min(M,Ne.point.x_km),_=Math.max(_,Ne.point.x_km),N=Math.min(N,Ne.point.y_km),k=Math.max(k,Ne.point.y_km);const A=.12;M-=A,_+=A,N-=A,k+=A;const O=18,Y=(M-Ie)*at,ee=(_-Ie)*at,te=(N-Ie)*at,de=(k-Ie)*at,pe=[],fe=[],ye=[];for(let Ne=0;Ne<=O;Ne+=1)for(let Oe=0;Oe<=O;Oe+=1){const ze=Y+(ee-Y)*Oe/O,Ae=te+(de-te)*Ne/O;pe.push(ze,zt(ze,Ae)+.02*T,Ae),fe.push(ze/T/4,Ae/T/4)}for(let Ne=0;Ne<O;Ne+=1)for(let Oe=0;Oe<O;Oe+=1){const ze=Ne*(O+1)+Oe;ye.push(ze,ze+O+1,ze+1,ze+1,ze+O+1,ze+O+2)}const Se=new sa;Se.setAttribute("position",new st(pe,3)),Se.setAttribute("uv",new st(fe,2)),Se.setIndex(ye),Se.computeVertexNormals();const xe=new xt(Se,il);xe.renderOrder=-1,_o.add(xe)}const Pl=new ka,rl=new ka,Zl=new Ke({color:5066316,roughness:.93,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),Qs=new Ke({color:7955527,roughness:1,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),qn=new Dn({color:15197400,polygonOffset:!0,polygonOffsetFactor:-4,polygonOffsetUnits:-4}),Bo=new Dn({color:14200114,polygonOffset:!0,polygonOffsetFactor:-4,polygonOffsetUnits:-4}),Ql=[],cl=o.worldSizeKm*.5,y0=g=>{if(g.length<2)return[];let y=g.map(([N,k])=>[N,k]);for(let N=0;N<2;N+=1){const k=[y[0]];for(let A=0;A+1<y.length;A+=1){const O=y[A],Y=y[A+1];k.push([O[0]*.75+Y[0]*.25,O[1]*.75+Y[1]*.25]),k.push([O[0]*.25+Y[0]*.75,O[1]*.25+Y[1]*.75])}k.push(y[y.length-1]),y=k}const M=[];let _=0;for(let N=0;N+1<y.length;N+=1){const k=y[N],A=y[N+1],O=Math.hypot(A[0]-k[0],A[1]-k[1])*1e3,Y=Math.max(1,Math.ceil(O/80));for(let ee=N===0?0:1;ee<=Y;ee+=1){const te=ee/Y,de=Ge.lerp(k[0],A[0],te),pe=Ge.lerp(k[1],A[1],te),fe=(de-cl)*1e3*T,ye=(pe-cl)*1e3*T;if(M.length>0){const Se=M[M.length-1];_+=Math.hypot(fe-Se.x,ye-Se.z)*F}M.push({x:fe,z:ye,y:zt(fe,ye)+.06*T,distanceMetres:_})}}return M.map((N,k)=>{const A=M[Math.max(0,k-1)],O=M[Math.min(M.length-1,k+1)],Y=Math.max(1e-9,Math.hypot(O.x-A.x,O.z-A.z));return{...N,nx:-(O.z-A.z)/Y,nz:(O.x-A.x)/Y}})},va=(g,y,M=0)=>{const _=[],N=[],k=y*T*.5,A=M*T;for(const Y of g){const ee=Y.x+Y.nx*A,te=Y.z+Y.nz*A;_.push(ee+Y.nx*k,Y.y,te+Y.nz*k,ee-Y.nx*k,Y.y,te-Y.nz*k)}for(let Y=0;Y+1<g.length;Y+=1){const ee=Y*2;N.push(ee,ee+2,ee+1,ee+1,ee+2,ee+3)}const O=new sa;return O.setAttribute("position",new st(_,3)),O.setIndex(N),O.computeVertexNormals(),Ql.push(O),O},Is=(g,y,M)=>{const _=[],N=[],k=y*T*.5;for(let O=0;O+1<g.length;O+=1){const Y=g[O],ee=g[O+1],te=ee.distanceMetres-Y.distanceMetres;let de=Y.distanceMetres;for(;de<ee.distanceMetres;){const pe=de%9,fe=pe<3?de:de+(9-pe),ye=Math.min(ee.distanceMetres,fe+3);if(fe>=ee.distanceMetres||ye<=fe)break;const Se=(fe-Y.distanceMetres)/Math.max(te,1e-6),xe=(ye-Y.distanceMetres)/Math.max(te,1e-6),Ne=_.length/3;for(const Oe of[Se,xe]){const ze=Ge.lerp(Y.nx,ee.nx,Oe),Ae=Ge.lerp(Y.nz,ee.nz,Oe),He=Math.max(1e-9,Math.hypot(ze,Ae)),ke=ze/He,Le=Ae/He,je=Ge.lerp(Y.x,ee.x,Oe)+ke*M*T,De=Ge.lerp(Y.z,ee.z,Oe)+Le*M*T,Be=Ge.lerp(Y.y,ee.y,Oe)+.018*T;_.push(je+ke*k,Be,De+Le*k,je-ke*k,Be,De-Le*k)}N.push(Ne,Ne+2,Ne+1,Ne+1,Ne+2,Ne+3),de=ye+6}}const A=new sa;return A.setAttribute("position",new st(_,3)),A.setIndex(N),A.computeVertexNormals(),Ql.push(A),A};for(const g of o.roads??[]){const y=y0(g.pathKm);if(y.length<2)continue;const M=new xt(va(y,g.profile.carriagewayWidthMetres),g.profile.paved?Zl:Qs);M.renderOrder=3,Pl.add(M);const _=(N,k)=>{const A=new xt(N,k);A.renderOrder=4,rl.add(A)};g.class==="motorway"?(_(Is(y,.15,-g.profile.carriagewayWidthMetres*.25),qn),_(Is(y,.15,g.profile.carriagewayWidthMetres*.25),qn),_(va(y,.14,-.16),Bo),_(va(y,.14,.16),Bo)):g.class==="arterial"?(_(va(y,.12,-.14),Bo),_(va(y,.12,.14),Bo)):g.class==="collector"&&_(Is(y,.12,0),Bo)}Pl.visible=!1,rl.visible=!1,U.add(Pl,rl);const fo=(g,y,M)=>new Ke({color:g,roughness:.92,polygonOffset:!0,polygonOffsetFactor:y,polygonOffsetUnits:y,...M}),Hi={metresToScene:T,worldSizeKm:o.worldSizeKm,terrainHeightAt:zt,layer:fo},ul=L3(o.railways??[],Hi);ul.group.visible=!1,U.add(ul.group);const ho=H3(o.railLines??[],Hi);U.add(ho.group);const Js=o.airport?G3(o.airport,Hi):null;Js&&U.add(Js.group);const Vc=zt(X.target.x,X.target.z);X.target.y=Vc+Te,R instanceof Fo&&(R.position.y=Math.max(R.position.y,X.target.y+Ee)),X.update();let $s=Number.POSITIVE_INFINITY,Li=Number.POSITIVE_INFINITY,qa=0;const fl=new Mo,hl=new Pu,ei=new rt,Il=new rt,Ia=new $t,fn=new rt(0,1,0),Ho=g=>g<=420?8:g<=900?12:g<=1800?20:g<=3600?36:80,Gn=(g,y)=>{const M=Ge.clamp(g/T*.72*y,180,8e3),_=Ho(M),N=Math.hypot(X.target.x-$s,X.target.z-Li)/T,k=Math.abs(M-qa)/Math.max(1,qa);if(N<_*5&&k<.14)return;$s=X.target.x,Li=X.target.z,qa=M;const A=($s+1.6)/T,O=(Li+1.6)/T,Y=Math.floor((A-M)/_),ee=Math.ceil((A+M)/_),te=Math.floor((O-M)/_),de=Math.ceil((O+M)/_);let pe=0;const fe=wa[M<=320?0:1],ye=new Array(fe.length).fill(0);for(let Se=te;Se<=de&&pe<nn;Se+=1)for(let xe=Y;xe<=ee&&pe<nn;xe+=1){const Ne=ut(xe,Se,31),Oe=ut(xe,Se,32),ze=(xe+.08+Ne*.84)*_,Ae=(Se+.08+Oe*.84)*_,He=ze-A,ke=Ae-O;if(He*He+ke*ke>M*M)continue;const Le=-1.6+ze*T,je=-1.6+Ae*T;if(Le<=-1.6||Le>=1.6||je<=-1.6||je>=1.6)continue;const De=Vt(Ha,Le,je)/255;if(Vt(Xa,Le,je)/255>.12)continue;const Ve=Ge.clamp((De-.18)*1.45,0,.96);if(ut(xe,Se,33)>Ve)continue;const qe=Math.max(0,Vt(we,Le,je)-o.stats.minElevation)*T,Qe=.68+ut(xe,Se,34)*.76,it=.72+ut(xe,Se,35)*.58;hl.setFromAxisAngle(fn,ut(xe,Se,36)*Math.PI*2);const $e=Math.floor(ut(xe,Se,39)*fe.length)%fe.length;if(ye[$e]>=nn)continue;Il.set(Le,qe,je),ei.set(it,Qe,it),fl.compose(Il,hl,ei),fe[$e].setMatrixAt(ye[$e],fl);const nt=.8+ut(xe,Se,38)*.36,vt=(ut(xe,Se,37)-.5)*.1;Ia.setRGB(nt*(1+vt),nt,nt*(1-vt)),fe[$e].setColorAt(ye[$e],Ia),ye[$e]+=1,pe+=1}for(const Se of sn)Se.count=0;fe.forEach((Se,xe)=>{Se.count=ye[xe],Se.instanceMatrix.needsUpdate=!0,Se.instanceColor&&(Se.instanceColor.needsUpdate=!0),Se.count>0&&Se.computeBoundingSphere()})},Fn=new f2(14082282,9077108,.24);U.add(Fn);const Tn=Ge.degToRad(l.sunAzimuth),ti=Ge.degToRad(l.sunElevation),Kt=new a1(16773590,1.95);Kt.position.set(Math.sin(Tn)*Math.cos(ti)*5,Math.sin(ti)*5,Math.cos(Tn)*Math.cos(ti)*5),Kt.castShadow=!0,Kt.shadow.mapSize.set(ia?1024:2048,ia?1024:2048),Kt.shadow.camera.near=.001,Kt.shadow.camera.far=12,Kt.shadow.bias=-3e-4,Kt.shadow.normalBias=.5*T,U.add(Kt),U.add(Kt.target);const Jl=Ge.degToRad(l.windDirection),Vr=q1(o.waterHeightDataBase64),Yt=El(o.waterMaskBase64),kt=El(o.waterKindBase64),mt=new Int8Array(El(o.flowDirectionBase64).buffer),Vn=El(o.flowStrengthBase64),Da=O3(T,{hash01:ut,forest:(g,y)=>Vt(Ha,g,y),excluded:(g,y)=>Vt(Xa,g,y),water:(g,y)=>{const M=o.waterDataSize,_=Ge.clamp(Math.round((g+1.6)/3.2*(M-1)),0,M-1),N=Ge.clamp(Math.round((y+1.6)/3.2*(M-1)),0,M-1);return Yt[N*M+_]},height:(g,y)=>{const M=o.meshSize-1,_=Ge.clamp((g+1.6)/3.2*M,0,M-1e-6),N=Ge.clamp((y+1.6)/3.2*M,0,M-1e-6),k=Math.floor(_),A=Math.floor(N),O=_-k,Y=N-A,ee=(ye,Se)=>Math.max(0,we[Se*o.meshSize+ye]-o.stats.minElevation)*T,te=ee(k,A),de=ee(k,A+1),pe=ee(k+1,A+1),fe=ee(k+1,A);return O+Y<=1?te+O*(fe-te)+Y*(de-te):pe+(1-O)*(de-pe)+(1-Y)*(fe-pe)},elevation01:(g,y)=>(Vt(we,g,y)-o.stats.minElevation)/Math.max(1,o.stats.maxElevation-o.stats.minElevation)});U.add(Da.group);const ai=new gc(3.2,3.2,o.meshSize-1,o.meshSize-1),mo=ai.attributes.position;for(let g=0;g<Vr.length;g+=1){const y=Math.max(0,Vr[g]-o.stats.minElevation);mo.setZ(g,y*T+T*.18)}mo.needsUpdate=!0;const $l=new Uint8Array(Yt.length*4),es=new Uint8Array(Yt.length*4),zn=o.waterDataSize,dl=.18*255,ni=new Uint8Array(Yt),ts=new Uint8Array(kt),ji=new Uint8Array(Vn);for(let g=0;g<zn;g+=1)for(let y=0;y<zn;y+=1){const M=g*zn+y;if(Yt[M]===0||kt[M]>dl)continue;const _=Math.round(Math.min(255,Yt[M]*1.5)*.9),N=Math.max(Vn[M],96);for(const[k,A]of[[1,0],[-1,0],[0,1],[0,-1]]){const O=y+k,Y=g+A;if(O<0||Y<0||O>=zn||Y>=zn)continue;const ee=Y*zn+O;Yt[ee]===0&&_>ni[ee]&&(ni[ee]=_,ts[ee]=kt[M],ji[ee]=N)}}for(let g=0;g<Yt.length;g+=1)$l[g*4]=ni[g],$l[g*4+1]=ts[g],$l[g*4+2]=Math.max(ji[g],Vn[g]),$l[g*4+3]=255,es[g*4]=Math.round((mt[g*2]/127*.5+.5)*255),es[g*4+1]=Math.round((mt[g*2+1]/127*.5+.5)*255),es[g*4+2]=128,es[g*4+3]=255;const Ga=new ma($l,o.waterDataSize,o.waterDataSize,$n);Ga.minFilter=ua,Ga.magFilter=ua,Ga.wrapS=fi,Ga.wrapT=fi,Ga.flipY=!0,Ga.needsUpdate=!0;const Ma=new ma(es,o.waterDataSize,o.waterDataSize,$n);Ma.minFilter=ua,Ma.magFilter=ua,Ma.wrapS=fi,Ma.wrapT=fi,Ma.flipY=!0,Ma.needsUpdate=!0;const Kn={uTime:{value:0},uMetresToScene:{value:T},uWind:{value:new Et(Math.cos(Jl),Math.sin(Jl))},uSunDirection:{value:Kt.position.clone().normalize()},uHydrologyMap:{value:Ga},uFlowMap:{value:Ma}},Yn=new Wa({vertexShader:Y3,fragmentShader:W3,uniforms:Kn,transparent:!0,depthWrite:!1,polygonOffset:!0,polygonOffsetFactor:-1,polygonOffsetUnits:-2,side:ct}),as=new xt(ai,Yn);as.rotation.x=-Math.PI/2,as.renderOrder=2,U.add(as);const oi=new Et(Math.cos(Jl),Math.sin(Jl)),Cn={uTime:{value:0},uCoverage:{value:l.cloudCoverage/100},uWind:{value:oi.clone().multiplyScalar(l.cloudSpeed*6e-4)},uSunColor:{value:new $t(1,.91,.76)},uShadowMode:{value:0}},li=new Wa({vertexShader:G1,fragmentShader:F1,uniforms:Cn,transparent:!0,depthWrite:!1,side:ct}),Lo=new gc(4.4,4.4,1,1),na=new xt(Lo,li);na.rotation.x=-Math.PI/2;const si=o.stats.maxElevation-o.stats.minElevation;na.position.y=(si+1800)*T,U.add(na);{const g=new Set((new URLSearchParams(window.location.search).get("hide")??"").split(","));g.has("clouds")&&(na.visible=!1),g.has("water")&&(as.visible=!1),g.has("forest")&&(xa.visible=!1)}const Kc=new URLSearchParams(window.location.search).get("hide")?.split(",").includes("forest")??!1,qi={uTime:{value:0},uCoverage:{value:l.cloudCoverage/100},uWind:{value:oi.clone().multiplyScalar(l.cloudSpeed*6e-4)},uSunColor:{value:new $t(1,1,1)},uShadowMode:{value:1}},Yc=new Wa({vertexShader:G1,fragmentShader:F1,uniforms:qi,transparent:!0,depthWrite:!1,side:ct,blending:h2}),Wc=new gc(3.28,3.28,1,1),Wt=new xt(Wc,Yc);Wt.rotation.x=-Math.PI/2,Wt.position.y=(si+35)*T,Wt.renderOrder=1,Wt.visible=r==="satellite",U.add(Wt);const Gi=()=>{const g=Math.max(1,K.clientWidth),y=Math.max(1,K.clientHeight),M=g/y;R instanceof Fo?R.aspect=M:(R.left=-1.7*M,R.right=1.7*M,R.top=1.7,R.bottom=-1.7),R.updateProjectionMatrix(),V.setSize(g,y,!1),le&&"setSize"in le&&le.setSize(g,y)},ns=new ResizeObserver(Gi);ns.observe(K),Gi();const ii=new d2,Fi=new rt;let Ja=0,ml=0,Kr=0;const hn={applied:!1,settled:0,frames:0,reported:!1,startedAt:performance.now()};let Xc=0,Pc=0,jo=!1;const Vi=()=>{const g=ii.getDelta();Ja+=g;const y=Math.sin(Ja*.035+l.seed*.0017),M=.82+Math.sin(Ja*.21+l.seed*.013)*.18,_=Ge.clamp(l.cloudCoverage/100+y*.11,0,1);Cn.uTime.value+=g,Cn.uCoverage.value=_,Cn.uWind.value.copy(oi).multiplyScalar(l.cloudSpeed*6e-4*M),qi.uTime.value=Cn.uTime.value,qi.uCoverage.value=_,qi.uWind.value.copy(Cn.uWind.value),Kn.uTime.value+=g;const N=Tn+Ja*.0015,k=ti+Math.sin(Ja*.012)*.035;Kt.position.set(Math.sin(N)*Math.cos(k)*5,Math.sin(k)*5,Math.cos(N)*Math.cos(k)*5),Kt.intensity=3.4*(1-_*.28),Kn.uSunDirection.value.copy(Kt.position).normalize(),U.fog.density=z*(1+Math.max(0,y)*.32);const A=x.current,O=C.current;if(O&&O.nonce!==Kr){Kr=O.nonce,dt?.active&&bi(!1);const De=(O.xKm-Ie)*at,Be=(O.yKm-Ie)*at,Ve=zt(De,Be);if(X.target.set(De,Ve+Te,Be),R instanceof Fo){const qe=O.spanKm*1e3*T/(2*Math.tan(Ge.degToRad(R.fov*.5))),Qe=R.position.clone().sub(X.target);if(Qe.lengthSq()<1e-9&&Qe.set(.7,.55,.7),O.pitchDeg!==void 0){const it=Ge.degToRad(Math.min(89.5,O.pitchDeg)),$e=Ge.degToRad(O.azimuthDeg??200);Qe.set(Math.cos(it)*Math.sin($e),Math.sin(it),Math.cos(it)*Math.cos($e))}R.position.copy(X.target).add(Qe.setLength(qe))}else{const qe=R;qe.zoom=3.4/Math.max(1e-6,O.spanKm*1e3*T),qe.updateProjectionMatrix(),R.position.set(De,Ve+5.2,Be+.001)}}if(A?.camera&&!hn.applied&&R instanceof Fo){hn.applied=!0;const De=qe=>{const Qe=A.camera;let it,$e;if(Qe.city!==void 0){const nt=o.modernCities?.[Qe.city]?.frame;if(!nt)return null;const vt=qe[0]*T,Ct=qe[2]*T,Rt=Math.cos(nt.rotationRadians),It=Math.sin(nt.rotationRadians);it=(nt.origin.x_km-Ie)*at+vt*Rt-Ct*It,$e=(nt.origin.y_km-Ie)*at+vt*It+Ct*Rt}else it=(qe[0]-Ie)*at,$e=(qe[2]-Ie)*at;return new rt(it,zt(it,$e)+qe[1]*T,$e)},Be=De(A.camera.pos),Ve=De(A.camera.target);if(Be&&Ve){R.up.set(0,1,0),R.position.copy(Be),R.lookAt(Ve),R.updateMatrixWorld(!0),X.target.copy(Ve);const qe=A.camera.mode??"orbit";qe==="orbit"?X.update():dt&&(dt.active&&dt.exit(),dt.enter(qe==="walk"),X.enabled=!1,Qa())}}if(dt?.active?(dt.update(g),X.target.copy(dt.groundTarget)):X.update(),R instanceof Fo){X.target.x=Ge.clamp(X.target.x,-ce+ue,ce-ue),X.target.z=Ge.clamp(X.target.z,-ce+ue,ce-ue),X.target.y=zt(X.target.x,X.target.z)+Te;const De=zt(R.position.x,R.position.z);R.position.y<De+Ee&&(R.position.y=De+Ee);const Be=R.position.distanceTo(X.target),Ve=Ge.clamp(Be/300,.05*T,40*T),qe=Math.max(0,o.stats.maxElevation-o.stats.minElevation)*T,Qe=Ge.clamp(Be*8+qe*2,2e3*T,30);(Math.abs(R.near-Ve)/Ve>.12||Math.abs(R.far-Qe)/Qe>.12)&&(R.near=Ve,R.far=Qe,R.updateProjectionMatrix())}else X.target.x=Ge.clamp(X.target.x,-ce+ue,ce-ue),X.target.z=Ge.clamp(X.target.z,-ce+ue,ce-ue),X.target.y=zt(X.target.x,X.target.z)+Te;const Y=R instanceof ah?3.4/R.zoom:R.position.distanceTo(X.target)*Math.tan(Ge.degToRad(R.fov*.5))*2;R.getWorldDirection(Fi);const ee=R instanceof Fo?Math.min(3,1/Math.max(.34,Math.abs(Fi.y))):1,te=Y*F,de=Ge.clamp((Math.log10(Math.max(te,1))-Math.log10(2e3))/(Math.log10(2e4)-Math.log10(2e3)),0,1);X.zoomSpeed=1+de*2;const pe=io(te,3200,7200),fe=Ds(te,3800,19e3),ye=pe>.001;for(const De of sn)De.visible=ye&&!Kc;Ot(ln,pe),xa.visible=fe>.001,Ot(to,fe),ye&&Gn(Y,ee);const Se=Y/T*ee,xe=io(Se,380,2450),Ne=io(Se,230,760),Oe=xe>.001,ze=Ne>.001;_a.visible=Oe,Ot(Hl,xe);for(const De of zo)De.visible=ze;for(const De of[To,_s,yi,xi,On,ra,ba])Ot(De,Ne);(Oe||ze)&&zc(Y,ee),Da.setFade(io(Se,320,780)),Da.update(Y,ee,X.target.x,X.target.z);const Ae=io(te,7500,18e3),He=io(te,1800,5500),ke=io(te,9e3,26e3),Le=io(te,650,7500);Pl.visible=Ae>.001,ul.group.visible=Ae>.001,ho.group.visible=Ae<.999,rl.visible=He>.001,js.visible=!1,Fs.visible=!1,_o.visible=!1,Ea.visible=!1,Ls.visible=!1,Ps.visible=!1,Zs.visible=!1,Ot(Zl,Ae),Ot(Qs,Ae),Ot(qn,He),Ot(Bo,He);for(const De of Ns)Ot(De,ke);Ot(il,ke),Ot(Sr,ke);for(const De of Object.values(qs))Ot(De,Le);Ot(ol,Le);for(const De of Object.values(ja))Ot(De,Le);Ot(Br,ke),Ot(Ys,ke),Ot(Ws,ke),Ot(Ra,ke),Ot(ll,ke),Ot(Oi,ke);for(const De of Ks)Ot(De,ke);for(const De of Ui)Ot(De,ke);for(const De of No)Ot(De,ke);for(const De of Object.values(Fr))Ot(De,ke);if(Ot(Xl,ke),ae.uTime.value=Ja,Mn?.update(Ja,R.position.distanceTo(X.target)*F,R)&&(V.shadowMap.needsUpdate=!0),G){const De=te<1500&&!me;G.enabled!==De&&(G.enabled=De)}if(Ai(g),u0(),h0(Ja),rr){if(Mn){const De=Mn.lodStats();Va("lodInstances",De.instances),Va("lodHiddenStatics",De.hiddenStatics)}Va("viewSpanM",te),Va("roam",dt?.active?dt.walking?2:1:0),Va("camXm",R.position.x*F),Va("camYm",R.position.y*F),Va("camZm",R.position.z*F),Va("groundYm",zt(R.position.x,R.position.z)*F)}ge.position.copy(R.position),ge.scale.setScalar(Math.max(1e-6,R.far*.9/9e3)),U.fog.near=Math.max(500*T,Y*1.5),U.fog.far=Math.max(4e3*T,Y*6);const je=Ge.clamp(Y*.85,40*T,1.5);if(Kt.target.position.copy(X.target),Kt.position.copy(X.target).addScaledVector(vc,je*2.5+.05),Kt.shadow.camera.left=-je,Kt.shadow.camera.right=je,Kt.shadow.camera.top=je,Kt.shadow.camera.bottom=-je,Kt.shadow.camera.near=1e-4,Kt.shadow.camera.far=je*6+.2,Kt.shadow.camera.updateProjectionMatrix(),Ca.uSunViewDir.value.copy(vc).transformDirection(R.matrixWorldInverse),Xc+=1,Xc%45===1&&(V.shadowMap.needsUpdate=!0),B.begin(),le&&r==="3d"?le.render():V.render(U,R),B.end(),!jo&&(Pc+=1)>=3&&(jo=!0,lh()),A&&!hn.reported){hn.frames+=1;const De=C.current?.nonce??0,Be=(A.camera?hn.applied:!0)&&Kr===De,Ve=Hs.reduce((it,$e)=>it+($e?$e.pending():0),0);hn.settled=Be&&Ve===0?hn.settled+1:0;const qe=(performance.now()-hn.startedAt)/1e3,Qe=qe>(A.tileTimeoutSec??240);if(hn.settled>=(A.settleFrames??6)||Qe&&Be){hn.reported=!0;const it=Hs.map($e=>$e?$e.summary():null);D.current?.({frames:hn.frames,waitedSec:Number(qe.toFixed(1)),timedOut:Qe,camera:{xm:R.position.x*F,ym:R.position.y*F,zm:R.position.z*F,groundYm:zt(R.position.x,R.position.z)*F,roam:dt?.active?dt.walking?"walk":"fly":"orbit"},streamers:it,wholeCityMounted:!!Mn,render:{calls:V.info.render.calls,triangles:V.info.render.triangles,geometries:V.info.memory.geometries,textures:V.info.memory.textures},webgl2:V.capabilities.isWebGL2,composer:!!(le&&r==="3d")})}}if(rr){const De=window;De.__SNAP__&&(De.__SNAPSHOT__=V.domElement.toDataURL("image/png"),De.__SNAP__=!1)}ml=requestAnimationFrame(Vi)};return Vi(),()=>{cancelAnimationFrame(ml),ns.disconnect(),p.current=null,B.dispose(),window.removeEventListener("keydown",Cc),dt?.dispose(),Za?.root.remove(),Nc(),nl?.dispose(),Mn?.dispose();for(const g of Eo)g&&(U.remove(g.mesh),g.water&&U.remove(g.water),g.dispose());X.dispose(),oe.dispose(),E.dispose(),_e.dispose(),Da.dispose(),qt.dispose();for(const g of on)g.dispose();xn.dispose(),Hl.dispose(),zs.dispose(),$o.dispose(),fr.dispose(),Cs.dispose(),gi.dispose(),hr.dispose(),jl.dispose(),_s.dispose(),yi.dispose(),xi.dispose(),On.dispose(),ra.dispose(),ba.dispose(),To.dispose();for(const g of vr)g.dispose();for(const g of Rs)g.dispose();for(const g of Ns)g.dispose();for(const g of Tr.values())g.map?.dispose(),g.normalMap?.dispose(),g.dispose();for(const g of Gl)g.dispose();Kl.dispose(),Sr.dispose();for(const g of Er)g.dispose();for(const g of Object.values(ja))g.dispose();for(const g of Dr)g.dispose();for(const g of Ks)g.dispose();for(const g of Or)g.dispose();for(const g of Ui)g.dispose();for(const g of kr)g.dispose();for(const g of No)g.dispose();Br.dispose(),Ys.dispose(),Ws.dispose(),Ra.dispose(),ll.dispose(),Oi.dispose();for(const g of Cr)g.dispose();ol.dispose();for(const g of Object.values(qs))g.dispose();for(const g of p0)g.dispose();for(const g of Fc)g.dispose();for(const g of Object.values(Fr))g.dispose();for(const g of ko)g.dispose();Xl.dispose();for(const g of Ql)g.dispose();for(const g of[ul,ho,Js])if(g){for(const y of g.geometries)y.dispose();for(const y of g.materials)y.dispose();U.remove(g.group)}Zl.dispose(),Qs.dispose(),qn.dispose(),Bo.dispose(),ai.dispose(),Yn.dispose(),Ga.dispose(),Ma.dispose(),Lo.dispose(),li.dispose(),Wc.dispose(),Yc.dispose(),V.dispose(),ia&&V.forceContextLoss(),V.domElement.remove()}};return pt.useEffect(()=>{const K=m.current;if(K){q(null),ep();try{return P(K)}catch(U){console.error("[terrain3d] mount failed",U),K.replaceChildren(),lh(),q(U instanceof Error?U.message:String(U));return}}},[o,r,l.cloudCoverage,l.cloudSpeed,l.haze,l.sunAzimuth,l.sunElevation,l.windDirection]),se.jsx("div",{ref:m,className:"terrain-3d",children:H?se.jsxs("div",{className:"mount-error",role:"alert",children:[se.jsx("strong",{children:"3D 视图无法在此设备上启动"}),se.jsx("span",{children:H}),se.jsx("small",{children:"可切换到「动态卫星」或分析图层查看同一片地貌，或在设置中降低网格后重新生成。"})]}):null})}function P3(o,l=!1){return window.__TAURI_INTERNALS__.transformCallback(o,l)}async function yh(o,l={},r){return window.__TAURI_INTERNALS__.invoke(o,l,r)}const Z3=Object.freeze(Object.defineProperty({__proto__:null,invoke:yh,transformCallback:P3},Symbol.toStringTag,{value:"Module"}));async function Q3(){try{return await yh("automation_spec")??null}catch(o){return typeof window<"u"&&"__TAURI_INTERNALS__"in window&&console.error("[automation]",o),null}}function Ul(o,l={}){yh("automation_event",{event:o,detail:l}).catch(()=>{})}function I3(){for(const o of["log","info","warn","error"]){const l=console[o].bind(console);console[o]=(...r)=>{l(...r),Ul("console",{level:o,text:r.map(u=>u instanceof Error?u.stack??u.message:typeof u=="string"?u:JSON.stringify(u)).join(" ").slice(0,2e3)})}}window.addEventListener("error",o=>Ul("console",{level:"uncaught",text:`${o.message} @ ${o.filename}:${o.lineno}`})),window.addEventListener("unhandledrejection",o=>Ul("console",{level:"unhandledrejection",text:String(o.reason)}))}const bs={arid:{name:"干旱高山",description:"裸岩、冲积扇与干涸河谷",rainfall:425,evaporation:975,windSpeed:11,windDirection:35},temperate:{name:"温带湿润",description:"森林、活跃河网与冲积平原",rainfall:1275,evaporation:600,windSpeed:7,windDirection:225},glacial:{name:"高寒冰川",description:"冰斗、雪线与冰川侵蚀谷",rainfall:750,evaporation:250,windSpeed:13,windDirection:285}},eh={mountainRange:{name:"山脉",description:"造山带与深切河谷"},hills:{name:"丘陵",description:"低缓山丘与谷地"},plains:{name:"平原",description:"低起伏与大型河网"},plateau:{name:"高原",description:"台地、陡崖与峡谷"},coastal:{name:"海岸",description:"大陆边缘与海湾"},archipelago:{name:"群岛",description:"岛链、海峡与浅海"}},J3={seed:284735,preset:"temperate",landform:"mountainRange",gridSize:ia?256:512,worldSizeKm:80,rainfall:bs.temperate.rainfall,evaporation:bs.temperate.evaporation,windSpeed:bs.temperate.windSpeed,windDirection:bs.temperate.windDirection,sunAzimuth:235,sunElevation:42,haze:2.5,cloudCoverage:35,cloudSpeed:24},Wu={discharge:"汇流量",lake:"湖泊",wetland:"湿地",floodplain:"洪泛区",basin:"流域",riverOrder:"河流级序",geology:"岩性分区",soilDepth:"土壤深度",landCover:"地表覆盖",travelCost:"通行成本",hazard:"灾害风险",settlementSuitability:"聚落适宜度",agriculturalSuitability:"农业适宜度",urbanLand:"城市占地",cultivatedLand:"耕作带",infrastructure:"聚落与道路"};function Vo({label:o,value:l,min:r,max:u,step:i=1,unit:f="",onChange:h}){return se.jsxs("label",{className:"control",children:[se.jsxs("span",{children:[se.jsx("b",{children:o}),se.jsxs("output",{children:[l,f]})]}),se.jsx("input",{type:"range",value:l,min:r,max:u,step:i,onChange:v=>h(Number(v.target.value))})]})}function $3(){const[o,l]=pt.useState(J3),[r,u]=pt.useState(null),[i,f]=pt.useState(!1),[h,v]=pt.useState("等待生成"),[m,p]=pt.useState(0),[b,S]=pt.useState(4096),[C,x]=pt.useState(null),[D,H]=pt.useState("3d"),[q,P]=pt.useState("discharge"),[K,U]=pt.useState(null),[z,R]=pt.useState(null),V=pt.useRef(null),[J,Q]=pt.useState(null),[ne,B]=pt.useState(null),[ae,I]=pt.useState(0),ge=pt.useRef(!1);pt.useEffect(()=>{Q3().then(oe=>{oe&&(I3(),B(oe),oe.config&&l(_e=>({..._e,...oe.config})),I(1))})},[]),pt.useEffect(()=>{let oe;return Q1("terrain-progress",_e=>{v(_e.payload.stage),p(_e.payload.progress)}).then(_e=>{oe=_e}),()=>oe?.()},[]),pt.useEffect(()=>{const oe=_e=>{!r||_e.ctrlKey||_e.altKey||_e.metaKey||_e.target?.matches("input, select, textarea")||(_e.key.toLowerCase()==="s"&&H("satellite"),_e.key==="3"&&H("3d"))};return window.addEventListener("keydown",oe),()=>window.removeEventListener("keydown",oe)},[r]);const Re=pt.useMemo(()=>(o.worldSizeKm*1e3/o.gridSize).toFixed(1),[o.gridSize,o.worldSizeKm]),le=pt.useMemo(()=>{if(!r?.modernCities?.length)return[];const oe=r.modernCities.map((_e,Ze)=>{let Ye=1/0,Xe=-1/0,tt=1/0,ot=-1/0;for(const ta of _e.nodes)Ye=Math.min(Ye,ta.point.x_km),Xe=Math.max(Xe,ta.point.x_km),tt=Math.min(tt,ta.point.y_km),ot=Math.max(ot,ta.point.y_km);const Ut=(Ye+Xe)/2,fa=(tt+ot)/2;return{label:Ze===0?"中心城（县城）":Ze<4?`镇区 ${Ze}`:`村庄 ${Ze-3}`,xKm:Ut,yKm:fa,spanKm:Ze===0?6:Ze<4?3.5:1.6}});return[{label:"全域视角",xKm:o.worldSizeKm/2,yKm:o.worldSizeKm/2,spanKm:o.worldSizeKm*.55},...oe]},[r,o.worldSizeKm]),[G,j]=pt.useState(null),[me,X]=pt.useState(!1);pt.useEffect(()=>{if(z===null){j(null);return}let oe=!1;return X(!0),ap(z).catch(()=>N2(z)).then(_e=>{oe||j(_e)}).catch(_e=>{oe||x(_e.message)}).finally(()=>{oe||X(!1)}),()=>{oe=!0}},[z]);const we=(oe,_e)=>{U({xKm:oe.xKm,yKm:oe.yKm,spanKm:oe.spanKm,nonce:Date.now()}),_e>=1&&H("3d")},E=(oe,_e)=>{l(Ze=>({...Ze,[oe]:_e}))},Z=oe=>{const _e=bs[oe];l(Ze=>({...Ze,preset:oe,rainfall:_e.rainfall,evaporation:_e.evaporation,windSpeed:_e.windSpeed,windDirection:_e.windDirection}))},T=oe=>E("landform",oe),F=()=>E("seed",Math.floor(Math.random()*2147483647)),ce=async()=>{f(!0),x(null),p(.02),v(ia?`初始化计算核心 · ${Zu}`:"初始化原生计算核心");try{const oe=await A2(o);u(oe),H("3d"),R(null),p(1),v(`完成 · ${(oe.elapsedMs/1e3).toFixed(2)} 秒${ia?` · ${Zu}`:""}`),Q(null)}catch(oe){x(String(oe)),v("生成失败")}finally{f(!1)}};pt.useEffect(()=>{ae===1&&(I(2),Ul("generating",{config:o}),ce().then(()=>Ul("generated",{})))},[ae]),pt.useEffect(()=>{if(!ne||!r||ge.current)return;ge.current=!0,ne.view==="satellite"?H("satellite"):ne.view==="analysis"&&(P(ne.layer??"discharge"),H("analysis"));const oe=ne.focus;if(oe){const _e=le[oe.option];_e&&U({xKm:_e.xKm,yKm:_e.yKm,spanKm:oe.spanKm??_e.spanKm,nonce:Date.now(),pitchDeg:oe.pitchDeg,azimuthDeg:oe.azimuthDeg})}(ne.view==="satellite"||ne.view==="analysis")&&window.setTimeout(()=>Ul("ready",{view:ne.view}),4e3)},[ne,r]),pt.useEffect(()=>{C&&ne&&Ul("error",{error:C})},[C,ne]);const ue=async()=>{const oe=await oh({title:"导出卫星正射影像",defaultPath:`wind-water-${o.preset}-${o.seed}-${b}.png`,filters:[{name:"PNG image",extensions:["png"]}]});if(oe){f(!0),x(null),v(`渲染 ${b} × ${b} 影像`),p(.02);try{await D2(o,oe,b),p(1),v(`已导出 · ${oe}`)}catch(_e){x(String(_e)),v("导出失败")}finally{f(!1)}}},Ee=async()=>{const oe=await oh({title:"保存地貌项目",defaultPath:`wind-water-${o.seed}.fwl`,filters:[{name:"Wind & Water project",extensions:["fwl"]}]});if(oe)try{await U2({schemaVersion:1,name:`Terrain ${o.seed}`,config:o},oe),v(`项目已保存 · ${oe}`),x(null)}catch(_e){x(String(_e))}},Te=async()=>{const oe=await J1({title:"打开地貌项目",multiple:!1,directory:!1,filters:[{name:"Wind & Water project",extensions:["fwl"]}]});if(oe)try{const _e=await O2(oe);l(_e.config),u(null),v(`已打开 · ${_e.name}`),x(null)}catch(_e){x(String(_e))}},Ue=async()=>{V.current&&(document.fullscreenElement?await document.exitFullscreen():await V.current.requestFullscreen())};return se.jsxs("main",{className:"app-shell",children:[se.jsxs("header",{className:"topbar",children:[se.jsxs("div",{className:"brand",children:[se.jsx("span",{className:"brand-mark",children:"W/W"}),se.jsxs("div",{children:[se.jsx("h1",{children:"风与水"}),se.jsx("p",{children:"无限地貌实验室 · Desktop Alpha"})]})]}),se.jsxs("div",{className:"top-actions",children:[se.jsxs("span",{className:"engine-badge",children:[se.jsx("i",{})," ",Bl()?"RUST NATIVE":"RUST · WASM"]}),se.jsx("button",{className:"ghost",onClick:Te,disabled:i,children:"打开"}),se.jsx("button",{className:"ghost",onClick:Ee,disabled:i,children:"保存"}),se.jsx("button",{className:"ghost",onClick:F,disabled:i,children:"随机种子"}),se.jsx("button",{className:"sheet-toggle",onClick:()=>Q(J==="left"?null:"left"),children:"参数"}),se.jsx("button",{className:"sheet-toggle",onClick:()=>Q(J==="right"?null:"right"),children:"光照"}),se.jsx("button",{className:"primary",onClick:ce,disabled:i,children:i?"计算中…":"生成地貌"})]})]}),se.jsxs("section",{className:"workspace",children:[se.jsxs("aside",{className:`sidebar left-panel${J==="left"?" sheet-open":""}`,children:[se.jsxs("section",{children:[se.jsxs("div",{className:"section-title",children:[se.jsx("span",{children:"地貌骨架"}),se.jsx("em",{children:"01"})]}),se.jsx("div",{className:"landform-grid",children:Object.keys(eh).map(oe=>se.jsxs("button",{className:`landform-card ${o.landform===oe?"active":""}`,onClick:()=>T(oe),disabled:i,children:[se.jsx("strong",{children:eh[oe].name}),se.jsx("small",{children:eh[oe].description})]},oe))})]}),se.jsxs("section",{children:[se.jsxs("div",{className:"section-title",children:[se.jsx("span",{children:"气候环境"}),se.jsx("em",{children:"02"})]}),se.jsx("div",{className:"preset-grid",children:Object.keys(bs).map(oe=>se.jsxs("button",{className:`preset-card ${o.preset===oe?"active":""}`,onClick:()=>Z(oe),disabled:i,children:[se.jsx("strong",{children:bs[oe].name}),se.jsx("small",{children:bs[oe].description})]},oe))})]}),se.jsxs("section",{children:[se.jsxs("div",{className:"section-title",children:[se.jsx("span",{children:"计算域"}),se.jsx("em",{children:"03"})]}),se.jsxs("label",{className:"select-control",children:[se.jsx("span",{children:"模拟网格"}),se.jsxs("select",{value:o.gridSize,onChange:oe=>E("gridSize",Number(oe.target.value)),disabled:i,children:[ia?se.jsx("option",{value:256,children:"256² · 手机"}):null,se.jsx("option",{value:512,children:"512² · 快速"}),ia?null:se.jsxs(se.Fragment,{children:[se.jsx("option",{value:1024,children:"1024² · 精细"}),se.jsx("option",{value:2048,children:"2048² · 专业"})]})]})]}),se.jsx(Vo,{label:"区域尺度",value:o.worldSizeKm,min:20,max:160,step:10,unit:" km",onChange:oe=>E("worldSizeKm",oe)}),se.jsxs("div",{className:"domain-note",children:[se.jsx("span",{children:"物理分辨率"}),se.jsxs("b",{children:[Re," m/cell"]})]}),se.jsxs("label",{className:"seed-input",children:[se.jsx("span",{children:"随机种子"}),se.jsx("input",{type:"number",value:o.seed,onChange:oe=>E("seed",Number(oe.target.value)),disabled:i})]})]}),se.jsxs("section",{children:[se.jsxs("div",{className:"section-title",children:[se.jsx("span",{children:"水文与气候"}),se.jsx("em",{children:"04"})]}),se.jsx(Vo,{label:"年降水",value:o.rainfall,min:100,max:2e3,step:25,unit:" mm",onChange:oe=>E("rainfall",oe)}),se.jsx(Vo,{label:"蒸散量",value:o.evaporation,min:0,max:1600,step:25,unit:" mm",onChange:oe=>E("evaporation",oe)}),se.jsx(Vo,{label:"风速",value:o.windSpeed,min:0,max:30,step:.5,unit:" m/s",onChange:oe=>E("windSpeed",oe)}),se.jsx(Vo,{label:"风向",value:o.windDirection,min:0,max:359,unit:"°",onChange:oe=>E("windDirection",oe)})]})]}),se.jsxs("section",{className:"viewport",ref:V,children:[r?D==="analysis"?se.jsx("img",{className:"terrain-preview",src:r.analysisPreviews[q],alt:`${Wu[q]}分析图层`}):se.jsx(X3,{result:r,config:o,cameraMode:D,cityFocus:K,cityScene:G,shot:ne&&(!ne.focus||K)?ne:null,onShotReady:oe=>{Ul("ready",oe),ne?.exitAfterReadyMs!==void 0&&window.setTimeout(()=>Ul("exit"),ne.exitAfterReadyMs)},onNeedCity:oe=>{R(oe===null?null:_e=>_e===oe?_e:oe)}}):se.jsxs("div",{className:"empty-state",children:[se.jsx("div",{className:"contour-art"}),se.jsx("span",{children:"80 × 80 KM SYNTHETIC EARTH"}),se.jsx("h2",{children:"创造一片不存在，却足够真实的土地"}),se.jsx("p",{children:"原生 Rust 地貌演化 · 河网推演 · 多尺度卫星辐射合成"}),se.jsx("button",{className:"primary large",onClick:ce,children:"生成第一片地貌"})]}),r&&se.jsxs("div",{className:"view-switcher",children:[se.jsxs(se.Fragment,{children:[se.jsx("button",{className:D==="3d"?"active":"",onClick:()=>H("3d"),children:"3D 地形"}),se.jsx("button",{className:D==="satellite"?"active":"",onClick:()=>H("satellite"),children:"动态卫星"}),se.jsxs("select",{className:D==="analysis"?"active":"",value:q,onChange:oe=>{P(oe.target.value),H("analysis")},children:[se.jsx("option",{disabled:!0,children:"分析图层"}),Object.keys(Wu).map(oe=>se.jsx("option",{value:oe,children:Wu[oe]},oe))]}),se.jsxs("select",{value:"",className:"city-jump",onChange:oe=>{const _e=Number(oe.target.value),Ze=le[_e];Ze&&we(Ze,_e)},children:[se.jsx("option",{disabled:!0,value:"",children:"跳转城市"}),le.map((oe,_e)=>se.jsx("option",{value:_e,children:oe.label},oe.label))]})]}),se.jsx("button",{onClick:Ue,children:"全屏"})]}),ia&&se.jsxs("div",{className:"light-notice",children:[Zu,xs?" · 安全模式":""]}),se.jsxs("div",{className:"viewport-meta",children:[se.jsx("span",{children:D==="3d"?"3D TERRAIN · VERTICAL 1:1 · LIVE WEATHER":D==="analysis"?`SEMANTIC LAYER · ${Wu[q]}`:"LIVE ORTHOGRAPHIC · SAME WORLD · SAME TIME"}),se.jsx("span",{children:me?"正在加载城市细节…":G&&z!==null?`${le[z+1]?.label??"城市"} · 米制细节已载入`:`${o.worldSizeKm} KM / ${o.gridSize} PX`})]}),i&&se.jsxs("div",{className:"progress-overlay",children:[se.jsxs("div",{children:[se.jsx("span",{children:h}),se.jsxs("b",{children:[Math.round(m*100),"%"]})]}),se.jsx("progress",{value:m,max:1})]})]}),se.jsxs("aside",{className:`sidebar right-panel${J==="right"?" sheet-open":""}`,children:[se.jsxs("section",{children:[se.jsxs("div",{className:"section-title",children:[se.jsx("span",{children:"光照与大气"}),se.jsx("em",{children:"04"})]}),se.jsx(Vo,{label:"太阳方位",value:o.sunAzimuth,min:0,max:359,unit:"°",onChange:oe=>E("sunAzimuth",oe)}),se.jsx(Vo,{label:"太阳高度",value:o.sunElevation,min:5,max:80,unit:"°",onChange:oe=>E("sunElevation",oe)}),se.jsx(Vo,{label:"大气霾度",value:o.haze,min:0,max:10,step:.5,onChange:oe=>E("haze",oe)}),se.jsx(Vo,{label:"云层覆盖",value:o.cloudCoverage,min:0,max:100,unit:"%",onChange:oe=>E("cloudCoverage",oe)}),se.jsx(Vo,{label:"云层速度",value:o.cloudSpeed,min:0,max:90,onChange:oe=>E("cloudSpeed",oe)})]}),se.jsxs("section",{children:[se.jsxs("div",{className:"section-title",children:[se.jsx("span",{children:"区域诊断"}),se.jsx("em",{children:"05"})]}),r?se.jsxs("div",{className:"stats-grid",children:[se.jsxs("div",{children:[se.jsx("span",{children:"高程范围"}),se.jsxs("b",{children:[r.stats.minElevation.toFixed(0),"–",r.stats.maxElevation.toFixed(0)," m"]})]}),se.jsxs("div",{children:[se.jsx("span",{children:"平均高程"}),se.jsxs("b",{children:[r.stats.meanElevation.toFixed(0)," m"]})]}),se.jsxs("div",{children:[se.jsx("span",{children:"平均坡度"}),se.jsxs("b",{children:[r.stats.meanSlope.toFixed(1),"°"]})]}),se.jsxs("div",{children:[se.jsx("span",{children:"森林覆盖"}),se.jsxs("b",{children:[(r.stats.forestCoverage*100).toFixed(1),"%"]})]}),se.jsxs("div",{children:[se.jsx("span",{children:"水体覆盖"}),se.jsxs("b",{children:[(r.stats.waterCoverage*100).toFixed(1),"%"]})]}),se.jsxs("div",{children:[se.jsx("span",{children:"积雪覆盖"}),se.jsxs("b",{children:[(r.stats.snowCoverage*100).toFixed(1),"%"]})]}),se.jsxs("div",{children:[se.jsx("span",{children:"候选聚落"}),se.jsx("b",{children:r.infrastructureSummary.settlements})]}),se.jsxs("div",{children:[se.jsx("span",{children:"道路连接"}),se.jsx("b",{children:r.infrastructureSummary.roads})]}),se.jsxs("div",{children:[se.jsx("span",{children:"桥梁 / 隧道"}),se.jsxs("b",{children:[r.infrastructureSummary.bridges," / ",r.infrastructureSummary.tunnels]})]})]}):se.jsx("p",{className:"muted-copy",children:"生成后显示地形、水文与地表覆盖统计。"})]}),se.jsxs("section",{className:"export-section",children:[se.jsxs("div",{className:"section-title",children:[se.jsx("span",{children:"卫星影像导出"}),se.jsx("em",{children:"06"})]}),se.jsxs("label",{className:"select-control",children:[se.jsx("span",{children:"输出尺寸"}),se.jsxs("select",{value:b,onChange:oe=>S(Number(oe.target.value)),disabled:i,children:[se.jsx("option",{value:2048,children:"2048 × 2048"}),se.jsx("option",{value:4096,children:"4096 × 4096"}),se.jsx("option",{value:8192,children:"8192 × 8192"})]})]}),se.jsxs("button",{className:"export-button",onClick:ue,disabled:i||!r,children:[se.jsx("span",{children:"导出 PNG"}),se.jsx("small",{children:"正射自然色 · 无水印"})]}),se.jsx("p",{className:"fine-print",children:"采用原生多线程重采样与亚像素材质细节，高分辨率导出不会阻塞界面。"})]})]})]}),se.jsxs("footer",{className:"statusbar",children:[se.jsxs("span",{className:C?"error":"",children:[C??h,xs?" · 上次 3D 视图被系统终止，已启用安全模式（?safe=0 恢复）":""]}),se.jsx("span",{children:"CORE 0.1 · LOCAL ONLY · NO TELEMETRY"})]})]})}S2.createRoot(document.getElementById("root")).render(se.jsx(pt.StrictMode,{children:se.jsx($3,{})}));export{yh as i,P3 as t};
