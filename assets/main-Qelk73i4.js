const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["./event-CJlvrBzi.js","./three-BNTycAmW.js","./index-Bl-Um1AN.js"])))=>i.map(i=>d[i]);
import{B as _h,V as lt,C as Zt,M as Co,P as Bc,I as Va,a as Ye,D as fa,R as $n,F as sp,N as Ks,b as ct,c as d1,d as m1,e as zt,G as _a,S as Iv,f as ql,g as pt,h as It,i as ft,j as ip,k as wu,l as Dn,m as Jt,n as Dc,o as zh,p as mu,q as za,r as So,L as al,s as la,t as Gs,O as rp,u as Jv,v as Pa,A as $v,w as eg,x as vh,U as ll,W as Vs,H as rl,y as Pl,T as tg,z as ag,E as W0,Z as X0,J as p1,K as v1,Q as ng,X as og,Y as lg,_ as sg,$ as gh,a0 as ig,a1 as rg,a2 as cg,a3 as ug,a4 as fg,a5 as hg,a6 as cp,a7 as dg,a8 as mg,a9 as pg,aa as vg,ab as g1,ac as Ah,ad as Ve,ae as gg,af as up,ag as fp,ah as hp,ai as yg,aj as xg,ak as wg,al as bg,am as Mg,an as su,ao as iu,ap as Sg,aq as Tg,ar as $o,as as Cg,at as _g,au as y1,av as zg,aw as Ag,ax as Eg,ay as x1,az as Ti,aA as Ar,aB as Er,aC as w1,aD as Rg,aE as Ng,aF as Dg,aG as kg,aH as Ug,aI as Og,aJ as Bg}from"./three-BNTycAmW.js";(function(){const l=document.createElement("link").relList;if(l&&l.supports&&l.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))c(i);new MutationObserver(i=>{for(const f of i)if(f.type==="childList")for(const h of f.addedNodes)h.tagName==="LINK"&&h.rel==="modulepreload"&&c(h)}).observe(document,{childList:!0,subtree:!0});function r(i){const f={};return i.integrity&&(f.integrity=i.integrity),i.referrerPolicy&&(f.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?f.credentials="include":i.crossOrigin==="anonymous"?f.credentials="omit":f.credentials="same-origin",f}function c(i){if(i.ep)return;i.ep=!0;const f=r(i);fetch(i.href,f)}})();var Xf={exports:{}},kc={};/**
 * @license React
 * react-jsx-runtime.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var b1;function Hg(){if(b1)return kc;b1=1;var o=Symbol.for("react.transitional.element"),l=Symbol.for("react.fragment");function r(c,i,f){var h=null;if(f!==void 0&&(h=""+f),i.key!==void 0&&(h=""+i.key),"key"in i){f={};for(var g in i)g!=="key"&&(f[g]=i[g])}else f=i;return i=f.ref,{$$typeof:o,type:c,key:h,ref:i!==void 0?i:null,props:f}}return kc.Fragment=l,kc.jsx=r,kc.jsxs=r,kc}var M1;function Lg(){return M1||(M1=1,Xf.exports=Hg()),Xf.exports}var re=Lg(),Zf={exports:{}},ut={};/**
 * @license React
 * react.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var S1;function jg(){if(S1)return ut;S1=1;var o=Symbol.for("react.transitional.element"),l=Symbol.for("react.portal"),r=Symbol.for("react.fragment"),c=Symbol.for("react.strict_mode"),i=Symbol.for("react.profiler"),f=Symbol.for("react.consumer"),h=Symbol.for("react.context"),g=Symbol.for("react.forward_ref"),m=Symbol.for("react.suspense"),d=Symbol.for("react.memo"),x=Symbol.for("react.lazy"),M=Symbol.for("react.activity"),T=Symbol.iterator;function A(S){return S===null||typeof S!="object"?null:(S=T&&S[T]||S["@@iterator"],typeof S=="function"?S:null)}var R={isMounted:function(){return!1},enqueueForceUpdate:function(){},enqueueReplaceState:function(){},enqueueSetState:function(){}},B=Object.assign,C={};function H(S,Q,ve){this.props=S,this.context=Q,this.refs=C,this.updater=ve||R}H.prototype.isReactComponent={},H.prototype.setState=function(S,Q){if(typeof S!="object"&&typeof S!="function"&&S!=null)throw Error("takes an object of state variables to update or a function which returns an object of state variables.");this.updater.enqueueSetState(this,S,Q,"setState")},H.prototype.forceUpdate=function(S){this.updater.enqueueForceUpdate(this,S,"forceUpdate")};function K(){}K.prototype=H.prototype;function U(S,Q,ve){this.props=S,this.context=Q,this.refs=C,this.updater=ve||R}var _=U.prototype=new K;_.constructor=U,B(_,H.prototype),_.isPureReactComponent=!0;var I=Array.isArray;function Z(){}var le={H:null,A:null,T:null,S:null},X=Object.prototype.hasOwnProperty;function se(S,Q,ve){var V=ve.ref;return{$$typeof:o,type:S,key:Q,ref:V!==void 0?V:null,props:ve}}function we(S,Q){return se(S.type,Q,S.props)}function ge(S){return typeof S=="object"&&S!==null&&S.$$typeof===o}function G(S){var Q={"=":"=0",":":"=2"};return"$"+S.replace(/[=:]/g,function(ve){return Q[ve]})}var F=/\/+/g;function ie(S,Q){return typeof S=="object"&&S!==null&&S.key!=null?G(""+S.key):Q.toString(36)}function $(S){switch(S.status){case"fulfilled":return S.value;case"rejected":throw S.reason;default:switch(typeof S.status=="string"?S.then(Z,Z):(S.status="pending",S.then(function(Q){S.status==="pending"&&(S.status="fulfilled",S.value=Q)},function(Q){S.status==="pending"&&(S.status="rejected",S.reason=Q)})),S.status){case"fulfilled":return S.value;case"rejected":throw S.reason}}throw S}function N(S,Q,ve,V,pe){var ue=typeof S;(ue==="undefined"||ue==="boolean")&&(S=null);var Ce=!1;if(S===null)Ce=!0;else switch(ue){case"bigint":case"string":case"number":Ce=!0;break;case"object":switch(S.$$typeof){case o:case l:Ce=!0;break;case x:return Ce=S._init,N(Ce(S._payload),Q,ve,V,pe)}}if(Ce)return pe=pe(S),Ce=V===""?"."+ie(S,0):V,I(pe)?(ve="",Ce!=null&&(ve=Ce.replace(F,"$&/")+"/"),N(pe,Q,ve,"",function(J){return J})):pe!=null&&(ge(pe)&&(pe=we(pe,ve+(pe.key==null||S&&S.key===pe.key?"":(""+pe.key).replace(F,"$&/")+"/")+Ce)),Q.push(pe)),1;Ce=0;var Ne=V===""?".":V+":";if(I(S))for(var oe=0;oe<S.length;oe++)V=S[oe],ue=Ne+ie(V,oe),Ce+=N(V,Q,ve,ue,pe);else if(oe=A(S),typeof oe=="function")for(S=oe.call(S),oe=0;!(V=S.next()).done;)V=V.value,ue=Ne+ie(V,oe++),Ce+=N(V,Q,ve,ue,pe);else if(ue==="object"){if(typeof S.then=="function")return N($(S),Q,ve,V,pe);throw Q=String(S),Error("Objects are not valid as a React child (found: "+(Q==="[object Object]"?"object with keys {"+Object.keys(S).join(", ")+"}":Q)+"). If you meant to render a collection of children, use an array instead.")}return Ce}function P(S,Q,ve){if(S==null)return S;var V=[],pe=0;return N(S,V,"","",function(ue){return Q.call(ve,ue,pe++)}),V}function k(S){if(S._status===-1){var Q=S._result;Q=Q(),Q.then(function(ve){(S._status===0||S._status===-1)&&(S._status=1,S._result=ve)},function(ve){(S._status===0||S._status===-1)&&(S._status=2,S._result=ve)}),S._status===-1&&(S._status=0,S._result=Q)}if(S._status===1)return S._result.default;throw S._result}var ee=typeof reportError=="function"?reportError:function(S){if(typeof window=="object"&&typeof window.ErrorEvent=="function"){var Q=new window.ErrorEvent("error",{bubbles:!0,cancelable:!0,message:typeof S=="object"&&S!==null&&typeof S.message=="string"?String(S.message):String(S),error:S});if(!window.dispatchEvent(Q))return}else if(typeof process=="object"&&typeof process.emit=="function"){process.emit("uncaughtException",S);return}console.error(S)},Me={map:P,forEach:function(S,Q,ve){P(S,function(){Q.apply(this,arguments)},ve)},count:function(S){var Q=0;return P(S,function(){Q++}),Q},toArray:function(S){return P(S,function(Q){return Q})||[]},only:function(S){if(!ge(S))throw Error("React.Children.only expected to receive a single React element child.");return S}};return ut.Activity=M,ut.Children=Me,ut.Component=H,ut.Fragment=r,ut.Profiler=i,ut.PureComponent=U,ut.StrictMode=c,ut.Suspense=m,ut.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE=le,ut.__COMPILER_RUNTIME={__proto__:null,c:function(S){return le.H.useMemoCache(S)}},ut.cache=function(S){return function(){return S.apply(null,arguments)}},ut.cacheSignal=function(){return null},ut.cloneElement=function(S,Q,ve){if(S==null)throw Error("The argument must be a React element, but you passed "+S+".");var V=B({},S.props),pe=S.key;if(Q!=null)for(ue in Q.key!==void 0&&(pe=""+Q.key),Q)!X.call(Q,ue)||ue==="key"||ue==="__self"||ue==="__source"||ue==="ref"&&Q.ref===void 0||(V[ue]=Q[ue]);var ue=arguments.length-2;if(ue===1)V.children=ve;else if(1<ue){for(var Ce=Array(ue),Ne=0;Ne<ue;Ne++)Ce[Ne]=arguments[Ne+2];V.children=Ce}return se(S.type,pe,V)},ut.createContext=function(S){return S={$$typeof:h,_currentValue:S,_currentValue2:S,_threadCount:0,Provider:null,Consumer:null},S.Provider=S,S.Consumer={$$typeof:f,_context:S},S},ut.createElement=function(S,Q,ve){var V,pe={},ue=null;if(Q!=null)for(V in Q.key!==void 0&&(ue=""+Q.key),Q)X.call(Q,V)&&V!=="key"&&V!=="__self"&&V!=="__source"&&(pe[V]=Q[V]);var Ce=arguments.length-2;if(Ce===1)pe.children=ve;else if(1<Ce){for(var Ne=Array(Ce),oe=0;oe<Ce;oe++)Ne[oe]=arguments[oe+2];pe.children=Ne}if(S&&S.defaultProps)for(V in Ce=S.defaultProps,Ce)pe[V]===void 0&&(pe[V]=Ce[V]);return se(S,ue,pe)},ut.createRef=function(){return{current:null}},ut.forwardRef=function(S){return{$$typeof:g,render:S}},ut.isValidElement=ge,ut.lazy=function(S){return{$$typeof:x,_payload:{_status:-1,_result:S},_init:k}},ut.memo=function(S,Q){return{$$typeof:d,type:S,compare:Q===void 0?null:Q}},ut.startTransition=function(S){var Q=le.T,ve={};le.T=ve;try{var V=S(),pe=le.S;pe!==null&&pe(ve,V),typeof V=="object"&&V!==null&&typeof V.then=="function"&&V.then(Z,ee)}catch(ue){ee(ue)}finally{Q!==null&&ve.types!==null&&(Q.types=ve.types),le.T=Q}},ut.unstable_useCacheRefresh=function(){return le.H.useCacheRefresh()},ut.use=function(S){return le.H.use(S)},ut.useActionState=function(S,Q,ve){return le.H.useActionState(S,Q,ve)},ut.useCallback=function(S,Q){return le.H.useCallback(S,Q)},ut.useContext=function(S){return le.H.useContext(S)},ut.useDebugValue=function(){},ut.useDeferredValue=function(S,Q){return le.H.useDeferredValue(S,Q)},ut.useEffect=function(S,Q){return le.H.useEffect(S,Q)},ut.useEffectEvent=function(S){return le.H.useEffectEvent(S)},ut.useId=function(){return le.H.useId()},ut.useImperativeHandle=function(S,Q,ve){return le.H.useImperativeHandle(S,Q,ve)},ut.useInsertionEffect=function(S,Q){return le.H.useInsertionEffect(S,Q)},ut.useLayoutEffect=function(S,Q){return le.H.useLayoutEffect(S,Q)},ut.useMemo=function(S,Q){return le.H.useMemo(S,Q)},ut.useOptimistic=function(S,Q){return le.H.useOptimistic(S,Q)},ut.useReducer=function(S,Q,ve){return le.H.useReducer(S,Q,ve)},ut.useRef=function(S){return le.H.useRef(S)},ut.useState=function(S){return le.H.useState(S)},ut.useSyncExternalStore=function(S,Q,ve){return le.H.useSyncExternalStore(S,Q,ve)},ut.useTransition=function(){return le.H.useTransition()},ut.version="19.2.8",ut}var T1;function Eh(){return T1||(T1=1,Zf.exports=jg()),Zf.exports}var mt=Eh(),Qf={exports:{}},Uc={},If={exports:{}},Jf={};/**
 * @license React
 * scheduler.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var C1;function Fg(){return C1||(C1=1,(function(o){function l(N,P){var k=N.length;N.push(P);e:for(;0<k;){var ee=k-1>>>1,Me=N[ee];if(0<i(Me,P))N[ee]=P,N[k]=Me,k=ee;else break e}}function r(N){return N.length===0?null:N[0]}function c(N){if(N.length===0)return null;var P=N[0],k=N.pop();if(k!==P){N[0]=k;e:for(var ee=0,Me=N.length,S=Me>>>1;ee<S;){var Q=2*(ee+1)-1,ve=N[Q],V=Q+1,pe=N[V];if(0>i(ve,k))V<Me&&0>i(pe,ve)?(N[ee]=pe,N[V]=k,ee=V):(N[ee]=ve,N[Q]=k,ee=Q);else if(V<Me&&0>i(pe,k))N[ee]=pe,N[V]=k,ee=V;else break e}}return P}function i(N,P){var k=N.sortIndex-P.sortIndex;return k!==0?k:N.id-P.id}if(o.unstable_now=void 0,typeof performance=="object"&&typeof performance.now=="function"){var f=performance;o.unstable_now=function(){return f.now()}}else{var h=Date,g=h.now();o.unstable_now=function(){return h.now()-g}}var m=[],d=[],x=1,M=null,T=3,A=!1,R=!1,B=!1,C=!1,H=typeof setTimeout=="function"?setTimeout:null,K=typeof clearTimeout=="function"?clearTimeout:null,U=typeof setImmediate<"u"?setImmediate:null;function _(N){for(var P=r(d);P!==null;){if(P.callback===null)c(d);else if(P.startTime<=N)c(d),P.sortIndex=P.expirationTime,l(m,P);else break;P=r(d)}}function I(N){if(B=!1,_(N),!R)if(r(m)!==null)R=!0,Z||(Z=!0,G());else{var P=r(d);P!==null&&$(I,P.startTime-N)}}var Z=!1,le=-1,X=5,se=-1;function we(){return C?!0:!(o.unstable_now()-se<X)}function ge(){if(C=!1,Z){var N=o.unstable_now();se=N;var P=!0;try{e:{R=!1,B&&(B=!1,K(le),le=-1),A=!0;var k=T;try{t:{for(_(N),M=r(m);M!==null&&!(M.expirationTime>N&&we());){var ee=M.callback;if(typeof ee=="function"){M.callback=null,T=M.priorityLevel;var Me=ee(M.expirationTime<=N);if(N=o.unstable_now(),typeof Me=="function"){M.callback=Me,_(N),P=!0;break t}M===r(m)&&c(m),_(N)}else c(m);M=r(m)}if(M!==null)P=!0;else{var S=r(d);S!==null&&$(I,S.startTime-N),P=!1}}break e}finally{M=null,T=k,A=!1}P=void 0}}finally{P?G():Z=!1}}}var G;if(typeof U=="function")G=function(){U(ge)};else if(typeof MessageChannel<"u"){var F=new MessageChannel,ie=F.port2;F.port1.onmessage=ge,G=function(){ie.postMessage(null)}}else G=function(){H(ge,0)};function $(N,P){le=H(function(){N(o.unstable_now())},P)}o.unstable_IdlePriority=5,o.unstable_ImmediatePriority=1,o.unstable_LowPriority=4,o.unstable_NormalPriority=3,o.unstable_Profiling=null,o.unstable_UserBlockingPriority=2,o.unstable_cancelCallback=function(N){N.callback=null},o.unstable_forceFrameRate=function(N){0>N||125<N?console.error("forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported"):X=0<N?Math.floor(1e3/N):5},o.unstable_getCurrentPriorityLevel=function(){return T},o.unstable_next=function(N){switch(T){case 1:case 2:case 3:var P=3;break;default:P=T}var k=T;T=P;try{return N()}finally{T=k}},o.unstable_requestPaint=function(){C=!0},o.unstable_runWithPriority=function(N,P){switch(N){case 1:case 2:case 3:case 4:case 5:break;default:N=3}var k=T;T=N;try{return P()}finally{T=k}},o.unstable_scheduleCallback=function(N,P,k){var ee=o.unstable_now();switch(typeof k=="object"&&k!==null?(k=k.delay,k=typeof k=="number"&&0<k?ee+k:ee):k=ee,N){case 1:var Me=-1;break;case 2:Me=250;break;case 5:Me=1073741823;break;case 4:Me=1e4;break;default:Me=5e3}return Me=k+Me,N={id:x++,callback:P,priorityLevel:N,startTime:k,expirationTime:Me,sortIndex:-1},k>ee?(N.sortIndex=k,l(d,N),r(m)===null&&N===r(d)&&(B?(K(le),le=-1):B=!0,$(I,k-ee))):(N.sortIndex=Me,l(m,N),R||A||(R=!0,Z||(Z=!0,G()))),N},o.unstable_shouldYield=we,o.unstable_wrapCallback=function(N){var P=T;return function(){var k=T;T=P;try{return N.apply(this,arguments)}finally{T=k}}}})(Jf)),Jf}var _1;function Gg(){return _1||(_1=1,If.exports=Fg()),If.exports}var $f={exports:{}},Fa={};/**
 * @license React
 * react-dom.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var z1;function qg(){if(z1)return Fa;z1=1;var o=Eh();function l(m){var d="https://react.dev/errors/"+m;if(1<arguments.length){d+="?args[]="+encodeURIComponent(arguments[1]);for(var x=2;x<arguments.length;x++)d+="&args[]="+encodeURIComponent(arguments[x])}return"Minified React error #"+m+"; visit "+d+" for the full message or use the non-minified dev environment for full errors and additional helpful warnings."}function r(){}var c={d:{f:r,r:function(){throw Error(l(522))},D:r,C:r,L:r,m:r,X:r,S:r,M:r},p:0,findDOMNode:null},i=Symbol.for("react.portal");function f(m,d,x){var M=3<arguments.length&&arguments[3]!==void 0?arguments[3]:null;return{$$typeof:i,key:M==null?null:""+M,children:m,containerInfo:d,implementation:x}}var h=o.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE;function g(m,d){if(m==="font")return"";if(typeof d=="string")return d==="use-credentials"?d:""}return Fa.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE=c,Fa.createPortal=function(m,d){var x=2<arguments.length&&arguments[2]!==void 0?arguments[2]:null;if(!d||d.nodeType!==1&&d.nodeType!==9&&d.nodeType!==11)throw Error(l(299));return f(m,d,null,x)},Fa.flushSync=function(m){var d=h.T,x=c.p;try{if(h.T=null,c.p=2,m)return m()}finally{h.T=d,c.p=x,c.d.f()}},Fa.preconnect=function(m,d){typeof m=="string"&&(d?(d=d.crossOrigin,d=typeof d=="string"?d==="use-credentials"?d:"":void 0):d=null,c.d.C(m,d))},Fa.prefetchDNS=function(m){typeof m=="string"&&c.d.D(m)},Fa.preinit=function(m,d){if(typeof m=="string"&&d&&typeof d.as=="string"){var x=d.as,M=g(x,d.crossOrigin),T=typeof d.integrity=="string"?d.integrity:void 0,A=typeof d.fetchPriority=="string"?d.fetchPriority:void 0;x==="style"?c.d.S(m,typeof d.precedence=="string"?d.precedence:void 0,{crossOrigin:M,integrity:T,fetchPriority:A}):x==="script"&&c.d.X(m,{crossOrigin:M,integrity:T,fetchPriority:A,nonce:typeof d.nonce=="string"?d.nonce:void 0})}},Fa.preinitModule=function(m,d){if(typeof m=="string")if(typeof d=="object"&&d!==null){if(d.as==null||d.as==="script"){var x=g(d.as,d.crossOrigin);c.d.M(m,{crossOrigin:x,integrity:typeof d.integrity=="string"?d.integrity:void 0,nonce:typeof d.nonce=="string"?d.nonce:void 0})}}else d==null&&c.d.M(m)},Fa.preload=function(m,d){if(typeof m=="string"&&typeof d=="object"&&d!==null&&typeof d.as=="string"){var x=d.as,M=g(x,d.crossOrigin);c.d.L(m,x,{crossOrigin:M,integrity:typeof d.integrity=="string"?d.integrity:void 0,nonce:typeof d.nonce=="string"?d.nonce:void 0,type:typeof d.type=="string"?d.type:void 0,fetchPriority:typeof d.fetchPriority=="string"?d.fetchPriority:void 0,referrerPolicy:typeof d.referrerPolicy=="string"?d.referrerPolicy:void 0,imageSrcSet:typeof d.imageSrcSet=="string"?d.imageSrcSet:void 0,imageSizes:typeof d.imageSizes=="string"?d.imageSizes:void 0,media:typeof d.media=="string"?d.media:void 0})}},Fa.preloadModule=function(m,d){if(typeof m=="string")if(d){var x=g(d.as,d.crossOrigin);c.d.m(m,{as:typeof d.as=="string"&&d.as!=="script"?d.as:void 0,crossOrigin:x,integrity:typeof d.integrity=="string"?d.integrity:void 0})}else c.d.m(m)},Fa.requestFormReset=function(m){c.d.r(m)},Fa.unstable_batchedUpdates=function(m,d){return m(d)},Fa.useFormState=function(m,d,x){return h.H.useFormState(m,d,x)},Fa.useFormStatus=function(){return h.H.useHostTransitionStatus()},Fa.version="19.2.8",Fa}var A1;function Vg(){if(A1)return $f.exports;A1=1;function o(){if(!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__>"u"||typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE!="function"))try{__REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(o)}catch(l){console.error(l)}}return o(),$f.exports=qg(),$f.exports}/**
 * @license React
 * react-dom-client.production.js
 *
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var E1;function Kg(){if(E1)return Uc;E1=1;var o=Gg(),l=Eh(),r=Vg();function c(e){var t="https://react.dev/errors/"+e;if(1<arguments.length){t+="?args[]="+encodeURIComponent(arguments[1]);for(var a=2;a<arguments.length;a++)t+="&args[]="+encodeURIComponent(arguments[a])}return"Minified React error #"+e+"; visit "+t+" for the full message or use the non-minified dev environment for full errors and additional helpful warnings."}function i(e){return!(!e||e.nodeType!==1&&e.nodeType!==9&&e.nodeType!==11)}function f(e){var t=e,a=e;if(e.alternate)for(;t.return;)t=t.return;else{e=t;do t=e,(t.flags&4098)!==0&&(a=t.return),e=t.return;while(e)}return t.tag===3?a:null}function h(e){if(e.tag===13){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function g(e){if(e.tag===31){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function m(e){if(f(e)!==e)throw Error(c(188))}function d(e){var t=e.alternate;if(!t){if(t=f(e),t===null)throw Error(c(188));return t!==e?null:e}for(var a=e,n=t;;){var s=a.return;if(s===null)break;var u=s.alternate;if(u===null){if(n=s.return,n!==null){a=n;continue}break}if(s.child===u.child){for(u=s.child;u;){if(u===a)return m(s),e;if(u===n)return m(s),t;u=u.sibling}throw Error(c(188))}if(a.return!==n.return)a=s,n=u;else{for(var p=!1,w=s.child;w;){if(w===a){p=!0,a=s,n=u;break}if(w===n){p=!0,n=s,a=u;break}w=w.sibling}if(!p){for(w=u.child;w;){if(w===a){p=!0,a=u,n=s;break}if(w===n){p=!0,n=u,a=s;break}w=w.sibling}if(!p)throw Error(c(189))}}if(a.alternate!==n)throw Error(c(190))}if(a.tag!==3)throw Error(c(188));return a.stateNode.current===a?e:t}function x(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e;for(e=e.child;e!==null;){if(t=x(e),t!==null)return t;e=e.sibling}return null}var M=Object.assign,T=Symbol.for("react.element"),A=Symbol.for("react.transitional.element"),R=Symbol.for("react.portal"),B=Symbol.for("react.fragment"),C=Symbol.for("react.strict_mode"),H=Symbol.for("react.profiler"),K=Symbol.for("react.consumer"),U=Symbol.for("react.context"),_=Symbol.for("react.forward_ref"),I=Symbol.for("react.suspense"),Z=Symbol.for("react.suspense_list"),le=Symbol.for("react.memo"),X=Symbol.for("react.lazy"),se=Symbol.for("react.activity"),we=Symbol.for("react.memo_cache_sentinel"),ge=Symbol.iterator;function G(e){return e===null||typeof e!="object"?null:(e=ge&&e[ge]||e["@@iterator"],typeof e=="function"?e:null)}var F=Symbol.for("react.client.reference");function ie(e){if(e==null)return null;if(typeof e=="function")return e.$$typeof===F?null:e.displayName||e.name||null;if(typeof e=="string")return e;switch(e){case B:return"Fragment";case H:return"Profiler";case C:return"StrictMode";case I:return"Suspense";case Z:return"SuspenseList";case se:return"Activity"}if(typeof e=="object")switch(e.$$typeof){case R:return"Portal";case U:return e.displayName||"Context";case K:return(e._context.displayName||"Context")+".Consumer";case _:var t=e.render;return e=e.displayName,e||(e=t.displayName||t.name||"",e=e!==""?"ForwardRef("+e+")":"ForwardRef"),e;case le:return t=e.displayName||null,t!==null?t:ie(e.type)||"Memo";case X:t=e._payload,e=e._init;try{return ie(e(t))}catch{}}return null}var $=Array.isArray,N=l.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,P=r.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,k={pending:!1,data:null,method:null,action:null},ee=[],Me=-1;function S(e){return{current:e}}function Q(e){0>Me||(e.current=ee[Me],ee[Me]=null,Me--)}function ve(e,t){Me++,ee[Me]=e.current,e.current=t}var V=S(null),pe=S(null),ue=S(null),Ce=S(null);function Ne(e,t){switch(ve(ue,t),ve(pe,e),ve(V,null),t.nodeType){case 9:case 11:e=(e=t.documentElement)&&(e=e.namespaceURI)?Bm(e):0;break;default:if(e=t.tagName,t=t.namespaceURI)t=Bm(t),e=Hm(t,e);else switch(e){case"svg":e=1;break;case"math":e=2;break;default:e=0}}Q(V),ve(V,e)}function oe(){Q(V),Q(pe),Q(ue)}function J(e){e.memoizedState!==null&&ve(Ce,e);var t=V.current,a=Hm(t,e.type);t!==a&&(ve(pe,e),ve(V,a))}function _e(e){pe.current===e&&(Q(V),Q(pe)),Ce.current===e&&(Q(Ce),Ac._currentValue=k)}var Je,O;function Pe(e){if(Je===void 0)try{throw Error()}catch(a){var t=a.stack.trim().match(/\n( *(at )?)/);Je=t&&t[1]||"",O=-1<a.stack.indexOf(`
    at`)?" (<anonymous>)":-1<a.stack.indexOf("@")?"@unknown:0:0":""}return`
`+Je+e+O}var Ie=!1;function tt(e,t){if(!e||Ie)return"";Ie=!0;var a=Error.prepareStackTrace;Error.prepareStackTrace=void 0;try{var n={DetermineComponentFrameRoot:function(){try{if(t){var Re=function(){throw Error()};if(Object.defineProperty(Re.prototype,"props",{set:function(){throw Error()}}),typeof Reflect=="object"&&Reflect.construct){try{Reflect.construct(Re,[])}catch(xe){var de=xe}Reflect.construct(e,[],Re)}else{try{Re.call()}catch(xe){de=xe}e.call(Re.prototype)}}else{try{throw Error()}catch(xe){de=xe}(Re=e())&&typeof Re.catch=="function"&&Re.catch(function(){})}}catch(xe){if(xe&&de&&typeof xe.stack=="string")return[xe.stack,de.stack]}return[null,null]}};n.DetermineComponentFrameRoot.displayName="DetermineComponentFrameRoot";var s=Object.getOwnPropertyDescriptor(n.DetermineComponentFrameRoot,"name");s&&s.configurable&&Object.defineProperty(n.DetermineComponentFrameRoot,"name",{value:"DetermineComponentFrameRoot"});var u=n.DetermineComponentFrameRoot(),p=u[0],w=u[1];if(p&&w){var q=p.split(`
`),fe=w.split(`
`);for(s=n=0;n<q.length&&!q[n].includes("DetermineComponentFrameRoot");)n++;for(;s<fe.length&&!fe[s].includes("DetermineComponentFrameRoot");)s++;if(n===q.length||s===fe.length)for(n=q.length-1,s=fe.length-1;1<=n&&0<=s&&q[n]!==fe[s];)s--;for(;1<=n&&0<=s;n--,s--)if(q[n]!==fe[s]){if(n!==1||s!==1)do if(n--,s--,0>s||q[n]!==fe[s]){var Te=`
`+q[n].replace(" at new "," at ");return e.displayName&&Te.includes("<anonymous>")&&(Te=Te.replace("<anonymous>",e.displayName)),Te}while(1<=n&&0<=s);break}}}finally{Ie=!1,Error.prepareStackTrace=a}return(a=e?e.displayName||e.name:"")?Pe(a):""}function Nt(e,t){switch(e.tag){case 26:case 27:case 5:return Pe(e.type);case 16:return Pe("Lazy");case 13:return e.child!==t&&t!==null?Pe("Suspense Fallback"):Pe("Suspense");case 19:return Pe("SuspenseList");case 0:case 15:return tt(e.type,!1);case 11:return tt(e.type.render,!1);case 1:return tt(e.type,!0);case 31:return Pe("Activity");default:return""}}function Ft(e){try{var t="",a=null;do t+=Nt(e,a),a=e,e=e.return;while(e);return t}catch(n){return`
Error generating stack: `+n.message+`
`+n.stack}}var $t=Object.prototype.hasOwnProperty,Yt=o.unstable_scheduleCallback,Aa=o.unstable_cancelCallback,mn=o.unstable_shouldYield,Ea=o.unstable_requestPaint,Dt=o.unstable_now,_o=o.unstable_getCurrentPriorityLevel,sa=o.unstable_ImmediatePriority,Ra=o.unstable_UserBlockingPriority,Ya=o.unstable_NormalPriority,pn=o.unstable_LowPriority,Na=o.unstable_IdlePriority,vn=o.log,Ia=o.unstable_setDisableYieldValue,gn=null,na=null;function yn(e){if(typeof vn=="function"&&Ia(e),na&&typeof na.setStrictMode=="function")try{na.setStrictMode(gn,e)}catch{}}var ha=Math.clz32?Math.clz32:dt,zo=Math.log,Ao=Math.LN2;function dt(e){return e>>>=0,e===0?32:31-(zo(e)/Ao|0)|0}var Da=256,Wa=262144,cl=4194304;function Eo(e){var t=e&42;if(t!==0)return t;switch(e&-e){case 1:return 1;case 2:return 2;case 4:return 4;case 8:return 8;case 16:return 16;case 32:return 32;case 64:return 64;case 128:return 128;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:return e&261888;case 262144:case 524288:case 1048576:case 2097152:return e&3932160;case 4194304:case 8388608:case 16777216:case 33554432:return e&62914560;case 67108864:return 67108864;case 134217728:return 134217728;case 268435456:return 268435456;case 536870912:return 536870912;case 1073741824:return 0;default:return e}}function Ps(e,t,a){var n=e.pendingLanes;if(n===0)return 0;var s=0,u=e.suspendedLanes,p=e.pingedLanes;e=e.warmLanes;var w=n&134217727;return w!==0?(n=w&~u,n!==0?s=Eo(n):(p&=w,p!==0?s=Eo(p):a||(a=w&~e,a!==0&&(s=Eo(a))))):(w=n&~u,w!==0?s=Eo(w):p!==0?s=Eo(p):a||(a=n&~e,a!==0&&(s=Eo(a)))),s===0?0:t!==0&&t!==s&&(t&u)===0&&(u=s&-s,a=t&-t,u>=a||u===32&&(a&4194048)!==0)?t:s}function ul(e,t){return(e.pendingLanes&~(e.suspendedLanes&~e.pingedLanes)&t)===0}function Un(e,t){switch(e){case 1:case 2:case 4:case 8:case 64:return t+250;case 16:case 32:case 128:case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:return t+5e3;case 4194304:case 8388608:case 16777216:case 33554432:return-1;case 67108864:case 134217728:case 268435456:case 536870912:case 1073741824:return-1;default:return-1}}function qc(){var e=cl;return cl<<=1,(cl&62914560)===0&&(cl=4194304),e}function Ur(e){for(var t=[],a=0;31>a;a++)t.push(e);return t}function Ys(e,t){e.pendingLanes|=t,t!==268435456&&(e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0)}function Vc(e,t,a,n,s,u){var p=e.pendingLanes;e.pendingLanes=a,e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0,e.expiredLanes&=a,e.entangledLanes&=a,e.errorRecoveryDisabledLanes&=a,e.shellSuspendCounter=0;var w=e.entanglements,q=e.expirationTimes,fe=e.hiddenUpdates;for(a=p&~a;0<a;){var Te=31-ha(a),Re=1<<Te;w[Te]=0,q[Te]=-1;var de=fe[Te];if(de!==null)for(fe[Te]=null,Te=0;Te<de.length;Te++){var xe=de[Te];xe!==null&&(xe.lane&=-536870913)}a&=~Re}n!==0&&Or(e,n,0),u!==0&&s===0&&e.tag!==0&&(e.suspendedLanes|=u&~(p&~t))}function Or(e,t,a){e.pendingLanes|=t,e.suspendedLanes&=~t;var n=31-ha(t);e.entangledLanes|=t,e.entanglements[n]=e.entanglements[n]|1073741824|a&261930}function Di(e,t){var a=e.entangledLanes|=t;for(e=e.entanglements;a;){var n=31-ha(a),s=1<<n;s&t|e[n]&t&&(e[n]|=t),a&=~s}}function On(e,t){var a=t&-t;return a=(a&42)!==0?1:Ws(a),(a&(e.suspendedLanes|t))!==0?0:a}function Ws(e){switch(e){case 2:e=1;break;case 8:e=4;break;case 32:e=16;break;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:case 4194304:case 8388608:case 16777216:case 33554432:e=128;break;case 268435456:e=134217728;break;default:e=0}return e}function es(e){return e&=-e,2<e?8<e?(e&134217727)!==0?32:268435456:8:2}function Br(){var e=P.p;return e!==0?e:(e=window.event,e===void 0?32:s1(e.type))}function Hr(e,t){var a=P.p;try{return P.p=e,t()}finally{P.p=a}}var Bn=Math.random().toString(36).slice(2),oa="__reactFiber$"+Bn,ma="__reactProps$"+Bn,to="__reactContainer$"+Bn,Xs="__reactEvents$"+Bn,Kc="__reactListeners$"+Bn,ki="__reactHandles$"+Bn,Ui="__reactResources$"+Bn,fl="__reactMarker$"+Bn;function Zs(e){delete e[oa],delete e[ma],delete e[Xs],delete e[Kc],delete e[ki]}function Ro(e){var t=e[oa];if(t)return t;for(var a=e.parentNode;a;){if(t=a[to]||a[oa]){if(a=t.alternate,t.child!==null||a!==null&&a.child!==null)for(e=Km(e);e!==null;){if(a=e[oa])return a;e=Km(e)}return t}e=a,a=e.parentNode}return null}function No(e){if(e=e[oa]||e[to]){var t=e.tag;if(t===5||t===6||t===13||t===31||t===26||t===27||t===3)return e}return null}function hl(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e.stateNode;throw Error(c(33))}function Do(e){var t=e[Ui];return t||(t=e[Ui]={hoistableStyles:new Map,hoistableScripts:new Map}),t}function ea(e){e[fl]=!0}var Oi=new Set,Bi={};function ao(e,t){ko(e,t),ko(e+"Capture",t)}function ko(e,t){for(Bi[e]=t,e=0;e<t.length;e++)Oi.add(t[e])}var Lr=RegExp("^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$"),Qs={},Hi={};function jr(e){return $t.call(Hi,e)?!0:$t.call(Qs,e)?!1:Lr.test(e)?Hi[e]=!0:(Qs[e]=!0,!1)}function ts(e,t,a){if(jr(t))if(a===null)e.removeAttribute(t);else{switch(typeof a){case"undefined":case"function":case"symbol":e.removeAttribute(t);return;case"boolean":var n=t.toLowerCase().slice(0,5);if(n!=="data-"&&n!=="aria-"){e.removeAttribute(t);return}}e.setAttribute(t,""+a)}}function as(e,t,a){if(a===null)e.removeAttribute(t);else{switch(typeof a){case"undefined":case"function":case"symbol":case"boolean":e.removeAttribute(t);return}e.setAttribute(t,""+a)}}function Hn(e,t,a,n){if(n===null)e.removeAttribute(a);else{switch(typeof n){case"undefined":case"function":case"symbol":case"boolean":e.removeAttribute(a);return}e.setAttributeNS(t,a,""+n)}}function Wt(e){switch(typeof e){case"bigint":case"boolean":case"number":case"string":case"undefined":return e;case"object":return e;default:return""}}function ns(e){var t=e.type;return(e=e.nodeName)&&e.toLowerCase()==="input"&&(t==="checkbox"||t==="radio")}function os(e,t,a){var n=Object.getOwnPropertyDescriptor(e.constructor.prototype,t);if(!e.hasOwnProperty(t)&&typeof n<"u"&&typeof n.get=="function"&&typeof n.set=="function"){var s=n.get,u=n.set;return Object.defineProperty(e,t,{configurable:!0,get:function(){return s.call(this)},set:function(p){a=""+p,u.call(this,p)}}),Object.defineProperty(e,t,{enumerable:n.enumerable}),{getValue:function(){return a},setValue:function(p){a=""+p},stopTracking:function(){e._valueTracker=null,delete e[t]}}}}function Uo(e){if(!e._valueTracker){var t=ns(e)?"checked":"value";e._valueTracker=os(e,t,""+e[t])}}function Pc(e){if(!e)return!1;var t=e._valueTracker;if(!t)return!0;var a=t.getValue(),n="";return e&&(n=ns(e)?e.checked?"true":"false":e.value),e=n,e!==a?(t.setValue(e),!0):!1}function Li(e){if(e=e||(typeof document<"u"?document:void 0),typeof e>"u")return null;try{return e.activeElement||e.body}catch{return e.body}}var Mt=/[\n"\\]/g;function nt(e){return e.replace(Mt,function(t){return"\\"+t.charCodeAt(0).toString(16)+" "})}function Ja(e,t,a,n,s,u,p,w){e.name="",p!=null&&typeof p!="function"&&typeof p!="symbol"&&typeof p!="boolean"?e.type=p:e.removeAttribute("type"),t!=null?p==="number"?(t===0&&e.value===""||e.value!=t)&&(e.value=""+Wt(t)):e.value!==""+Wt(t)&&(e.value=""+Wt(t)):p!=="submit"&&p!=="reset"||e.removeAttribute("value"),t!=null?Oo(e,p,Wt(t)):a!=null?Oo(e,p,Wt(a)):n!=null&&e.removeAttribute("value"),s==null&&u!=null&&(e.defaultChecked=!!u),s!=null&&(e.checked=s&&typeof s!="function"&&typeof s!="symbol"),w!=null&&typeof w!="function"&&typeof w!="symbol"&&typeof w!="boolean"?e.name=""+Wt(w):e.removeAttribute("name")}function ls(e,t,a,n,s,u,p,w){if(u!=null&&typeof u!="function"&&typeof u!="symbol"&&typeof u!="boolean"&&(e.type=u),t!=null||a!=null){if(!(u!=="submit"&&u!=="reset"||t!=null)){Uo(e);return}a=a!=null?""+Wt(a):"",t=t!=null?""+Wt(t):a,w||t===e.value||(e.value=t),e.defaultValue=t}n=n??s,n=typeof n!="function"&&typeof n!="symbol"&&!!n,e.checked=w?e.checked:!!n,e.defaultChecked=!!n,p!=null&&typeof p!="function"&&typeof p!="symbol"&&typeof p!="boolean"&&(e.name=p),Uo(e)}function Oo(e,t,a){t==="number"&&Li(e.ownerDocument)===e||e.defaultValue===""+a||(e.defaultValue=""+a)}function Bo(e,t,a,n){if(e=e.options,t){t={};for(var s=0;s<a.length;s++)t["$"+a[s]]=!0;for(a=0;a<e.length;a++)s=t.hasOwnProperty("$"+e[a].value),e[a].selected!==s&&(e[a].selected=s),s&&n&&(e[a].defaultSelected=!0)}else{for(a=""+Wt(a),t=null,s=0;s<e.length;s++){if(e[s].value===a){e[s].selected=!0,n&&(e[s].defaultSelected=!0);return}t!==null||e[s].disabled||(t=e[s])}t!==null&&(t.selected=!0)}}function Fr(e,t,a){if(t!=null&&(t=""+Wt(t),t!==e.value&&(e.value=t),a==null)){e.defaultValue!==t&&(e.defaultValue=t);return}e.defaultValue=a!=null?""+Wt(a):""}function dl(e,t,a,n){if(t==null){if(n!=null){if(a!=null)throw Error(c(92));if($(n)){if(1<n.length)throw Error(c(93));n=n[0]}a=n}a==null&&(a=""),t=a}a=Wt(t),e.defaultValue=a,n=e.textContent,n===a&&n!==""&&n!==null&&(e.value=n),Uo(e)}function ml(e,t){if(t){var a=e.firstChild;if(a&&a===e.lastChild&&a.nodeType===3){a.nodeValue=t;return}}e.textContent=t}var Yc=new Set("animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp".split(" "));function pl(e,t,a){var n=t.indexOf("--")===0;a==null||typeof a=="boolean"||a===""?n?e.setProperty(t,""):t==="float"?e.cssFloat="":e[t]="":n?e.setProperty(t,a):typeof a!="number"||a===0||Yc.has(t)?t==="float"?e.cssFloat=a:e[t]=(""+a).trim():e[t]=a+"px"}function Is(e,t,a){if(t!=null&&typeof t!="object")throw Error(c(62));if(e=e.style,a!=null){for(var n in a)!a.hasOwnProperty(n)||t!=null&&t.hasOwnProperty(n)||(n.indexOf("--")===0?e.setProperty(n,""):n==="float"?e.cssFloat="":e[n]="");for(var s in t)n=t[s],t.hasOwnProperty(s)&&a[s]!==n&&pl(e,s,n)}else for(var u in t)t.hasOwnProperty(u)&&pl(e,u,t[u])}function ji(e){if(e.indexOf("-")===-1)return!1;switch(e){case"annotation-xml":case"color-profile":case"font-face":case"font-face-src":case"font-face-uri":case"font-face-format":case"font-face-name":case"missing-glyph":return!1;default:return!0}}var Mu=new Map([["acceptCharset","accept-charset"],["htmlFor","for"],["httpEquiv","http-equiv"],["crossOrigin","crossorigin"],["accentHeight","accent-height"],["alignmentBaseline","alignment-baseline"],["arabicForm","arabic-form"],["baselineShift","baseline-shift"],["capHeight","cap-height"],["clipPath","clip-path"],["clipRule","clip-rule"],["colorInterpolation","color-interpolation"],["colorInterpolationFilters","color-interpolation-filters"],["colorProfile","color-profile"],["colorRendering","color-rendering"],["dominantBaseline","dominant-baseline"],["enableBackground","enable-background"],["fillOpacity","fill-opacity"],["fillRule","fill-rule"],["floodColor","flood-color"],["floodOpacity","flood-opacity"],["fontFamily","font-family"],["fontSize","font-size"],["fontSizeAdjust","font-size-adjust"],["fontStretch","font-stretch"],["fontStyle","font-style"],["fontVariant","font-variant"],["fontWeight","font-weight"],["glyphName","glyph-name"],["glyphOrientationHorizontal","glyph-orientation-horizontal"],["glyphOrientationVertical","glyph-orientation-vertical"],["horizAdvX","horiz-adv-x"],["horizOriginX","horiz-origin-x"],["imageRendering","image-rendering"],["letterSpacing","letter-spacing"],["lightingColor","lighting-color"],["markerEnd","marker-end"],["markerMid","marker-mid"],["markerStart","marker-start"],["overlinePosition","overline-position"],["overlineThickness","overline-thickness"],["paintOrder","paint-order"],["panose-1","panose-1"],["pointerEvents","pointer-events"],["renderingIntent","rendering-intent"],["shapeRendering","shape-rendering"],["stopColor","stop-color"],["stopOpacity","stop-opacity"],["strikethroughPosition","strikethrough-position"],["strikethroughThickness","strikethrough-thickness"],["strokeDasharray","stroke-dasharray"],["strokeDashoffset","stroke-dashoffset"],["strokeLinecap","stroke-linecap"],["strokeLinejoin","stroke-linejoin"],["strokeMiterlimit","stroke-miterlimit"],["strokeOpacity","stroke-opacity"],["strokeWidth","stroke-width"],["textAnchor","text-anchor"],["textDecoration","text-decoration"],["textRendering","text-rendering"],["transformOrigin","transform-origin"],["underlinePosition","underline-position"],["underlineThickness","underline-thickness"],["unicodeBidi","unicode-bidi"],["unicodeRange","unicode-range"],["unitsPerEm","units-per-em"],["vAlphabetic","v-alphabetic"],["vHanging","v-hanging"],["vIdeographic","v-ideographic"],["vMathematical","v-mathematical"],["vectorEffect","vector-effect"],["vertAdvY","vert-adv-y"],["vertOriginX","vert-origin-x"],["vertOriginY","vert-origin-y"],["wordSpacing","word-spacing"],["writingMode","writing-mode"],["xmlnsXlink","xmlns:xlink"],["xHeight","x-height"]]),Fi=/^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;function vl(e){return Fi.test(""+e)?"javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')":e}function no(){}var st=null;function it(e){return e=e.target||e.srcElement||window,e.correspondingUseElement&&(e=e.correspondingUseElement),e.nodeType===3?e.parentNode:e}var ss=null,Ho=null;function Gr(e){var t=No(e);if(t&&(e=t.stateNode)){var a=e[ma]||null;e:switch(e=t.stateNode,t.type){case"input":if(Ja(e,a.value,a.defaultValue,a.defaultValue,a.checked,a.defaultChecked,a.type,a.name),t=a.name,a.type==="radio"&&t!=null){for(a=e;a.parentNode;)a=a.parentNode;for(a=a.querySelectorAll('input[name="'+nt(""+t)+'"][type="radio"]'),t=0;t<a.length;t++){var n=a[t];if(n!==e&&n.form===e.form){var s=n[ma]||null;if(!s)throw Error(c(90));Ja(n,s.value,s.defaultValue,s.defaultValue,s.checked,s.defaultChecked,s.type,s.name)}}for(t=0;t<a.length;t++)n=a[t],n.form===e.form&&Pc(n)}break e;case"textarea":Fr(e,a.value,a.defaultValue);break e;case"select":t=a.value,t!=null&&Bo(e,!!a.multiple,t,!1)}}}var Js=!1;function Wc(e,t,a){if(Js)return e(t,a);Js=!0;try{var n=e(t);return n}finally{if(Js=!1,(ss!==null||Ho!==null)&&(C0(),ss&&(t=ss,e=Ho,Ho=ss=null,Gr(t),e)))for(t=0;t<e.length;t++)Gr(e[t])}}function gl(e,t){var a=e.stateNode;if(a===null)return null;var n=a[ma]||null;if(n===null)return null;a=n[t];e:switch(t){case"onClick":case"onClickCapture":case"onDoubleClick":case"onDoubleClickCapture":case"onMouseDown":case"onMouseDownCapture":case"onMouseMove":case"onMouseMoveCapture":case"onMouseUp":case"onMouseUpCapture":case"onMouseEnter":(n=!n.disabled)||(e=e.type,n=!(e==="button"||e==="input"||e==="select"||e==="textarea")),e=!n;break e;default:e=!1}if(e)return null;if(a&&typeof a!="function")throw Error(c(231,t,typeof a));return a}var ka=!(typeof window>"u"||typeof window.document>"u"||typeof window.document.createElement>"u"),is=!1;if(ka)try{var rs={};Object.defineProperty(rs,"passive",{get:function(){is=!0}}),window.addEventListener("test",rs,rs),window.removeEventListener("test",rs,rs)}catch{is=!1}var oo=null,yl=null,xl=null;function Xc(){if(xl)return xl;var e,t=yl,a=t.length,n,s="value"in oo?oo.value:oo.textContent,u=s.length;for(e=0;e<a&&t[e]===s[e];e++);var p=a-e;for(n=1;n<=p&&t[a-n]===s[u-n];n++);return xl=s.slice(e,1<n?1-n:void 0)}function Et(e){var t=e.keyCode;return"charCode"in e?(e=e.charCode,e===0&&t===13&&(e=13)):e=t,e===10&&(e=13),32<=e||e===13?e:0}function xn(){return!0}function Zc(){return!1}function Ma(e){function t(a,n,s,u,p){this._reactName=a,this._targetInst=s,this.type=n,this.nativeEvent=u,this.target=p,this.currentTarget=null;for(var w in e)e.hasOwnProperty(w)&&(a=e[w],this[w]=a?a(u):u[w]);return this.isDefaultPrevented=(u.defaultPrevented!=null?u.defaultPrevented:u.returnValue===!1)?xn:Zc,this.isPropagationStopped=Zc,this}return M(t.prototype,{preventDefault:function(){this.defaultPrevented=!0;var a=this.nativeEvent;a&&(a.preventDefault?a.preventDefault():typeof a.returnValue!="unknown"&&(a.returnValue=!1),this.isDefaultPrevented=xn)},stopPropagation:function(){var a=this.nativeEvent;a&&(a.stopPropagation?a.stopPropagation():typeof a.cancelBubble!="unknown"&&(a.cancelBubble=!0),this.isPropagationStopped=xn)},persist:function(){},isPersistent:xn}),t}var Ln={eventPhase:0,bubbles:0,cancelable:0,timeStamp:function(e){return e.timeStamp||Date.now()},defaultPrevented:0,isTrusted:0},Gi=Ma(Ln),$s=M({},Ln,{view:0,detail:0}),Su=Ma($s),cs,qr,us,qi=M({},$s,{screenX:0,screenY:0,clientX:0,clientY:0,pageX:0,pageY:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,getModifierState:Vi,button:0,buttons:0,relatedTarget:function(e){return e.relatedTarget===void 0?e.fromElement===e.srcElement?e.toElement:e.fromElement:e.relatedTarget},movementX:function(e){return"movementX"in e?e.movementX:(e!==us&&(us&&e.type==="mousemove"?(cs=e.screenX-us.screenX,qr=e.screenY-us.screenY):qr=cs=0,us=e),cs)},movementY:function(e){return"movementY"in e?e.movementY:qr}}),wn=Ma(qi),Lo=M({},qi,{dataTransfer:0}),ei=Ma(Lo),wl=M({},$s,{relatedTarget:0}),ti=Ma(wl),Qc=M({},Ln,{animationName:0,elapsedTime:0,pseudoElement:0}),Ic=Ma(Qc),Tu=M({},Ln,{clipboardData:function(e){return"clipboardData"in e?e.clipboardData:window.clipboardData}}),Cu=Ma(Tu),_u=M({},Ln,{data:0}),ai=Ma(_u),jo={Esc:"Escape",Spacebar:" ",Left:"ArrowLeft",Up:"ArrowUp",Right:"ArrowRight",Down:"ArrowDown",Del:"Delete",Win:"OS",Menu:"ContextMenu",Apps:"ContextMenu",Scroll:"ScrollLock",MozPrintableKey:"Unidentified"},Jc={8:"Backspace",9:"Tab",12:"Clear",13:"Enter",16:"Shift",17:"Control",18:"Alt",19:"Pause",20:"CapsLock",27:"Escape",32:" ",33:"PageUp",34:"PageDown",35:"End",36:"Home",37:"ArrowLeft",38:"ArrowUp",39:"ArrowRight",40:"ArrowDown",45:"Insert",46:"Delete",112:"F1",113:"F2",114:"F3",115:"F4",116:"F5",117:"F6",118:"F7",119:"F8",120:"F9",121:"F10",122:"F11",123:"F12",144:"NumLock",145:"ScrollLock",224:"Meta"},zu={Alt:"altKey",Control:"ctrlKey",Meta:"metaKey",Shift:"shiftKey"};function Au(e){var t=this.nativeEvent;return t.getModifierState?t.getModifierState(e):(e=zu[e])?!!t[e]:!1}function Vi(){return Au}var Eu=M({},$s,{key:function(e){if(e.key){var t=jo[e.key]||e.key;if(t!=="Unidentified")return t}return e.type==="keypress"?(e=Et(e),e===13?"Enter":String.fromCharCode(e)):e.type==="keydown"||e.type==="keyup"?Jc[e.keyCode]||"Unidentified":""},code:0,location:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,repeat:0,locale:0,getModifierState:Vi,charCode:function(e){return e.type==="keypress"?Et(e):0},keyCode:function(e){return e.type==="keydown"||e.type==="keyup"?e.keyCode:0},which:function(e){return e.type==="keypress"?Et(e):e.type==="keydown"||e.type==="keyup"?e.keyCode:0}}),Ru=Ma(Eu),ni=M({},qi,{pointerId:0,width:0,height:0,pressure:0,tangentialPressure:0,tiltX:0,tiltY:0,twist:0,pointerType:0,isPrimary:0}),Vr=Ma(ni),$c=M({},$s,{touches:0,targetTouches:0,changedTouches:0,altKey:0,metaKey:0,ctrlKey:0,shiftKey:0,getModifierState:Vi}),Nu=Ma($c),Du=M({},Ln,{propertyName:0,elapsedTime:0,pseudoElement:0}),fs=Ma(Du),Kr=M({},qi,{deltaX:function(e){return"deltaX"in e?e.deltaX:"wheelDeltaX"in e?-e.wheelDeltaX:0},deltaY:function(e){return"deltaY"in e?e.deltaY:"wheelDeltaY"in e?-e.wheelDeltaY:"wheelDelta"in e?-e.wheelDelta:0},deltaZ:0,deltaMode:0}),Pr=Ma(Kr),Yr=M({},Ln,{newState:0,oldState:0}),Wr=Ma(Yr),ku=[9,13,27,32],Xr=ka&&"CompositionEvent"in window,oi=null;ka&&"documentMode"in document&&(oi=document.documentMode);var li=ka&&"TextEvent"in window&&!oi,Ki=ka&&(!Xr||oi&&8<oi&&11>=oi),si=" ",e0=!1;function Pi(e,t){switch(e){case"keyup":return ku.indexOf(t.keyCode)!==-1;case"keydown":return t.keyCode!==229;case"keypress":case"mousedown":case"focusout":return!0;default:return!1}}function t0(e){return e=e.detail,typeof e=="object"&&"data"in e?e.data:null}var bl=!1;function Uu(e,t){switch(e){case"compositionend":return t0(t);case"keypress":return t.which!==32?null:(e0=!0,si);case"textInput":return e=t.data,e===si&&e0?null:e;default:return null}}function Zr(e,t){if(bl)return e==="compositionend"||!Xr&&Pi(e,t)?(e=Xc(),xl=yl=oo=null,bl=!1,e):null;switch(e){case"paste":return null;case"keypress":if(!(t.ctrlKey||t.altKey||t.metaKey)||t.ctrlKey&&t.altKey){if(t.char&&1<t.char.length)return t.char;if(t.which)return String.fromCharCode(t.which)}return null;case"compositionend":return Ki&&t.locale!=="ko"?null:t.data;default:return null}}var Qr={color:!0,date:!0,datetime:!0,"datetime-local":!0,email:!0,month:!0,number:!0,password:!0,range:!0,search:!0,tel:!0,text:!0,time:!0,url:!0,week:!0};function Yi(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t==="input"?!!Qr[e.type]:t==="textarea"}function Wi(e,t,a,n){ss?Ho?Ho.push(n):Ho=[n]:ss=n,t=D0(t,"onChange"),0<t.length&&(a=new Gi("onChange","change",null,a,n),e.push({event:a,listeners:t}))}var Ml=null,Fo=null;function Xi(e){Rm(e,0)}function ii(e){var t=hl(e);if(Pc(t))return e}function Zi(e,t){if(e==="change")return t}var jn=!1;if(ka){var pa;if(ka){var ri="oninput"in document;if(!ri){var Fn=document.createElement("div");Fn.setAttribute("oninput","return;"),ri=typeof Fn.oninput=="function"}pa=ri}else pa=!1;jn=pa&&(!document.documentMode||9<document.documentMode)}function hs(){Ml&&(Ml.detachEvent("onpropertychange",Sl),Fo=Ml=null)}function Sl(e){if(e.propertyName==="value"&&ii(Fo)){var t=[];Wi(t,Fo,e,it(e)),Wc(Xi,t)}}function Ou(e,t,a){e==="focusin"?(hs(),Ml=t,Fo=a,Ml.attachEvent("onpropertychange",Sl)):e==="focusout"&&hs()}function Ir(e){if(e==="selectionchange"||e==="keyup"||e==="keydown")return ii(Fo)}function $a(e,t){if(e==="click")return ii(t)}function a0(e,t){if(e==="input"||e==="change")return ii(t)}function Jr(e,t){return e===t&&(e!==0||1/e===1/t)||e!==e&&t!==t}var va=typeof Object.is=="function"?Object.is:Jr;function Tl(e,t){if(va(e,t))return!0;if(typeof e!="object"||e===null||typeof t!="object"||t===null)return!1;var a=Object.keys(e),n=Object.keys(t);if(a.length!==n.length)return!1;for(n=0;n<a.length;n++){var s=a[n];if(!$t.call(t,s)||!va(e[s],t[s]))return!1}return!0}function $r(e){for(;e&&e.firstChild;)e=e.firstChild;return e}function ec(e,t){var a=$r(e);e=0;for(var n;a;){if(a.nodeType===3){if(n=e+a.textContent.length,e<=t&&n>=t)return{node:a,offset:t-e};e=n}e:{for(;a;){if(a.nextSibling){a=a.nextSibling;break e}a=a.parentNode}a=void 0}a=$r(a)}}function Qi(e,t){return e&&t?e===t?!0:e&&e.nodeType===3?!1:t&&t.nodeType===3?Qi(e,t.parentNode):"contains"in e?e.contains(t):e.compareDocumentPosition?!!(e.compareDocumentPosition(t)&16):!1:!1}function Ii(e){e=e!=null&&e.ownerDocument!=null&&e.ownerDocument.defaultView!=null?e.ownerDocument.defaultView:window;for(var t=Li(e.document);t instanceof e.HTMLIFrameElement;){try{var a=typeof t.contentWindow.location.href=="string"}catch{a=!1}if(a)e=t.contentWindow;else break;t=Li(e.document)}return t}function Gn(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t&&(t==="input"&&(e.type==="text"||e.type==="search"||e.type==="tel"||e.type==="url"||e.type==="password")||t==="textarea"||e.contentEditable==="true")}var tc=ka&&"documentMode"in document&&11>=document.documentMode,qn=null,Cl=null,_l=null,ci=!1;function Ji(e,t,a){var n=a.window===a?a.document:a.nodeType===9?a:a.ownerDocument;ci||qn==null||qn!==Li(n)||(n=qn,"selectionStart"in n&&Gn(n)?n={start:n.selectionStart,end:n.selectionEnd}:(n=(n.ownerDocument&&n.ownerDocument.defaultView||window).getSelection(),n={anchorNode:n.anchorNode,anchorOffset:n.anchorOffset,focusNode:n.focusNode,focusOffset:n.focusOffset}),_l&&Tl(_l,n)||(_l=n,n=D0(Cl,"onSelect"),0<n.length&&(t=new Gi("onSelect","select",null,t,a),e.push({event:t,listeners:n}),t.target=qn)))}function Ua(e,t){var a={};return a[e.toLowerCase()]=t.toLowerCase(),a["Webkit"+e]="webkit"+t,a["Moz"+e]="moz"+t,a}var ds={animationend:Ua("Animation","AnimationEnd"),animationiteration:Ua("Animation","AnimationIteration"),animationstart:Ua("Animation","AnimationStart"),transitionrun:Ua("Transition","TransitionRun"),transitionstart:Ua("Transition","TransitionStart"),transitioncancel:Ua("Transition","TransitionCancel"),transitionend:Ua("Transition","TransitionEnd")},ac={},$i={};ka&&($i=document.createElement("div").style,"AnimationEvent"in window||(delete ds.animationend.animation,delete ds.animationiteration.animation,delete ds.animationstart.animation),"TransitionEvent"in window||delete ds.transitionend.transition);function Go(e){if(ac[e])return ac[e];if(!ds[e])return e;var t=ds[e],a;for(a in t)if(t.hasOwnProperty(a)&&a in $i)return ac[e]=t[a];return e}var nc=Go("animationend"),n0=Go("animationiteration"),ms=Go("animationstart"),Bu=Go("transitionrun"),o0=Go("transitionstart"),l0=Go("transitioncancel"),er=Go("transitionend"),s0=new Map,tr="abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel".split(" ");tr.push("scrollEnd");function en(e,t){s0.set(e,t),ao(t,[e])}var ar=typeof reportError=="function"?reportError:function(e){if(typeof window=="object"&&typeof window.ErrorEvent=="function"){var t=new window.ErrorEvent("error",{bubbles:!0,cancelable:!0,message:typeof e=="object"&&e!==null&&typeof e.message=="string"?String(e.message):String(e),error:e});if(!window.dispatchEvent(t))return}else if(typeof process=="object"&&typeof process.emit=="function"){process.emit("uncaughtException",e);return}console.error(e)},Oa=[],lo=0,nr=0;function ui(){for(var e=lo,t=nr=lo=0;t<e;){var a=Oa[t];Oa[t++]=null;var n=Oa[t];Oa[t++]=null;var s=Oa[t];Oa[t++]=null;var u=Oa[t];if(Oa[t++]=null,n!==null&&s!==null){var p=n.pending;p===null?s.next=s:(s.next=p.next,p.next=s),n.pending=s}u!==0&&zl(a,s,u)}}function ps(e,t,a,n){Oa[lo++]=e,Oa[lo++]=t,Oa[lo++]=a,Oa[lo++]=n,nr|=n,e.lanes|=n,e=e.alternate,e!==null&&(e.lanes|=n)}function fi(e,t,a,n){return ps(e,t,a,n),hi(e)}function Vn(e,t){return ps(e,null,null,t),hi(e)}function zl(e,t,a){e.lanes|=a;var n=e.alternate;n!==null&&(n.lanes|=a);for(var s=!1,u=e.return;u!==null;)u.childLanes|=a,n=u.alternate,n!==null&&(n.childLanes|=a),u.tag===22&&(e=u.stateNode,e===null||e._visibility&1||(s=!0)),e=u,u=u.return;return e.tag===3?(u=e.stateNode,s&&t!==null&&(s=31-ha(a),e=u.hiddenUpdates,n=e[s],n===null?e[s]=[t]:n.push(t),t.lane=a|536870912),u):null}function hi(e){if(50<bc)throw bc=0,yf=null,Error(c(185));for(var t=e.return;t!==null;)e=t,t=e.return;return e.tag===3?e.stateNode:null}var qo={};function or(e,t,a,n){this.tag=e,this.key=a,this.sibling=this.child=this.return=this.stateNode=this.type=this.elementType=null,this.index=0,this.refCleanup=this.ref=null,this.pendingProps=t,this.dependencies=this.memoizedState=this.updateQueue=this.memoizedProps=null,this.mode=n,this.subtreeFlags=this.flags=0,this.deletions=null,this.childLanes=this.lanes=0,this.alternate=null}function Ba(e,t,a,n){return new or(e,t,a,n)}function vs(e){return e=e.prototype,!(!e||!e.isReactComponent)}function tn(e,t){var a=e.alternate;return a===null?(a=Ba(e.tag,t,e.key,e.mode),a.elementType=e.elementType,a.type=e.type,a.stateNode=e.stateNode,a.alternate=e,e.alternate=a):(a.pendingProps=t,a.type=e.type,a.flags=0,a.subtreeFlags=0,a.deletions=null),a.flags=e.flags&65011712,a.childLanes=e.childLanes,a.lanes=e.lanes,a.child=e.child,a.memoizedProps=e.memoizedProps,a.memoizedState=e.memoizedState,a.updateQueue=e.updateQueue,t=e.dependencies,a.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext},a.sibling=e.sibling,a.index=e.index,a.ref=e.ref,a.refCleanup=e.refCleanup,a}function lr(e,t){e.flags&=65011714;var a=e.alternate;return a===null?(e.childLanes=0,e.lanes=t,e.child=null,e.subtreeFlags=0,e.memoizedProps=null,e.memoizedState=null,e.updateQueue=null,e.dependencies=null,e.stateNode=null):(e.childLanes=a.childLanes,e.lanes=a.lanes,e.child=a.child,e.subtreeFlags=0,e.deletions=null,e.memoizedProps=a.memoizedProps,e.memoizedState=a.memoizedState,e.updateQueue=a.updateQueue,e.type=a.type,t=a.dependencies,e.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext}),e}function gs(e,t,a,n,s,u){var p=0;if(n=e,typeof e=="function")vs(e)&&(p=1);else if(typeof e=="string")p=jv(e,a,V.current)?26:e==="html"||e==="head"||e==="body"?27:5;else e:switch(e){case se:return e=Ba(31,a,t,s),e.elementType=se,e.lanes=u,e;case B:return Kn(a.children,s,u,t);case C:p=8,s|=24;break;case H:return e=Ba(12,a,t,s|2),e.elementType=H,e.lanes=u,e;case I:return e=Ba(13,a,t,s),e.elementType=I,e.lanes=u,e;case Z:return e=Ba(19,a,t,s),e.elementType=Z,e.lanes=u,e;default:if(typeof e=="object"&&e!==null)switch(e.$$typeof){case U:p=10;break e;case K:p=9;break e;case _:p=11;break e;case le:p=14;break e;case X:p=16,n=null;break e}p=29,a=Error(c(130,e===null?"null":typeof e,"")),n=null}return t=Ba(p,a,t,s),t.elementType=e,t.type=n,t.lanes=u,t}function Kn(e,t,a,n){return e=Ba(7,e,n,t),e.lanes=a,e}function so(e,t,a){return e=Ba(6,e,null,t),e.lanes=a,e}function sr(e){var t=Ba(18,null,null,0);return t.stateNode=e,t}function ir(e,t,a){return t=Ba(4,e.children!==null?e.children:[],e.key,t),t.lanes=a,t.stateNode={containerInfo:e.containerInfo,pendingChildren:null,implementation:e.implementation},t}var i0=new WeakMap;function ga(e,t){if(typeof e=="object"&&e!==null){var a=i0.get(e);return a!==void 0?a:(t={value:e,source:t,stack:Ft(t)},i0.set(e,t),t)}return{value:e,source:t,stack:Ft(t)}}var Vo=[],ys=0,xs=null,Ko=0,Ha=[],La=0,Po=null,an=1,nn="";function bn(e,t){Vo[ys++]=Ko,Vo[ys++]=xs,xs=e,Ko=t}function oc(e,t,a){Ha[La++]=an,Ha[La++]=nn,Ha[La++]=Po,Po=e;var n=an;e=nn;var s=32-ha(n)-1;n&=~(1<<s),a+=1;var u=32-ha(t)+s;if(30<u){var p=s-s%5;u=(n&(1<<p)-1).toString(32),n>>=p,s-=p,an=1<<32-ha(t)+s|a<<s|n,nn=u+e}else an=1<<u|a<<s|n,nn=e}function rr(e){e.return!==null&&(bn(e,1),oc(e,1,0))}function cr(e){for(;e===xs;)xs=Vo[--ys],Vo[ys]=null,Ko=Vo[--ys],Vo[ys]=null;for(;e===Po;)Po=Ha[--La],Ha[La]=null,nn=Ha[--La],Ha[La]=null,an=Ha[--La],Ha[La]=null}function lc(e,t){Ha[La++]=an,Ha[La++]=nn,Ha[La++]=Po,an=t.id,nn=t.overflow,Po=e}var ia=null,Ht=null,vt=!1,Yo=null,Xa=!1,di=Error(c(519));function Mn(e){var t=Error(c(418,1<arguments.length&&arguments[1]!==void 0&&arguments[1]?"text":"HTML",""));throw Al(ga(t,e)),di}function St(e){var t=e.stateNode,a=e.type,n=e.memoizedProps;switch(t[oa]=e,t[ma]=n,a){case"dialog":yt("cancel",t),yt("close",t);break;case"iframe":case"object":case"embed":yt("load",t);break;case"video":case"audio":for(a=0;a<Sc.length;a++)yt(Sc[a],t);break;case"source":yt("error",t);break;case"img":case"image":case"link":yt("error",t),yt("load",t);break;case"details":yt("toggle",t);break;case"input":yt("invalid",t),ls(t,n.value,n.defaultValue,n.checked,n.defaultChecked,n.type,n.name,!0);break;case"select":yt("invalid",t);break;case"textarea":yt("invalid",t),dl(t,n.value,n.defaultValue,n.children)}a=n.children,typeof a!="string"&&typeof a!="number"&&typeof a!="bigint"||t.textContent===""+a||n.suppressHydrationWarning===!0||Um(t.textContent,a)?(n.popover!=null&&(yt("beforetoggle",t),yt("toggle",t)),n.onScroll!=null&&yt("scroll",t),n.onScrollEnd!=null&&yt("scrollend",t),n.onClick!=null&&(t.onclick=no),t=!0):t=!1,t||Mn(e,!0)}function sc(e){for(ia=e.return;ia;)switch(ia.tag){case 5:case 31:case 13:Xa=!1;return;case 27:case 3:Xa=!0;return;default:ia=ia.return}}function io(e){if(e!==ia)return!1;if(!vt)return sc(e),vt=!0,!1;var t=e.tag,a;if((a=t!==3&&t!==27)&&((a=t===5)&&(a=e.type,a=!(a!=="form"&&a!=="button")||kf(e.type,e.memoizedProps)),a=!a),a&&Ht&&Mn(e),sc(e),t===13){if(e=e.memoizedState,e=e!==null?e.dehydrated:null,!e)throw Error(c(317));Ht=Vm(e)}else if(t===31){if(e=e.memoizedState,e=e!==null?e.dehydrated:null,!e)throw Error(c(317));Ht=Vm(e)}else t===27?(t=Ht,ks(e.type)?(e=Lf,Lf=null,Ht=e):Ht=t):Ht=ia?Qn(e.stateNode.nextSibling):null;return!0}function ro(){Ht=ia=null,vt=!1}function Sn(){var e=Yo;return e!==null&&(hn===null?hn=e:hn.push.apply(hn,e),Yo=null),e}function Al(e){Yo===null?Yo=[e]:Yo.push(e)}var ur=S(null),co=null,on=null;function Pn(e,t,a){ve(ur,t._currentValue),t._currentValue=a}function Yn(e){e._currentValue=ur.current,Q(ur)}function El(e,t,a){for(;e!==null;){var n=e.alternate;if((e.childLanes&t)!==t?(e.childLanes|=t,n!==null&&(n.childLanes|=t)):n!==null&&(n.childLanes&t)!==t&&(n.childLanes|=t),e===a)break;e=e.return}}function Rl(e,t,a,n){var s=e.child;for(s!==null&&(s.return=e);s!==null;){var u=s.dependencies;if(u!==null){var p=s.child;u=u.firstContext;e:for(;u!==null;){var w=u;u=s;for(var q=0;q<t.length;q++)if(w.context===t[q]){u.lanes|=a,w=u.alternate,w!==null&&(w.lanes|=a),El(u.return,a,e),n||(p=null);break e}u=w.next}}else if(s.tag===18){if(p=s.return,p===null)throw Error(c(341));p.lanes|=a,u=p.alternate,u!==null&&(u.lanes|=a),El(p,a,e),p=null}else p=s.child;if(p!==null)p.return=s;else for(p=s;p!==null;){if(p===e){p=null;break}if(s=p.sibling,s!==null){s.return=p.return,p=s;break}p=p.return}s=p}}function Tn(e,t,a,n){e=null;for(var s=t,u=!1;s!==null;){if(!u){if((s.flags&524288)!==0)u=!0;else if((s.flags&262144)!==0)break}if(s.tag===10){var p=s.alternate;if(p===null)throw Error(c(387));if(p=p.memoizedProps,p!==null){var w=s.type;va(s.pendingProps.value,p.value)||(e!==null?e.push(w):e=[w])}}else if(s===Ce.current){if(p=s.alternate,p===null)throw Error(c(387));p.memoizedState.memoizedState!==s.memoizedState.memoizedState&&(e!==null?e.push(Ac):e=[Ac])}s=s.return}e!==null&&Rl(t,e,a,n),t.flags|=262144}function fr(e){for(e=e.firstContext;e!==null;){if(!va(e.context._currentValue,e.memoizedValue))return!0;e=e.next}return!1}function uo(e){co=e,on=null,e=e.dependencies,e!==null&&(e.firstContext=null)}function ra(e){return fo(co,e)}function mi(e,t){return co===null&&uo(e),fo(e,t)}function fo(e,t){var a=t._currentValue;if(t={context:t,memoizedValue:a,next:null},on===null){if(e===null)throw Error(c(308));on=t,e.dependencies={lanes:0,firstContext:t},e.flags|=524288}else on=on.next=t;return a}var Wo=typeof AbortController<"u"?AbortController:function(){var e=[],t=this.signal={aborted:!1,addEventListener:function(a,n){e.push(n)}};this.abort=function(){t.aborted=!0,e.forEach(function(a){return a()})}},ic=o.unstable_scheduleCallback,r0=o.unstable_NormalPriority,Kt={$$typeof:U,Consumer:null,Provider:null,_currentValue:null,_currentValue2:null,_threadCount:0};function pi(){return{controller:new Wo,data:new Map,refCount:0}}function Wn(e){e.refCount--,e.refCount===0&&ic(r0,function(){e.controller.abort()})}var ws=null,hr=0,ho=0,Nl=null;function Hu(e,t){if(ws===null){var a=ws=[];hr=0,ho=Tf(),Nl={status:"pending",value:void 0,then:function(n){a.push(n)}}}return hr++,t.then(vi,vi),t}function vi(){if(--hr===0&&ws!==null){Nl!==null&&(Nl.status="fulfilled");var e=ws;ws=null,ho=0,Nl=null;for(var t=0;t<e.length;t++)(0,e[t])()}}function c0(e,t){var a=[],n={status:"pending",value:null,reason:null,then:function(s){a.push(s)}};return e.then(function(){n.status="fulfilled",n.value=t;for(var s=0;s<a.length;s++)(0,a[s])(t)},function(s){for(n.status="rejected",n.reason=s,s=0;s<a.length;s++)(0,a[s])(void 0)}),n}var rc=N.S;N.S=function(e,t){om=Dt(),typeof t=="object"&&t!==null&&typeof t.then=="function"&&Hu(e,t),rc!==null&&rc(e,t)};var Cn=S(null);function dr(){var e=Cn.current;return e!==null?e:jt.pooledCache}function gi(e,t){t===null?ve(Cn,Cn.current):ve(Cn,t.pool)}function u0(){var e=dr();return e===null?null:{parent:Kt._currentValue,pool:e}}var mo=Error(c(460)),Za=Error(c(474)),Dl=Error(c(542)),bs={then:function(){}};function cc(e){return e=e.status,e==="fulfilled"||e==="rejected"}function _n(e,t,a){switch(a=e[a],a===void 0?e.push(t):a!==t&&(t.then(no,no),t=a),t.status){case"fulfilled":return t.value;case"rejected":throw e=t.reason,uc(e),e;default:if(typeof t.status=="string")t.then(no,no);else{if(e=jt,e!==null&&100<e.shellSuspendCounter)throw Error(c(482));e=t,e.status="pending",e.then(function(n){if(t.status==="pending"){var s=t;s.status="fulfilled",s.value=n}},function(n){if(t.status==="pending"){var s=t;s.status="rejected",s.reason=n}})}switch(t.status){case"fulfilled":return t.value;case"rejected":throw e=t.reason,uc(e),e}throw po=t,mo}}function Xo(e){try{var t=e._init;return t(e._payload)}catch(a){throw a!==null&&typeof a=="object"&&typeof a.then=="function"?(po=a,mo):a}}var po=null;function ln(){if(po===null)throw Error(c(459));var e=po;return po=null,e}function uc(e){if(e===mo||e===Dl)throw Error(c(483))}var Ms=null,Ss=0;function yi(e){var t=Ss;return Ss+=1,Ms===null&&(Ms=[]),_n(Ms,e,t)}function y(e,t){t=t.props.ref,e.ref=t!==void 0?t:null}function v(e,t){throw t.$$typeof===T?Error(c(525)):(e=Object.prototype.toString.call(t),Error(c(31,e==="[object Object]"?"object with keys {"+Object.keys(t).join(", ")+"}":e)))}function b(e){function t(ae,W){if(e){var ce=ae.deletions;ce===null?(ae.deletions=[W],ae.flags|=16):ce.push(W)}}function a(ae,W){if(!e)return null;for(;W!==null;)t(ae,W),W=W.sibling;return null}function n(ae){for(var W=new Map;ae!==null;)ae.key!==null?W.set(ae.key,ae):W.set(ae.index,ae),ae=ae.sibling;return W}function s(ae,W){return ae=tn(ae,W),ae.index=0,ae.sibling=null,ae}function u(ae,W,ce){return ae.index=ce,e?(ce=ae.alternate,ce!==null?(ce=ce.index,ce<W?(ae.flags|=67108866,W):ce):(ae.flags|=67108866,W)):(ae.flags|=1048576,W)}function p(ae){return e&&ae.alternate===null&&(ae.flags|=67108866),ae}function w(ae,W,ce,ze){return W===null||W.tag!==6?(W=so(ce,ae.mode,ze),W.return=ae,W):(W=s(W,ce),W.return=ae,W)}function q(ae,W,ce,ze){var et=ce.type;return et===B?Te(ae,W,ce.props.children,ze,ce.key):W!==null&&(W.elementType===et||typeof et=="object"&&et!==null&&et.$$typeof===X&&Xo(et)===W.type)?(W=s(W,ce.props),y(W,ce),W.return=ae,W):(W=gs(ce.type,ce.key,ce.props,null,ae.mode,ze),y(W,ce),W.return=ae,W)}function fe(ae,W,ce,ze){return W===null||W.tag!==4||W.stateNode.containerInfo!==ce.containerInfo||W.stateNode.implementation!==ce.implementation?(W=ir(ce,ae.mode,ze),W.return=ae,W):(W=s(W,ce.children||[]),W.return=ae,W)}function Te(ae,W,ce,ze,et){return W===null||W.tag!==7?(W=Kn(ce,ae.mode,ze,et),W.return=ae,W):(W=s(W,ce),W.return=ae,W)}function Re(ae,W,ce){if(typeof W=="string"&&W!==""||typeof W=="number"||typeof W=="bigint")return W=so(""+W,ae.mode,ce),W.return=ae,W;if(typeof W=="object"&&W!==null){switch(W.$$typeof){case A:return ce=gs(W.type,W.key,W.props,null,ae.mode,ce),y(ce,W),ce.return=ae,ce;case R:return W=ir(W,ae.mode,ce),W.return=ae,W;case X:return W=Xo(W),Re(ae,W,ce)}if($(W)||G(W))return W=Kn(W,ae.mode,ce,null),W.return=ae,W;if(typeof W.then=="function")return Re(ae,yi(W),ce);if(W.$$typeof===U)return Re(ae,mi(ae,W),ce);v(ae,W)}return null}function de(ae,W,ce,ze){var et=W!==null?W.key:null;if(typeof ce=="string"&&ce!==""||typeof ce=="number"||typeof ce=="bigint")return et!==null?null:w(ae,W,""+ce,ze);if(typeof ce=="object"&&ce!==null){switch(ce.$$typeof){case A:return ce.key===et?q(ae,W,ce,ze):null;case R:return ce.key===et?fe(ae,W,ce,ze):null;case X:return ce=Xo(ce),de(ae,W,ce,ze)}if($(ce)||G(ce))return et!==null?null:Te(ae,W,ce,ze,null);if(typeof ce.then=="function")return de(ae,W,yi(ce),ze);if(ce.$$typeof===U)return de(ae,W,mi(ae,ce),ze);v(ae,ce)}return null}function xe(ae,W,ce,ze,et){if(typeof ze=="string"&&ze!==""||typeof ze=="number"||typeof ze=="bigint")return ae=ae.get(ce)||null,w(W,ae,""+ze,et);if(typeof ze=="object"&&ze!==null){switch(ze.$$typeof){case A:return ae=ae.get(ze.key===null?ce:ze.key)||null,q(W,ae,ze,et);case R:return ae=ae.get(ze.key===null?ce:ze.key)||null,fe(W,ae,ze,et);case X:return ze=Xo(ze),xe(ae,W,ce,ze,et)}if($(ze)||G(ze))return ae=ae.get(ce)||null,Te(W,ae,ze,et,null);if(typeof ze.then=="function")return xe(ae,W,ce,yi(ze),et);if(ze.$$typeof===U)return xe(ae,W,ce,mi(W,ze),et);v(W,ze)}return null}function We(ae,W,ce,ze){for(var et=null,Ct=null,Qe=W,ht=W=0,bt=null;Qe!==null&&ht<ce.length;ht++){Qe.index>ht?(bt=Qe,Qe=null):bt=Qe.sibling;var _t=de(ae,Qe,ce[ht],ze);if(_t===null){Qe===null&&(Qe=bt);break}e&&Qe&&_t.alternate===null&&t(ae,Qe),W=u(_t,W,ht),Ct===null?et=_t:Ct.sibling=_t,Ct=_t,Qe=bt}if(ht===ce.length)return a(ae,Qe),vt&&bn(ae,ht),et;if(Qe===null){for(;ht<ce.length;ht++)Qe=Re(ae,ce[ht],ze),Qe!==null&&(W=u(Qe,W,ht),Ct===null?et=Qe:Ct.sibling=Qe,Ct=Qe);return vt&&bn(ae,ht),et}for(Qe=n(Qe);ht<ce.length;ht++)bt=xe(Qe,ae,ht,ce[ht],ze),bt!==null&&(e&&bt.alternate!==null&&Qe.delete(bt.key===null?ht:bt.key),W=u(bt,W,ht),Ct===null?et=bt:Ct.sibling=bt,Ct=bt);return e&&Qe.forEach(function(Ls){return t(ae,Ls)}),vt&&bn(ae,ht),et}function at(ae,W,ce,ze){if(ce==null)throw Error(c(151));for(var et=null,Ct=null,Qe=W,ht=W=0,bt=null,_t=ce.next();Qe!==null&&!_t.done;ht++,_t=ce.next()){Qe.index>ht?(bt=Qe,Qe=null):bt=Qe.sibling;var Ls=de(ae,Qe,_t.value,ze);if(Ls===null){Qe===null&&(Qe=bt);break}e&&Qe&&Ls.alternate===null&&t(ae,Qe),W=u(Ls,W,ht),Ct===null?et=Ls:Ct.sibling=Ls,Ct=Ls,Qe=bt}if(_t.done)return a(ae,Qe),vt&&bn(ae,ht),et;if(Qe===null){for(;!_t.done;ht++,_t=ce.next())_t=Re(ae,_t.value,ze),_t!==null&&(W=u(_t,W,ht),Ct===null?et=_t:Ct.sibling=_t,Ct=_t);return vt&&bn(ae,ht),et}for(Qe=n(Qe);!_t.done;ht++,_t=ce.next())_t=xe(Qe,ae,ht,_t.value,ze),_t!==null&&(e&&_t.alternate!==null&&Qe.delete(_t.key===null?ht:_t.key),W=u(_t,W,ht),Ct===null?et=_t:Ct.sibling=_t,Ct=_t);return e&&Qe.forEach(function(Qv){return t(ae,Qv)}),vt&&bn(ae,ht),et}function Bt(ae,W,ce,ze){if(typeof ce=="object"&&ce!==null&&ce.type===B&&ce.key===null&&(ce=ce.props.children),typeof ce=="object"&&ce!==null){switch(ce.$$typeof){case A:e:{for(var et=ce.key;W!==null;){if(W.key===et){if(et=ce.type,et===B){if(W.tag===7){a(ae,W.sibling),ze=s(W,ce.props.children),ze.return=ae,ae=ze;break e}}else if(W.elementType===et||typeof et=="object"&&et!==null&&et.$$typeof===X&&Xo(et)===W.type){a(ae,W.sibling),ze=s(W,ce.props),y(ze,ce),ze.return=ae,ae=ze;break e}a(ae,W);break}else t(ae,W);W=W.sibling}ce.type===B?(ze=Kn(ce.props.children,ae.mode,ze,ce.key),ze.return=ae,ae=ze):(ze=gs(ce.type,ce.key,ce.props,null,ae.mode,ze),y(ze,ce),ze.return=ae,ae=ze)}return p(ae);case R:e:{for(et=ce.key;W!==null;){if(W.key===et)if(W.tag===4&&W.stateNode.containerInfo===ce.containerInfo&&W.stateNode.implementation===ce.implementation){a(ae,W.sibling),ze=s(W,ce.children||[]),ze.return=ae,ae=ze;break e}else{a(ae,W);break}else t(ae,W);W=W.sibling}ze=ir(ce,ae.mode,ze),ze.return=ae,ae=ze}return p(ae);case X:return ce=Xo(ce),Bt(ae,W,ce,ze)}if($(ce))return We(ae,W,ce,ze);if(G(ce)){if(et=G(ce),typeof et!="function")throw Error(c(150));return ce=et.call(ce),at(ae,W,ce,ze)}if(typeof ce.then=="function")return Bt(ae,W,yi(ce),ze);if(ce.$$typeof===U)return Bt(ae,W,mi(ae,ce),ze);v(ae,ce)}return typeof ce=="string"&&ce!==""||typeof ce=="number"||typeof ce=="bigint"?(ce=""+ce,W!==null&&W.tag===6?(a(ae,W.sibling),ze=s(W,ce),ze.return=ae,ae=ze):(a(ae,W),ze=so(ce,ae.mode,ze),ze.return=ae,ae=ze),p(ae)):a(ae,W)}return function(ae,W,ce,ze){try{Ss=0;var et=Bt(ae,W,ce,ze);return Ms=null,et}catch(Qe){if(Qe===mo||Qe===Dl)throw Qe;var Ct=Ba(29,Qe,null,ae.mode);return Ct.lanes=ze,Ct.return=ae,Ct}finally{}}}var z=b(!0),D=b(!1),L=!1;function E(e){e.updateQueue={baseState:e.memoizedState,firstBaseUpdate:null,lastBaseUpdate:null,shared:{pending:null,lanes:0,hiddenCallbacks:null},callbacks:null}}function j(e,t){e=e.updateQueue,t.updateQueue===e&&(t.updateQueue={baseState:e.baseState,firstBaseUpdate:e.firstBaseUpdate,lastBaseUpdate:e.lastBaseUpdate,shared:e.shared,callbacks:null})}function Y(e){return{lane:e,tag:0,payload:null,callback:null,next:null}}function ne(e,t,a){var n=e.updateQueue;if(n===null)return null;if(n=n.shared,(At&2)!==0){var s=n.pending;return s===null?t.next=t:(t.next=s.next,s.next=t),n.pending=t,t=hi(e),zl(e,null,a),t}return ps(e,n,t,a),hi(e)}function te(e,t,a){if(t=t.updateQueue,t!==null&&(t=t.shared,(a&4194048)!==0)){var n=t.lanes;n&=e.pendingLanes,a|=n,t.lanes=a,Di(e,a)}}function me(e,t){var a=e.updateQueue,n=e.alternate;if(n!==null&&(n=n.updateQueue,a===n)){var s=null,u=null;if(a=a.firstBaseUpdate,a!==null){do{var p={lane:a.lane,tag:a.tag,payload:a.payload,callback:null,next:null};u===null?s=u=p:u=u.next=p,a=a.next}while(a!==null);u===null?s=u=t:u=u.next=t}else s=u=t;a={baseState:n.baseState,firstBaseUpdate:s,lastBaseUpdate:u,shared:n.shared,callbacks:n.callbacks},e.updateQueue=a;return}e=a.lastBaseUpdate,e===null?a.firstBaseUpdate=t:e.next=t,a.lastBaseUpdate=t}var ye=!1;function he(){if(ye){var e=Nl;if(e!==null)throw e}}function be(e,t,a,n){ye=!1;var s=e.updateQueue;L=!1;var u=s.firstBaseUpdate,p=s.lastBaseUpdate,w=s.shared.pending;if(w!==null){s.shared.pending=null;var q=w,fe=q.next;q.next=null,p===null?u=fe:p.next=fe,p=q;var Te=e.alternate;Te!==null&&(Te=Te.updateQueue,w=Te.lastBaseUpdate,w!==p&&(w===null?Te.firstBaseUpdate=fe:w.next=fe,Te.lastBaseUpdate=q))}if(u!==null){var Re=s.baseState;p=0,Te=fe=q=null,w=u;do{var de=w.lane&-536870913,xe=de!==w.lane;if(xe?(wt&de)===de:(n&de)===de){de!==0&&de===ho&&(ye=!0),Te!==null&&(Te=Te.next={lane:0,tag:w.tag,payload:w.payload,callback:null,next:null});e:{var We=e,at=w;de=t;var Bt=a;switch(at.tag){case 1:if(We=at.payload,typeof We=="function"){Re=We.call(Bt,Re,de);break e}Re=We;break e;case 3:We.flags=We.flags&-65537|128;case 0:if(We=at.payload,de=typeof We=="function"?We.call(Bt,Re,de):We,de==null)break e;Re=M({},Re,de);break e;case 2:L=!0}}de=w.callback,de!==null&&(e.flags|=64,xe&&(e.flags|=8192),xe=s.callbacks,xe===null?s.callbacks=[de]:xe.push(de))}else xe={lane:de,tag:w.tag,payload:w.payload,callback:w.callback,next:null},Te===null?(fe=Te=xe,q=Re):Te=Te.next=xe,p|=de;if(w=w.next,w===null){if(w=s.shared.pending,w===null)break;xe=w,w=xe.next,xe.next=null,s.lastBaseUpdate=xe,s.shared.pending=null}}while(!0);Te===null&&(q=Re),s.baseState=q,s.firstBaseUpdate=fe,s.lastBaseUpdate=Te,u===null&&(s.shared.lanes=0),As|=p,e.lanes=p,e.memoizedState=Re}}function Ee(e,t){if(typeof e!="function")throw Error(c(191,e));e.call(t)}function Se(e,t){var a=e.callbacks;if(a!==null)for(e.callbacks=null,e=0;e<a.length;e++)Ee(a[e],t)}var Ue=S(null),Le=S(0);function Ae(e,t){e=jl,ve(Le,e),ve(Ue,t),jl=e|t.baseLanes}function ke(){ve(Le,jl),ve(Ue,Ue.current)}function qe(){jl=Le.current,Q(Ue),Q(Le)}var Oe=S(null),Fe=null;function Ge(e){var t=e.alternate;ve(Xe,Xe.current&1),ve(Oe,e),Fe===null&&(t===null||Ue.current!==null||t.memoizedState!==null)&&(Fe=e)}function De(e){ve(Xe,Xe.current),ve(Oe,e),Fe===null&&(Fe=e)}function He(e){e.tag===22?(ve(Xe,Xe.current),ve(Oe,e),Fe===null&&(Fe=e)):Ke()}function Ke(){ve(Xe,Xe.current),ve(Oe,Oe.current)}function Be(e){Q(Oe),Fe===e&&(Fe=null),Q(Xe)}var Xe=S(0);function rt(e){for(var t=e;t!==null;){if(t.tag===13){var a=t.memoizedState;if(a!==null&&(a=a.dehydrated,a===null||Bf(a)||Hf(a)))return t}else if(t.tag===19&&(t.memoizedProps.revealOrder==="forwards"||t.memoizedProps.revealOrder==="backwards"||t.memoizedProps.revealOrder==="unstable_legacy-backwards"||t.memoizedProps.revealOrder==="together")){if((t.flags&128)!==0)return t}else if(t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return null;t=t.return}t.sibling.return=t.return,t=t.sibling}return null}var $e=0,je=null,Ze=null,ot=null,Tt=!1,Lt=!1,Rt=!1,Gt=0,Qt=0,ya=null,sn=0;function xt(){throw Error(c(321))}function Sa(e,t){if(t===null)return!1;for(var a=0;a<t.length&&a<e.length;a++)if(!va(e[a],t[a]))return!1;return!0}function Qa(e,t,a,n,s,u){return $e=u,je=t,t.memoizedState=null,t.updateQueue=null,t.lanes=0,N.H=e===null||e.memoizedState===null?pd:Yu,Rt=!1,u=a(n,s),Rt=!1,Lt&&(u=vo(t,a,n,s)),Zo(e),u}function Zo(e){N.H=hc;var t=Ze!==null&&Ze.next!==null;if($e=0,ot=Ze=je=null,Tt=!1,Qt=0,ya=null,t)throw Error(c(300));e===null||ca||(e=e.dependencies,e!==null&&fr(e)&&(ca=!0))}function vo(e,t,a,n){je=e;var s=0;do{if(Lt&&(ya=null),Qt=0,Lt=!1,25<=s)throw Error(c(301));if(s+=1,ot=Ze=null,e.updateQueue!=null){var u=e.updateQueue;u.lastEffect=null,u.events=null,u.stores=null,u.memoCache!=null&&(u.memoCache.index=0)}N.H=vd,u=t(a,n)}while(Lt);return u}function Ts(){var e=N.H,t=e.useState()[0];return t=typeof t.then=="function"?Xn(t):t,e=e.useState()[0],(Ze!==null?Ze.memoizedState:null)!==e&&(je.flags|=1024),t}function ja(){var e=Gt!==0;return Gt=0,e}function rn(e,t,a){t.updateQueue=e.updateQueue,t.flags&=-2053,e.lanes&=~a}function Cs(e){if(Tt){for(e=e.memoizedState;e!==null;){var t=e.queue;t!==null&&(t.pending=null),e=e.next}Tt=!1}$e=0,ot=Ze=je=null,Lt=!1,Qt=Gt=0,ya=null}function xa(){var e={memoizedState:null,baseState:null,baseQueue:null,queue:null,next:null};return ot===null?je.memoizedState=ot=e:ot=ot.next=e,ot}function qt(){if(Ze===null){var e=je.alternate;e=e!==null?e.memoizedState:null}else e=Ze.next;var t=ot===null?je.memoizedState:ot.next;if(t!==null)ot=t,Ze=e;else{if(e===null)throw je.alternate===null?Error(c(467)):Error(c(310));Ze=e,e={memoizedState:Ze.memoizedState,baseState:Ze.baseState,baseQueue:Ze.baseQueue,queue:Ze.queue,next:null},ot===null?je.memoizedState=ot=e:ot=ot.next=e}return ot}function go(){return{lastEffect:null,events:null,stores:null,memoCache:null}}function Xn(e){var t=Qt;return Qt+=1,ya===null&&(ya=[]),e=_n(ya,e,t),t=je,(ot===null?t.memoizedState:ot.next)===null&&(t=t.alternate,N.H=t===null||t.memoizedState===null?pd:Yu),e}function yo(e){if(e!==null&&typeof e=="object"){if(typeof e.then=="function")return Xn(e);if(e.$$typeof===U)return ra(e)}throw Error(c(438,String(e)))}function Qo(e){var t=null,a=je.updateQueue;if(a!==null&&(t=a.memoCache),t==null){var n=je.alternate;n!==null&&(n=n.updateQueue,n!==null&&(n=n.memoCache,n!=null&&(t={data:n.data.map(function(s){return s.slice()}),index:0})))}if(t==null&&(t={data:[],index:0}),a===null&&(a=go(),je.updateQueue=a),a.memoCache=t,a=t.data[t.index],a===void 0)for(a=t.data[t.index]=Array(e),n=0;n<e;n++)a[n]=we;return t.index++,a}function zn(e,t){return typeof t=="function"?t(e):t}function xi(e){var t=qt();return Zn(t,Ze,e)}function Zn(e,t,a){var n=e.queue;if(n===null)throw Error(c(311));n.lastRenderedReducer=a;var s=e.baseQueue,u=n.pending;if(u!==null){if(s!==null){var p=s.next;s.next=u.next,u.next=p}t.baseQueue=s=u,n.pending=null}if(u=e.baseState,s===null)e.memoizedState=u;else{t=s.next;var w=p=null,q=null,fe=t,Te=!1;do{var Re=fe.lane&-536870913;if(Re!==fe.lane?(wt&Re)===Re:($e&Re)===Re){var de=fe.revertLane;if(de===0)q!==null&&(q=q.next={lane:0,revertLane:0,gesture:null,action:fe.action,hasEagerState:fe.hasEagerState,eagerState:fe.eagerState,next:null}),Re===ho&&(Te=!0);else if(($e&de)===de){fe=fe.next,de===ho&&(Te=!0);continue}else Re={lane:0,revertLane:fe.revertLane,gesture:null,action:fe.action,hasEagerState:fe.hasEagerState,eagerState:fe.eagerState,next:null},q===null?(w=q=Re,p=u):q=q.next=Re,je.lanes|=de,As|=de;Re=fe.action,Rt&&a(u,Re),u=fe.hasEagerState?fe.eagerState:a(u,Re)}else de={lane:Re,revertLane:fe.revertLane,gesture:fe.gesture,action:fe.action,hasEagerState:fe.hasEagerState,eagerState:fe.eagerState,next:null},q===null?(w=q=de,p=u):q=q.next=de,je.lanes|=Re,As|=Re;fe=fe.next}while(fe!==null&&fe!==t);if(q===null?p=u:q.next=w,!va(u,e.memoizedState)&&(ca=!0,Te&&(a=Nl,a!==null)))throw a;e.memoizedState=u,e.baseState=p,e.baseQueue=q,n.lastRenderedState=u}return s===null&&(n.lanes=0),[e.memoizedState,n.dispatch]}function wi(e){var t=qt(),a=t.queue;if(a===null)throw Error(c(311));a.lastRenderedReducer=e;var n=a.dispatch,s=a.pending,u=t.memoizedState;if(s!==null){a.pending=null;var p=s=s.next;do u=e(u,p.action),p=p.next;while(p!==s);va(u,t.memoizedState)||(ca=!0),t.memoizedState=u,t.baseQueue===null&&(t.baseState=u),a.lastRenderedState=u}return[u,n]}function mr(e,t,a){var n=je,s=qt(),u=vt;if(u){if(a===void 0)throw Error(c(407));a=a()}else a=t();var p=!va((Ze||s).memoizedState,a);if(p&&(s.memoizedState=a,ca=!0),s=s.queue,Fu(Lh.bind(null,n,s,e),[e]),s.getSnapshot!==t||p||ot!==null&&ot.memoizedState.tag&1){if(n.flags|=2048,pr(9,{destroy:void 0},Hh.bind(null,n,s,a,t),null),jt===null)throw Error(c(349));u||($e&127)!==0||Bh(n,t,a)}return a}function Bh(e,t,a){e.flags|=16384,e={getSnapshot:t,value:a},t=je.updateQueue,t===null?(t=go(),je.updateQueue=t,t.stores=[e]):(a=t.stores,a===null?t.stores=[e]:a.push(e))}function Hh(e,t,a,n){t.value=a,t.getSnapshot=n,jh(t)&&Fh(e)}function Lh(e,t,a){return a(function(){jh(t)&&Fh(e)})}function jh(e){var t=e.getSnapshot;e=e.value;try{var a=t();return!va(e,a)}catch{return!0}}function Fh(e){var t=Vn(e,2);t!==null&&dn(t,e,2)}function Lu(e){var t=xa();if(typeof e=="function"){var a=e;if(e=a(),Rt){yn(!0);try{a()}finally{yn(!1)}}}return t.memoizedState=t.baseState=e,t.queue={pending:null,lanes:0,dispatch:null,lastRenderedReducer:zn,lastRenderedState:e},t}function Gh(e,t,a,n){return e.baseState=a,Zn(e,Ze,typeof n=="function"?n:zn)}function qp(e,t,a,n,s){if(d0(e))throw Error(c(485));if(e=t.action,e!==null){var u={payload:s,action:e,next:null,isTransition:!0,status:"pending",value:null,reason:null,listeners:[],then:function(p){u.listeners.push(p)}};N.T!==null?a(!0):u.isTransition=!1,n(u),a=t.pending,a===null?(u.next=t.pending=u,qh(t,u)):(u.next=a.next,t.pending=a.next=u)}}function qh(e,t){var a=t.action,n=t.payload,s=e.state;if(t.isTransition){var u=N.T,p={};N.T=p;try{var w=a(s,n),q=N.S;q!==null&&q(p,w),Vh(e,t,w)}catch(fe){ju(e,t,fe)}finally{u!==null&&p.types!==null&&(u.types=p.types),N.T=u}}else try{u=a(s,n),Vh(e,t,u)}catch(fe){ju(e,t,fe)}}function Vh(e,t,a){a!==null&&typeof a=="object"&&typeof a.then=="function"?a.then(function(n){Kh(e,t,n)},function(n){return ju(e,t,n)}):Kh(e,t,a)}function Kh(e,t,a){t.status="fulfilled",t.value=a,Ph(t),e.state=a,t=e.pending,t!==null&&(a=t.next,a===t?e.pending=null:(a=a.next,t.next=a,qh(e,a)))}function ju(e,t,a){var n=e.pending;if(e.pending=null,n!==null){n=n.next;do t.status="rejected",t.reason=a,Ph(t),t=t.next;while(t!==n)}e.action=null}function Ph(e){e=e.listeners;for(var t=0;t<e.length;t++)(0,e[t])()}function Yh(e,t){return t}function Wh(e,t){if(vt){var a=jt.formState;if(a!==null){e:{var n=je;if(vt){if(Ht){t:{for(var s=Ht,u=Xa;s.nodeType!==8;){if(!u){s=null;break t}if(s=Qn(s.nextSibling),s===null){s=null;break t}}u=s.data,s=u==="F!"||u==="F"?s:null}if(s){Ht=Qn(s.nextSibling),n=s.data==="F!";break e}}Mn(n)}n=!1}n&&(t=a[0])}}return a=xa(),a.memoizedState=a.baseState=t,n={pending:null,lanes:0,dispatch:null,lastRenderedReducer:Yh,lastRenderedState:t},a.queue=n,a=hd.bind(null,je,n),n.dispatch=a,n=Lu(!1),u=Pu.bind(null,je,!1,n.queue),n=xa(),s={state:t,dispatch:null,action:e,pending:null},n.queue=s,a=qp.bind(null,je,s,u,a),s.dispatch=a,n.memoizedState=e,[t,a,!1]}function Xh(e){var t=qt();return Zh(t,Ze,e)}function Zh(e,t,a){if(t=Zn(e,t,Yh)[0],e=xi(zn)[0],typeof t=="object"&&t!==null&&typeof t.then=="function")try{var n=Xn(t)}catch(p){throw p===mo?Dl:p}else n=t;t=qt();var s=t.queue,u=s.dispatch;return a!==t.memoizedState&&(je.flags|=2048,pr(9,{destroy:void 0},Vp.bind(null,s,a),null)),[n,u,e]}function Vp(e,t){e.action=t}function Qh(e){var t=qt(),a=Ze;if(a!==null)return Zh(t,a,e);qt(),t=t.memoizedState,a=qt();var n=a.queue.dispatch;return a.memoizedState=e,[t,n,!1]}function pr(e,t,a,n){return e={tag:e,create:a,deps:n,inst:t,next:null},t=je.updateQueue,t===null&&(t=go(),je.updateQueue=t),a=t.lastEffect,a===null?t.lastEffect=e.next=e:(n=a.next,a.next=e,e.next=n,t.lastEffect=e),e}function Ih(){return qt().memoizedState}function f0(e,t,a,n){var s=xa();je.flags|=e,s.memoizedState=pr(1|t,{destroy:void 0},a,n===void 0?null:n)}function h0(e,t,a,n){var s=qt();n=n===void 0?null:n;var u=s.memoizedState.inst;Ze!==null&&n!==null&&Sa(n,Ze.memoizedState.deps)?s.memoizedState=pr(t,u,a,n):(je.flags|=e,s.memoizedState=pr(1|t,u,a,n))}function Jh(e,t){f0(8390656,8,e,t)}function Fu(e,t){h0(2048,8,e,t)}function Kp(e){je.flags|=4;var t=je.updateQueue;if(t===null)t=go(),je.updateQueue=t,t.events=[e];else{var a=t.events;a===null?t.events=[e]:a.push(e)}}function $h(e){var t=qt().memoizedState;return Kp({ref:t,nextImpl:e}),function(){if((At&2)!==0)throw Error(c(440));return t.impl.apply(void 0,arguments)}}function ed(e,t){return h0(4,2,e,t)}function td(e,t){return h0(4,4,e,t)}function ad(e,t){if(typeof t=="function"){e=e();var a=t(e);return function(){typeof a=="function"?a():t(null)}}if(t!=null)return e=e(),t.current=e,function(){t.current=null}}function nd(e,t,a){a=a!=null?a.concat([e]):null,h0(4,4,ad.bind(null,t,e),a)}function Gu(){}function od(e,t){var a=qt();t=t===void 0?null:t;var n=a.memoizedState;return t!==null&&Sa(t,n[1])?n[0]:(a.memoizedState=[e,t],e)}function ld(e,t){var a=qt();t=t===void 0?null:t;var n=a.memoizedState;if(t!==null&&Sa(t,n[1]))return n[0];if(n=e(),Rt){yn(!0);try{e()}finally{yn(!1)}}return a.memoizedState=[n,t],n}function qu(e,t,a){return a===void 0||($e&1073741824)!==0&&(wt&261930)===0?e.memoizedState=t:(e.memoizedState=a,e=sm(),je.lanes|=e,As|=e,a)}function sd(e,t,a,n){return va(a,t)?a:Ue.current!==null?(e=qu(e,a,n),va(e,t)||(ca=!0),e):($e&42)===0||($e&1073741824)!==0&&(wt&261930)===0?(ca=!0,e.memoizedState=a):(e=sm(),je.lanes|=e,As|=e,t)}function id(e,t,a,n,s){var u=P.p;P.p=u!==0&&8>u?u:8;var p=N.T,w={};N.T=w,Pu(e,!1,t,a);try{var q=s(),fe=N.S;if(fe!==null&&fe(w,q),q!==null&&typeof q=="object"&&typeof q.then=="function"){var Te=c0(q,n);fc(e,t,Te,Rn(e))}else fc(e,t,n,Rn(e))}catch(Re){fc(e,t,{then:function(){},status:"rejected",reason:Re},Rn())}finally{P.p=u,p!==null&&w.types!==null&&(p.types=w.types),N.T=p}}function Pp(){}function Vu(e,t,a,n){if(e.tag!==5)throw Error(c(476));var s=rd(e).queue;id(e,s,t,k,a===null?Pp:function(){return cd(e),a(n)})}function rd(e){var t=e.memoizedState;if(t!==null)return t;t={memoizedState:k,baseState:k,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:zn,lastRenderedState:k},next:null};var a={};return t.next={memoizedState:a,baseState:a,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:zn,lastRenderedState:a},next:null},e.memoizedState=t,e=e.alternate,e!==null&&(e.memoizedState=t),t}function cd(e){var t=rd(e);t.next===null&&(t=e.alternate.memoizedState),fc(e,t.next.queue,{},Rn())}function Ku(){return ra(Ac)}function ud(){return qt().memoizedState}function fd(){return qt().memoizedState}function Yp(e){for(var t=e.return;t!==null;){switch(t.tag){case 24:case 3:var a=Rn();e=Y(a);var n=ne(t,e,a);n!==null&&(dn(n,t,a),te(n,t,a)),t={cache:pi()},e.payload=t;return}t=t.return}}function Wp(e,t,a){var n=Rn();a={lane:n,revertLane:0,gesture:null,action:a,hasEagerState:!1,eagerState:null,next:null},d0(e)?dd(t,a):(a=fi(e,t,a,n),a!==null&&(dn(a,e,n),md(a,t,n)))}function hd(e,t,a){var n=Rn();fc(e,t,a,n)}function fc(e,t,a,n){var s={lane:n,revertLane:0,gesture:null,action:a,hasEagerState:!1,eagerState:null,next:null};if(d0(e))dd(t,s);else{var u=e.alternate;if(e.lanes===0&&(u===null||u.lanes===0)&&(u=t.lastRenderedReducer,u!==null))try{var p=t.lastRenderedState,w=u(p,a);if(s.hasEagerState=!0,s.eagerState=w,va(w,p))return ps(e,t,s,0),jt===null&&ui(),!1}catch{}finally{}if(a=fi(e,t,s,n),a!==null)return dn(a,e,n),md(a,t,n),!0}return!1}function Pu(e,t,a,n){if(n={lane:2,revertLane:Tf(),gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null},d0(e)){if(t)throw Error(c(479))}else t=fi(e,a,n,2),t!==null&&dn(t,e,2)}function d0(e){var t=e.alternate;return e===je||t!==null&&t===je}function dd(e,t){Lt=Tt=!0;var a=e.pending;a===null?t.next=t:(t.next=a.next,a.next=t),e.pending=t}function md(e,t,a){if((a&4194048)!==0){var n=t.lanes;n&=e.pendingLanes,a|=n,t.lanes=a,Di(e,a)}}var hc={readContext:ra,use:yo,useCallback:xt,useContext:xt,useEffect:xt,useImperativeHandle:xt,useLayoutEffect:xt,useInsertionEffect:xt,useMemo:xt,useReducer:xt,useRef:xt,useState:xt,useDebugValue:xt,useDeferredValue:xt,useTransition:xt,useSyncExternalStore:xt,useId:xt,useHostTransitionStatus:xt,useFormState:xt,useActionState:xt,useOptimistic:xt,useMemoCache:xt,useCacheRefresh:xt};hc.useEffectEvent=xt;var pd={readContext:ra,use:yo,useCallback:function(e,t){return xa().memoizedState=[e,t===void 0?null:t],e},useContext:ra,useEffect:Jh,useImperativeHandle:function(e,t,a){a=a!=null?a.concat([e]):null,f0(4194308,4,ad.bind(null,t,e),a)},useLayoutEffect:function(e,t){return f0(4194308,4,e,t)},useInsertionEffect:function(e,t){f0(4,2,e,t)},useMemo:function(e,t){var a=xa();t=t===void 0?null:t;var n=e();if(Rt){yn(!0);try{e()}finally{yn(!1)}}return a.memoizedState=[n,t],n},useReducer:function(e,t,a){var n=xa();if(a!==void 0){var s=a(t);if(Rt){yn(!0);try{a(t)}finally{yn(!1)}}}else s=t;return n.memoizedState=n.baseState=s,e={pending:null,lanes:0,dispatch:null,lastRenderedReducer:e,lastRenderedState:s},n.queue=e,e=e.dispatch=Wp.bind(null,je,e),[n.memoizedState,e]},useRef:function(e){var t=xa();return e={current:e},t.memoizedState=e},useState:function(e){e=Lu(e);var t=e.queue,a=hd.bind(null,je,t);return t.dispatch=a,[e.memoizedState,a]},useDebugValue:Gu,useDeferredValue:function(e,t){var a=xa();return qu(a,e,t)},useTransition:function(){var e=Lu(!1);return e=id.bind(null,je,e.queue,!0,!1),xa().memoizedState=e,[!1,e]},useSyncExternalStore:function(e,t,a){var n=je,s=xa();if(vt){if(a===void 0)throw Error(c(407));a=a()}else{if(a=t(),jt===null)throw Error(c(349));(wt&127)!==0||Bh(n,t,a)}s.memoizedState=a;var u={value:a,getSnapshot:t};return s.queue=u,Jh(Lh.bind(null,n,u,e),[e]),n.flags|=2048,pr(9,{destroy:void 0},Hh.bind(null,n,u,a,t),null),a},useId:function(){var e=xa(),t=jt.identifierPrefix;if(vt){var a=nn,n=an;a=(n&~(1<<32-ha(n)-1)).toString(32)+a,t="_"+t+"R_"+a,a=Gt++,0<a&&(t+="H"+a.toString(32)),t+="_"}else a=sn++,t="_"+t+"r_"+a.toString(32)+"_";return e.memoizedState=t},useHostTransitionStatus:Ku,useFormState:Wh,useActionState:Wh,useOptimistic:function(e){var t=xa();t.memoizedState=t.baseState=e;var a={pending:null,lanes:0,dispatch:null,lastRenderedReducer:null,lastRenderedState:null};return t.queue=a,t=Pu.bind(null,je,!0,a),a.dispatch=t,[e,t]},useMemoCache:Qo,useCacheRefresh:function(){return xa().memoizedState=Yp.bind(null,je)},useEffectEvent:function(e){var t=xa(),a={impl:e};return t.memoizedState=a,function(){if((At&2)!==0)throw Error(c(440));return a.impl.apply(void 0,arguments)}}},Yu={readContext:ra,use:yo,useCallback:od,useContext:ra,useEffect:Fu,useImperativeHandle:nd,useInsertionEffect:ed,useLayoutEffect:td,useMemo:ld,useReducer:xi,useRef:Ih,useState:function(){return xi(zn)},useDebugValue:Gu,useDeferredValue:function(e,t){var a=qt();return sd(a,Ze.memoizedState,e,t)},useTransition:function(){var e=xi(zn)[0],t=qt().memoizedState;return[typeof e=="boolean"?e:Xn(e),t]},useSyncExternalStore:mr,useId:ud,useHostTransitionStatus:Ku,useFormState:Xh,useActionState:Xh,useOptimistic:function(e,t){var a=qt();return Gh(a,Ze,e,t)},useMemoCache:Qo,useCacheRefresh:fd};Yu.useEffectEvent=$h;var vd={readContext:ra,use:yo,useCallback:od,useContext:ra,useEffect:Fu,useImperativeHandle:nd,useInsertionEffect:ed,useLayoutEffect:td,useMemo:ld,useReducer:wi,useRef:Ih,useState:function(){return wi(zn)},useDebugValue:Gu,useDeferredValue:function(e,t){var a=qt();return Ze===null?qu(a,e,t):sd(a,Ze.memoizedState,e,t)},useTransition:function(){var e=wi(zn)[0],t=qt().memoizedState;return[typeof e=="boolean"?e:Xn(e),t]},useSyncExternalStore:mr,useId:ud,useHostTransitionStatus:Ku,useFormState:Qh,useActionState:Qh,useOptimistic:function(e,t){var a=qt();return Ze!==null?Gh(a,Ze,e,t):(a.baseState=e,[e,a.queue.dispatch])},useMemoCache:Qo,useCacheRefresh:fd};vd.useEffectEvent=$h;function Wu(e,t,a,n){t=e.memoizedState,a=a(n,t),a=a==null?t:M({},t,a),e.memoizedState=a,e.lanes===0&&(e.updateQueue.baseState=a)}var Xu={enqueueSetState:function(e,t,a){e=e._reactInternals;var n=Rn(),s=Y(n);s.payload=t,a!=null&&(s.callback=a),t=ne(e,s,n),t!==null&&(dn(t,e,n),te(t,e,n))},enqueueReplaceState:function(e,t,a){e=e._reactInternals;var n=Rn(),s=Y(n);s.tag=1,s.payload=t,a!=null&&(s.callback=a),t=ne(e,s,n),t!==null&&(dn(t,e,n),te(t,e,n))},enqueueForceUpdate:function(e,t){e=e._reactInternals;var a=Rn(),n=Y(a);n.tag=2,t!=null&&(n.callback=t),t=ne(e,n,a),t!==null&&(dn(t,e,a),te(t,e,a))}};function gd(e,t,a,n,s,u,p){return e=e.stateNode,typeof e.shouldComponentUpdate=="function"?e.shouldComponentUpdate(n,u,p):t.prototype&&t.prototype.isPureReactComponent?!Tl(a,n)||!Tl(s,u):!0}function yd(e,t,a,n){e=t.state,typeof t.componentWillReceiveProps=="function"&&t.componentWillReceiveProps(a,n),typeof t.UNSAFE_componentWillReceiveProps=="function"&&t.UNSAFE_componentWillReceiveProps(a,n),t.state!==e&&Xu.enqueueReplaceState(t,t.state,null)}function bi(e,t){var a=t;if("ref"in t){a={};for(var n in t)n!=="ref"&&(a[n]=t[n])}if(e=e.defaultProps){a===t&&(a=M({},a));for(var s in e)a[s]===void 0&&(a[s]=e[s])}return a}function xd(e){ar(e)}function wd(e){console.error(e)}function bd(e){ar(e)}function m0(e,t){try{var a=e.onUncaughtError;a(t.value,{componentStack:t.stack})}catch(n){setTimeout(function(){throw n})}}function Md(e,t,a){try{var n=e.onCaughtError;n(a.value,{componentStack:a.stack,errorBoundary:t.tag===1?t.stateNode:null})}catch(s){setTimeout(function(){throw s})}}function Zu(e,t,a){return a=Y(a),a.tag=3,a.payload={element:null},a.callback=function(){m0(e,t)},a}function Sd(e){return e=Y(e),e.tag=3,e}function Td(e,t,a,n){var s=a.type.getDerivedStateFromError;if(typeof s=="function"){var u=n.value;e.payload=function(){return s(u)},e.callback=function(){Md(t,a,n)}}var p=a.stateNode;p!==null&&typeof p.componentDidCatch=="function"&&(e.callback=function(){Md(t,a,n),typeof s!="function"&&(Es===null?Es=new Set([this]):Es.add(this));var w=n.stack;this.componentDidCatch(n.value,{componentStack:w!==null?w:""})})}function Xp(e,t,a,n,s){if(a.flags|=32768,n!==null&&typeof n=="object"&&typeof n.then=="function"){if(t=a.alternate,t!==null&&Tn(t,a,s,!0),a=Oe.current,a!==null){switch(a.tag){case 31:case 13:return Fe===null?_0():a.alternate===null&&ta===0&&(ta=3),a.flags&=-257,a.flags|=65536,a.lanes=s,n===bs?a.flags|=16384:(t=a.updateQueue,t===null?a.updateQueue=new Set([n]):t.add(n),bf(e,n,s)),!1;case 22:return a.flags|=65536,n===bs?a.flags|=16384:(t=a.updateQueue,t===null?(t={transitions:null,markerInstances:null,retryQueue:new Set([n])},a.updateQueue=t):(a=t.retryQueue,a===null?t.retryQueue=new Set([n]):a.add(n)),bf(e,n,s)),!1}throw Error(c(435,a.tag))}return bf(e,n,s),_0(),!1}if(vt)return t=Oe.current,t!==null?((t.flags&65536)===0&&(t.flags|=256),t.flags|=65536,t.lanes=s,n!==di&&(e=Error(c(422),{cause:n}),Al(ga(e,a)))):(n!==di&&(t=Error(c(423),{cause:n}),Al(ga(t,a))),e=e.current.alternate,e.flags|=65536,s&=-s,e.lanes|=s,n=ga(n,a),s=Zu(e.stateNode,n,s),me(e,s),ta!==4&&(ta=2)),!1;var u=Error(c(520),{cause:n});if(u=ga(u,a),wc===null?wc=[u]:wc.push(u),ta!==4&&(ta=2),t===null)return!0;n=ga(n,a),a=t;do{switch(a.tag){case 3:return a.flags|=65536,e=s&-s,a.lanes|=e,e=Zu(a.stateNode,n,e),me(a,e),!1;case 1:if(t=a.type,u=a.stateNode,(a.flags&128)===0&&(typeof t.getDerivedStateFromError=="function"||u!==null&&typeof u.componentDidCatch=="function"&&(Es===null||!Es.has(u))))return a.flags|=65536,s&=-s,a.lanes|=s,s=Sd(s),Td(s,e,a,n),me(a,s),!1}a=a.return}while(a!==null);return!1}var Qu=Error(c(461)),ca=!1;function Ta(e,t,a,n){t.child=e===null?D(t,null,a,n):z(t,e.child,a,n)}function Cd(e,t,a,n,s){a=a.render;var u=t.ref;if("ref"in n){var p={};for(var w in n)w!=="ref"&&(p[w]=n[w])}else p=n;return uo(t),n=Qa(e,t,a,p,u,s),w=ja(),e!==null&&!ca?(rn(e,t,s),kl(e,t,s)):(vt&&w&&rr(t),t.flags|=1,Ta(e,t,n,s),t.child)}function _d(e,t,a,n,s){if(e===null){var u=a.type;return typeof u=="function"&&!vs(u)&&u.defaultProps===void 0&&a.compare===null?(t.tag=15,t.type=u,zd(e,t,u,n,s)):(e=gs(a.type,null,n,t,t.mode,s),e.ref=t.ref,e.return=t,t.child=e)}if(u=e.child,!of(e,s)){var p=u.memoizedProps;if(a=a.compare,a=a!==null?a:Tl,a(p,n)&&e.ref===t.ref)return kl(e,t,s)}return t.flags|=1,e=tn(u,n),e.ref=t.ref,e.return=t,t.child=e}function zd(e,t,a,n,s){if(e!==null){var u=e.memoizedProps;if(Tl(u,n)&&e.ref===t.ref)if(ca=!1,t.pendingProps=n=u,of(e,s))(e.flags&131072)!==0&&(ca=!0);else return t.lanes=e.lanes,kl(e,t,s)}return Iu(e,t,a,n,s)}function Ad(e,t,a,n){var s=n.children,u=e!==null?e.memoizedState:null;if(e===null&&t.stateNode===null&&(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),n.mode==="hidden"){if((t.flags&128)!==0){if(u=u!==null?u.baseLanes|a:a,e!==null){for(n=t.child=e.child,s=0;n!==null;)s=s|n.lanes|n.childLanes,n=n.sibling;n=s&~u}else n=0,t.child=null;return Ed(e,t,u,a,n)}if((a&536870912)!==0)t.memoizedState={baseLanes:0,cachePool:null},e!==null&&gi(t,u!==null?u.cachePool:null),u!==null?Ae(t,u):ke(),He(t);else return n=t.lanes=536870912,Ed(e,t,u!==null?u.baseLanes|a:a,a,n)}else u!==null?(gi(t,u.cachePool),Ae(t,u),Ke(),t.memoizedState=null):(e!==null&&gi(t,null),ke(),Ke());return Ta(e,t,s,a),t.child}function dc(e,t){return e!==null&&e.tag===22||t.stateNode!==null||(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),t.sibling}function Ed(e,t,a,n,s){var u=dr();return u=u===null?null:{parent:Kt._currentValue,pool:u},t.memoizedState={baseLanes:a,cachePool:u},e!==null&&gi(t,null),ke(),He(t),e!==null&&Tn(e,t,n,!0),t.childLanes=s,null}function p0(e,t){return t=g0({mode:t.mode,children:t.children},e.mode),t.ref=e.ref,e.child=t,t.return=e,t}function Rd(e,t,a){return z(t,e.child,null,a),e=p0(t,t.pendingProps),e.flags|=2,Be(t),t.memoizedState=null,e}function Zp(e,t,a){var n=t.pendingProps,s=(t.flags&128)!==0;if(t.flags&=-129,e===null){if(vt){if(n.mode==="hidden")return e=p0(t,n),t.lanes=536870912,dc(null,e);if(De(t),(e=Ht)?(e=qm(e,Xa),e=e!==null&&e.data==="&"?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:Po!==null?{id:an,overflow:nn}:null,retryLane:536870912,hydrationErrors:null},a=sr(e),a.return=t,t.child=a,ia=t,Ht=null)):e=null,e===null)throw Mn(t);return t.lanes=536870912,null}return p0(t,n)}var u=e.memoizedState;if(u!==null){var p=u.dehydrated;if(De(t),s)if(t.flags&256)t.flags&=-257,t=Rd(e,t,a);else if(t.memoizedState!==null)t.child=e.child,t.flags|=128,t=null;else throw Error(c(558));else if(ca||Tn(e,t,a,!1),s=(a&e.childLanes)!==0,ca||s){if(n=jt,n!==null&&(p=On(n,a),p!==0&&p!==u.retryLane))throw u.retryLane=p,Vn(e,p),dn(n,e,p),Qu;_0(),t=Rd(e,t,a)}else e=u.treeContext,Ht=Qn(p.nextSibling),ia=t,vt=!0,Yo=null,Xa=!1,e!==null&&lc(t,e),t=p0(t,n),t.flags|=4096;return t}return e=tn(e.child,{mode:n.mode,children:n.children}),e.ref=t.ref,t.child=e,e.return=t,e}function v0(e,t){var a=t.ref;if(a===null)e!==null&&e.ref!==null&&(t.flags|=4194816);else{if(typeof a!="function"&&typeof a!="object")throw Error(c(284));(e===null||e.ref!==a)&&(t.flags|=4194816)}}function Iu(e,t,a,n,s){return uo(t),a=Qa(e,t,a,n,void 0,s),n=ja(),e!==null&&!ca?(rn(e,t,s),kl(e,t,s)):(vt&&n&&rr(t),t.flags|=1,Ta(e,t,a,s),t.child)}function Nd(e,t,a,n,s,u){return uo(t),t.updateQueue=null,a=vo(t,n,a,s),Zo(e),n=ja(),e!==null&&!ca?(rn(e,t,u),kl(e,t,u)):(vt&&n&&rr(t),t.flags|=1,Ta(e,t,a,u),t.child)}function Dd(e,t,a,n,s){if(uo(t),t.stateNode===null){var u=qo,p=a.contextType;typeof p=="object"&&p!==null&&(u=ra(p)),u=new a(n,u),t.memoizedState=u.state!==null&&u.state!==void 0?u.state:null,u.updater=Xu,t.stateNode=u,u._reactInternals=t,u=t.stateNode,u.props=n,u.state=t.memoizedState,u.refs={},E(t),p=a.contextType,u.context=typeof p=="object"&&p!==null?ra(p):qo,u.state=t.memoizedState,p=a.getDerivedStateFromProps,typeof p=="function"&&(Wu(t,a,p,n),u.state=t.memoizedState),typeof a.getDerivedStateFromProps=="function"||typeof u.getSnapshotBeforeUpdate=="function"||typeof u.UNSAFE_componentWillMount!="function"&&typeof u.componentWillMount!="function"||(p=u.state,typeof u.componentWillMount=="function"&&u.componentWillMount(),typeof u.UNSAFE_componentWillMount=="function"&&u.UNSAFE_componentWillMount(),p!==u.state&&Xu.enqueueReplaceState(u,u.state,null),be(t,n,u,s),he(),u.state=t.memoizedState),typeof u.componentDidMount=="function"&&(t.flags|=4194308),n=!0}else if(e===null){u=t.stateNode;var w=t.memoizedProps,q=bi(a,w);u.props=q;var fe=u.context,Te=a.contextType;p=qo,typeof Te=="object"&&Te!==null&&(p=ra(Te));var Re=a.getDerivedStateFromProps;Te=typeof Re=="function"||typeof u.getSnapshotBeforeUpdate=="function",w=t.pendingProps!==w,Te||typeof u.UNSAFE_componentWillReceiveProps!="function"&&typeof u.componentWillReceiveProps!="function"||(w||fe!==p)&&yd(t,u,n,p),L=!1;var de=t.memoizedState;u.state=de,be(t,n,u,s),he(),fe=t.memoizedState,w||de!==fe||L?(typeof Re=="function"&&(Wu(t,a,Re,n),fe=t.memoizedState),(q=L||gd(t,a,q,n,de,fe,p))?(Te||typeof u.UNSAFE_componentWillMount!="function"&&typeof u.componentWillMount!="function"||(typeof u.componentWillMount=="function"&&u.componentWillMount(),typeof u.UNSAFE_componentWillMount=="function"&&u.UNSAFE_componentWillMount()),typeof u.componentDidMount=="function"&&(t.flags|=4194308)):(typeof u.componentDidMount=="function"&&(t.flags|=4194308),t.memoizedProps=n,t.memoizedState=fe),u.props=n,u.state=fe,u.context=p,n=q):(typeof u.componentDidMount=="function"&&(t.flags|=4194308),n=!1)}else{u=t.stateNode,j(e,t),p=t.memoizedProps,Te=bi(a,p),u.props=Te,Re=t.pendingProps,de=u.context,fe=a.contextType,q=qo,typeof fe=="object"&&fe!==null&&(q=ra(fe)),w=a.getDerivedStateFromProps,(fe=typeof w=="function"||typeof u.getSnapshotBeforeUpdate=="function")||typeof u.UNSAFE_componentWillReceiveProps!="function"&&typeof u.componentWillReceiveProps!="function"||(p!==Re||de!==q)&&yd(t,u,n,q),L=!1,de=t.memoizedState,u.state=de,be(t,n,u,s),he();var xe=t.memoizedState;p!==Re||de!==xe||L||e!==null&&e.dependencies!==null&&fr(e.dependencies)?(typeof w=="function"&&(Wu(t,a,w,n),xe=t.memoizedState),(Te=L||gd(t,a,Te,n,de,xe,q)||e!==null&&e.dependencies!==null&&fr(e.dependencies))?(fe||typeof u.UNSAFE_componentWillUpdate!="function"&&typeof u.componentWillUpdate!="function"||(typeof u.componentWillUpdate=="function"&&u.componentWillUpdate(n,xe,q),typeof u.UNSAFE_componentWillUpdate=="function"&&u.UNSAFE_componentWillUpdate(n,xe,q)),typeof u.componentDidUpdate=="function"&&(t.flags|=4),typeof u.getSnapshotBeforeUpdate=="function"&&(t.flags|=1024)):(typeof u.componentDidUpdate!="function"||p===e.memoizedProps&&de===e.memoizedState||(t.flags|=4),typeof u.getSnapshotBeforeUpdate!="function"||p===e.memoizedProps&&de===e.memoizedState||(t.flags|=1024),t.memoizedProps=n,t.memoizedState=xe),u.props=n,u.state=xe,u.context=q,n=Te):(typeof u.componentDidUpdate!="function"||p===e.memoizedProps&&de===e.memoizedState||(t.flags|=4),typeof u.getSnapshotBeforeUpdate!="function"||p===e.memoizedProps&&de===e.memoizedState||(t.flags|=1024),n=!1)}return u=n,v0(e,t),n=(t.flags&128)!==0,u||n?(u=t.stateNode,a=n&&typeof a.getDerivedStateFromError!="function"?null:u.render(),t.flags|=1,e!==null&&n?(t.child=z(t,e.child,null,s),t.child=z(t,null,a,s)):Ta(e,t,a,s),t.memoizedState=u.state,e=t.child):e=kl(e,t,s),e}function kd(e,t,a,n){return ro(),t.flags|=256,Ta(e,t,a,n),t.child}var Ju={dehydrated:null,treeContext:null,retryLane:0,hydrationErrors:null};function $u(e){return{baseLanes:e,cachePool:u0()}}function ef(e,t,a){return e=e!==null?e.childLanes&~a:0,t&&(e|=En),e}function Ud(e,t,a){var n=t.pendingProps,s=!1,u=(t.flags&128)!==0,p;if((p=u)||(p=e!==null&&e.memoizedState===null?!1:(Xe.current&2)!==0),p&&(s=!0,t.flags&=-129),p=(t.flags&32)!==0,t.flags&=-33,e===null){if(vt){if(s?Ge(t):Ke(),(e=Ht)?(e=qm(e,Xa),e=e!==null&&e.data!=="&"?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:Po!==null?{id:an,overflow:nn}:null,retryLane:536870912,hydrationErrors:null},a=sr(e),a.return=t,t.child=a,ia=t,Ht=null)):e=null,e===null)throw Mn(t);return Hf(e)?t.lanes=32:t.lanes=536870912,null}var w=n.children;return n=n.fallback,s?(Ke(),s=t.mode,w=g0({mode:"hidden",children:w},s),n=Kn(n,s,a,null),w.return=t,n.return=t,w.sibling=n,t.child=w,n=t.child,n.memoizedState=$u(a),n.childLanes=ef(e,p,a),t.memoizedState=Ju,dc(null,n)):(Ge(t),tf(t,w))}var q=e.memoizedState;if(q!==null&&(w=q.dehydrated,w!==null)){if(u)t.flags&256?(Ge(t),t.flags&=-257,t=af(e,t,a)):t.memoizedState!==null?(Ke(),t.child=e.child,t.flags|=128,t=null):(Ke(),w=n.fallback,s=t.mode,n=g0({mode:"visible",children:n.children},s),w=Kn(w,s,a,null),w.flags|=2,n.return=t,w.return=t,n.sibling=w,t.child=n,z(t,e.child,null,a),n=t.child,n.memoizedState=$u(a),n.childLanes=ef(e,p,a),t.memoizedState=Ju,t=dc(null,n));else if(Ge(t),Hf(w)){if(p=w.nextSibling&&w.nextSibling.dataset,p)var fe=p.dgst;p=fe,n=Error(c(419)),n.stack="",n.digest=p,Al({value:n,source:null,stack:null}),t=af(e,t,a)}else if(ca||Tn(e,t,a,!1),p=(a&e.childLanes)!==0,ca||p){if(p=jt,p!==null&&(n=On(p,a),n!==0&&n!==q.retryLane))throw q.retryLane=n,Vn(e,n),dn(p,e,n),Qu;Bf(w)||_0(),t=af(e,t,a)}else Bf(w)?(t.flags|=192,t.child=e.child,t=null):(e=q.treeContext,Ht=Qn(w.nextSibling),ia=t,vt=!0,Yo=null,Xa=!1,e!==null&&lc(t,e),t=tf(t,n.children),t.flags|=4096);return t}return s?(Ke(),w=n.fallback,s=t.mode,q=e.child,fe=q.sibling,n=tn(q,{mode:"hidden",children:n.children}),n.subtreeFlags=q.subtreeFlags&65011712,fe!==null?w=tn(fe,w):(w=Kn(w,s,a,null),w.flags|=2),w.return=t,n.return=t,n.sibling=w,t.child=n,dc(null,n),n=t.child,w=e.child.memoizedState,w===null?w=$u(a):(s=w.cachePool,s!==null?(q=Kt._currentValue,s=s.parent!==q?{parent:q,pool:q}:s):s=u0(),w={baseLanes:w.baseLanes|a,cachePool:s}),n.memoizedState=w,n.childLanes=ef(e,p,a),t.memoizedState=Ju,dc(e.child,n)):(Ge(t),a=e.child,e=a.sibling,a=tn(a,{mode:"visible",children:n.children}),a.return=t,a.sibling=null,e!==null&&(p=t.deletions,p===null?(t.deletions=[e],t.flags|=16):p.push(e)),t.child=a,t.memoizedState=null,a)}function tf(e,t){return t=g0({mode:"visible",children:t},e.mode),t.return=e,e.child=t}function g0(e,t){return e=Ba(22,e,null,t),e.lanes=0,e}function af(e,t,a){return z(t,e.child,null,a),e=tf(t,t.pendingProps.children),e.flags|=2,t.memoizedState=null,e}function Od(e,t,a){e.lanes|=t;var n=e.alternate;n!==null&&(n.lanes|=t),El(e.return,t,a)}function nf(e,t,a,n,s,u){var p=e.memoizedState;p===null?e.memoizedState={isBackwards:t,rendering:null,renderingStartTime:0,last:n,tail:a,tailMode:s,treeForkCount:u}:(p.isBackwards=t,p.rendering=null,p.renderingStartTime=0,p.last=n,p.tail=a,p.tailMode=s,p.treeForkCount=u)}function Bd(e,t,a){var n=t.pendingProps,s=n.revealOrder,u=n.tail;n=n.children;var p=Xe.current,w=(p&2)!==0;if(w?(p=p&1|2,t.flags|=128):p&=1,ve(Xe,p),Ta(e,t,n,a),n=vt?Ko:0,!w&&e!==null&&(e.flags&128)!==0)e:for(e=t.child;e!==null;){if(e.tag===13)e.memoizedState!==null&&Od(e,a,t);else if(e.tag===19)Od(e,a,t);else if(e.child!==null){e.child.return=e,e=e.child;continue}if(e===t)break e;for(;e.sibling===null;){if(e.return===null||e.return===t)break e;e=e.return}e.sibling.return=e.return,e=e.sibling}switch(s){case"forwards":for(a=t.child,s=null;a!==null;)e=a.alternate,e!==null&&rt(e)===null&&(s=a),a=a.sibling;a=s,a===null?(s=t.child,t.child=null):(s=a.sibling,a.sibling=null),nf(t,!1,s,a,u,n);break;case"backwards":case"unstable_legacy-backwards":for(a=null,s=t.child,t.child=null;s!==null;){if(e=s.alternate,e!==null&&rt(e)===null){t.child=s;break}e=s.sibling,s.sibling=a,a=s,s=e}nf(t,!0,a,null,u,n);break;case"together":nf(t,!1,null,null,void 0,n);break;default:t.memoizedState=null}return t.child}function kl(e,t,a){if(e!==null&&(t.dependencies=e.dependencies),As|=t.lanes,(a&t.childLanes)===0)if(e!==null){if(Tn(e,t,a,!1),(a&t.childLanes)===0)return null}else return null;if(e!==null&&t.child!==e.child)throw Error(c(153));if(t.child!==null){for(e=t.child,a=tn(e,e.pendingProps),t.child=a,a.return=t;e.sibling!==null;)e=e.sibling,a=a.sibling=tn(e,e.pendingProps),a.return=t;a.sibling=null}return t.child}function of(e,t){return(e.lanes&t)!==0?!0:(e=e.dependencies,!!(e!==null&&fr(e)))}function Qp(e,t,a){switch(t.tag){case 3:Ne(t,t.stateNode.containerInfo),Pn(t,Kt,e.memoizedState.cache),ro();break;case 27:case 5:J(t);break;case 4:Ne(t,t.stateNode.containerInfo);break;case 10:Pn(t,t.type,t.memoizedProps.value);break;case 31:if(t.memoizedState!==null)return t.flags|=128,De(t),null;break;case 13:var n=t.memoizedState;if(n!==null)return n.dehydrated!==null?(Ge(t),t.flags|=128,null):(a&t.child.childLanes)!==0?Ud(e,t,a):(Ge(t),e=kl(e,t,a),e!==null?e.sibling:null);Ge(t);break;case 19:var s=(e.flags&128)!==0;if(n=(a&t.childLanes)!==0,n||(Tn(e,t,a,!1),n=(a&t.childLanes)!==0),s){if(n)return Bd(e,t,a);t.flags|=128}if(s=t.memoizedState,s!==null&&(s.rendering=null,s.tail=null,s.lastEffect=null),ve(Xe,Xe.current),n)break;return null;case 22:return t.lanes=0,Ad(e,t,a,t.pendingProps);case 24:Pn(t,Kt,e.memoizedState.cache)}return kl(e,t,a)}function Hd(e,t,a){if(e!==null)if(e.memoizedProps!==t.pendingProps)ca=!0;else{if(!of(e,a)&&(t.flags&128)===0)return ca=!1,Qp(e,t,a);ca=(e.flags&131072)!==0}else ca=!1,vt&&(t.flags&1048576)!==0&&oc(t,Ko,t.index);switch(t.lanes=0,t.tag){case 16:e:{var n=t.pendingProps;if(e=Xo(t.elementType),t.type=e,typeof e=="function")vs(e)?(n=bi(e,n),t.tag=1,t=Dd(null,t,e,n,a)):(t.tag=0,t=Iu(null,t,e,n,a));else{if(e!=null){var s=e.$$typeof;if(s===_){t.tag=11,t=Cd(null,t,e,n,a);break e}else if(s===le){t.tag=14,t=_d(null,t,e,n,a);break e}}throw t=ie(e)||e,Error(c(306,t,""))}}return t;case 0:return Iu(e,t,t.type,t.pendingProps,a);case 1:return n=t.type,s=bi(n,t.pendingProps),Dd(e,t,n,s,a);case 3:e:{if(Ne(t,t.stateNode.containerInfo),e===null)throw Error(c(387));n=t.pendingProps;var u=t.memoizedState;s=u.element,j(e,t),be(t,n,null,a);var p=t.memoizedState;if(n=p.cache,Pn(t,Kt,n),n!==u.cache&&Rl(t,[Kt],a,!0),he(),n=p.element,u.isDehydrated)if(u={element:n,isDehydrated:!1,cache:p.cache},t.updateQueue.baseState=u,t.memoizedState=u,t.flags&256){t=kd(e,t,n,a);break e}else if(n!==s){s=ga(Error(c(424)),t),Al(s),t=kd(e,t,n,a);break e}else{switch(e=t.stateNode.containerInfo,e.nodeType){case 9:e=e.body;break;default:e=e.nodeName==="HTML"?e.ownerDocument.body:e}for(Ht=Qn(e.firstChild),ia=t,vt=!0,Yo=null,Xa=!0,a=D(t,null,n,a),t.child=a;a;)a.flags=a.flags&-3|4096,a=a.sibling}else{if(ro(),n===s){t=kl(e,t,a);break e}Ta(e,t,n,a)}t=t.child}return t;case 26:return v0(e,t),e===null?(a=Xm(t.type,null,t.pendingProps,null))?t.memoizedState=a:vt||(a=t.type,e=t.pendingProps,n=k0(ue.current).createElement(a),n[oa]=t,n[ma]=e,Ca(n,a,e),ea(n),t.stateNode=n):t.memoizedState=Xm(t.type,e.memoizedProps,t.pendingProps,e.memoizedState),null;case 27:return J(t),e===null&&vt&&(n=t.stateNode=Pm(t.type,t.pendingProps,ue.current),ia=t,Xa=!0,s=Ht,ks(t.type)?(Lf=s,Ht=Qn(n.firstChild)):Ht=s),Ta(e,t,t.pendingProps.children,a),v0(e,t),e===null&&(t.flags|=4194304),t.child;case 5:return e===null&&vt&&((s=n=Ht)&&(n=_v(n,t.type,t.pendingProps,Xa),n!==null?(t.stateNode=n,ia=t,Ht=Qn(n.firstChild),Xa=!1,s=!0):s=!1),s||Mn(t)),J(t),s=t.type,u=t.pendingProps,p=e!==null?e.memoizedProps:null,n=u.children,kf(s,u)?n=null:p!==null&&kf(s,p)&&(t.flags|=32),t.memoizedState!==null&&(s=Qa(e,t,Ts,null,null,a),Ac._currentValue=s),v0(e,t),Ta(e,t,n,a),t.child;case 6:return e===null&&vt&&((e=a=Ht)&&(a=zv(a,t.pendingProps,Xa),a!==null?(t.stateNode=a,ia=t,Ht=null,e=!0):e=!1),e||Mn(t)),null;case 13:return Ud(e,t,a);case 4:return Ne(t,t.stateNode.containerInfo),n=t.pendingProps,e===null?t.child=z(t,null,n,a):Ta(e,t,n,a),t.child;case 11:return Cd(e,t,t.type,t.pendingProps,a);case 7:return Ta(e,t,t.pendingProps,a),t.child;case 8:return Ta(e,t,t.pendingProps.children,a),t.child;case 12:return Ta(e,t,t.pendingProps.children,a),t.child;case 10:return n=t.pendingProps,Pn(t,t.type,n.value),Ta(e,t,n.children,a),t.child;case 9:return s=t.type._context,n=t.pendingProps.children,uo(t),s=ra(s),n=n(s),t.flags|=1,Ta(e,t,n,a),t.child;case 14:return _d(e,t,t.type,t.pendingProps,a);case 15:return zd(e,t,t.type,t.pendingProps,a);case 19:return Bd(e,t,a);case 31:return Zp(e,t,a);case 22:return Ad(e,t,a,t.pendingProps);case 24:return uo(t),n=ra(Kt),e===null?(s=dr(),s===null&&(s=jt,u=pi(),s.pooledCache=u,u.refCount++,u!==null&&(s.pooledCacheLanes|=a),s=u),t.memoizedState={parent:n,cache:s},E(t),Pn(t,Kt,s)):((e.lanes&a)!==0&&(j(e,t),be(t,null,null,a),he()),s=e.memoizedState,u=t.memoizedState,s.parent!==n?(s={parent:n,cache:n},t.memoizedState=s,t.lanes===0&&(t.memoizedState=t.updateQueue.baseState=s),Pn(t,Kt,n)):(n=u.cache,Pn(t,Kt,n),n!==s.cache&&Rl(t,[Kt],a,!0))),Ta(e,t,t.pendingProps.children,a),t.child;case 29:throw t.pendingProps}throw Error(c(156,t.tag))}function Ul(e){e.flags|=4}function lf(e,t,a,n,s){if((t=(e.mode&32)!==0)&&(t=!1),t){if(e.flags|=16777216,(s&335544128)===s)if(e.stateNode.complete)e.flags|=8192;else if(um())e.flags|=8192;else throw po=bs,Za}else e.flags&=-16777217}function Ld(e,t){if(t.type!=="stylesheet"||(t.state.loading&4)!==0)e.flags&=-16777217;else if(e.flags|=16777216,!$m(t))if(um())e.flags|=8192;else throw po=bs,Za}function y0(e,t){t!==null&&(e.flags|=4),e.flags&16384&&(t=e.tag!==22?qc():536870912,e.lanes|=t,xr|=t)}function mc(e,t){if(!vt)switch(e.tailMode){case"hidden":t=e.tail;for(var a=null;t!==null;)t.alternate!==null&&(a=t),t=t.sibling;a===null?e.tail=null:a.sibling=null;break;case"collapsed":a=e.tail;for(var n=null;a!==null;)a.alternate!==null&&(n=a),a=a.sibling;n===null?t||e.tail===null?e.tail=null:e.tail.sibling=null:n.sibling=null}}function Pt(e){var t=e.alternate!==null&&e.alternate.child===e.child,a=0,n=0;if(t)for(var s=e.child;s!==null;)a|=s.lanes|s.childLanes,n|=s.subtreeFlags&65011712,n|=s.flags&65011712,s.return=e,s=s.sibling;else for(s=e.child;s!==null;)a|=s.lanes|s.childLanes,n|=s.subtreeFlags,n|=s.flags,s.return=e,s=s.sibling;return e.subtreeFlags|=n,e.childLanes=a,t}function Ip(e,t,a){var n=t.pendingProps;switch(cr(t),t.tag){case 16:case 15:case 0:case 11:case 7:case 8:case 12:case 9:case 14:return Pt(t),null;case 1:return Pt(t),null;case 3:return a=t.stateNode,n=null,e!==null&&(n=e.memoizedState.cache),t.memoizedState.cache!==n&&(t.flags|=2048),Yn(Kt),oe(),a.pendingContext&&(a.context=a.pendingContext,a.pendingContext=null),(e===null||e.child===null)&&(io(t)?Ul(t):e===null||e.memoizedState.isDehydrated&&(t.flags&256)===0||(t.flags|=1024,Sn())),Pt(t),null;case 26:var s=t.type,u=t.memoizedState;return e===null?(Ul(t),u!==null?(Pt(t),Ld(t,u)):(Pt(t),lf(t,s,null,n,a))):u?u!==e.memoizedState?(Ul(t),Pt(t),Ld(t,u)):(Pt(t),t.flags&=-16777217):(e=e.memoizedProps,e!==n&&Ul(t),Pt(t),lf(t,s,e,n,a)),null;case 27:if(_e(t),a=ue.current,s=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==n&&Ul(t);else{if(!n){if(t.stateNode===null)throw Error(c(166));return Pt(t),null}e=V.current,io(t)?St(t):(e=Pm(s,n,a),t.stateNode=e,Ul(t))}return Pt(t),null;case 5:if(_e(t),s=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==n&&Ul(t);else{if(!n){if(t.stateNode===null)throw Error(c(166));return Pt(t),null}if(u=V.current,io(t))St(t);else{var p=k0(ue.current);switch(u){case 1:u=p.createElementNS("http://www.w3.org/2000/svg",s);break;case 2:u=p.createElementNS("http://www.w3.org/1998/Math/MathML",s);break;default:switch(s){case"svg":u=p.createElementNS("http://www.w3.org/2000/svg",s);break;case"math":u=p.createElementNS("http://www.w3.org/1998/Math/MathML",s);break;case"script":u=p.createElement("div"),u.innerHTML="<script><\/script>",u=u.removeChild(u.firstChild);break;case"select":u=typeof n.is=="string"?p.createElement("select",{is:n.is}):p.createElement("select"),n.multiple?u.multiple=!0:n.size&&(u.size=n.size);break;default:u=typeof n.is=="string"?p.createElement(s,{is:n.is}):p.createElement(s)}}u[oa]=t,u[ma]=n;e:for(p=t.child;p!==null;){if(p.tag===5||p.tag===6)u.appendChild(p.stateNode);else if(p.tag!==4&&p.tag!==27&&p.child!==null){p.child.return=p,p=p.child;continue}if(p===t)break e;for(;p.sibling===null;){if(p.return===null||p.return===t)break e;p=p.return}p.sibling.return=p.return,p=p.sibling}t.stateNode=u;e:switch(Ca(u,s,n),s){case"button":case"input":case"select":case"textarea":n=!!n.autoFocus;break e;case"img":n=!0;break e;default:n=!1}n&&Ul(t)}}return Pt(t),lf(t,t.type,e===null?null:e.memoizedProps,t.pendingProps,a),null;case 6:if(e&&t.stateNode!=null)e.memoizedProps!==n&&Ul(t);else{if(typeof n!="string"&&t.stateNode===null)throw Error(c(166));if(e=ue.current,io(t)){if(e=t.stateNode,a=t.memoizedProps,n=null,s=ia,s!==null)switch(s.tag){case 27:case 5:n=s.memoizedProps}e[oa]=t,e=!!(e.nodeValue===a||n!==null&&n.suppressHydrationWarning===!0||Um(e.nodeValue,a)),e||Mn(t,!0)}else e=k0(e).createTextNode(n),e[oa]=t,t.stateNode=e}return Pt(t),null;case 31:if(a=t.memoizedState,e===null||e.memoizedState!==null){if(n=io(t),a!==null){if(e===null){if(!n)throw Error(c(318));if(e=t.memoizedState,e=e!==null?e.dehydrated:null,!e)throw Error(c(557));e[oa]=t}else ro(),(t.flags&128)===0&&(t.memoizedState=null),t.flags|=4;Pt(t),e=!1}else a=Sn(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=a),e=!0;if(!e)return t.flags&256?(Be(t),t):(Be(t),null);if((t.flags&128)!==0)throw Error(c(558))}return Pt(t),null;case 13:if(n=t.memoizedState,e===null||e.memoizedState!==null&&e.memoizedState.dehydrated!==null){if(s=io(t),n!==null&&n.dehydrated!==null){if(e===null){if(!s)throw Error(c(318));if(s=t.memoizedState,s=s!==null?s.dehydrated:null,!s)throw Error(c(317));s[oa]=t}else ro(),(t.flags&128)===0&&(t.memoizedState=null),t.flags|=4;Pt(t),s=!1}else s=Sn(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=s),s=!0;if(!s)return t.flags&256?(Be(t),t):(Be(t),null)}return Be(t),(t.flags&128)!==0?(t.lanes=a,t):(a=n!==null,e=e!==null&&e.memoizedState!==null,a&&(n=t.child,s=null,n.alternate!==null&&n.alternate.memoizedState!==null&&n.alternate.memoizedState.cachePool!==null&&(s=n.alternate.memoizedState.cachePool.pool),u=null,n.memoizedState!==null&&n.memoizedState.cachePool!==null&&(u=n.memoizedState.cachePool.pool),u!==s&&(n.flags|=2048)),a!==e&&a&&(t.child.flags|=8192),y0(t,t.updateQueue),Pt(t),null);case 4:return oe(),e===null&&Af(t.stateNode.containerInfo),Pt(t),null;case 10:return Yn(t.type),Pt(t),null;case 19:if(Q(Xe),n=t.memoizedState,n===null)return Pt(t),null;if(s=(t.flags&128)!==0,u=n.rendering,u===null)if(s)mc(n,!1);else{if(ta!==0||e!==null&&(e.flags&128)!==0)for(e=t.child;e!==null;){if(u=rt(e),u!==null){for(t.flags|=128,mc(n,!1),e=u.updateQueue,t.updateQueue=e,y0(t,e),t.subtreeFlags=0,e=a,a=t.child;a!==null;)lr(a,e),a=a.sibling;return ve(Xe,Xe.current&1|2),vt&&bn(t,n.treeForkCount),t.child}e=e.sibling}n.tail!==null&&Dt()>S0&&(t.flags|=128,s=!0,mc(n,!1),t.lanes=4194304)}else{if(!s)if(e=rt(u),e!==null){if(t.flags|=128,s=!0,e=e.updateQueue,t.updateQueue=e,y0(t,e),mc(n,!0),n.tail===null&&n.tailMode==="hidden"&&!u.alternate&&!vt)return Pt(t),null}else 2*Dt()-n.renderingStartTime>S0&&a!==536870912&&(t.flags|=128,s=!0,mc(n,!1),t.lanes=4194304);n.isBackwards?(u.sibling=t.child,t.child=u):(e=n.last,e!==null?e.sibling=u:t.child=u,n.last=u)}return n.tail!==null?(e=n.tail,n.rendering=e,n.tail=e.sibling,n.renderingStartTime=Dt(),e.sibling=null,a=Xe.current,ve(Xe,s?a&1|2:a&1),vt&&bn(t,n.treeForkCount),e):(Pt(t),null);case 22:case 23:return Be(t),qe(),n=t.memoizedState!==null,e!==null?e.memoizedState!==null!==n&&(t.flags|=8192):n&&(t.flags|=8192),n?(a&536870912)!==0&&(t.flags&128)===0&&(Pt(t),t.subtreeFlags&6&&(t.flags|=8192)):Pt(t),a=t.updateQueue,a!==null&&y0(t,a.retryQueue),a=null,e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(a=e.memoizedState.cachePool.pool),n=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(n=t.memoizedState.cachePool.pool),n!==a&&(t.flags|=2048),e!==null&&Q(Cn),null;case 24:return a=null,e!==null&&(a=e.memoizedState.cache),t.memoizedState.cache!==a&&(t.flags|=2048),Yn(Kt),Pt(t),null;case 25:return null;case 30:return null}throw Error(c(156,t.tag))}function Jp(e,t){switch(cr(t),t.tag){case 1:return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 3:return Yn(Kt),oe(),e=t.flags,(e&65536)!==0&&(e&128)===0?(t.flags=e&-65537|128,t):null;case 26:case 27:case 5:return _e(t),null;case 31:if(t.memoizedState!==null){if(Be(t),t.alternate===null)throw Error(c(340));ro()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 13:if(Be(t),e=t.memoizedState,e!==null&&e.dehydrated!==null){if(t.alternate===null)throw Error(c(340));ro()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 19:return Q(Xe),null;case 4:return oe(),null;case 10:return Yn(t.type),null;case 22:case 23:return Be(t),qe(),e!==null&&Q(Cn),e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 24:return Yn(Kt),null;case 25:return null;default:return null}}function jd(e,t){switch(cr(t),t.tag){case 3:Yn(Kt),oe();break;case 26:case 27:case 5:_e(t);break;case 4:oe();break;case 31:t.memoizedState!==null&&Be(t);break;case 13:Be(t);break;case 19:Q(Xe);break;case 10:Yn(t.type);break;case 22:case 23:Be(t),qe(),e!==null&&Q(Cn);break;case 24:Yn(Kt)}}function pc(e,t){try{var a=t.updateQueue,n=a!==null?a.lastEffect:null;if(n!==null){var s=n.next;a=s;do{if((a.tag&e)===e){n=void 0;var u=a.create,p=a.inst;n=u(),p.destroy=n}a=a.next}while(a!==s)}}catch(w){Ut(t,t.return,w)}}function _s(e,t,a){try{var n=t.updateQueue,s=n!==null?n.lastEffect:null;if(s!==null){var u=s.next;n=u;do{if((n.tag&e)===e){var p=n.inst,w=p.destroy;if(w!==void 0){p.destroy=void 0,s=t;var q=a,fe=w;try{fe()}catch(Te){Ut(s,q,Te)}}}n=n.next}while(n!==u)}}catch(Te){Ut(t,t.return,Te)}}function Fd(e){var t=e.updateQueue;if(t!==null){var a=e.stateNode;try{Se(t,a)}catch(n){Ut(e,e.return,n)}}}function Gd(e,t,a){a.props=bi(e.type,e.memoizedProps),a.state=e.memoizedState;try{a.componentWillUnmount()}catch(n){Ut(e,t,n)}}function vc(e,t){try{var a=e.ref;if(a!==null){switch(e.tag){case 26:case 27:case 5:var n=e.stateNode;break;case 30:n=e.stateNode;break;default:n=e.stateNode}typeof a=="function"?e.refCleanup=a(n):a.current=n}}catch(s){Ut(e,t,s)}}function Io(e,t){var a=e.ref,n=e.refCleanup;if(a!==null)if(typeof n=="function")try{n()}catch(s){Ut(e,t,s)}finally{e.refCleanup=null,e=e.alternate,e!=null&&(e.refCleanup=null)}else if(typeof a=="function")try{a(null)}catch(s){Ut(e,t,s)}else a.current=null}function qd(e){var t=e.type,a=e.memoizedProps,n=e.stateNode;try{e:switch(t){case"button":case"input":case"select":case"textarea":a.autoFocus&&n.focus();break e;case"img":a.src?n.src=a.src:a.srcSet&&(n.srcset=a.srcSet)}}catch(s){Ut(e,e.return,s)}}function sf(e,t,a){try{var n=e.stateNode;wv(n,e.type,a,t),n[ma]=t}catch(s){Ut(e,e.return,s)}}function Vd(e){return e.tag===5||e.tag===3||e.tag===26||e.tag===27&&ks(e.type)||e.tag===4}function rf(e){e:for(;;){for(;e.sibling===null;){if(e.return===null||Vd(e.return))return null;e=e.return}for(e.sibling.return=e.return,e=e.sibling;e.tag!==5&&e.tag!==6&&e.tag!==18;){if(e.tag===27&&ks(e.type)||e.flags&2||e.child===null||e.tag===4)continue e;e.child.return=e,e=e.child}if(!(e.flags&2))return e.stateNode}}function cf(e,t,a){var n=e.tag;if(n===5||n===6)e=e.stateNode,t?(a.nodeType===9?a.body:a.nodeName==="HTML"?a.ownerDocument.body:a).insertBefore(e,t):(t=a.nodeType===9?a.body:a.nodeName==="HTML"?a.ownerDocument.body:a,t.appendChild(e),a=a._reactRootContainer,a!=null||t.onclick!==null||(t.onclick=no));else if(n!==4&&(n===27&&ks(e.type)&&(a=e.stateNode,t=null),e=e.child,e!==null))for(cf(e,t,a),e=e.sibling;e!==null;)cf(e,t,a),e=e.sibling}function x0(e,t,a){var n=e.tag;if(n===5||n===6)e=e.stateNode,t?a.insertBefore(e,t):a.appendChild(e);else if(n!==4&&(n===27&&ks(e.type)&&(a=e.stateNode),e=e.child,e!==null))for(x0(e,t,a),e=e.sibling;e!==null;)x0(e,t,a),e=e.sibling}function Kd(e){var t=e.stateNode,a=e.memoizedProps;try{for(var n=e.type,s=t.attributes;s.length;)t.removeAttributeNode(s[0]);Ca(t,n,a),t[oa]=e,t[ma]=a}catch(u){Ut(e,e.return,u)}}var Ol=!1,ua=!1,uf=!1,Pd=typeof WeakSet=="function"?WeakSet:Set,wa=null;function $p(e,t){if(e=e.containerInfo,Nf=F0,e=Ii(e),Gn(e)){if("selectionStart"in e)var a={start:e.selectionStart,end:e.selectionEnd};else e:{a=(a=e.ownerDocument)&&a.defaultView||window;var n=a.getSelection&&a.getSelection();if(n&&n.rangeCount!==0){a=n.anchorNode;var s=n.anchorOffset,u=n.focusNode;n=n.focusOffset;try{a.nodeType,u.nodeType}catch{a=null;break e}var p=0,w=-1,q=-1,fe=0,Te=0,Re=e,de=null;t:for(;;){for(var xe;Re!==a||s!==0&&Re.nodeType!==3||(w=p+s),Re!==u||n!==0&&Re.nodeType!==3||(q=p+n),Re.nodeType===3&&(p+=Re.nodeValue.length),(xe=Re.firstChild)!==null;)de=Re,Re=xe;for(;;){if(Re===e)break t;if(de===a&&++fe===s&&(w=p),de===u&&++Te===n&&(q=p),(xe=Re.nextSibling)!==null)break;Re=de,de=Re.parentNode}Re=xe}a=w===-1||q===-1?null:{start:w,end:q}}else a=null}a=a||{start:0,end:0}}else a=null;for(Df={focusedElem:e,selectionRange:a},F0=!1,wa=t;wa!==null;)if(t=wa,e=t.child,(t.subtreeFlags&1028)!==0&&e!==null)e.return=t,wa=e;else for(;wa!==null;){switch(t=wa,u=t.alternate,e=t.flags,t.tag){case 0:if((e&4)!==0&&(e=t.updateQueue,e=e!==null?e.events:null,e!==null))for(a=0;a<e.length;a++)s=e[a],s.ref.impl=s.nextImpl;break;case 11:case 15:break;case 1:if((e&1024)!==0&&u!==null){e=void 0,a=t,s=u.memoizedProps,u=u.memoizedState,n=a.stateNode;try{var We=bi(a.type,s);e=n.getSnapshotBeforeUpdate(We,u),n.__reactInternalSnapshotBeforeUpdate=e}catch(at){Ut(a,a.return,at)}}break;case 3:if((e&1024)!==0){if(e=t.stateNode.containerInfo,a=e.nodeType,a===9)Of(e);else if(a===1)switch(e.nodeName){case"HEAD":case"HTML":case"BODY":Of(e);break;default:e.textContent=""}}break;case 5:case 26:case 27:case 6:case 4:case 17:break;default:if((e&1024)!==0)throw Error(c(163))}if(e=t.sibling,e!==null){e.return=t.return,wa=e;break}wa=t.return}}function Yd(e,t,a){var n=a.flags;switch(a.tag){case 0:case 11:case 15:Hl(e,a),n&4&&pc(5,a);break;case 1:if(Hl(e,a),n&4)if(e=a.stateNode,t===null)try{e.componentDidMount()}catch(p){Ut(a,a.return,p)}else{var s=bi(a.type,t.memoizedProps);t=t.memoizedState;try{e.componentDidUpdate(s,t,e.__reactInternalSnapshotBeforeUpdate)}catch(p){Ut(a,a.return,p)}}n&64&&Fd(a),n&512&&vc(a,a.return);break;case 3:if(Hl(e,a),n&64&&(e=a.updateQueue,e!==null)){if(t=null,a.child!==null)switch(a.child.tag){case 27:case 5:t=a.child.stateNode;break;case 1:t=a.child.stateNode}try{Se(e,t)}catch(p){Ut(a,a.return,p)}}break;case 27:t===null&&n&4&&Kd(a);case 26:case 5:Hl(e,a),t===null&&n&4&&qd(a),n&512&&vc(a,a.return);break;case 12:Hl(e,a);break;case 31:Hl(e,a),n&4&&Zd(e,a);break;case 13:Hl(e,a),n&4&&Qd(e,a),n&64&&(e=a.memoizedState,e!==null&&(e=e.dehydrated,e!==null&&(a=rv.bind(null,a),Av(e,a))));break;case 22:if(n=a.memoizedState!==null||Ol,!n){t=t!==null&&t.memoizedState!==null||ua,s=Ol;var u=ua;Ol=n,(ua=t)&&!u?Ll(e,a,(a.subtreeFlags&8772)!==0):Hl(e,a),Ol=s,ua=u}break;case 30:break;default:Hl(e,a)}}function Wd(e){var t=e.alternate;t!==null&&(e.alternate=null,Wd(t)),e.child=null,e.deletions=null,e.sibling=null,e.tag===5&&(t=e.stateNode,t!==null&&Zs(t)),e.stateNode=null,e.return=null,e.dependencies=null,e.memoizedProps=null,e.memoizedState=null,e.pendingProps=null,e.stateNode=null,e.updateQueue=null}var Xt=null,cn=!1;function Bl(e,t,a){for(a=a.child;a!==null;)Xd(e,t,a),a=a.sibling}function Xd(e,t,a){if(na&&typeof na.onCommitFiberUnmount=="function")try{na.onCommitFiberUnmount(gn,a)}catch{}switch(a.tag){case 26:ua||Io(a,t),Bl(e,t,a),a.memoizedState?a.memoizedState.count--:a.stateNode&&(a=a.stateNode,a.parentNode.removeChild(a));break;case 27:ua||Io(a,t);var n=Xt,s=cn;ks(a.type)&&(Xt=a.stateNode,cn=!1),Bl(e,t,a),Cc(a.stateNode),Xt=n,cn=s;break;case 5:ua||Io(a,t);case 6:if(n=Xt,s=cn,Xt=null,Bl(e,t,a),Xt=n,cn=s,Xt!==null)if(cn)try{(Xt.nodeType===9?Xt.body:Xt.nodeName==="HTML"?Xt.ownerDocument.body:Xt).removeChild(a.stateNode)}catch(u){Ut(a,t,u)}else try{Xt.removeChild(a.stateNode)}catch(u){Ut(a,t,u)}break;case 18:Xt!==null&&(cn?(e=Xt,Fm(e.nodeType===9?e.body:e.nodeName==="HTML"?e.ownerDocument.body:e,a.stateNode),zr(e)):Fm(Xt,a.stateNode));break;case 4:n=Xt,s=cn,Xt=a.stateNode.containerInfo,cn=!0,Bl(e,t,a),Xt=n,cn=s;break;case 0:case 11:case 14:case 15:_s(2,a,t),ua||_s(4,a,t),Bl(e,t,a);break;case 1:ua||(Io(a,t),n=a.stateNode,typeof n.componentWillUnmount=="function"&&Gd(a,t,n)),Bl(e,t,a);break;case 21:Bl(e,t,a);break;case 22:ua=(n=ua)||a.memoizedState!==null,Bl(e,t,a),ua=n;break;default:Bl(e,t,a)}}function Zd(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null))){e=e.dehydrated;try{zr(e)}catch(a){Ut(t,t.return,a)}}}function Qd(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null&&(e=e.dehydrated,e!==null))))try{zr(e)}catch(a){Ut(t,t.return,a)}}function ev(e){switch(e.tag){case 31:case 13:case 19:var t=e.stateNode;return t===null&&(t=e.stateNode=new Pd),t;case 22:return e=e.stateNode,t=e._retryCache,t===null&&(t=e._retryCache=new Pd),t;default:throw Error(c(435,e.tag))}}function w0(e,t){var a=ev(e);t.forEach(function(n){if(!a.has(n)){a.add(n);var s=cv.bind(null,e,n);n.then(s,s)}})}function un(e,t){var a=t.deletions;if(a!==null)for(var n=0;n<a.length;n++){var s=a[n],u=e,p=t,w=p;e:for(;w!==null;){switch(w.tag){case 27:if(ks(w.type)){Xt=w.stateNode,cn=!1;break e}break;case 5:Xt=w.stateNode,cn=!1;break e;case 3:case 4:Xt=w.stateNode.containerInfo,cn=!0;break e}w=w.return}if(Xt===null)throw Error(c(160));Xd(u,p,s),Xt=null,cn=!1,u=s.alternate,u!==null&&(u.return=null),s.return=null}if(t.subtreeFlags&13886)for(t=t.child;t!==null;)Id(t,e),t=t.sibling}var xo=null;function Id(e,t){var a=e.alternate,n=e.flags;switch(e.tag){case 0:case 11:case 14:case 15:un(t,e),fn(e),n&4&&(_s(3,e,e.return),pc(3,e),_s(5,e,e.return));break;case 1:un(t,e),fn(e),n&512&&(ua||a===null||Io(a,a.return)),n&64&&Ol&&(e=e.updateQueue,e!==null&&(n=e.callbacks,n!==null&&(a=e.shared.hiddenCallbacks,e.shared.hiddenCallbacks=a===null?n:a.concat(n))));break;case 26:var s=xo;if(un(t,e),fn(e),n&512&&(ua||a===null||Io(a,a.return)),n&4){var u=a!==null?a.memoizedState:null;if(n=e.memoizedState,a===null)if(n===null)if(e.stateNode===null){e:{n=e.type,a=e.memoizedProps,s=s.ownerDocument||s;t:switch(n){case"title":u=s.getElementsByTagName("title")[0],(!u||u[fl]||u[oa]||u.namespaceURI==="http://www.w3.org/2000/svg"||u.hasAttribute("itemprop"))&&(u=s.createElement(n),s.head.insertBefore(u,s.querySelector("head > title"))),Ca(u,n,a),u[oa]=e,ea(u),n=u;break e;case"link":var p=Im("link","href",s).get(n+(a.href||""));if(p){for(var w=0;w<p.length;w++)if(u=p[w],u.getAttribute("href")===(a.href==null||a.href===""?null:a.href)&&u.getAttribute("rel")===(a.rel==null?null:a.rel)&&u.getAttribute("title")===(a.title==null?null:a.title)&&u.getAttribute("crossorigin")===(a.crossOrigin==null?null:a.crossOrigin)){p.splice(w,1);break t}}u=s.createElement(n),Ca(u,n,a),s.head.appendChild(u);break;case"meta":if(p=Im("meta","content",s).get(n+(a.content||""))){for(w=0;w<p.length;w++)if(u=p[w],u.getAttribute("content")===(a.content==null?null:""+a.content)&&u.getAttribute("name")===(a.name==null?null:a.name)&&u.getAttribute("property")===(a.property==null?null:a.property)&&u.getAttribute("http-equiv")===(a.httpEquiv==null?null:a.httpEquiv)&&u.getAttribute("charset")===(a.charSet==null?null:a.charSet)){p.splice(w,1);break t}}u=s.createElement(n),Ca(u,n,a),s.head.appendChild(u);break;default:throw Error(c(468,n))}u[oa]=e,ea(u),n=u}e.stateNode=n}else Jm(s,e.type,e.stateNode);else e.stateNode=Qm(s,n,e.memoizedProps);else u!==n?(u===null?a.stateNode!==null&&(a=a.stateNode,a.parentNode.removeChild(a)):u.count--,n===null?Jm(s,e.type,e.stateNode):Qm(s,n,e.memoizedProps)):n===null&&e.stateNode!==null&&sf(e,e.memoizedProps,a.memoizedProps)}break;case 27:un(t,e),fn(e),n&512&&(ua||a===null||Io(a,a.return)),a!==null&&n&4&&sf(e,e.memoizedProps,a.memoizedProps);break;case 5:if(un(t,e),fn(e),n&512&&(ua||a===null||Io(a,a.return)),e.flags&32){s=e.stateNode;try{ml(s,"")}catch(We){Ut(e,e.return,We)}}n&4&&e.stateNode!=null&&(s=e.memoizedProps,sf(e,s,a!==null?a.memoizedProps:s)),n&1024&&(uf=!0);break;case 6:if(un(t,e),fn(e),n&4){if(e.stateNode===null)throw Error(c(162));n=e.memoizedProps,a=e.stateNode;try{a.nodeValue=n}catch(We){Ut(e,e.return,We)}}break;case 3:if(B0=null,s=xo,xo=U0(t.containerInfo),un(t,e),xo=s,fn(e),n&4&&a!==null&&a.memoizedState.isDehydrated)try{zr(t.containerInfo)}catch(We){Ut(e,e.return,We)}uf&&(uf=!1,Jd(e));break;case 4:n=xo,xo=U0(e.stateNode.containerInfo),un(t,e),fn(e),xo=n;break;case 12:un(t,e),fn(e);break;case 31:un(t,e),fn(e),n&4&&(n=e.updateQueue,n!==null&&(e.updateQueue=null,w0(e,n)));break;case 13:un(t,e),fn(e),e.child.flags&8192&&e.memoizedState!==null!=(a!==null&&a.memoizedState!==null)&&(M0=Dt()),n&4&&(n=e.updateQueue,n!==null&&(e.updateQueue=null,w0(e,n)));break;case 22:s=e.memoizedState!==null;var q=a!==null&&a.memoizedState!==null,fe=Ol,Te=ua;if(Ol=fe||s,ua=Te||q,un(t,e),ua=Te,Ol=fe,fn(e),n&8192)e:for(t=e.stateNode,t._visibility=s?t._visibility&-2:t._visibility|1,s&&(a===null||q||Ol||ua||Mi(e)),a=null,t=e;;){if(t.tag===5||t.tag===26){if(a===null){q=a=t;try{if(u=q.stateNode,s)p=u.style,typeof p.setProperty=="function"?p.setProperty("display","none","important"):p.display="none";else{w=q.stateNode;var Re=q.memoizedProps.style,de=Re!=null&&Re.hasOwnProperty("display")?Re.display:null;w.style.display=de==null||typeof de=="boolean"?"":(""+de).trim()}}catch(We){Ut(q,q.return,We)}}}else if(t.tag===6){if(a===null){q=t;try{q.stateNode.nodeValue=s?"":q.memoizedProps}catch(We){Ut(q,q.return,We)}}}else if(t.tag===18){if(a===null){q=t;try{var xe=q.stateNode;s?Gm(xe,!0):Gm(q.stateNode,!1)}catch(We){Ut(q,q.return,We)}}}else if((t.tag!==22&&t.tag!==23||t.memoizedState===null||t===e)&&t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break e;for(;t.sibling===null;){if(t.return===null||t.return===e)break e;a===t&&(a=null),t=t.return}a===t&&(a=null),t.sibling.return=t.return,t=t.sibling}n&4&&(n=e.updateQueue,n!==null&&(a=n.retryQueue,a!==null&&(n.retryQueue=null,w0(e,a))));break;case 19:un(t,e),fn(e),n&4&&(n=e.updateQueue,n!==null&&(e.updateQueue=null,w0(e,n)));break;case 30:break;case 21:break;default:un(t,e),fn(e)}}function fn(e){var t=e.flags;if(t&2){try{for(var a,n=e.return;n!==null;){if(Vd(n)){a=n;break}n=n.return}if(a==null)throw Error(c(160));switch(a.tag){case 27:var s=a.stateNode,u=rf(e);x0(e,u,s);break;case 5:var p=a.stateNode;a.flags&32&&(ml(p,""),a.flags&=-33);var w=rf(e);x0(e,w,p);break;case 3:case 4:var q=a.stateNode.containerInfo,fe=rf(e);cf(e,fe,q);break;default:throw Error(c(161))}}catch(Te){Ut(e,e.return,Te)}e.flags&=-3}t&4096&&(e.flags&=-4097)}function Jd(e){if(e.subtreeFlags&1024)for(e=e.child;e!==null;){var t=e;Jd(t),t.tag===5&&t.flags&1024&&t.stateNode.reset(),e=e.sibling}}function Hl(e,t){if(t.subtreeFlags&8772)for(t=t.child;t!==null;)Yd(e,t.alternate,t),t=t.sibling}function Mi(e){for(e=e.child;e!==null;){var t=e;switch(t.tag){case 0:case 11:case 14:case 15:_s(4,t,t.return),Mi(t);break;case 1:Io(t,t.return);var a=t.stateNode;typeof a.componentWillUnmount=="function"&&Gd(t,t.return,a),Mi(t);break;case 27:Cc(t.stateNode);case 26:case 5:Io(t,t.return),Mi(t);break;case 22:t.memoizedState===null&&Mi(t);break;case 30:Mi(t);break;default:Mi(t)}e=e.sibling}}function Ll(e,t,a){for(a=a&&(t.subtreeFlags&8772)!==0,t=t.child;t!==null;){var n=t.alternate,s=e,u=t,p=u.flags;switch(u.tag){case 0:case 11:case 15:Ll(s,u,a),pc(4,u);break;case 1:if(Ll(s,u,a),n=u,s=n.stateNode,typeof s.componentDidMount=="function")try{s.componentDidMount()}catch(fe){Ut(n,n.return,fe)}if(n=u,s=n.updateQueue,s!==null){var w=n.stateNode;try{var q=s.shared.hiddenCallbacks;if(q!==null)for(s.shared.hiddenCallbacks=null,s=0;s<q.length;s++)Ee(q[s],w)}catch(fe){Ut(n,n.return,fe)}}a&&p&64&&Fd(u),vc(u,u.return);break;case 27:Kd(u);case 26:case 5:Ll(s,u,a),a&&n===null&&p&4&&qd(u),vc(u,u.return);break;case 12:Ll(s,u,a);break;case 31:Ll(s,u,a),a&&p&4&&Zd(s,u);break;case 13:Ll(s,u,a),a&&p&4&&Qd(s,u);break;case 22:u.memoizedState===null&&Ll(s,u,a),vc(u,u.return);break;case 30:break;default:Ll(s,u,a)}t=t.sibling}}function ff(e,t){var a=null;e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(a=e.memoizedState.cachePool.pool),e=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(e=t.memoizedState.cachePool.pool),e!==a&&(e!=null&&e.refCount++,a!=null&&Wn(a))}function hf(e,t){e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&Wn(e))}function wo(e,t,a,n){if(t.subtreeFlags&10256)for(t=t.child;t!==null;)$d(e,t,a,n),t=t.sibling}function $d(e,t,a,n){var s=t.flags;switch(t.tag){case 0:case 11:case 15:wo(e,t,a,n),s&2048&&pc(9,t);break;case 1:wo(e,t,a,n);break;case 3:wo(e,t,a,n),s&2048&&(e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&Wn(e)));break;case 12:if(s&2048){wo(e,t,a,n),e=t.stateNode;try{var u=t.memoizedProps,p=u.id,w=u.onPostCommit;typeof w=="function"&&w(p,t.alternate===null?"mount":"update",e.passiveEffectDuration,-0)}catch(q){Ut(t,t.return,q)}}else wo(e,t,a,n);break;case 31:wo(e,t,a,n);break;case 13:wo(e,t,a,n);break;case 23:break;case 22:u=t.stateNode,p=t.alternate,t.memoizedState!==null?u._visibility&2?wo(e,t,a,n):gc(e,t):u._visibility&2?wo(e,t,a,n):(u._visibility|=2,vr(e,t,a,n,(t.subtreeFlags&10256)!==0||!1)),s&2048&&ff(p,t);break;case 24:wo(e,t,a,n),s&2048&&hf(t.alternate,t);break;default:wo(e,t,a,n)}}function vr(e,t,a,n,s){for(s=s&&((t.subtreeFlags&10256)!==0||!1),t=t.child;t!==null;){var u=e,p=t,w=a,q=n,fe=p.flags;switch(p.tag){case 0:case 11:case 15:vr(u,p,w,q,s),pc(8,p);break;case 23:break;case 22:var Te=p.stateNode;p.memoizedState!==null?Te._visibility&2?vr(u,p,w,q,s):gc(u,p):(Te._visibility|=2,vr(u,p,w,q,s)),s&&fe&2048&&ff(p.alternate,p);break;case 24:vr(u,p,w,q,s),s&&fe&2048&&hf(p.alternate,p);break;default:vr(u,p,w,q,s)}t=t.sibling}}function gc(e,t){if(t.subtreeFlags&10256)for(t=t.child;t!==null;){var a=e,n=t,s=n.flags;switch(n.tag){case 22:gc(a,n),s&2048&&ff(n.alternate,n);break;case 24:gc(a,n),s&2048&&hf(n.alternate,n);break;default:gc(a,n)}t=t.sibling}}var yc=8192;function gr(e,t,a){if(e.subtreeFlags&yc)for(e=e.child;e!==null;)em(e,t,a),e=e.sibling}function em(e,t,a){switch(e.tag){case 26:gr(e,t,a),e.flags&yc&&e.memoizedState!==null&&Fv(a,xo,e.memoizedState,e.memoizedProps);break;case 5:gr(e,t,a);break;case 3:case 4:var n=xo;xo=U0(e.stateNode.containerInfo),gr(e,t,a),xo=n;break;case 22:e.memoizedState===null&&(n=e.alternate,n!==null&&n.memoizedState!==null?(n=yc,yc=16777216,gr(e,t,a),yc=n):gr(e,t,a));break;default:gr(e,t,a)}}function tm(e){var t=e.alternate;if(t!==null&&(e=t.child,e!==null)){t.child=null;do t=e.sibling,e.sibling=null,e=t;while(e!==null)}}function xc(e){var t=e.deletions;if((e.flags&16)!==0){if(t!==null)for(var a=0;a<t.length;a++){var n=t[a];wa=n,nm(n,e)}tm(e)}if(e.subtreeFlags&10256)for(e=e.child;e!==null;)am(e),e=e.sibling}function am(e){switch(e.tag){case 0:case 11:case 15:xc(e),e.flags&2048&&_s(9,e,e.return);break;case 3:xc(e);break;case 12:xc(e);break;case 22:var t=e.stateNode;e.memoizedState!==null&&t._visibility&2&&(e.return===null||e.return.tag!==13)?(t._visibility&=-3,b0(e)):xc(e);break;default:xc(e)}}function b0(e){var t=e.deletions;if((e.flags&16)!==0){if(t!==null)for(var a=0;a<t.length;a++){var n=t[a];wa=n,nm(n,e)}tm(e)}for(e=e.child;e!==null;){switch(t=e,t.tag){case 0:case 11:case 15:_s(8,t,t.return),b0(t);break;case 22:a=t.stateNode,a._visibility&2&&(a._visibility&=-3,b0(t));break;default:b0(t)}e=e.sibling}}function nm(e,t){for(;wa!==null;){var a=wa;switch(a.tag){case 0:case 11:case 15:_s(8,a,t);break;case 23:case 22:if(a.memoizedState!==null&&a.memoizedState.cachePool!==null){var n=a.memoizedState.cachePool.pool;n!=null&&n.refCount++}break;case 24:Wn(a.memoizedState.cache)}if(n=a.child,n!==null)n.return=a,wa=n;else e:for(a=e;wa!==null;){n=wa;var s=n.sibling,u=n.return;if(Wd(n),n===a){wa=null;break e}if(s!==null){s.return=u,wa=s;break e}wa=u}}}var tv={getCacheForType:function(e){var t=ra(Kt),a=t.data.get(e);return a===void 0&&(a=e(),t.data.set(e,a)),a},cacheSignal:function(){return ra(Kt).controller.signal}},av=typeof WeakMap=="function"?WeakMap:Map,At=0,jt=null,gt=null,wt=0,kt=0,An=null,zs=!1,yr=!1,df=!1,jl=0,ta=0,As=0,Si=0,mf=0,En=0,xr=0,wc=null,hn=null,pf=!1,M0=0,om=0,S0=1/0,T0=null,Es=null,da=0,Rs=null,wr=null,Fl=0,vf=0,gf=null,lm=null,bc=0,yf=null;function Rn(){return(At&2)!==0&&wt!==0?wt&-wt:N.T!==null?Tf():Br()}function sm(){if(En===0)if((wt&536870912)===0||vt){var e=Wa;Wa<<=1,(Wa&3932160)===0&&(Wa=262144),En=e}else En=536870912;return e=Oe.current,e!==null&&(e.flags|=32),En}function dn(e,t,a){(e===jt&&(kt===2||kt===9)||e.cancelPendingCommit!==null)&&(br(e,0),Ns(e,wt,En,!1)),Ys(e,a),((At&2)===0||e!==jt)&&(e===jt&&((At&2)===0&&(Si|=a),ta===4&&Ns(e,wt,En,!1)),Jo(e))}function im(e,t,a){if((At&6)!==0)throw Error(c(327));var n=!a&&(t&127)===0&&(t&e.expiredLanes)===0||ul(e,t),s=n?lv(e,t):wf(e,t,!0),u=n;do{if(s===0){yr&&!n&&Ns(e,t,0,!1);break}else{if(a=e.current.alternate,u&&!nv(a)){s=wf(e,t,!1),u=!1;continue}if(s===2){if(u=t,e.errorRecoveryDisabledLanes&u)var p=0;else p=e.pendingLanes&-536870913,p=p!==0?p:p&536870912?536870912:0;if(p!==0){t=p;e:{var w=e;s=wc;var q=w.current.memoizedState.isDehydrated;if(q&&(br(w,p).flags|=256),p=wf(w,p,!1),p!==2){if(df&&!q){w.errorRecoveryDisabledLanes|=u,Si|=u,s=4;break e}u=hn,hn=s,u!==null&&(hn===null?hn=u:hn.push.apply(hn,u))}s=p}if(u=!1,s!==2)continue}}if(s===1){br(e,0),Ns(e,t,0,!0);break}e:{switch(n=e,u=s,u){case 0:case 1:throw Error(c(345));case 4:if((t&4194048)!==t)break;case 6:Ns(n,t,En,!zs);break e;case 2:hn=null;break;case 3:case 5:break;default:throw Error(c(329))}if((t&62914560)===t&&(s=M0+300-Dt(),10<s)){if(Ns(n,t,En,!zs),Ps(n,0,!0)!==0)break e;Fl=t,n.timeoutHandle=Lm(rm.bind(null,n,a,hn,T0,pf,t,En,Si,xr,zs,u,"Throttled",-0,0),s);break e}rm(n,a,hn,T0,pf,t,En,Si,xr,zs,u,null,-0,0)}}break}while(!0);Jo(e)}function rm(e,t,a,n,s,u,p,w,q,fe,Te,Re,de,xe){if(e.timeoutHandle=-1,Re=t.subtreeFlags,Re&8192||(Re&16785408)===16785408){Re={stylesheets:null,count:0,imgCount:0,imgBytes:0,suspenseyImages:[],waitingForImages:!0,waitingForViewTransition:!1,unsuspend:no},em(t,u,Re);var We=(u&62914560)===u?M0-Dt():(u&4194048)===u?om-Dt():0;if(We=Gv(Re,We),We!==null){Fl=u,e.cancelPendingCommit=We(vm.bind(null,e,t,u,a,n,s,p,w,q,Te,Re,null,de,xe)),Ns(e,u,p,!fe);return}}vm(e,t,u,a,n,s,p,w,q)}function nv(e){for(var t=e;;){var a=t.tag;if((a===0||a===11||a===15)&&t.flags&16384&&(a=t.updateQueue,a!==null&&(a=a.stores,a!==null)))for(var n=0;n<a.length;n++){var s=a[n],u=s.getSnapshot;s=s.value;try{if(!va(u(),s))return!1}catch{return!1}}if(a=t.child,t.subtreeFlags&16384&&a!==null)a.return=t,t=a;else{if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return!0;t=t.return}t.sibling.return=t.return,t=t.sibling}}return!0}function Ns(e,t,a,n){t&=~mf,t&=~Si,e.suspendedLanes|=t,e.pingedLanes&=~t,n&&(e.warmLanes|=t),n=e.expirationTimes;for(var s=t;0<s;){var u=31-ha(s),p=1<<u;n[u]=-1,s&=~p}a!==0&&Or(e,a,t)}function C0(){return(At&6)===0?(Mc(0),!1):!0}function xf(){if(gt!==null){if(kt===0)var e=gt.return;else e=gt,on=co=null,Cs(e),Ms=null,Ss=0,e=gt;for(;e!==null;)jd(e.alternate,e),e=e.return;gt=null}}function br(e,t){var a=e.timeoutHandle;a!==-1&&(e.timeoutHandle=-1,Sv(a)),a=e.cancelPendingCommit,a!==null&&(e.cancelPendingCommit=null,a()),Fl=0,xf(),jt=e,gt=a=tn(e.current,null),wt=t,kt=0,An=null,zs=!1,yr=ul(e,t),df=!1,xr=En=mf=Si=As=ta=0,hn=wc=null,pf=!1,(t&8)!==0&&(t|=t&32);var n=e.entangledLanes;if(n!==0)for(e=e.entanglements,n&=t;0<n;){var s=31-ha(n),u=1<<s;t|=e[s],n&=~u}return jl=t,ui(),a}function cm(e,t){je=null,N.H=hc,t===mo||t===Dl?(t=ln(),kt=3):t===Za?(t=ln(),kt=4):kt=t===Qu?8:t!==null&&typeof t=="object"&&typeof t.then=="function"?6:1,An=t,gt===null&&(ta=1,m0(e,ga(t,e.current)))}function um(){var e=Oe.current;return e===null?!0:(wt&4194048)===wt?Fe===null:(wt&62914560)===wt||(wt&536870912)!==0?e===Fe:!1}function fm(){var e=N.H;return N.H=hc,e===null?hc:e}function hm(){var e=N.A;return N.A=tv,e}function _0(){ta=4,zs||(wt&4194048)!==wt&&Oe.current!==null||(yr=!0),(As&134217727)===0&&(Si&134217727)===0||jt===null||Ns(jt,wt,En,!1)}function wf(e,t,a){var n=At;At|=2;var s=fm(),u=hm();(jt!==e||wt!==t)&&(T0=null,br(e,t)),t=!1;var p=ta;e:do try{if(kt!==0&&gt!==null){var w=gt,q=An;switch(kt){case 8:xf(),p=6;break e;case 3:case 2:case 9:case 6:Oe.current===null&&(t=!0);var fe=kt;if(kt=0,An=null,Mr(e,w,q,fe),a&&yr){p=0;break e}break;default:fe=kt,kt=0,An=null,Mr(e,w,q,fe)}}ov(),p=ta;break}catch(Te){cm(e,Te)}while(!0);return t&&e.shellSuspendCounter++,on=co=null,At=n,N.H=s,N.A=u,gt===null&&(jt=null,wt=0,ui()),p}function ov(){for(;gt!==null;)dm(gt)}function lv(e,t){var a=At;At|=2;var n=fm(),s=hm();jt!==e||wt!==t?(T0=null,S0=Dt()+500,br(e,t)):yr=ul(e,t);e:do try{if(kt!==0&&gt!==null){t=gt;var u=An;t:switch(kt){case 1:kt=0,An=null,Mr(e,t,u,1);break;case 2:case 9:if(cc(u)){kt=0,An=null,mm(t);break}t=function(){kt!==2&&kt!==9||jt!==e||(kt=7),Jo(e)},u.then(t,t);break e;case 3:kt=7;break e;case 4:kt=5;break e;case 7:cc(u)?(kt=0,An=null,mm(t)):(kt=0,An=null,Mr(e,t,u,7));break;case 5:var p=null;switch(gt.tag){case 26:p=gt.memoizedState;case 5:case 27:var w=gt;if(p?$m(p):w.stateNode.complete){kt=0,An=null;var q=w.sibling;if(q!==null)gt=q;else{var fe=w.return;fe!==null?(gt=fe,z0(fe)):gt=null}break t}}kt=0,An=null,Mr(e,t,u,5);break;case 6:kt=0,An=null,Mr(e,t,u,6);break;case 8:xf(),ta=6;break e;default:throw Error(c(462))}}sv();break}catch(Te){cm(e,Te)}while(!0);return on=co=null,N.H=n,N.A=s,At=a,gt!==null?0:(jt=null,wt=0,ui(),ta)}function sv(){for(;gt!==null&&!mn();)dm(gt)}function dm(e){var t=Hd(e.alternate,e,jl);e.memoizedProps=e.pendingProps,t===null?z0(e):gt=t}function mm(e){var t=e,a=t.alternate;switch(t.tag){case 15:case 0:t=Nd(a,t,t.pendingProps,t.type,void 0,wt);break;case 11:t=Nd(a,t,t.pendingProps,t.type.render,t.ref,wt);break;case 5:Cs(t);default:jd(a,t),t=gt=lr(t,jl),t=Hd(a,t,jl)}e.memoizedProps=e.pendingProps,t===null?z0(e):gt=t}function Mr(e,t,a,n){on=co=null,Cs(t),Ms=null,Ss=0;var s=t.return;try{if(Xp(e,s,t,a,wt)){ta=1,m0(e,ga(a,e.current)),gt=null;return}}catch(u){if(s!==null)throw gt=s,u;ta=1,m0(e,ga(a,e.current)),gt=null;return}t.flags&32768?(vt||n===1?e=!0:yr||(wt&536870912)!==0?e=!1:(zs=e=!0,(n===2||n===9||n===3||n===6)&&(n=Oe.current,n!==null&&n.tag===13&&(n.flags|=16384))),pm(t,e)):z0(t)}function z0(e){var t=e;do{if((t.flags&32768)!==0){pm(t,zs);return}e=t.return;var a=Ip(t.alternate,t,jl);if(a!==null){gt=a;return}if(t=t.sibling,t!==null){gt=t;return}gt=t=e}while(t!==null);ta===0&&(ta=5)}function pm(e,t){do{var a=Jp(e.alternate,e);if(a!==null){a.flags&=32767,gt=a;return}if(a=e.return,a!==null&&(a.flags|=32768,a.subtreeFlags=0,a.deletions=null),!t&&(e=e.sibling,e!==null)){gt=e;return}gt=e=a}while(e!==null);ta=6,gt=null}function vm(e,t,a,n,s,u,p,w,q){e.cancelPendingCommit=null;do A0();while(da!==0);if((At&6)!==0)throw Error(c(327));if(t!==null){if(t===e.current)throw Error(c(177));if(u=t.lanes|t.childLanes,u|=nr,Vc(e,a,u,p,w,q),e===jt&&(gt=jt=null,wt=0),wr=t,Rs=e,Fl=a,vf=u,gf=s,lm=n,(t.subtreeFlags&10256)!==0||(t.flags&10256)!==0?(e.callbackNode=null,e.callbackPriority=0,uv(Ya,function(){return bm(),null})):(e.callbackNode=null,e.callbackPriority=0),n=(t.flags&13878)!==0,(t.subtreeFlags&13878)!==0||n){n=N.T,N.T=null,s=P.p,P.p=2,p=At,At|=4;try{$p(e,t,a)}finally{At=p,P.p=s,N.T=n}}da=1,gm(),ym(),xm()}}function gm(){if(da===1){da=0;var e=Rs,t=wr,a=(t.flags&13878)!==0;if((t.subtreeFlags&13878)!==0||a){a=N.T,N.T=null;var n=P.p;P.p=2;var s=At;At|=4;try{Id(t,e);var u=Df,p=Ii(e.containerInfo),w=u.focusedElem,q=u.selectionRange;if(p!==w&&w&&w.ownerDocument&&Qi(w.ownerDocument.documentElement,w)){if(q!==null&&Gn(w)){var fe=q.start,Te=q.end;if(Te===void 0&&(Te=fe),"selectionStart"in w)w.selectionStart=fe,w.selectionEnd=Math.min(Te,w.value.length);else{var Re=w.ownerDocument||document,de=Re&&Re.defaultView||window;if(de.getSelection){var xe=de.getSelection(),We=w.textContent.length,at=Math.min(q.start,We),Bt=q.end===void 0?at:Math.min(q.end,We);!xe.extend&&at>Bt&&(p=Bt,Bt=at,at=p);var ae=ec(w,at),W=ec(w,Bt);if(ae&&W&&(xe.rangeCount!==1||xe.anchorNode!==ae.node||xe.anchorOffset!==ae.offset||xe.focusNode!==W.node||xe.focusOffset!==W.offset)){var ce=Re.createRange();ce.setStart(ae.node,ae.offset),xe.removeAllRanges(),at>Bt?(xe.addRange(ce),xe.extend(W.node,W.offset)):(ce.setEnd(W.node,W.offset),xe.addRange(ce))}}}}for(Re=[],xe=w;xe=xe.parentNode;)xe.nodeType===1&&Re.push({element:xe,left:xe.scrollLeft,top:xe.scrollTop});for(typeof w.focus=="function"&&w.focus(),w=0;w<Re.length;w++){var ze=Re[w];ze.element.scrollLeft=ze.left,ze.element.scrollTop=ze.top}}F0=!!Nf,Df=Nf=null}finally{At=s,P.p=n,N.T=a}}e.current=t,da=2}}function ym(){if(da===2){da=0;var e=Rs,t=wr,a=(t.flags&8772)!==0;if((t.subtreeFlags&8772)!==0||a){a=N.T,N.T=null;var n=P.p;P.p=2;var s=At;At|=4;try{Yd(e,t.alternate,t)}finally{At=s,P.p=n,N.T=a}}da=3}}function xm(){if(da===4||da===3){da=0,Ea();var e=Rs,t=wr,a=Fl,n=lm;(t.subtreeFlags&10256)!==0||(t.flags&10256)!==0?da=5:(da=0,wr=Rs=null,wm(e,e.pendingLanes));var s=e.pendingLanes;if(s===0&&(Es=null),es(a),t=t.stateNode,na&&typeof na.onCommitFiberRoot=="function")try{na.onCommitFiberRoot(gn,t,void 0,(t.current.flags&128)===128)}catch{}if(n!==null){t=N.T,s=P.p,P.p=2,N.T=null;try{for(var u=e.onRecoverableError,p=0;p<n.length;p++){var w=n[p];u(w.value,{componentStack:w.stack})}}finally{N.T=t,P.p=s}}(Fl&3)!==0&&A0(),Jo(e),s=e.pendingLanes,(a&261930)!==0&&(s&42)!==0?e===yf?bc++:(bc=0,yf=e):bc=0,Mc(0)}}function wm(e,t){(e.pooledCacheLanes&=t)===0&&(t=e.pooledCache,t!=null&&(e.pooledCache=null,Wn(t)))}function A0(){return gm(),ym(),xm(),bm()}function bm(){if(da!==5)return!1;var e=Rs,t=vf;vf=0;var a=es(Fl),n=N.T,s=P.p;try{P.p=32>a?32:a,N.T=null,a=gf,gf=null;var u=Rs,p=Fl;if(da=0,wr=Rs=null,Fl=0,(At&6)!==0)throw Error(c(331));var w=At;if(At|=4,am(u.current),$d(u,u.current,p,a),At=w,Mc(0,!1),na&&typeof na.onPostCommitFiberRoot=="function")try{na.onPostCommitFiberRoot(gn,u)}catch{}return!0}finally{P.p=s,N.T=n,wm(e,t)}}function Mm(e,t,a){t=ga(a,t),t=Zu(e.stateNode,t,2),e=ne(e,t,2),e!==null&&(Ys(e,2),Jo(e))}function Ut(e,t,a){if(e.tag===3)Mm(e,e,a);else for(;t!==null;){if(t.tag===3){Mm(t,e,a);break}else if(t.tag===1){var n=t.stateNode;if(typeof t.type.getDerivedStateFromError=="function"||typeof n.componentDidCatch=="function"&&(Es===null||!Es.has(n))){e=ga(a,e),a=Sd(2),n=ne(t,a,2),n!==null&&(Td(a,n,t,e),Ys(n,2),Jo(n));break}}t=t.return}}function bf(e,t,a){var n=e.pingCache;if(n===null){n=e.pingCache=new av;var s=new Set;n.set(t,s)}else s=n.get(t),s===void 0&&(s=new Set,n.set(t,s));s.has(a)||(df=!0,s.add(a),e=iv.bind(null,e,t,a),t.then(e,e))}function iv(e,t,a){var n=e.pingCache;n!==null&&n.delete(t),e.pingedLanes|=e.suspendedLanes&a,e.warmLanes&=~a,jt===e&&(wt&a)===a&&(ta===4||ta===3&&(wt&62914560)===wt&&300>Dt()-M0?(At&2)===0&&br(e,0):mf|=a,xr===wt&&(xr=0)),Jo(e)}function Sm(e,t){t===0&&(t=qc()),e=Vn(e,t),e!==null&&(Ys(e,t),Jo(e))}function rv(e){var t=e.memoizedState,a=0;t!==null&&(a=t.retryLane),Sm(e,a)}function cv(e,t){var a=0;switch(e.tag){case 31:case 13:var n=e.stateNode,s=e.memoizedState;s!==null&&(a=s.retryLane);break;case 19:n=e.stateNode;break;case 22:n=e.stateNode._retryCache;break;default:throw Error(c(314))}n!==null&&n.delete(t),Sm(e,a)}function uv(e,t){return Yt(e,t)}var E0=null,Sr=null,Mf=!1,R0=!1,Sf=!1,Ds=0;function Jo(e){e!==Sr&&e.next===null&&(Sr===null?E0=Sr=e:Sr=Sr.next=e),R0=!0,Mf||(Mf=!0,hv())}function Mc(e,t){if(!Sf&&R0){Sf=!0;do for(var a=!1,n=E0;n!==null;){if(e!==0){var s=n.pendingLanes;if(s===0)var u=0;else{var p=n.suspendedLanes,w=n.pingedLanes;u=(1<<31-ha(42|e)+1)-1,u&=s&~(p&~w),u=u&201326741?u&201326741|1:u?u|2:0}u!==0&&(a=!0,zm(n,u))}else u=wt,u=Ps(n,n===jt?u:0,n.cancelPendingCommit!==null||n.timeoutHandle!==-1),(u&3)===0||ul(n,u)||(a=!0,zm(n,u));n=n.next}while(a);Sf=!1}}function fv(){Tm()}function Tm(){R0=Mf=!1;var e=0;Ds!==0&&Mv()&&(e=Ds);for(var t=Dt(),a=null,n=E0;n!==null;){var s=n.next,u=Cm(n,t);u===0?(n.next=null,a===null?E0=s:a.next=s,s===null&&(Sr=a)):(a=n,(e!==0||(u&3)!==0)&&(R0=!0)),n=s}da!==0&&da!==5||Mc(e),Ds!==0&&(Ds=0)}function Cm(e,t){for(var a=e.suspendedLanes,n=e.pingedLanes,s=e.expirationTimes,u=e.pendingLanes&-62914561;0<u;){var p=31-ha(u),w=1<<p,q=s[p];q===-1?((w&a)===0||(w&n)!==0)&&(s[p]=Un(w,t)):q<=t&&(e.expiredLanes|=w),u&=~w}if(t=jt,a=wt,a=Ps(e,e===t?a:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),n=e.callbackNode,a===0||e===t&&(kt===2||kt===9)||e.cancelPendingCommit!==null)return n!==null&&n!==null&&Aa(n),e.callbackNode=null,e.callbackPriority=0;if((a&3)===0||ul(e,a)){if(t=a&-a,t===e.callbackPriority)return t;switch(n!==null&&Aa(n),es(a)){case 2:case 8:a=Ra;break;case 32:a=Ya;break;case 268435456:a=Na;break;default:a=Ya}return n=_m.bind(null,e),a=Yt(a,n),e.callbackPriority=t,e.callbackNode=a,t}return n!==null&&n!==null&&Aa(n),e.callbackPriority=2,e.callbackNode=null,2}function _m(e,t){if(da!==0&&da!==5)return e.callbackNode=null,e.callbackPriority=0,null;var a=e.callbackNode;if(A0()&&e.callbackNode!==a)return null;var n=wt;return n=Ps(e,e===jt?n:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),n===0?null:(im(e,n,t),Cm(e,Dt()),e.callbackNode!=null&&e.callbackNode===a?_m.bind(null,e):null)}function zm(e,t){if(A0())return null;im(e,t,!0)}function hv(){Tv(function(){(At&6)!==0?Yt(sa,fv):Tm()})}function Tf(){if(Ds===0){var e=ho;e===0&&(e=Da,Da<<=1,(Da&261888)===0&&(Da=256)),Ds=e}return Ds}function Am(e){return e==null||typeof e=="symbol"||typeof e=="boolean"?null:typeof e=="function"?e:vl(""+e)}function Em(e,t){var a=t.ownerDocument.createElement("input");return a.name=t.name,a.value=t.value,e.id&&a.setAttribute("form",e.id),t.parentNode.insertBefore(a,t),e=new FormData(e),a.parentNode.removeChild(a),e}function dv(e,t,a,n,s){if(t==="submit"&&a&&a.stateNode===s){var u=Am((s[ma]||null).action),p=n.submitter;p&&(t=(t=p[ma]||null)?Am(t.formAction):p.getAttribute("formAction"),t!==null&&(u=t,p=null));var w=new Gi("action","action",null,n,s);e.push({event:w,listeners:[{instance:null,listener:function(){if(n.defaultPrevented){if(Ds!==0){var q=p?Em(s,p):new FormData(s);Vu(a,{pending:!0,data:q,method:s.method,action:u},null,q)}}else typeof u=="function"&&(w.preventDefault(),q=p?Em(s,p):new FormData(s),Vu(a,{pending:!0,data:q,method:s.method,action:u},u,q))},currentTarget:s}]})}}for(var Cf=0;Cf<tr.length;Cf++){var _f=tr[Cf],mv=_f.toLowerCase(),pv=_f[0].toUpperCase()+_f.slice(1);en(mv,"on"+pv)}en(nc,"onAnimationEnd"),en(n0,"onAnimationIteration"),en(ms,"onAnimationStart"),en("dblclick","onDoubleClick"),en("focusin","onFocus"),en("focusout","onBlur"),en(Bu,"onTransitionRun"),en(o0,"onTransitionStart"),en(l0,"onTransitionCancel"),en(er,"onTransitionEnd"),ko("onMouseEnter",["mouseout","mouseover"]),ko("onMouseLeave",["mouseout","mouseover"]),ko("onPointerEnter",["pointerout","pointerover"]),ko("onPointerLeave",["pointerout","pointerover"]),ao("onChange","change click focusin focusout input keydown keyup selectionchange".split(" ")),ao("onSelect","focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange".split(" ")),ao("onBeforeInput",["compositionend","keypress","textInput","paste"]),ao("onCompositionEnd","compositionend focusout keydown keypress keyup mousedown".split(" ")),ao("onCompositionStart","compositionstart focusout keydown keypress keyup mousedown".split(" ")),ao("onCompositionUpdate","compositionupdate focusout keydown keypress keyup mousedown".split(" "));var Sc="abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting".split(" "),vv=new Set("beforetoggle cancel close invalid load scroll scrollend toggle".split(" ").concat(Sc));function Rm(e,t){t=(t&4)!==0;for(var a=0;a<e.length;a++){var n=e[a],s=n.event;n=n.listeners;e:{var u=void 0;if(t)for(var p=n.length-1;0<=p;p--){var w=n[p],q=w.instance,fe=w.currentTarget;if(w=w.listener,q!==u&&s.isPropagationStopped())break e;u=w,s.currentTarget=fe;try{u(s)}catch(Te){ar(Te)}s.currentTarget=null,u=q}else for(p=0;p<n.length;p++){if(w=n[p],q=w.instance,fe=w.currentTarget,w=w.listener,q!==u&&s.isPropagationStopped())break e;u=w,s.currentTarget=fe;try{u(s)}catch(Te){ar(Te)}s.currentTarget=null,u=q}}}}function yt(e,t){var a=t[Xs];a===void 0&&(a=t[Xs]=new Set);var n=e+"__bubble";a.has(n)||(Nm(t,e,2,!1),a.add(n))}function zf(e,t,a){var n=0;t&&(n|=4),Nm(a,e,n,t)}var N0="_reactListening"+Math.random().toString(36).slice(2);function Af(e){if(!e[N0]){e[N0]=!0,Oi.forEach(function(a){a!=="selectionchange"&&(vv.has(a)||zf(a,!1,e),zf(a,!0,e))});var t=e.nodeType===9?e:e.ownerDocument;t===null||t[N0]||(t[N0]=!0,zf("selectionchange",!1,t))}}function Nm(e,t,a,n){switch(s1(t)){case 2:var s=Kv;break;case 8:s=Pv;break;default:s=Vf}a=s.bind(null,t,a,e),s=void 0,!is||t!=="touchstart"&&t!=="touchmove"&&t!=="wheel"||(s=!0),n?s!==void 0?e.addEventListener(t,a,{capture:!0,passive:s}):e.addEventListener(t,a,!0):s!==void 0?e.addEventListener(t,a,{passive:s}):e.addEventListener(t,a,!1)}function Ef(e,t,a,n,s){var u=n;if((t&1)===0&&(t&2)===0&&n!==null)e:for(;;){if(n===null)return;var p=n.tag;if(p===3||p===4){var w=n.stateNode.containerInfo;if(w===s)break;if(p===4)for(p=n.return;p!==null;){var q=p.tag;if((q===3||q===4)&&p.stateNode.containerInfo===s)return;p=p.return}for(;w!==null;){if(p=Ro(w),p===null)return;if(q=p.tag,q===5||q===6||q===26||q===27){n=u=p;continue e}w=w.parentNode}}n=n.return}Wc(function(){var fe=u,Te=it(a),Re=[];e:{var de=s0.get(e);if(de!==void 0){var xe=Gi,We=e;switch(e){case"keypress":if(Et(a)===0)break e;case"keydown":case"keyup":xe=Ru;break;case"focusin":We="focus",xe=ti;break;case"focusout":We="blur",xe=ti;break;case"beforeblur":case"afterblur":xe=ti;break;case"click":if(a.button===2)break e;case"auxclick":case"dblclick":case"mousedown":case"mousemove":case"mouseup":case"mouseout":case"mouseover":case"contextmenu":xe=wn;break;case"drag":case"dragend":case"dragenter":case"dragexit":case"dragleave":case"dragover":case"dragstart":case"drop":xe=ei;break;case"touchcancel":case"touchend":case"touchmove":case"touchstart":xe=Nu;break;case nc:case n0:case ms:xe=Ic;break;case er:xe=fs;break;case"scroll":case"scrollend":xe=Su;break;case"wheel":xe=Pr;break;case"copy":case"cut":case"paste":xe=Cu;break;case"gotpointercapture":case"lostpointercapture":case"pointercancel":case"pointerdown":case"pointermove":case"pointerout":case"pointerover":case"pointerup":xe=Vr;break;case"toggle":case"beforetoggle":xe=Wr}var at=(t&4)!==0,Bt=!at&&(e==="scroll"||e==="scrollend"),ae=at?de!==null?de+"Capture":null:de;at=[];for(var W=fe,ce;W!==null;){var ze=W;if(ce=ze.stateNode,ze=ze.tag,ze!==5&&ze!==26&&ze!==27||ce===null||ae===null||(ze=gl(W,ae),ze!=null&&at.push(Tc(W,ze,ce))),Bt)break;W=W.return}0<at.length&&(de=new xe(de,We,null,a,Te),Re.push({event:de,listeners:at}))}}if((t&7)===0){e:{if(de=e==="mouseover"||e==="pointerover",xe=e==="mouseout"||e==="pointerout",de&&a!==st&&(We=a.relatedTarget||a.fromElement)&&(Ro(We)||We[to]))break e;if((xe||de)&&(de=Te.window===Te?Te:(de=Te.ownerDocument)?de.defaultView||de.parentWindow:window,xe?(We=a.relatedTarget||a.toElement,xe=fe,We=We?Ro(We):null,We!==null&&(Bt=f(We),at=We.tag,We!==Bt||at!==5&&at!==27&&at!==6)&&(We=null)):(xe=null,We=fe),xe!==We)){if(at=wn,ze="onMouseLeave",ae="onMouseEnter",W="mouse",(e==="pointerout"||e==="pointerover")&&(at=Vr,ze="onPointerLeave",ae="onPointerEnter",W="pointer"),Bt=xe==null?de:hl(xe),ce=We==null?de:hl(We),de=new at(ze,W+"leave",xe,a,Te),de.target=Bt,de.relatedTarget=ce,ze=null,Ro(Te)===fe&&(at=new at(ae,W+"enter",We,a,Te),at.target=ce,at.relatedTarget=Bt,ze=at),Bt=ze,xe&&We)t:{for(at=gv,ae=xe,W=We,ce=0,ze=ae;ze;ze=at(ze))ce++;ze=0;for(var et=W;et;et=at(et))ze++;for(;0<ce-ze;)ae=at(ae),ce--;for(;0<ze-ce;)W=at(W),ze--;for(;ce--;){if(ae===W||W!==null&&ae===W.alternate){at=ae;break t}ae=at(ae),W=at(W)}at=null}else at=null;xe!==null&&Dm(Re,de,xe,at,!1),We!==null&&Bt!==null&&Dm(Re,Bt,We,at,!0)}}e:{if(de=fe?hl(fe):window,xe=de.nodeName&&de.nodeName.toLowerCase(),xe==="select"||xe==="input"&&de.type==="file")var Ct=Zi;else if(Yi(de))if(jn)Ct=a0;else{Ct=Ir;var Qe=Ou}else xe=de.nodeName,!xe||xe.toLowerCase()!=="input"||de.type!=="checkbox"&&de.type!=="radio"?fe&&ji(fe.elementType)&&(Ct=Zi):Ct=$a;if(Ct&&(Ct=Ct(e,fe))){Wi(Re,Ct,a,Te);break e}Qe&&Qe(e,de,fe),e==="focusout"&&fe&&de.type==="number"&&fe.memoizedProps.value!=null&&Oo(de,"number",de.value)}switch(Qe=fe?hl(fe):window,e){case"focusin":(Yi(Qe)||Qe.contentEditable==="true")&&(qn=Qe,Cl=fe,_l=null);break;case"focusout":_l=Cl=qn=null;break;case"mousedown":ci=!0;break;case"contextmenu":case"mouseup":case"dragend":ci=!1,Ji(Re,a,Te);break;case"selectionchange":if(tc)break;case"keydown":case"keyup":Ji(Re,a,Te)}var ht;if(Xr)e:{switch(e){case"compositionstart":var bt="onCompositionStart";break e;case"compositionend":bt="onCompositionEnd";break e;case"compositionupdate":bt="onCompositionUpdate";break e}bt=void 0}else bl?Pi(e,a)&&(bt="onCompositionEnd"):e==="keydown"&&a.keyCode===229&&(bt="onCompositionStart");bt&&(Ki&&a.locale!=="ko"&&(bl||bt!=="onCompositionStart"?bt==="onCompositionEnd"&&bl&&(ht=Xc()):(oo=Te,yl="value"in oo?oo.value:oo.textContent,bl=!0)),Qe=D0(fe,bt),0<Qe.length&&(bt=new ai(bt,e,null,a,Te),Re.push({event:bt,listeners:Qe}),ht?bt.data=ht:(ht=t0(a),ht!==null&&(bt.data=ht)))),(ht=li?Uu(e,a):Zr(e,a))&&(bt=D0(fe,"onBeforeInput"),0<bt.length&&(Qe=new ai("onBeforeInput","beforeinput",null,a,Te),Re.push({event:Qe,listeners:bt}),Qe.data=ht)),dv(Re,e,fe,a,Te)}Rm(Re,t)})}function Tc(e,t,a){return{instance:e,listener:t,currentTarget:a}}function D0(e,t){for(var a=t+"Capture",n=[];e!==null;){var s=e,u=s.stateNode;if(s=s.tag,s!==5&&s!==26&&s!==27||u===null||(s=gl(e,a),s!=null&&n.unshift(Tc(e,s,u)),s=gl(e,t),s!=null&&n.push(Tc(e,s,u))),e.tag===3)return n;e=e.return}return[]}function gv(e){if(e===null)return null;do e=e.return;while(e&&e.tag!==5&&e.tag!==27);return e||null}function Dm(e,t,a,n,s){for(var u=t._reactName,p=[];a!==null&&a!==n;){var w=a,q=w.alternate,fe=w.stateNode;if(w=w.tag,q!==null&&q===n)break;w!==5&&w!==26&&w!==27||fe===null||(q=fe,s?(fe=gl(a,u),fe!=null&&p.unshift(Tc(a,fe,q))):s||(fe=gl(a,u),fe!=null&&p.push(Tc(a,fe,q)))),a=a.return}p.length!==0&&e.push({event:t,listeners:p})}var yv=/\r\n?/g,xv=/\u0000|\uFFFD/g;function km(e){return(typeof e=="string"?e:""+e).replace(yv,`
`).replace(xv,"")}function Um(e,t){return t=km(t),km(e)===t}function Ot(e,t,a,n,s,u){switch(a){case"children":typeof n=="string"?t==="body"||t==="textarea"&&n===""||ml(e,n):(typeof n=="number"||typeof n=="bigint")&&t!=="body"&&ml(e,""+n);break;case"className":as(e,"class",n);break;case"tabIndex":as(e,"tabindex",n);break;case"dir":case"role":case"viewBox":case"width":case"height":as(e,a,n);break;case"style":Is(e,n,u);break;case"data":if(t!=="object"){as(e,"data",n);break}case"src":case"href":if(n===""&&(t!=="a"||a!=="href")){e.removeAttribute(a);break}if(n==null||typeof n=="function"||typeof n=="symbol"||typeof n=="boolean"){e.removeAttribute(a);break}n=vl(""+n),e.setAttribute(a,n);break;case"action":case"formAction":if(typeof n=="function"){e.setAttribute(a,"javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')");break}else typeof u=="function"&&(a==="formAction"?(t!=="input"&&Ot(e,t,"name",s.name,s,null),Ot(e,t,"formEncType",s.formEncType,s,null),Ot(e,t,"formMethod",s.formMethod,s,null),Ot(e,t,"formTarget",s.formTarget,s,null)):(Ot(e,t,"encType",s.encType,s,null),Ot(e,t,"method",s.method,s,null),Ot(e,t,"target",s.target,s,null)));if(n==null||typeof n=="symbol"||typeof n=="boolean"){e.removeAttribute(a);break}n=vl(""+n),e.setAttribute(a,n);break;case"onClick":n!=null&&(e.onclick=no);break;case"onScroll":n!=null&&yt("scroll",e);break;case"onScrollEnd":n!=null&&yt("scrollend",e);break;case"dangerouslySetInnerHTML":if(n!=null){if(typeof n!="object"||!("__html"in n))throw Error(c(61));if(a=n.__html,a!=null){if(s.children!=null)throw Error(c(60));e.innerHTML=a}}break;case"multiple":e.multiple=n&&typeof n!="function"&&typeof n!="symbol";break;case"muted":e.muted=n&&typeof n!="function"&&typeof n!="symbol";break;case"suppressContentEditableWarning":case"suppressHydrationWarning":case"defaultValue":case"defaultChecked":case"innerHTML":case"ref":break;case"autoFocus":break;case"xlinkHref":if(n==null||typeof n=="function"||typeof n=="boolean"||typeof n=="symbol"){e.removeAttribute("xlink:href");break}a=vl(""+n),e.setAttributeNS("http://www.w3.org/1999/xlink","xlink:href",a);break;case"contentEditable":case"spellCheck":case"draggable":case"value":case"autoReverse":case"externalResourcesRequired":case"focusable":case"preserveAlpha":n!=null&&typeof n!="function"&&typeof n!="symbol"?e.setAttribute(a,""+n):e.removeAttribute(a);break;case"inert":case"allowFullScreen":case"async":case"autoPlay":case"controls":case"default":case"defer":case"disabled":case"disablePictureInPicture":case"disableRemotePlayback":case"formNoValidate":case"hidden":case"loop":case"noModule":case"noValidate":case"open":case"playsInline":case"readOnly":case"required":case"reversed":case"scoped":case"seamless":case"itemScope":n&&typeof n!="function"&&typeof n!="symbol"?e.setAttribute(a,""):e.removeAttribute(a);break;case"capture":case"download":n===!0?e.setAttribute(a,""):n!==!1&&n!=null&&typeof n!="function"&&typeof n!="symbol"?e.setAttribute(a,n):e.removeAttribute(a);break;case"cols":case"rows":case"size":case"span":n!=null&&typeof n!="function"&&typeof n!="symbol"&&!isNaN(n)&&1<=n?e.setAttribute(a,n):e.removeAttribute(a);break;case"rowSpan":case"start":n==null||typeof n=="function"||typeof n=="symbol"||isNaN(n)?e.removeAttribute(a):e.setAttribute(a,n);break;case"popover":yt("beforetoggle",e),yt("toggle",e),ts(e,"popover",n);break;case"xlinkActuate":Hn(e,"http://www.w3.org/1999/xlink","xlink:actuate",n);break;case"xlinkArcrole":Hn(e,"http://www.w3.org/1999/xlink","xlink:arcrole",n);break;case"xlinkRole":Hn(e,"http://www.w3.org/1999/xlink","xlink:role",n);break;case"xlinkShow":Hn(e,"http://www.w3.org/1999/xlink","xlink:show",n);break;case"xlinkTitle":Hn(e,"http://www.w3.org/1999/xlink","xlink:title",n);break;case"xlinkType":Hn(e,"http://www.w3.org/1999/xlink","xlink:type",n);break;case"xmlBase":Hn(e,"http://www.w3.org/XML/1998/namespace","xml:base",n);break;case"xmlLang":Hn(e,"http://www.w3.org/XML/1998/namespace","xml:lang",n);break;case"xmlSpace":Hn(e,"http://www.w3.org/XML/1998/namespace","xml:space",n);break;case"is":ts(e,"is",n);break;case"innerText":case"textContent":break;default:(!(2<a.length)||a[0]!=="o"&&a[0]!=="O"||a[1]!=="n"&&a[1]!=="N")&&(a=Mu.get(a)||a,ts(e,a,n))}}function Rf(e,t,a,n,s,u){switch(a){case"style":Is(e,n,u);break;case"dangerouslySetInnerHTML":if(n!=null){if(typeof n!="object"||!("__html"in n))throw Error(c(61));if(a=n.__html,a!=null){if(s.children!=null)throw Error(c(60));e.innerHTML=a}}break;case"children":typeof n=="string"?ml(e,n):(typeof n=="number"||typeof n=="bigint")&&ml(e,""+n);break;case"onScroll":n!=null&&yt("scroll",e);break;case"onScrollEnd":n!=null&&yt("scrollend",e);break;case"onClick":n!=null&&(e.onclick=no);break;case"suppressContentEditableWarning":case"suppressHydrationWarning":case"innerHTML":case"ref":break;case"innerText":case"textContent":break;default:if(!Bi.hasOwnProperty(a))e:{if(a[0]==="o"&&a[1]==="n"&&(s=a.endsWith("Capture"),t=a.slice(2,s?a.length-7:void 0),u=e[ma]||null,u=u!=null?u[a]:null,typeof u=="function"&&e.removeEventListener(t,u,s),typeof n=="function")){typeof u!="function"&&u!==null&&(a in e?e[a]=null:e.hasAttribute(a)&&e.removeAttribute(a)),e.addEventListener(t,n,s);break e}a in e?e[a]=n:n===!0?e.setAttribute(a,""):ts(e,a,n)}}}function Ca(e,t,a){switch(t){case"div":case"span":case"svg":case"path":case"a":case"g":case"p":case"li":break;case"img":yt("error",e),yt("load",e);var n=!1,s=!1,u;for(u in a)if(a.hasOwnProperty(u)){var p=a[u];if(p!=null)switch(u){case"src":n=!0;break;case"srcSet":s=!0;break;case"children":case"dangerouslySetInnerHTML":throw Error(c(137,t));default:Ot(e,t,u,p,a,null)}}s&&Ot(e,t,"srcSet",a.srcSet,a,null),n&&Ot(e,t,"src",a.src,a,null);return;case"input":yt("invalid",e);var w=u=p=s=null,q=null,fe=null;for(n in a)if(a.hasOwnProperty(n)){var Te=a[n];if(Te!=null)switch(n){case"name":s=Te;break;case"type":p=Te;break;case"checked":q=Te;break;case"defaultChecked":fe=Te;break;case"value":u=Te;break;case"defaultValue":w=Te;break;case"children":case"dangerouslySetInnerHTML":if(Te!=null)throw Error(c(137,t));break;default:Ot(e,t,n,Te,a,null)}}ls(e,u,w,q,fe,p,s,!1);return;case"select":yt("invalid",e),n=p=u=null;for(s in a)if(a.hasOwnProperty(s)&&(w=a[s],w!=null))switch(s){case"value":u=w;break;case"defaultValue":p=w;break;case"multiple":n=w;default:Ot(e,t,s,w,a,null)}t=u,a=p,e.multiple=!!n,t!=null?Bo(e,!!n,t,!1):a!=null&&Bo(e,!!n,a,!0);return;case"textarea":yt("invalid",e),u=s=n=null;for(p in a)if(a.hasOwnProperty(p)&&(w=a[p],w!=null))switch(p){case"value":n=w;break;case"defaultValue":s=w;break;case"children":u=w;break;case"dangerouslySetInnerHTML":if(w!=null)throw Error(c(91));break;default:Ot(e,t,p,w,a,null)}dl(e,n,s,u);return;case"option":for(q in a)if(a.hasOwnProperty(q)&&(n=a[q],n!=null))switch(q){case"selected":e.selected=n&&typeof n!="function"&&typeof n!="symbol";break;default:Ot(e,t,q,n,a,null)}return;case"dialog":yt("beforetoggle",e),yt("toggle",e),yt("cancel",e),yt("close",e);break;case"iframe":case"object":yt("load",e);break;case"video":case"audio":for(n=0;n<Sc.length;n++)yt(Sc[n],e);break;case"image":yt("error",e),yt("load",e);break;case"details":yt("toggle",e);break;case"embed":case"source":case"link":yt("error",e),yt("load",e);case"area":case"base":case"br":case"col":case"hr":case"keygen":case"meta":case"param":case"track":case"wbr":case"menuitem":for(fe in a)if(a.hasOwnProperty(fe)&&(n=a[fe],n!=null))switch(fe){case"children":case"dangerouslySetInnerHTML":throw Error(c(137,t));default:Ot(e,t,fe,n,a,null)}return;default:if(ji(t)){for(Te in a)a.hasOwnProperty(Te)&&(n=a[Te],n!==void 0&&Rf(e,t,Te,n,a,void 0));return}}for(w in a)a.hasOwnProperty(w)&&(n=a[w],n!=null&&Ot(e,t,w,n,a,null))}function wv(e,t,a,n){switch(t){case"div":case"span":case"svg":case"path":case"a":case"g":case"p":case"li":break;case"input":var s=null,u=null,p=null,w=null,q=null,fe=null,Te=null;for(xe in a){var Re=a[xe];if(a.hasOwnProperty(xe)&&Re!=null)switch(xe){case"checked":break;case"value":break;case"defaultValue":q=Re;default:n.hasOwnProperty(xe)||Ot(e,t,xe,null,n,Re)}}for(var de in n){var xe=n[de];if(Re=a[de],n.hasOwnProperty(de)&&(xe!=null||Re!=null))switch(de){case"type":u=xe;break;case"name":s=xe;break;case"checked":fe=xe;break;case"defaultChecked":Te=xe;break;case"value":p=xe;break;case"defaultValue":w=xe;break;case"children":case"dangerouslySetInnerHTML":if(xe!=null)throw Error(c(137,t));break;default:xe!==Re&&Ot(e,t,de,xe,n,Re)}}Ja(e,p,w,q,fe,Te,u,s);return;case"select":xe=p=w=de=null;for(u in a)if(q=a[u],a.hasOwnProperty(u)&&q!=null)switch(u){case"value":break;case"multiple":xe=q;default:n.hasOwnProperty(u)||Ot(e,t,u,null,n,q)}for(s in n)if(u=n[s],q=a[s],n.hasOwnProperty(s)&&(u!=null||q!=null))switch(s){case"value":de=u;break;case"defaultValue":w=u;break;case"multiple":p=u;default:u!==q&&Ot(e,t,s,u,n,q)}t=w,a=p,n=xe,de!=null?Bo(e,!!a,de,!1):!!n!=!!a&&(t!=null?Bo(e,!!a,t,!0):Bo(e,!!a,a?[]:"",!1));return;case"textarea":xe=de=null;for(w in a)if(s=a[w],a.hasOwnProperty(w)&&s!=null&&!n.hasOwnProperty(w))switch(w){case"value":break;case"children":break;default:Ot(e,t,w,null,n,s)}for(p in n)if(s=n[p],u=a[p],n.hasOwnProperty(p)&&(s!=null||u!=null))switch(p){case"value":de=s;break;case"defaultValue":xe=s;break;case"children":break;case"dangerouslySetInnerHTML":if(s!=null)throw Error(c(91));break;default:s!==u&&Ot(e,t,p,s,n,u)}Fr(e,de,xe);return;case"option":for(var We in a)if(de=a[We],a.hasOwnProperty(We)&&de!=null&&!n.hasOwnProperty(We))switch(We){case"selected":e.selected=!1;break;default:Ot(e,t,We,null,n,de)}for(q in n)if(de=n[q],xe=a[q],n.hasOwnProperty(q)&&de!==xe&&(de!=null||xe!=null))switch(q){case"selected":e.selected=de&&typeof de!="function"&&typeof de!="symbol";break;default:Ot(e,t,q,de,n,xe)}return;case"img":case"link":case"area":case"base":case"br":case"col":case"embed":case"hr":case"keygen":case"meta":case"param":case"source":case"track":case"wbr":case"menuitem":for(var at in a)de=a[at],a.hasOwnProperty(at)&&de!=null&&!n.hasOwnProperty(at)&&Ot(e,t,at,null,n,de);for(fe in n)if(de=n[fe],xe=a[fe],n.hasOwnProperty(fe)&&de!==xe&&(de!=null||xe!=null))switch(fe){case"children":case"dangerouslySetInnerHTML":if(de!=null)throw Error(c(137,t));break;default:Ot(e,t,fe,de,n,xe)}return;default:if(ji(t)){for(var Bt in a)de=a[Bt],a.hasOwnProperty(Bt)&&de!==void 0&&!n.hasOwnProperty(Bt)&&Rf(e,t,Bt,void 0,n,de);for(Te in n)de=n[Te],xe=a[Te],!n.hasOwnProperty(Te)||de===xe||de===void 0&&xe===void 0||Rf(e,t,Te,de,n,xe);return}}for(var ae in a)de=a[ae],a.hasOwnProperty(ae)&&de!=null&&!n.hasOwnProperty(ae)&&Ot(e,t,ae,null,n,de);for(Re in n)de=n[Re],xe=a[Re],!n.hasOwnProperty(Re)||de===xe||de==null&&xe==null||Ot(e,t,Re,de,n,xe)}function Om(e){switch(e){case"css":case"script":case"font":case"img":case"image":case"input":case"link":return!0;default:return!1}}function bv(){if(typeof performance.getEntriesByType=="function"){for(var e=0,t=0,a=performance.getEntriesByType("resource"),n=0;n<a.length;n++){var s=a[n],u=s.transferSize,p=s.initiatorType,w=s.duration;if(u&&w&&Om(p)){for(p=0,w=s.responseEnd,n+=1;n<a.length;n++){var q=a[n],fe=q.startTime;if(fe>w)break;var Te=q.transferSize,Re=q.initiatorType;Te&&Om(Re)&&(q=q.responseEnd,p+=Te*(q<w?1:(w-fe)/(q-fe)))}if(--n,t+=8*(u+p)/(s.duration/1e3),e++,10<e)break}}if(0<e)return t/e/1e6}return navigator.connection&&(e=navigator.connection.downlink,typeof e=="number")?e:5}var Nf=null,Df=null;function k0(e){return e.nodeType===9?e:e.ownerDocument}function Bm(e){switch(e){case"http://www.w3.org/2000/svg":return 1;case"http://www.w3.org/1998/Math/MathML":return 2;default:return 0}}function Hm(e,t){if(e===0)switch(t){case"svg":return 1;case"math":return 2;default:return 0}return e===1&&t==="foreignObject"?0:e}function kf(e,t){return e==="textarea"||e==="noscript"||typeof t.children=="string"||typeof t.children=="number"||typeof t.children=="bigint"||typeof t.dangerouslySetInnerHTML=="object"&&t.dangerouslySetInnerHTML!==null&&t.dangerouslySetInnerHTML.__html!=null}var Uf=null;function Mv(){var e=window.event;return e&&e.type==="popstate"?e===Uf?!1:(Uf=e,!0):(Uf=null,!1)}var Lm=typeof setTimeout=="function"?setTimeout:void 0,Sv=typeof clearTimeout=="function"?clearTimeout:void 0,jm=typeof Promise=="function"?Promise:void 0,Tv=typeof queueMicrotask=="function"?queueMicrotask:typeof jm<"u"?function(e){return jm.resolve(null).then(e).catch(Cv)}:Lm;function Cv(e){setTimeout(function(){throw e})}function ks(e){return e==="head"}function Fm(e,t){var a=t,n=0;do{var s=a.nextSibling;if(e.removeChild(a),s&&s.nodeType===8)if(a=s.data,a==="/$"||a==="/&"){if(n===0){e.removeChild(s),zr(t);return}n--}else if(a==="$"||a==="$?"||a==="$~"||a==="$!"||a==="&")n++;else if(a==="html")Cc(e.ownerDocument.documentElement);else if(a==="head"){a=e.ownerDocument.head,Cc(a);for(var u=a.firstChild;u;){var p=u.nextSibling,w=u.nodeName;u[fl]||w==="SCRIPT"||w==="STYLE"||w==="LINK"&&u.rel.toLowerCase()==="stylesheet"||a.removeChild(u),u=p}}else a==="body"&&Cc(e.ownerDocument.body);a=s}while(a);zr(t)}function Gm(e,t){var a=e;e=0;do{var n=a.nextSibling;if(a.nodeType===1?t?(a._stashedDisplay=a.style.display,a.style.display="none"):(a.style.display=a._stashedDisplay||"",a.getAttribute("style")===""&&a.removeAttribute("style")):a.nodeType===3&&(t?(a._stashedText=a.nodeValue,a.nodeValue=""):a.nodeValue=a._stashedText||""),n&&n.nodeType===8)if(a=n.data,a==="/$"){if(e===0)break;e--}else a!=="$"&&a!=="$?"&&a!=="$~"&&a!=="$!"||e++;a=n}while(a)}function Of(e){var t=e.firstChild;for(t&&t.nodeType===10&&(t=t.nextSibling);t;){var a=t;switch(t=t.nextSibling,a.nodeName){case"HTML":case"HEAD":case"BODY":Of(a),Zs(a);continue;case"SCRIPT":case"STYLE":continue;case"LINK":if(a.rel.toLowerCase()==="stylesheet")continue}e.removeChild(a)}}function _v(e,t,a,n){for(;e.nodeType===1;){var s=a;if(e.nodeName.toLowerCase()!==t.toLowerCase()){if(!n&&(e.nodeName!=="INPUT"||e.type!=="hidden"))break}else if(n){if(!e[fl])switch(t){case"meta":if(!e.hasAttribute("itemprop"))break;return e;case"link":if(u=e.getAttribute("rel"),u==="stylesheet"&&e.hasAttribute("data-precedence"))break;if(u!==s.rel||e.getAttribute("href")!==(s.href==null||s.href===""?null:s.href)||e.getAttribute("crossorigin")!==(s.crossOrigin==null?null:s.crossOrigin)||e.getAttribute("title")!==(s.title==null?null:s.title))break;return e;case"style":if(e.hasAttribute("data-precedence"))break;return e;case"script":if(u=e.getAttribute("src"),(u!==(s.src==null?null:s.src)||e.getAttribute("type")!==(s.type==null?null:s.type)||e.getAttribute("crossorigin")!==(s.crossOrigin==null?null:s.crossOrigin))&&u&&e.hasAttribute("async")&&!e.hasAttribute("itemprop"))break;return e;default:return e}}else if(t==="input"&&e.type==="hidden"){var u=s.name==null?null:""+s.name;if(s.type==="hidden"&&e.getAttribute("name")===u)return e}else return e;if(e=Qn(e.nextSibling),e===null)break}return null}function zv(e,t,a){if(t==="")return null;for(;e.nodeType!==3;)if((e.nodeType!==1||e.nodeName!=="INPUT"||e.type!=="hidden")&&!a||(e=Qn(e.nextSibling),e===null))return null;return e}function qm(e,t){for(;e.nodeType!==8;)if((e.nodeType!==1||e.nodeName!=="INPUT"||e.type!=="hidden")&&!t||(e=Qn(e.nextSibling),e===null))return null;return e}function Bf(e){return e.data==="$?"||e.data==="$~"}function Hf(e){return e.data==="$!"||e.data==="$?"&&e.ownerDocument.readyState!=="loading"}function Av(e,t){var a=e.ownerDocument;if(e.data==="$~")e._reactRetry=t;else if(e.data!=="$?"||a.readyState!=="loading")t();else{var n=function(){t(),a.removeEventListener("DOMContentLoaded",n)};a.addEventListener("DOMContentLoaded",n),e._reactRetry=n}}function Qn(e){for(;e!=null;e=e.nextSibling){var t=e.nodeType;if(t===1||t===3)break;if(t===8){if(t=e.data,t==="$"||t==="$!"||t==="$?"||t==="$~"||t==="&"||t==="F!"||t==="F")break;if(t==="/$"||t==="/&")return null}}return e}var Lf=null;function Vm(e){e=e.nextSibling;for(var t=0;e;){if(e.nodeType===8){var a=e.data;if(a==="/$"||a==="/&"){if(t===0)return Qn(e.nextSibling);t--}else a!=="$"&&a!=="$!"&&a!=="$?"&&a!=="$~"&&a!=="&"||t++}e=e.nextSibling}return null}function Km(e){e=e.previousSibling;for(var t=0;e;){if(e.nodeType===8){var a=e.data;if(a==="$"||a==="$!"||a==="$?"||a==="$~"||a==="&"){if(t===0)return e;t--}else a!=="/$"&&a!=="/&"||t++}e=e.previousSibling}return null}function Pm(e,t,a){switch(t=k0(a),e){case"html":if(e=t.documentElement,!e)throw Error(c(452));return e;case"head":if(e=t.head,!e)throw Error(c(453));return e;case"body":if(e=t.body,!e)throw Error(c(454));return e;default:throw Error(c(451))}}function Cc(e){for(var t=e.attributes;t.length;)e.removeAttributeNode(t[0]);Zs(e)}var In=new Map,Ym=new Set;function U0(e){return typeof e.getRootNode=="function"?e.getRootNode():e.nodeType===9?e:e.ownerDocument}var Gl=P.d;P.d={f:Ev,r:Rv,D:Nv,C:Dv,L:kv,m:Uv,X:Bv,S:Ov,M:Hv};function Ev(){var e=Gl.f(),t=C0();return e||t}function Rv(e){var t=No(e);t!==null&&t.tag===5&&t.type==="form"?cd(t):Gl.r(e)}var Tr=typeof document>"u"?null:document;function Wm(e,t,a){var n=Tr;if(n&&typeof t=="string"&&t){var s=nt(t);s='link[rel="'+e+'"][href="'+s+'"]',typeof a=="string"&&(s+='[crossorigin="'+a+'"]'),Ym.has(s)||(Ym.add(s),e={rel:e,crossOrigin:a,href:t},n.querySelector(s)===null&&(t=n.createElement("link"),Ca(t,"link",e),ea(t),n.head.appendChild(t)))}}function Nv(e){Gl.D(e),Wm("dns-prefetch",e,null)}function Dv(e,t){Gl.C(e,t),Wm("preconnect",e,t)}function kv(e,t,a){Gl.L(e,t,a);var n=Tr;if(n&&e&&t){var s='link[rel="preload"][as="'+nt(t)+'"]';t==="image"&&a&&a.imageSrcSet?(s+='[imagesrcset="'+nt(a.imageSrcSet)+'"]',typeof a.imageSizes=="string"&&(s+='[imagesizes="'+nt(a.imageSizes)+'"]')):s+='[href="'+nt(e)+'"]';var u=s;switch(t){case"style":u=Cr(e);break;case"script":u=_r(e)}In.has(u)||(e=M({rel:"preload",href:t==="image"&&a&&a.imageSrcSet?void 0:e,as:t},a),In.set(u,e),n.querySelector(s)!==null||t==="style"&&n.querySelector(_c(u))||t==="script"&&n.querySelector(zc(u))||(t=n.createElement("link"),Ca(t,"link",e),ea(t),n.head.appendChild(t)))}}function Uv(e,t){Gl.m(e,t);var a=Tr;if(a&&e){var n=t&&typeof t.as=="string"?t.as:"script",s='link[rel="modulepreload"][as="'+nt(n)+'"][href="'+nt(e)+'"]',u=s;switch(n){case"audioworklet":case"paintworklet":case"serviceworker":case"sharedworker":case"worker":case"script":u=_r(e)}if(!In.has(u)&&(e=M({rel:"modulepreload",href:e},t),In.set(u,e),a.querySelector(s)===null)){switch(n){case"audioworklet":case"paintworklet":case"serviceworker":case"sharedworker":case"worker":case"script":if(a.querySelector(zc(u)))return}n=a.createElement("link"),Ca(n,"link",e),ea(n),a.head.appendChild(n)}}}function Ov(e,t,a){Gl.S(e,t,a);var n=Tr;if(n&&e){var s=Do(n).hoistableStyles,u=Cr(e);t=t||"default";var p=s.get(u);if(!p){var w={loading:0,preload:null};if(p=n.querySelector(_c(u)))w.loading=5;else{e=M({rel:"stylesheet",href:e,"data-precedence":t},a),(a=In.get(u))&&jf(e,a);var q=p=n.createElement("link");ea(q),Ca(q,"link",e),q._p=new Promise(function(fe,Te){q.onload=fe,q.onerror=Te}),q.addEventListener("load",function(){w.loading|=1}),q.addEventListener("error",function(){w.loading|=2}),w.loading|=4,O0(p,t,n)}p={type:"stylesheet",instance:p,count:1,state:w},s.set(u,p)}}}function Bv(e,t){Gl.X(e,t);var a=Tr;if(a&&e){var n=Do(a).hoistableScripts,s=_r(e),u=n.get(s);u||(u=a.querySelector(zc(s)),u||(e=M({src:e,async:!0},t),(t=In.get(s))&&Ff(e,t),u=a.createElement("script"),ea(u),Ca(u,"link",e),a.head.appendChild(u)),u={type:"script",instance:u,count:1,state:null},n.set(s,u))}}function Hv(e,t){Gl.M(e,t);var a=Tr;if(a&&e){var n=Do(a).hoistableScripts,s=_r(e),u=n.get(s);u||(u=a.querySelector(zc(s)),u||(e=M({src:e,async:!0,type:"module"},t),(t=In.get(s))&&Ff(e,t),u=a.createElement("script"),ea(u),Ca(u,"link",e),a.head.appendChild(u)),u={type:"script",instance:u,count:1,state:null},n.set(s,u))}}function Xm(e,t,a,n){var s=(s=ue.current)?U0(s):null;if(!s)throw Error(c(446));switch(e){case"meta":case"title":return null;case"style":return typeof a.precedence=="string"&&typeof a.href=="string"?(t=Cr(a.href),a=Do(s).hoistableStyles,n=a.get(t),n||(n={type:"style",instance:null,count:0,state:null},a.set(t,n)),n):{type:"void",instance:null,count:0,state:null};case"link":if(a.rel==="stylesheet"&&typeof a.href=="string"&&typeof a.precedence=="string"){e=Cr(a.href);var u=Do(s).hoistableStyles,p=u.get(e);if(p||(s=s.ownerDocument||s,p={type:"stylesheet",instance:null,count:0,state:{loading:0,preload:null}},u.set(e,p),(u=s.querySelector(_c(e)))&&!u._p&&(p.instance=u,p.state.loading=5),In.has(e)||(a={rel:"preload",as:"style",href:a.href,crossOrigin:a.crossOrigin,integrity:a.integrity,media:a.media,hrefLang:a.hrefLang,referrerPolicy:a.referrerPolicy},In.set(e,a),u||Lv(s,e,a,p.state))),t&&n===null)throw Error(c(528,""));return p}if(t&&n!==null)throw Error(c(529,""));return null;case"script":return t=a.async,a=a.src,typeof a=="string"&&t&&typeof t!="function"&&typeof t!="symbol"?(t=_r(a),a=Do(s).hoistableScripts,n=a.get(t),n||(n={type:"script",instance:null,count:0,state:null},a.set(t,n)),n):{type:"void",instance:null,count:0,state:null};default:throw Error(c(444,e))}}function Cr(e){return'href="'+nt(e)+'"'}function _c(e){return'link[rel="stylesheet"]['+e+"]"}function Zm(e){return M({},e,{"data-precedence":e.precedence,precedence:null})}function Lv(e,t,a,n){e.querySelector('link[rel="preload"][as="style"]['+t+"]")?n.loading=1:(t=e.createElement("link"),n.preload=t,t.addEventListener("load",function(){return n.loading|=1}),t.addEventListener("error",function(){return n.loading|=2}),Ca(t,"link",a),ea(t),e.head.appendChild(t))}function _r(e){return'[src="'+nt(e)+'"]'}function zc(e){return"script[async]"+e}function Qm(e,t,a){if(t.count++,t.instance===null)switch(t.type){case"style":var n=e.querySelector('style[data-href~="'+nt(a.href)+'"]');if(n)return t.instance=n,ea(n),n;var s=M({},a,{"data-href":a.href,"data-precedence":a.precedence,href:null,precedence:null});return n=(e.ownerDocument||e).createElement("style"),ea(n),Ca(n,"style",s),O0(n,a.precedence,e),t.instance=n;case"stylesheet":s=Cr(a.href);var u=e.querySelector(_c(s));if(u)return t.state.loading|=4,t.instance=u,ea(u),u;n=Zm(a),(s=In.get(s))&&jf(n,s),u=(e.ownerDocument||e).createElement("link"),ea(u);var p=u;return p._p=new Promise(function(w,q){p.onload=w,p.onerror=q}),Ca(u,"link",n),t.state.loading|=4,O0(u,a.precedence,e),t.instance=u;case"script":return u=_r(a.src),(s=e.querySelector(zc(u)))?(t.instance=s,ea(s),s):(n=a,(s=In.get(u))&&(n=M({},a),Ff(n,s)),e=e.ownerDocument||e,s=e.createElement("script"),ea(s),Ca(s,"link",n),e.head.appendChild(s),t.instance=s);case"void":return null;default:throw Error(c(443,t.type))}else t.type==="stylesheet"&&(t.state.loading&4)===0&&(n=t.instance,t.state.loading|=4,O0(n,a.precedence,e));return t.instance}function O0(e,t,a){for(var n=a.querySelectorAll('link[rel="stylesheet"][data-precedence],style[data-precedence]'),s=n.length?n[n.length-1]:null,u=s,p=0;p<n.length;p++){var w=n[p];if(w.dataset.precedence===t)u=w;else if(u!==s)break}u?u.parentNode.insertBefore(e,u.nextSibling):(t=a.nodeType===9?a.head:a,t.insertBefore(e,t.firstChild))}function jf(e,t){e.crossOrigin==null&&(e.crossOrigin=t.crossOrigin),e.referrerPolicy==null&&(e.referrerPolicy=t.referrerPolicy),e.title==null&&(e.title=t.title)}function Ff(e,t){e.crossOrigin==null&&(e.crossOrigin=t.crossOrigin),e.referrerPolicy==null&&(e.referrerPolicy=t.referrerPolicy),e.integrity==null&&(e.integrity=t.integrity)}var B0=null;function Im(e,t,a){if(B0===null){var n=new Map,s=B0=new Map;s.set(a,n)}else s=B0,n=s.get(a),n||(n=new Map,s.set(a,n));if(n.has(e))return n;for(n.set(e,null),a=a.getElementsByTagName(e),s=0;s<a.length;s++){var u=a[s];if(!(u[fl]||u[oa]||e==="link"&&u.getAttribute("rel")==="stylesheet")&&u.namespaceURI!=="http://www.w3.org/2000/svg"){var p=u.getAttribute(t)||"";p=e+p;var w=n.get(p);w?w.push(u):n.set(p,[u])}}return n}function Jm(e,t,a){e=e.ownerDocument||e,e.head.insertBefore(a,t==="title"?e.querySelector("head > title"):null)}function jv(e,t,a){if(a===1||t.itemProp!=null)return!1;switch(e){case"meta":case"title":return!0;case"style":if(typeof t.precedence!="string"||typeof t.href!="string"||t.href==="")break;return!0;case"link":if(typeof t.rel!="string"||typeof t.href!="string"||t.href===""||t.onLoad||t.onError)break;switch(t.rel){case"stylesheet":return e=t.disabled,typeof t.precedence=="string"&&e==null;default:return!0}case"script":if(t.async&&typeof t.async!="function"&&typeof t.async!="symbol"&&!t.onLoad&&!t.onError&&t.src&&typeof t.src=="string")return!0}return!1}function $m(e){return!(e.type==="stylesheet"&&(e.state.loading&3)===0)}function Fv(e,t,a,n){if(a.type==="stylesheet"&&(typeof n.media!="string"||matchMedia(n.media).matches!==!1)&&(a.state.loading&4)===0){if(a.instance===null){var s=Cr(n.href),u=t.querySelector(_c(s));if(u){t=u._p,t!==null&&typeof t=="object"&&typeof t.then=="function"&&(e.count++,e=H0.bind(e),t.then(e,e)),a.state.loading|=4,a.instance=u,ea(u);return}u=t.ownerDocument||t,n=Zm(n),(s=In.get(s))&&jf(n,s),u=u.createElement("link"),ea(u);var p=u;p._p=new Promise(function(w,q){p.onload=w,p.onerror=q}),Ca(u,"link",n),a.instance=u}e.stylesheets===null&&(e.stylesheets=new Map),e.stylesheets.set(a,t),(t=a.state.preload)&&(a.state.loading&3)===0&&(e.count++,a=H0.bind(e),t.addEventListener("load",a),t.addEventListener("error",a))}}var Gf=0;function Gv(e,t){return e.stylesheets&&e.count===0&&j0(e,e.stylesheets),0<e.count||0<e.imgCount?function(a){var n=setTimeout(function(){if(e.stylesheets&&j0(e,e.stylesheets),e.unsuspend){var u=e.unsuspend;e.unsuspend=null,u()}},6e4+t);0<e.imgBytes&&Gf===0&&(Gf=62500*bv());var s=setTimeout(function(){if(e.waitingForImages=!1,e.count===0&&(e.stylesheets&&j0(e,e.stylesheets),e.unsuspend)){var u=e.unsuspend;e.unsuspend=null,u()}},(e.imgBytes>Gf?50:800)+t);return e.unsuspend=a,function(){e.unsuspend=null,clearTimeout(n),clearTimeout(s)}}:null}function H0(){if(this.count--,this.count===0&&(this.imgCount===0||!this.waitingForImages)){if(this.stylesheets)j0(this,this.stylesheets);else if(this.unsuspend){var e=this.unsuspend;this.unsuspend=null,e()}}}var L0=null;function j0(e,t){e.stylesheets=null,e.unsuspend!==null&&(e.count++,L0=new Map,t.forEach(qv,e),L0=null,H0.call(e))}function qv(e,t){if(!(t.state.loading&4)){var a=L0.get(e);if(a)var n=a.get(null);else{a=new Map,L0.set(e,a);for(var s=e.querySelectorAll("link[data-precedence],style[data-precedence]"),u=0;u<s.length;u++){var p=s[u];(p.nodeName==="LINK"||p.getAttribute("media")!=="not all")&&(a.set(p.dataset.precedence,p),n=p)}n&&a.set(null,n)}s=t.instance,p=s.getAttribute("data-precedence"),u=a.get(p)||n,u===n&&a.set(null,s),a.set(p,s),this.count++,n=H0.bind(this),s.addEventListener("load",n),s.addEventListener("error",n),u?u.parentNode.insertBefore(s,u.nextSibling):(e=e.nodeType===9?e.head:e,e.insertBefore(s,e.firstChild)),t.state.loading|=4}}var Ac={$$typeof:U,Provider:null,Consumer:null,_currentValue:k,_currentValue2:k,_threadCount:0};function Vv(e,t,a,n,s,u,p,w,q){this.tag=1,this.containerInfo=e,this.pingCache=this.current=this.pendingChildren=null,this.timeoutHandle=-1,this.callbackNode=this.next=this.pendingContext=this.context=this.cancelPendingCommit=null,this.callbackPriority=0,this.expirationTimes=Ur(-1),this.entangledLanes=this.shellSuspendCounter=this.errorRecoveryDisabledLanes=this.expiredLanes=this.warmLanes=this.pingedLanes=this.suspendedLanes=this.pendingLanes=0,this.entanglements=Ur(0),this.hiddenUpdates=Ur(null),this.identifierPrefix=n,this.onUncaughtError=s,this.onCaughtError=u,this.onRecoverableError=p,this.pooledCache=null,this.pooledCacheLanes=0,this.formState=q,this.incompleteTransitions=new Map}function e1(e,t,a,n,s,u,p,w,q,fe,Te,Re){return e=new Vv(e,t,a,p,q,fe,Te,Re,w),t=1,u===!0&&(t|=24),u=Ba(3,null,null,t),e.current=u,u.stateNode=e,t=pi(),t.refCount++,e.pooledCache=t,t.refCount++,u.memoizedState={element:n,isDehydrated:a,cache:t},E(u),e}function t1(e){return e?(e=qo,e):qo}function a1(e,t,a,n,s,u){s=t1(s),n.context===null?n.context=s:n.pendingContext=s,n=Y(t),n.payload={element:a},u=u===void 0?null:u,u!==null&&(n.callback=u),a=ne(e,n,t),a!==null&&(dn(a,e,t),te(a,e,t))}function n1(e,t){if(e=e.memoizedState,e!==null&&e.dehydrated!==null){var a=e.retryLane;e.retryLane=a!==0&&a<t?a:t}}function qf(e,t){n1(e,t),(e=e.alternate)&&n1(e,t)}function o1(e){if(e.tag===13||e.tag===31){var t=Vn(e,67108864);t!==null&&dn(t,e,67108864),qf(e,67108864)}}function l1(e){if(e.tag===13||e.tag===31){var t=Rn();t=Ws(t);var a=Vn(e,t);a!==null&&dn(a,e,t),qf(e,t)}}var F0=!0;function Kv(e,t,a,n){var s=N.T;N.T=null;var u=P.p;try{P.p=2,Vf(e,t,a,n)}finally{P.p=u,N.T=s}}function Pv(e,t,a,n){var s=N.T;N.T=null;var u=P.p;try{P.p=8,Vf(e,t,a,n)}finally{P.p=u,N.T=s}}function Vf(e,t,a,n){if(F0){var s=Kf(n);if(s===null)Ef(e,t,n,G0,a),i1(e,n);else if(Wv(s,e,t,a,n))n.stopPropagation();else if(i1(e,n),t&4&&-1<Yv.indexOf(e)){for(;s!==null;){var u=No(s);if(u!==null)switch(u.tag){case 3:if(u=u.stateNode,u.current.memoizedState.isDehydrated){var p=Eo(u.pendingLanes);if(p!==0){var w=u;for(w.pendingLanes|=2,w.entangledLanes|=2;p;){var q=1<<31-ha(p);w.entanglements[1]|=q,p&=~q}Jo(u),(At&6)===0&&(S0=Dt()+500,Mc(0))}}break;case 31:case 13:w=Vn(u,2),w!==null&&dn(w,u,2),C0(),qf(u,2)}if(u=Kf(n),u===null&&Ef(e,t,n,G0,a),u===s)break;s=u}s!==null&&n.stopPropagation()}else Ef(e,t,n,null,a)}}function Kf(e){return e=it(e),Pf(e)}var G0=null;function Pf(e){if(G0=null,e=Ro(e),e!==null){var t=f(e);if(t===null)e=null;else{var a=t.tag;if(a===13){if(e=h(t),e!==null)return e;e=null}else if(a===31){if(e=g(t),e!==null)return e;e=null}else if(a===3){if(t.stateNode.current.memoizedState.isDehydrated)return t.tag===3?t.stateNode.containerInfo:null;e=null}else t!==e&&(e=null)}}return G0=e,null}function s1(e){switch(e){case"beforetoggle":case"cancel":case"click":case"close":case"contextmenu":case"copy":case"cut":case"auxclick":case"dblclick":case"dragend":case"dragstart":case"drop":case"focusin":case"focusout":case"input":case"invalid":case"keydown":case"keypress":case"keyup":case"mousedown":case"mouseup":case"paste":case"pause":case"play":case"pointercancel":case"pointerdown":case"pointerup":case"ratechange":case"reset":case"resize":case"seeked":case"submit":case"toggle":case"touchcancel":case"touchend":case"touchstart":case"volumechange":case"change":case"selectionchange":case"textInput":case"compositionstart":case"compositionend":case"compositionupdate":case"beforeblur":case"afterblur":case"beforeinput":case"blur":case"fullscreenchange":case"focus":case"hashchange":case"popstate":case"select":case"selectstart":return 2;case"drag":case"dragenter":case"dragexit":case"dragleave":case"dragover":case"mousemove":case"mouseout":case"mouseover":case"pointermove":case"pointerout":case"pointerover":case"scroll":case"touchmove":case"wheel":case"mouseenter":case"mouseleave":case"pointerenter":case"pointerleave":return 8;case"message":switch(_o()){case sa:return 2;case Ra:return 8;case Ya:case pn:return 32;case Na:return 268435456;default:return 32}default:return 32}}var Yf=!1,Us=null,Os=null,Bs=null,Ec=new Map,Rc=new Map,Hs=[],Yv="mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset".split(" ");function i1(e,t){switch(e){case"focusin":case"focusout":Us=null;break;case"dragenter":case"dragleave":Os=null;break;case"mouseover":case"mouseout":Bs=null;break;case"pointerover":case"pointerout":Ec.delete(t.pointerId);break;case"gotpointercapture":case"lostpointercapture":Rc.delete(t.pointerId)}}function Nc(e,t,a,n,s,u){return e===null||e.nativeEvent!==u?(e={blockedOn:t,domEventName:a,eventSystemFlags:n,nativeEvent:u,targetContainers:[s]},t!==null&&(t=No(t),t!==null&&o1(t)),e):(e.eventSystemFlags|=n,t=e.targetContainers,s!==null&&t.indexOf(s)===-1&&t.push(s),e)}function Wv(e,t,a,n,s){switch(t){case"focusin":return Us=Nc(Us,e,t,a,n,s),!0;case"dragenter":return Os=Nc(Os,e,t,a,n,s),!0;case"mouseover":return Bs=Nc(Bs,e,t,a,n,s),!0;case"pointerover":var u=s.pointerId;return Ec.set(u,Nc(Ec.get(u)||null,e,t,a,n,s)),!0;case"gotpointercapture":return u=s.pointerId,Rc.set(u,Nc(Rc.get(u)||null,e,t,a,n,s)),!0}return!1}function r1(e){var t=Ro(e.target);if(t!==null){var a=f(t);if(a!==null){if(t=a.tag,t===13){if(t=h(a),t!==null){e.blockedOn=t,Hr(e.priority,function(){l1(a)});return}}else if(t===31){if(t=g(a),t!==null){e.blockedOn=t,Hr(e.priority,function(){l1(a)});return}}else if(t===3&&a.stateNode.current.memoizedState.isDehydrated){e.blockedOn=a.tag===3?a.stateNode.containerInfo:null;return}}}e.blockedOn=null}function q0(e){if(e.blockedOn!==null)return!1;for(var t=e.targetContainers;0<t.length;){var a=Kf(e.nativeEvent);if(a===null){a=e.nativeEvent;var n=new a.constructor(a.type,a);st=n,a.target.dispatchEvent(n),st=null}else return t=No(a),t!==null&&o1(t),e.blockedOn=a,!1;t.shift()}return!0}function c1(e,t,a){q0(e)&&a.delete(t)}function Xv(){Yf=!1,Us!==null&&q0(Us)&&(Us=null),Os!==null&&q0(Os)&&(Os=null),Bs!==null&&q0(Bs)&&(Bs=null),Ec.forEach(c1),Rc.forEach(c1)}function V0(e,t){e.blockedOn===t&&(e.blockedOn=null,Yf||(Yf=!0,o.unstable_scheduleCallback(o.unstable_NormalPriority,Xv)))}var K0=null;function u1(e){K0!==e&&(K0=e,o.unstable_scheduleCallback(o.unstable_NormalPriority,function(){K0===e&&(K0=null);for(var t=0;t<e.length;t+=3){var a=e[t],n=e[t+1],s=e[t+2];if(typeof n!="function"){if(Pf(n||a)===null)continue;break}var u=No(a);u!==null&&(e.splice(t,3),t-=3,Vu(u,{pending:!0,data:s,method:a.method,action:n},n,s))}}))}function zr(e){function t(q){return V0(q,e)}Us!==null&&V0(Us,e),Os!==null&&V0(Os,e),Bs!==null&&V0(Bs,e),Ec.forEach(t),Rc.forEach(t);for(var a=0;a<Hs.length;a++){var n=Hs[a];n.blockedOn===e&&(n.blockedOn=null)}for(;0<Hs.length&&(a=Hs[0],a.blockedOn===null);)r1(a),a.blockedOn===null&&Hs.shift();if(a=(e.ownerDocument||e).$$reactFormReplay,a!=null)for(n=0;n<a.length;n+=3){var s=a[n],u=a[n+1],p=s[ma]||null;if(typeof u=="function")p||u1(a);else if(p){var w=null;if(u&&u.hasAttribute("formAction")){if(s=u,p=u[ma]||null)w=p.formAction;else if(Pf(s)!==null)continue}else w=p.action;typeof w=="function"?a[n+1]=w:(a.splice(n,3),n-=3),u1(a)}}}function f1(){function e(u){u.canIntercept&&u.info==="react-transition"&&u.intercept({handler:function(){return new Promise(function(p){return s=p})},focusReset:"manual",scroll:"manual"})}function t(){s!==null&&(s(),s=null),n||setTimeout(a,20)}function a(){if(!n&&!navigation.transition){var u=navigation.currentEntry;u&&u.url!=null&&navigation.navigate(u.url,{state:u.getState(),info:"react-transition",history:"replace"})}}if(typeof navigation=="object"){var n=!1,s=null;return navigation.addEventListener("navigate",e),navigation.addEventListener("navigatesuccess",t),navigation.addEventListener("navigateerror",t),setTimeout(a,100),function(){n=!0,navigation.removeEventListener("navigate",e),navigation.removeEventListener("navigatesuccess",t),navigation.removeEventListener("navigateerror",t),s!==null&&(s(),s=null)}}}function Wf(e){this._internalRoot=e}P0.prototype.render=Wf.prototype.render=function(e){var t=this._internalRoot;if(t===null)throw Error(c(409));var a=t.current,n=Rn();a1(a,n,e,t,null,null)},P0.prototype.unmount=Wf.prototype.unmount=function(){var e=this._internalRoot;if(e!==null){this._internalRoot=null;var t=e.containerInfo;a1(e.current,2,null,e,null,null),C0(),t[to]=null}};function P0(e){this._internalRoot=e}P0.prototype.unstable_scheduleHydration=function(e){if(e){var t=Br();e={blockedOn:null,target:e,priority:t};for(var a=0;a<Hs.length&&t!==0&&t<Hs[a].priority;a++);Hs.splice(a,0,e),a===0&&r1(e)}};var h1=l.version;if(h1!=="19.2.8")throw Error(c(527,h1,"19.2.8"));P.findDOMNode=function(e){var t=e._reactInternals;if(t===void 0)throw typeof e.render=="function"?Error(c(188)):(e=Object.keys(e).join(","),Error(c(268,e)));return e=d(t),e=e!==null?x(e):null,e=e===null?null:e.stateNode,e};var Zv={bundleType:0,version:"19.2.8",rendererPackageName:"react-dom",currentDispatcherRef:N,reconcilerVersion:"19.2.8"};if(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__<"u"){var Y0=__REACT_DEVTOOLS_GLOBAL_HOOK__;if(!Y0.isDisabled&&Y0.supportsFiber)try{gn=Y0.inject(Zv),na=Y0}catch{}}return Uc.createRoot=function(e,t){if(!i(e))throw Error(c(299));var a=!1,n="",s=xd,u=wd,p=bd;return t!=null&&(t.unstable_strictMode===!0&&(a=!0),t.identifierPrefix!==void 0&&(n=t.identifierPrefix),t.onUncaughtError!==void 0&&(s=t.onUncaughtError),t.onCaughtError!==void 0&&(u=t.onCaughtError),t.onRecoverableError!==void 0&&(p=t.onRecoverableError)),t=e1(e,1,!1,null,null,a,n,null,s,u,p,f1),e[to]=t.current,Af(e),new Wf(t)},Uc.hydrateRoot=function(e,t,a){if(!i(e))throw Error(c(299));var n=!1,s="",u=xd,p=wd,w=bd,q=null;return a!=null&&(a.unstable_strictMode===!0&&(n=!0),a.identifierPrefix!==void 0&&(s=a.identifierPrefix),a.onUncaughtError!==void 0&&(u=a.onUncaughtError),a.onCaughtError!==void 0&&(p=a.onCaughtError),a.onRecoverableError!==void 0&&(w=a.onRecoverableError),a.formState!==void 0&&(q=a.formState)),t=e1(e,1,!0,t,a??null,n,s,q,u,p,w,f1),t.context=t1(null),a=t.current,n=Rn(),n=Ws(n),s=Y(n),s.callback=null,ne(a,s,n),a=n,t.current.lanes=a,Ys(t,a),Jo(t),e[to]=t.current,Af(e),new P0(t)},Uc.version="19.2.8",Uc}var R1;function Pg(){if(R1)return Qf.exports;R1=1;function o(){if(!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__>"u"||typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE!="function"))try{__REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(o)}catch(l){console.error(l)}}return o(),Qf.exports=Kg(),Qf.exports}var Yg=Pg();const Wg="modulepreload",Xg=function(o,l){return new URL(o,l).href},N1={},eo=function(l,r,c){let i=Promise.resolve();if(r&&r.length>0){let h=function(x){return Promise.all(x.map(M=>Promise.resolve(M).then(T=>({status:"fulfilled",value:T}),T=>({status:"rejected",reason:T}))))};const g=document.getElementsByTagName("link"),m=document.querySelector("meta[property=csp-nonce]"),d=m?.nonce||m?.getAttribute("nonce");i=h(r.map(x=>{if(x=Xg(x,c),x in N1)return;N1[x]=!0;const M=x.endsWith(".css"),T=M?'[rel="stylesheet"]':"";if(!!c)for(let B=g.length-1;B>=0;B--){const C=g[B];if(C.href===x&&(!M||C.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${x}"]${T}`))return;const R=document.createElement("link");if(R.rel=M?"stylesheet":Wg,M||(R.as="script"),R.crossOrigin="",R.href=x,d&&R.setAttribute("nonce",d),document.head.appendChild(R),M)return new Promise((B,C)=>{R.addEventListener("load",B),R.addEventListener("error",()=>C(new Error(`Unable to preload CSS for ${x}`)))})}))}function f(h){const g=new Event("vite:preloadError",{cancelable:!0});if(g.payload=h,window.dispatchEvent(g),!g.defaultPrevented)throw h}return i.then(h=>{for(const g of h||[])g.status==="rejected"&&f(g.reason);return l().catch(f)})},$l=()=>typeof window<"u"&&"__TAURI_INTERNALS__"in window;async function Ri(o,l){if($l()){const{invoke:c}=await eo(async()=>{const{invoke:i}=await Promise.resolve().then(()=>R5);return{invoke:i}},void 0,import.meta.url);return c(o,l)}const{webInvoke:r}=await eo(async()=>{const{webInvoke:c}=await import("./webBackend-sNFyw_1v.js");return{webInvoke:c}},[],import.meta.url);return r(o,l)}const Zg=Object.freeze(Object.defineProperty({__proto__:null,invoke:Ri,isDesktop:$l},Symbol.toStringTag,{value:"Module"}));async function dp(o,l){if($l()){const{listen:c}=await eo(async()=>{const{listen:i}=await import("./event-CJlvrBzi.js");return{listen:i}},__vite__mapDeps([0,1]),import.meta.url);return c(o,l)}const{onProgress:r}=await eo(async()=>{const{onProgress:c}=await import("./webBackend-sNFyw_1v.js");return{onProgress:c}},[],import.meta.url);return r(c=>l({payload:c}))}async function yh(o){if($l()){const{save:l}=await eo(async()=>{const{save:r}=await import("./index-Bl-Um1AN.js");return{save:r}},__vite__mapDeps([2,1]),import.meta.url);return l(o)}return o.defaultPath??"download"}const mp=new Map;async function pp(o){if($l()){const{open:l}=await eo(async()=>{const{open:c}=await import("./index-Bl-Um1AN.js");return{open:c}},__vite__mapDeps([2,1]),import.meta.url),r=await l(o);return Array.isArray(r)?r[0]??null:r}return new Promise(l=>{const r=document.createElement("input");r.type="file";const c=o.filters?.flatMap(i=>i.extensions.map(f=>`.${f}`))??[];c.length&&(r.accept=c.join(",")),r.onchange=()=>{const i=r.files?.[0];if(!i)return l(null);const f=`browser-file:${i.name}`;mp.set(f,i),l(f)},r.oncancel=()=>l(null),r.click()})}const aa=(()=>{try{const o=new URLSearchParams(window.location.search).get("mobile");if(o==="0")return!1;if(o==="1")return!0;const l=navigator.userAgent;return/iPhone|iPad|iPod|Android/i.test(l)||/Macintosh/.test(l)&&navigator.maxTouchPoints>1}catch{return!1}})(),ru="该设备内存有限，已使用轻量模式",Fc="terrainLab.mounting",eh="terrainLab.safeMode",Rh=()=>{try{return window.localStorage}catch{return null}},vp=(()=>{try{const o=Rh(),l=new URLSearchParams(window.location.search).get("safe");return l==="0"?(o?.removeItem(eh),o?.removeItem(Fc),!1):l==="1"?!0:!aa||!o?!1:(o.getItem(Fc)&&(o.removeItem(Fc),o.setItem(eh,"1")),o.getItem(eh)==="1")}catch{return!1}})(),Fs=vp;function gp(){aa&&Rh()?.setItem(Fc,String(Date.now()))}function xh(){Rh()?.removeItem(Fc)}const Qg=Object.freeze(Object.defineProperty({__proto__:null,LIGHT_MODE_NOTICE:ru,crashedLastTime:vp,isConstrainedDevice:aa,isSafeMode:Fs,listen:dp,markMountFinished:xh,markMountStarted:gp,open:pp,pickedFiles:mp,save:yh},Symbol.toStringTag,{value:"Module"}));let th=[];async function Ig(o){const l=await Ri("generate_terrain",{config:o});return aa&&await Jg(l),l}async function Jg(o){for(const r of th)URL.revokeObjectURL(r);th=[];const l=o.analysisPreviews;for(const r of Object.keys(l)){const c=l[r];if(!(typeof c!="string"||!c.startsWith("data:")))try{const i=await(await fetch(c)).blob(),f=URL.createObjectURL(i);th.push(f),l[r]=f}catch{}}}const ah=new Map;async function $g(o,l,r){const{decodeCityBinary:c}=await eo(async()=>{const{decodeCityBinary:m}=await Promise.resolve().then(()=>s2);return{decodeCityBinary:m}},void 0,import.meta.url),i=async m=>{try{const d=await Ri("city_tile_bin",{index:o,column:l,row:r,withTextures:m});return c(d instanceof ArrayBuffer?d:new Uint8Array(d).buffer)}catch(d){const x=await fetch(`/city-tiles/${o}-${l}-${r}.json`);if(!x.ok)throw d;const[M]=await x.json();return M}},f=ah.get(o);if(f){const m=await f,d=await i(!1),x=new Set(d.textures.map(M=>M.name));return d.textures=[...m.filter(M=>!x.has(M.name)),...d.textures],d}const h=(async()=>await i(!0))(),g=h.then(m=>m.textures.filter(d=>d.name!=="cnfacade/table"&&typeof d.data!="string").map(d=>({...d,data:d.data.slice()})));return ah.set(o,g),g.catch(()=>ah.delete(o)),h}async function e2(o){return Ri("city_scene",{index:o})}async function t2(o,l,r){if(!$l()){const{exportPng:c}=await eo(async()=>{const{exportPng:i}=await import("./webBackend-sNFyw_1v.js");return{exportPng:i}},[],import.meta.url);return c(o,l,r)}return Ri("export_terrain",{config:o,outputPath:l,outputSize:r})}async function a2(o,l){if(!$l()){const{projectToFile:r}=await eo(async()=>{const{projectToFile:c}=await import("./webBackend-sNFyw_1v.js");return{projectToFile:c}},[],import.meta.url);return r(o,l)}return Ri("save_project",{project:o,outputPath:l})}async function n2(o){if(!$l()){const{pickedFiles:l}=await eo(async()=>{const{pickedFiles:i}=await Promise.resolve().then(()=>Qg);return{pickedFiles:i}},void 0,import.meta.url),r=l.get(o);if(!r)throw new Error("the chosen project file is no longer available");const{projectFromBytes:c}=await eo(async()=>{const{projectFromBytes:i}=await import("./webBackend-sNFyw_1v.js");return{projectFromBytes:i}},[],import.meta.url);return c(new Uint8Array(await r.arrayBuffer()))}return Ri("load_project",{inputPath:o})}const Dr=(()=>{try{return typeof window<"u"&&new URLSearchParams(window.location.search).get("perf")==="1"}catch{return!1}})(),Vt={frames:0,frameMs:0,worstFrameMs:0,renderCpuMs:0,drawCalls:0,triangles:0,geometries:0,textures:0,timings:{}};Dr&&typeof window<"u"&&(window.__PERF__=Vt);function qa(o,l){Dr&&(o.endsWith("MaxMs")?Vt.timings[o]=Math.max(Vt.timings[o]??0,l):Vt.timings[o]=l)}function o2(o,l){if(!Dr)return l();const r=performance.now();try{return l()}finally{qa(o,performance.now()-r)}}function l2(o){if(!Dr)return{begin(){},end(){},dispose(){}};o.info.autoReset=!1;const l=document.createElement("div");l.style.cssText="position:fixed;left:8px;top:8px;z-index:99999;padding:4px 8px;background:rgba(0,0,0,.65);color:#9f9;font:11px/1.35 ui-monospace,monospace;white-space:pre;pointer-events:none;border-radius:4px",document.body.appendChild(l);let r=0,c=0;return{begin(){o.info.reset(),c=performance.now()},end(){const i=performance.now(),f=i-c;if(Vt.renderCpuMs=Vt.renderCpuMs===0?f:Vt.renderCpuMs*.9+f*.1,r>0){const h=i-r;Vt.frameMs=Vt.frameMs===0?h:Vt.frameMs*.9+h*.1,Vt.worstFrameMs=Math.max(Vt.worstFrameMs*.995,h)}if(r=i,Vt.frames+=1,Vt.drawCalls=o.info.render.calls,Vt.triangles=o.info.render.triangles,Vt.geometries=o.info.memory.geometries,Vt.textures=o.info.memory.textures,Vt.frames%15===0){const h=Vt.timings;l.textContent=`frame ${Vt.frameMs.toFixed(1)} ms (worst ${Vt.worstFrameMs.toFixed(0)})  render ${Vt.renderCpuMs.toFixed(1)}
draws ${Vt.drawCalls}  tris ${(Vt.triangles/1e3).toFixed(0)}k  geo ${Vt.geometries}  tex ${Vt.textures}
`+Object.entries(h).map(([g,m])=>`${g} ${m.toFixed(0)}`).join("  ")}},dispose(){l.remove()}}}function yp(o){const l=new DataView(o,0,8),r=String.fromCharCode(l.getUint8(0),l.getUint8(1),l.getUint8(2),l.getUint8(3));if(r!=="CSB1")throw new Error(`not a city scene container (magic ${JSON.stringify(r)})`);const c=l.getUint32(4,!0),i=JSON.parse(new TextDecoder().decode(new Uint8Array(o,8,c))),f=Math.ceil((8+c)/4)*4,h=([d,x])=>new Float32Array(o,f+d,x/4),g=([d,x])=>new Uint32Array(o,f+d,x/4),m=([d,x])=>new Uint8Array(o,f+d,x);return{...i,meshes:i.meshes.map(d=>({...d,positions:h(d.positions),normals:m(d.normals),colors:d.colors?m(d.colors):void 0,uvs:d.uvs?h(d.uvs):void 0,indices:g(d.indices)})),instances:i.instances.map(d=>({...d,data:h(d.data)})),textures:i.textures.map(d=>({...d,data:m(d.data)}))}}async function xp(o){const{invoke:l}=await eo(async()=>{const{invoke:f}=await Promise.resolve().then(()=>Zg);return{invoke:f}},void 0,import.meta.url),r=performance.now(),c=await l("city_scene_bin",{index:o});qa("cityFetchMs",performance.now()-r);const i=c instanceof ArrayBuffer?c:new Uint8Array(c).buffer;return o2("cityDecodeMs",()=>yp(i))}const s2=Object.freeze(Object.defineProperty({__proto__:null,decodeCityBinary:yp,loadCityScene:xp},Symbol.toStringTag,{value:"Module"})),wp={tileLoadM:600,maxTiles:9,anisotropy:16,lodScale:1,grassShells:!0,treeLodM:[22,45,160],treeFarM:1500,tileBudgetMs:6},i2={...wp,tileBudgetMs:3},Jl=aa?i2:wp,Ka={treeDetail:450,treeCoreNear:150,treeCoreFar:500,hedge:350,tuft:180,tallProp:380,lowProp:160,fineStatic:700,flatStatic:1500,moveEpsilon:2.5,turnCosine:.9997,frustumMarginM:90,shadowTrees:700,shadowAll:2200},Ql=(()=>{const o=Jl.lodScale;return{treeDetail:Ka.treeDetail*o,treeCoreNear:Ka.treeCoreNear*o,treeCoreFar:Ka.treeCoreFar*o,hedge:Ka.hedge*o,tuft:Ka.tuft*o,tallProp:Ka.tallProp*o,lowProp:Ka.lowProp*o,fineStatic:Ka.fineStatic*o,flatStatic:Ka.flatStatic*o}})(),r2=/^(trim\.|wire|sign\/|signal\.|awning|balcony)/,c2=/^(kerb|marking\.|barrier|median)/,bp=45;function u2(o){return o.key.startsWith("car/far.")?bp:0}function f2(o){const{key:l,part:r}=o;return l.startsWith("car/")?l.startsWith("car/far.")?Number.POSITIVE_INFINITY:bp:l.startsWith("tree/")?Number.POSITIVE_INFINITY:l==="hedge"?Ql.hedge:l==="tuft"?Ql.tuft:l.startsWith("furniture/")?/lamp|pole|gantry|sign\.guide|shelter/.test(l)?Ql.tallProp:Ql.lowProp:Number.POSITIVE_INFINITY}function h2(o){return r2.test(o)?Ql.fineStatic:c2.test(o)?Ql.flatStatic:Number.POSITIVE_INFINITY}class d2{constructor(l){this.handles=l;for(const r of l.statics){const c=r.geometry.boundingBox??new _h().setFromObject(r);this.statics.push({object:r,box:c,radius:h2(r.name),cast:r.castShadow})}this.sets=l.lodSets,this.setRadius=this.sets.map(f2),this.setMinimum=this.sets.map(u2);for(const r of this.sets)r.mesh.userData.cast=r.mesh.castShadow}statics=[];sets;setRadius;setMinimum;lastCamera=new lt(Number.NaN,0,0);lastForward=new lt(0,0,-1);frustum=null;shadowTier=-1;keptInstances=0;hiddenStatics=0;update(l,r,c,i){const f=i<Ka.shadowTrees?0:i<Ka.shadowAll?1:2;let h=!1;if(f!==this.shadowTier){this.shadowTier=f,h=!0;for(const m of this.statics)m.object.castShadow=m.cast&&f<2;for(const m of this.sets)m.mesh.castShadow=!!m.mesh.userData.cast&&f<1}return!Number.isNaN(this.lastCamera.x)&&l.distanceToSquared(this.lastCamera)<=Ka.moveEpsilon*Ka.moveEpsilon&&r.dot(this.lastForward)>=Ka.turnCosine||(this.lastCamera.copy(l),this.lastForward.copy(r),this.frustum=c,this.rebuild(l)),h}rebuild(l){let r=0;for(const i of this.statics){const f=i.radius===Number.POSITIVE_INFINITY||i.box.distanceToPoint(l)<=i.radius;i.object.visible!==f&&(i.object.visible=f),f||(r+=1)}this.hiddenStatics=r;let c=0;for(let i=0;i<this.sets.length;i+=1){const f=this.sets[i],h=this.setRadius[i],g=this.setMinimum[i],m=f.key.startsWith("tree/"),d=m&&f.part==="mass",x=f.mesh,M=x.instanceMatrix.array,T=x.instanceColor?x.instanceColor.array:null,A=f.far,R=A?A.instanceMatrix.array:null,B=A&&A.instanceColor?A.instanceColor.array:null,C=f.bounds,H=this.frustum?this.frustum.planes:null,K=Ka.frustumMarginM,U=f.matrices,_=f.colors;let I=0,Z=0;for(let le=0;le<f.count;le+=1){const X=le*4,se=C[X]-l.x,we=C[X+1]-l.y,ge=C[X+2]-l.z,G=Math.sqrt(se*se+we*we+ge*ge)-C[X+3];if(H){let $=!1;for(let N=0;N<6;N+=1){const P=H[N],k=P.normal;if(k.x*C[X]+k.y*C[X+1]+k.z*C[X+2]+P.constant+K+C[X+3]<0){$=!0;break}}if($)continue}let F=!1;if(m){if(d){if(G<Ql.treeCoreNear)continue;F=G>=Ql.treeCoreFar}else if(G>=Ql.treeDetail)continue}else if(G>=h||G<g)continue;const ie=le*16;if(F&&R){const $=Z*16;for(let N=0;N<16;N+=1)R[$+N]=U[ie+N];B&&_&&(B[Z*3]=_[le*3],B[Z*3+1]=_[le*3+1],B[Z*3+2]=_[le*3+2]),Z+=1}else{const $=I*16;for(let N=0;N<16;N+=1)M[$+N]=U[ie+N];T&&_&&(T[I*3]=_[le*3],T[I*3+1]=_[le*3+1],T[I*3+2]=_[le*3+2]),I+=1}}this.commit(x,I,T!==null),A&&this.commit(A,Z,B!==null),c+=I+Z}this.keptInstances=c}commit(l,r,c){l.count=r,l.visible=r>0,r!==0&&(l.instanceMatrix.clearUpdateRanges(),l.instanceMatrix.addUpdateRange(0,r*16),l.instanceMatrix.needsUpdate=!0,c&&l.instanceColor&&(l.instanceColor.clearUpdateRanges(),l.instanceColor.addUpdateRange(0,r*3),l.instanceColor.needsUpdate=!0),l.computeBoundingSphere())}}const Mp=`
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
`,m2=`
${Mp}
uniform vec3 uFurTip;
uniform vec3 uFurDry;
`,p2=`
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
  // Worn and thin ground in broad drifts that survive distance: a lawn that is
  // one green from kerb to kerb is a carpet. And turf is a dull, dark green.
  float worn = furNoise(p * 0.19 + 23.0) * 0.65 + furNoise(p * 0.043 + 7.0) * 0.35;
  col = mix(col, col * vec3(1.12, 0.98, 0.74), smoothstep(0.45, 0.75, worn) * 0.5);
  col *= 0.82 + 0.30 * furNoise(p * 0.11 + 51.0);
  col = mix(vec3(dot(col, vec3(0.30, 0.59, 0.11))), col, 0.82) * 0.86;
  diffuseColor.rgb = col;
}
`,v2=`
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
`,g2=`
#if NUM_DIR_LIGHTS > 0
{
  vec3 L = directionalLights[0].direction;
  vec3 V = normalize(vViewPosition);
  float ndv = clamp(dot(normalize(vNormal), V), 0.0, 1.0);
  // A pile of thin blades is lighter across the grain...
  float rim = pow(1.0 - ndv, 3.0);
  // ...and glows when the sun is behind it.
  float back = pow(max(dot(-V, L), 0.0), 3.0) * 0.5 + 0.25;
  // The pile's glow is sunlight: none of it in shadow, and diffuse (1/pi).
  // Unshadowed and without that factor a lawn in shade was as bright as one in sun.
  float sunShade = 1.0;
  #if defined( USE_SHADOWMAP ) && NUM_DIR_LIGHT_SHADOWS > 0
    if ( receiveShadow ) sunShade = getShadow( directionalShadowMap[ 0 ], directionalLightShadows[ 0 ].shadowMapSize, directionalLightShadows[ 0 ].shadowIntensity, directionalLightShadows[ 0 ].shadowBias, directionalLightShadows[ 0 ].shadowRadius, vDirectionalShadowCoord[ 0 ] );
  #endif
  vec3 pile = diffuseColor.rgb * directionalLights[0].color * RECIPROCAL_PI * sunShade;
  totalEmissiveRadiance += pile * (rim * 1.2 * back + 0.10);
}
#endif
`;function y2(o){return o.replace("#include <common>",`#include <common>
varying vec3 vFurRel;`).replace("#include <project_vertex>",`#include <project_vertex>
#ifdef USE_INSTANCING
  vFurRel = (modelMatrix * instanceMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;
#else
  vFurRel = (modelMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;
#endif`)}function x2(o,l=14208906,r=12037482){const c=o.onBeforeCompile,i=o.customProgramCacheKey?.bind(o);o.customProgramCacheKey=()=>`${i?i():""}|fur`;const f=new Zt(l),h=new Zt(r);return f.multiplyScalar(1/.85),h.multiplyScalar(1/.72),o.onBeforeCompile=(g,m)=>{c?.(g,m),g.uniforms.uFurTip={value:f},g.uniforms.uFurDry={value:h},g.vertexShader=y2(g.vertexShader),g.fragmentShader=g.fragmentShader.replace("void main() {",`${m2}
void main() {`).replace("#include <color_fragment>",`#include <color_fragment>
${p2}`).replace("#include <normal_fragment_maps>",`#include <normal_fragment_maps>
${v2}`).replace("#include <emissivemap_fragment>",`#include <emissivemap_fragment>
${g2}`)},o.roughness=1,o.needsUpdate=!0,o}const Hc=16,w2=.11;function b2(o){const l=new Ye({color:7044167,roughness:1,metalness:0});return l.customProgramCacheKey=()=>"grass-shell",l.onBeforeCompile=r=>{r.uniforms.uShells={value:Hc},r.uniforms.uShellHeight={value:w2},r.vertexShader=r.vertexShader.replace("#include <common>",`#include <common>
varying vec3 vFurRel;
varying float vShell;
uniform float uShells;
uniform float uShellHeight;`).replace("#include <begin_vertex>",`#include <begin_vertex>
float shell = float(gl_InstanceID + 1) / uShells;
vShell = shell;
transformed += normalize(normal) * (uShellHeight * shell);`).replace("#include <project_vertex>",`#include <project_vertex>
vFurRel = (modelMatrix * vec4(transformed, 1.0)).xyz - cameraPosition;`),r.fragmentShader=r.fragmentShader.replace("void main() {",`${Mp}
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
}`)},l}class M2{constructor(l,r,c=36){this.reachM=c,this.material=b2();for(const i of l){const f=i.geometry.boundingBox??(i.geometry.computeBoundingBox(),i.geometry.boundingBox);this.tracked.push({object:i,box:f,shells:null})}}tracked=[];material;identity=new Co;carpetMesh=null;carpet(l,r,c=40){const i=new Bc(c*2,c*2,1,1);i.rotateX(-Math.PI/2);const f=new Va(i,this.material,Hc);for(let h=0;h<Hc;h+=1)f.setMatrixAt(h,this.identity);f.instanceMatrix.needsUpdate=!0,f.position.y=l,f.castShadow=!1,f.receiveShadow=!0,f.frustumCulled=!1,r.add(f),this.carpetMesh=f}update(l){this.carpetMesh&&(this.carpetMesh.position.x=Math.round(l.x),this.carpetMesh.position.z=Math.round(l.z));for(const r of this.tracked){const c=r.object.visible&&r.box.distanceToPoint(l)<this.reachM;if(c&&!r.shells){const i=new Va(r.object.geometry,this.material,Hc);for(let f=0;f<Hc;f+=1)i.setMatrixAt(f,this.identity);i.instanceMatrix.needsUpdate=!0,i.castShadow=!1,i.receiveShadow=!0,i.frustumCulled=!1,i.position.copy(r.object.position),i.quaternion.copy(r.object.quaternion),i.scale.copy(r.object.scale),r.object.parent?.add(i),r.shells=i}r.shells&&(r.shells.visible=c)}}dispose(){this.carpetMesh&&(this.carpetMesh.removeFromParent(),this.carpetMesh.geometry.dispose(),this.carpetMesh.dispose());for(const l of this.tracked)l.shells&&(l.shells.removeFromParent(),l.shells.dispose());this.material.dispose()}}const S2=32;class Nh{exports;speciesKeys;constructor(l){this.exports=l;const r=[],c=new TextDecoder;for(let i=0;i<l.tree_species_count();i+=1){const f=l.tree_species_key(i);r.push(c.decode(new Uint8Array(l.memory.buffer,l.tree_buffer_ptr(),f)))}this.speciesKeys=r}static async load(l="./worldgen_trees.wasm"){const r=await fetch(l);if(!r.ok)throw new Error(`${l}: ${r.status} ${r.statusText}`);const c=await r.arrayBuffer(),{instance:i}=await WebAssembly.instantiate(c,{});return new Nh(i.exports)}speciesIndex(l){return this.speciesKeys.indexOf(l)}grow(l,r){const c=Math.floor(l.seed),i=this.exports.tree_generate(l.species,c>>>0,Math.floor(c/4294967296)>>>0,l.height,l.openness??.7,l.age??.7,l.season??.5,l.health??1,l.lift??0,r),f=new Uint8Array(this.exports.memory.buffer,this.exports.tree_buffer_ptr(),i).slice().buffer,h=new DataView(f),g=U=>h.getUint32(U*4,!0),m=U=>h.getFloat32(U*4,!0);if(g(0)!==1162170964)throw new Error("not a tree buffer");const d=g(2),x=g(3),M=U=>[m(U),m(U+1),m(U+2)];let T=S2*4;const A=new Float32Array(f,T,d*8);T+=d*32;const R=new Float32Array(f,T,x*4);T+=x*16;const B=new Int8Array(f,T,x*4);T+=x*4;const C=new Int8Array(f,T,x*4);T+=x*4;const H=new Uint8Array(f,T,x*4);T+=x*4;const K=new Uint8Array(f,T,x*4);return{species:g(15),leafForm:g(16),lod:g(28),height:m(4),crownRadius:m(5),crownBase:m(6),trunkRadius:m(7),bark:M(8),cover:m(11),autumn:m(12),flush:m(13),bloom:m(14),foliage:M(17),autumnColour:M(20),bloomColour:M(23),leafAspect:m(26),fissure:m(27),segments:A,segmentCount:d,leafPlacement:R,leafDirection:B,leafNormal:C,leafShape:H,leafTint:K,leafCount:x}}}const T2=16;class Dh{workers=[];busy=[];waiting=new Map;next=1;static create(l=Math.max(1,Math.min(3,(navigator.hardwareConcurrency??4)-1)),r="./worldgen_trees.wasm"){if(typeof Worker>"u")return null;try{const c=new Dh,i=new URL(r,location.href).href;for(let f=0;f<l;f+=1){const h=new Worker(new URL(""+new URL("growWorker-B1vhxpPZ.js",import.meta.url).href,import.meta.url),{type:"module"});h.postMessage({init:i}),h.onmessage=g=>{const m=c.waiting.get(g.data.id);m&&(c.waiting.delete(g.data.id),c.busy[m.worker]-=1,g.data.data?m.resolve(g.data.data):m.reject(new Error(g.data.error??"worker failed")))},c.workers.push(h),c.busy.push(0)}return c}catch{return null}}get inFlight(){return this.busy.reduce((l,r)=>l+r,0)}get capacity(){return this.workers.length*T2}grow(l,r){let c=0;for(let f=1;f<this.busy.length;f+=1)this.busy[f]<this.busy[c]&&(c=f);const i=this.next++;return this.busy[c]+=1,new Promise((f,h)=>{this.waiting.set(i,{resolve:f,reject:h,worker:c}),this.workers[c].postMessage({id:i,spec:l,lod:r})})}dispose(){for(const l of this.workers)l.terminate();this.workers.length=0}}const _i=7,tl=256,C2=typeof location<"u"&&new URLSearchParams(location.search).get("treedebug")==="nobump"?"true":"false",_2=typeof location<"u"&&new URLSearchParams(location.search).get("treedebug")==="nodisp"?"0.0":"1.0",z2=`
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
  return texelFetch(uTreeTable, ivec2((i & ${tl-1}) * ${_i} + k, i >> 8), 0);
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
    // A tube that lies wholly below the ground (the planted foot of a trunk) is not
    // extended past its top: the overlap would stand out of the ground as a ring.
    float extTop = (aSegB.y < 0.001 && aSegA.y < -0.1) ? 0.0 : ext;
    float tt = mix(-ext, 1.0 + extTop, position.y);
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
    r *= 1.0 + flare * bark.w * 0.11 * ridge * ${_2};
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
    if (aTreeKind > 2.5) { hw = 0.55; curl = 0.0; }
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
`,A2=`
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
`,E2=`
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
`,R2=`
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

// A leaf card: a clump of many small leaves inside a ragged round envelope, the way far
// foliage is drawn in games. uv is the card's [-1,1]^2.
vec3 cardHash(vec2 c) {
  vec3 p = fract(vec3(c.xyx) * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yzx + 33.33);
  return fract((p.xxy + p.yzz) * p.zyx);
}

float cardCoverage(vec2 uv, float seed, out float vein, out float inner) {
  vec2 g = uv * 3.6 + seed;
  vec2 base = floor(g);
  float dmin = 1e3;
  float shade = 0.5;
  for (int j = -1; j <= 1; j++) {
    for (int i = -1; i <= 1; i++) {
      vec2 cell = base + vec2(float(i), float(j));
      vec3 h = cardHash(cell);
      vec2 c = cell + 0.5 + (h.xy - 0.5) * 0.9;
      float a = h.z * 6.2832;
      vec2 l = g - c;
      l = vec2(cos(a) * l.x + sin(a) * l.y, -sin(a) * l.x + cos(a) * l.y);
      float d = length(l / vec2(0.42, 0.85)) - 1.0;
      if (d < dmin) { dmin = d; shade = h.x; }
    }
  }
  float r = length(uv * vec2(1.0, 0.92));
  float wob = 0.12 * (cardHash(floor(uv * 4.0 + seed)).x - 0.5);
  float env = 1.0 - r + wob;
  float aa = max(fwidth(dmin), 1e-4);
  float cover = clamp(0.5 - dmin / aa, 0.0, 1.0) * clamp(env / 0.12, 0.0, 1.0);
  vein = 0.5 * shade;
  inner = clamp(-dmin / 0.6, 0.0, 1.0) * clamp(env * 2.0, 0.0, 1.0);
  return cover;
}

// Coverage of the blade at this fragment, 0 outside the outline and 1 inside.
float treeLeafCoverage(out float vein, out float inner) {
  if (vTreeKind > 2.5) return cardCoverage(vTreeB.xy / vec2(max(vTreeB.w, 0.2), 0.5) - vec2(0.0, 1.0), vTreeA.x * 37.0, vein, inner);
  float d = leafOutline(vTreeB.xy, vTreeB.z, vTreeB.w, vTreeA, vein);
  float aa = max(fwidth(d), 1e-5);
  inner = clamp(-d / 0.08, 0.0, 1.0);
  // A feature thinner than a pixel (a needle, a twig of leaflets, a distant leaf)
  // must fade towards the mass it belongs to, not vanish: grow the outline by half
  // a pixel, so nothing is ever drawn thinner than one.
  d -= 0.5 * aa;
  return clamp(0.5 - d / aa, 0.0, 1.0);
}
`,N2=`
float tBark = 0.0;
// PBR channels for bark, applied after the albedo: cavity occlusion and roughness.
float tBarkAO = 1.0;
float tBarkRough = 1.0;
if (vTreeKind > 0.5) {
  if (vTreeKind < 1.5) {
    vec3 axis = normalize(vTreeA.xyz);
    float fissure = vTreeA.w;
    float vert = smoothstep(0.5, 0.9, abs(axis.y));
    vec3 q = vTreeB.xyz;
    float R = max(vBarkAR.y, 0.004);
    // Metres of bark per pixel. Furrows and pits are a centimetre or less across, so
    // from a few metres away they are sub-pixel; drawn at full contrast they alias into
    // black dots that crawl as the camera moves. Fade them out as they stop resolving,
    // leaving the soft plate-scale tone that a real trunk shows from a distance.
    float barkPx = length(fwidth(q));
    float detail = 1.0 - smoothstep(0.004, 0.022, barkPx);
    // Plates about ten centimetres across the grain and several times that along it.
    vec3 sq = vec3(1.0, 1.0, 1.0);
    // Plate size by species: a deeply fissured trunk has plates a hand across, a lightly
    // ridged one a thumb across.
    float plateK = mix(20.0, 10.0, clamp(fissure, 0.0, 1.0));
    vec3 qa = q * plateK;
    vec3 qb = q * plateK * 2.0;
    float vv = vert;
    qa.y = mix(qa.y, qa.y * 0.07, vv);
    qb.y = mix(qb.y, qb.y * 0.22, vv);
    vec3 wv = vec3(tNoise(qa * 0.35), tNoise(qa * 0.35 + 7.0), tNoise(qa * 0.35 + 17.0)) - 0.5;
    qa += wv * 1.6;
    vec3 wa = tWorley(qa);
    // Ragged edges: the furrow wanders, and pinches shut in places.
    float edge = wa.y - wa.x + (tNoise(qa * 2.3 + 3.0) - 0.5) * 0.22 + (tNoise(qa * 7.0) - 0.5) * 0.08;
    // The furrow's soft edge is never narrower than a pixel: a hard threshold on a
    // sub-pixel feature steps in blocks, and the height-derived normal steps with it.
    float pxA = barkPx * plateK * 1.4;
    float furrow = 1.0 - smoothstep(0.0, 0.045 + 0.06 * fissure + pxA, edge);
    float rounding = smoothstep(0.0, 0.42, edge);
    vec3 wb = tWorley(qb + wv * 1.5);
    float crack = (1.0 - smoothstep(0.0, 0.05 + pxA * 2.0, wb.y - wb.x + (tNoise(qb * 1.5) - 0.5) * 0.12)) * (1.0 - furrow) * 0.6;
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
    tBark = h * smoothstep(0.025, 0.07, R) * (0.35 + 0.65 * detail) * 0.6;
    float woodGrain = 0.55 + 0.45 * sin(q.x * 90.0 + q.z * 90.0 + 9.0 * tNoise(q * vec3(18.0, 4.0, 18.0)));
    vec3 woodCol = mix(vec3(0.30, 0.13, 0.045), vec3(0.78, 0.43, 0.15), woodGrain) * (0.8 + 0.4 * fine);
    // Bark itself: warm grey-brown, each plate its own tone, silvered on the high ones, near-black in the furrows.
    // Colour by age of the wood: thin young twigs are warm tan-brown, old thick limbs
    // dark brown. Only smooth-barked species (plane, ginkgo) stay pale, and mottled.
    float thick = smoothstep(0.015, 0.14, R);
    vec3 youngC = vec3(0.34, 0.26, 0.185);
    vec3 oldC = vec3(0.285, 0.215, 0.155);
    vec3 woody = mix(youngC, oldC, thick);
    float smoothBark = 1.0 - smoothstep(0.18, 0.40, fissure);
    // London plane: cream-grey, with the pale inner bark showing where the outer plates
    // have flaked away. The species colour only tints it; the old warm multiplier made
    // every plane trunk read orange-brown.
    vec3 paleC = mix(vec3(0.52, 0.46, 0.34), vTreeColour.rgb * vec3(1.15, 1.10, 0.95), 0.2);
    vec3 base = mix(woody, mix(paleC, woody, 0.15 * thick), smoothBark);
    base *= mix(vec3(1.0), vTreeColour.rgb / max(dot(vTreeColour.rgb, vec3(0.33)), 0.02) * 0.9, 0.18);
    vec3 plateCol = base * (0.94 + 0.10 * plate) * (1.0 + (grain - 0.5) * 0.14 * detail) * (0.96 + 0.08 * dome);
    // Wood, not stone: bark is fibrous, long streaks of lighter and darker fibre running
    // with the grain (the axis is vertical, so the noise is stretched along y), ridge
    // crowns lighter than the grooves between them, and hue that wanders between warm
    // red-brown and olive-grey from one plate to the next. Stone is uniform in hue and
    // is cut by closed polygonal cracks.
    float fibre = tNoise(vec3(q.x * 70.0 + q.z * 70.0, q.y * 3.0, 0.7));
    float fibre2 = tNoise(vec3(q.x * 180.0 + q.z * 180.0, q.y * 9.0, 3.1));
    float woodiness = 1.0 - 0.6 * smoothBark;
    plateCol *= 1.0 + woodiness * detail * (0.40 * (fibre - 0.5) + 0.22 * (fibre2 - 0.5));
    plateCol *= mix(0.78 + 0.44 * rounding, 1.0, smoothBark);
    plateCol *= mix(vec3(1.0), vec3(1.14, 0.96, 0.78), tNoise(q * vec3(5.0, 1.2, 5.0) + 13.0) * woodiness);
    // Mottling on smooth bark: flaked patches of olive-grey and cream.
    float mott = tNoise(q * 6.0 + 21.0);
    plateCol = mix(plateCol, plateCol * vec3(0.92, 0.95, 0.88), smoothBark * smoothstep(0.5, 0.75, mott) * 0.25);
    // Mottling of a plane trunk: broad, soft, low-contrast patches of olive-grey and
    // greyer cream, a hand's width or more across, equally wide and tall. Real trunks
    // are pale with gentle tonal variation; hard, tall, high-contrast patches read as
    // camouflage or zebra stripes.
    float mottBig = tNoise(q * 3.0 + 41.0);
    float mottMid = tNoise(q * 7.0 + 63.0);
    float mPatch = smoothstep(0.34, 0.68, mottBig * 0.75 + mottMid * 0.25);
    float mDark = smoothstep(0.55, 0.90, tNoise(q * 4.5 + 87.0));
    plateCol = mix(plateCol, plateCol * vec3(0.70, 0.74, 0.56), smoothBark * mPatch * 0.75);
    plateCol = mix(plateCol, plateCol * vec3(0.72, 0.64, 0.52), smoothBark * mDark * 0.30);
    vec3 creviceCol = vec3(0.135, 0.100, 0.072);
    // Real bark is matte and low in saturation, and varies a great deal in value over a
    // metre: weathering, damp, where rain runs down it. Plastic is uniform and clean.
    float lum0 = dot(plateCol, vec3(0.30, 0.59, 0.11));
    plateCol = mix(vec3(lum0), plateCol, mix(1.0, 0.9, smoothBark));
    float weather = tNoise(q * vec3(1.6, 0.35, 1.6) + 5.0);
    float streak = tNoise(vec3(q.x * 9.0 + q.z * 9.0, q.y * 0.8, 3.0));
    plateCol *= mix(0.95 + 0.55 * weather, 0.90 + 0.20 * weather, smoothBark);
    plateCol *= mix(0.88 + 0.24 * streak, 0.96 + 0.08 * streak, smoothBark);
    // Fine pitting and a dusting of grit, so no surface is perfectly clean.
    plateCol *= 1.0 - detail * 0.08 + detail * 0.08 * tNoise(q * 320.0);
    // Young wood is smooth: bark only fissures into plates once a limb is a few
    // centimetres thick. On a twig a centimetre across, plates ten centimetres wide
    // put its whole surface in a furrow, and every twig drew near-black.
    float fissured = smoothstep(0.025, 0.07, R) * (1.0 - 0.9 * smoothBark);
    vec3 col = mix(plateCol, creviceCol, clamp((furrow * 0.45 + crack * 0.12) * fissured * detail, 0.0, 1.0));
    // Cavity occlusion is its own channel, applied to ambient light only: a furrow is
    // dark because little sky reaches into it, not because its albedo is black. Doing it
    // in albedo is what made bark a field of black dots.
    float cavity = clamp(furrow * 0.9 + crack * 0.3, 0.0, 1.0) * fissured;
    tBarkAO = 1.0 - 0.45 * cavity * mix(0.5, 1.0, detail);
    // Dry bark is rough everywhere; plate faces polished by weather are a little less so.
    tBarkRough = 0.97 - 0.07 * (1.0 - cavity) * smoothstep(0.3, 0.9, plateId);
    // Planted in the ground: soil splashed up the foot, damp and dark where the wood meets
    // the earth, and less light reaching a surface so close to it.
    float foot = 1.0 - smoothstep(-0.2, 0.9, vAbove);
    float splash = (1.0 - smoothstep(0.0, 0.9, vAbove)) * (0.55 + 0.45 * tNoise(q * 14.0));
    vec3 soil = vec3(0.060, 0.043, 0.030);
    col = mix(col, col * 0.45 + soil * 0.9, clamp(splash * 0.9 + foot * 0.7, 0.0, 0.95));
    col *= mix(0.40, 1.0, smoothstep(0.0, 1.8, vAbove));
    col = mix(col, woodCol, wornMask);
    // Bark is among the darkest things in a street (5 - 15 % reflectance) and
    // grey-brown: at twice that and warmer, trunks read as terracotta.
    col = mix(vec3(dot(col, vec3(0.30, 0.59, 0.11))), col, 0.72) * 0.56;
    // Lichen and damp in patches, on the shaded faces.
    float lichen = smoothstep(0.66, 0.84, tNoise(q * 4.5 + 9.0));
    col = mix(col, col * vec3(0.78, 1.18, 0.62) + vec3(0.01, 0.02, 0.0), lichen * 0.0);
    diffuseColor.rgb = col;
  } else {
    float vein;
    float inner;
    float cover = treeLeafCoverage(vein, inner);
    // Sunlit plane and broadleaf canopy is yellow-green and lighter than the species
    // reflectance suggests once the shade between leaves is handled by the lights.
    // A leaf's own reflectance: mid green, not lime. The brightness of a sunlit
    // crown comes from the sun and from light through the leaf, not from albedo.
    vec3 col = vTreeColour.rgb * vec3(1.02, 1.04, 0.76);
    col = mix(col, col * 1.22 + vec3(0.02, 0.025, 0.0), vein * 0.6);
    col *= 0.88 + 0.12 * inner;
    diffuseColor.rgb = col;
    diffuseColor.a = cover;
  }
}
`,Z0=typeof location<"u"?new URLSearchParams(location.search).get("treedebug"):null,D2=`
if (vTreeKind > 0.5 && vTreeKind < 1.5 && !${C2}) {
  // Bump from the bark height (metres), by screen-space derivatives.
  vec3 sx = dFdx(-vViewPosition);
  vec3 sy = dFdy(-vViewPosition);
  float hx = dFdx(tBark);
  float hy = dFdy(tBark);
  vec3 r1 = cross(sy, normal);
  vec3 r2 = cross(normal, sx);
  float det = dot(sx, r1);
  vec3 grad = sign(det) * (hx * r1 + hy * r2);
  // The tilt is bounded: derivatives are per 2x2 pixel quad, so an unbounded one flips
  // the normal in blocks wherever the height jumps, and those blocks go black.
  vec3 tilt = grad / max(abs(det), 1e-12);
  float tl = length(tilt);
  tilt *= min(1.0, 0.35 / max(tl, 1e-6));
  normal = normalize(normal - tilt);
}
`,k2=`
${Z0==="normal"?"if (vTreeKind > 0.5) { totalEmissiveRadiance = normal * 0.5 + 0.5; diffuseColor.rgb = vec3(0.0); }":""}
${Z0==="albedo"?"if (vTreeKind > 0.5) { totalEmissiveRadiance = diffuseColor.rgb; diffuseColor.rgb = vec3(0.0); }":""}
${Z0==="sun"?"if (vTreeKind > 0.5) { totalEmissiveRadiance = vec3(max(dot(normal, directionalLights[0].direction), 0.0)); diffuseColor.rgb = vec3(0.0); }":""}
${Z0==="front"?"if (vTreeKind > 0.5) { totalEmissiveRadiance = gl_FrontFacing ? vec3(0.0,1.0,0.0) : vec3(1.0,0.0,0.0); diffuseColor.rgb = vec3(0.0); }":""}
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
  // Transmitted light is sunlight like any other: it needs the sun to reach the
  // leaf (the shadow map says whether it does), and it leaves the leaf diffusely,
  // so it carries the same 1/pi as reflected light. Without either, every leaf
  // in a crown glowed lime whether or not it was lit, at twice the brightness
  // of a leaf in full sun, and a canopy seen from below had no dark in it.
  // Looked up a clump's depth toward the sun: a leaf lit from behind must not be
  // shadowed by itself, and light does carry through a few leaves.
  float sunShade = 1.0;
  #if defined( USE_SHADOWMAP ) && NUM_DIR_LIGHT_SHADOWS > 0
    if ( receiveShadow ) sunShade = getShadow( directionalShadowMap[ 0 ], directionalLightShadows[ 0 ].shadowMapSize, directionalLightShadows[ 0 ].shadowIntensity, directionalLightShadows[ 0 ].shadowBias * 5.0, directionalLightShadows[ 0 ].shadowRadius, vDirectionalShadowCoord[ 0 ] );
  #endif
  totalEmissiveRadiance += through * (m * 0.9 + 0.006) * directionalLights[0].color * (t * 2.2 * RECIPROCAL_PI) * seen * sunShade;
}
#endif
`;class U2{uniform;data;capacity;free=[];constructor(l=16){this.capacity=l*tl,this.data=new Float32Array(tl*_i*l*4);const r=new fa(this.data,tl*_i,l,$n,sp);r.minFilter=Ks,r.magFilter=Ks,r.generateMipmaps=!1,r.needsUpdate=!0,this.uniform={value:r},this.free.push([0,this.capacity])}allocate(l){for(;;){const r=this.free.findIndex(([,c])=>c>=l);if(r>=0){const[c,i]=this.free[r];return i===l?this.free.splice(r,1):this.free[r]=[c+l,i-l],c}this.grow(Math.max(l,this.capacity))}}release(l,r){this.free.push([l,r]),this.free.sort((i,f)=>i[0]-f[0]);const c=[];for(const i of this.free){const f=c[c.length-1];f&&f[0]+f[1]===i[0]?f[1]+=i[1]:c.push([i[0],i[1]])}this.free=c}grow(l){const r=Math.ceil((this.capacity+l)/tl),c=new Float32Array(tl*_i*r*4);c.set(this.data);const i=this.capacity;this.data=c,this.capacity=r*tl;const f=this.uniform.value;f.dispose(),f.image={data:c,width:tl*_i,height:r},f.needsUpdate=!0,this.release(i,this.capacity-i)}put(l,r){const c=((l>>8)*tl*_i+(l&tl-1)*_i)*4;r.forEach((i,f)=>this.data.set(i,c+f*4)),this.uniform.value.needsUpdate=!0}}const cu=new U2;function O2(o){let l=o.replace("#include <common>",`#include <common>
${z2}`);return l.includes("#include <beginnormal_vertex>")&&(l=l.replace("#include <beginnormal_vertex>",A2)),l.replace("#include <begin_vertex>",E2)}function Lc(o,l){const r=o.onBeforeCompile,c=`tree-${l}`,i=o.customProgramCacheKey?.bind(o);o.customProgramCacheKey=()=>`${i?i():""}${c}`,o.onBeforeCompile=(f,h)=>{r?.(f,h),f.uniforms.uTreeTable=cu.uniform,f.vertexShader=O2(f.vertexShader);let g=f.fragmentShader;g=g.replace("void main() {",`${R2}
void main() {`),l==="lit"?(g=g.replace("#include <color_fragment>",`#include <color_fragment>
${N2}`),g=g.replace("#include <normal_fragment_maps>",`#include <normal_fragment_maps>
${D2}`),g=g.replace("#include <roughnessmap_fragment>",`#include <roughnessmap_fragment>
if (vTreeKind > 0.5 && vTreeKind < 1.5) roughnessFactor = tBarkRough;`),g=g.replace("#include <aomap_fragment>",`#include <aomap_fragment>
if (vTreeKind > 0.5 && vTreeKind < 1.5) { reflectedLight.indirectDiffuse *= tBarkAO * vec3(1.12, 0.96, 0.78); reflectedLight.indirectSpecular *= tBarkAO; }
// Sky reaches the skin of a crown and little of its inside: the leaf's depth in the crown occludes its ambient light.
if (vTreeKind > 1.5) { float crownAO = mix(0.40, 1.0, smoothstep(0.05, 0.70, vTreeColour.a)); reflectedLight.indirectDiffuse *= crownAO; reflectedLight.indirectSpecular *= crownAO; }`),g=g.replace("#include <emissivemap_fragment>",`#include <emissivemap_fragment>
${k2}`)):l==="depth"?g=g.replace("#include <alphamap_fragment>",`#include <alphamap_fragment>
if (vTreeKind > 1.5) {
  float tv; float ti;
  diffuseColor.a = treeLeafCoverage(tv, ti);
  // A crown is not opaque: sun reaches the ground through gaps between leaf clumps.
  // Hash the shadow-map texel in 2x2 blocks and let a share of leaves cast nothing, so
  // the ground gets dappled light and not one solid shadow.
  vec2 gp = floor(gl_FragCoord.xy * 0.5);
  float gh = fract(sin(dot(gp, vec2(12.9898, 78.233))) * 43758.5453);
  if (gh < 0.34) diffuseColor.a = 0.0;
}`):g=g.replace("#include <normal_fragment_begin>",`#include <normal_fragment_begin>
if (vTreeKind > 1.5) { float tv; float ti; if (treeLeafCoverage(tv, ti) < 0.5) discard; }`),f.fragmentShader=g},o.needsUpdate=!0}function B2(){const o=new Ye({color:16777215,roughness:1,metalness:0,envMapIntensity:.1});Lc(o,"lit");const l=new Ye({color:16777215,roughness:.88,metalness:0,envMapIntensity:.6,side:ct,alphaTest:.5,alphaToCoverage:!1});Lc(l,"lit");const r=new d1({depthPacking:m1});Lc(r,"depth");const c=new d1({depthPacking:m1,alphaTest:.5});return c.side=ct,Lc(c,"depth"),{wood:o,leaf:l,woodDepth:r,leafDepth:c,dispose(){o.dispose(),l.dispose(),r.dispose(),c.dispose()}}}let Q0=null;function H2(){if(Q0)return Q0;const o=64,l=document.createElement("canvas");l.width=o,l.height=o;const r=l.getContext("2d");if(r){const i=r.createRadialGradient(o/2,o/2,0,o/2,o/2,o/2);i.addColorStop(0,"rgba(255,255,255,0.95)"),i.addColorStop(.35,"rgba(255,255,255,0.55)"),i.addColorStop(1,"rgba(255,255,255,0)"),r.fillStyle=i,r.fillRect(0,0,o,o)}const c=new wu(l);return Q0=new Dn({map:c,color:723205,transparent:!0,opacity:.6,depthWrite:!1,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),Q0}const L2=[16,10,7,5];function j2(o){const l=o+1,r=new Float32Array(l*2*3),c=new Float32Array(l*2*3),i=new Float32Array(l*2).fill(1);for(let g=0;g<l;g+=1){const m=g/o*Math.PI*2,d=Math.cos(m),x=Math.sin(m);for(let M=0;M<2;M+=1){const T=(g*2+M)*3;r[T]=d,r[T+1]=M,r[T+2]=x,c[T]=d,c[T+2]=x}}const f=[];for(let g=0;g<o;g+=1){const m=g*2,d=m+1,x=m+2,M=m+3;f.push(m,x,d,d,x,M)}const h=new ip;return h.setAttribute("position",new Jt(r,3)),h.setAttribute("normal",new Jt(c,3)),h.setAttribute("aTreeKind",new Jt(i,1)),h.setIndex(f),h}function F2(o){const l=o===0?2:1,r=o===0||o===1?2:1,c=o>=2?3:2,i=[],f=[],h=[];for(let x=0;x<=l;x+=1)for(let M=0;M<=r;M+=1)i.push(M/r*2-1,x/l,0),f.push(0,0,1),h.push(c);const g=[],m=r+1;for(let x=0;x<l;x+=1)for(let M=0;M<r;M+=1){const T=x*m+M;g.push(T,T+1,T+m,T+1,T+m+1,T+m)}const d=new ip;return d.setAttribute("position",new ft(i,3)),d.setAttribute("normal",new ft(f,3)),d.setAttribute("aTreeKind",new ft(h,1)),d.setIndex(g),d}class G2{constructor(l,r,c={},i=null){this.grower=l,this.pool=i,this.trees=r,this.lodDistances=c.lodDistancesM??[22,45,160],this.farM=c.farM??1500,this.season=c.season??.5,this.rowBase=cu.allocate(r.length),this.materials=B2(),this.rowDone=new Uint8Array(r.length),this.group.name="unique-forest";const f=c.chunkM??64,h=new Map;r.forEach((g,m)=>{const d=Math.floor(g.x/f),x=Math.floor(g.z/f),M=`${d},${x}`;let T=h.get(M);T||(T={key:M,trees:[],min:new lt(1/0,1/0,1/0),max:new lt(-1/0,-1/0,-1/0),centre:new zt,lod:-1,wanted:-1,distance:1/0,wood:null,thin:null,leaf:null,decal:null,job:null},h.set(M,T),this.chunks.push(T)),T.trees.push(m);const A=g.height*.7;T.min.min(new lt(g.x-A,g.y,g.z-A)),T.max.max(new lt(g.x+A,g.y+g.height*1.05,g.z+A))});for(const g of this.chunks)g.centre.set((g.min.x+g.max.x)/2,(g.min.z+g.max.z)/2);this.stats.chunks=this.chunks.length}group=new _a;chunks=[];trees;rowBase;materials;rowDone;lodDistances;farM;season;casting=!0;generation=0;stats={chunks:0,built:0,trees:0,segments:0,leaves:0,growMs:0,jobs:0};get pending(){let l=0;for(const r of this.chunks)r.wanted>=0&&r.lod!==r.wanted&&(l+=1);return l}get treeCount(){return this.trees.length}setSeason(l){if(!(Math.abs(l-this.season)<1e-4)){this.season=l,this.generation+=1,this.rowDone.fill(0);for(const r of this.chunks)r.job=null,r.lod=-1}}setCasting(l){if(l!==this.casting){this.casting=l;for(const r of this.chunks)this.applyCasting(r)}}applyCasting(l){const r=this.casting&&l.lod>=0&&l.lod<=2;l.wood&&(l.wood.castShadow=r),l.thin&&(l.thin.castShadow=!1),l.leaf&&(l.leaf.castShadow=r)}wantedLevel(l,r){if(l>this.farM)return-2;const[c,i,f]=this.lodDistances,h=g=>r>=0&&r<=g?1.1:.92;return l<c*h(0)?0:l<i*h(1)?1:l<f*h(2)?2:3}update(l,r){const c=performance.now();let i=!1;const f=[];for(const h of this.chunks){const g=Math.max(h.min.x-l.x,0,l.x-h.max.x),m=Math.max(h.min.y-l.y,0,l.y-h.max.y),d=Math.max(h.min.z-l.z,0,l.z-h.max.z);if(h.distance=Math.sqrt(g*g+m*m+d*d),h.wanted=this.wantedLevel(h.distance,h.lod),h.wanted===-2){h.job=null,(h.wood?.visible||h.leaf?.visible)&&(this.hide(h),i=!0);continue}h.lod!==h.wanted?(h.job&&h.job.lod!==h.wanted&&(h.job=null),f.push(h)):h.wood&&!h.wood.visible&&(h.wood.visible=!0,h.thin&&(h.thin.visible=!0),h.leaf&&(h.leaf.visible=!0),i=!0)}this.stats.jobs=f.length,f.sort((h,g)=>h.distance-g.distance);for(const h of f){if(performance.now()-c>=r)break;h.job??={chunk:h,lod:h.wanted,next:0,grown:[],requested:0,arrived:0,generation:this.generation,failed:!1},this.advance(h.job,c+r)&&(i=!0)}return i}hide(l){l.wood&&(l.wood.visible=!1),l.thin&&(l.thin.visible=!1),l.leaf&&(l.leaf.visible=!1)}advance(l,r){if(this.pool)return this.advanceAsync(l);const{chunk:c}=l,i=this.generation;for(;l.next<c.trees.length;){const f=c.trees[l.next],h=this.trees[f],g=performance.now(),m=this.grower.grow({species:h.species,seed:h.seed,height:h.height,openness:h.openness,age:h.age,season:this.season,health:h.health,lift:h.lift},l.lod);if(this.stats.growMs+=performance.now()-g,this.rowDone[f]||this.writeRow(f,m),l.grown.push(m),l.next+=1,performance.now()>=r&&l.next<c.trees.length)return!1}return i!==this.generation?!1:(this.finish(l),!0)}advanceAsync(l){const{chunk:r}=l,c=this.pool;for(;l.requested<r.trees.length&&c.inFlight<c.capacity;){const i=l.requested;l.requested+=1;const f=this.trees[r.trees[i]],h=performance.now();c.grow({species:f.species,seed:f.seed,height:f.height,openness:f.openness,age:f.age,season:this.season,health:f.health,lift:f.lift},l.lod).then(g=>{this.stats.growMs+=performance.now()-h,l.grown[i]=g,l.arrived+=1}).catch(()=>{l.failed=!0,l.arrived+=1})}return l.arrived<r.trees.length?!1:l.failed||l.generation!==this.generation||r.job!==l?(r.job=null,!1):(r.trees.forEach((i,f)=>{this.rowDone[i]||this.writeRow(i,l.grown[f])}),this.finish(l),!0)}writeRow(l,r){const c=i=>[...i];cu.put(this.rowBase+l,[[...c(r.foliage),r.leafForm],[...c(r.autumnColour),r.autumn],[...c(r.bloomColour),r.bloom],[...c(r.bark).map(i=>i*1.3),r.fissure],[r.flush,r.leafAspect,r.cover,r.crownRadius],[this.trees[l].x,this.trees[l].y+r.crownBase+.5*(r.height-r.crownBase),this.trees[l].z,.5*(r.height-r.crownBase)],[this.trees[l].x,this.trees[l].y,this.trees[l].z,r.crownBase]]),this.rowDone[l]=1}finish(l){const{chunk:r,lod:c,grown:i}=l;let f=0,h=0;for(const P of i)f+=P.segmentCount,h+=P.leafCount;const g=new Float32Array(f*4),m=new Float32Array(f*4),d=new Uint16Array(f),x=new Float32Array(h*4),M=new Int8Array(h*4),T=new Int8Array(h*4),A=new Uint8Array(h*4),R=new Uint8Array(h*4),B=new Uint16Array(h);let C=0,H=0;i.forEach((P,k)=>{const ee=r.trees[k],Me=this.trees[ee],{x:S,y:Q,z:ve}=Me;for(let V=0;V<P.segmentCount;V+=1){const pe=V*8,ue=(C+V)*4;g[ue]=P.segments[pe]+S,g[ue+1]=P.segments[pe+1]+Q,g[ue+2]=P.segments[pe+2]+ve,g[ue+3]=P.segments[pe+3],m[ue]=P.segments[pe+4]+S,m[ue+1]=P.segments[pe+5]+Q,m[ue+2]=P.segments[pe+6]+ve,m[ue+3]=P.segments[pe+7],d[C+V]=this.rowBase+ee}C+=P.segmentCount;for(let V=0;V<P.leafCount;V+=1){const pe=V*4,ue=(H+V)*4;x[ue]=P.leafPlacement[pe]+S,x[ue+1]=P.leafPlacement[pe+1]+Q,x[ue+2]=P.leafPlacement[pe+2]+ve,x[ue+3]=P.leafPlacement[pe+3],B[H+V]=this.rowBase+ee}M.set(P.leafDirection,H*4),T.set(P.leafNormal,H*4),A.set(P.leafShape,H*4),R.set(P.leafTint,H*4),H+=P.leafCount});const K=new Iv;new _h(r.min,r.max).getBoundingSphere(K),K.radius+=2;const U=[],_=[];for(let P=0;P<f;P+=1)(Math.max(g[P*4+3],m[P*4+3])<.02?_:U).push(P);const I=(P,k)=>{const ee=new Float32Array(P.length*4),Me=new Float32Array(P.length*4),S=new Uint16Array(P.length);P.forEach((ve,V)=>{ee.set(g.subarray(ve*4,ve*4+4),V*4),Me.set(m.subarray(ve*4,ve*4+4),V*4),S[V]=d[ve]});const Q=j2(k);return Q.setAttribute("aSegA",new ql(ee,4)),Q.setAttribute("aSegB",new ql(Me,4)),Q.setAttribute("aTreeId",new ql(S,1)),Q.instanceCount=P.length,Q.boundingSphere=K.clone(),Q},Z=I(U,L2[c]),le=I(_,3),X=F2(c);X.setAttribute("aLeafPos",new ql(x,4)),X.setAttribute("aLeafDir",new ql(M,4,!0)),X.setAttribute("aLeafNor",new ql(T,4,!0)),X.setAttribute("aLeafShape",new ql(A,4,!0)),X.setAttribute("aLeafTint",new ql(R,4,!0)),X.setAttribute("aTreeId",new ql(B,1)),X.instanceCount=h,X.boundingSphere=K.clone();const se=new pt(Z,this.materials.wood);se.customDepthMaterial=this.materials.woodDepth,se.receiveShadow=!0,se.name=`forest/${r.key}/wood`;const we=new pt(le,this.materials.wood);we.receiveShadow=!1,we.name=`forest/${r.key}/twigs`;const ge=new pt(X,this.materials.leaf);ge.customDepthMaterial=this.materials.leafDepth,ge.receiveShadow=!0,ge.name=`forest/${r.key}/leaf`;const G=[],F=[];r.trees.forEach((P,k)=>{const ee=this.trees[P],Me=Math.max(1.6,i[k].trunkRadius*7+.8),S=ee.y+.03;G.push(ee.x-Me,S,ee.z-Me,ee.x+Me,S,ee.z-Me,ee.x+Me,S,ee.z+Me,ee.x-Me,S,ee.z+Me),F.push(0,0,1,0,1,1,0,1)});const ie=new It;ie.setAttribute("position",new ft(G,3)),ie.setAttribute("uv",new ft(F,2));const $=[];for(let P=0;P<r.trees.length;P+=1)$.push(P*4,P*4+2,P*4+1,P*4,P*4+3,P*4+2);ie.setIndex($);const N=new pt(ie,H2());N.renderOrder=1,N.frustumCulled=!1,N.name=`forest/${r.key}/contact`,N.userData.noOcclusion=!0,this.release(r),r.decal=N,this.group.add(N),r.wood=se,r.thin=we,this.group.add(we),r.leaf=ge,r.lod=c,r.job=null,this.group.add(se,ge),this.applyCasting(r),this.recount()}release(l){for(const r of[l.wood,l.thin,l.leaf])r&&(r.removeFromParent(),r.geometry.dispose());l.wood=null,l.thin=null,l.leaf=null,l.decal&&(l.decal.removeFromParent(),l.decal.geometry.dispose(),l.decal=null)}recount(){let l=0,r=0,c=0,i=0;for(const f of this.chunks)!f.wood||!f.leaf||(l+=1,r+=f.trees.length,c+=f.wood.geometry.instanceCount+(f.thin?f.thin.geometry.instanceCount:0),i+=f.leaf.geometry.instanceCount);Object.assign(this.stats,{built:l,trees:r,segments:c,leaves:i})}dispose(){for(const l of this.chunks)this.release(l);this.materials.dispose(),cu.release(this.rowBase,this.trees.length),this.group.removeFromParent()}}let D1=null,Sp=null;function q2(o){return D1??=Nh.load(o).then(l=>(Sp=l,l)).catch(l=>(console.warn("tree grower unavailable:",l),null)),D1}function V2(){return Sp}function K2(...o){let l=2654435769,r=2246822507;for(const c of o){const i=Math.round(c*8)|0;l=Math.imul(l^i,2246822507),l^=l>>>13,r=Math.imul(r+l+668265263,3266489909),r^=r>>>16}return l=Math.imul(l^l>>>15,739982445),r=Math.imul(r^r>>>12,695872825),(r>>>0&2097151)*4294967296+(l>>>0)}let nh;function P2(){return nh===void 0&&(nh=new URLSearchParams(location.search).get("workers")==="0"?null:Dh.create()),nh}function oh(o,l){return(Math.sin(o*12.9898+l*78.233)*43758.5453%1+1)%1}function Y2(o,l,r){const c=[],i=[],f=[],h=new Set,g=new Map;for(const d of o){if(!d.key.startsWith("tree/"))continue;i.push(d.mesh),d.far&&i.push(d.far),f.push(d);const x=d.mesh.geometry;x.boundingBox||x.computeBoundingBox();const M=x.boundingBox?.max.y??0;g.set(d.key,Math.max(g.get(d.key)??0,M))}const m=Number(r)%1000003;for(const d of f){if(h.has(d.key))continue;h.add(d.key);const x=d.key.split("/")[1],M=l.speciesIndex(x),T=M>=0?M:Math.max(0,l.speciesIndex("xiang-zhang")),A=g.get(d.key)||10,R=d.matrices;for(let B=0;B<d.count;B+=1){const C=B*16,H=Math.hypot(R[C+4],R[C+5],R[C+6]),K=R[C+12],U=R[C+13],_=R[C+14],I=K2(m,K,_,T);c.push({x:K,y:U,z:_,species:T,height:Math.max(2,A*H),seed:I,openness:.55+.4*oh(K,_),age:.55+.45*oh(_,K+3.1),health:.85+.15*oh(K+1.7,_-.4),lift:0})}}return{trees:c,replaced:i,sets:f}}function W2(o,l){const r=V2();if(!r)return null;const c=o.lodSets.length>0?o.lodSets:[...o.instanced.entries()].map(([m,d])=>({mesh:d,key:m,count:d.count,matrices:d.instanceMatrix.array.slice(0,d.count*16),far:null})),i=Y2(c,r,l.seed);if(i.trees.length===0)return null;const f=new G2(r,i.trees,{lodDistancesM:Jl.treeLodM,farM:Jl.treeFarM},P2());for(const m of i.replaced)m.removeFromParent();const h=new Set(i.sets);o.lodSets=o.lodSets.filter(m=>!h.has(m));const g=[];o.group.traverse(m=>{m.isInstancedMesh&&m.name.startsWith("tree/")&&g.push(m)});for(const m of g)m.removeFromParent();for(const m of[...o.instanced.keys()])m.startsWith("tree/")&&o.instanced.delete(m);return f}const nl={yard:4,parking:5,asphaltLot:6,plaza:7,lawn:8,carParkBase:9},Tp=`
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
    // (Far form only: the metre-scale scene uses carParkBase.)
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
  } else if (kind > 8.5) {
    // Car-park base: worn asphalt, tyre-polished aisles and oil stains. Bays and
    // cars are real geometry (buildings/parking.rs), so none are painted here.
    float aisle = smoothstep(0.55, 0.8, gNoise(vec2(q.x / 40.0, q.y / 3.0) + 5.0));
    c *= 1.0 - 0.07 * aisle * (1.0 - smoothstep(0.5, 3.0, fw));
    float oil = smoothstep(0.62, 0.9, gNoise(q / 2.2 + 17.0)) * (0.6 + 0.4 * gNoise(q / 0.5));
    c *= 1.0 - 0.18 * oil * (1.0 - smoothstep(0.8, 4.0, fw));
    vec2 pc = floor(q / 9.0);
    float repair = step(gHash(pc + 23.0), 0.10) * gBox(fract(q / 9.0), vec2(0.2, 0.2), vec2(0.8, 0.75));
    c *= 1.0 - 0.12 * repair;
  } else {
    // Lawn: mown bands and patchiness.
    float band = step(0.5, fract(q.x / 6.0));
    c *= 1.0 + 0.05 * (band - 0.5) * (1.0 - smoothstep(1.0, 4.0, fw));
    c *= 0.9 + 0.2 * gNoise(q / 2.4 + 3.0);
  }
  return c;
}
`;function lh(o,l){const r=o.onBeforeCompile;return o.onBeforeCompile=(c,i)=>{r?.call(o,c,i),c.vertexShader=c.vertexShader.replace("#include <common>",`#include <common>
varying vec2 vGroundP;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vGroundP = position.xz;`),c.fragmentShader=c.fragmentShader.replace("#include <common>",`#include <common>
varying vec2 vGroundP;
`+Tp).replace("#include <color_fragment>",`#include <color_fragment>
        diffuseColor.rgb = groundSurface(${l.toFixed(1)}, diffuseColor.rgb, vGroundP, length(fwidth(vGroundP)), 0.0);`)},o.customProgramCacheKey=()=>`ground-${l}`,o}const Ai="京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼",uu="ABCDEFGHJKLMNPQRSTUVWXYZ",Cp="0123456789",Nr=16,_p=5;function X2(){const o={width:64,height:112},l=document.createElement("canvas");l.width=o.width*Nr,l.height=o.height*_p;const r=l.getContext("2d");r.fillStyle="#000",r.fillRect(0,0,l.width,l.height),r.fillStyle="#fff",r.textAlign="center",r.textBaseline="middle",[...Ai,...uu,...Cp].forEach((i,f)=>{const h=f%Nr*o.width+o.width/2,g=Math.floor(f/Nr)*o.height+o.height/2;r.save(),r.translate(h,g+4);const m=f<Ai.length;r.font=`bold ${m?62:104}px "Noto Sans SC", "PingFang SC", "Microsoft YaHei", "DIN Alternate", "Arial Narrow", Arial, sans-serif`,r.scale(m?.98:.8,m?1.5:1),r.fillText(i,0,0),r.restore()});const c=new wu(l);return c.colorSpace=zh,c.anisotropy=8,c}function Z2(){const o=new Ye({color:16777215,roughness:.4,map:X2()});return o.onBeforeCompile=l=>{l.fragmentShader=l.fragmentShader.replace("#include <map_pars_fragment>",`#include <map_pars_fragment>
float plateHash(float n) { return fract(sin(n * 12.9898) * 43758.5453); }
float plateGlyph(float index, vec2 cell) {
  if (cell.x < 0.04 || cell.x > 0.96 || cell.y < 0.02 || cell.y > 0.98) return 0.0;
  float column = mod(index, ${Nr}.0);
  float row = floor(index / ${Nr}.0);
  return texture2D(map, vec2((column + cell.x) / ${Nr}.0, 1.0 - (row + 1.0 - cell.y) / ${_p}.0)).r;
}`).replace("#include <map_fragment>",`
#ifdef USE_COLOR
  vec3 registration = vColor.rgb;
#else
  vec3 registration = vec3(0.37, 0.0, 0.21);
#endif
  // The number as a whole number: an interpolated tint is not exactly equal
  // from pixel to pixel, and a hash of it would be noise.
  float plateSeed = floor(registration.r * 8192.0 + 0.5);
  float plateGreen = step(0.5, registration.g);
  float plateCity = floor(registration.b * 1024.0 + 0.5) / 1024.0;
  vec2 plateAt = vMapUv;
  float plateCount = 7.0 + plateGreen;
  float plateGap = 0.06;
  float plateCell = (1.0 - 0.10 - plateGap) / plateCount;
  float plateX = plateAt.x - 0.05;
  bool plateInGap = plateX > 2.0 * plateCell && plateX < 2.0 * plateCell + plateGap;
  if (plateX > 2.0 * plateCell) plateX -= plateGap;
  float plateSlot = floor(plateX / plateCell);
  vec2 plateIn = vec2(fract(plateX / plateCell), (plateAt.y - 0.17) / 0.66);
  float plateLocal = step(plateHash(plateSeed + 0.31), 0.86);
  float plateProvince = floor(mix(plateHash(plateSeed + 0.57), plateCity, plateLocal) * ${Ai.length}.0);
  float plateLetter = ${Ai.length}.0 + floor(mix(plateHash(plateSeed + 0.73), fract(plateCity * 57.3), plateLocal) * ${uu.length}.0);
  float plateRoll = plateHash(plateSeed + plateSlot * 0.113 + 0.07);
  float plateIndex = plateRoll < 0.3
    ? ${Ai.length}.0 + floor(plateRoll / 0.3 * ${uu.length}.0)
    : ${Ai.length+uu.length}.0 + floor((plateRoll - 0.3) / 0.7 * ${Cp.length}.0);
  if (plateSlot < 0.5) plateIndex = plateProvince;
  else if (plateSlot < 1.5) plateIndex = plateLetter;
  // A new-energy plate's third character says what kind: D electric, F other.
  else if (plateGreen > 0.5 && plateSlot < 2.5) plateIndex = ${Ai.length}.0 + (plateHash(plateSeed + 0.91) < 0.7 ? 3.0 : 5.0);
  float plateInk = (plateInGap || plateSlot < 0.0 || plateSlot >= plateCount) ? 0.0 : plateGlyph(plateIndex, plateIn);
  float plateAspect = 3.2;
  vec2 plateDot = (plateAt - vec2(0.05 + 2.0 * plateCell + plateGap * 0.5, 0.5)) * vec2(plateAspect, 1.0);
  plateInk = max(plateInk, (1.0 - smoothstep(0.035, 0.05, length(plateDot))) * (1.0 - plateGreen));
  vec2 plateEdge = min(plateAt, 1.0 - plateAt) * vec2(plateAspect, 1.0);
  float plateRim = min(plateEdge.x, plateEdge.y);
  float plateLine = smoothstep(0.035, 0.045, plateRim) * (1.0 - smoothstep(0.075, 0.085, plateRim));
  vec3 plateGround = mix(
    vec3(0.010, 0.045, 0.36),
    mix(vec3(0.10, 0.50, 0.22), vec3(0.80, 0.90, 0.82), smoothstep(0.1, 0.9, plateAt.y)),
    plateGreen
  );
  vec3 plateText = mix(vec3(0.92), vec3(0.015), plateGreen);
  diffuseColor.rgb = mix(plateGround, plateText, max(plateInk, plateLine));`).replace("#include <color_fragment>","")},o}function Q2(){const o=(l,r)=>new Ye({color:r,...l});return[["car/body",new Dc({color:16777215,roughness:.42,metalness:.55,clearcoat:1,clearcoatRoughness:.035,envMapIntensity:1})],["car/glass",new Dc({color:1713192,roughness:.03,metalness:0,transparent:!0,opacity:.8,side:ct,depthWrite:!1,envMapIntensity:1.3})],["car/wheel",o({roughness:.9},6974064)],["car/rim",new Dc({color:16777215,roughness:.3,metalness:.85,clearcoat:.6,clearcoatRoughness:.12,envMapIntensity:1.1})],["car/trim",o({roughness:.55},1381911)],["car/chrome",o({roughness:.14,metalness:1,envMapIntensity:1.15},13949148)],["car/lens",new Dc({color:14673898,roughness:.03,metalness:0,transparent:!0,opacity:.16,depthWrite:!1,envMapIntensity:1.6,clearcoat:1,clearcoatRoughness:.02})],["car/lens.red",new Dc({color:9177612,roughness:.05,metalness:0,transparent:!0,opacity:.62,depthWrite:!1,envMapIntensity:1.4,clearcoat:1,clearcoatRoughness:.03})],["car/lamp.front",o({roughness:.25,emissive:15922943,emissiveIntensity:1.6},16054266)],["car/lamp.rear",o({roughness:.25,emissive:16718354,emissiveIntensity:.9},11538444)],["car/lamp.amber",o({roughness:.22,emissive:16742912,emissiveIntensity:.25},14711312)],["car/interior",o({roughness:.95,side:ct},16777215)],["car/plate",Z2()],["car/well",o({roughness:.9,side:ct},789517)],["car/far.body",o({roughness:.3,metalness:.35,envMapIntensity:1},16777215)],["car/far.glass",o({roughness:.12,metalness:.5,envMapIntensity:1.1},1844265)],["car/far.wheel",o({roughness:.92},6974064)]]}const I2=`
varying vec3 vBarkPos;
`,J2=`
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
`,$2=`
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
`;function ey(o){const l=o.onBeforeCompile;o.onBeforeCompile=(c,i)=>{l?.call(o,c,i),c.vertexShader=c.vertexShader.replace("#include <common>",`#include <common>
${I2}`).replace("#include <begin_vertex>",`#include <begin_vertex>
vBarkPos = position;`),c.fragmentShader=c.fragmentShader.replace("#include <common>",`#include <common>
${J2}`).replace("#include <color_fragment>",$2)};const r=o.customProgramCacheKey.bind(o);return o.customProgramCacheKey=()=>`${r()}|bark`,o.needsUpdate=!0,o}const ty=`
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
`,ay={asphalt:`
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
}`};function I0(o,l){const r=l==="grass"?.3:.22;return o.onBeforeCompile=c=>{c.fragmentShader=c.fragmentShader.replace("#include <common>",`#include <common>
${l==="paving"?`#define AT_ALIGNED
`:""}${ty}
${ay[l]}`).replace("#include <map_fragment>",mu.map_fragment.replace("texture2D( map, vMapUv )",`antiTileSample( map, vMapUv, ${r.toFixed(2)} )`).replace("diffuseColor *= sampledDiffuseColor;",`diffuseColor *= sampledDiffuseColor;
	diffuseColor.rgb *= antiTone(vMapUv);`)).replace("#include <normal_fragment_maps>",mu.normal_fragment_maps.split("texture2D( normalMap, vNormalMapUv )").join("antiTileSample( normalMap, vNormalMapUv, 0.0 )"))},o.customProgramCacheKey=()=>`antitile-${l}`,o.needsUpdate=!0,o}const ny=`
uniform float uElevTileW;
float elevHash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}
`,oy=`
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
  // A second, slower noise gives each run of shelves its own cast.
  float shelfHue = elevHash(seed + floor(cellW * 0.25) * 5.3 + gi.y * 1.9);
  vec3 goods = vec3(0.20 + 0.50 * n, 0.19 + 0.47 * n, 0.17 + 0.42 * n)
    * mix(vec3(1.0), 0.75 + 0.5 * vec3(shelfHue, fract(shelfHue * 7.3), fract(shelfHue * 13.7)), 0.45);
  float edgeM = min(fract(cellW), 1.0 - fract(cellW));
  goods *= 1.0 - 0.35 * (1.0 - smoothstep(0.067 - fu, 0.067 + fu, edgeM));
  vec3 backWall = mix(vec3(0.55, 0.52, 0.46), vec3(0.62, 0.62, 0.60), h3);
  float inRows = smoothstep(0.06 - fv, 0.06 + fv, v) * (1.0 - smoothstep(0.74 - fv, 0.74 + fv, v));
  float board = smoothstep(0.16 - fv * 7.0, 0.16 + fv * 7.0, fract(v * 7.0));
  float shelf = inRows * board * step(0.30, h2);
  vec3 inside = mix(backWall * 0.7, goods, shelf);
  inside = mix(inside, vec3(0.85), step(0.92, v) * step(fract(u * widthM / 1.5 + h4), 0.32));
  float lit = step(0.12, h1);
  // Shelving is 45 cm cells: it stays readable until a pixel spans about that,
  // far beyond where a curtain fold has gone. Past it the window settles to
  // the dim average of a shop seen through glass, not to a pale board.
  float shopDetail = 1.0 - smoothstep(0.08, 0.30, mpp);
  inside = mix(vec3(0.27, 0.26, 0.24), inside, shopDetail);
  // Its own light, well under the daylight outside: at 0.42 every shopfront
  // was the brightest surface in the street and read as a blank white board.
  elevEmit = inside * 0.20 * mix(0.88, lit, shopDetail);
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
`,ly=`
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, elevF0, elevGlass);
material.specularColorBlended = mix(material.specularColorBlended, elevF0, elevGlass);
`,sy=`
#include <emissivemap_fragment>
totalEmissiveRadiance += elevEmit * elevGlass;
`,iy=`
#include <roughnessmap_fragment>
roughnessFactor = mix(mix(0.62, 0.90, elevPaint), 0.045, elevGlass);
`,ry=`
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
`;function cy(o,l,r){const c={};c.ELEV_SHOP="",o.defines={...o.defines??{},...c},o.onBeforeCompile=i=>{i.uniforms.uElevTileW={value:r},i.fragmentShader=i.fragmentShader.replace("void main() {",`${ny}
void main() {`).replace("#include <color_fragment>",oy).replace("#include <roughnessmap_fragment>",iy).replace("#include <lights_physical_fragment>",ly).replace("#include <emissivemap_fragment>",sy).replace("#include <normal_fragment_maps>",ry)},o.customProgramCacheKey=()=>`cn-elevation-${l}`,o.needsUpdate=!0}const uy=`
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
`,fy=`
varying vec3 vFacadeWorld;
`,hy=`
#include <worldpos_vertex>
vFacadeWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;
`,dy=`
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
    // Read against photographs of twenty-year-old render: half as strong again as it was.
    float dirt = (0.16 + 0.28 * weather) * rain + (0.24 + 0.30 * weather) * tear + 0.20 * splash;
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
`,my=`
#include <roughnessmap_fragment>
roughnessFactor = fRough;
`,py=`
#include <normal_fragment_maps>
normal = normalize(normal + fGlass * vec3(fTilt, 0.0));
`,vy=`
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, fF0, fGlass);
material.specularColorBlended = mix(material.specularColorBlended, fF0, fGlass);
`,gy=`
#include <emissivemap_fragment>
totalEmissiveRadiance += fEmit;
`,zp=.16;function yy(o){const l=new Ye({color:16777215,roughness:.9,metalness:0,envMapIntensity:1,vertexColors:!0});return l.defines={...l.defines??{},USE_UV:""},l.onBeforeCompile=r=>{r.uniforms.uFacadeTable={value:o},r.uniforms.uInteriorLight={value:zp},r.uniforms.uFacadeFlat={value:new URLSearchParams(window.location.search).get("facade")==="flat"?1:0},r.uniforms.uInteriorMap={value:new URLSearchParams(window.location.search).get("interior")==="0"?0:1},r.vertexShader=r.vertexShader.replace("#include <common>",`#include <common>
${fy}`).replace("#include <worldpos_vertex>",hy),r.fragmentShader=r.fragmentShader.replace("#include <common>",`#include <common>
${uy}`).replace("#include <color_fragment>",dy).replace("#include <roughnessmap_fragment>",my).replace("#include <normal_fragment_maps>",py).replace("#include <lights_physical_fragment>",vy).replace("#include <emissivemap_fragment>",gy)},l.customProgramCacheKey=()=>"cn-facade",l}function xy(o,l,r){const c=new fa(o,l,r,$n);return c.colorSpace=zh,c.magFilter=Ks,c.minFilter=Ks,c.generateMipmaps=!1,c.flipY=!1,c.needsUpdate=!0,c}function k1(o,l,r,c){o.defines={...o.defines??{},USE_UV:""},o.onBeforeCompile=f=>{f.uniforms.uRibPeriod={value:l},f.uniforms.uRibDepth={value:r},f.fragmentShader=f.fragmentShader.replace("#include <common>",`#include <common>
uniform float uRibPeriod;
uniform float uRibDepth;`).replace("#include <normal_fragment_maps>",`#include <normal_fragment_maps>
        {
          float t = vUv[${c}] / uRibPeriod;
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
          vec3 tangent = ${c===0?"dp1 * duv2.y - dp2 * duv1.y":"dp2 * duv1.x - dp1 * duv2.x"};
          float tl = length(tangent);
          if (tl > 1e-6) normal = normalize(normal + (tangent / tl) * tilt);
        }`)};const i=o.customProgramCacheKey.bind(o);o.customProgramCacheKey=()=>`${i()}|ribs${c}`,o.needsUpdate=!0}const wy=`
varying vec3 vWxPos;
varying vec3 vWxNormal;
`,by=`
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
`,My=`
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
`;function Vl(o,l=1){const r=o.onBeforeCompile;o.onBeforeCompile=(i,f)=>{r?.call(o,i,f),i.uniforms.uWxStrength={value:l},i.vertexShader=i.vertexShader.replace("#include <common>",`#include <common>
${wy}`).replace("#include <begin_vertex>",`#include <begin_vertex>
vWxPos = position;
vWxNormal = normal;`),i.fragmentShader=i.fragmentShader.replace("#include <common>",`#include <common>
${by}`).replace("#include <color_fragment>",My).replace("#include <roughnessmap_fragment>",`#include <roughnessmap_fragment>
roughnessFactor = clamp(roughnessFactor - 0.10 * wxRain * uWxStrength, 0.3, 1.0);`)};const c=o.customProgramCacheKey.bind(o);o.customProgramCacheKey=()=>`${c()}|weathered`,o.needsUpdate=!0}const Sy='"Microsoft YaHei","PingFang SC","Noto Sans CJK SC","Source Han Sans SC","WenQuanYi Zen Hei","Noto Sans SC",sans-serif',Ty='"SimHei","Heiti SC","Microsoft YaHei","PingFang SC","Noto Sans CJK SC","WenQuanYi Zen Hei",sans-serif',Cy='"SimSun","Songti SC","Noto Serif CJK SC","Source Han Serif SC","STSong","WenQuanYi Zen Hei",serif',_y='"KaiTi","STKaiti","Kaiti SC","AR PL UKai CN","DFKai-SB","WenQuanYi Zen Hei",serif',zy='"Microsoft YaHei UI","PingFang SC","HarmonyOS Sans SC","Noto Sans CJK SC","WenQuanYi Zen Hei",sans-serif',pu=[[800,Sy,0],[900,Ty,.035],[700,Cy,.02],[400,_y,.045],[700,zy,.02]],sl=o=>`#${o}`;function Ay(o){return{cols:+o[2],rows:+o[3],cw:+o[4],ch:+o[5]}}function fu(o,l){if(typeof document>"u")return null;const r=document.createElement("canvas");return r.width=o,r.height=l,r.getContext("2d")}function Ey(o,l,r,c){return[...l].map(i=>o.measureText(i).width+r*c)}function wh(o,l,r,c,i){const[f,h,g]=pu[i.font%pu.length];o.save(),o.font=`${f} ${i.size}px ${h}`,o.textBaseline="middle",o.textAlign="left";const m=[...l].length,d=i.gap??(m<=2?.28:m===3?.06:0),x=Ey(o,l,d,i.size),M=x.reduce((R,B)=>R+B,0)-d*i.size,T=M>i.maxW?i.maxW/M:1;o.translate(r,c),o.scale(T,1);let A=i.align==="left"?0:-M/2;if(i.shadow){o.fillStyle=i.shadow[2];let R=A;[...l].forEach((B,C)=>{o.fillText(B,R+i.shadow[0],i.shadow[1]),R+=x[C]})}i.glow&&(o.shadowColor=i.glow,o.shadowBlur=i.size*.25),o.lineJoin="round",[...l].forEach((R,B)=>{i.stroke&&i.strokeW&&(o.lineWidth=i.strokeW,o.strokeStyle=i.stroke,o.strokeText(R,A,0)),o.fillStyle=i.fill,o.fillText(R,A,0),g>0&&(o.lineWidth=i.size*g,o.strokeStyle=i.fill,o.strokeText(R,A,0)),A+=x[B]}),o.restore()}function Oc(o,l,r,c,i,f){o.beginPath(),o.moveTo(l+f,r),o.arcTo(l+c,r,l+c,r+i,f),o.arcTo(l+c,r+i,l,r+i,f),o.arcTo(l,r+i,l,r,f),o.arcTo(l,r,l+c,r,f),o.closePath()}function Ry(o,l,r,c,i,f,h,g){o.save(),o.translate(r,c),o.lineCap="round",o.lineJoin="round";const m=(d,x,M,T)=>{o.fillStyle=T,o.beginPath(),o.arc(d,x,M,0,Math.PI*2),o.fill()};switch(l){case 1:{m(0,i*.42,i*.58,f),m(0,-i*.3,i*.42,f),o.fillStyle=g,o.beginPath(),o.moveTo(-i*.36,-i*.55),o.lineTo(-i*.3,-i*.95),o.lineTo(-i*.1,-i*.7),o.lineTo(0,-i),o.lineTo(i*.1,-i*.7),o.lineTo(i*.3,-i*.95),o.lineTo(i*.36,-i*.55),o.closePath(),o.fill(),m(-i*.14,-i*.32,i*.06,h),m(i*.14,-i*.32,i*.06,h),o.fillStyle=h,o.fillRect(-i*.4,i*0,i*.8,i*.1);break}case 2:{o.strokeStyle=f,o.lineWidth=i*.4,o.beginPath(),o.moveTo(-i*.88,i*.85),o.bezierCurveTo(-i*.88,-i*1.1,-i*.02,-i*1.1,-i*.02,i*.3),o.moveTo(i*.88,i*.85),o.bezierCurveTo(i*.88,-i*1.1,i*.02,-i*1.1,i*.02,i*.3),o.stroke();break}case 3:{Oc(o,-i*.85,-i*.85,i*1.7,i*1.7,i*.3),o.fillStyle=f,o.fill(),o.fillStyle=g;for(let d=-2;d<=2;d++)o.fillRect(d*i*.3-i*.07,-i*.5,i*.14,i*1.1);o.fillRect(-i*.6,-i*.7,i*1.2,i*.16);break}case 4:{m(0,0,i,f),m(0,0,i*.84,g),m(0,0,i*.74,f),m(0,0,i*.66,g),o.fillStyle=f,o.beginPath();for(let d=0;d<10;d++){const x=-Math.PI/2+d*Math.PI/5,M=d%2===0?i*.5:i*.2;o.lineTo(Math.cos(x)*M,Math.sin(x)*M)}o.closePath(),o.fill();break}case 5:{m(0,i*.3,i*.62,f),o.strokeStyle=f,o.lineWidth=i*.14;for(const d of[-1,1])o.beginPath(),o.moveTo(d*i*.3,-i*.2),o.lineTo(d*i*.55,-i*.8),o.moveTo(d*i*.45,-i*.5),o.lineTo(d*i*.85,-i*.55),o.stroke();m(-i*.2,i*.25,i*.07,h),m(i*.2,i*.25,i*.07,h);break}case 6:{o.strokeStyle=f,o.lineWidth=i*.17;for(let d=1;d<=3;d++)o.beginPath(),o.arc(-i*.4,i*.5,i*.45*d,-Math.PI*.45,0),o.stroke();m(-i*.4,i*.5,i*.12,f);break}case 7:{o.strokeStyle=f,o.lineWidth=i*.2;for(const d of[-1,1])o.beginPath(),o.arc(d*i*.36,0,i*.52,0,Math.PI*2),o.stroke();break}case 8:{o.strokeStyle=f,o.lineWidth=i*.24,o.beginPath(),o.arc(0,0,i*.72,.35,Math.PI*2-.05),o.stroke(),o.fillStyle=g,o.fillRect(-i*.72,-i*.08,i*1.2,i*.17);break}case 9:{m(0,0,i,f),o.fillStyle=h,o.fillRect(-i*.36,-i*.36,i*.72,i*.72),o.fillStyle=f,o.fillRect(-i*.22,-i*.22,i*.44,i*.44),o.fillStyle=h,o.fillRect(-i*.1,-i*.1,i*.2,i*.2);break}case 10:{m(0,0,i,f),o.fillStyle=h,o.fillRect(-i*.5,-i*.5,i*1,i*.18),o.fillRect(-i*.09,-i*.5,i*.18,i*1),o.fillRect(-i*.5,i*.32,i*1,i*.18);break}case 11:{Oc(o,-i*.9,-i*.9,i*1.8,i*1.8,i*.28),o.fillStyle=f,o.fill(),o.strokeStyle=h,o.lineWidth=i*.16,o.strokeRect(-i*.45,-i*.45,i*.9,i*.9),o.fillStyle=h,o.fillRect(-i*.9,-i*.08,i*1.8,i*.16);break}case 12:{o.fillStyle=f,Oc(o,-i*.3,-i*.9,i*.6,i*1.8,i*.1),o.fill(),Oc(o,-i*.9,-i*.3,i*1.8,i*.6,i*.1),o.fill();break}case 13:{o.fillStyle=f,o.beginPath();for(let d=0;d<6;d++){const x=Math.PI/6+d*Math.PI/3;o.lineTo(Math.cos(x)*i*.95,Math.sin(x)*i*.95)}o.closePath(),o.fill(),o.fillStyle=h,o.fillRect(-i*.7,-i*.1,i*1.4,i*.2);break}case 14:{o.fillStyle=f,o.beginPath(),o.moveTo(-i*.9,i*.8),o.lineTo(0,-i*.85),o.lineTo(i*.9,i*.8),o.lineTo(i*.45,i*.8),o.lineTo(0,-i*.05),o.lineTo(-i*.45,i*.8),o.closePath(),o.fill();break}case 15:{o.fillStyle=f,o.beginPath(),o.moveTo(-i*.95,-i*.1),o.lineTo(-i*.5,-i*.35),o.lineTo(-i*.12,i*.25),o.lineTo(i*.7,-i*.8),o.lineTo(i*.95,-i*.7),o.lineTo(-i*.1,i*.85),o.closePath(),o.fill();break}case 16:{o.fillStyle=f,Oc(o,-i*.5,-i*.1,i*1,i*1,i*.2),o.fill(),o.fillRect(-i*.26,-i*.7,i*.52,i*.7),o.fillStyle=g,o.fillRect(-i*.32,-i*.9,i*.64,i*.2),o.fillStyle=h,o.fillRect(-i*.5,i*.2,i*1,i*.14);break}case 17:{o.fillStyle=f,o.beginPath(),o.arc(0,0,i*.85,0,Math.PI),o.closePath(),o.fill(),o.strokeStyle=f,o.lineWidth=i*.1;for(const d of[-1,0,1])o.beginPath(),o.moveTo(d*i*.4,-i*.1),o.bezierCurveTo(d*i*.4-i*.2,-i*.35,d*i*.4+i*.2,-i*.5,d*i*.4,-i*.8),o.stroke();break}case 18:{o.fillStyle=f,o.beginPath(),o.ellipse(0,i*.1,i*.95,i*.5,0,Math.PI,0),o.fill(),o.fillRect(-i*.9,i*.05,i*1.8,i*.28);break}case 19:{o.fillStyle=f,o.beginPath(),o.moveTo(-i*.8,i*.8),o.bezierCurveTo(-i*.9,-i*.6,i*.2,-i*1,i*.85,-i*.8),o.bezierCurveTo(i*.9,-i*0,i*.3,i*.8,-i*.8,i*.8),o.fill(),o.strokeStyle=h,o.lineWidth=i*.08,o.beginPath(),o.moveTo(-i*.7,i*.7),o.lineTo(i*.5,-i*.5),o.stroke();break}case 22:{o.fillStyle=f,o.beginPath(),o.moveTo(-i*.6,-i*.3),o.lineTo(i*.6,-i*.3),o.lineTo(i*.42,i*.9),o.lineTo(-i*.42,i*.9),o.closePath(),o.fill(),o.fillRect(-i*.7,-i*.5,i*1.4,i*.2),o.strokeStyle=f,o.lineWidth=i*.1,o.beginPath(),o.moveTo(i*.1,-i*.5),o.lineTo(i*.3,-i*1),o.stroke();break}case 23:{o.fillStyle=f,o.beginPath(),o.moveTo(0,-i*.9),o.lineTo(i*.7,0),o.lineTo(0,i*.9),o.lineTo(-i*.7,0),o.closePath(),o.fill(),o.fillStyle=h,o.beginPath(),o.moveTo(0,-i*.45),o.lineTo(i*.35,0),o.lineTo(0,i*.45),o.lineTo(-i*.35,0),o.closePath(),o.fill();break}}o.restore()}function Ny(o,l,r,c,i,f){o.save(),o.translate(l,r),o.beginPath(),o.rect(0,0,c,i),o.clip();const h=sl(f.bg),g=sl(f.fg),m=sl(f.acc);o.fillStyle=h,o.fillRect(0,0,c,i);let d=c/2,x=c*.9,M=i*.5,T=i*.62;f.sub&&(M=i*.42,T=i*.5);const A=i*.84,R=i*.17,B=g;switch(f.style){case 1:{const H=o.createLinearGradient(0,0,0,i);H.addColorStop(0,"rgba(255,255,255,0.16)"),H.addColorStop(.5,"rgba(255,255,255,0.0)"),H.addColorStop(1,"rgba(0,0,0,0.12)"),o.fillStyle=H,o.fillRect(0,0,c,i),o.strokeStyle="rgba(200,205,210,0.9)",o.lineWidth=5,o.strokeRect(2.5,2.5,c-5,i-5),o.strokeStyle=m,o.globalAlpha=.7,o.lineWidth=2,o.strokeRect(9,9,c-18,i-18),o.globalAlpha=1;break}case 2:{o.fillStyle=m,o.globalAlpha=.9,o.fillRect(0,i*.8,c,i*.2),o.globalAlpha=1,T=f.sub?i*.56:i*.72,M=f.sub?i*.42:i*.46;break}case 3:{const H=i*.36,K=i*.62;f.mark,Ry(o,f.mark,K,i*.5,H,g,h,m),d=(K+H+c)/2+2,x=c-(K+H)-i*.28,T=f.sub?i*.46:i*.54,M=f.sub?i*.4:i*.5;break}case 5:{o.fillStyle="#101114",o.fillRect(0,0,c,i),o.strokeStyle=g,o.globalAlpha=.55,o.lineWidth=3,o.strokeRect(6,6,c-12,i-12),o.globalAlpha=1;break}case 6:{const H=f.mark===20?["#f07a1a","#00833e","#e03a2c"]:[sl(f.acc),"#1b8a43",sl(f.acc)],K=i*.075;H.forEach((U,_)=>{o.fillStyle=U,o.fillRect(0,_*K+2,c,K*.8),o.fillRect(0,i-(_+1)*K-2,c,K*.8)}),M=f.sub?i*.46:i*.5,T=f.sub?i*.4:i*.46;break}default:o.strokeStyle=m,o.globalAlpha=.55,o.lineWidth=2,o.strokeRect(6,6,c-12,i-12),o.globalAlpha=1}const C=f.style===2;wh(o,f.text,d,M,{fill:g,size:T,font:f.font,maxW:x,stroke:C?m:void 0,strokeW:C?T*.09:0,shadow:C?[T*.045,T*.06,"rgba(0,0,0,0.55)"]:void 0,glow:f.style===5?g:void 0}),f.sub&&wh(o,f.sub,d,A,{fill:B,size:R,font:0,maxW:x*.95,gap:.04}),o.restore()}function Dy(o,l,r,c,i,f){o.save(),o.translate(l,r),o.beginPath(),o.rect(0,0,c,i),o.clip(),o.fillStyle=sl(f.bg),o.fillRect(0,0,c,i),o.strokeStyle=sl(f.acc),o.globalAlpha=.8,o.lineWidth=3,o.strokeRect(8,8,c-16,i-16),o.globalAlpha=1;const h=[...f.text],g=h.length,m=Math.min(c*.62,(i-44)/g*.92),d=(i-44)/g;h.forEach((x,M)=>{wh(o,x,c/2,22+d*(M+.5),{fill:sl(f.fg),size:m,font:f.font,maxW:c*.8,stroke:f.style===2?sl(f.acc):void 0,strokeW:f.style===2?m*.08:0})}),o.restore()}function ky(o,l,r,c,i){const h=i.mark>0?120:0,g=h+[...i.name].length*100,m=(r-g)/2,d=sl(i.colour);if(o.save(),o.translate(0,l),i.mark>0){const T=m+50,A=c/2;o.fillStyle="rgba(0,0,0,0.55)",o.strokeStyle="rgba(0,0,0,0.55)",o.save(),o.translate(3,4),U1(o,i.mark,T,A,40,"rgba(0,0,0,0.55)"),o.restore(),U1(o,i.mark,T,A,40,d)}const[x,M]=pu[i.font%pu.length];o.font=`${x} 92px ${M}`,o.textBaseline="middle",o.textAlign="center",o.lineJoin="round",[...i.name].forEach((T,A)=>{const R=m+h+100*(A+.5),B=c/2+2;o.fillStyle="rgba(0,0,0,0.5)",o.fillText(T,R+4,B+5),o.lineWidth=6,o.strokeStyle="rgba(20,22,26,0.75)",o.strokeText(T,R,B),o.fillStyle=d,o.fillText(T,R,B),i.style===1&&(o.lineWidth=2,o.strokeStyle="rgba(255,255,255,0.35)",o.strokeText(T,R,B))}),o.restore()}function U1(o,l,r,c,i,f){switch(o.save(),o.translate(r,c),o.fillStyle=f,o.strokeStyle=f,o.lineWidth=i*.28,o.lineJoin="round",l){case 1:o.beginPath(),o.arc(0,0,i*.85,0,Math.PI*2),o.stroke(),o.beginPath(),o.arc(0,0,i*.3,0,Math.PI*2),o.fill();break;case 2:o.beginPath(),o.moveTo(0,-i),o.lineTo(i*.8,0),o.lineTo(0,i),o.lineTo(-i*.8,0),o.closePath(),o.fill();break;case 3:o.beginPath(),o.moveTo(0,-i*.95),o.lineTo(i*.95,i*.8),o.lineTo(-i*.95,i*.8),o.closePath(),o.fill();break;case 4:o.beginPath();for(let h=0;h<6;h++){const g=h*Math.PI/3;o.lineTo(Math.cos(g)*i,Math.sin(g)*i)}o.closePath(),o.stroke();break;case 5:o.beginPath(),o.moveTo(-i,i*.2),o.bezierCurveTo(-i*.5,-i*.9,0,i*.9,i*.5,-i*.2),o.lineTo(i,-i*.5),o.stroke();break;default:o.strokeRect(-i*.8,-i*.8,i*1.6,i*1.6),o.fillRect(-i*.3,-i*.3,i*.6,i*.6)}o.restore()}function Uy(){const l=fu(128,128);if(!l)return null;const r=l.createImageData(128,128),c=[[200,28,36],[244,244,240],[34,74,166],[244,244,240]];for(let f=0;f<128;f++)for(let h=0;h<128;h++){const g=(h+f)%128/128*c.length,m=Math.floor(g)%c.length,d=g-Math.floor(g),x=(m+1)%c.length,M=d>.94?(d-.94)/.06:0,T=(f*128+h)*4;for(let A=0;A<3;A++)r.data[T+A]=c[m][A]*(1-M)+c[x][A]*M;r.data[T+3]=255}l.putImageData(r,0,0);const i=new wu(l.canvas);return i.wrapS=za,i.wrapT=za,i.colorSpace=So,i.flipY=!1,i.anisotropy=4,i}function sh(o){const l=new wu(o.canvas);return l.colorSpace=So,l.flipY=!1,l.generateMipmaps=!0,l.minFilter=al,l.magFilter=la,l.anisotropy=8,l.needsUpdate=!0,l}function Oy(o){if(typeof document>"u")return null;const l=new TextDecoder("utf-8").decode(o).replace(/\0+$/,""),r={},c=[],i=[],f=[];for(const A of l.split(`
`)){if(!A)continue;const R=A.split("	");switch(R[0]){case"SHEET":r[R[1]]=Ay(R);break;case"B":c.push({cell:+R[1],bays:+R[2],style:+R[3],font:+R[4],bg:R[5],fg:R[6],acc:R[7],mark:+R[8],text:R[9],sub:R[10]??""});break;case"V":i.push({cell:+R[1],style:+R[2],font:+R[3],bg:R[4],fg:R[5],acc:R[6],text:R[7]});break;case"C":f.push({cell:+R[1],style:+R[2],font:+R[3],colour:R[4],mark:+R[5],name:R[6]});break}}const h=r.board,g=r.vert,m=r.crown;if(!h||!g||!m)return null;const d=fu(h.cols*h.cw,h.rows*h.ch),x=fu(g.cols*g.cw,g.rows*g.ch),M=fu(m.cw,m.rows*m.ch),T=Uy();if(!d||!x||!M||!T)return null;for(const A of c)Ny(d,A.cell%h.cols*h.cw,Math.floor(A.cell/h.cols)*h.ch,h.cw*A.bays,h.ch,A);for(const A of i)Dy(x,A.cell%g.cols*g.cw,Math.floor(A.cell/g.cols)*g.ch,g.cw,g.ch,A);for(const A of f)ky(M,A.cell*m.ch,m.cw,m.ch,A);return{board:sh(d),vert:sh(x),crown:sh(M),pole:T}}function Il(o){if(typeof o!="string")return new Uint8Array(o.buffer,o.byteOffset,o.byteLength);const r=atob(o),c=new Uint8Array(r.length);for(let i=0;i<r.length;i+=1)c[i]=r.charCodeAt(i);return c}function hu(o,l){if(o instanceof Float32Array){if(o.length!==l)throw new Error(`buffer is ${o.length} floats, expected ${l}`);return o}const r=Il(o);if(r.length!==l*4)throw new Error(`buffer is ${r.length} bytes, expected ${l*4}`);const c=r.slice();return new Float32Array(c.buffer,0,l)}function By(o,l){if(o instanceof Uint32Array){if(o.length!==l)throw new Error(`index buffer is ${o.length} entries, expected ${l}`);return o}const r=Il(o);if(r.length!==l*4)throw new Error(`index buffer is ${r.length} bytes, expected ${l*4}`);const c=r.slice();return new Uint32Array(c.buffer,0,l)}const Hy=[16722456,16757760,1638234],Ly=[2755080,2761224,666134],jy=24,Rr=new Map;function Fy(o){const l=Il(o.data);let r=2166136261;const c=Math.max(1,Math.floor(l.length/257));for(let i=0;i<l.length;i+=c)r=Math.imul(r^l[i],16777619);return`${o.name}|${o.width}x${o.height}|${l.length}|${(r>>>0).toString(36)}`}function Gy(o,l=64){const r=new Map;for(const F of o)r.set(F.name,F);const c=[],i=[],f=[],h=new Map,g=new Map,m=new Set,d=new Set,x=new Map,M=new Map,T=F=>{const ie=g.get(F);if(ie)return ie;const $=r.get(F);if(!$)return null;const N=Fy($),P=Rr.get(N);if(P)return P.refs+=1,f.push(N),h.set(F,N),g.set(F,P.texture),P.texture;const k=new fa(Il($.data),$.width,$.height,$n);k.wrapS=za,k.wrapT=za,k.anisotropy=Jl.anisotropy,k.magFilter=la,k.minFilter=al,k.generateMipmaps=!0,k.colorSpace=So,k.flipY=!1;const ee=F.startsWith("facade/")||F.startsWith("cnshop/");return k.repeat.set(1/$.tileWidthM,ee?1:1/$.tileHeightM),k.needsUpdate=!0,g.set(F,k),Rr.set(N,{texture:k,refs:1}),f.push(N),h.set(F,N),k},A=[],R=(F,ie)=>{const $=r.get(F);if(!$?.hasNormalSource)return null;const N=`${F}#normal`,P=g.get(N);if(P)return P;const k=T(F);if(!k)return null;const ee=`${h.get(F)}#normal#${ie}`,Me=Rr.get(ee);if(Me)return Me.refs+=1,f.push(ee),g.set(N,Me.texture),Me.texture;const{width:S,height:Q}=$,ve=new Uint8Array(S*Q*4);for(let Ce=0;Ce<ve.length;Ce+=4)ve[Ce]=128,ve[Ce+1]=128,ve[Ce+2]=255,ve[Ce+3]=255;const V=new fa(ve,S,Q,$n);V.wrapS=za,V.wrapT=za,V.flipY=!1,V.anisotropy=Jl.anisotropy,V.minFilter=al,V.generateMipmaps=!0,V.repeat.copy(k.repeat),V.needsUpdate=!0,g.set(N,V),Rr.set(ee,{texture:V,refs:1}),f.push(ee);let pe=null;A.push(()=>{const Ce=k.image.data;let Ne=255,oe=0;for(let _e=3;_e<Ce.length;_e+=4)Ne=Math.min(Ne,Ce[_e]),oe=Math.max(oe,Ce[_e]);const J=oe-Ne>8;pe=new Float32Array(S*Q);for(let _e=0;_e<pe.length;_e+=1){const Je=_e*4;pe[_e]=J?Ce[Je+3]/255:(Ce[Je]*.2126+Ce[Je+1]*.7152+Ce[Je+2]*.0722)/255}});const ue=4;for(let Ce=0;Ce<ue;Ce+=1)A.push(()=>{const Ne=pe;for(let oe=Math.floor(Q*Ce/ue);oe<Math.floor(Q*(Ce+1)/ue);oe+=1){const J=(oe+Q-1)%Q*S,_e=oe*S,Je=(oe+1)%Q*S;for(let O=0;O<S;O+=1){const Pe=(O+S-1)%S,Ie=(O+1)%S,tt=(Ne[_e+Pe]-Ne[_e+Ie])*ie,Nt=(Ne[J+O]-Ne[Je+O])*ie,Ft=1/Math.sqrt(tt*tt+Nt*Nt+1),$t=(_e+O)*4;ve[$t]=(tt*Ft*.5+.5)*255,ve[$t+1]=(Nt*Ft*.5+.5)*255,ve[$t+2]=(Ft*.5+.5)*255,ve[$t+3]=255}}Ce===ue-1&&(V.needsUpdate=!0,pe=null)});return V},B=(F,ie)=>(x.set(F,ie),c.push(ie),ie),C=(F,ie,$)=>B(F,new Ye({color:$,...ie})),H=T("ground/asphalt");I0(C("asphalt",{map:H??void 0,normalMap:R("ground/asphalt",2.4)??void 0,roughness:.88,metalness:0},H?16777215:6578782),"asphalt"),C("asphalt.pavement",{roughness:.95},7302246);const K=T("ground/paving");I0(C("sidewalk",{map:K??void 0,normalMap:R("ground/paving",1.6)??void 0,roughness:.9},K?16777215:7237224),"paving"),I0(C("parcel.paving",{map:K??void 0,roughness:.92},16777215),"paving"),lh(C("parcel.yard",{roughness:.93},9209724),nl.yard),lh(C("parcel.lot",{roughness:.9},6447196),nl.asphaltLot),lh(C("parcel.parking",{roughness:.9},7302505),nl.carParkBase),C("kerb",{roughness:.86,polygonOffset:!0,polygonOffsetFactor:0,polygonOffsetUnits:-2},9341826),C("tactile.guide",{map:T("tactile/guide")??void 0,normalMap:R("tactile/guide",2)??void 0,roughness:.8},16777215),C("tactile.warn",{map:T("tactile/warn")??void 0,normalMap:R("tactile/warn",2)??void 0,roughness:.8,polygonOffset:!0,polygonOffsetFactor:0,polygonOffsetUnits:-2},16777215);const U={polygonOffset:!0,polygonOffsetFactor:0,polygonOffsetUnits:-2};C("treepit.edge",{roughness:.78,...U},10591894),C("treepit.grate",{roughness:.6,metalness:.25,...U},4539967),C("treepit.gravel",{roughness:1,...U},12170409),C("treepit.planter",{roughness:.35,metalness:.05},4935248);const _=T("ground/grass");for(const F of["block.ground","parcel.green","median.plant"])x2(I0(C(F,{map:_??void 0,normalMap:R("ground/grass",1.2)??void 0,roughness:.95,...F==="block.ground"?{polygonOffset:!0,polygonOffsetFactor:2,polygonOffsetUnits:2}:{}},_?16777215:3029538),"grass"));{const F=C("water",{roughness:.2,metalness:0,envMapIntensity:.45,vertexColors:!0},16777215),ie={value:0};F.userData.uTime=ie,F.onBeforeCompile=$=>{$.uniforms.uWaterTime=ie,$.vertexShader=$.vertexShader.replace("#include <common>",`#include <common>
varying vec2 vWaterP;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vWaterP = position.xz;`),$.fragmentShader=$.fragmentShader.replace("#include <common>",`#include <common>
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
}`)}}Vl(C("bank.earth",{roughness:.96,side:ct,vertexColors:!0},16777215),.5);const I=T("foliage/hedge");for(const F of["hedge","hedge.clipped"])C(F,{map:I??void 0,normalMap:R("foliage/hedge",2.6)??void 0,roughness:.88,flatShading:!1},I?16777215:2771487);for(const F of["red","pink","yellow"]){const ie=T(`flowers/${F}`);C(`flowers/${F}`,{map:ie??void 0,normalMap:R(`flowers/${F}`,1.8)??void 0,roughness:.8},ie?16777215:9054752)}C("barrier.concrete",{roughness:.9},10131598),C("rail.steel",{roughness:.42,metalness:.75,envMapIntensity:.9},9409688),C("bridge.concrete",{roughness:.9},8157810),Vl(C("viaduct.concrete",{roughness:.86,vertexColors:!0},16777215),.7),C("viaduct.glass",{roughness:.12,metalness:.1,transparent:!0,opacity:.42,depthWrite:!1,side:ct,envMapIntensity:1.2},7317922),C("viaduct.planter",{roughness:.9,vertexColors:!0},16777215),C("bridge.stone",{roughness:.88,side:ct,vertexColors:!0},11840672),C("quay.stone",{map:K??void 0,normalMap:R("ground/paving",1.4)??void 0,roughness:.9,side:ct,vertexColors:!0},K?15920354:12893360);{const F=new Ye({color:15660274,roughness:1,metalness:0,transparent:!0,opacity:.2,depthWrite:!1,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2});B("water.foam",F)}C("bridge.steel",{roughness:.5,metalness:.6,side:ct,vertexColors:!0},10133670),C("wire",{roughness:.55,metalness:.3,side:ct},1316378),C("rail.ballast",{roughness:1},8091503),C("rail.sleeper",{roughness:.9},10197135),C("rail.platform",{roughness:.88,side:ct},11841960),C("station.wall",{roughness:.85},13486527),C("station.roof",{roughness:.6,metalness:.2,side:ct},13883096),C("station.glass",{roughness:.1,metalness:.1,envMapIntensity:1.2},5666956),C("train.body",{roughness:.32,metalness:.3,side:ct,vertexColors:!0},16777215),C("train.glass",{roughness:.08,metalness:.1,side:ct,envMapIntensity:1.3},1384231),C("train.stripe",{roughness:.4,side:ct},2051994);const Z=(F,ie,$,N=!1)=>B(F,new Ye({color:$?16777215:ie,map:$??void 0,alphaTest:N?.5:0,transparent:!1,roughness:.45,metalness:0,emissive:new Zt($?0:ie).multiplyScalar(.06),polygonOffset:!0,polygonOffsetFactor:-4,polygonOffsetUnits:-4,envMapIntensity:.5}));Z("marking.white",15263452,null),Z("marking.yellow",14197788,null);for(const F of["marking.crosswalk","marking.dashed-3-5","marking.dashed-6-9"])Z(F,15263452,T(F.replace(".","/")),!0);const le=T("roof");C("roof",{map:le??void 0,normalMap:R("roof",1.4)??void 0,roughness:.94},le?16777215:6973538);for(const F of["wheat","rice","rapeseed","corn","vegetables","fallow","orchard"])for(let ie=0;ie<3;ie+=1){const $=`field/${F}.${ie}`;M.set($,()=>{const N=T($);for(C($,{map:N??void 0,normalMap:R($,1.6)??void 0,roughness:.96},N?16777215:7039546);A.length>0;)A.shift()()})}C("field.ridge",{roughness:1},7035455);for(const[F,ie,$]of[["roof.tile",.82,0],["roof.terracotta",.78,0],["roof.steel",.45,.35]]){const N=T(F);C(F,{map:N??void 0,normalMap:R(F,2)??void 0,roughness:ie,metalness:$},N?16777215:5917252)}for(let F=0;F<jy;F+=1){const ie=`facade/${F.toString().padStart(2,"0")}`,$=T(ie),N=F>=8&&F<=15;if(C(ie,{map:$??void 0,roughness:N?.28:.85,metalness:N?.1:0,envMapIntensity:N?1.25:.5,vertexColors:!0},$?16777215:11052188),N||Vl(x.get(ie)),N){const P=x.get(ie);P.onBeforeCompile=k=>{k.vertexShader=k.vertexShader.replace("#include <common>",`#include <common>
varying vec3 vGlassW;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vGlassW = (modelMatrix * vec4(transformed, 1.0)).xyz;`),k.fragmentShader=k.fragmentShader.replace("#include <common>",`#include <common>
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
}`)},P.needsUpdate=!0}}const X=r.get("cnfacade/table");if(X){const F=xy(Il(X.data),X.width,X.height);i.push(F),B("cnfacade",yy(F))}for(const F of o){if(!F.name.startsWith("cnshop/"))continue;const ie=C(F.name,{map:T(F.name)??void 0,roughness:.85,metalness:0,vertexColors:!0},16777215);cy(ie,"shop",F.tileWidthM)}Vl(C("cnres.paint",{roughness:.9,vertexColors:!0},16777215)),Vl(C("cnres.trim",{roughness:.6},14342869),.6),C("cnres.glass",{roughness:.06,metalness:0,envMapIntensity:1.4},11123910),C("solar.tubes",{roughness:.18,metalness:.2,envMapIntensity:1},1712944);for(const F of["shop","lobby","home"])C(`ground/${F}`,{map:T(`ground/${F}`)??void 0,roughness:.72},16777215);B("sign/shop",new Ye({map:T("sign/shop")??void 0,roughness:.4,emissiveMap:T("sign/shop")??void 0,emissive:16777215,emissiveIntensity:.55}));{const F=r.get("sign/manifest"),ie=F?Oy(Il(F.data)):null;if(ie){i.push(ie.board,ie.vert,ie.crown,ie.pole),B("sign/board",new Ye({map:ie.board,roughness:.45,emissiveMap:ie.board,emissive:16777215,emissiveIntensity:.22})),B("sign/vert",new Ye({map:ie.vert,roughness:.5,emissiveMap:ie.vert,emissive:16777215,emissiveIntensity:.22,side:ct})),B("sign/crown",new Ye({map:ie.crown,alphaTest:.4,roughness:.35,metalness:.3,emissiveMap:ie.crown,emissive:16777215,emissiveIntensity:.3}));const $=B("sign/pole",new Ye({map:ie.pole,roughness:.25,emissiveMap:ie.pole,emissive:16777215,emissiveIntensity:.35}));$.userData.poleMap=ie.pole}else for(const $ of["sign/board","sign/vert","sign/crown","sign/pole"])C($,{roughness:.6},13617856)}Vl(C("trim.light",{roughness:.72},9341052),.6),C("trim.dark",{roughness:.45,metalness:.2},2764596),C("balcony.slab",{roughness:.82},8288880),Vl(C("metal.ac",{roughness:.5,metalness:.05},12105390),.5),C("awning",{roughness:.55},1844780),Vl(C("wall.render",{roughness:.88},10262154));const se=C("metal.clad",{roughness:.42,metalness:.3,vertexColors:!0},16777215);k1(se,.25,.55,0),Vl(se,.5),k1(C("metal.door",{roughness:.5,metalness:.35},9409430),.11,.6,1),C("roof.skylight",{roughness:.3,metalness:0,emissive:2764592},13226966),C("pole.concrete",{roughness:.9},8420468),C("steel",{roughness:.38,metalness:.72,envMapIntensity:.9},10133670),C("furniture/lamp",{roughness:.55,metalness:.35},7633276),C("furniture/lamp.double",{roughness:.55,metalness:.35},7633276),C("furniture/lamp.deco",{roughness:.42,metalness:.55},2896179),C("furniture/lamp.globe",{roughness:.2,emissive:16773592,emissiveIntensity:.12},15855592),C("furniture/lantern",{roughness:.65,emissive:3802628,emissiveIntensity:.4},11804184),C("furniture/pole",{roughness:.9},8420468),C("furniture/bollard",{roughness:.5,metalness:.2},12106934),C("furniture/railing",{roughness:.45,metalness:.5},11054512),C("furniture/shelter",{roughness:.55,metalness:.25},7238778),C("furniture/sign.guide",{roughness:.45},1002646),C("furniture/sign.crossing",{roughness:.45},1002646),C("furniture/sign.roundabout",{roughness:.45},1002646),C("furniture/sign.glyph",{roughness:.4},15921902),C("signal.body",{roughness:.55,metalness:.35},1711649),C("asphalt.cycle",{roughness:.9,polygonOffset:!0,polygonOffsetFactor:0,polygonOffsetUnits:-2},7104868),C("furniture/bin",{roughness:.6,metalness:.05},10133666),C("furniture/kiosk",{roughness:.5,metalness:.2},9080982),C("furniture/gantry.steel",{roughness:.42,metalness:.7,envMapIntensity:.9},9607328),C("furniture/gantry.board",{roughness:.4,metalness:.1},1002646);for(const[F,ie]of Q2())B(F,ie);ey(C("bark",{roughness:.95,vertexColors:!0},16777215));const we=(F,ie)=>{const $=T(ie);return B(F,new Ye({map:$??void 0,alphaTest:$?.42:0,side:ct,vertexColors:!0,roughness:.86,metalness:0,emissive:1451023}))};we("leaf","vegetation/leaf");const ge=T("vegetation/tuft");B("tuft",new Ye({map:ge??void 0,alphaTest:ge?.45:0,side:ct,vertexColors:!0,roughness:.92}));const G=[];for(let F=0;F<3;F+=1){const ie=new Gs(.17,10,8),$=new Ye({color:Ly[F],emissive:Hy[F],emissiveIntensity:0,roughness:.25});c.push($);const N=new Va(ie,$,Math.max(1,l));N.count=0,N.frustumCulled=!1,G.push(N)}return{get(F){const ie=M.get(F);ie&&!x.has(F)&&(M.delete(F),ie());const $=x.get(F);if($)return $;const N=F.indexOf("~");if(N>=0)return this.get(F.slice(0,N)+F.slice(N).replace(/^~\d+/,""));const P=F.indexOf("#");if(P>0){const ee=F.slice(0,P).split("/"),Me=F.slice(P+1),S=ee.length>=2?ee[1]:"";if(Me==="leaf"&&T(`vegetation/leaf/${S}`))return we(F,`vegetation/leaf/${S}`);if(Me==="mass")return B(F,new Ye({color:16777215,roughness:1,metalness:0,vertexColors:!0,emissive:1451023,emissiveIntensity:.5,alphaHash:!0,opacity:1}));if(Me==="bark")return B(F,new Ye({color:16777215,roughness:.95,metalness:0,flatShading:!0,vertexColors:!0}))}m.add(F);const k=new Dn({color:16711935});return c.push(k),x.set(F,k),k},textures:i,lamps:G,missing:m,uvMismatch:d,pending:A,dispose(){for(const F of c)F.dispose();for(const F of i)F.dispose();for(const F of f){const ie=Rr.get(F);ie&&(ie.refs-=1,ie.refs<=0&&(ie.texture.dispose(),Rr.delete(F)))}f.length=0;for(const F of G)F.geometry.dispose()}}}function qy(o){const l=o.find(c=>c.name.startsWith("vegetation/leaf"));if(!l)return null;const r=new fa(Il(l.data),l.width,l.height,$n);return r.magFilter=la,r.minFilter=la,r.generateMipmaps=!1,r.needsUpdate=!0,r}function Vy(o){if(typeof o!="string")return o.byteLength;const l=o.endsWith("==")?2:o.endsWith("=")?1:0;return Math.floor(o.length*3/4)-l}function*Ky(o,l){const r=hu(o.positions,o.vertexCount*3);yield;let c;if(o.normalsSnorm){const g=Il(o.normals),m=new Int8Array(g.buffer,g.byteOffset,o.vertexCount*3);c=new Float32Array(o.vertexCount*3);for(let d=0;d<c.length;d+=1)c[d]=Math.max(-1,m[d]/127)}else c=hu(o.normals,o.vertexCount*3);yield;const i=o.colors?Il(o.colors):null;let f=null;if(o.uvs){const g=o.vertexCount*2,m=Vy(o.uvs);Math.abs(m-g*4)>2?l.add(`${o.material}: ${o.vertexCount} verts need ${g*4} B of UV, got ${m} B`):f=hu(o.uvs,g),yield}const h=By(o.indices,o.triangleCount*3);return yield,{positions:r,normals:c,uvs:f,colors:i,indices:h}}function O1(o,l){const r=new It;return r.setAttribute("position",new Jt(o.positions,3)),l.alphaCutout&&(l.material==="tuft"||l.material.startsWith("leaf"))&&r.setAttribute("aLeafCard",new Jt(new Float32Array(o.positions.length/3).fill(1),1)),r.setAttribute("normal",new Jt(o.normals,3)),o.colors&&r.setAttribute("color",new Jt(o.colors,4,!0)),o.uvs&&r.setAttribute("uv",new Jt(o.uvs,2)),r.setIndex(new Jt(o.indices,1)),r.computeBoundingSphere(),r.computeBoundingBox(),r}function*Py(o,l){const{positions:r,indices:c}=o,i=c.length/3,f=new Map;for(let d=0;d<i;d+=1){const x=c[d*3]*3,M=c[d*3+1]*3,T=c[d*3+2]*3,A=Math.floor((r[x]+r[M]+r[T])/(3*l)),R=Math.floor((r[x+2]+r[M+2]+r[T+2])/(3*l)),B=(A+2048)*4096+(R+2048);let C=f.get(B);C||(C=[],f.set(B,C)),C.push(d),(d&65535)===65535&&(yield)}if(yield,f.size<=1)return[o];const h=r.length/3,g=new Int32Array(h).fill(-1),m=[];for(const d of f.values()){const x=[],M=new Uint32Array(d.length*3);let T=0;for(const K of d)for(let U=0;U<3;U+=1){const _=c[K*3+U];let I=g[_];I<0&&(I=x.length,g[_]=I,x.push(_)),M[T]=I,T+=1}const A=x.length,R=new Float32Array(A*3),B=new Float32Array(A*3),C=o.uvs?new Float32Array(A*2):null,H=o.colors?new Uint8Array(A*4):null;for(let K=0;K<A;K+=1){const U=x[K];R[K*3]=r[U*3],R[K*3+1]=r[U*3+1],R[K*3+2]=r[U*3+2],B[K*3]=o.normals[U*3],B[K*3+1]=o.normals[U*3+1],B[K*3+2]=o.normals[U*3+2],C&&o.uvs&&(C[K*2]=o.uvs[U*2],C[K*2+1]=o.uvs[U*2+1]),H&&o.colors&&(H[K*4]=o.colors[U*4],H[K*4+1]=o.colors[U*4+1],H[K*4+2]=o.colors[U*4+2],H[K*4+3]=o.colors[U*4+3])}for(let K=0;K<A;K+=1)g[x[K]]=-1;m.push({positions:R,normals:B,uvs:C,colors:H,indices:M}),yield}return m}function Yy(o,l){const r=o.boundingBox??new _h().setFromBufferAttribute(o.attributes.position),c=r.getCenter(new lt),i=r.getSize(new lt),f=new Jv(.5,1);f.scale(i.x,i.y,i.z),f.translate(c.x,c.y,c.z);const h=f.attributes.position.count;if(l.uvs&&f.setAttribute("uv",new Jt(new Float32Array(h*2),2)),l.colors){let g=0,m=0,d=0;const x=l.colors.length/4,M=Math.max(1,Math.floor(x/512));let T=0;for(let B=0;B<x;B+=M)g+=l.colors[B*4],m+=l.colors[B*4+1],d+=l.colors[B*4+2],T+=1;const A=[Math.round(g/T),Math.round(m/T),Math.round(d/T)],R=new Uint8Array(h*4);for(let B=0;B<h;B+=1)R[B*4]=A[0],R[B*4+1]=A[1],R[B*4+2]=A[2],R[B*4+3]=255;f.setAttribute("color",new Jt(R,4,!0))}return f.computeBoundingSphere(),f.computeBoundingBox(),f}const Wy=new Set(["water","bank.earth","quay.stone"]),Ap=-20;let B1=null;function Xy(){return B1??=new Pa({vertexShader:"void main() { gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0); gl_Position.z = gl_Position.w; }",fragmentShader:"void main() { gl_FragColor = vec4(0.0); }",colorWrite:!1,depthWrite:!0,depthTest:!0,depthFunc:$v,side:ct}),B1}function Zy(o,l,r={}){const c=new _a;c.name="city";const i=[],f=[],h=new Map,g=new Map,m=[],d=[],x=r.chunkCellM??0,M=r.chunkMinTriangles??6e3;let T,A,R=!1,B=!1,C=0;const H=()=>{for(const I of f)I.dispose();for(const I of i)I.dispose();c.clear()},K={group:c,materials:l,instanced:h,prototypes:g,lamps:l.lamps,vehicleCount:o.traffic.agents.length,statics:m,lodSets:d,dispose:H};function*U(){const I=new Map;for(const ge of o.instances)I.set(ge.key,{count:ge.count,floats:hu(ge.data,ge.count*10)}),yield;for(;l.pending.length>0;)l.pending.shift()(),yield;const Z=new rp,le=new Zt;for(const ge of o.meshes){if(C+=1,!ge.vertexCount||!ge.triangleCount)continue;const G=yield*Ky(ge,l.uvMismatch),F=l.get(ge.material),ie=ge.instanceOf;if(ie){const N=I.get(ie);if(!N){l.missing.add(`instance-list:${ie}`);continue}const P=O1(G,ge),k=new Va(P,F,Math.max(1,N.count));k.name=`${ie}`,k.count=N.count,k.castShadow=ge.castShadow,k.receiveShadow=ge.receiveShadow;const ee=ge.material.includes("#")?ge.material.split("#")[1]:"";k.userData.part=ee;const Me=!!F.vertexColors||ie.startsWith("car/"),S=N.floats,ve=r.lod===!0?new Float32Array(N.count*4):null,V=P.boundingSphere,pe=4096;for(let ue=0;ue<N.count;ue+=pe){const Ce=Math.min(N.count,ue+pe);for(let Ne=ue;Ne<Ce;Ne+=1){const oe=Ne*10;if(Z.position.set(S[oe],S[oe+1],S[oe+2]),Z.rotation.set(0,S[oe+3],0),Z.scale.set(S[oe+4],S[oe+5],S[oe+6]),Z.updateMatrix(),k.setMatrixAt(Ne,Z.matrix),Me&&(le.setRGB(S[oe+7],S[oe+8],S[oe+9]),k.setColorAt(Ne,le)),ve){const J=Math.max(S[oe+4],S[oe+5],S[oe+6]);ve[Ne*4]=S[oe]+V.center.x*S[oe+4],ve[Ne*4+1]=S[oe+1]+V.center.y*S[oe+5],ve[Ne*4+2]=S[oe+2]+V.center.z*S[oe+6],ve[Ne*4+3]=V.radius*J}}Ce<N.count&&(yield)}if(k.instanceMatrix.needsUpdate=!0,k.instanceColor&&(k.instanceColor.needsUpdate=!0),k.computeBoundingSphere(),k.computeBoundingBox(),h.set(ie,k),g.set(ie,{geometry:P,material:F}),f.push(k),c.add(k),ve){let ue=null;if(ee==="mass"){const Ce=Yy(P,G);i.push(Ce),ue=new Va(Ce,F,Math.max(1,N.count)),ue.name=`${ie}#far`,ue.count=0,ue.castShadow=!1,ue.receiveShadow=ge.receiveShadow,ue.visible=!1,f.push(ue),c.add(ue)}d.push({mesh:k,key:ie,part:ee,count:N.count,matrices:k.instanceMatrix.array.slice(0,N.count*16),colors:k.instanceColor?k.instanceColor.array.slice(0,N.count*3):null,bounds:ve,far:ue})}yield;continue}const $=x>0&&ge.triangleCount>=M?yield*Py(G,x):[G];for(const N of $){const P=O1(N,ge),k=new pt(P,F);if(k.name=ge.material,k.castShadow=ge.castShadow,k.receiveShadow=ge.receiveShadow,i.push(P),m.push(k),c.add(k),Wy.has(ge.material)){const ee=new pt(P,Xy());ee.name="river.mask",ee.renderOrder=Ap+5,ee.frustumCulled=k.frustumCulled,c.add(ee)}yield}}const X=h.get("car/body"),se=h.get("car/glass");T=X?X.geometry.clone():void 0,A=se?se.geometry.clone():void 0,T&&i.push(T),A&&i.push(A),K.carBody=T,K.carGlass=A;const we=h.get("car/wheel");we&&(K.carWheel=we.geometry.clone(),i.push(K.carWheel))}const _=U();return{handles:K,get done(){return R},get progress(){return R?1:Math.min(.99,C/Math.max(1,o.meshes.length))},step(I){if(R||B)return R;const Z=performance.now()+I;for(;;){if(_.next().done)return R=!0,!0;if(performance.now()>=Z)return!1}},cancel(){B||R||(B=!0,_.return(void 0),H())}}}q2();function Ep(o){let l=0,r=null,c=null,i=null,f=!1,h=0;return{get layer(){return i},get done(){return i!==null},get progress(){return i?1:l===0?0:.05+.9*(c?.progress??0)},step(g){if(i||f)return i!==null;const m=performance.now(),d=m+g;if(l===0){let M=0;for(const T of o.signals)M+=T.lamps.length;r=Gy(o.textures,M),qa("cityMaterialsMs",performance.now()-m),l=1,c=Zy(o,r,{chunkCellM:320,chunkMinTriangles:8e3,lod:!0})}l===1&&c&&c.step(Math.max(0,d-performance.now()))&&(l=2),l===2&&c&&r&&(i=Qy(o,r,c.handles));const x=performance.now()-m;return h+=x,qa("cityBuildSliceMaxMs",x),qa("cityBuildCpuMs",h),i!==null},cancel(){f||i||(f=!0,c?.cancel(),r?.dispose())}}}function Qy(o,l,r){const c=[],i=new _a;i.name="city-layer",i.add(r.group);const f=new rp,h=[],g=[[],[],[]];for(const H of o.signals)for(const K of H.lamps)(g[K.aspect]??g[0]).push(...K.position);r.lamps.forEach((H,K)=>{const U=g[K]??[],_=U.length/3;_>H.instanceMatrix.count&&c.push(`lamps of aspect ${K} exceed capacity`),H.count=Math.min(_,H.instanceMatrix.count);for(let Z=0;Z<H.count;Z+=1)f.position.set(U[Z*3],U[Z*3+1],U[Z*3+2]),f.rotation.set(0,0,0),f.updateMatrix(),H.setMatrixAt(Z,f.matrix);H.instanceMatrix.needsUpdate=!0;const I=H.material;I.emissiveIntensity=K===3?0:3.2,i.add(H)});const m=W2(r,o);m&&i.add(m.group);const d=r.statics.filter(H=>/^(parcel\.green|block\.ground)$/.test(H.name)),x=Jl.grassShells&&d.length>0?new M2(d,l.get("parcel.green")):null,M=new d2(r),T=new Co,A=new Co,R=new eg,B=new lt,C=new lt;return{group:i,scene:o,problems:c,update(H,K=0,U){const _=l.get("water").userData.uTime;_&&(_.value=H);const I=l.get("sign/pole").userData?.poleMap;if(I&&(I.offset.y=-(H/3%1)),!U)return!1;U.updateMatrixWorld(),T.copy(i.matrixWorld).invert(),B.setFromMatrixPosition(U.matrixWorld).applyMatrix4(T),U.getWorldDirection(C).transformDirection(T),A.copy(U.matrixWorld).invert(),A.premultiply(U.projectionMatrix).multiply(i.matrixWorld),R.setFromProjectionMatrix(A);let Z=M.update(B,C,R,K);return x?.update(B),m&&(m.setCasting(K<Ka.shadowTrees),Z=m.update(B,6)||Z),Z},lodStats(){return{hiddenStatics:M.hiddenStatics,instances:M.keptInstances}},dispose(){i.removeFromParent(),m?.dispose(),x?.dispose();for(const H of h)H.dispose();r.dispose(),l.dispose()}}}const Ei={name:"CopyShader",uniforms:{tDiffuse:{value:null},opacity:{value:1}},vertexShader:`

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


		}`};class Ni{constructor(){this.isPass=!0,this.enabled=!0,this.needsSwap=!0,this.clear=!1,this.renderToScreen=!1}setSize(){}render(){console.error("THREE.Pass: .render() must be implemented in derived pass.")}dispose(){}}const Iy=new vh(-1,1,1,-1,0,1);class Jy extends It{constructor(){super(),this.setAttribute("position",new ft([-1,3,0,-1,-1,0,3,-1,0],3)),this.setAttribute("uv",new ft([0,2,0,0,2,0],2))}}const $y=new Jy;class bu{constructor(l){this._mesh=new pt($y,l)}dispose(){this._mesh.geometry.dispose()}render(l){l.render(this._mesh,Iy)}get material(){return this._mesh.material}set material(l){this._mesh.material=l}}class kh extends Ni{constructor(l,r="tDiffuse"){super(),this.textureID=r,this.uniforms=null,this.material=null,l instanceof Pa?(this.uniforms=l.uniforms,this.material=l):l&&(this.uniforms=ll.clone(l.uniforms),this.material=new Pa({name:l.name!==void 0?l.name:"unspecified",defines:Object.assign({},l.defines),uniforms:this.uniforms,vertexShader:l.vertexShader,fragmentShader:l.fragmentShader})),this._fsQuad=new bu(this.material)}render(l,r,c){this.uniforms[this.textureID]&&(this.uniforms[this.textureID].value=c.texture),this._fsQuad.material=this.material,this.renderToScreen?(l.setRenderTarget(null),this._fsQuad.render(l)):(l.setRenderTarget(r),this.clear&&l.clear(l.autoClearColor,l.autoClearDepth,l.autoClearStencil),this._fsQuad.render(l))}dispose(){this.material.dispose(),this._fsQuad.dispose()}}class H1 extends Ni{constructor(l,r){super(),this.scene=l,this.camera=r,this.clear=!0,this.needsSwap=!1,this.inverse=!1}render(l,r,c){const i=l.getContext(),f=l.state;f.buffers.color.setMask(!1),f.buffers.depth.setMask(!1),f.buffers.color.setLocked(!0),f.buffers.depth.setLocked(!0);let h,g;this.inverse?(h=0,g=1):(h=1,g=0),f.buffers.stencil.setTest(!0),f.buffers.stencil.setOp(i.REPLACE,i.REPLACE,i.REPLACE),f.buffers.stencil.setFunc(i.ALWAYS,h,4294967295),f.buffers.stencil.setClear(g),f.buffers.stencil.setLocked(!0),l.setRenderTarget(c),this.clear&&l.clear(),l.render(this.scene,this.camera),l.setRenderTarget(r),this.clear&&l.clear(),l.render(this.scene,this.camera),f.buffers.color.setLocked(!1),f.buffers.depth.setLocked(!1),f.buffers.color.setMask(!0),f.buffers.depth.setMask(!0),f.buffers.stencil.setLocked(!1),f.buffers.stencil.setFunc(i.EQUAL,1,4294967295),f.buffers.stencil.setOp(i.KEEP,i.KEEP,i.KEEP),f.buffers.stencil.setLocked(!0)}}class e3 extends Ni{constructor(){super(),this.needsSwap=!1}render(l){l.state.buffers.stencil.setLocked(!1),l.state.buffers.stencil.setTest(!1)}}class t3{constructor(l,r){if(this.renderer=l,this._pixelRatio=l.getPixelRatio(),r===void 0){const c=l.getSize(new zt);this._width=c.width,this._height=c.height,r=new Vs(this._width*this._pixelRatio,this._height*this._pixelRatio,{type:rl}),r.texture.name="EffectComposer.rt1"}else this._width=r.width,this._height=r.height;this.renderTarget1=r,this.renderTarget2=r.clone(),this.renderTarget2.texture.name="EffectComposer.rt2",this.writeBuffer=this.renderTarget1,this.readBuffer=this.renderTarget2,this.renderToScreen=!0,this.passes=[],this.copyPass=new kh(Ei),this.copyPass.material.blending=Pl,this.timer=new tg}swapBuffers(){const l=this.readBuffer;this.readBuffer=this.writeBuffer,this.writeBuffer=l}addPass(l){this.passes.push(l),l.setSize(this._width*this._pixelRatio,this._height*this._pixelRatio)}insertPass(l,r){this.passes.splice(r,0,l),l.setSize(this._width*this._pixelRatio,this._height*this._pixelRatio)}removePass(l){const r=this.passes.indexOf(l);r!==-1&&this.passes.splice(r,1)}isLastEnabledPass(l){for(let r=l+1;r<this.passes.length;r++)if(this.passes[r].enabled)return!1;return!0}render(l){this.timer.update(),l===void 0&&(l=this.timer.getDelta());const r=this.renderer.getRenderTarget();let c=!1;for(let i=0,f=this.passes.length;i<f;i++){const h=this.passes[i];if(h.enabled!==!1){if(h.renderToScreen=this.renderToScreen&&this.isLastEnabledPass(i),h.render(this.renderer,this.writeBuffer,this.readBuffer,l,c),h.needsSwap){if(c){const g=this.renderer.getContext(),m=this.renderer.state.buffers.stencil;m.setFunc(g.NOTEQUAL,1,4294967295),this.copyPass.render(this.renderer,this.writeBuffer,this.readBuffer,l),m.setFunc(g.EQUAL,1,4294967295)}this.swapBuffers()}H1!==void 0&&(h instanceof H1?c=!0:h instanceof e3&&(c=!1))}}this.renderer.setRenderTarget(r)}reset(l){if(l===void 0){const r=this.renderer.getSize(new zt);this._pixelRatio=this.renderer.getPixelRatio(),this._width=r.width,this._height=r.height,l=this.renderTarget1.clone(),l.setSize(this._width*this._pixelRatio,this._height*this._pixelRatio)}this.renderTarget1.dispose(),this.renderTarget2.dispose(),this.renderTarget1=l,this.renderTarget2=l.clone(),this.writeBuffer=this.renderTarget1,this.readBuffer=this.renderTarget2}setSize(l,r){this._width=l,this._height=r;const c=this._width*this._pixelRatio,i=this._height*this._pixelRatio;this.renderTarget1.setSize(c,i),this.renderTarget2.setSize(c,i);for(let f=0;f<this.passes.length;f++)this.passes[f].setSize(c,i)}setPixelRatio(l){this._pixelRatio=l,this.setSize(this._width,this._height)}dispose(){this.renderTarget1.dispose(),this.renderTarget2.dispose(),this.copyPass.dispose()}}const J0={defines:{PERSPECTIVE_CAMERA:1,SAMPLES:16,NORMAL_VECTOR_TYPE:1,DEPTH_SWIZZLING:"x",SCREEN_SPACE_RADIUS:0,SCREEN_SPACE_RADIUS_SCALE:100,SCENE_CLIP_BOX:0},uniforms:{tNormal:{value:null},tDepth:{value:null},tNoise:{value:null},resolution:{value:new zt},cameraNear:{value:null},cameraFar:{value:null},cameraProjectionMatrix:{value:new Co},cameraProjectionMatrixInverse:{value:new Co},cameraWorldMatrix:{value:new Co},radius:{value:.25},distanceExponent:{value:1},thickness:{value:1},distanceFallOff:{value:1},scale:{value:1},sceneBoxMin:{value:new lt(-1,-1,-1)},sceneBoxMax:{value:new lt(1,1,1)}},vertexShader:`

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
		}`},$0={defines:{PERSPECTIVE_CAMERA:1},uniforms:{tDepth:{value:null},cameraNear:{value:null},cameraFar:{value:null}},vertexShader:`
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

		}`},ih={uniforms:{tDiffuse:{value:null},intensity:{value:1}},vertexShader:`
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
		}`};function a3(o=5){const l=Math.floor(o)%2===0?Math.floor(o)+1:Math.floor(o),r=n3(l),c=r.length,i=new Uint8Array(c*4);for(let h=0;h<c;++h){const g=r[h],m=2*Math.PI*g/c,d=new lt(Math.cos(m),Math.sin(m),0).normalize();i[h*4]=(d.x*.5+.5)*255,i[h*4+1]=(d.y*.5+.5)*255,i[h*4+2]=127,i[h*4+3]=255}const f=new fa(i,l,l);return f.wrapS=za,f.wrapT=za,f.needsUpdate=!0,f}function n3(o){const l=Math.floor(o)%2===0?Math.floor(o)+1:Math.floor(o),r=l*l,c=Array(r).fill(0);let i=Math.floor(l/2),f=l-1;for(let h=1;h<=r;){if(i===-1&&f===l?(f=l-2,i=0):(f===l&&(f=0),i<0&&(i=l-1)),c[i*l+f]!==0){f-=2,i++;continue}else c[i*l+f]=h++;f++,i--}return c}const eu={defines:{SAMPLES:16,SAMPLE_VECTORS:Rp(16,2,1),NORMAL_VECTOR_TYPE:1,DEPTH_VALUE_SOURCE:0},uniforms:{tDiffuse:{value:null},tNormal:{value:null},tDepth:{value:null},tNoise:{value:null},resolution:{value:new zt},cameraProjectionMatrixInverse:{value:new Co},lumaPhi:{value:5},depthPhi:{value:5},normalPhi:{value:5},radius:{value:4},index:{value:0}},vertexShader:`

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
		}`};function Rp(o,l,r){const c=o3(o,l,r);let i="vec3[SAMPLES](";for(let f=0;f<o;f++){const h=c[f];i+=`vec3(${h.x}, ${h.y}, ${h.z})${f<o-1?",":")"}`}return i}function o3(o,l,r){const c=[];for(let i=0;i<o;i++){const f=2*Math.PI*l*i/o,h=Math.pow(i/(o-1),r);c.push(new lt(Math.cos(f),Math.sin(f),h))}return c}class l3{constructor(l=Math){this.grad3=[[1,1,0],[-1,1,0],[1,-1,0],[-1,-1,0],[1,0,1],[-1,0,1],[1,0,-1],[-1,0,-1],[0,1,1],[0,-1,1],[0,1,-1],[0,-1,-1]],this.grad4=[[0,1,1,1],[0,1,1,-1],[0,1,-1,1],[0,1,-1,-1],[0,-1,1,1],[0,-1,1,-1],[0,-1,-1,1],[0,-1,-1,-1],[1,0,1,1],[1,0,1,-1],[1,0,-1,1],[1,0,-1,-1],[-1,0,1,1],[-1,0,1,-1],[-1,0,-1,1],[-1,0,-1,-1],[1,1,0,1],[1,1,0,-1],[1,-1,0,1],[1,-1,0,-1],[-1,1,0,1],[-1,1,0,-1],[-1,-1,0,1],[-1,-1,0,-1],[1,1,1,0],[1,1,-1,0],[1,-1,1,0],[1,-1,-1,0],[-1,1,1,0],[-1,1,-1,0],[-1,-1,1,0],[-1,-1,-1,0]],this.p=[];for(let r=0;r<256;r++)this.p[r]=Math.floor(l.random()*256);this.perm=[];for(let r=0;r<512;r++)this.perm[r]=this.p[r&255];this.simplex=[[0,1,2,3],[0,1,3,2],[0,0,0,0],[0,2,3,1],[0,0,0,0],[0,0,0,0],[0,0,0,0],[1,2,3,0],[0,2,1,3],[0,0,0,0],[0,3,1,2],[0,3,2,1],[0,0,0,0],[0,0,0,0],[0,0,0,0],[1,3,2,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[1,2,0,3],[0,0,0,0],[1,3,0,2],[0,0,0,0],[0,0,0,0],[0,0,0,0],[2,3,0,1],[2,3,1,0],[1,0,2,3],[1,0,3,2],[0,0,0,0],[0,0,0,0],[0,0,0,0],[2,0,3,1],[0,0,0,0],[2,1,3,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0],[2,0,1,3],[0,0,0,0],[0,0,0,0],[0,0,0,0],[3,0,1,2],[3,0,2,1],[0,0,0,0],[3,1,2,0],[2,1,0,3],[0,0,0,0],[0,0,0,0],[0,0,0,0],[3,1,0,2],[0,0,0,0],[3,2,0,1],[3,2,1,0]]}noise(l,r){let c,i,f;const h=.5*(Math.sqrt(3)-1),g=(l+r)*h,m=Math.floor(l+g),d=Math.floor(r+g),x=(3-Math.sqrt(3))/6,M=(m+d)*x,T=m-M,A=d-M,R=l-T,B=r-A;let C,H;R>B?(C=1,H=0):(C=0,H=1);const K=R-C+x,U=B-H+x,_=R-1+2*x,I=B-1+2*x,Z=m&255,le=d&255,X=this.perm[Z+this.perm[le]]%12,se=this.perm[Z+C+this.perm[le+H]]%12,we=this.perm[Z+1+this.perm[le+1]]%12;let ge=.5-R*R-B*B;ge<0?c=0:(ge*=ge,c=ge*ge*this._dot(this.grad3[X],R,B));let G=.5-K*K-U*U;G<0?i=0:(G*=G,i=G*G*this._dot(this.grad3[se],K,U));let F=.5-_*_-I*I;return F<0?f=0:(F*=F,f=F*F*this._dot(this.grad3[we],_,I)),70*(c+i+f)}noise3d(l,r,c){let i,f,h,g;const d=(l+r+c)*.3333333333333333,x=Math.floor(l+d),M=Math.floor(r+d),T=Math.floor(c+d),A=1/6,R=(x+M+T)*A,B=x-R,C=M-R,H=T-R,K=l-B,U=r-C,_=c-H;let I,Z,le,X,se,we;K>=U?U>=_?(I=1,Z=0,le=0,X=1,se=1,we=0):K>=_?(I=1,Z=0,le=0,X=1,se=0,we=1):(I=0,Z=0,le=1,X=1,se=0,we=1):U<_?(I=0,Z=0,le=1,X=0,se=1,we=1):K<_?(I=0,Z=1,le=0,X=0,se=1,we=1):(I=0,Z=1,le=0,X=1,se=1,we=0);const ge=K-I+A,G=U-Z+A,F=_-le+A,ie=K-X+2*A,$=U-se+2*A,N=_-we+2*A,P=K-1+3*A,k=U-1+3*A,ee=_-1+3*A,Me=x&255,S=M&255,Q=T&255,ve=this.perm[Me+this.perm[S+this.perm[Q]]]%12,V=this.perm[Me+I+this.perm[S+Z+this.perm[Q+le]]]%12,pe=this.perm[Me+X+this.perm[S+se+this.perm[Q+we]]]%12,ue=this.perm[Me+1+this.perm[S+1+this.perm[Q+1]]]%12;let Ce=.6-K*K-U*U-_*_;Ce<0?i=0:(Ce*=Ce,i=Ce*Ce*this._dot3(this.grad3[ve],K,U,_));let Ne=.6-ge*ge-G*G-F*F;Ne<0?f=0:(Ne*=Ne,f=Ne*Ne*this._dot3(this.grad3[V],ge,G,F));let oe=.6-ie*ie-$*$-N*N;oe<0?h=0:(oe*=oe,h=oe*oe*this._dot3(this.grad3[pe],ie,$,N));let J=.6-P*P-k*k-ee*ee;return J<0?g=0:(J*=J,g=J*J*this._dot3(this.grad3[ue],P,k,ee)),32*(i+f+h+g)}noise4d(l,r,c,i){const f=this.grad4,h=this.simplex,g=this.perm,m=(Math.sqrt(5)-1)/4,d=(5-Math.sqrt(5))/20;let x,M,T,A,R;const B=(l+r+c+i)*m,C=Math.floor(l+B),H=Math.floor(r+B),K=Math.floor(c+B),U=Math.floor(i+B),_=(C+H+K+U)*d,I=C-_,Z=H-_,le=K-_,X=U-_,se=l-I,we=r-Z,ge=c-le,G=i-X,F=se>we?32:0,ie=se>ge?16:0,$=we>ge?8:0,N=se>G?4:0,P=we>G?2:0,k=ge>G?1:0,ee=F+ie+$+N+P+k,Me=h[ee][0]>=3?1:0,S=h[ee][1]>=3?1:0,Q=h[ee][2]>=3?1:0,ve=h[ee][3]>=3?1:0,V=h[ee][0]>=2?1:0,pe=h[ee][1]>=2?1:0,ue=h[ee][2]>=2?1:0,Ce=h[ee][3]>=2?1:0,Ne=h[ee][0]>=1?1:0,oe=h[ee][1]>=1?1:0,J=h[ee][2]>=1?1:0,_e=h[ee][3]>=1?1:0,Je=se-Me+d,O=we-S+d,Pe=ge-Q+d,Ie=G-ve+d,tt=se-V+2*d,Nt=we-pe+2*d,Ft=ge-ue+2*d,$t=G-Ce+2*d,Yt=se-Ne+3*d,Aa=we-oe+3*d,mn=ge-J+3*d,Ea=G-_e+3*d,Dt=se-1+4*d,_o=we-1+4*d,sa=ge-1+4*d,Ra=G-1+4*d,Ya=C&255,pn=H&255,Na=K&255,vn=U&255,Ia=g[Ya+g[pn+g[Na+g[vn]]]]%32,gn=g[Ya+Me+g[pn+S+g[Na+Q+g[vn+ve]]]]%32,na=g[Ya+V+g[pn+pe+g[Na+ue+g[vn+Ce]]]]%32,yn=g[Ya+Ne+g[pn+oe+g[Na+J+g[vn+_e]]]]%32,ha=g[Ya+1+g[pn+1+g[Na+1+g[vn+1]]]]%32;let zo=.6-se*se-we*we-ge*ge-G*G;zo<0?x=0:(zo*=zo,x=zo*zo*this._dot4(f[Ia],se,we,ge,G));let Ao=.6-Je*Je-O*O-Pe*Pe-Ie*Ie;Ao<0?M=0:(Ao*=Ao,M=Ao*Ao*this._dot4(f[gn],Je,O,Pe,Ie));let dt=.6-tt*tt-Nt*Nt-Ft*Ft-$t*$t;dt<0?T=0:(dt*=dt,T=dt*dt*this._dot4(f[na],tt,Nt,Ft,$t));let Da=.6-Yt*Yt-Aa*Aa-mn*mn-Ea*Ea;Da<0?A=0:(Da*=Da,A=Da*Da*this._dot4(f[yn],Yt,Aa,mn,Ea));let Wa=.6-Dt*Dt-_o*_o-sa*sa-Ra*Ra;return Wa<0?R=0:(Wa*=Wa,R=Wa*Wa*this._dot4(f[ha],Dt,_o,sa,Ra)),27*(x+M+T+A+R)}_dot(l,r,c){return l[0]*r+l[1]*c}_dot3(l,r,c,i){return l[0]*r+l[1]*c+l[2]*i}_dot4(l,r,c,i,f){return l[0]*r+l[1]*c+l[2]*i+l[3]*f}}class To extends Ni{constructor(l,r,c=512,i=512,f,h,g){super(),this.width=c,this.height=i,this.clear=!0,this.camera=r,this.scene=l,this.output=0,this._renderGBuffer=!0,this._visibilityCache=[],this.blendIntensity=1,this.pdRings=2,this.pdRadiusExponent=2,this.pdSamples=16,this.gtaoNoiseTexture=a3(),this.pdNoiseTexture=this._generateNoise(),this.gtaoRenderTarget=new Vs(this.width,this.height,{type:rl}),this.pdRenderTarget=this.gtaoRenderTarget.clone(),this.gtaoMaterial=new Pa({defines:Object.assign({},J0.defines),uniforms:ll.clone(J0.uniforms),vertexShader:J0.vertexShader,fragmentShader:J0.fragmentShader,blending:Pl,depthTest:!1,depthWrite:!1}),this.gtaoMaterial.defines.PERSPECTIVE_CAMERA=this.camera.isPerspectiveCamera?1:0,this.gtaoMaterial.uniforms.tNoise.value=this.gtaoNoiseTexture,this.gtaoMaterial.uniforms.resolution.value.set(this.width,this.height),this.gtaoMaterial.uniforms.cameraNear.value=this.camera.near,this.gtaoMaterial.uniforms.cameraFar.value=this.camera.far,this.normalMaterial=new ag,this.normalMaterial.blending=Pl,this.pdMaterial=new Pa({defines:Object.assign({},eu.defines),uniforms:ll.clone(eu.uniforms),vertexShader:eu.vertexShader,fragmentShader:eu.fragmentShader,depthTest:!1,depthWrite:!1}),this.pdMaterial.uniforms.tDiffuse.value=this.gtaoRenderTarget.texture,this.pdMaterial.uniforms.tNoise.value=this.pdNoiseTexture,this.pdMaterial.uniforms.resolution.value.set(this.width,this.height),this.pdMaterial.uniforms.lumaPhi.value=10,this.pdMaterial.uniforms.depthPhi.value=2,this.pdMaterial.uniforms.normalPhi.value=3,this.pdMaterial.uniforms.radius.value=8,this.depthRenderMaterial=new Pa({defines:Object.assign({},$0.defines),uniforms:ll.clone($0.uniforms),vertexShader:$0.vertexShader,fragmentShader:$0.fragmentShader,blending:Pl}),this.depthRenderMaterial.uniforms.cameraNear.value=this.camera.near,this.depthRenderMaterial.uniforms.cameraFar.value=this.camera.far,this.copyMaterial=new Pa({uniforms:ll.clone(Ei.uniforms),vertexShader:Ei.vertexShader,fragmentShader:Ei.fragmentShader,transparent:!0,depthTest:!1,depthWrite:!1,blendSrc:v1,blendDst:X0,blendEquation:W0,blendSrcAlpha:p1,blendDstAlpha:X0,blendEquationAlpha:W0}),this.blendMaterial=new Pa({uniforms:ll.clone(ih.uniforms),vertexShader:ih.vertexShader,fragmentShader:ih.fragmentShader,transparent:!0,depthTest:!1,depthWrite:!1,blending:ng,blendSrc:v1,blendDst:X0,blendEquation:W0,blendSrcAlpha:p1,blendDstAlpha:X0,blendEquationAlpha:W0}),this._fsQuad=new bu(null),this._originalClearColor=new Zt,this.setGBuffer(f?f.depthTexture:void 0,f?f.normalTexture:void 0),h!==void 0&&this.updateGtaoMaterial(h),g!==void 0&&this.updatePdMaterial(g)}setSize(l,r){this.width=l,this.height=r,this.gtaoRenderTarget.setSize(l,r),this.normalRenderTarget.setSize(l,r),this.pdRenderTarget.setSize(l,r),this.gtaoMaterial.uniforms.resolution.value.set(l,r),this.gtaoMaterial.uniforms.cameraProjectionMatrix.value.copy(this.camera.projectionMatrix),this.gtaoMaterial.uniforms.cameraProjectionMatrixInverse.value.copy(this.camera.projectionMatrixInverse),this.pdMaterial.uniforms.resolution.value.set(l,r),this.pdMaterial.uniforms.cameraProjectionMatrixInverse.value.copy(this.camera.projectionMatrixInverse)}dispose(){this.gtaoNoiseTexture.dispose(),this.pdNoiseTexture.dispose(),this.normalRenderTarget.dispose(),this.gtaoRenderTarget.dispose(),this.pdRenderTarget.dispose(),this.normalMaterial.dispose(),this.pdMaterial.dispose(),this.copyMaterial.dispose(),this.depthRenderMaterial.dispose(),this._fsQuad.dispose()}get gtaoMap(){return this.pdRenderTarget.texture}setGBuffer(l,r){l!==void 0?(this.depthTexture=l,this.normalTexture=r,this._renderGBuffer=!1):(this.depthTexture=new og,this.depthTexture.format=lg,this.depthTexture.type=sg,this.normalRenderTarget=new Vs(this.width,this.height,{minFilter:Ks,magFilter:Ks,type:rl,depthTexture:this.depthTexture}),this.normalTexture=this.normalRenderTarget.texture,this._renderGBuffer=!0);const c=this.normalTexture?1:0,i=this.depthTexture===this.normalTexture?"w":"x";this.gtaoMaterial.defines.NORMAL_VECTOR_TYPE=c,this.gtaoMaterial.defines.DEPTH_SWIZZLING=i,this.gtaoMaterial.uniforms.tNormal.value=this.normalTexture,this.gtaoMaterial.uniforms.tDepth.value=this.depthTexture,this.pdMaterial.defines.NORMAL_VECTOR_TYPE=c,this.pdMaterial.defines.DEPTH_SWIZZLING=i,this.pdMaterial.uniforms.tNormal.value=this.normalTexture,this.pdMaterial.uniforms.tDepth.value=this.depthTexture,this.depthRenderMaterial.uniforms.tDepth.value=this.normalRenderTarget.depthTexture}setSceneClipBox(l){l?(this.gtaoMaterial.needsUpdate=this.gtaoMaterial.defines.SCENE_CLIP_BOX!==1,this.gtaoMaterial.defines.SCENE_CLIP_BOX=1,this.gtaoMaterial.uniforms.sceneBoxMin.value.copy(l.min),this.gtaoMaterial.uniforms.sceneBoxMax.value.copy(l.max)):(this.gtaoMaterial.needsUpdate=this.gtaoMaterial.defines.SCENE_CLIP_BOX===0,this.gtaoMaterial.defines.SCENE_CLIP_BOX=0)}updateGtaoMaterial(l){l.radius!==void 0&&(this.gtaoMaterial.uniforms.radius.value=l.radius),l.distanceExponent!==void 0&&(this.gtaoMaterial.uniforms.distanceExponent.value=l.distanceExponent),l.thickness!==void 0&&(this.gtaoMaterial.uniforms.thickness.value=l.thickness),l.distanceFallOff!==void 0&&(this.gtaoMaterial.uniforms.distanceFallOff.value=l.distanceFallOff,this.gtaoMaterial.needsUpdate=!0),l.scale!==void 0&&(this.gtaoMaterial.uniforms.scale.value=l.scale),l.samples!==void 0&&l.samples!==this.gtaoMaterial.defines.SAMPLES&&(this.gtaoMaterial.defines.SAMPLES=l.samples,this.gtaoMaterial.needsUpdate=!0),l.screenSpaceRadius!==void 0&&(l.screenSpaceRadius?1:0)!==this.gtaoMaterial.defines.SCREEN_SPACE_RADIUS&&(this.gtaoMaterial.defines.SCREEN_SPACE_RADIUS=l.screenSpaceRadius?1:0,this.gtaoMaterial.needsUpdate=!0)}updatePdMaterial(l){let r=!1;l.lumaPhi!==void 0&&(this.pdMaterial.uniforms.lumaPhi.value=l.lumaPhi),l.depthPhi!==void 0&&(this.pdMaterial.uniforms.depthPhi.value=l.depthPhi),l.normalPhi!==void 0&&(this.pdMaterial.uniforms.normalPhi.value=l.normalPhi),l.radius!==void 0&&l.radius!==this.radius&&(this.pdMaterial.uniforms.radius.value=l.radius),l.radiusExponent!==void 0&&l.radiusExponent!==this.pdRadiusExponent&&(this.pdRadiusExponent=l.radiusExponent,r=!0),l.rings!==void 0&&l.rings!==this.pdRings&&(this.pdRings=l.rings,r=!0),l.samples!==void 0&&l.samples!==this.pdSamples&&(this.pdSamples=l.samples,r=!0),r&&(this.pdMaterial.defines.SAMPLES=this.pdSamples,this.pdMaterial.defines.SAMPLE_VECTORS=Rp(this.pdSamples,this.pdRings,this.pdRadiusExponent),this.pdMaterial.needsUpdate=!0)}render(l,r,c){switch(this._renderGBuffer&&(this._overrideVisibility(),this._renderOverride(l,this.normalMaterial,this.normalRenderTarget,7829503,1),this._restoreVisibility()),this.gtaoMaterial.uniforms.cameraNear.value=this.camera.near,this.gtaoMaterial.uniforms.cameraFar.value=this.camera.far,this.gtaoMaterial.uniforms.cameraProjectionMatrix.value.copy(this.camera.projectionMatrix),this.gtaoMaterial.uniforms.cameraProjectionMatrixInverse.value.copy(this.camera.projectionMatrixInverse),this.gtaoMaterial.uniforms.cameraWorldMatrix.value.copy(this.camera.matrixWorld),this._renderPass(l,this.gtaoMaterial,this.gtaoRenderTarget,16777215,1),this.pdMaterial.uniforms.cameraProjectionMatrixInverse.value.copy(this.camera.projectionMatrixInverse),this._renderPass(l,this.pdMaterial,this.pdRenderTarget,16777215,1),this.output){case To.OUTPUT.Off:break;case To.OUTPUT.Diffuse:this.copyMaterial.uniforms.tDiffuse.value=c.texture,this.copyMaterial.blending=Pl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r);break;case To.OUTPUT.AO:this.copyMaterial.uniforms.tDiffuse.value=this.gtaoRenderTarget.texture,this.copyMaterial.blending=Pl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r);break;case To.OUTPUT.Denoise:this.copyMaterial.uniforms.tDiffuse.value=this.pdRenderTarget.texture,this.copyMaterial.blending=Pl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r);break;case To.OUTPUT.Depth:this.depthRenderMaterial.uniforms.cameraNear.value=this.camera.near,this.depthRenderMaterial.uniforms.cameraFar.value=this.camera.far,this._renderPass(l,this.depthRenderMaterial,this.renderToScreen?null:r);break;case To.OUTPUT.Normal:this.copyMaterial.uniforms.tDiffuse.value=this.normalRenderTarget.texture,this.copyMaterial.blending=Pl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r);break;case To.OUTPUT.Default:this.copyMaterial.uniforms.tDiffuse.value=c.texture,this.copyMaterial.blending=Pl,this._renderPass(l,this.copyMaterial,this.renderToScreen?null:r),this.blendMaterial.uniforms.intensity.value=this.blendIntensity,this.blendMaterial.uniforms.tDiffuse.value=this.pdRenderTarget.texture,this._renderPass(l,this.blendMaterial,this.renderToScreen?null:r);break;default:console.warn("THREE.GTAOPass: Unknown output type.")}}_renderPass(l,r,c,i,f){l.getClearColor(this._originalClearColor);const h=l.getClearAlpha(),g=l.autoClear;l.setRenderTarget(c),l.autoClear=!1,i!=null&&(l.setClearColor(i),l.setClearAlpha(f||0),l.clear()),this._fsQuad.material=r,this._fsQuad.render(l),l.autoClear=g,l.setClearColor(this._originalClearColor),l.setClearAlpha(h)}_renderOverride(l,r,c,i,f){l.getClearColor(this._originalClearColor);const h=l.getClearAlpha(),g=l.autoClear;l.setRenderTarget(c),l.autoClear=!1,i=r.clearColor||i,f=r.clearAlpha||f,i!=null&&(l.setClearColor(i),l.setClearAlpha(f||0),l.clear()),this.scene.overrideMaterial=r,l.render(this.scene,this.camera),this.scene.overrideMaterial=null,l.autoClear=g,l.setClearColor(this._originalClearColor),l.setClearAlpha(h)}_overrideVisibility(){const l=this.scene,r=this._visibilityCache;l.traverse(function(c){(c.isPoints||c.isLine||c.isLine2)&&c.visible&&(c.visible=!1,r.push(c))})}_restoreVisibility(){const l=this._visibilityCache;for(let r=0;r<l.length;r++)l[r].visible=!0;l.length=0}_generateNoise(l=64){const r=new l3,c=l*l*4,i=new Uint8Array(c);for(let h=0;h<l;h++)for(let g=0;g<l;g++){const m=h,d=g;i[(h*l+g)*4]=(r.noise(m,d)*.5+.5)*255,i[(h*l+g)*4+1]=(r.noise(m+l,d)*.5+.5)*255,i[(h*l+g)*4+2]=(r.noise(m,d+l)*.5+.5)*255,i[(h*l+g)*4+3]=(r.noise(m+l,d+l)*.5+.5)*255}const f=new fa(i,l,l,$n,gh);return f.wrapS=za,f.wrapT=za,f.needsUpdate=!0,f}}To.OUTPUT={Off:-1,Default:0,Diffuse:1,Depth:2,Normal:3,AO:4,Denoise:5};const tu={name:"OutputShader",uniforms:{tDiffuse:{value:null},toneMappingExposure:{value:1}},vertexShader:`
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

		}`};class s3 extends Ni{constructor(){super(),this.isOutputPass=!0,this.uniforms=ll.clone(tu.uniforms),this.material=new ig({name:tu.name,uniforms:this.uniforms,vertexShader:tu.vertexShader,fragmentShader:tu.fragmentShader}),this._fsQuad=new bu(this.material),this._outputColorSpace=null,this._toneMapping=null}render(l,r,c){this.uniforms.tDiffuse.value=c.texture,this.uniforms.toneMappingExposure.value=l.toneMappingExposure,(this._outputColorSpace!==l.outputColorSpace||this._toneMapping!==l.toneMapping)&&(this._outputColorSpace=l.outputColorSpace,this._toneMapping=l.toneMapping,this.material.defines={},rg.getTransfer(this._outputColorSpace)===cg&&(this.material.defines.SRGB_TRANSFER=""),this._toneMapping===ug?this.material.defines.LINEAR_TONE_MAPPING="":this._toneMapping===fg?this.material.defines.REINHARD_TONE_MAPPING="":this._toneMapping===hg?this.material.defines.CINEON_TONE_MAPPING="":this._toneMapping===cp?this.material.defines.ACES_FILMIC_TONE_MAPPING="":this._toneMapping===dg?this.material.defines.AGX_TONE_MAPPING="":this._toneMapping===mg?this.material.defines.NEUTRAL_TONE_MAPPING="":this._toneMapping===pg&&(this.material.defines.CUSTOM_TONE_MAPPING=""),this.material.needsUpdate=!0),this.renderToScreen===!0?(l.setRenderTarget(null),this._fsQuad.render(l)):(l.setRenderTarget(r),this.clear&&l.clear(l.autoClearColor,l.autoClearDepth,l.autoClearStencil),this._fsQuad.render(l))}dispose(){this.material.dispose(),this._fsQuad.dispose()}}class i3 extends Ni{constructor(l,r,c=null,i=null,f=null){super(),this.scene=l,this.camera=r,this.overrideMaterial=c,this.clearColor=i,this.clearAlpha=f,this.clear=!0,this.clearDepth=!1,this.needsSwap=!1,this.isRenderPass=!0,this._oldClearColor=new Zt}render(l,r,c){const i=l.autoClear;l.autoClear=!1;let f,h;this.overrideMaterial!==null&&(h=this.scene.overrideMaterial,this.scene.overrideMaterial=this.overrideMaterial),this.clearColor!==null&&(l.getClearColor(this._oldClearColor),l.setClearColor(this.clearColor,l.getClearAlpha())),this.clearAlpha!==null&&(f=l.getClearAlpha(),l.setClearAlpha(this.clearAlpha)),this.clearDepth==!0&&l.clearDepth(),l.setRenderTarget(this.renderToScreen?null:c),this.clear===!0&&l.clear(l.autoClearColor,l.autoClearDepth,l.autoClearStencil),l.render(this.scene,this.camera),this.clearColor!==null&&l.setClearColor(this._oldClearColor),this.clearAlpha!==null&&l.setClearAlpha(f),this.overrideMaterial!==null&&(this.scene.overrideMaterial=h),l.autoClear=i}}const r3={uniforms:{tDiffuse:{value:null},luminosityThreshold:{value:1},smoothWidth:{value:1},defaultColor:{value:new Zt(0)},defaultOpacity:{value:0}},vertexShader:`

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

		}`};class kr extends Ni{constructor(l,r=1,c,i){super(),this.strength=r,this.radius=c,this.threshold=i,this.resolution=l!==void 0?new zt(l.x,l.y):new zt(256,256),this.clearColor=new Zt(0,0,0),this.needsSwap=!1,this.renderTargetsHorizontal=[],this.renderTargetsVertical=[],this.nMips=5;let f=Math.round(this.resolution.x/2),h=Math.round(this.resolution.y/2);this.renderTargetBright=new Vs(f,h,{type:rl}),this.renderTargetBright.texture.name="UnrealBloomPass.bright",this.renderTargetBright.texture.generateMipmaps=!1;for(let x=0;x<this.nMips;x++){const M=new Vs(f,h,{type:rl});M.texture.name="UnrealBloomPass.h"+x,M.texture.generateMipmaps=!1,this.renderTargetsHorizontal.push(M);const T=new Vs(f,h,{type:rl});T.texture.name="UnrealBloomPass.v"+x,T.texture.generateMipmaps=!1,this.renderTargetsVertical.push(T),f=Math.round(f/2),h=Math.round(h/2)}const g=r3;this.highPassUniforms=ll.clone(g.uniforms),this.highPassUniforms.luminosityThreshold.value=i,this.highPassUniforms.smoothWidth.value=.01,this.materialHighPassFilter=new Pa({uniforms:this.highPassUniforms,vertexShader:g.vertexShader,fragmentShader:g.fragmentShader}),this.separableBlurMaterials=[];const m=[6,10,14,18,22];f=Math.round(this.resolution.x/2),h=Math.round(this.resolution.y/2);for(let x=0;x<this.nMips;x++)this.separableBlurMaterials.push(this._getSeparableBlurMaterial(m[x])),this.separableBlurMaterials[x].uniforms.invSize.value=new zt(1/f,1/h),f=Math.round(f/2),h=Math.round(h/2);this.compositeMaterial=this._getCompositeMaterial(this.nMips),this.compositeMaterial.uniforms.blurTexture1.value=this.renderTargetsVertical[0].texture,this.compositeMaterial.uniforms.blurTexture2.value=this.renderTargetsVertical[1].texture,this.compositeMaterial.uniforms.blurTexture3.value=this.renderTargetsVertical[2].texture,this.compositeMaterial.uniforms.blurTexture4.value=this.renderTargetsVertical[3].texture,this.compositeMaterial.uniforms.blurTexture5.value=this.renderTargetsVertical[4].texture,this.compositeMaterial.uniforms.bloomStrength.value=r,this.compositeMaterial.uniforms.bloomRadius.value=.1;const d=[1,.8,.6,.4,.2];this.compositeMaterial.uniforms.bloomFactors.value=d,this.bloomTintColors=[new lt(1,1,1),new lt(1,1,1),new lt(1,1,1),new lt(1,1,1),new lt(1,1,1)],this.compositeMaterial.uniforms.bloomTintColors.value=this.bloomTintColors,this.copyUniforms=ll.clone(Ei.uniforms),this.blendMaterial=new Pa({uniforms:this.copyUniforms,vertexShader:Ei.vertexShader,fragmentShader:Ei.fragmentShader,premultipliedAlpha:!0,blending:vg,depthTest:!1,depthWrite:!1,transparent:!0}),this._oldClearColor=new Zt,this._oldClearAlpha=1,this._basic=new Dn,this._fsQuad=new bu(null)}dispose(){for(let l=0;l<this.renderTargetsHorizontal.length;l++)this.renderTargetsHorizontal[l].dispose();for(let l=0;l<this.renderTargetsVertical.length;l++)this.renderTargetsVertical[l].dispose();this.renderTargetBright.dispose();for(let l=0;l<this.separableBlurMaterials.length;l++)this.separableBlurMaterials[l].dispose();this.compositeMaterial.dispose(),this.blendMaterial.dispose(),this._basic.dispose(),this._fsQuad.dispose()}setSize(l,r){let c=Math.round(l/2),i=Math.round(r/2);this.renderTargetBright.setSize(c,i);for(let f=0;f<this.nMips;f++)this.renderTargetsHorizontal[f].setSize(c,i),this.renderTargetsVertical[f].setSize(c,i),this.separableBlurMaterials[f].uniforms.invSize.value=new zt(1/c,1/i),c=Math.round(c/2),i=Math.round(i/2)}render(l,r,c,i,f){l.getClearColor(this._oldClearColor),this._oldClearAlpha=l.getClearAlpha();const h=l.autoClear;l.autoClear=!1,l.setClearColor(this.clearColor,0),f&&l.state.buffers.stencil.setTest(!1),this.renderToScreen&&(this._fsQuad.material=this._basic,this._basic.map=c.texture,l.setRenderTarget(null),l.clear(),this._fsQuad.render(l)),this.highPassUniforms.tDiffuse.value=c.texture,this.highPassUniforms.luminosityThreshold.value=this.threshold,this._fsQuad.material=this.materialHighPassFilter,l.setRenderTarget(this.renderTargetBright),l.clear(),this._fsQuad.render(l);let g=this.renderTargetBright;for(let m=0;m<this.nMips;m++)this._fsQuad.material=this.separableBlurMaterials[m],this.separableBlurMaterials[m].uniforms.colorTexture.value=g.texture,this.separableBlurMaterials[m].uniforms.direction.value=kr.BlurDirectionX,l.setRenderTarget(this.renderTargetsHorizontal[m]),l.clear(),this._fsQuad.render(l),this.separableBlurMaterials[m].uniforms.colorTexture.value=this.renderTargetsHorizontal[m].texture,this.separableBlurMaterials[m].uniforms.direction.value=kr.BlurDirectionY,l.setRenderTarget(this.renderTargetsVertical[m]),l.clear(),this._fsQuad.render(l),g=this.renderTargetsVertical[m];this._fsQuad.material=this.compositeMaterial,this.compositeMaterial.uniforms.bloomStrength.value=this.strength,this.compositeMaterial.uniforms.bloomRadius.value=this.radius,this.compositeMaterial.uniforms.bloomTintColors.value=this.bloomTintColors,l.setRenderTarget(this.renderTargetsHorizontal[0]),l.clear(),this._fsQuad.render(l),this._fsQuad.material=this.blendMaterial,this.copyUniforms.tDiffuse.value=this.renderTargetsHorizontal[0].texture,f&&l.state.buffers.stencil.setTest(!0),this.renderToScreen?(l.setRenderTarget(null),this._fsQuad.render(l)):(l.setRenderTarget(c),this._fsQuad.render(l)),l.setClearColor(this._oldClearColor,this._oldClearAlpha),l.autoClear=h}_getSeparableBlurMaterial(l){const r=[],c=l/3;for(let i=0;i<l;i++)r.push(.39894*Math.exp(-.5*i*i/(c*c))/c);return new Pa({defines:{KERNEL_RADIUS:l},uniforms:{colorTexture:{value:null},invSize:{value:new zt(.5,.5)},direction:{value:new zt(.5,.5)},gaussianCoefficients:{value:r}},vertexShader:`

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

				}`})}_getCompositeMaterial(l){return new Pa({defines:{NUM_MIPS:l},uniforms:{blurTexture1:{value:null},blurTexture2:{value:null},blurTexture3:{value:null},blurTexture4:{value:null},blurTexture5:{value:null},bloomStrength:{value:1},bloomFactors:{value:null},bloomTintColors:{value:null},bloomRadius:{value:0}},vertexShader:`

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

				}`})}}kr.BlurDirectionX=new zt(1,0);kr.BlurDirectionY=new zt(0,1);function c3(){return new kh({name:"SanitizeShader",uniforms:{tDiffuse:{value:null}},vertexShader:`
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
      }`})}const vu={value:null};function Np(o){vu.value||(vu.value=qy(o))}function u3(o){const l=o,r=l._overrideVisibility.bind(o);l._overrideVisibility=()=>{r(),l.scene.traverse(c=>{c.userData.noOcclusion&&c.visible&&(c.visible=!1,l._visibilityCache.push(c))})}}function f3(o){const l={get value(){return vu.value?1:0}};o.onBeforeCompile=r=>{r.uniforms.foliageMask=vu,r.uniforms.foliageMaskOn=l,r.vertexShader=r.vertexShader.replace("#include <common>",`#include <common>
attribute float aLeafCard;
varying vec2 vCardUv;
varying float vLeafCard;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vCardUv = uv; vLeafCard = aLeafCard;`),r.fragmentShader=r.fragmentShader.replace("void main() {",`uniform sampler2D foliageMask;
uniform float foliageMaskOn;
varying vec2 vCardUv;
varying float vLeafCard;
void main() {`).replace("#include <normal_fragment_begin>",`#include <normal_fragment_begin>
if (foliageMaskOn > 0.5 && vLeafCard > 0.5 && texture2D(foliageMask, vCardUv).a < 0.45) discard;`)},o.needsUpdate=!0}const il=500,h3=Jl.tileLoadM,d3=300,m3=2;function js(o,l){const r=Math.sin(o.rotationRadians),c=Math.cos(o.rotationRadians),i=(l.x_km-o.origin.x_km)*1e3,f=(l.y_km-o.origin.y_km)*1e3,h=i*c+f*r,g=-i*r+f*c;return[Math.floor(h/il),Math.floor(g/il)]}function gu(o,l){return`${o},${l}`}function rh(o,l,r){const c=Math.max(o*il-r.x,0,r.x-(o+1)*il),i=Math.max(l*il-r.z,0,r.z-(l+1)*il);return Math.hypot(c,i)}const bh={log:!1};class p3{constructor(l,r,c,i=h3){this.index=l,this.load=r,this.onDetail=c,this.loadM=i}group=new _a;tiles=new Map;fetching=0;disposed=!1;lastCamera=null;update(l,r){if(this.disposed)return;this.lastCamera=l.clone();const c=performance.now(),i=Math.ceil((this.loadM+il)/il),f=Math.floor(l.x/il),h=Math.floor(l.z/il);for(let d=h-i;d<=h+i;d+=1)for(let x=f-i;x<=f+i;x+=1){if(rh(x,d,l)>this.loadM)continue;const M=gu(x,d);this.tiles.has(M)||this.tiles.set(M,{column:x,row:d,state:"idle",scene:null,build:null,layer:null})}const g=[...this.tiles.values()].map(d=>({tile:d,distance:rh(d.column,d.row,l)}));g.sort((d,x)=>d.distance-x.distance);let m=0;for(const{tile:d}of g)(d.state==="fetching"||d.state==="ready"||d.state==="building"||d.state==="mounted")&&(m+=1);for(const{tile:d,distance:x}of g){if(x>this.loadM+d3){d.state!=="idle"&&d.state!=="empty"&&d.state!=="failed"&&(m-=1),this.drop(d);continue}if(d.state==="idle"&&x<=this.loadM&&this.fetching<m3&&m<Jl.maxTiles&&(this.fetch(d),m+=1),d.state==="ready"){const M=performance.now();d.build=Ep(d.scene),bh.log&&console.log(`[tiles] begin ${d.column},${d.row} ${(performance.now()-M).toFixed(0)}ms`),d.scene=null,d.state="building"}if(d.state==="building"&&d.build){const M=r-(performance.now()-c);if(M>0){const T=performance.now();d.build.step(M),bh.log&&console.log(`[tiles] step ${d.column},${d.row} ${(performance.now()-T).toFixed(1)}ms progress ${d.build.progress.toFixed(2)}`)}d.build.done&&this.mount(d,d.build.layer)}}}tick(l,r,c){for(const i of this.tiles.values())i.layer?.update(l,r,c)}summary(){const l={};for(const r of this.tiles.values())l[r.state]=(l[r.state]??0)+1;return l}pending(){let l=0;for(const r of this.tiles.values())(r.state==="fetching"||r.state==="ready"||r.state==="building"||r.state==="idle"&&this.lastCamera&&rh(r.column,r.row,this.lastCamera)<=this.loadM)&&(l+=1);return l}dispose(){this.disposed=!0;for(const l of this.tiles.values())this.drop(l);this.tiles.clear()}fetch(l){l.state="fetching",this.fetching+=1,this.load(this.index,l.column,l.row).then(r=>{if(this.disposed)return;const c=r.meshes.length===0&&r.instances.every(i=>i.count===0);c||Np(r.textures),l.scene=c?null:r,l.state=c?"empty":"ready"}).catch(()=>{this.disposed||(l.state="failed")}).finally(()=>{this.fetching-=1})}mount(l,r){if(!r){l.state="empty",l.build=null;return}l.layer=r,l.build=null,l.state="mounted",this.group.add(r.group),this.onDetail(l.column,l.row,!0)}drop(l){l.build&&(l.build.cancel(),l.build=null),l.layer&&(this.group.remove(l.layer.group),l.layer.dispose(),l.layer=null,this.onDetail(l.column,l.row,!1)),(l.state==="mounted"||l.state==="building"||l.state==="ready")&&(l.state="idle"),l.scene=null}}const Gc={"xiang-zhang":{form:"Rounded",leaf:4218688,bark:7302505,height:14,crown:6.8,clear:.3},"yin-xing":{form:"Fan",leaf:6254918,bark:8157556,height:16,crown:6.2,clear:.44},"wu-tong":{form:"Open",leaf:6648159,bark:9078915,height:16,crown:7.5,clear:.45},"huai-shu":{form:"Rounded",leaf:5992013,bark:7828591,height:13,crown:6.8,clear:.34},"yu-shu":{form:"Rounded",leaf:5992276,bark:7433318,height:16,crown:7.2,clear:.36},"gui-hua":{form:"Oval",leaf:3626048,bark:7236456,height:6,crown:2.8,clear:.26},"xue-song":{form:"Layered",leaf:5991012,bark:6972767,height:16,crown:5.5,clear:.18},"liu-shu":{form:"Weeping",leaf:7438424,bark:6907230,height:11,crown:6.2,clear:.28},"song-shu":{form:"Irregular",leaf:5071686,bark:7103579,height:12.5,crown:5.2,clear:.35},"shui-shan":{form:"Conical",leaf:6909539,bark:7630185,height:24,crown:3,clear:.62},"ci-huai":{form:"Rounded",leaf:6320993,bark:7631212,height:13,crown:5.5,clear:.4},"rong-shu":{form:"Banyan",leaf:4613450,bark:7630955,height:14,crown:10,clear:.24},"tao-shu":{form:"Vase",leaf:5401688,bark:7959404,height:5,crown:3.5,clear:.3},"sha-shu":{form:"Conical",leaf:4349017,bark:6643804,height:18,crown:2.9,clear:.4},"he-huan":{form:"Umbrella",leaf:6976088,bark:7433575,height:11,crown:6.2,clear:.42},"yang-shu":{form:"Fastigiate",leaf:6583131,bark:7828591,height:21,crown:3,clear:.18},"bai-la":{form:"Rounded",leaf:5863760,bark:7565163,height:15,crown:6,clear:.36},"long-bai":{form:"Fastigiate",leaf:4549728,bark:7103582,height:10,crown:1.8,clear:.14},"zi-wei":{form:"Vase",leaf:5071676,bark:7564648,height:5.5,crown:2.8,clear:.2},"luan-shu":{form:"Rounded",leaf:7108928,bark:7827818,height:11,crown:5.8,clear:.32},"yu-lan":{form:"Oval",leaf:5336132,bark:9012866,height:8.5,crown:4.1,clear:.24},"bai-hua":{form:"Open",leaf:5796436,bark:9671052,height:10.5,crown:3.9,clear:.38},"hu-yang":{form:"Open",leaf:6779216,bark:8156009,height:11.5,crown:5.5,clear:.3},"sha-zao":{form:"Rounded",leaf:7107935,bark:7103837,height:6,crown:3.4,clear:.2},"zong-lu":{form:"Palm",leaf:5073989,bark:6641481,height:6,crown:3.1,clear:.8},"pu-kui":{form:"Palm",leaf:5600069,bark:7564387,height:9,crown:4.8,clear:.82},"da-wang-ye":{form:"Palm",leaf:5405003,bark:9012866,height:16,crown:8,clear:.84},"ye-zi":{form:"Palm",leaf:6127173,bark:7959402,height:12.5,crown:7,clear:.86},"mu-ma-huang":{form:"Conical",leaf:4940118,bark:7563615,height:17,crown:4,clear:.4},zhu:{form:"Bamboo",leaf:5402948,bark:7110733,height:8,crown:2.3,clear:.35},"lu-wei":{form:"Reed",leaf:6123339,bark:9011297,height:2.9,crown:1.2,clear:.1},"hong-shu":{form:"Shrub",leaf:3955012,bark:6904656,height:3.8,crown:2.2,clear:.25},"sha-ji":{form:"Shrub",leaf:6779227,bark:6642514,height:2.9,crown:1.8,clear:.12},"suo-suo":{form:"Shrub",leaf:6122071,bark:7828329,height:2.8,crown:2.1,clear:.1},"dian-zhuang":{form:"Cushion",leaf:4940621,bark:7761765,height:.4,crown:.5,clear:0}},L1=[["huai-shu",32],["wu-tong",28],["xiang-zhang",14],["yin-xing",9],["bai-la",9],["yang-shu",8]],v3={ChinesePlane:"wu-tong",Ginkgo:"yin-xing",Cedar:"xue-song",Bamboo:"zhu",Willow:"liu-shu",chinesePlane:"wu-tong",ginkgo:"yin-xing",cedar:"xue-song",bamboo:"zhu",willow:"liu-shu"};function ba(o,l=0,r=0){const c=Math.sin(o*12.9898+l*78.233+r*37.719)*43758.5453;return c-Math.floor(c)}function g3(o){return o&&v3[o]||"huai-shu"}function y3(o,l,r){const c=o&&o.length?o.filter(([g])=>Gc[g]):L1,i=c.length?c:L1,f=i.reduce((g,[,m])=>g+m,0);let h=ba(l,r,5.1)*f;for(const[g,m]of i){if(h<m)return g;h-=m}return i[i.length-1][0]}function j1(o){const l=Gc[o]??Gc["huai-shu"];return{height:l.height,crown:l.crown*.5}}function ol(o,l,r){const c=o>>16&255,i=o>>8&255,f=o&255,h=g=>Math.max(0,Math.min(255,Math.round(g)));return h(c*l*(1+.1*r))<<16|h(i*l)<<8|h(f*l*(1-.16*r))}const Jn=(1+Math.sqrt(5))/2,Dp=[[-1,Jn,0],[1,Jn,0],[-1,-Jn,0],[1,-Jn,0],[0,-1,Jn],[0,1,Jn],[0,-1,-Jn],[0,1,-Jn],[Jn,0,-1],[Jn,0,1],[-Jn,0,-1],[-Jn,0,1]].map(o=>{const l=Math.hypot(o[0],o[1],o[2]);return[o[0]/l,o[1]/l,o[2]/l]}),kp=[[0,11,5],[0,5,1],[0,1,7],[0,7,10],[0,10,11],[1,5,9],[5,11,4],[11,10,2],[10,7,6],[7,1,8],[3,9,4],[3,4,2],[3,2,6],[3,6,8],[3,8,9],[4,9,5],[2,4,11],[6,2,10],[8,6,7],[9,8,1]];function yu(o){const l=Math.hypot(o[0],o[1],o[2])||1;return[o[0]/l,o[1]/l,o[2]/l]}const Yl=Dp.map(o=>o.slice()),Up=[];{const o=new Map,l=(r,c)=>{const i=r<c?`${r}_${c}`:`${c}_${r}`;let f=o.get(i);return f===void 0&&(Yl.push(yu([(Yl[r][0]+Yl[c][0])/2,(Yl[r][1]+Yl[c][1])/2,(Yl[r][2]+Yl[c][2])/2])),f=Yl.length-1,o.set(i,f)),f};for(const[r,c,i]of kp){const f=l(r,c),h=l(c,i),g=l(i,r);Up.push([r,f,g],[f,c,h],[g,h,i],[f,h,g])}}function bo(o,l,r,c,i,f,h,g,m,d,x,M=!0){const T=M?Yl:Dp,A=M?Up:kp,R=T.map((H,K)=>{const U=ba(g,K,1.7)*2-1,_=ba(g,K,8.3)*2-1,I=1+x*(.7*U+(M?.35*_:0));return[l+H[0]*i*I,r+H[1]*f*I,c+H[2]*h*I]}),B=Math.max(i,f,h),C=T.map(H=>yu([H[0]/i*B,H[1]/f*B,H[2]/h*B]));for(let H=0;H<A.length;H+=1){const[K,U,_]=A[H],I=[(T[K][0]+T[U][0]+T[_][0])/3,(T[K][1]+T[U][1]+T[_][1])/3,(T[K][2]+T[U][2]+T[_][2])/3],Z=yu([I[0]/i*B,I[1]/f*B,I[2]/h*B]),le=.46+.54*(.5+.5*Z[1]),X=.96+.08*ba(g,H,3.3),se=ol(m,X,(ba(g,H,9.1)-.5)*.25);o.triSmooth?o.triSmooth(R[K],R[U],R[_],C[K],C[U],C[_],se,d*le):o.tri(R[K],R[U],R[_],Z,se,d*le)}}function F1(o,l,r,c,i,f,h,g,m){const d=[];for(let T=0;T<m;T+=1){const A=T/m*Math.PI*2,R=f*(.78+.44*ba(h,T,2.2));d.push([l+Math.cos(A)*R,r+(ba(h,T,4.4)-.5)*(i-r)*.18,c+Math.sin(A)*R])}const x=[l,i,c],M=[l,r-(i-r)*.1,c];for(let T=0;T<m;T+=1){const A=d[T],R=d[(T+1)%m],B=(T+.5)/m*Math.PI*2,C=yu([Math.cos(B)*.7,.55,Math.sin(B)*.7]),H=.92+.16*ba(h,T,6.6);o.tri(x,R,A,C,ol(g,H*.78,.6),1),o.tri(M,A,R,[C[0],-.7,C[2]],ol(g,.5,.6),.55)}}function G1(o,l,r,c,i,f,h){const m=[],d=[];for(let x=0;x<4;x+=1){const M=x/4*Math.PI*2+.4;m.push([l+Math.cos(M)*f,r,c+Math.sin(M)*f]),d.push([l+Math.cos(M)*f*.72,i,c+Math.sin(M)*f*.72])}for(let x=0;x<4;x+=1){const M=(x+1)%4,T=(x+.5)/4*Math.PI*2+.4,A=[Math.cos(T),0,Math.sin(T)];o.tri(m[x],m[M],d[M],A,h,.9),o.tri(m[x],d[M],d[x],A,h,.9)}}function x3(o,l,r,c,i,f,h,g){const m=Gc[h]??Gc["huai-shu"],d=.84+.3*ba(g,1.3,7.7),x=ba(g,2.9,1.1)*2-1,M=ol(m.leaf,1*d,x),T=ol(m.bark,.9,0),A=Math.floor(g*1e6),R=Math.min(.7,Math.max(.05,m.clear)),B=r+i*R,C=i*(1-R),H=Math.max(f*.09,i*.022),K=_=>G1(o,l,r,c,_,H,T),U=m.form;if(U==="Conical"||U==="Layered"){const _=U==="Layered"?4:3,I=U==="Layered"?1.2:.8;K(B+C*.15);for(let Z=0;Z<_;Z+=1){const le=Z/_,X=(Z+1.35)/_,se=f*I*(1-le*.82);F1(o,l,B+C*le*.92,c,B+C*Math.min(1,X),se,A+Z,M,7)}}else if(U==="Fastigiate")K(B),bo(o,l,B+C*.3,c,f*.62,C*.32,f*.62,A,M,1,.16),bo(o,l,B+C*.7,c,f*.52,C*.32,f*.52,A+1,M,1,.16);else if(U==="Umbrella")K(B+C*.3),bo(o,l,B+C*.68,c,f*1.12,C*.26,f*1.12,A,M,1,.14),bo(o,l+f*.4,B+C*.55,c-f*.2,f*.6,C*.2,f*.6,A+1,M,1,.14);else if(U==="Weeping")K(B+C*.2),bo(o,l,B+C*.58,c,f*.95,C*.34,f*.95,A,M,1,.14),bo(o,l,B+C*.28,c,f*.86,C*.34,f*.86,A+1,ol(M,.94,.2),.95,.2);else if(U==="Palm"){const _=r+i*.84;G1(o,l,r,c,_,Math.max(i*.017,.08*f),ol(T,1.5,0));const I=f*1.05;for(let Z=0;Z<2;Z+=1){const le=Z===0?9:6,X=I*(Z===0?1:.62),se=Z===0?-i*.075:i*.03;for(let we=0;we<le;we+=1){const ge=we/le*Math.PI*2+ba(A,we,1.1)+Z*.4,G=X*(Z===0?.26:.2),F=[l+Math.cos(ge)*X,_+se,c+Math.sin(ge)*X],ie=[l+Math.cos(ge)*X*.5,_+i*.035,c+Math.sin(ge)*X*.5],$=[ie[0]+Math.cos(ge+1.5708)*G,ie[1],ie[2]+Math.sin(ge+1.5708)*G],N=[ie[0]+Math.cos(ge-1.5708)*G,ie[1],ie[2]+Math.sin(ge-1.5708)*G],P=[l,_+i*.01,c],k=ol(M,.85+.2*ba(A,we,2.2),0);o.tri(P,$,ie,[0,1,.2],k,1),o.tri(P,ie,N,[0,1,.2],k,1),o.tri(ie,$,F,[0,.8,.3],k,.95),o.tri(ie,F,N,[0,.8,.3],k,.95)}}}else if(U==="Bamboo")for(let _=0;_<3;_+=1){const I=ba(A,_,8.1)*Math.PI*2;bo(o,l+Math.cos(I)*f*.35,r+i*(.55+.1*_),c+Math.sin(I)*f*.35,f*.55,i*.36,f*.55,A+_,M,1,.2)}else if(U==="Reed")for(let _=0;_<2;_+=1)F1(o,l+(_-.5)*f*.5,r,c,r+i*(.9-.1*_),f*.7,A+_,ol(M,1.1,.3),5);else if(U==="Shrub"||U==="Cushion")bo(o,l,r+i*.42,c,f*1,i*.42,f*1,A,M,1,.2),U==="Shrub"&&bo(o,l+f*.5,r+i*.35,c+f*.3,f*.7,i*.35,f*.7,A+1,M,1,.2);else{const _=U==="Open"||U==="Irregular",I=U==="Oval"?1.25:U==="Fan"?1.1:1,Z=U==="Banyan"?1.15:1;K(B+C*.3),bo(o,l,B+C*.52,c,f*1.05*Z,C*.44*I,f*1.05*Z,A,M,1,_?.22:.17);const le=_?3:2;for(let X=0;X<le;X+=1){const se=ba(A,X,3.9)*Math.PI*2,we=f*(.45+.25*ba(A,X,4.1)),ge=B+C*(.4+.35*ba(A,X,5.7)),G=f*(.48+.2*ba(A,X,6.3));bo(o,l+Math.cos(se)*we,ge,c+Math.sin(se)*we,G,C*.26,G,A+3+X,ol(M,.94+.12*ba(A,X,2.5),0),1,.2,!1)}}}const w3=0,b3=1,q1=2,M3=3,Mh=4,ch=8;function Op(o){return{roofM:0,startM:0,storeyM:3,shopM:0,wall:o,baseBand:o,baseN:0,topBand:o,topN:0,unitW:3.6,kind:Mh,window:[0,0,0,0],balcony:[0,0,0],moduleM:0,glass:[.1,.12,.14],glassKind:0,frame:[.3,.3,.3],trim:o,skin:0,flags:0,weather:.4,crown:0,terracotta:!1,roof:[.17,.165,.152],solar:!1,podiumFloors:0,podiumStoreyM:3,setbackM:0,podiumUnitW:3.6,podiumKind:Mh,podiumWindow:[0,0,0,0]}}function S3(o){return{...o,storeyM:o.podiumStoreyM,unitW:o.podiumUnitW,kind:o.podiumKind,window:o.podiumWindow,balcony:[0,0,0],podiumFloors:0}}function V1(o){return{...o,shopM:0}}function T3(o){const l=Math.max(o.length,1),r=new Float32Array(l*ch*4);o.forEach(({look:i,storeys:f},h)=>{const g=h*ch*4,m=(d,x,M,T,A)=>{r.set([x,M,T,A],g+d*4)};m(0,i.wall[0],i.wall[1],i.wall[2],i.storeyM),m(1,i.baseBand[0],i.baseBand[1],i.baseBand[2],i.baseN+256*f),m(2,i.topBand[0],i.topBand[1],i.topBand[2],i.topN),m(3,i.unitW,i.window[0],i.window[1],i.kind),m(4,i.window[2],i.window[3],i.balcony[0],i.balcony[1]),m(5,i.glass[0],i.glass[1],i.glass[2],i.moduleM),m(6,i.frame[0],i.frame[1],i.frame[2],i.skin*8+i.flags),m(7,i.shopM,i.weather,i.glassKind,i.balcony[2])});const c=new fa(r,ch,l,$n,sp);return c.colorSpace=zh,c.magFilter=Ks,c.minFilter=Ks,c.generateMipmaps=!1,c.flipY=!1,c.needsUpdate=!0,c}const C3=`
attribute float aBld;
attribute vec2 aFace;
varying float vBld;
varying vec2 vFace;
`,_3=`
vBld = aBld;
vFace = aFace;
`,z3=`
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
`,A3=`
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
    // A few per cent: at a fifth, a tower's glass was a patchwork and its grid was lost in it.
    massTone = mix(0.94 + 0.12 * mHash(vec3(cell, 13.0)), 1.0, smoothstep(0.15, 0.6, pix));
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
    massTone = mix(0.92 + 0.16 * mHash(vec3(cell, 13.0)), 1.0, smoothstep(0.15, 0.6, pix));
    // Moulded surround and the shadowed head reveal.
    if (surround) {
      float ring = mBoxP(x, t3.y - 0.14, t3.z + 0.14, fw, unitW) * mBoxP(ml, sill - 0.16, head + 0.18, fh, storeyM);
      colour = mix(colour, trimC, clamp(ring - glassCov, 0.0, 1.0) * (1.0 - 0.4 * fadeAny));
    }
    colour *= 1.0 - 0.30 * mBoxP(x, t3.y, t3.z, fw, unitW) * mBoxP(ml, head, head + 0.1, fh, storeyM);
    // What makes one flat not the next: the tear of dirt under each sill, as
    // strong as that household's window is old, and on a block of flats the
    // air-conditioner hung beside the window by whoever lives there, or not,
    // with its own rust run. A wall of identical clean openings is a drawing.
    float life = mHash(vec3(cell, 71.0));
    float drop = sill - ml;
    float tearRun = smoothstep(0.0, 0.06, drop) * (1.0 - smoothstep(0.0, max(sill, 0.3), drop));
    float tear = mBoxP(x, t3.y - 0.04, t3.z + 0.04, fw, unitW) * tearRun * (0.35 + 0.65 * life);
    colour *= 1.0 - vec3(0.85, 0.92, 1.0) * (0.14 + 0.26 * weather) * tear * (1.0 - 0.7 * fadeAny);
    if (!office) {
      float acX0 = t3.z + 0.14;
      float room = step(acX0 + 0.86, unitW);
      float has = step(0.42, mHash(vec3(cell, 53.0))) * room;
      float acY0 = sill - 0.62 + 0.10 * life;
      float box = mBoxP(x, acX0, acX0 + 0.78, fw, unitW) * mBoxP(ml, acY0, acY0 + 0.54, fh, storeyM) * has;
      float under = mBoxP(x, acX0 - 0.03, acX0 + 0.81, fw, unitW) * mBoxP(ml, acY0 - 0.16, acY0, fh, storeyM) * has;
      float rust = mBoxP(x, acX0 + 0.10, acX0 + 0.34, fw, unitW) * has
        * smoothstep(0.0, 0.05, acY0 - 0.16 - ml) * (1.0 - smoothstep(0.0, 1.4, acY0 - 0.16 - ml));
      float seen = 1.0 - smoothstep(0.25, 0.8, fadeAny);
      vec3 casing = vec3(0.62, 0.62, 0.59) * (0.82 + 0.3 * life);
      // The fan grille: a dark disc on the casing.
      float fan = 1.0 - smoothstep(0.17, 0.21, length(vec2(x - acX0 - 0.30, ml - acY0 - 0.27)));
      casing = mix(casing, casing * 0.45, fan * (1.0 - smoothstep(0.1, 0.4, fw / 0.4)));
      colour = mix(colour, casing, box * seen * (1.0 - glassCov));
      colour *= 1.0 - 0.45 * under * seen * (1.0 - box);
      colour *= 1.0 - vec3(0.55, 0.75, 0.95) * 0.30 * rust * seen * (0.5 + weather);
    }
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
  colour *= 1.0 - vec3(0.85, 0.92, 1.0) * (0.16 + 0.30 * weather) * streak * (1.0 - glassCov);
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
`,E3=`
#include <roughnessmap_fragment>
roughnessFactor = massRough;
`,R3=`
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, massF0, massGlass);
material.specularColorBlended = mix(material.specularColorBlended, massF0, massGlass);
`,N3=`
#include <emissivemap_fragment>
{
  // The room behind the glass, and a sky patch where there is no environment map.
  float facing = clamp(abs(dot(normalize(vViewPosition), normal)), 0.0, 1.0);
  float fresnel = pow(1.0 - facing, 3.0);
  totalEmissiveRadiance += massEmit;
  totalEmissiveRadiance += massGlass * massTone * vec3(0.030, 0.037, 0.048) * (0.4 + 3.0 * fresnel);
}
`,D3=`#define uInteriorLightM ${zp.toFixed(4)}
`,Sh=256;function kn(o,l){let r=o*2654435761^l*2246822507;return r^=r>>>16,r=Math.imul(r,2146121005),r^=r>>>15,(r>>>0)/4294967295}const Bp=10246986,K1=[13222838,12169892,13815233,11051928,12563871,10462115,14207920,Bp,15131871],k3=[599e4,7174016,9081755,12106944,5069670],P1=[7565681,7368816,7697266],U3=[14474973,12962252,11451595,14077885,12567487],Y1=[6061470,5205903,9081753,10857648,7244448],O3=[13546914,13546914,13546914,13546914,13546914,12623502,12623502,12623502,12623502,11308676,11308676,11308676,11308676,14140853,14140853,14140853,14140853,10647923,10647923,10647923,14538443,14538443,14538443,12894396,12894396,12894396,12894396,10657433,10657433,10657433,14802907,14802907,9333089,9333089,12618102,12618102,7113109,11320516,11191475],B3=6258591,H3=7239036,W1=14211280,L3=9737364,X1=9067077,j3=6578782,F3=9144706,G3=10131343,q3={arterial:5,collector:4,local:3},V3=2,xu=3;function K3(o,l,r,c,i,f,h,g){let m=1,d=0,x=0;for(let X=0;X<r.length;X+=1){const[se,we]=r[X],[ge,G]=r[(X+1)%r.length],F=Math.hypot(ge-se,G-we);F>x&&(x=F,m=(ge-se)/F,d=(G-we)/F)}const M=r.reduce((X,se)=>X+se[0],0)/r.length,T=r.reduce((X,se)=>X+se[1],0)/r.length;let A=1/0,R=-1/0,B=1/0,C=-1/0;for(const[X,se]of r){const we=(X-M)*m+(se-T)*d,ge=-(X-M)*d+(se-T)*m;A=Math.min(A,we),R=Math.max(R,we),B=Math.min(B,ge),C=Math.max(C,ge)}const H=R-A,K=C-B,U=(X,se)=>[M+m*X-d*se,T+d*X+m*se],_=X=>kn(l.id,X);if(f){if(H>24*h){const[X,se]=U((A+R)*.5,(B+C)*.5);o.orientedBox(X,se,m,d,H*.4,.8*h,c,c+.7*h,13159631)}return}if(l.useType==="village"){if(_(71)<.45&&H>6*h){const X=.35+.35*_(72),[se,we]=U(R-H*X*.5,(B+C)*.5);o.orientedBox(se,we,m,d,H*X*.5,K*.45,c,c+2.6*h,B3)}if(_(73)<.6){const[X,se]=U(A+1.5*h,B+1.5*h);o.orientedBox(X,se,m,d,.8*h,.8*h,c,c+1.6*h,W1)}return}if(l.heightMetres<9)return;if(l.useType==="commercial"){const X=Hp(r,3.5*h);X&&X3(X)>60*h*h&&o.parapet(X,c,3.2*h,L3);const se=1+Math.floor(_(77)*2);for(let we=0;we<se;we+=1){const[ge,G]=U(A+H*(.15+.2*we),C-K*.22);o.orientedBox(ge,G,m,d,2*h,2*h,c,c+3.5*h,W1)}return}const I=(16+4*_(78))*h,Z=Math.max(1,Math.round(H/I)),le=l.floors>=8;for(let X=0;X<Z;X+=1){const se=A+(X+.5)*(H/Z),[we,ge]=U(se,B+K*.62);o.orientedBox(we,ge,m,d,(le?3.2:2)*h,(le?3:1.8)*h,c,c+(le?3.6:2.6)*h,i)}if(g??(l.floors<=7&&_(79)<.75)){const X=Math.max(1,Math.floor((K-3*h)/(2.6*h)));for(let se=0;se<X;se+=1){const we=B+1.6*h+se*2.6*h,[ge,G]=U((A+R)*.5,we);o.orientedBox(ge,G,m,d,H*(.3+.12*_(80+se)),.7*h,c,c+1.3*h,H3)}}}class Z1{positions=[];normals=[];colors=[];tileIds=[];bases=[];seeds=[];blds=[];faces=[];bld=-1;plainBld=-1;base=0;seed=0;tileIndex=new Map;current=-1;colour=new Zt;select(l,r){const c=gu(l,r);let i=this.tileIndex.get(c);i===void 0&&(i=this.tileIndex.size,this.tileIndex.set(c,i)),this.current=i<Sh?i:-1}triangles(){return this.positions.length/9}push(l,r,c,i,f,h,g){this.colour.setHex(f),[l,r,c].forEach((m,d)=>{this.positions.push(m[0],m[1],m[2]),this.normals.push(i[0],i[1],i[2]),this.colors.push(this.colour.r*h,this.colour.g*h,this.colour.b*h),this.tileIds.push(this.current),this.bases.push(this.base),this.seeds.push(this.seed),this.blds.push(this.bld),this.faces.push(g?g.u[d]:0,g?g.length:1)})}tri(l,r,c,i,f,h=1,g){const m=r[0]-l[0],d=r[1]-l[1],x=r[2]-l[2],M=c[0]-l[0],T=c[1]-l[1],A=c[2]-l[2],R=d*A-x*T,B=x*M-m*A,C=m*T-d*M;R*i[0]+B*i[1]+C*i[2]>=0?this.push(l,r,c,i,f,h,g):this.push(l,c,r,i,f,h,g&&{u:[g.u[0],g.u[2],g.u[1]],length:g.length})}triSmooth(l,r,c,i,f,h,g,m=1){const d=r[0]-l[0],x=r[1]-l[1],M=r[2]-l[2],T=c[0]-l[0],A=c[1]-l[1],R=c[2]-l[2],B=x*R-M*A,C=M*T-d*R,H=d*A-x*T,K=i[0]+f[0]+h[0],U=i[1]+f[1]+h[1],_=i[2]+f[2]+h[2],I=B*K+C*U+H*_<0,Z=I?[l,c,r]:[l,r,c],le=I?[i,h,f]:[i,f,h];this.colour.setHex(g);for(let X=0;X<3;X+=1)this.positions.push(Z[X][0],Z[X][1],Z[X][2]),this.normals.push(le[X][0],le[X][1],le[X][2]),this.colors.push(this.colour.r*m,this.colour.g*m,this.colour.b*m),this.tileIds.push(this.current),this.bases.push(this.base),this.seeds.push(this.seed),this.blds.push(this.bld),this.faces.push(0,1)}prism(l,r,c,i,f,h=r,g=!0){const m=l.length;if(m<3||!(c>r))return;this.base=h;let d=0,x=0;for(const[T,A]of l)d+=T,x+=A;d/=m,x/=m;for(let T=0;T<m;T+=1){const[A,R]=l[T],[B,C]=l[(T+1)%m],H=B-A,K=C-R,U=Math.hypot(H,K);if(U<1e-9)continue;let _=K/U,I=-H/U;_*((A+B)*.5-d)+I*((R+C)*.5-x)<0&&(_=-_,I=-I);const Z=[_,0,I];this.tri([A,r,R],[B,r,C],[B,c,C],Z,i,1,{u:[0,U,U],length:U}),this.tri([A,r,R],[B,c,C],[A,c,R],Z,i,1,{u:[0,U,0],length:U})}if(!g)return;const M=l.map(([T,A])=>new zt(T,A));for(const[T,A,R]of g1.triangulateShape(M,[]))this.tri([l[T][0],c,l[T][1]],[l[A][0],c,l[A][1]],[l[R][0],c,l[R][1]],[0,1,0],f,1)}parapet(l,r,c,i){const f=l.length;if(f<3||!(c>0))return;let h=0,g=0;for(const[m,d]of l)h+=m,g+=d;h/=f,g/=f,this.base=r;for(let m=0;m<f;m+=1){const[d,x]=l[m],[M,T]=l[(m+1)%f],A=M-d,R=T-x,B=Math.hypot(A,R);if(B<1e-9)continue;let C=R/B,H=-A/B;C*((d+M)*.5-h)+H*((x+T)*.5-g)<0&&(C=-C,H=-H);for(const K of[1,-1]){const U=[C*K,0,H*K];this.tri([d,r,x],[M,r,T],[M,r+c,T],U,i,1,{u:[0,B,B],length:B}),this.tri([d,r,x],[M,r+c,T],[d,r+c,x],U,i,1,{u:[0,B,0],length:B})}}}pitchedRoof(l,r,c,i,f,h,g){let m=1,d=0,x=0;for(let G=0;G<l.length;G+=1){const[F,ie]=l[G],[$,N]=l[(G+1)%l.length],P=Math.hypot($-F,N-ie);P>x&&(x=P,m=($-F)/P,d=(N-ie)/P)}const M=l.reduce((G,F)=>G+F[0],0)/l.length,T=l.reduce((G,F)=>G+F[1],0)/l.length;let A=1/0,R=-1/0,B=1/0,C=-1/0;for(const[G,F]of l){const ie=(G-M)*m+(F-T)*d,$=-(G-M)*d+(F-T)*m;A=Math.min(A,ie),R=Math.max(R,ie),B=Math.min(B,$),C=Math.max(C,$)}const H=(G,F,ie)=>[M+m*G-d*F,ie,T+d*G+m*F],U=((C-B)*.5+c)*i,_=r,I=r+U,Z=(B+C)*.5,le=A-c,X=R+c,se=f?Math.min(A+(C-B)*.5,(A+R)*.5):le+c,we=f?Math.max(R-(C-B)*.5,(A+R)*.5):X-c,ge=(G,F)=>{const ie=(G[1][1]-G[0][1])*(G[2][2]-G[0][2])-(G[1][2]-G[0][2])*(G[2][1]-G[0][1]),$=(G[1][2]-G[0][2])*(G[2][0]-G[0][0])-(G[1][0]-G[0][0])*(G[2][2]-G[0][2]),N=(G[1][0]-G[0][0])*(G[2][1]-G[0][1])-(G[1][1]-G[0][1])*(G[2][0]-G[0][0]),P=Math.hypot(ie,$,N)||1,k=$<0?-1:1,ee=[ie/P*k,$/P*k,N/P*k];this.tri(G[0],G[1],G[2],ee,F),G.length===4&&this.tri(G[0],G[2],G[3],ee,F)};ge([H(le,B-c,_),H(X,B-c,_),H(we,Z,I),H(se,Z,I)],h),ge([H(le,C+c,_),H(X,C+c,_),H(we,Z,I),H(se,Z,I)],h),f?(ge([H(le,B-c,_),H(le,C+c,_),H(se,Z,I)],h),ge([H(X,B-c,_),H(X,C+c,_),H(we,Z,I)],h)):(ge([H(A,B,_),H(A,C,_),H(A,Z,I)],g),ge([H(R,B,_),H(R,C,_),H(R,Z,I)],g))}box(l,r,c,i,f,h){this.orientedBox(l,r,1,0,c,c,i,f,h)}orientedBox(l,r,c,i,f,h,g,m,d){const x=-i,M=c,T=(C,H)=>[l+c*C+x*H,r+i*C+M*H],A=[T(-f,-h),T(f,-h),T(f,h),T(-f,h)],R=this.seed,B=this.bld;this.seed=xu+R%1,this.bld=this.plainBld,this.prism(A,g,m,d,d),this.seed=R,this.bld=B}flat(l,r,c,i){if(l.length<3)return;const f=l.map(([g,m])=>new zt(g,m)),h=g=>[l[g][0],r(l[g][0],l[g][1])+c,l[g][1]];for(const[g,m,d]of g1.triangulateShape(f,[]))this.tri(h(g),h(m),h(d),[0,1,0],i)}smoothTri(l,r,c,i,f,h){const m=(c[0]-l[0])*(f[2]-l[2])-(c[2]-l[2])*(f[0]-l[0])<0?[[l,r],[c,i],[f,h]]:[[l,r],[f,h],[c,i]];for(const[d,x]of m)this.positions.push(d[0],d[1],d[2]),this.normals.push(0,1,0),this.colors.push(x.r,x.g,x.b),this.tileIds.push(this.current),this.bases.push(0),this.seeds.push(xu),this.blds.push(-1),this.faces.push(0,1)}ribbon(l,r,c){for(let i=0;i+1<l.length;i+=1){const[f,h,g]=l[i],[m,d,x]=l[i+1],M=m-f,T=x-g,A=Math.hypot(M,T);if(A<1e-9)continue;const R=-T/A*r*.5,B=M/A*r*.5,C=[f+R,h,g+B],H=[f-R,h,g-B],K=[m+R,d,x+B],U=[m-R,d,x-B];this.tri(C,H,U,[0,1,0],c),this.tri(C,U,K,[0,1,0],c)}}geometry(){const l=new It;return l.setAttribute("position",new ft(this.positions,3)),l.setAttribute("normal",new ft(this.normals,3)),l.setAttribute("color",new ft(this.colors,3)),l.setAttribute("aTile",new ft(this.tileIds,1)),l.setAttribute("aBase",new ft(this.bases,1)),l.setAttribute("aSeed",new ft(this.seeds,1)),l.setAttribute("aBld",new ft(this.blds,1)),l.setAttribute("aFace",new ft(this.faces,2)),l.computeBoundingSphere(),l.computeBoundingBox(),l}}function Q1(o,l){return o.map((r,c)=>{const i=o[Math.max(0,c-1)],f=o[Math.min(o.length-1,c+1)],h=f[0]-i[0],g=f[2]-i[2],m=Math.hypot(h,g)||1;return[r[0]-g/m*l,r[1],r[2]+h/m*l]})}function I1(o,l,r,c,i,f,h){x3(o,l,r,c,i,f,h,Math.abs(Math.sin(l*12.9898+c*78.233)*43758.5453)%1)}function au(o){const l=Math.max(o.length,1);return{x_km:o.reduce((r,c)=>r+c.x_km,0)/l,y_km:o.reduce((r,c)=>r+c.y_km,0)/l}}const P3=[{at:-1,colour:5069374},{at:-.9,colour:7306079},{at:-.62,colour:5204064},{at:-.25,colour:3625820},{at:0,colour:3099222},{at:.25,colour:3625820},{at:.62,colour:5204064},{at:.9,colour:7306079},{at:1,colour:5069374}];function Y3(o){if(!o)return P3;const l=Math.min(1,Math.max(0,o.moisture)),r=Math.min(1,Math.max(0,.3+1.2*o.sediment+.25*(1-l))),c=(d,x,M)=>new Zt(d).lerp(new Zt(x),M).getHex(),i=c(2902610,4868656,r),f=c(3627100,5920058,r),h=c(5204064,8024136,r),g=c(5069374,10129528,Math.min(1,(1-l)*1.4)),m=c(7306079,11576196,Math.min(1,(1-l)*1.4));return[{at:-1,colour:g},{at:-.9,colour:m},{at:-.62,colour:h},{at:-.25,colour:f},{at:0,colour:i},{at:.25,colour:f},{at:.62,colour:h},{at:.9,colour:m},{at:1,colour:g}]}function J1(o,l,r,c,i,f){const h=Y3(f),g=c.metresToScene;let m=l.map(H=>c.toScene(H.x_km,H.y_km));if(m.length<2)return;for(let H=0;H<3&&m.length<600;H+=1){const K=[m[0]];for(let U=0;U+1<m.length;U+=1){const[_,I]=m[U],[Z,le]=m[U+1];K.push([_*.75+Z*.25,I*.75+le*.25],[_*.25+Z*.75,I*.25+le*.75])}K.push(m[m.length-1]),m=K}const d=r*.5*g,x=d+Math.max(2*g,d*.1),M=m.length,T=m.map((H,K)=>{const U=m[Math.max(0,K-1)],_=m[Math.min(M-1,K+1)],I=_[0]-U[0],Z=_[1]-U[1],le=Math.hypot(I,Z)||1;return[-Z/le,I/le]}),A=m.map(([H,K],U)=>{let _=c.heightAt(H,K);for(const I of[-1,1])_=Math.max(_,c.heightAt(H+T[U][0]*d*I,K+T[U][1]*d*I));return _+.22*g}),R=h.map(H=>new Zt(H.colour)),B=new Zt(6253130);let C=0;for(let H=0;H+1<M;H+=1){i(l[Math.min(l.length-1,Math.round(H/(M-1)*(l.length-1)))]),C+=Math.hypot(m[H+1][0]-m[H][0],m[H+1][1]-m[H][1]);const K=.5+.5*Math.sin(C/g*.011+1.3)*Math.sin(C/g*.031),U=(_,I)=>{const Z=h[I].at,le=Z*(Math.abs(Z)===1?x:d),X=R[I].clone();return I>0&&I<h.length-1&&X.lerp(B,K*.28),{p:[m[_][0]+T[_][0]*le,A[_],m[_][1]+T[_][1]*le],colour:X}};for(let _=0;_+1<h.length;_+=1){const I=U(H,_),Z=U(H,_+1),le=U(H+1,_),X=U(H+1,_+1);o.smoothTri(I.p,I.colour,Z.p,Z.colour,X.p,X.colour),o.smoothTri(I.p,I.colour,X.p,X.colour,le.p,le.colour)}}}const jc=new Zt;function uh(o){return jc.setRGB(o[0],o[1],o[2]).getHex()}function W3(o){return jc.setHex(o),[jc.r,jc.g,jc.b]}function X3(o){let l=0;for(let r=0;r<o.length;r+=1){const[c,i]=o[r],[f,h]=o[(r+1)%o.length];l+=c*h-f*i}return Math.abs(l)*.5}function Hp(o,l){const r=o.length;if(r<3||l<=0)return null;let c=0;for(let m=0;m<r;m+=1){const[d,x]=o[m],[M,T]=o[(m+1)%r];c+=d*T-M*x}const i=c>=0?1:-1,f=o.map((m,d)=>{const x=o[(d+1)%r],M=Math.hypot(x[0]-m[0],x[1]-m[1])||1,T=-(x[1]-m[1])/M*i,A=(x[0]-m[0])/M*i;return{px:m[0]+T*l,pz:m[1]+A*l,dx:(x[0]-m[0])/M,dz:(x[1]-m[1])/M}}),h=[];for(let m=0;m<r;m+=1){const d=f[(m+r-1)%r],x=f[m],M=d.dx*x.dz-d.dz*x.dx;if(Math.abs(M)<1e-6){h.push([x.px,x.pz]);continue}const T=((x.px-d.px)*x.dz-(x.pz-d.pz)*x.dx)/M;h.push([d.px+d.dx*T,d.pz+d.dz*T])}let g=0;for(let m=0;m<r;m+=1)g+=h[m][0]*h[(m+1)%r][1]-h[(m+1)%r][0]*h[m][1];return g*i<=0||Math.abs(g)<Math.abs(c)/3?null:h}function Z3(o,l){const r=o.heightMetres>=40&&o.useType==="commercial",c=l?U3:o.useType==="village"?O3:r?k3:K1;let i=c[Math.floor(kn(o.id,1)*c.length)];i===Bp&&o.heightMetres>25&&(i=K1[0]);const f=W3(i),h=Op(f),m=(o.useType==="commercial"||o.useType==="mixedUse")&&o.floors>=2?4.5:0,d=Math.max(o.floors-(m>0?1:0),1);h.shopM=m,h.roofM=Math.max(o.heightMetres,3),h.storeyM=Math.min(Math.max((h.roofM-m)/d,2.7),4.8),h.weather=.2+.4*kn(o.id,5),h.baseBand=[f[0]*.7,f[1]*.7,f[2]*.7],h.topBand=f,h.trim=f,h.glassKind=o.useType==="commercial"?2:kn(o.id,7)<.4?1:0;const x=kn(o.id,8);h.glass=x<.5?[.3,.55,.52]:[.28,.42,.58],h.frame=[.2,.2,.2],h.flags=kn(o.id,9)<.35?2:0;const M=o.heightMetres>=24,T=Math.max(...o.footprint.map(A=>Math.hypot(A.x_km-o.footprint[0].x_km,A.y_km-o.footprint[0].y_km)))*1e3<34;if(l)h.kind=Mh;else if(o.facade==="curtainWall")h.kind=b3,h.moduleM=1.2+.6*kn(o.id,10),h.unitW=h.moduleM*3,h.glassKind=3,h.glass=[[.42,.66,.66],[.38,.52,.74],[.66,.7,.72],[.3,.33,.36]][Math.floor(kn(o.id,11)*4)],h.frame=[.075,.08,.085],h.window=[0,h.unitW,.9,h.storeyM];else if(o.facade==="concreteGlass"||o.facade==="stoneCivic"){h.kind=o.facade==="stoneCivic"?q1:M3,h.unitW=(o.facade==="stoneCivic"?4.9:4.6)*(T?.78:1);const A=h.kind===q1?.9:.3;h.window=[A,h.unitW-A,.8,h.storeyM-.5],h.glassKind=2}else{h.kind=w3,h.unitW=M?3.3:3.6;const A=!!o.balconyBays&&o.balconyBays>0&&o.useType!=="village";h.window=A?[.35,1.55,.9,Math.min(2.4,h.storeyM-.45)]:[h.unitW*.5-.9,h.unitW*.5+.9,.9,Math.min(2.4,h.storeyM-.45)],A&&(h.balcony=[1.75,h.unitW-.35,h.storeyM-.45])}return h}function Q3(o,l){const r=l.metresToScene,c=new Z1,i=new Map;for(const _ of o.parcels)i.set(_.id,js(o.frame,au(_.ring)));const f=new Float32Array(Sh),h=_=>_.map(I=>l.toScene(I.x_km,I.y_km));for(const _ of o.blocks)c.select(...js(o.frame,au(_.boundary))),c.seed=nl.lawn+kn(_.boundary.length,9)*.999,c.flat(h(_.boundary),l.heightAt,.15*r,7109198);const g={park:5139254,residential:6716492,villa:6256454,farmstead:7042119,village:8026722,commercial:7302505,mixedUse:7631469,civic:10131084,industrial:9209724},m=(_,I)=>{const Z=I*7919%100/100;switch(_){case"industrial":return Z<.55?{colour:g.industrial,kind:nl.yard}:{colour:6447196,kind:nl.asphaltLot};case"commercial":case"mixedUse":return Z<.7?{colour:g[_],kind:nl.parking}:{colour:g.civic,kind:nl.plaza};case"civic":return{colour:g.civic,kind:nl.plaza};case"park":case"residential":case"villa":case"farmstead":return{colour:g[_],kind:nl.lawn};default:return{colour:g[_]??F3,kind:xu}}};for(const _ of o.parcels){c.select(...js(o.frame,au(_.ring)));const I=m(_.useType,_.id);c.seed=I.kind+kn(_.id,6)*.999,c.flat(h(_.ring),l.heightAt,.25*r,I.colour)}const d=[];for(const _ of o.hdRoads){const I=_.centreline[Math.floor(_.centreline.length/2)];I&&c.select(...js(o.frame,I));const Z=_.centreline.map(se=>{const[we,ge]=l.toScene(se.x_km,se.y_km);return[we,l.heightAt(we,ge),ge]});if(Z.length<2)continue;if(_.bridge){const se=Z[0],we=Z[Z.length-1];Z.forEach((ge,G)=>{ge[1]=se[1]+(we[1]-se[1])*(G/(Z.length-1))})}for(const se of Z)se[1]+=.35*r;const le=Math.max(_.widthMetres,6),X=_.layer?0:q3[_.class]??0;if(c.ribbon(Z,(le-2*X)*r,j3),X>0)for(const se of[-1,1])c.ribbon(Q1(Z,se*(le-X)*.5*r),X*r,G3);X>=2.4&&d.push({pts:Z,offset:(le*.5-X+1.05)*r,tile:I?js(o.frame,I):null})}c.seed=xu;for(const _ of o.trees??[]){if(!_.point)continue;c.select(...js(o.frame,_.point));const[I,Z]=l.toScene(_.point.x_km,_.point.y_km),le=l.heightAt(I,Z),X=g3(_.species),se=j1(X);I1(c,I,le,Z,(_.heightMetres??se.height)*r,Math.max(_.crownRadiusMetres??se.crown,1.5)*r,X)}for(const _ of d){_.tile&&c.select(..._.tile);for(const I of[-1,1]){const Z=Q1(_.pts,I*_.offset);let le=4;for(let X=0;X+1<Z.length;X+=1){const[se,we,ge]=Z[X],[G,F,ie]=Z[X+1],$=Math.hypot(G-se,ie-ge);for(;le<$;){const N=le/$,P=se+(G-se)*N,k=ge+(ie-ge)*N,ee=y3(o.streetMix,P,k),Me=j1(ee);I1(c,P,we+(F-we)*N-.35*r,k,Me.height*r,Me.crown*r,ee),le+=8*r}le-=$}}}c.seed=0;const x=[],M=(_,I)=>{const Z=Math.max(1,Math.round((I-_.shopM)/Math.max(_.storeyM,2.4)));return x.push({look:_,storeys:Z}),x.length-1};c.plainBld=M(Op([.5,.5,.5]),3);for(const _ of o.buildings){c.select(...i.get(_.parcelId)??js(o.frame,au(_.footprint)));const I=h(_.footprint);if(I.length<3)continue;let Z=1/0;for(const[pe,ue]of I)Z=Math.min(Z,l.heightAt(pe,ue));const le=_.farLook?.startM??0,X=le>0?Z+le*r:Z-1.5*r,se=_.useType==="industrial"&&_.floors<=1,we=_.farLook??Z3(_,se),ge=_.farLook?we.roofM:Math.max(_.heightMetres,3),G=ge*r,F=uh(we.wall),ie=_.farLook?uh(we.roof):se?Y1[Math.floor(kn(_.id,2)*Y1.length)]:_.roof==="terracotta"?X1:P1[Math.floor(kn(_.id,2)*P1.length)],$=Z+G;c.seed=kn(_.id,61)*.999+(se?V3:0);const N=uh(we.topN>0||we.baseN>0?we.topBand:we.wall),P=se||ge<9?0:_.useType==="commercial"?_.floors>=12?1.6:1.1:1.2,k=_.floors>=19,ee=_.farLook?we.crown===2&&!k||we.crown===4&&!k&&_.floors<=7||we.crown===3&&!k:!1,Me=!_.farLook&&(_.roof==="gable"||_.roof==="hip")&&I.length===4,S=(ee||Me)&&I.length===4,Q=_.farLook?ie:_.roof==="terracotta"?X1:8215114,ve=we.podiumFloors>0?(we.shopM+we.podiumFloors*we.podiumStoreyM)*r:0,V=le===0&&ve>0&&ve<G-6*r?Hp(I,we.setbackM*r):null;if(V)c.bld=M(S3(we),ve/r),c.prism(I,X,Z+ve,F,ie,Z,!0),c.bld=c.plainBld,c.parapet(I,Z+ve,1.1*r,N),c.bld=M(V1(we),(G-ve)/r),c.prism(V,Z+ve,$,F,ie,Z+ve,!0);else if(_.tierRing&&_.tierRing.length>=3&&(_.podiumHeightMetres??0)>0&&(_.podiumHeightMetres??0)*r<G){const pe=(_.podiumHeightMetres??0)*r;c.bld=M(we,_.podiumHeightMetres??0),c.prism(I,X,Z+pe,F,ie,Z,!0),c.bld=M(V1(we),ge-(_.podiumHeightMetres??0)),c.prism(h(_.tierRing),Z+pe,$,F,ie,Z+pe,!0)}else if(c.bld=M(we,ge-le),c.prism(I,X,$,F,ie,le>0?X:Z,!S),S){c.bld=c.plainBld;const pe=ee||_.roof==="hip";c.pitchedRoof(I,$,.6*r,we.crown===3?.7:ee?.5:.62,pe,Q,F)}c.bld=c.plainBld,!S&&P>0&&c.parapet(V??I,$,P*r,N),S||K3(c,_,V??I,$,F,se,r,_.farLook?we.solar:void 0),c.bld=-1}const T=new Z1,A=_=>{const[I,Z]=js(o.frame,_),le=gu(I,Z);let X=c.tileIndex.get(le);X===void 0&&(X=c.tileIndex.size,c.tileIndex.set(le,X)),T.tileIndex.set(le,X),T.select(I,Z)};o.river&&o.river.length>=2&&J1(T,o.river,o.riverWidthMetres??60,l,A,o.riverSetting);for(const _ of o.tributaries??[])_.line.length>=2&&J1(T,_.line,_.widthMetres,l,A,o.riverSetting);let R=null,B=null;if(T.triangles()>0){B=new Ye({vertexColors:!0,roughness:.32,metalness:0,envMapIntensity:.4,polygonOffset:!0,polygonOffsetFactor:-3,polygonOffsetUnits:-3}),B.onBeforeCompile=I=>{I.uniforms.uHidden={value:f},I.vertexShader=I.vertexShader.replace("#include <common>",`#include <common>
attribute float aTile;
varying float vTile;
varying vec3 vWaterWorld;`).replace("#include <worldpos_vertex>",`#include <worldpos_vertex>
vTile = aTile;
vWaterWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;`),I.fragmentShader=I.fragmentShader.replace("#include <common>",`#include <common>
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
          }`)};const _=T.geometry();R=new pt(_,B),R.name="cityWater",R.receiveShadow=!1,R.castShadow=!1,R.renderOrder=3}const C=c.geometry(),H=T3(x),K=new Ye({vertexColors:!0,roughness:.92,metalness:0,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2});K.onBeforeCompile=_=>{_.uniforms.uMetre={value:r},_.uniforms.uHidden={value:f},_.uniforms.uMassLook={value:H},_.vertexShader=_.vertexShader.replace("#include <common>",`#include <common>
attribute float aTile;
attribute float aBase;
attribute float aSeed;
varying float vTile;
varying float vBase;
varying float vSeed;
varying vec3 vMassWorld;
varying vec3 vMassNormal;
`+C3).replace("#include <worldpos_vertex>",`#include <worldpos_vertex>
vTile = aTile;
vBase = aBase;
vSeed = aSeed;
vMassWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;
vMassNormal = normalize(mat3(modelMatrix) * objectNormal);
`+_3),_.fragmentShader=_.fragmentShader.replace("#include <common>",`#include <common>
`+D3+`uniform float uMetre;
uniform float uHidden[256];
varying float vTile;
varying float vBase;
varying float vSeed;
varying vec3 vMassWorld;
varying vec3 vMassNormal;
`+Tp+z3).replace("#include <color_fragment>",`if (vTile >= 0.0 && uHidden[int(vTile + 0.5)] > 0.5) discard;
        ${A3}
        // Open ground of a parcel: joints, bays, containers, stains by use.
        if (seedKind > 3.5 && vMassNormal.y > 0.9) {
          vec2 gp = vMassWorld.xz / uMetre;
          diffuseColor.rgb = groundSurface(seedKind, diffuseColor.rgb, gp, length(fwidth(gp)), fract(vSeed));
        }
        `).replace("#include <roughnessmap_fragment>",E3).replace("#include <lights_physical_fragment>",R3).replace("#include <emissivemap_fragment>",N3)},K.customProgramCacheKey=()=>"city-mass-look";const U=new pt(C,K);return U.name="cityMass",U.castShadow=!0,U.receiveShadow=!0,{mesh:U,water:R,triangles:c.triangles()+T.triangles(),setTileHidden(_,I,Z){const le=c.tileIndex.get(gu(_,I));le!==void 0&&le<Sh&&(f[le]=Z?1:0)},dispose(){C.dispose(),K.dispose(),H.dispose(),R?.geometry.dispose(),B?.dispose()}}}const $1=1.7,I3=4.5,J3=4,$3=9e3,e5=.9,t5=new Set(["KeyW","KeyA","KeyS","KeyD","KeyQ","KeyE","Space","ArrowUp","ArrowDown","ArrowLeft","ArrowRight","ShiftLeft","ShiftRight","ControlLeft","ControlRight"]);class a5{constructor(l){this.host=l;const r=(m,d)=>{window.addEventListener(m,d),this.cleanup.push(()=>window.removeEventListener(m,d))};r("keydown",m=>{!this.active||this.typing(m)||t5.has(m.code)&&(this.keys.add(m.code),m.preventDefault())}),r("keyup",m=>{this.keys.delete(m.code)}),r("blur",()=>this.keys.clear());const c=l.dom,i=m=>{this.active&&(this.dragging=!0,this.pointer=m.pointerId,c.setPointerCapture?.(m.pointerId))},f=m=>{!this.active||!this.dragging||m.pointerId!==this.pointer||(this.yaw-=m.movementX*.0026,this.pitch=Ve.clamp(this.pitch-m.movementY*.0026,-1.5,1.5))},h=m=>{m.pointerId===this.pointer&&(this.dragging=!1,this.pointer=-1,c.releasePointerCapture?.(m.pointerId))},g=m=>{this.active&&(m.preventDefault(),this.speedScale=Ve.clamp(this.speedScale*Math.exp(-m.deltaY*.0015),.05,20))};c.addEventListener("pointerdown",i),c.addEventListener("pointermove",f),c.addEventListener("pointerup",h),c.addEventListener("pointercancel",h),c.addEventListener("wheel",g,{passive:!1}),this.cleanup.push(()=>{c.removeEventListener("pointerdown",i),c.removeEventListener("pointermove",f),c.removeEventListener("pointerup",h),c.removeEventListener("pointercancel",h),c.removeEventListener("wheel",g)})}active=!1;walking=!1;groundTarget=new lt;speedMs=0;onChange=null;yaw=0;pitch=0;speedScale=1;keys=new Set;dragging=!1;pointer=-1;forward=new lt;right=new lt;euler=new Ah(0,0,0,"YXZ");cleanup=[];typing(l){const r=l.target;return!!(r&&/^(INPUT|SELECT|TEXTAREA)$/.test(r.tagName))}enter(l=!1){const r=this.host.camera;r.up.set(0,1,0),r.getWorldDirection(this.forward),this.yaw=Math.atan2(-this.forward.x,-this.forward.z),this.pitch=Math.asin(Ve.clamp(this.forward.y,-1,1)),this.active=!0,this.walking=l,this.keys.clear(),l&&this.snapToEye(),this.onChange?.()}exit(){this.active&&(this.active=!1,this.dragging=!1,this.keys.clear(),this.onChange?.())}setWalking(l){this.walking!==l&&(this.walking=l,l&&(this.pitch=Math.max(this.pitch,-.35),this.snapToEye()),this.onChange?.())}snapToEye(){const{camera:l,metresToScene:r}=this.host;l.position.y=this.host.groundHeight(l.position.x,l.position.z)+$1*r}update(l){if(!this.active)return;const{camera:r,metresToScene:c,halfExtent:i}=this.host,f=Math.min(l,.1);this.euler.set(this.pitch,this.yaw,0),r.quaternion.setFromEuler(this.euler);const h=this.host.groundHeight(r.position.x,r.position.z),g=Math.max(0,(r.position.y-h)/c),m=this.keys.has("ShiftLeft")||this.keys.has("ShiftRight")?4:1,d=this.walking?I3:Ve.clamp(g*e5,J3,$3);this.speedMs=d*m*this.speedScale;const x=this.speedMs*c*f,M=this.keys,T=(M.has("KeyW")||M.has("ArrowUp")?1:0)-(M.has("KeyS")||M.has("ArrowDown")?1:0),A=(M.has("KeyD")||M.has("ArrowRight")?1:0)-(M.has("KeyA")||M.has("ArrowLeft")?1:0),R=(M.has("Space")||M.has("KeyE")?1:0)-(M.has("KeyQ")||M.has("ControlLeft")||M.has("ControlRight")?1:0);this.walking?this.forward.set(-Math.sin(this.yaw),0,-Math.cos(this.yaw)):r.getWorldDirection(this.forward),this.right.set(-this.forward.z,0,this.forward.x),this.right.lengthSq()>1e-9&&this.right.normalize(),r.position.addScaledVector(this.forward,T*x),r.position.addScaledVector(this.right,A*x),this.walking||(r.position.y+=R*x);const B=i*.998;r.position.x=Ve.clamp(r.position.x,-B,B),r.position.z=Ve.clamp(r.position.z,-B,B);const C=this.host.groundHeight(r.position.x,r.position.z),H=$1*c;if(this.walking){const K=C+H;r.position.y+=(K-r.position.y)*Math.min(1,f*14)}else r.position.y<C+H&&(r.position.y=C+H);r.updateMatrixWorld(),this.updateGroundTarget(C,g)}updateGroundTarget(l,r){const{camera:c,metresToScene:i}=this.host;c.getWorldDirection(this.forward);const h=Math.max(30,r*14)*i;let g=h;if(this.forward.y<-1e-4){let m=0,d=Math.max(2*i,h/64),x=0;for(let M=0;M<96&&m<h;M+=1){m+=d;const T=c.position.x+this.forward.x*m,A=c.position.z+this.forward.z*m;if(c.position.y+this.forward.y*m<=this.host.groundHeight(T,A)){let B=x,C=m;for(let H=0;H<6;H+=1){const K=(B+C)*.5,U=c.position.x+this.forward.x*K,_=c.position.z+this.forward.z*K;c.position.y+this.forward.y*K<=this.host.groundHeight(U,_)?C=K:B=K}g=Math.min(h,C);break}x=m,d*=1.08}}this.groundTarget.copy(c.position).addScaledVector(this.forward,Math.max(g,1.2*i))}dispose(){for(const l of this.cleanup)l();this.cleanup.length=0}}new lt(-.58,.55,-.6).normalize();const n5=new Zt().setRGB(.2,.195,.185,gg);function o5(o,l){const r=new hp(o),c=new up,i=new pt(new Gs(100,32,20),l);i.frustumCulled=!1,c.add(i);const f=new pt(new Gs(90,32,10,0,Math.PI*2,Math.PI*.515,Math.PI*.485),new Dn({color:n5,side:fp,fog:!1}));f.renderOrder=1,f.frustumCulled=!1,c.add(f);const h=r.fromScene(c,.04).texture;return i.geometry.dispose(),f.geometry.dispose(),f.material.dispose(),r.dispose(),h}const Ga=1.6,Xl=[];for(let o=0;o<=44;o+=1)Xl.push(1+.9*Math.pow(o/44,1.6));Xl.push(2.1,2.4,2.9,3.6,4.8,7,11,19);function Ci(o,l,r=1,c=2){if(l<c*4||o.length<l*l*r)return o;const i=f=>Math.min(l-1-c,Math.max(c,f));for(let f=0;f<l;f+=1){const h=f>=c&&f<l-c;for(let g=0;g<l;g+=1){if(h&&g>=c&&g<l-c){g=l-c-1;continue}const m=(i(f)*l+i(g))*r,d=(f*l+g)*r;for(let x=0;x<r;x+=1)o[d+x]=o[m+x]}}return o}function l5(o){const{heights:l,meshSize:r,minElevation:c,metresToScene:i,map:f,waterHeights:h,waterMask:g,waterMaskSize:m}=o,d=r-1,x=(k,ee)=>Math.max(0,l[ee*r+k]-c)*i,M=(k,ee)=>{const Me=Ve.clamp((k+Ga)/(2*Ga)*d,0,d),S=Ve.clamp((ee+Ga)/(2*Ga)*d,0,d),Q=Math.floor(Me),ve=Math.floor(S),V=Math.min(d,Q+1),pe=Math.min(d,ve+1),ue=Me-Q,Ce=S-ve;return Ve.lerp(Ve.lerp(x(Q,ve),x(V,ve),ue),Ve.lerp(x(Q,pe),x(V,pe),ue),Ce)},T=k=>k<-Ga?-2*Ga-k:k>Ga?2*Ga-k:k,A=[],R=[],B=k=>-Ga+2*Ga*k/d,C=(k,ee)=>{A.push([B(k),B(ee),x(k,ee)]),R.push([k,ee])};for(let k=0;k<d;k+=1)C(k,0);for(let k=0;k<d;k+=1)C(d,k);for(let k=d;k>0;k-=1)C(k,d);for(let k=d;k>0;k-=1)C(0,k);const H=A.length,K=new Float32Array(H),U=[];for(let k=0;k<H;k+=1){const[ee,Me]=R[k],S=Math.round(ee/d*(m-1)),Q=Math.round(Me/d*(m-1));g[Q*m+S]>96&&(K[k]=1,U.push(Math.max(0,h[Me*r+ee]-c)*i))}U.sort((k,ee)=>k-ee);const _=U.length?U[Math.floor(U.length/2)]:0,I=k=>{const ee=new Float32Array(H*3+1);for(let S=0;S<H*3;S+=1)ee[S+1]=ee[S]+K[S%H];const Me=new Float32Array(H);for(let S=0;S<H;S+=1)Me[S]=(ee[H+S+k+1]-ee[H+S-k])/(2*k+1);return Me},Z=I(Math.max(2,Math.round(r/60))),le=I(Math.max(4,Math.round(r/5))),X=A.filter((k,ee)=>K[ee]===0).map(k=>k[2]).sort((k,ee)=>k-ee),se=X.length?X[Math.floor(X.length*.2)]:_,we=new Float32Array(H*Xl.length*3),ge=new Float32Array(H*Xl.length*2),G=new Float32Array(H*Xl.length*2);for(let k=0;k<Xl.length;k+=1){const ee=Xl[k];for(let Me=0;Me<H;Me+=1){const[S,Q,ve]=A[Me],V=S*ee,pe=Q*ee;let ue=ve;if(k>0){const oe=(ee-1)*.5,J=1-Ve.smoothstep(oe,0,.42);ue=se+(M(T(V),T(pe))-se)*J*J,ue=Ve.lerp(ue,_,Math.max(Z[Me],Ve.smoothstep(le[Me],.05,.3)))}G[(k*H+Me)*2]=Z[Me],G[(k*H+Me)*2+1]=le[Me];const Ce=(k*H+Me)*3;we[Ce]=V,we[Ce+1]=ue,we[Ce+2]=pe;const Ne=(k*H+Me)*2;ge[Ne]=(V+Ga)/(2*Ga),ge[Ne+1]=1-(pe+Ga)/(2*Ga)}}const F=new Uint32Array(H*(Xl.length-1)*6);let ie=0;for(let k=0;k<Xl.length-1;k+=1)for(let ee=0;ee<H;ee+=1){const Me=k*H+ee,S=k*H+(ee+1)%H,Q=Me+H,ve=S+H;F[ie++]=Me,F[ie++]=S,F[ie++]=Q,F[ie++]=S,F[ie++]=ve,F[ie++]=Q}const $=new It;$.setAttribute("position",new Jt(we,3)),$.setAttribute("uv",new Jt(ge,2)),$.setAttribute("aSea",new Jt(G,2)),$.setIndex(new Jt(F,1)),$.computeVertexNormals(),f.wrapS=f.wrapT=yg,f.needsUpdate=!0;const N=new Ye({map:f,roughness:1,metalness:0});N.onBeforeCompile=k=>{k.vertexShader=k.vertexShader.replace("#include <common>",`#include <common>
attribute vec2 aSea;
varying vec2 vLandP;
varying vec3 vLandW;
varying vec2 vSea;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vLandP = position.xz;
vLandW = position;
vSea = aSea;`),k.fragmentShader=k.fragmentShader.replace("#include <common>",`#include <common>
varying vec2 vLandP;
varying vec3 vLandW;
varying vec2 vSea;
float gSeaMask = 0.0;
float landHash(vec2 p) { p = fract(p * vec2(123.34, 456.21)); p += dot(p, p + 45.32); return fract(p.x * p.y); }
float landNoise(vec2 p) {
  vec2 i = floor(p), f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(landHash(i), landHash(i + vec2(1.0, 0.0)), f.x), mix(landHash(i + vec2(0.0, 1.0)), landHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
float landFbm(vec2 p) {
  float v = 0.0, a = 0.5;
  mat2 r = mat2(0.80, -0.60, 0.60, 0.80);
  for (int i = 0; i < 5; i++) { v += a * landNoise(p); p = r * p * 2.03 + 11.7; a *= 0.5; }
  return v;
}`).replace("#include <map_fragment>",`
{
  vec2 t = (vLandP + ${Ga.toFixed(1)}) / ${(2*Ga).toFixed(1)};
  vec2 beyond = max(max(-t, t - 1.0), 0.0);
  float outside = length(beyond); // tile widths past the edge
  // Close to the edge: the tile's own image, losing detail with distance (the
  // mirror must not read as a reflection). Far: its regional average.
  // Mirrored by hand and held off the image's own border (the generator's outlet
  // row and the wet margin shaded around it). Only its broad tones are used: the
  // terrain repaints its fields and natural ground itself, so the image's detail
  // would not match what the tile shows.
  vec2 mirrored = 0.03 + 0.94 * (1.0 - abs(1.0 - mod(vMapUv, 2.0)));
  vec3 nearColour = texture2D(map, mirrored, 4.5 + 3.5 * smoothstep(0.0, 0.10, outside)).rgb;
  vec3 region = mix(texture2D(map, mirrored, 8.0).rgb, texture2D(map, vec2(0.5), 12.0).rgb, 0.55);
  float broad = landFbm(vLandP * 1.9) * 0.7 + landFbm(vLandP * 7.3 + 4.0) * 0.3;
  region *= 0.72 + 0.56 * broad;
  vec3 land = mix(nearColour, region, smoothstep(0.04, 0.30, outside));
  // The terrain builds its natural ground darker and greyer than the baked image
  // (rock, soil and grass at their measured reflectance); match it.
  land = mix(land, vec3(dot(land, vec3(0.3, 0.55, 0.15))), 0.30) * 0.62;
  // The shoreline between a wet and a dry stretch of the edge wanders instead of
  // leaving the tile along a ruled line.
  float shore = (landFbm(vLandP * 1.7 + 7.0) - 0.5) * 1.5 * smoothstep(0.0, 0.25, vSea.y) * smoothstep(1.0, 0.75, vSea.y);
  gSeaMask = smoothstep(0.47, 0.53, mix(vSea.x, vSea.y + shore, smoothstep(0.0, 0.08, outside)));
  diffuseColor.rgb *= land * (1.0 - gSeaMask);
}`).replace("#include <emissivemap_fragment>",`#include <emissivemap_fragment>
#if defined( USE_FOG ) && ! defined( FOG_EXP2 )
if (gSeaMask > 0.0) {
  // Open sea, the same water as the tile's (Terrain3D's water shader): a dark
  // body under the mirrored sky, and the sun's glitter path.
  vec3 seaView = normalize(cameraPosition - vLandW);
  float seaFacing = max(seaView.y, 0.0);
  float seaFresnel = 0.02 + 0.98 * pow(1.0 - seaFacing, 5.0);
  vec3 seaMirror = vec3(-seaView.x, abs(seaView.y) + 0.02, -seaView.z);
  vec3 seaSky = atmSky(normalize(seaMirror), max(cameraPosition.y * fogNear, 2.0), fogFar);
  float seaSun = max(dot(reflect(-ATM_SUN, vec3(0.0, 1.0, 0.0)), seaView), 0.0);
  float seaGlitter = pow(seaSun, 90.0) * 0.35 + pow(seaSun, 900.0) * 6.0;
  vec3 sea = vec3(0.009, 0.056, 0.077) * (1.0 - seaFresnel) + seaSky * seaFresnel * 0.75
    + atmSunTransmittance(fogFar) * seaGlitter * (0.3 + 0.7 * seaFresnel);
  totalEmissiveRadiance += sea * gSeaMask;
}
#endif`)};const P=new pt($,N);return P.name="horizon-land",P.frustumCulled=!1,P.matrixAutoUpdate=!1,P.userData.noOcclusion=!0,P}function Uh(o){return 4e-6*Math.pow(30,Ve.clamp(o,0,10)/10)}const fh=o=>Number.isInteger(o)?`${o}.0`:`${o}`;function Lp(o){return`
const vec3 ATM_SUN = vec3(${fh(o.x)}, ${fh(o.y)}, ${fh(o.z)});
// Rayleigh scattering of air at sea level (1/m) for 680, 550, 440 nm, and the
// scale heights of air and of the haze layer.
const vec3 ATM_BR = vec3(5.8e-6, 13.5e-6, 33.1e-6);
const float ATM_HR = 8000.0;
const float ATM_HM = 1300.0;
// Sun irradiance in scene radiance units, set so a clear horizon sits where a
// photograph exposed for the land puts it.
const float ATM_E = 5.8;

// Relative optical air mass (Kasten and Young): 1 overhead, about 38 at the horizon.
float atmAirmass(float y) {
  float z = degrees(acos(clamp(y, 0.0, 1.0)));
  return 1.0 / (max(y, 0.0) + 0.50572 * pow(96.07995 - z, -1.6364));
}
// Colour of direct sunlight after the air above the ground.
vec3 atmSunTransmittance(float betaM) {
  return exp(-(ATM_BR * ATM_HR + vec3(betaM * ATM_HM)) * atmAirmass(ATM_SUN.y));
}
// What a long lit path converges to in direction d (unit): the horizon's colour.
vec3 atmSource(vec3 d, float betaM) {
  float c = dot(d, ATM_SUN);
  float pR = 0.0597 * (1.0 + c * c);
  // Two lobes: a tight one for the glare around the sun, a broad one for the
  // whitening of the whole sunward half of the sky.
  float g1 = 0.80, g2 = 0.35;
  float pM = 0.0796 * (0.55 * (1.0 - g1 * g1) / pow(1.0 + g1 * g1 - 2.0 * g1 * c, 1.5)
                     + 0.45 * (1.0 - g2 * g2) / pow(1.0 + g2 * g2 - 2.0 * g2 * c, 1.5));
  vec3 tR = ATM_BR * ATM_HR;
  vec3 tM = vec3(betaM * ATM_HM);
  // The sun reaches the scattering air already reddened; multiple scattering puts
  // most of the blue back, so only part of that tint is applied.
  // Strongly toward the sun (the glow of a low sun is the colour of the sun),
  // hardly at all across the sky from it.
  vec3 sun = pow(atmSunTransmittance(betaM), vec3(0.10 + 0.55 * pow(max(c, 0.0), 1.5)));
  vec3 single = (tR * pR + tM * pM) / (tR + tM);
  // Light scattered more than once: isotropic, the colour of the open sky.
  // Generous: it is most of what makes a clear sky luminous overhead instead of navy.
  vec3 multiple = vec3(0.011, 0.028, 0.052) * (0.6 + 0.4 * ATM_SUN.y);
  return ATM_E * sun * (single + multiple);
}
// Optical depth (Rayleigh rgb, Mie a) of the segment from height h0 (m) along a
// ray climbing dy per metre for dist metres, in an exponential atmosphere.
vec4 atmSegment(float h0, float dy, float dist, float betaM) {
  float h1 = max(h0 + dy * dist, 0.0);
  float sR, sM;
  if (abs(dy) < 1.0e-3) {
    sR = exp(-h0 / ATM_HR) * dist;
    sM = exp(-h0 / ATM_HM) * dist;
  } else {
    sR = ATM_HR * (exp(-h0 / ATM_HR) - exp(-h1 / ATM_HR)) / dy;
    sM = ATM_HM * (exp(-h0 / ATM_HM) - exp(-h1 / ATM_HM)) / dy;
  }
  return vec4(ATM_BR * sR, betaM * sM);
}
// Sky radiance in direction d seen from height h0 (m).
vec3 atmSky(vec3 d, float h0, float betaM) {
  float m = atmAirmass(d.y);
  vec3 up = (ATM_BR * ATM_HR * exp(-h0 / ATM_HR) + vec3(betaM * ATM_HM * exp(-h0 / ATM_HM))) * m;
  // Below the horizon the ray ends on the ground: all the air beneath the eye.
  float slope = max(-d.y, 1.0e-4);
  vec3 down = (ATM_BR * ATM_HR * (1.0 - exp(-h0 / ATM_HR)) + vec3(betaM * ATM_HM * (1.0 - exp(-h0 / ATM_HM)))) / slope
    + (ATM_BR * ATM_HR + vec3(betaM * ATM_HM)) * 38.0 * exp(-h0 / 2500.0);
  // Blend what is seen, not the depths: the depth below the horizon is all but
  // infinite and would hold the blend at "saturated" until its last pixel.
  vec3 filled = mix(1.0 - exp(-down), 1.0 - exp(-up), smoothstep(-0.02, 0.035, d.y));
  return atmSource(d, betaM) * filled;
}
// Surface radiance after the air between it and the eye. rayDir is unit, from the
// eye; dist in metres; h0 the eye's height in metres.
vec3 atmAerial(vec3 colour, vec3 rayDir, float dist, float h0, float betaM, out vec3 airlight, out vec3 transmittance) {
  vec4 seg = atmSegment(max(h0, 0.0), rayDir.y, dist, betaM);
  transmittance = exp(-(seg.rgb + vec3(seg.a)));
  airlight = atmSource(rayDir, betaM);
  // Looking down from high up, the whole Rayleigh column lies over the land as a
  // blue veil. Every high-altitude camera filters most of it out; so does this
  // one, increasingly with height. Level views are untouched, so the far ground
  // still meets the sky at the sky's own colour.
  airlight *= mix(1.0, 0.45, smoothstep(1500.0, 14000.0, h0) * smoothstep(0.0, -0.2, rayDir.y));
  return colour * transmittance + airlight * (1.0 - transmittance);
}
`}let ep=null;function s5(o){const l=o.toArray().join(",");if(ep===l)return;ep=l;const r=mu;r.fog_pars_vertex=`
#ifdef USE_FOG
  varying float vFogDepth;
  varying vec3 vFogView;
#endif
`,r.fog_vertex=`
#ifdef USE_FOG
  vFogDepth = - mvPosition.z;
  vFogView = mvPosition.xyz;
#endif
`,r.fog_pars_fragment=`
#ifdef USE_FOG
  uniform vec3 fogColor;
  varying float vFogDepth;
  varying vec3 vFogView;
  #ifdef FOG_EXP2
    uniform float fogDensity;
  #else
    // near: metres per scene unit. far: haze (Mie) coefficient per metre.
    uniform float fogNear;
    uniform float fogFar;
    ${Lp(o)}
  #endif
#endif
`,r.fog_fragment=`
#ifdef USE_FOG
  #ifdef FOG_EXP2
    float fogFactor = 1.0 - exp( - fogDensity * fogDensity * vFogDepth * vFogDepth );
    gl_FragColor.rgb = mix( gl_FragColor.rgb, fogColor, fogFactor );
  #else
    // The orthographic satellite view looks straight down through the whole
    // column from nowhere in particular: its haze belongs to the image, not here.
    if ( ! isOrthographic ) {
      vec3 fogRay = normalize( ( vec4( vFogView, 0.0 ) * viewMatrix ).xyz );
      vec3 fogAir, fogT;
      atmAerial( vec3( 0.0 ), fogRay, length( vFogView ) * fogNear, cameraPosition.y * fogNear, fogFar, fogAir, fogT );
      // Drawn straight to the screen, the surface is already tone mapped and
      // encoded at this point, so the air has to be too.
      #ifdef TONE_MAPPING
        fogAir = toneMapping( fogAir );
      #endif
      fogAir = linearToOutputTexel( vec4( fogAir, 1.0 ) ).rgb;
      gl_FragColor.rgb = gl_FragColor.rgb * fogT + fogAir * ( 1.0 - fogT );
    }
  #endif
#endif
`}function i5(o,l){return new xg(16777215,o,Uh(l))}const r5=`
  varying vec3 vDir;
  void main() {
    vDir = position;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`,c5=o=>`
  varying vec3 vDir;
  uniform float uTime;
  uniform float uBetaM;
  uniform float uCameraHeight;
  uniform float uCoverage;
  uniform float uAmbientBake;
  ${Lp(o)}

  float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
  float vnoise(vec2 p) {
    vec2 i = floor(p), f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float v = 0.0, a = 0.5;
    mat2 r = mat2(0.80, -0.60, 0.60, 0.80);
    for (int i = 0; i < 6; i++) { v += a * vnoise(p); p = r * p * 2.07 + vec2(19.7, 7.3); a *= 0.5; }
    return v;
  }
  // Cloud density over the layer's plane (unit: about 1.5 km).
  float cloudField(vec2 uv, vec2 drift) {
    float warp = fbm(uv * 0.18 + drift);
    float broad = fbm(uv * 0.30 + (warp - 0.5) * 2.2 + drift * 1.7);
    float detail = fbm(uv * 1.9 + warp * 3.0 - drift * 2.3);
    return broad * 0.80 + detail * 0.20;
  }

  void main() {
    vec3 d = normalize(vDir);
    float y = d.y;
    vec3 col = atmSky(d, uCameraHeight, uBetaM);

    // The disc: bright enough to bloom, coloured by the air it came through.
    float s = max(dot(d, ATM_SUN), 0.0);
    vec3 sunColour = atmSunTransmittance(uBetaM);
    col += sunColour * (smoothstep(0.99989, 0.99996, s) * 90.0 + pow(s, 1800.0) * 3.0) * step(0.0, y);

    // Cumulus on a plane overhead, seen from below. From the stratosphere the
    // layer is underneath, where the terrain view draws its own.
    float below = 1.0 - smoothstep(2500.0, 7000.0, uCameraHeight);
    if (y > 0.004 && below > 0.0) {
      vec2 uv = d.xz / (y + 0.10) * 3.2;
      vec2 drift = vec2(uTime * 0.0022, uTime * 0.0009);
      float f = cloudField(uv, drift);
      float edge = mix(0.72, 0.40, uCoverage);
      float cover = smoothstep(edge, edge + 0.20, f);
      if (cover > 0.001) {
        // Light through the cloud: sample the field a step toward the sun; a
        // thick neighbour there puts this spot in shade (grey base, lit rim).
        vec2 toSun = normalize(ATM_SUN.xz - d.xz * (ATM_SUN.y / max(y, 0.05)) + 1.0e-4);
        float toward = cloudField(uv + toSun * 0.55, drift);
        float thickness = smoothstep(edge, edge + 0.34, f);
        float shade = clamp(1.0 - 1.15 * smoothstep(edge - 0.05, edge + 0.30, toward) * thickness, 0.0, 1.0);
        vec3 ambient = atmSky(vec3(0.0, 1.0, 0.0), uCameraHeight, uBetaM) * 1.35 + atmSource(vec3(d.x, 0.0, d.z), uBetaM) * 0.30;
        vec3 lit = sunColour * (0.95 + 1.6 * pow(s, 8.0) * (1.0 - thickness));
        vec3 cloud = ambient * (0.55 + 0.45 * (1.0 - thickness)) + lit * (0.16 + 0.84 * shade);
        // The layer is 2 km up: toward the horizon it is tens of kilometres of
        // air away and goes the way of everything else that far.
        float dist = 2000.0 / (y + 0.02);
        vec3 airlight, trans;
        cloud = atmAerial(cloud, d, dist, uCameraHeight, uBetaM, airlight, trans);
        col = mix(col, cloud, cover * 0.96 * below * smoothstep(0.004, 0.05, y));
      }
    }

    // Baked as the scene's ambient light, the sky stands for all the light that
    // is not the sun: itself, but also what the ground and every sunlit surface
    // send back up, which is not blue. A single-scattering sky alone lit every
    // shadow a saturated blue.
    col = mix(col, vec3(dot(col, vec3(0.2126, 0.7152, 0.0722))) * 0.78, 0.5 * uAmbientBake);
    col *= mix(1.0, 0.70, uAmbientBake);
    gl_FragColor = vec4(col, 1.0);
    #include <tonemapping_fragment>
    #include <colorspace_fragment>
  }
`;function u5(o,l,r){const c={uTime:{value:0},uBetaM:{value:Uh(l)},uCameraHeight:{value:2},uCoverage:{value:r},uAmbientBake:{value:0}};return{material:new Pa({vertexShader:r5,fragmentShader:c5(o),uniforms:c,side:fp,depthWrite:!1,depthTest:!1,fog:!1}),uniforms:c}}function f5(){const o=new kh({name:"LensShader",uniforms:{tDiffuse:{value:null},uTime:{value:0}},vertexShader:`
      varying vec2 vUv;
      void main() {
        vUv = uv;
        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
      }`,fragmentShader:`
      uniform sampler2D tDiffuse;
      uniform float uTime;
      varying vec2 vUv;
      float grainHash(vec2 p) {
        vec3 q = fract(vec3(p.xyx) * 0.1031);
        q += dot(q, q.yzx + 33.33);
        return fract((q.x + q.y) * q.z);
      }
      void main() {
        vec2 fromCentre = vUv - 0.5;
        float r2 = dot(fromCentre, fromCentre);
        // Lateral chromatic aberration: red and blue magnified a hair differently.
        vec2 spread = fromCentre * r2 * 0.0045;
        vec3 colour = vec3(
          texture2D(tDiffuse, vUv + spread).r,
          texture2D(tDiffuse, vUv).g,
          texture2D(tDiffuse, vUv - spread).b
        );
        // The print: a film's shoulder and toe on top of the tone curve (deeper
        // shadows, highlights that keep rolling), and a little more colour.
        colour = clamp(colour, 0.0, 1.0);
        vec3 curved = colour * colour * (3.0 - 2.0 * colour);
        colour = mix(colour, curved, 0.26);
        float grey = dot(colour, vec3(0.2126, 0.7152, 0.0722));
        colour = mix(vec3(grey), colour, 1.10);
        // Natural vignetting, under half a stop at the corners.
        colour *= mix(1.0, 0.74, smoothstep(0.08, 0.62, r2));
        // Grain, re-drawn each frame; least in the highlights.
        float luma = dot(colour, vec3(0.2126, 0.7152, 0.0722));
        float grain = grainHash(gl_FragCoord.xy + fract(uTime) * 517.0) - 0.5;
        colour += grain * 0.030 * (1.0 - 0.7 * luma);
        gl_FragColor = vec4(colour, 1.0);
      }`});return{pass:o,update(l){o.uniforms.uTime.value=l}}}function h5(o,l,r=256){const c=new wg(r,{type:rl,generateMipmaps:!1}),i=new bg(.01,30,c);i.coordinateSystem=o.coordinateSystem,i.updateCoordinateSystem();const f=new hp(o);let h=null,g=-1;const m=new lt(Number.NaN,0,0);let d=-1/0;return{update(x,M,T,A,R,B){if(!x)return g=-1,m.x=Number.NaN,null;if(g<0&&(Number.isNaN(m.x)||m.distanceTo(M)>T||B-d>5)){g=0,i.position.copy(M);for(const H of i.children)H.near=A,H.far=R,H.updateProjectionMatrix();i.updateMatrixWorld(!0)}if(g>=0&&g<6){const C=o.getRenderTarget(),H=o.getActiveCubeFace(),K=o.getActiveMipmapLevel(),U=o.xr.enabled;o.xr.enabled=!1,o.setRenderTarget(c,g),o.render(l,i.children[g]),o.setRenderTarget(C,H,K),o.xr.enabled=U,g+=1}else if(g===6){const C=f.fromCubemap(c.texture);h?.dispose(),h=C,m.copy(i.position),d=B,g=-1}return h?h.texture:null},dispose(){h?.dispose(),c.dispose(),f.dispose()}}}function Nn(o,l=!1){const r=o[0].index!==null,c=new Set(Object.keys(o[0].attributes)),i=new Set(Object.keys(o[0].morphAttributes)),f={},h={},g=o[0].morphTargetsRelative,m=new It;let d=0;for(let x=0;x<o.length;++x){const M=o[x];let T=0;if(r!==(M.index!==null))return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+x+". All geometries must have compatible attributes; make sure index attribute exists among all geometries, or in none of them."),null;for(const A in M.attributes){if(!c.has(A))return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+x+'. All geometries must have compatible attributes; make sure "'+A+'" attribute exists among all geometries, or in none of them.'),null;f[A]===void 0&&(f[A]=[]),f[A].push(M.attributes[A]),T++}if(T!==c.size)return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+x+". Make sure all geometries have the same number of attributes."),null;if(g!==M.morphTargetsRelative)return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+x+". .morphTargetsRelative must be consistent throughout all geometries."),null;for(const A in M.morphAttributes){if(!i.has(A))return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+x+".  .morphAttributes must be consistent throughout all geometries."),null;h[A]===void 0&&(h[A]=[]),h[A].push(M.morphAttributes[A])}if(l){let A;if(r)A=M.index.count;else if(M.attributes.position!==void 0)A=M.attributes.position.count;else return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed with geometry at index "+x+". The geometry must have either an index or a position attribute"),null;m.addGroup(d,A,x),d+=A}}if(r){let x=0;const M=[];for(let T=0;T<o.length;++T){const A=o[T].index;for(let R=0;R<A.count;++R)M.push(A.getX(R)+x);x+=o[T].attributes.position.count}m.setIndex(M)}for(const x in f){const M=tp(f[x]);if(!M)return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed while trying to merge the "+x+" attribute."),null;m.setAttribute(x,M)}for(const x in h){const M=h[x][0].length;if(M!==0){m.morphAttributes=m.morphAttributes||{},m.morphAttributes[x]=[];for(let T=0;T<M;++T){const A=[];for(let B=0;B<h[x].length;++B)A.push(h[x][B][T]);const R=tp(A);if(!R)return console.error("THREE.BufferGeometryUtils: .mergeGeometries() failed while trying to merge the "+x+" morphAttribute."),null;m.morphAttributes[x].push(R)}}}return m}function tp(o){let l,r,c,i=-1,f=0;for(let d=0;d<o.length;++d){const x=o[d];if(l===void 0&&(l=x.array.constructor),l!==x.array.constructor)return console.error("THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.array must be of consistent array types across matching attributes."),null;if(r===void 0&&(r=x.itemSize),r!==x.itemSize)return console.error("THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.itemSize must be consistent across matching attributes."),null;if(c===void 0&&(c=x.normalized),c!==x.normalized)return console.error("THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.normalized must be consistent across matching attributes."),null;if(i===-1&&(i=x.gpuType),i!==x.gpuType)return console.error("THREE.BufferGeometryUtils: .mergeAttributes() failed. BufferAttribute.gpuType must be consistent across matching attributes."),null;f+=x.count*r}const h=new l(f),g=new Jt(h,r,c);let m=0;for(let d=0;d<o.length;++d){const x=o[d];if(x.isInterleavedBufferAttribute){const M=m/r;for(let T=0,A=x.count;T<A;T++)for(let R=0;R<r;R++){const B=x.getComponent(T,R);g.setComponent(T+M,R,B)}}else h.set(x.array,m);m+=x.count*r}return i!==void 0&&(g.gpuType=i),g}function jp(o){const l=atob(o),r=new Uint8Array(l.length);for(let c=0;c<l.length;c+=1)r[c]=l.charCodeAt(c);return r}function hh(o){const l=jp(o);return new Float32Array(l.buffer,l.byteOffset,l.byteLength>>2)}function dh(o,l,r){const c=[];for(const f of o??[]){if(f.lod!==l)continue;const h=hh(f.positions),g=f.heightMetres*r,m=new Float32Array(h.length);for(let M=0;M<h.length;M+=1)m[M]=h[M]*g;const d=jp(f.indices),x=new It;x.setAttribute("position",new Jt(m,3)),x.setAttribute("normal",new Jt(hh(f.normals).slice(),3)),x.setAttribute("color",new Jt(hh(f.colors).slice(),3)),x.setIndex(new Jt(new Uint16Array(d.buffer.slice(d.byteOffset,d.byteOffset+d.byteLength)),1)),x.computeBoundingSphere(),c.push(x)}if(!c.length){const f=new Mg(3.8*r,18*r,5,1);f.translate(0,9*r,0);const h=new Float32Array(f.attributes.position.count*3);for(let g=0;g<h.length;g+=3)h[g]=.02,h[g+1]=.05,h[g+2]=.02;f.setAttribute("color",new Jt(h,3)),c.push(f)}const i=new Ye({vertexColors:!0,roughness:.94,metalness:0});return{geometries:c,material:i,dispose(){for(const f of c)f.dispose();i.dispose()}}}function d5(o){let l=o>>>0;return()=>(l=Math.imul(l,1664525)+1013904223>>>0,l/4294967296)}function m5(o){const l=d5(2135587861),r=[],c=[],i=[],f=9;for(let g=0;g<f;g+=1){const m=g/f*Math.PI*2+l()*.6,d=(.2+l()*.24)*o,x=(.011+l()*.008)*o,M=(.05+l()*.16)*o,T=l()*.05*o,A=Math.cos(m),R=Math.sin(m),B=-R,C=A,H=A*T,K=R*T,U=.85+l()*.3,_=3,I=[];for(let le=0;le<=_;le+=1){const X=le/_;I.push({x:H+A*M*X*X,y:d*X-d*.1*X*X,z:K+R*M*X*X,w:x*(1-X*X*.92),v:(.38+.62*X)*U})}const Z=(le,X)=>{r.push(le.x+B*le.w*X,le.y,le.z+C*le.w*X);const se=new lt(A*.35+B*X*.25,1,R*.35+C*X*.25).normalize();c.push(se.x,se.y,se.z),i.push(le.v*.9,le.v,le.v*(.62+.2*(1-le.v)))};for(let le=0;le<_;le+=1){const X=I[le],se=I[le+1];le===_-1?(Z(X,-1),Z(X,1),Z(se,0)):(Z(X,-1),Z(X,1),Z(se,1),Z(X,-1),Z(se,1),Z(se,-1))}}const h=new It;return h.setAttribute("position",new ft(r,3)),h.setAttribute("normal",new ft(c,3)),h.setAttribute("color",new ft(i,3)),h}function p5(o){const l=new Gs(.5*o,11,8),r=l.attributes.position,c=new Float32Array(r.count*3);for(let i=0;i<r.count;i+=1){const f=r.getX(i)/o,h=r.getY(i)/o,g=r.getZ(i)/o,m=.86+.2*Math.sin(f*19+g*7)*Math.cos(g*17-h*9)+.1*Math.sin(h*31+f*23);r.setXYZ(i,f*m*1.15*o,Math.max(-.05,h*m*.78+.3)*o,g*m*1.15*o);const d=Ve.clamp(h+.5,0,1),x=.5+.5*Math.sin(f*57+h*43)*Math.sin(g*61-h*37),M=.42+.5*d+.3*x;c[i*3]=M*.82,c[i*3+1]=M,c[i*3+2]=M*.55}return l.setAttribute("color",new Jt(c,3)),l.computeVertexNormals(),l}function v5(o){const l=[],r=[],c=.24*o,i=.006*o;for(const[m,d]of[[1,0],[0,1]]){l.push(-m*i,0,-d*i,m*i,0,d*i,m*i*.6,c,d*i*.6),l.push(-m*i,0,-d*i,m*i*.6,c,d*i*.6,-m*i*.6,c,-d*i*.6);for(let x=0;x<6;x+=1)r.push(.16,.42,.06)}const f=.034*o;for(let m=0;m<5;m+=1){const d=m/5*Math.PI*2,x=(m+.5)/5*Math.PI*2,M=(m+1)/5*Math.PI*2;l.push(0,c+.004*o,0),l.push(Math.cos(d)*f*.6,c+.01*o,Math.sin(d)*f*.6),l.push(Math.cos(x)*f,c+.012*o,Math.sin(x)*f),l.push(0,c+.004*o,0),l.push(Math.cos(x)*f,c+.012*o,Math.sin(x)*f),l.push(Math.cos(M)*f*.6,c+.01*o,Math.sin(M)*f*.6);for(let T=0;T<6;T+=1)r.push(T%3===0?.55:1,T%3===0?.42:1,T%3===0?.1:1)}const h=new It;h.setAttribute("position",new ft(l,3)),h.setAttribute("color",new ft(r,3)),h.computeVertexNormals();const g=h.attributes.normal;for(let m=0;m<g.count;m+=1)g.setXYZ(m,0,1,0);return h}function g5(o,l){const r=new _a,c=[],i=[],f=(se,we,ge)=>{const G=new Ye({vertexColors:!0,roughness:ge,metalness:0,side:ct,transparent:!1});c.push(G),i.push(se);const F=new Va(se,G,we);return F.instanceMatrix.setUsage(su),F.count=0,F.frustumCulled=!1,F.castShadow=!1,F.receiveShadow=!0,r.add(F),{mesh:F,capacity:we,count:0}},h=f(m5(o),4e4,1),g=f(p5(o),4e3,.95),m=f(v5(o),9e3,.8),d=[h,g,m];r.visible=!1;let x=Number.POSITIVE_INFINITY,M=Number.POSITIVE_INFINITY,T=0,A=0,R=0;const B=new Co,C=new iu,H=new Ah,K=new lt,U=new lt,_=new Zt,I=new lt(0,1,0),Z=[16052454,16177226,14243215,9071568,15764028],le=(se,we,ge)=>se.count>=se.capacity?!1:(se.mesh.setMatrixAt(se.count,we),se.mesh.setColorAt(se.count,ge),se.count+=1,!0),X=(se,we,ge,G)=>{const F=se/o*we,ie=Ve.clamp(F*.62,10,130),$=ie<=22?.42:ie<=50?.75:ie<=90?1.25:1.9,N=Math.hypot(ge-x,G-M)/o,P=Math.abs(ie-T)/Math.max(1,T);if(N<$*4&&P<.12&&A===$)return;x=ge,M=G,T=ie,A=$;for(const Ne of d)Ne.count=0;const k=(x+1.6)/o,ee=(M+1.6)/o,Me=Math.floor((k-ie)/$),S=Math.ceil((k+ie)/$),Q=Math.floor((ee-ie)/$),ve=Math.ceil((ee+ie)/$),V=l.hash01,ue=Math.max(1.5,$)*o,Ce=34;for(let Ne=Q;Ne<=ve;Ne+=1)for(let oe=Me;oe<=S;oe+=1){const J=(oe+.08+V(oe,Ne,101)*.84)*$,_e=(Ne+.08+V(oe,Ne,102)*.84)*$,Je=J-k,O=_e-ee,Pe=Math.hypot(Je,O);if(Pe>ie)continue;const Ie=-1.6+J*o,tt=-1.6+_e*o;if(Ie<=-1.6||Ie>=1.6||tt<=-1.6||tt>=1.6||l.excluded(Ie,tt)>48||l.water(Ie,tt)>24||l.elevation01(Ie,tt)>.8)continue;const Ft=l.height(Ie,tt),$t=Math.hypot(l.height(Ie+ue,tt)-l.height(Ie-ue,tt),l.height(Ie,tt+ue)-l.height(Ie,tt-ue))/(2*ue);if($t>.85)continue;const Yt=l.forest(Ie,tt)/255,Aa=Math.floor(J/Ce),mn=Math.floor(_e/Ce),Ea=V(Aa,mn,111),Dt=V(Aa,mn,112)>.9,_o=V(Aa,mn,113)>.86,sa=1-Ve.smoothstep(Pe,ie*.72,ie);if(sa<=.02)continue;const Ra=(Dt?.1:1)*(1-Ve.smoothstep($t,.35,.85))*(1-Yt*.45);if(V(oe,Ne,103)>Ra*sa)continue;const Ya=V(oe,Ne,104)*Math.PI*2,pn=(V(oe,Ne,105)-.5)*.22;C.setFromEuler(H.set(pn,Ya,pn*.7));const Na=(.7+V(oe,Ne,106)*.85)*(.6+.4*sa)*(1.12-Ea*.32);if(K.set(Ie,Ft,tt),U.set(Na*(.85+V(oe,Ne,107)*.5),Na,Na*(.85+V(oe,Ne,108)*.5)),B.compose(K,C,U),_.setHSL(.245-Ea*.125+(V(oe,Ne,109)-.5)*.03,.34+(1-Ea)*.18,.25+V(oe,Ne,110)*.14+Ea*.05),le(h,B,_),!Dt&&Ea<.55&&Yt<.5&&V(oe,Ne,120)>.93&&sa>.4){const Ia=V(oe,Ne,121)*Math.PI*2;C.setFromAxisAngle(I,Ia),K.set(Ie+(V(oe,Ne,122)-.5)*$,Ft,tt+(V(oe,Ne,123)-.5)*$);const gn=.8+V(oe,Ne,124)*.7;U.set(gn,gn,gn),B.compose(K,C,U),_.setHex(Z[Math.floor(V(Aa,mn,125)*Z.length)%Z.length]),le(m,B,_)}const vn=(_o?.16:.006)+Yt*.03;if(!Dt&&V(oe,Ne,130)<vn&&sa>.55&&$t<.6){const Ia=(.7+V(oe,Ne,131)*1.5)*(_o?1.1:.9);C.setFromAxisAngle(I,V(oe,Ne,132)*Math.PI*2),K.set(Ie,Ft-.04*o,tt),U.set(Ia*(.9+V(oe,Ne,133)*.4),Ia*(.75+V(oe,Ne,134)*.5),Ia*(.9+V(oe,Ne,135)*.4)),B.compose(K,C,U),_.setHSL(.27-Ea*.09+(V(oe,Ne,136)-.5)*.05,.4,.13+V(oe,Ne,137)*.09),le(g,B,_)}}for(const Ne of d)Ne.mesh.count=Ne.count,Ne.mesh.instanceMatrix.needsUpdate=!0,Ne.mesh.instanceColor&&(Ne.mesh.instanceColor.needsUpdate=!0)};return{group:r,setFade(se){R=se,r.visible=R>.001},update(se,we,ge,G){R<=.001||X(se,we,ge,G)},dispose(){for(const se of i)se.dispose();for(const se of c)se.dispose();r.removeFromParent()}}}const du=.21,Mo=[1040,560],Fp=12,Th=6,Gp=2.5;var zi=(o=>(o[o.Cotton=0]="Cotton",o[o.Maize=1]="Maize",o[o.Wheat=2]="Wheat",o[o.Stubble=3]="Stubble",o[o.Soil=4]="Soil",o[o.Alfalfa=5]="Alfalfa",o))(zi||{});function nu(o){return o-Math.floor(o)}function mh(o,l){let r=nu(o*.1031),c=nu(l*.1031),i=nu(o*.1031);const f=r*(c+33.33)+c*(i+33.33)+i*(r+33.33);return r+=f,c+=f,i+=f,nu((r+c)*i)}function y5(o){return o<.34?0:o<.54?1:o<.68?2:o<.78?3:o<.86?4:5}function ap(o,l){const r=Math.cos(du),c=Math.sin(du),i=r*o+c*l,f=-c*o+r*l,h=Math.floor(i/Mo[0]),g=Math.floor(f/Mo[1]),m=i-h*Mo[0],d=f-g*Mo[1],x=3+Math.floor(mh(h,g)*3),M=Mo[1]/x,T=Math.floor(d/M),A=mh(h+17,g-5)>.5,R=A&&m>Mo[0]*.5?1:0,B=y5(mh(h*8+T,g*8+R*4+1)),C=Math.min(d,Mo[1]-d),H=Math.min(Math.abs(d-T*M),Math.abs(d-(T+1)*M)),K=Math.min(m,Mo[0]-m,A?Math.abs(m-Mo[0]*.5):1/0),U=C<Fp+Th||H<Gp*.5+.6||K<Th*.5+.6;return{crop:B,margin:U,rowAngle:-du}}const x5=`
const float FARM_ANGLE = ${du.toFixed(4)};
const vec2 FARM_BLOCK = vec2(${Mo[0].toFixed(1)}, ${Mo[1].toFixed(1)});
const float FARM_BELT = ${Fp.toFixed(1)};
const float FARM_ROAD = ${Th.toFixed(1)};
const float FARM_BUND = ${Gp.toFixed(1)};
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
`;class Wl{positions=[];indices=[];quad(l,r,c,i){const f=this.positions.length/3;this.positions.push(...l,...r,...c,...i),this.indices.push(f,f+1,f+2,f,f+2,f+3)}geometry(){const l=new It;return l.setAttribute("position",new ft(this.positions,3)),l.setIndex(this.indices),l.computeVertexNormals(),l}get empty(){return this.positions.length===0}}function ou(o,l,r,c,i){o.quad(l,r,c,i),o.quad(i,c,r,l)}function w5(o,l){const r=new _a,c=[],i=l.worldSizeKm*.5,f=new Sg({color:2828840});for(const h of o){const g=[];h.pathKm.forEach(([x,M],T)=>{const A=(x-i)*1e3*l.metresToScene,R=(M-i)*1e3*l.metresToScene;g.push(A,l.terrainHeightAt(A,R)+(Math.max(h.fillM[T],0)+6)*l.metresToScene,R)});const m=new It;m.setAttribute("position",new ft(g,3)),c.push(m);const d=new Tg(m,f);d.frustumCulled=!1,r.add(d)}return{group:r,geometries:c,materials:[f]}}function b5(o,l){const r=new _a,c=[],i=[],f=l.metresToScene,h=l.worldSizeKm*.5,g=l.layer(8223346,-3),m=l.layer(7038033,-2,{side:ct}),d=l.layer(11118238,-3,{side:ct}),x=l.layer(4934992,-5,{metalness:.6,roughness:.4}),M=l.layer(12170926,-4),T=l.layer(14014682,-4,{side:ct}),A=l.layer(13618372,-3,{side:ct});i.push(g,m,d,x,M,T,A);for(const R of o){const B=R.pathKm.length;if(B<2)continue;const C=R.pathKm.map(([$,N],P)=>{const k=($-h)*1e3*f,ee=(N-h)*1e3*f,Me=l.terrainHeightAt(k,ee),S=Math.max(R.designM[P]-R.groundM[P],0);return{x:k,z:ee,y0:Me,fill:S,structure:R.structure[P],s:R.sM[P]}}),H=C.map(($,N)=>{const P=C[Math.max(0,N-1)],k=C[Math.min(B-1,N+1)],ee=Math.max(1e-12,Math.hypot(k.x-P.x,k.z-P.z));return{tx:(k.x-P.x)/ee,tz:(k.z-P.z)/ee}}),K=($,N,P)=>{const k=C[$],ee=H[$];return[k.x+-ee.tz*N*f,k.y0+(k.fill+P)*f,k.z+ee.tx*N*f]},U=new Wl,_=new Wl,I=new Wl,Z=new Wl,le=new Wl,X=new Wl,se=new Wl,we=R.formationWidthM*.5,ge=R.trackSpacingM*.5;for(let $=0;$+1<B;$+=1){const N=$+1,P=C[$].structure>=3&&C[$].structure<=4,k=P?.5:.3;U.quad(K($,-we,k),K($,we,k),K(N,we,k),K(N,-we,k));for(const ee of[-1,1]){if(!P&&C[$].fill>.4){const Me=C[$].fill*1.5+.6,S=C[N].fill*1.5+.6,Q=(ve,V)=>[K(ve,ee*(we+V),0)[0],C[ve].y0-.2*f,K(ve,ee*(we+V),0)[2]];_.quad(K($,ee*we,k),Q($,Me),Q(N,S),K(N,ee*we,k))}if(P){const Me=S=>Math.max(0,Math.min(C[S].fill,2.5));I.quad(K($,ee*we,k),K(N,ee*we,k),K(N,ee*we,-Me(N)),K($,ee*we,-Me($)))}for(const Me of[-1,1]){const S=ge*ee+Me*.75;Z.quad(K($,S-.06,k+.2),K($,S+.06,k+.2),K(N,S+.06,k+.2),K(N,S-.06,k+.2))}}}const G=new Wl;let F=Math.ceil(C[0].s/32.6)*32.6;for(let $=0;$+1<B;$+=1)for(;F<=C[$+1].s;){if(C[$].structure>=3&&C[$].structure<=4&&C[$].fill>3){const N=(F-C[$].s)/Math.max(1e-6,C[$+1].s-C[$].s),P=C[$].x+(C[$+1].x-C[$].x)*N,k=C[$].z+(C[$+1].z-C[$].z)*N,ee=l.terrainHeightAt(P,k),Me=ee+(C[$].fill-.5)*f,S=H[$].tx,Q=H[$].tz,ve=(V,pe,ue)=>[P+S*V*f-Q*pe*f,ue,k+Q*V*f+S*pe*f];for(const[V,pe,ue,Ce]of[[-1.3,-3,1.3,-3],[1.3,-3,1.3,3],[1.3,3,-1.3,3],[-1.3,3,-1.3,-3]])G.quad(ve(V,pe,ee),ve(ue,Ce,ee),ve(ue,Ce,Me),ve(V,pe,Me))}F+=32.6}for(const $ of R.stations){const N=$.plan.sM,P=$.plan.platformLengthM;let k=C.findIndex(O=>O.s>=N-P/2),ee=C.findIndex(O=>O.s>=N+P/2);if(k<0||ee<0)continue;k=Math.max(0,k),ee=Math.max(k+1,ee);const Me=$.plan.tracks>=4,S=Me?[[-16.25,-4.25],[4.25,16.25]]:[[-(ge+9.75),-(ge+1.75)],[ge+1.75,ge+9.75]];for(let O=k;O<ee;O+=1)for(const[Pe,Ie]of S)le.quad(K(O,Pe,1.35),K(O,Ie,1.35),K(O+1,Ie,1.35),K(O+1,Pe,1.35)),X.quad(K(O,Pe-1,6.5),K(O,Ie+1,6.5),K(O+1,Ie+1,6.5),K(O+1,Pe-1,6.5));const Q=Math.floor((k+ee)/2),ve=$.plan.buildingSide>=0?1:-1,V=(Me?38:ge+12+20)*ve,pe=Me?150:60,ue=Me?38:20,Ce=Me?17:8.5,Ne=K(Q,V,0),oe=H[Q],J=(O,Pe,Ie)=>[Ne[0]+oe.tx*O*f-oe.tz*Pe*f,Ie,Ne[2]+oe.tz*O*f+oe.tx*Pe*f],_e=C[Q].y0,Je=[[-pe/2,-ue/2],[pe/2,-ue/2],[pe/2,ue/2],[-pe/2,ue/2]];for(let O=0;O<4;O+=1){const[Pe,Ie]=Je[O],[tt,Nt]=Je[(O+1)%4];se.quad(J(Pe,Ie,_e),J(tt,Nt,_e),J(tt,Nt,_e+Ce*f),J(Pe,Ie,_e+Ce*f))}se.quad(J(-pe/2-1,-ue/2-1,_e+Ce*f),J(pe/2+1,-ue/2-1,_e+Ce*f),J(pe/2+1,ue/2+1,_e+Ce*f),J(-pe/2-1,ue/2+1,_e+Ce*f))}const ie=($,N,P)=>{if($.empty)return;const k=$.geometry();c.push(k);const ee=new pt(k,N);ee.renderOrder=P,ee.frustumCulled=!1,r.add(ee)};ie(_,m,2),ie(U,g,3),ie(I,d,3),ie(G,d,3),ie(Z,x,5),ie(le,M,4),ie(X,T,5),ie(se,A,4)}return{group:r,geometries:c,materials:i}}const M5={0:[0,1,2,4,5,6],1:[2,5],2:[0,2,3,4,6],3:[0,2,3,5,6],4:[1,2,3,5],5:[0,1,3,5,6],6:[0,1,3,4,5,6],7:[0,2,5],8:[0,1,2,3,4,5,6],9:[0,1,2,3,5,6]},S5=[[0,6,10.4,12],[0,1.6,6,12],[4.4,6,6,12],[0,6,5.2,6.8],[0,1.6,0,6],[4.4,6,0,6],[0,6,0,1.6]];function T5(o,l){const r=new _a,c=[],i=[],f=l.metresToScene,h=l.worldSizeKm*.5,[g,m]=o.axis,[d,x]=o.normal,M=(V,pe)=>{const ue=o.centre.x_km+(V*g+pe*d)/1e3,Ce=o.centre.y_km+(V*m+pe*x)/1e3;return[(ue-h)*1e3*f,(Ce-h)*1e3*f]},[T,A]=M(0,0),R=l.terrainHeightAt(T,A),B=new Map,C=(V,pe,ue,Ce,Ne)=>{let oe=B.get(V);if(!oe){const J=l.layer(pe,ue,{side:ct,...Ne});i.push(J),oe={batch:new Wl,material:J,order:Ce},B.set(V,oe)}return oe.batch},H=(V,pe,ue,Ce,Ne,oe)=>{const J=(_e,Je)=>{const[O,Pe]=M(_e,Je);return[O,R+oe*f,Pe]};ou(V,J(pe,Ce),J(ue,Ce),J(ue,Ne),J(pe,Ne))},K=(V,pe,ue,Ce,Ne)=>{const oe=ue[0]-pe[0],J=ue[1]-pe[1],_e=Math.max(1e-6,Math.hypot(oe,J)),Je=-J/_e*Ce*.5,O=oe/_e*Ce*.5,Pe=(Ie,tt)=>{const[Nt,Ft]=M(Ie,tt);return[Nt,R+Ne*f,Ft]};ou(V,Pe(pe[0]-Je,pe[1]-O),Pe(pe[0]+Je,pe[1]+O),Pe(ue[0]+Je,ue[1]+O),Pe(ue[0]-Je,ue[1]-O))},U=(V,pe,ue,Ce,Ne,oe,J=0)=>{const _e=(O,Pe,Ie)=>{const[tt,Nt]=M(O,Pe);return[tt,R+(J+Ie)*f,Nt]},Je=[[pe,Ce],[ue,Ce],[ue,Ne],[pe,Ne]];for(let O=0;O<4;O+=1){const[Pe,Ie]=Je[O],[tt,Nt]=Je[(O+1)%4];ou(V,_e(Pe,Ie,0),_e(tt,Nt,0),_e(tt,Nt,oe),_e(Pe,Ie,oe))}ou(V,_e(pe,Ce,oe),_e(ue,Ce,oe),_e(ue,Ne,oe),_e(pe,Ne,oe))},_=o.runwayLengthM*.5,I=o.runwayWidthM,Z=C("shoulder",5000528,-2,2);H(Z,-_,_,-o.pavedWidthM/2,o.pavedWidthM/2,.05);const le=C("asphalt",3618875,-3,3);H(le,-_,_,-I/2,I/2,.07);const X=C("paint",15658730,-6,6,{emissive:2236962});H(X,-_,_,-I/2+.9,-I/2+1.8,.1),H(X,-_,_,I/2-1.8,I/2-.9,.1);for(let V=-_+12;V+30<_-12;V+=50)H(X,V,V+30,-.45,.45,.1);const se=o.code.startsWith("3")?8:12;for(const V of[-1,1]){const pe=-V,ue=J=>V*_+pe*J;for(let J=0;J<se;J+=1){const _e=-I/2+1.8+(I-3.6)/se*(J+.5)-.9,Je=6;H(X,Math.min(ue(Je),ue(Je+30)),Math.max(ue(Je),ue(Je+30)),_e,_e+1.8,.1)}const Ce=400;for(const J of[-1,1]){const _e=J*9;H(X,Math.min(ue(Ce),ue(Ce+45)),Math.max(ue(Ce),ue(Ce+45)),_e-3,_e+3,.1)}for(let J=0;J<3;J+=1){const _e=150*(J+1)+400;if(!(_e+22.5>_))for(const Je of[-1,1])for(let O=0;O<=2-J;O+=1){const Pe=Je*(6+3.6*O);H(X,Math.min(ue(_e),ue(_e+22.5)),Math.max(ue(_e),ue(_e+22.5)),Pe-.9,Pe+.9,.1)}}const Ne=V<0?o.designatorHi:o.designatorLo,oe=50;[...Ne].forEach((J,_e)=>{for(const Je of M5[J]??[]){const[O,Pe,Ie,tt]=S5[Je],Nt=oe+Ie,Ft=oe+tt,$t=(_e-(Ne.length-1)/2)*9-3+O,Yt=(_e-(Ne.length-1)/2)*9-3+Pe;H(X,Math.min(ue(Nt),ue(Ft)),Math.max(ue(Nt),ue(Ft)),$t,Yt,.1)}})}const we=C("taxi",4539978,-3,3),ge=C("yellow",14202674,-6,6),G=o.taxiwayZ,F=o.taxiwayWidthM;H(we,-_+60,_-60,G-F/2,G+F/2,.07),H(ge,-_+60,_-60,G-.1,G+.1,.1);for(const V of o.links)K(we,V.from,V.to,F,.07),K(ge,V.from,V.to,.2,.1);const ie=C("apron",10197395,-3,3);H(ie,o.apron.x0,o.apron.x1,o.apron.z0,o.apron.z1,.07);for(const[V,pe]of o.stands)K(ge,[V,o.apron.z0+6],[V,pe-10],.25,.1);const $=C("carpark",6184802,-3,3);H($,o.carPark.x0,o.carPark.x1,o.carPark.z0,o.carPark.z1,.07);const N=C("white",14605526,-1,4),P=C("glass",5205894,-1,4,{metalness:.4,roughness:.2}),k=C("grey",9409689,-1,4),ee=o.terminal;U(N,ee.x0,ee.x1,ee.z0,ee.z1,ee.heightM,0),U(P,ee.x0-.2,ee.x1+.2,ee.z0-.2,ee.z1+.2,ee.heightM*.45,ee.heightM*.25);for(const[V]of o.stands)U(k,V-1.5,V+1.5,o.apron.z1-38,ee.z0,.3,4.5);const Me=o.tower;U(N,Me.x0+4,Me.x1-4,Me.z0+4,Me.z1-4,Me.heightM-7,0),U(P,Me.x0-3,Me.x1+3,Me.z0-3,Me.z1+3,6,Me.heightM-7),U(k,Me.x0-3.5,Me.x1+3.5,Me.z0-3.5,Me.z1+3.5,.6,Me.heightM-1);for(const V of o.hangars)U(k,V.x0,V.x1,V.z0,V.z1,V.heightM,0);o.stands.forEach(([V,pe],ue)=>{ue%2!==1&&(U(N,V-2,V+2,pe-18,pe+18,4,1.2),U(N,V-17.9,V+17.9,pe-4,pe-1,.4,1.8),U(N,V-.2,V+.2,pe+13,pe+18,5,1.2+2.5))});const S=C("fence",3159098,-6,6),Q=o.fence;for(const[V,pe,ue,Ce]of[[Q.x0,Q.x1,Q.z0,Q.z0],[Q.x0,Q.x1,Q.z1,Q.z1],[Q.x0,Q.x0,Q.z0,Q.z1],[Q.x1,Q.x1,Q.z0,Q.z1]])K(S,[V,ue],[pe,Ce],.5,.12);const ve=C("lights",16774080,-6,6,{emissive:16774080,emissiveIntensity:.9});for(const V of[-1,1]){for(let ue=60;ue<=900;ue+=30){const Ce=V*(_+ue);H(ve,Ce-.4,Ce+.4,-2.5,2.5,.5)}const pe=V*(_+300);H(ve,pe-.4,pe+.4,-15,15,.5),H(ve,V*(_+150)-.4,V*(_+150)+.4,-11,11,.5)}for(const{batch:V,material:pe,order:ue}of B.values()){const Ce=V.geometry();c.push(Ce);const Ne=new pt(Ce,pe);Ne.renderOrder=ue,Ne.frustumCulled=!1,r.add(Ne)}return{group:r,geometries:c,materials:i}}function np(o){const l=atob(o),r=new Uint8Array(l.length);for(let c=0;c<l.length;c+=1)r[c]=l.charCodeAt(c);return new Float32Array(r.buffer)}function Kl(o){const l=atob(o),r=new Uint8Array(l.length);for(let c=0;c<l.length;c+=1)r[c]=l.charCodeAt(c);return r}const op=`
  varying vec2 vUv;
  void main() {
    vUv = uv;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`,lp=`
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
`,C5=31,_5=`
  precision highp float;
  varying vec2 vUv;
  varying vec3 vWorldPosition;
  uniform float uTime;
  uniform float uMetresToScene;
  uniform vec2 uWind;
  uniform sampler2D uHydrologyMap;
  uniform sampler2D uFlowMap;
  #include <fog_pars_vertex>

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
    vec4 mvPosition = viewMatrix * world;
    gl_Position = projectionMatrix * mvPosition;
    #include <fog_vertex>
  }
`,z5=`
  precision highp float;
  varying vec2 vUv;
  varying vec3 vWorldPosition;
  uniform float uTime;
  uniform vec2 uWind;
  uniform vec3 uSunDirection;
  uniform sampler2D uHydrologyMap;
  uniform sampler2D uFlowMap;
  #include <fog_pars_fragment>

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
    vec3 viewDirection = isOrthographic ? vec3(0.0, 1.0, 0.0) : normalize(cameraPosition - vWorldPosition);
    // Water is a dark body under a mirror: what it shows is the sky in the
    // mirrored direction, more of it the more grazing the view (Fresnel), plus
    // the sun's glitter path. The sky comes from the same atmosphere as the dome.
    float facing = max(dot(normal, viewDirection), 0.0);
    float fresnel = 0.02 + 0.98 * pow(1.0 - facing, 5.0);
    vec3 mirrored = reflect(-viewDirection, normal);
    mirrored.y = abs(mirrored.y) + 0.02;
    #if defined( USE_FOG ) && ! defined( FOG_EXP2 )
      vec3 skyMirror = atmSky(normalize(mirrored), max(cameraPosition.y * fogNear, 2.0), fogFar);
      vec3 sunTint = atmSunTransmittance(fogFar);
    #else
      vec3 skyMirror = vec3(0.39, 0.53, 0.56);
      vec3 sunTint = vec3(1.0, 0.91, 0.72);
    #endif
    vec3 reflectedSun = reflect(-normalize(uSunDirection), normal);
    float sunFacing = max(dot(reflectedSun, viewDirection), 0.0);
    // Unresolved ripples spread the sun's image into a path; the far, grazing
    // part of it is the brightest thing in a landscape.
    float glitter = pow(sunFacing, 90.0) * 0.35 + pow(sunFacing, 900.0) * 6.0;
    vec3 body = vec3(0.018, 0.105, 0.135) * ocean
      + vec3(0.035, 0.145, 0.155) * lake
      + vec3(0.075, 0.175, 0.155) * river;
    vec3 base = body * (1.0 - fresnel) + skyMirror * fresnel * 0.92 + sunTint * glitter * (0.3 + 0.7 * fresnel);
    float edgeFoam = smoothstep(0.16, 0.42, water) * (1.0 - smoothstep(0.42, 0.72, water));
    float streamFoam = river * flowStrength * smoothstep(0.73, 0.92, centre);
    base = mix(base, vec3(0.72, 0.79, 0.75), edgeFoam * ocean * 0.38 + streamFoam * 0.14);
    float alpha = mix(mix(0.52, 0.86, river * smoothstep(0.2, 0.5, water)), 0.82, ocean) * smoothstep(0.12, 0.58, water);
    alpha = max(alpha, river * 0.9 * smoothstep(0.16, 0.42, water));
    gl_FragColor = vec4(base, alpha);
    #include <fog_fragment>
  }
`;function A5({result:o,config:l,cameraMode:r,cityFocus:c,cityScene:i,onNeedCity:f,shot:h,onShotReady:g}){const m=mt.useRef(null),d=mt.useRef(null),x=mt.useRef(i??null);x.current=i??null;const M=mt.useRef(f);M.current=f,mt.useEffect(()=>{d.current?.(i??null)},[i]);const T=mt.useRef(c);T.current=c;const A=mt.useRef(h);A.current=h;const R=mt.useRef(g);R.current=g;const[B,C]=mt.useState(null),H=K=>{const U=new up,_=Ve.degToRad(l.sunAzimuth),I=Ve.degToRad(Ve.clamp(l.sunElevation,3,85)),Z=new lt(Math.sin(_)*Math.cos(I),Math.sin(I),Math.cos(_)*Math.cos(I)).normalize(),le=y=>{const v=Math.sin(y),b=1/(v+.50572*Math.pow(96.07995-Ve.radToDeg(Math.acos(v)),-1.6364)),z=Uh(l.haze);return[58e-7,135e-7,331e-7].map(D=>Math.exp(-(D*8e3+z*1300)*b))},X=le(I),se=le(Ve.degToRad(C5)),we=X.map((y,v)=>y/se[v]),ge=Math.max(...we);s5(Z),U.fog=i5(o.worldSizeKm*1e3/3.2,l.haze);const G=r==="satellite"?new vh(-1.7,1.7,1.7,-1.7,5e-6,30):new $o(42,1,5e-6,30);r==="satellite"?(G.position.set(0,5.2,.001),G.up.set(0,0,-1)):G.position.set(3.15,1.65,3.15);const F=new Cg({antialias:!aa,powerPreference:"high-performance"});F.setPixelRatio(aa?1:Math.min(window.devicePixelRatio,2)),F.outputColorSpace=So,F.toneMapping=cp,F.shadowMap.enabled=!Fs,F.shadowMap.type=_g,F.shadowMap.autoUpdate=!1,F.toneMappingExposure=1.08,K.appendChild(F.domElement);const ie=F.capabilities;if(!ie.isWebGL2)throw new Error("此浏览器没有 WebGL2，无法绘制 3D 地形。");if(o.meshSize>ie.maxTextureSize)throw new Error(`此设备的最大纹理尺寸为 ${ie.maxTextureSize}，低于网格 ${o.meshSize}；请选择更小的网格。`);const $=F.extensions.has("EXT_color_buffer_float")||F.extensions.has("EXT_color_buffer_half_float"),N=aa||Fs;F.domElement.addEventListener("webglcontextlost",y=>{y.preventDefault(),console.error("[webgl] context lost")}),F.domElement.addEventListener("webglcontextrestored",()=>console.warn("[webgl] context restored"));const P=l2(F);let k=null;const ee=!N&&$&&new URLSearchParams(window.location.search).get("probe")!=="0"?h5(F,U):null,{material:Me,uniforms:S}=u5(Z,l.haze,l.cloudCoverage/100),Q=new pt(new Gs(9e3,48,28),Me);Q.frustumCulled=!1,Q.renderOrder=-100,U.add(Q);try{if(!$&&N)throw new Error("no half-float render targets");S.uAmbientBake.value=1,U.environment=o5(F,Me),S.uAmbientBake.value=0,k=U.environment}catch(y){console.warn("[terrain3d] sky environment skipped",y),U.environment=null}U.environmentIntensity=1,F.toneMappingExposure=r==="satellite"?1.05:1.3;const ve=new y1(13950704,.1);ve.position.set(.5,.4,.6),U.add(ve);let V=null,pe=null,ue=null;const Ce=[],Ne=new URLSearchParams(window.location.search).get("ao")==="0";try{if(aa||!$||new URLSearchParams(window.location.search).get("post")==="0")throw new Error("post disabled for diagnosis");const y=new Vs(2,2,{type:rl,samples:4}),v=new t3(F,y);v.setPixelRatio(F.getPixelRatio());const b=new i3(U,G),z=new To(U,G,K.clientWidth||2,K.clientHeight||2);u3(z),z.output=To.OUTPUT.Default,z.blendIntensity=.85;const D=3.2/(o.worldSizeKm*1e3);z.updateGtaoMaterial({radius:.8*D,distanceExponent:1.1,thickness:1.4*D,scale:1,samples:16}),z.updatePdMaterial({lumaPhi:10,depthPhi:2,normalPhi:3,radius:4,rings:2,samples:8}),f3(z.normalMaterial),Lc(z.normalMaterial,"normal");{const E=z,j=E._renderOverride.bind(z);E._renderOverride=(...Y)=>{const ne=Ce.map(te=>te.visible);for(const te of Ce)te.visible=!1;j(...Y),Ce.forEach((te,me)=>{te.visible=ne[me]})}}const L=new kr(new zt(960,540),.2,.6,1.05);v.addPass(b),v.addPass(z),v.addPass(c3()),v.addPass(L),v.addPass(new s3),new URLSearchParams(window.location.search).get("lens")!=="0"&&(ue=f5(),v.addPass(ue.pass)),v.setSize(K.clientWidth||2,K.clientHeight||2),V=v,pe=z}catch{V=null,pe=null}const oe=new zg(G,F.domElement);oe.enableDamping=!0,oe.dampingFactor=.065,oe.zoomToCursor=!0,oe.screenSpacePanning=!0,oe.target.set(0,.055,0),r==="satellite"?(oe.enableRotate=!1,oe.enablePan=!0,oe.minZoom=.75,oe.maxZoom=131072):(oe.minDistance=8e-5,oe.maxDistance=7.5,oe.maxPolarAngle=Math.PI*.495);const J=Ci(np(o.heightDataBase64),o.meshSize),_e=new Bc(3.2,3.2,o.meshSize-1,o.meshSize-1),Je=_e.attributes.position;{const y=_e.attributes.uv,v=o.meshSize,b=z=>Math.min(v-3,Math.max(2,z));for(let z=0;z<v;z+=1)for(let D=0;D<v;D+=1)D>=2&&D<v-2&&z>=2&&z<v-2||y.setXY(z*v+D,b(D)/(v-1),1-b(z)/(v-1))}const O=3.2/(o.worldSizeKm*1e3),Pe=1/O,Ie=1.6,tt=3.2/Math.max(2,o.meshSize-1)*.5,Nt=1.7*O,Ft=.35*O;G instanceof $o&&(oe.minDistance=1.2*O,oe.maxDistance=Math.max(7.5,o.worldSizeKm*1e3*O*2.4));const $t=o.citySites??[];for(let y=0;y<J.length;y+=1){const v=Math.max(0,J[y]-o.stats.minElevation);Je.setZ(y,v*O)}Je.needsUpdate=!0,_e.computeVertexNormals();const Yt=new Ag().load(o.previewDataUrl);Yt.colorSpace=So,Yt.anisotropy=Math.min(N?4:16,F.capabilities.getMaxAnisotropy());const Aa=new Ye({map:Yt,roughness:.96,metalness:0}),mn=new Uint16Array(J.length);let Ea=1;for(let y=0;y<J.length;y+=1){const v=Math.max(0,J[y]-o.stats.minElevation);Ea=Math.max(Ea,v),mn[y]=Eg.toHalfFloat(v)}const Dt=new fa(mn,o.meshSize,o.meshSize,x1,rl);Dt.minFilter=la,Dt.magFilter=la,Dt.wrapS=Dt.wrapT=Ti,Dt.generateMipmaps=!1,Dt.needsUpdate=!0;const _o=Ci(o.materialDataBase64?Kl(o.materialDataBase64):new Uint8Array(o.meshSize*o.meshSize*4),o.meshSize,4,5),sa=new fa(_o,o.meshSize,o.meshSize,$n,gh);sa.minFilter=la,sa.magFilter=la,sa.wrapS=sa.wrapT=Ti,sa.generateMipmaps=!1,sa.needsUpdate=!0;const Ra=new fa(Kl(o.cultivatedDataBase64),o.meshSize,o.meshSize,x1,gh);Ra.minFilter=la,Ra.magFilter=la,Ra.wrapS=Ra.wrapT=Ti,Ra.generateMipmaps=!1,Ra.needsUpdate=!0;const Ya=Ve.degToRad(l.windDirection),pn={uMaterialTex:{value:sa},uFarmTex:{value:Ra},uWindDir:{value:new zt(Math.cos(Ya),Math.sin(Ya))},uHeightTex:{value:Dt},uHeightN:{value:o.meshSize},uM2S:{value:O},uMaxHeightScene:{value:Ea*O},uSunWorld:{value:Z.clone()},uSunViewDir:{value:Z.clone()}};Aa.onBeforeCompile=y=>{y.uniforms.uMetres={value:Pe},Object.assign(y.uniforms,pn),y.vertexShader=y.vertexShader.replace("#include <common>",`#include <common>
varying vec3 vDetailP;`).replace("#include <begin_vertex>",`#include <begin_vertex>
vDetailP = (modelMatrix * vec4(position, 1.0)).xyz;`);const v="RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );",b=mu.lights_fragment_begin,z=b.lastIndexOf(v),D=z<0?b:b.slice(0,z)+`directLight.color *= mix(1.0, gSunLit, step(0.98, dot(directionalLight.direction, uSunViewDir)));
		`+b.slice(z);y.fragmentShader=y.fragmentShader.replace("#include <lights_fragment_begin>",D).replace("#include <common>",`#include <common>
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
${x5}
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
      // Beds of uneven thickness, dipping a few degrees and gently folded: evenly
      // spaced level bands read as scan lines, not as rock.
      float bedM = altM + dot(pm, vec2(0.052, 0.031)) + 60.0 * warpS;
      float strata = mix(0.5, dNoise(vec2(bedM / 30.0, 0.37)), bandA);
      float strataF = mix(0.5, dNoise(vec2(bedM / 5.0 + 3.0 * dFbm(pm / 40.0), 7.1)), bandB);
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
      // From far enough that a pixel spans the patches, the snow line is an
      // average, not a scatter of white pixels.
      float a4 = 1.0 - smoothstep(25.0, 110.0, foot);
      float depthNoise = mix(dFbm(pm / 520.0), dFbm(pm / 130.0), a4) * 0.8 + dFbm(pm / 17.0) * 0.25 * a2;
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
}`)};const Na=new pt(_e,Aa);Na.renderOrder=Ap,Na.rotation.x=-Math.PI/2,U.add(Na);const vn=Kl(o.forestDataBase64),Ia=Kl(o.vegetationExclusionDataBase64),gn=Fs?0:aa?12e3:45e3,na=dh(o.farTrees,2,O),yn=na.material,ha=[],zo=[],Ao=[],dt=(y,v,b)=>{const z=Math.sin(y*127.1+v*311.7+b*74.7+l.seed*.013)*43758.5453;return z-Math.floor(z)};for(let y=1;y+1<o.meshSize;y+=1)for(let v=1;v+1<o.meshSize;v+=1){const b=y*o.meshSize+v,z=vn[b]/255;if(Ia[b]>32||z<.38||dt(v,y,1)>z*.62||ha.length>=gn)continue;const D=(dt(v,y,2)-.5)*.86,L=(dt(v,y,3)-.5)*.86,E=-1.6+(v+D)/(o.meshSize-1)*3.2,j=-1.6+(y+L)/(o.meshSize-1)*3.2,Y=.68+dt(v,y,4)*.72,ne=new Co;ne.compose(new lt(E,Math.max(0,J[b]-o.stats.minElevation)*O,j),new iu().setFromAxisAngle(new lt(0,1,0),dt(v,y,5)*Math.PI*2),new lt(.75+dt(v,y,6)*.55,Y,.75+dt(v,y,7)*.55)),ha.push(ne);const te=.8+dt(v,y,9)*.36;zo.push(new Zt(te*(1+(dt(v,y,8)-.5)*.1),te,te*(1-(dt(v,y,8)-.5)*.1))),Ao.push(Math.floor(dt(v,y,10)*na.geometries.length)%na.geometries.length)}const Da=new _a;na.geometries.forEach((y,v)=>{const b=[];if(Ao.forEach((D,L)=>{D===v&&b.push(L)}),!b.length)return;const z=new Va(y,yn,b.length);b.forEach((D,L)=>{z.setMatrixAt(L,ha[D]),z.setColorAt(L,zo[D])}),z.instanceMatrix.needsUpdate=!0,z.instanceColor&&(z.instanceColor.needsUpdate=!0),Da.add(z)}),U.add(Da),Ce.push(Da);const Wa=Fs?0:aa?6e3:32e3,cl=[dh(o.farTrees,0,O),dh(o.farTrees,1,O)],Eo=cl[1].material;cl[0].material.dispose();const Ps=cl.map(y=>y.geometries.map(v=>{const b=new Va(v,Eo,Wa);return b.instanceMatrix.setUsage(su),b.count=0,U.add(b),b})),ul=Ps.flat();Ce.push(...ul);const Un=(y,v,b)=>{const z=Ve.clamp((v+1.6)/3.2*(o.meshSize-1),0,o.meshSize-1),D=Ve.clamp((b+1.6)/3.2*(o.meshSize-1),0,o.meshSize-1),L=Math.floor(z),E=Math.floor(D),j=Math.min(o.meshSize-1,L+1),Y=Math.min(o.meshSize-1,E+1),ne=z-L,te=D-E,me=Ve.lerp(y[E*o.meshSize+L],y[E*o.meshSize+j],ne),ye=Ve.lerp(y[Y*o.meshSize+L],y[Y*o.meshSize+j],ne);return Ve.lerp(me,ye,te)},qc=Kl(o.cultivatedDataBase64),Ur={[zi.Cotton]:3,[zi.Maize]:2,[zi.Wheat]:1,[zi.Stubble]:0,[zi.Soil]:0,[zi.Alfalfa]:3},Ys=(y,v)=>{if(Un(qc,y,v)<.3*255)return 0;const b=ap(y/O,v/O);return b.margin?0:Ur[b.crop]},Vc=14e3,Or=new Ar(1,1,1),Di=new Ye({color:10987588,roughness:1}),On=new Va(Or,Di,Vc);On.instanceMatrix.setUsage(su),On.count=0,On.frustumCulled=!1,U.add(On);const Ws=3e4,es=14e3,Br=24e3,Hr=6e3,Bn=new Er(.008*O,.012*O,.82*O,4);Bn.translate(0,.41*O,0);const oa=new Er(.018*O,.028*O,.15*O,5);oa.translate(0,.88*O,0);const ma=new Er(.018*O,.028*O,2.05*O,5);ma.translate(0,1.025*O,0);const to=new It;to.setAttribute("position",new ft([0,.65,0,.34,.98,.035,0,.88,0,0,.9,0,-.31,1.28,-.025,0,1.13,0,0,1.16,0,.035,1.52,.32,0,1.38,0,0,1.38,0,-.02,1.73,-.27,0,1.57,0].map(y=>y*O),3)),to.computeVertexNormals();const Xs=new Er(.036*O,.052*O,.2*O,8);Xs.translate(.065*O,1.2*O,0);const Kc=new Gs(.012*O,4,3),ki=new It;ki.setAttribute("position",new ft([-.018,0,0,.018,0,0,0,.42,0,0,0,-.018,0,0,.018,0,.42,0].map(y=>y*O),3)),ki.computeVertexNormals();const Ui=new Ye({color:12100680,roughness:.96}),fl=new Ye({color:13151572,roughness:.91}),Zs=new Ye({color:4486700,roughness:.96}),Ro=new Ye({color:4027948,roughness:.98,side:ct}),No=new Ye({color:13018409,roughness:.88}),hl=new Ye({color:15057728,roughness:.82}),Do=new Ye({color:6916668,roughness:1,side:ct}),ea=new Va(Bn,Ui,Ws),Oi=new Va(oa,fl,Ws),Bi=new Va(ma,Zs,es),ao=new Va(to,Ro,es),ko=new Va(Xs,No,es),Lr=new Va(Kc,hl,Hr),Qs=new Va(ki,Do,Br),Hi=[Qs,ea,Oi,Bi,ao,ko,Lr];for(const y of Hi)y.instanceMatrix.setUsage(su),y.count=0,y.frustumCulled=!1,U.add(y);let jr=Number.POSITIVE_INFINITY,ts=Number.POSITIVE_INFINITY,as=0,Hn=0;const Wt=new Co,ns=new iu,os=new lt,Uo=new lt,Pc=new lt(0,1,0),Li=(y,v)=>{const b=y/O*v,z=b<520,D=z?Ve.clamp(b*.62,12,230):Ve.clamp(b*.58,100,1150),L=z?D<=24?.28:D<=55?.55:D<=120?1.15:2.1:D<=320?3.2:D<=650?5.5:9,E=Math.hypot(oe.target.x-jr,oe.target.z-ts)/O,j=Math.abs(D-as)/Math.max(1,as);if(E<L*3&&j<.12&&Hn===L)return;jr=oe.target.x,ts=oe.target.z,as=D,Hn=L;const Y=(jr+1.6)/O,ne=(ts+1.6)/O;let te=0,me=0,ye=0,he=0,be=0;const Ee=Math.floor((Y-D)/L),Se=Math.ceil((Y+D)/L),Ue=Math.floor((ne-D)/L),Le=Math.ceil((ne+D)/L);for(let Ae=Ue;Ae<=Le;Ae+=1)for(let ke=Ee;ke<=Se;ke+=1){const qe=(ke+.12+dt(ke,Ae,72)*.76)*L,Oe=(Ae+.12+dt(ke,Ae,73)*.76)*L,Fe=qe-Y,Ge=Oe-ne;if(Fe*Fe+Ge*Ge>D*D)continue;const De=-1.6+qe*O,He=-1.6+Oe*O;if(De<=-1.6||De>=1.6||He<=-1.6||He>=1.6)continue;const Ke=Ys(De,He);if(Ke<1||Ke>3||Un(Ia,De,He)>48)continue;const Be=Math.max(0,Un(J,De,He)-o.stats.minElevation)*O,Xe=ap(De/O,He/O).rowAngle;if(ns.setFromAxisAngle(Pc,Xe+(dt(ke,Ae,75)-.5)*.035),!z){if(te>=Vc)continue;const je=L*(2.2+dt(ke,Ae,76)*1.4);os.set(De,Be+(Ke===1?.34:Ke===2?.72:.18)*O,He),Uo.set(je*O,(Ke===1?.55:Ke===2?1.25:.3)*O,.24*O),Wt.compose(os,ns,Uo),On.setMatrixAt(te,Wt),On.setColorAt(te,new Zt(Ke===1?11116867:Ke===2?5208622:7440444)),te+=1;continue}const rt=.86+dt(ke,Ae,77)*.26,$e=(dt(ke,Ae,78)-.5)*.08;if(ns.setFromEuler(new Ah($e,Xe+dt(ke,Ae,79)*.12,$e*.6)),os.set(De,Be,He),Uo.set(1,rt,1),Wt.compose(os,ns,Uo),Ke===3&&me<Br)Qs.setMatrixAt(me,Wt),Qs.setColorAt(me,new Zt().setHSL(.22+dt(ke,Ae,80)*.06,.38,.28+dt(ke,Ae,81)*.12)),me+=1;else if(Ke===1&&ye<Ws)ea.setMatrixAt(ye,Wt),Oi.setMatrixAt(ye,Wt),ye+=1;else if(Ke===2&&he<es){if(Bi.setMatrixAt(he,Wt),ao.setMatrixAt(he,Wt),ko.setMatrixAt(he,Wt),Fe*Fe+Ge*Ge<196)for(let je=0;je<12&&be<Hr;je+=1){const Ze=je%6,ot=Math.floor(je/6),Tt=Xe+Ze/6*Math.PI*2;os.set(De+Math.cos(Tt)*.054*O,Be+(1.145+ot*.07)*O*rt,He+Math.sin(Tt)*.054*O),Uo.set(.78,1.18,.78),Wt.compose(os,ns.identity(),Uo),Lr.setMatrixAt(be,Wt),be+=1}he+=1}}On.count=te,Qs.count=me,ea.count=ye,Oi.count=ye,Bi.count=he,ao.count=he,ko.count=he,Lr.count=be;for(const Ae of[On,...Hi])Ae.instanceMatrix.needsUpdate=!0,Ae.instanceColor&&(Ae.instanceColor.needsUpdate=!0)},Mt=(y,v)=>Math.max(0,Un(J,y,v)-o.stats.minElevation)*O,nt=G instanceof $o?new a5({camera:G,dom:F.domElement,groundHeight:Mt,metresToScene:O,halfExtent:Ie}):null,Ja=nt?(()=>{const y=document.createElement("div");y.style.cssText="position:absolute;z-index:4;top:14px;right:14px;display:flex;gap:1px;border:1px solid rgba(255,255,255,.16);background:rgba(9,13,11,.78);backdrop-filter:blur(14px)";const v="padding:7px 10px;border:0;background:transparent;color:#9da8a3;font-size:11px;cursor:pointer",b=document.createElement("button");b.style.cssText=v;const z=document.createElement("button");return z.style.cssText=v,y.append(b,z),K.appendChild(y),{root:y,roamButton:b,modeButton:z}})():null,ls=()=>{!nt||!Ja||(Ja.roamButton.textContent=nt.active?"轨道视角 (R)":"自由漫游 (R)",Ja.roamButton.style.background=nt.active?"var(--accent, #d7ff72)":"transparent",Ja.roamButton.style.color=nt.active?"#11160f":"#9da8a3",Ja.modeButton.style.display=nt.active?"":"none",Ja.modeButton.textContent=nt.walking?"步行 → 飞行 (G)":"飞行 → 步行 (G)")};let Oo=null;const Bo=y=>{!nt||!Oo||(y&&!nt.active?(nt.enter(!1),Oo.enabled=!1):!y&&nt.active&&(nt.exit(),Oo.enabled=!0,Oo.target.copy(nt.groundTarget),Oo.update()),ls())},Fr=y=>{const v=y.target;v&&/^(INPUT|SELECT|TEXTAREA)$/.test(v.tagName)||y.ctrlKey||y.metaKey||y.altKey||(y.code==="KeyR"?Bo(!nt?.active):y.code==="KeyG"&&nt?.active&&(nt.setWalking(!nt.walking),ls()))};nt&&Ja&&(Oo=oe,Ja.roamButton.onclick=()=>Bo(!nt.active),Ja.modeButton.onclick=()=>{nt.setWalking(!nt.walking),ls()},window.addEventListener("keydown",Fr),ls());const dl=new _a,ml=[],Yc=[],pl=[],Is=(y,v,b)=>{const D=new Uint8Array(65536);for(let E=0;E<128;E+=1)for(let j=0;j<128;j+=1){const Y=E%24,ne=j%18,te=Y>5&&Y<18&&ne>4&&ne<14&&(j+E+b)%11!==0,me=Y===0||ne===0,ye=(j*17+E*31+b*13)%17-8,he=(E*128+j)*4,be=te?v:y;D[he]=Ve.clamp(be[0]+ye+(me?7:0),0,255),D[he+1]=Ve.clamp(be[1]+ye+(me?7:0),0,255),D[he+2]=Ve.clamp(be[2]+ye+(me?7:0),0,255),D[he+3]=255}const L=new fa(D,128,128,$n);return L.wrapS=za,L.wrapT=za,L.magFilter=la,L.minFilter=al,L.generateMipmaps=!0,L.colorSpace=So,L.repeat.set(2.8,3.8),L.needsUpdate=!0,pl.push(L),L},ji=new Map,Mu=(o.materialTextures?.length??0)>0,Fi=(y,v,b)=>{if(!o.materialTextures?.length)return null;const z=ji.get(`${y}|${v[0]}|${v[1]}`);if(z)return z;const D=o.materialTextures.find(Y=>Y.name===y);if(!D)return null;const L=atob(D.data),E=new Uint8Array(L.length);for(let Y=0;Y<L.length;Y+=1)E[Y]=L.charCodeAt(Y);const j=new fa(E,D.width,D.height,$n);return j.wrapS=za,j.wrapT=za,j.magFilter=la,j.minFilter=al,j.generateMipmaps=!0,j.colorSpace=So,j.repeat.set(v[0],v[1]),j.needsUpdate=!0,ji.set(`${y}|${v[0]}|${v[1]}`,j),b.push(j),j};Fi("facade/residential",[1/22,1/30],pl)??Is([144,151,150],[36,66,77],3),Fi("facade/glass",[1/43,1/25],pl)??Is([96,122,130],[18,47,59],7),Fi("facade/stone",[1/7,1/6],pl)??Is([177,166,144],[48,66,69],11),Fi("facade/residential",[1/24,1/32],pl)??Is([146,108,91],[42,58,63],17);const vl=new Map,no=y=>{const v=`v${y%Ho.length}`;let b=vl.get(v);return b||(b=[],vl.set(v,b)),v},st=1e3*O,it=o.worldSizeKm*.5,ss={generateTopUV(y,v,b,z,D,L){const E=1/O;return[new zt(v[b*3]*E,v[b*3+2]*E),new zt(v[z*3]*E,v[z*3+2]*E),new zt(v[D*3]*E,v[D*3+2]*E),new zt(v[L*3]*E,v[L*3+2]*E)]},generateSideWallUV(y,v,b,z,D,L){const E=1/O,j=v[b*3],Y=v[b*3+1],ne=v[b*3+2],te=v[z*3],me=v[z*3+1],ye=v[z*3+2],he=v[D*3],be=v[D*3+1],Ee=v[D*3+2],Se=v[L*3],Ue=v[L*3+1],Le=v[L*3+2];return Math.abs(Y-me)<1e-9?[new zt(j*E,ne*E),new zt(te*E,ye*E),new zt(he*E,Ee*E),new zt(Se*E,Le*E)]:[new zt(j*E,Y*E),new zt(te*E,me*E),new zt(he*E,be*E),new zt(Se*E,Ue*E)]}},Ho=[{base:[198,188,168],window:[96,104,112],cols:2,sill:.34},{base:[176,179,176],window:[88,97,106],cols:2,sill:.32},{base:[216,211,197],window:[112,142,154],cols:3,sill:.28},{base:[156,100,80],window:[64,56,54],cols:2,sill:.36,brick:!0},{base:[208,196,168],window:[98,118,128],cols:3,sill:.3},{base:[150,151,149],window:[80,92,100],cols:4,sill:.24,band:!0},{base:[190,148,120],window:[84,90,96],cols:2,sill:.32},{base:[132,136,142],window:[98,130,142],cols:3,sill:.3},{base:[76,90,100],window:[134,170,186],cols:4,glass:!0},{base:[66,80,88],window:[120,160,152],cols:5,glass:!0},{base:[90,94,106],window:[150,152,160],cols:4,glass:!0},{base:[112,98,90],window:[128,150,158],cols:3,glass:!0},{base:[58,62,70],window:[110,150,168],cols:6,glass:!0},{base:[168,170,172],window:[96,118,130],cols:5,glass:!0},{base:[122,96,72],window:[150,138,116],cols:4,glass:!0},{base:[70,92,96],window:[128,164,170],cols:5,glass:!0},{base:[214,212,206],window:[104,116,126],cols:4,sill:.26},{base:[142,138,130],window:[88,96,104],cols:4,sill:.26,band:!0},{base:[186,172,148],window:[96,110,118],cols:3,sill:.3},{base:[84,86,90],window:[118,128,136],cols:5,sill:.22},{base:[226,222,212],window:[100,110,118],cols:2,sill:.4},{base:[164,132,96],window:[76,64,54],cols:2,sill:.36},{base:[142,74,60],window:[70,58,52],cols:2,sill:.36,brick:!0},{base:[172,178,164],window:[92,104,110],cols:3,sill:.34}],Gr=new Map,Js=[],Wc=y=>{const v=y%Ho.length,b=Gr.get(v);if(b)return b;const z=Ho[v],D=128,L=new Uint8Array(D*D*4);let E=v*131+17;const j=()=>(E=E*1664525+1013904223>>>0,E/4294967296),Y=(ye,he)=>{const be=Math.sin(ye*127.1+he*311.7+v*74.7)*43758.5453;return be-Math.floor(be)},ne=ye=>Y(ye*7.3,v*3.1);for(let ye=0;ye<D;ye++)for(let he=0;he<D;he++){const be=ye/D,Ee=he/D,Se=Math.min(3,Math.floor(be*4)),Ue=be*4-Se,Le=Math.floor(Ee*z.cols),Ae=Ee*z.cols-Le,ke=ne(Se),qe=.8+(ke-.5)*.14,Oe=z.glass?.1:Math.max(.26,.32+(z.sill??.3)*.4-(ke-.5)*.18),Fe=z.glass&&Ae>.92;let Ge;if(Le<z.cols&&Ae<(z.glass?.92:.6)&&Ue>Oe&&Ue<qe&&!Fe){const He=Y(Le+v*31,Se*17.3+Math.floor(v*5.7)),Ke=Math.floor(Ue*8)%4===0?12:0;Ge=z.window.map(Be=>Be*(.5+He*.85)+Ke+(j()-.5)*8)}else Ue>.9||Ue<.08?Ge=z.base.map(He=>He*.55+8):Fe?Ge=z.base.map(He=>He*.45):(Ge=z.base.map(He=>He+(j()-.5)*10),z.brick&&ye%7<2&&(Ge=Ge.map(He=>He+16)));const De=(ye*D+he)*4;for(let He=0;He<3;He++)L[De+He]=Ve.clamp(Ge[He],0,255);L[De+3]=255}const te=new fa(L,D,D);te.wrapS=te.wrapT=za,te.magFilter=la,te.minFilter=al,te.generateMipmaps=!0,te.colorSpace=So,te.needsUpdate=!0,te.repeat.set(1/3,1/12.8);const me=z.glass?new Ye({map:te,roughness:.34,metalness:.45,vertexColors:!0,side:ct,envMapIntensity:.85}):new Ye({map:te,roughness:.82,vertexColors:!0,side:ct,envMapIntensity:.55});return Gr.set(v,me),Js.push(me),me},gl=[.4,.41,.4],ka=aa?[]:o.modernCities?.length?o.modernCities.map(y=>({style:"modernChinese",streets:[],blocks:y.blocks,buildings:y.buildings.map(v=>({id:v.id,parcelId:v.parcelId,footprint:v.footprint,courtyard:null,height_metres:v.heightMetres,floors:v.floors,variant:v.variant,style:v.style,pitched:v.pitched,tierRing:v.tierRing,podiumFloors:v.podiumFloors,facade:v.facade,podiumHeightMetres:v.podiumHeightMetres,windowBays:v.windowBays,balconyBays:v.balconyBays,entranceCount:v.entranceCount,roof:v.roof})),parcels:y.parcels.map(v=>({boundary:v.ring,landUse:v.useType})),sdRoads:[],hdRoads:y.hdRoads.map(v=>({id:v.id,class:v.class,widthMetres:v.widthMetres,pathKm:v.centreline.map(b=>[b.x_km,b.y_km]),lanesForward:v.lanesForward,lanesBackward:v.lanesBackward,medianMetres:v.medianMetres,layer:v.layer,structure:v.structure,lanes:v.lanes,connectors:v.connectors})),lanes:y.lanes??y.hdRoads.flatMap(v=>v.lanes??[]),connectors:y.connectors??y.hdRoads.flatMap(v=>v.connectors??[]),junctions:y.junctions??[],river:y.river?{centerline:y.river,widthMetres:y.riverWidthMetres??64}:void 0,compounds:y.compounds??[],trees:y.trees??[]})):o.cities??[],is=[],rs=[],oo=[];let yl=0;const xl=22e4,Xc=(y,v,b)=>{if(yl>=xl||y.length<3)return;let z=0;for(let E=0;E<y.length;E+=1){const j=y[(E+1)%y.length];z+=y[E].x*j.z-j.x*y[E].z}const D=z>0,L=Ve.clamp(Math.floor(b/3.25)-1,1,28);for(let E=0;E<y.length&&yl<xl;E+=1){const j=y[E],Y=y[(E+1)%y.length],ne=Y.x-j.x,te=Y.z-j.z,me=Math.hypot(ne,te),ye=me*Pe;if(ye<7)continue;const he=ne/Math.max(1e-9,me),be=te/Math.max(1e-9,me),Ee=D?be:-be,Se=D?-he:he,Ue=Ve.clamp(Math.floor(ye/5.4),1,30),Le=me/Ue,Ae=Math.min(.78,Le*.31);for(let ke=0;ke<L&&yl<xl;ke+=1){const qe=(ke+1)*3.25+.38;if(qe+1.45>b)break;const Oe=v+qe*O,Fe=Oe+1.38*O;for(let Ge=0;Ge<Ue&&yl<xl;Ge+=1){const De=j.x+he*Le*(Ge+.5)+Ee*.012*O,He=j.z+be*Le*(Ge+.5)+Se*.012*O,Ke=Ae*O,Be=De-he*Ke,Xe=He-be*Ke,rt=De+he*Ke,$e=He+be*Ke,je=is.length/3;is.push(Be,Oe,Xe,rt,Oe,$e,rt,Fe,$e,Be,Fe,Xe);for(let Ze=0;Ze<4;Ze+=1)rs.push(Ee,0,Se);oo.push(je,je+1,je+2,je,je+2,je+3),yl+=1}}}},Et=(y,v)=>{const b=Ve.clamp(v,0,1),z=y,D=z.transparent===!0;if(z.transparent=!0,z.opacity!==void 0&&(z.opacity=b),z.depthWrite!==void 0){const L=b>.985;z.depthWrite!==L&&(z.depthWrite=L,y.needsUpdate=!0)}D||(y.needsUpdate=!0)},xn=(y,v,b)=>1-Ve.smoothstep(y,v,b),Zc=(y,v,b)=>Ve.smoothstep(y,v,b);let Ma=0;const Ln={curtain:[8,9,10,11,12,13,14,15],residential:[0,1,3,22,23],stone:[18,19,16],other:[4,5,6,7,17,20]};for(const y of ka)for(const v of y.buildings??[]){if(Ma>=8e3||v.footprint.length<3)break;const b=String(v.facade??"").toLowerCase(),z=b.includes("curtain")?Ln.curtain:b.includes("brick")?Ln.residential:b.includes("stone")?Ln.stone:Ln.other,D=z[(Number(v.id??0)*7+3)%z.length],L=no(D),E=v.footprint.map(Ae=>({x:(Ae.x_km-it)*st,z:(Ae.y_km-it)*st})),j=new w1;j.moveTo(E[0].x,-E[0].z);for(const Ae of E.slice(1))j.lineTo(Ae.x,-Ae.z);if(j.closePath(),v.courtyard&&v.courtyard.length>=3){const Ae=new Rg,ke=v.courtyard.map(qe=>({x:(qe.x_km-it)*st,z:(qe.y_km-it)*st}));Ae.moveTo(ke[0].x,-ke[0].z);for(const qe of ke.slice(1))Ae.lineTo(qe.x,-qe.z);Ae.closePath(),j.holes.push(Ae)}const Y=E.reduce((Ae,ke)=>Ae+ke.x,0)/E.length,ne=E.reduce((Ae,ke)=>Ae+ke.z,0)/E.length,te=new Ng(j,{depth:v.height_metres*O,bevelEnabled:!1,curveSegments:1,steps:1,UVGenerator:ss});te.rotateX(-Math.PI/2),te.translate(0,Mt(Y,ne)+.08*O,0),te.computeVertexNormals();const me=te.attributes.position,ye=new Float32Array(me.count*3).fill(1),he=te.attributes.uv.array,be=.82+Number(v.id??0)%37/37*.36,Ee=[be*(.92+Number(v.parcelId??1)%13/13*.16),be*.99,be*(.86+Number(v.id??0)%7/7*.28)],Se=new lt,Ue=new lt,Le=new lt;for(let Ae=0;Ae<me.count;Ae+=3)if(Se.fromBufferAttribute(me,Ae),Ue.fromBufferAttribute(me,Ae+1),Le.fromBufferAttribute(me,Ae+2),Ue.sub(Se),Le.sub(Se),Se.crossVectors(Ue,Le),Math.abs(Se.y)>.9*Se.length())for(let ke=0;ke<3;ke+=1)ye[(Ae+ke)*3]=gl[0],ye[(Ae+ke)*3+1]=gl[1],ye[(Ae+ke)*3+2]=gl[2],he[(Ae+ke)*2]=.02,he[(Ae+ke)*2+1]=.06;else for(let ke=0;ke<3;ke+=1)ye[(Ae+ke)*3]=Ee[0],ye[(Ae+ke)*3+1]=Ee[1],ye[(Ae+ke)*3+2]=Ee[2];te.setAttribute("color",new Jt(ye,3)),te.attributes.uv.needsUpdate=!0,ml.push(te),vl.get(L).push(te),Mu||Xc(E,Mt(Y,ne)+.1*O,v.height_metres),Ma+=1}for(const[y,v]of vl){if(v.length===0)continue;const b=Nn(v,!1);if(!b)continue;b.computeBoundingSphere(),Yc.push(b);const z=Number(y.slice(1)),D=new pt(b,Wc(z));D.castShadow=!0,D.receiveShadow=!0,dl.add(D)}dl.visible=!1,U.add(dl);const Gi=6500,$s=13e3,Su=4,cs=Number(new URLSearchParams(window.location.search).get("cityslice")),qr=y=>Number.isFinite(cs)&&cs>0?cs:y>.06?Math.min(250,y*200):Su,us=7,qi=4*1024*1024;let wn=null,Lo=null,ei=null,wl=null,ti=null;const Qc=()=>{Lo?.cancel(),Lo=null,ei=null,wl&&(wl.layer.dispose(),wl=null)},Ic=y=>{Qc(),wn&&(wn.dispose(),wn=null),y&&(Np(y.textures),Lo=Ep(y),ei=y,new lt((y.origin[0]-it)*st,0,(y.origin[1]-it)*st))},Tu=(y,v)=>{const b=(v.origin[0]-it)*st,z=(v.origin[1]-it)*st;y.group.scale.setScalar(O),y.group.rotation.y=-v.rotationRadians;const D=Mt(b,z);y.group.position.set(b,D+.12*O,z),y.group.updateMatrixWorld(!0);const L=[];y.group.traverse(he=>{he!==y.group&&(he.matrixAutoUpdate=!1);const be=he;be.geometry&&(be.isMesh||be.isInstancedMesh)&&(L.push(be),be.userData.wasVisible=be.visible,be.layers.set(us),be.visible=!1)});const E=new Map;for(const he of L){const be=he.material.uuid;E.has(be)||E.set(be,E.size)}L.sort((he,be)=>E.get(he.material.uuid)-E.get(be.material.uuid));const[j,Y,ne,te]=v.extentM,me=Math.max(ne-j,te-Y,200)*.75*O,ye=new $o(120,1,me*.05,me*12);ye.position.set(b,D+me*1.1,z),ye.up.set(0,0,-1),ye.lookAt(b,D,z),ye.layers.set(us),ye.updateMatrixWorld(!0),U.add(y.group),wl={layer:y,pieces:L,next:0,camera:ye},ti??=new Vs(8,8,{type:rl})},Cu=y=>{let v=0;for(const b in y.geometry.attributes)v+=y.geometry.attributes[b].array.byteLength;return y.geometry.index&&(v+=y.geometry.index.array.byteLength),v},_u=y=>{const v=y>.06&&!(cs>0);if(Lo&&ei){if(Lo.step(qr(y))&&Lo.layer){const Y=Lo.layer,ne=ei;Lo=null,ei=null,Tu(Y,ne)}return}if(!wl)return;const b=wl,z=performance.now(),D=[],L=new Set;let E=0;const j=v?24:3;for(;b.next<b.pieces.length&&E<qi*(v?8:1);){const Y=b.pieces[b.next],ne=Y.material.uuid;if(L.size>=j&&!L.has(ne))break;L.add(ne),b.next+=1,E+=Cu(Y),Y.visible=!!Y.userData.wasVisible,D.push(Y)}if(D.length>0){const Y=ti,ne=F.shadowMap.needsUpdate;F.shadowMap.needsUpdate=!1,F.setRenderTarget(Y),F.render(U,b.camera),F.setRenderTarget(null),F.shadowMap.needsUpdate=ne;for(const te of D)te.visible=!1;qa("cityWarmSliceMaxMs",performance.now()-z),qa(`cityWarm${b.next}Ms`,performance.now()-z)}if(b.next>=b.pieces.length){for(const Y of b.pieces)Y.visible=!!Y.userData.wasVisible,Y.layers.set(0);wn=b.layer,wl=null,console.info("[city] whole-city layer mounted"),qa("cityMountedAtS",performance.now()/1e3),F.shadowMap.needsUpdate=!0}};d.current=Ic,Ic(x.current);const ai=o.modernCities??[],jo=ai.map(()=>null),Jc=y=>{const v=ai[y],b=Q3(v,{toScene:(z,D)=>[(z-it)*st,(D-it)*st],heightAt:Mt,metresToScene:O});U.add(b.mesh),b.water&&U.add(b.water),jo[y]=b},zu=y=>{const v=jo[y];v&&(U.remove(v.mesh),v.water&&U.remove(v.water),v.dispose(),jo[y]=null)},Au=9e3;let Vi=0;const Eu=()=>{if(Vi+=1,Fs||Vi%30!==1)return;let y=-1,v=Au;ai.forEach((b,z)=>{const D=$t[z];if(!D)return;const L=(D.xKm-it)*st-G.position.x,E=(D.yKm-it)*st-G.position.z,j=Math.hypot(L,E)*Pe;j<v&&(v=j,y=z)}),jo.forEach((b,z)=>{b&&z!==y&&zu(z)}),y>=0&&!jo[y]&&Jc(y)};aa||ai.forEach((y,v)=>Jc(v));const Ru=()=>{aa&&Eu();const y=wn?x.current:null;jo.forEach((v,b)=>{const z=$t[b];if(!v||!z)return;const D=y!==null&&Math.hypot(y.origin[0]-z.xKm,y.origin[1]-z.yKm)<.05;v.mesh.visible===D&&(v.mesh.visible=!D),v.water&&(v.water.visible=!D)})},ni=(o.modernCities??[]).map(()=>null);let Vr=0;const $c=new URLSearchParams(window.location.search).has("tilelog");bh.log=$c;const Nu=Number(new URLSearchParams(window.location.search).get("tilebudget"))||(A.current?.tileBudgetMs??Jl.tileBudgetMs),Du=y=>{(o.modernCities??[]).forEach((v,b)=>{const z=v.frame.origin,D=(z.x_km-it)*st,L=(z.y_km-it)*st,E=Math.hypot(D-G.position.x,L-G.position.z)*Pe;let j=ni[b];if(!j&&E<Gi){const ne=Number(new URLSearchParams(window.location.search).get("tileload"))||(A.current?.tileLoadM??0);j=new p3(b,$g,(me,ye,he)=>{jo[b]?.setTileHidden(me,ye,he)},ne>0?ne:void 0);const te=Mt(D,L);j.group.scale.setScalar(O),j.group.rotation.y=-v.frame.rotationRadians,j.group.position.set(D,te+.12*O,L),j.group.updateMatrixWorld(!0),U.add(j.group),ni[b]=j}if(!j)return;if(E>$s){j.dispose(),U.remove(j.group),ni[b]=null;return}const Y=j.group.worldToLocal(G.position.clone());j.update(Y,Nu),j.tick(y,E,G),Vr+=1,$c&&Vr%30===0&&console.log(`[tiles] city ${b} camera ${Y.x.toFixed(0)},${Y.z.toFixed(0)} ${JSON.stringify(j.summary())}`)})},fs=new It;is.length>0&&(fs.setAttribute("position",new ft(is,3)),fs.setAttribute("normal",new ft(rs,3)),fs.setIndex(oo),fs.computeBoundingSphere());const Kr=new Ye({color:2506569,roughness:.34,metalness:.34,emissive:463126,emissiveIntensity:.08,side:ct,polygonOffset:!0,polygonOffsetFactor:-1,polygonOffsetUnits:-1}),Pr=new pt(fs,Kr);Pr.visible=!1,dl.add(Pr);const Yr=new Map,Wr=(y,v,b={})=>{const z=Yr.get(y);if(z)return z;let D=y.length*131+7;const L=()=>(D=D*1664525+1013904223>>>0,D/4294967296),E=b.size??256,j=new Uint8Array(E*E*4),Y=new Float32Array(E*E);let ne=!1;for(let ye=0;ye<E;ye++)for(let he=0;he<E;he++){const be=v(he,ye,L),Ee=(ye*E+he)*4;for(let Se=0;Se<3;Se++)j[Ee+Se]=Ve.clamp(be[Se],0,255);j[Ee+3]=255,be.length>3&&(ne=!0,Y[ye*E+he]=be[3])}const te=new fa(j,E,E);te.wrapS=te.wrapT=za,te.magFilter=la,te.minFilter=al,te.generateMipmaps=!0,te.colorSpace=So,te.anisotropy=16,te.needsUpdate=!0;const me=new Ye({map:te,roughness:b.roughness??.95,metalness:b.metalness??0});if(ne){const ye=b.bump??2.2,he=new Uint8Array(E*E*4);for(let Ee=0;Ee<E;Ee++)for(let Se=0;Se<E;Se++){const Ue=Y[Ee*E+(Se+E-1)%E],Le=Y[Ee*E+(Se+1)%E],Ae=Y[(Ee+E-1)%E*E+Se],ke=Y[(Ee+1)%E*E+Se],qe=(Ue-Le)*ye,Oe=(Ae-ke)*ye,Fe=Math.hypot(qe,Oe,1)||1,Ge=(Ee*E+Se)*4;he[Ge]=(qe/Fe*.5+.5)*255,he[Ge+1]=(Oe/Fe*.5+.5)*255,he[Ge+2]=(1/Fe*.5+.5)*255,he[Ge+3]=255}const be=new fa(he,E,E);be.wrapS=be.wrapT=za,be.magFilter=la,be.minFilter=al,be.generateMipmaps=!0,be.needsUpdate=!0,me.normalMap=be,me.normalScale=new zt(b.normalScale??.6,b.normalScale??.6)}return Yr.set(y,me),me},ku=()=>Wr("asphalt",(y,v,b)=>{const z=Math.sin(y*.043+Math.sin(v*.031)*2.7)*Math.cos(v*.037+y*.017)*6,D=(b()-.5)*22,L=(b()-.5)*13,E=b()<.045?24:0,j=62+z+D*.5+L+E;return[j*.97,j,j*1.05,.5+(L+E)/56+(D<-9?-.14:0)]},{roughness:.97,bump:1.7,normalScale:.5}),Xr=()=>Wr("paving",(y,v,b)=>{const z=Math.floor(v/16),D=z%2*16,L=(y+D)%32,E=v%16,j=L<2||E<2?-30:0,Y=(Math.floor((y+D)/32)+z)%2?5:0,ne=Math.sin((y+z*11)*.021)*Math.cos(v*.019)*7,te=(b()-.5)*7,me=152+j+Y+ne+te;return[me,me*.985,me*.94,.6+(j?-.42:0)+(b()-.5)*.08]},{roughness:.88,bump:2.6,normalScale:.7}),oi=()=>Wr("grass",(y,v,b)=>{const z=Math.sin(y*.09+v*.05)*Math.sin(v*.07+1.3)*10,D=Math.sin(y*.017+1.1)*Math.cos(v*.013)>.74,L=b()<.09?20:(b()-.5)*13,E=[76+z+L,98+z+L,54+z*.7+L*.8];return D&&(E[0]+=32,E[1]+=16,E[2]-=6),[...E,.5+L/60+(b()-.5)*.3]},{roughness:.98,bump:1.4,normalScale:.55}),li=new _a,Ki=[],si=[],e0=[],Pi=ku().map,t0=Pi,bl=Pi,Uu=Xr().map,Zr={boulevard:new Ye({map:Pi,color:9211789,roughness:.9,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),avenue:new Ye({map:t0,color:9474446,roughness:.93,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),street:new Ye({map:bl,color:9737359,roughness:.95,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),service:new Ye({map:bl,color:10066580,roughness:.98,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2})},Qr=new Ye({map:Uu,color:11908266,roughness:.98,polygonOffset:!0,polygonOffsetFactor:-1,polygonOffsetUnits:-1}),Yi={boulevard:[],avenue:[],street:[],service:[]},Wi=(y,v,b,z,D=0)=>{const L=(y.x_km-it)*st,E=(y.y_km-it)*st,j=(v.x_km-it)*st,Y=(v.y_km-it)*st,ne=Math.hypot(j-L,Y-E)*Pe,te=Math.max(1,Math.ceil(ne/90)),me=[],ye=[];for(let Ee=0;Ee<=te;Ee+=1){const Se=Ee/te,Ue=Ve.lerp(L,j,Se),Le=Ve.lerp(E,Y,Se),Ae=j-L,ke=Y-E,qe=Math.max(1e-9,Math.hypot(Ae,ke)),Oe=-ke/qe,Fe=Ae/qe,Ge=b*O*.5,De=Mt(Ue,Le)+(z+D)*O;me.push(Ue+Oe*Ge,De,Le+Fe*Ge,Ue-Oe*Ge,De,Le-Fe*Ge)}for(let Ee=0;Ee<te;Ee+=1){const Se=Ee*2;ye.push(Se,Se+2,Se+1,Se+1,Se+2,Se+3)}const he=new It;he.setAttribute("position",new ft(me,3));const be=[];for(let Ee=0;Ee<me.length;Ee+=3)be.push(me[Ee]*Pe/4,me[Ee+2]*Pe/4);return he.setAttribute("uv",new ft(be,2)),he.setIndex(ye),he.computeVertexNormals(),he},Ml=(y,v,b,z,D=0)=>{if(!y||!v||!Number.isFinite(z)||(z??0)<=0)return;const L=b==="expressway"||b==="arterial"?"boulevard":b==="collector"?"avenue":b==="local"?"street":b,E=["boulevard","avenue","street","service"].includes(L??"")?L:"street",j=Wi(y,v,z+5,.045,D),Y=Wi(y,v,z,.075,D);si.push(j),Yi[E].push(Y)};for(const y of ka){for(const v of y.streets??[])Ml(v.from,v.to,v.class,v.widthMetres??v.width_metres);for(const v of[...y.sdRoads??[],...y.hdRoads??[]]){const b=("widthMetres"in v?v.widthMetres:void 0)??("width_metres"in v?v.width_metres:void 0),z=("pathKm"in v?v.pathKm:void 0)??("path_km"in v?v.path_km:void 0);if(z&&z.length>=2)for(let D=0;D+1<z.length;D+=1)Ml({x_km:z[D][0],y_km:z[D][1]},{x_km:z[D+1][0],y_km:z[D+1][1]},v.class,b,Number(v.layer??0)>0?6:0);else"from"in v&&"to"in v&&v.from&&v.to&&Ml(v.from,v.to,v.class,b)}}const Fo=si.length>0?Nn(si,!1):null;for(const y of si)y.dispose();Fo&&(Fo.computeBoundingSphere(),li.add(new pt(Fo,Qr)),Ki.push(Fo));for(const y of Object.keys(Yi)){const v=Yi[y];if(v.length===0)continue;const b=Nn(v,!1);for(const z of v)z.dispose();b&&(b.computeBoundingSphere(),li.add(new pt(b,Zr[y])),Ki.push(b))}li.visible=!1,U.add(li);const Xi=new _a,ii=[],Zi=new Map,jn={polygonOffset:!0,polygonOffsetFactor:-6,polygonOffsetUnits:-6},pa={white:new Dn({color:15330276,transparent:!0,opacity:.92,side:ct,depthWrite:!1,...jn}),yellow:new Dn({color:15123291,transparent:!0,opacity:.96,side:ct,depthWrite:!1,...jn}),median:new Ye({color:8225402,roughness:.9,transparent:!0,opacity:.95,side:ct,depthWrite:!1,...jn}),paving:new Ye({color:9407872,roughness:.96,transparent:!0,opacity:.85,side:ct,depthWrite:!1,...jn}),arrow:new Dn({color:14674143,transparent:!0,opacity:.95,side:ct,depthWrite:!1,...jn}),crosswalk:new Dn({color:15921380,transparent:!0,opacity:.9,side:ct,depthWrite:!1,...jn}),connectorLeft:new Dn({color:13620428,transparent:!0,opacity:.3,side:ct,depthWrite:!1,...jn}),connectorRight:new Dn({color:13620428,transparent:!0,opacity:.3,side:ct,depthWrite:!1,...jn}),connectorThrough:new Dn({color:13620428,transparent:!0,opacity:.3,side:ct,depthWrite:!1,...jn})},ri=(y,v)=>{const b=Zi.get(v);b?b.push(y):Zi.set(v,[y])},Fn=(y,v=0)=>{const b=y.map(z=>({...z,x:(z.x_km-it)*st,z:(z.y_km-it)*st}));return b.map((z,D)=>{const L=b[Math.max(0,D-1)],E=b[Math.min(b.length-1,D+1)],j=E.x-L.x,Y=E.z-L.z,ne=Math.max(1e-9,Math.hypot(j,Y));return{x:z.x-Y/ne*v*O,z:z.z+j/ne*v*O}})},hs=(y,v,b)=>{if(y.length<2)return null;const z=[],D=[];for(let E=0;E<y.length;E+=1){const j=y[Math.max(0,E-1)],Y=y[Math.min(y.length-1,E+1)],ne=Y.x-j.x,te=Y.z-j.z,me=Math.max(1e-9,Math.hypot(ne,te)),ye=-te/me,he=ne/me,be=v*O*.5,Ee=Mt(y[E].x,y[E].z)+b*O;z.push(y[E].x+ye*be,Ee,y[E].z+he*be,y[E].x-ye*be,Ee,y[E].z-he*be)}for(let E=0;E+1<y.length;E+=1){const j=E*2;D.push(j,j+2,j+1,j+1,j+2,j+3)}const L=new It;return L.setAttribute("position",new ft(z,3)),L.setIndex(D),L.computeVertexNormals(),L},Sl=(y,v,b,z,D)=>{const L=hs(Fn(y,v),b,z);L&&ri(L,D)},Ou=(y,v,b,z,D,L,E=.1)=>{const j=Fn(y,v);for(let Y=0;Y+1<j.length;Y+=1){const ne=j[Y],te=j[Y+1],ye=Math.hypot(te.x-ne.x,te.z-ne.z)*Pe;for(let he=0;he<ye;he+=z+D){const be=Math.min(ye,he+z);if(be<=he+.05)continue;const Ee=he/Math.max(1e-9,ye),Se=be/Math.max(1e-9,ye),Ue=hs([{x:Ve.lerp(ne.x,te.x,Ee),z:Ve.lerp(ne.z,te.z,Ee)},{x:Ve.lerp(ne.x,te.x,Se),z:Ve.lerp(ne.z,te.z,Se)}],b,E);Ue&&ri(Ue,L)}}},Ir=(y,v,b,z="straight",D=.12)=>{const L=Fn(y,v);if(L.length<2)return;let E=0;for(let Be=0;Be+1<L.length;Be+=1)E+=Math.hypot(L[Be+1].x-L[Be].x,L[Be+1].z-L[Be].z);const j=Math.max(2*O,E-16*O);let Y=0,ne=L[0],te=L[1];for(let Be=0;Be+1<L.length;Be+=1){const Xe=Math.hypot(L[Be+1].x-L[Be].x,L[Be+1].z-L[Be].z);if(Y+Xe>=j){ne=L[Be],te=L[Be+1];break}Y+=Xe}const me=te.x-ne.x,ye=te.z-ne.z,he=Math.max(1e-9,Math.hypot(me,ye)),be=me/he,Ee=ye/he,Se=-Ee,Ue=be,Le={x:ne.x+be*Math.min(8*O,he*.45),z:ne.z+Ee*Math.min(8*O,he*.45)},Ae=4.8*O,ke=1.35*O,qe={x:Le.x+be*Ae*.55,z:Le.z+Ee*Ae*.55},Oe={x:Le.x-be*Ae*.45,z:Le.z-Ee*Ae*.45},Fe={x:Le.x-be*Ae*.05,z:Le.z-Ee*Ae*.05},Ge=z==="left"?-1:z==="right"?1:0,De=[qe,{x:Fe.x+Se*ke,z:Fe.z+Ue*ke},Oe,{x:Fe.x-Se*ke,z:Fe.z-Ue*ke}];if(Ge!==0){const Be=Ge*1.2*O;De[0]={x:De[0].x+Se*Be,z:De[0].z+Ue*Be}}const He=Mt(Le.x,Le.z)+D*O,Ke=new It;Ke.setAttribute("position",new ft(De.flatMap(Be=>[Be.x,He,Be.z]),3)),Ke.setIndex([0,1,2,0,2,3]),Ke.computeVertexNormals(),ri(Ke,b)};for(const y of ka){for(const v of y.hdRoads??[]){const b=v,z=b.centreline??b.pathKm?.map(([me,ye])=>({x_km:me,y_km:ye}))??b.path_km?.map(([me,ye])=>({x_km:me,y_km:ye}))??[];if(z.length<2)continue;const D=Number(b.layer??0)>0?6:0,L=b.lanes??[],E=Math.max(1,L.length?Math.max(...L.map(me=>Number(me.indexFromCurb??me.index??0)+1)):v.class==="expressway"||v.class==="arterial"?3:v.class==="collector"?2:1),j=(v.widthMetres??10)*.5;Sl(z,-j+.35,.14,.11+D,pa.white),Sl(z,j-.35,.14,.11+D,pa.white);const Y=(v.widthMetres??10)/Math.max(2,E*2),ne=L.length>0?[-1,1].flatMap(me=>{const ye=L.filter(he=>Number(he.direction??me)===me).map(he=>Math.abs(Number(he.offsetMetres??0))).filter(he=>Number.isFinite(he)).sort((he,be)=>he-be);return ye.slice(0,-1).map((he,be)=>me*((he+ye[be+1])*.5))}):[-1,1].flatMap(me=>Array.from({length:Math.max(0,E-1)},(ye,he)=>me*(he+1)*Y));for(const me of ne)Ou(z,me,.13,v.class==="expressway"?6:3,v.class==="expressway"?9:5,pa.white,.1+D);const te=Number(b.medianMetres??0);te>.3?Sl(z,0,te*.85,.115+D,pa.median):(v.class==="arterial"||v.class==="expressway")&&Sl(z,0,.16,.12+D,pa.yellow);for(const me of L){const ye=Number(me.offsetMetres??0),be=(me.markings??[]).find(Se=>String(Se.kind).toLowerCase()==="arrow")?.arrow,Ee=Number(me.direction??1)<0?z.slice().reverse():z;be&&Ir(Ee,ye,pa.arrow,String(be).toLowerCase().includes("left")?"left":String(be).toLowerCase().includes("right")?"right":"straight",.12+D)}}for(const v of y.lanes??[]){const b=v.path;if(b&&b.length>=2){const z=Number(v.direction??1)<0?b.slice().reverse():b;for(const D of v.markings??[])String(D.kind).toLowerCase()==="arrow"&&Ir(z,Number(v.offsetMetres??0),pa.arrow,String(D.arrow??"straight").toLowerCase().includes("left")?"left":String(D.arrow??"").toLowerCase().includes("right")?"right":"straight")}}for(const v of[...y.connectors??[],...(y.junctions??[]).flatMap(b=>b.connectors??[])]){const b=v.path??v.centreline;if(!b||b.length<2)continue;const z=String(v.movement??"through").toLowerCase(),D=z.includes("left")?pa.connectorLeft:z.includes("right")?pa.connectorRight:pa.connectorThrough;Sl(b,0,v.widthMetres??.22,.14,D),Ir(b,0,D,z)}for(const v of y.junctions??[])for(const b of v.crosswalks??[]){if(b.length<2)continue;const z=b[0],D=b[b.length-1],L=D.x_km-z.x_km,E=D.y_km-z.y_km,j=Math.hypot(L,E);if(j<.001)continue;const Y=-E/j,ne=L/j;for(let te=-3;te<=3;te+=1){const me=te*.9/1e3,ye={x_km:z.x_km+Y*me-L/j*2/1e3,y_km:z.y_km+ne*me-E/j*2/1e3},he={x_km:z.x_km+Y*me+L/j*2/1e3,y_km:z.y_km+ne*me+E/j*2/1e3};Sl([ye,he],0,.45,.15,pa.crosswalk)}}}for(const[y,v]of Zi){if(v.length===0)continue;const b=v.length===1?v[0]:Nn(v,!1);if(!b){for(const z of v)z.dispose();continue}if(b!==v[0])for(const z of v)z.dispose();b.computeBoundingSphere(),ii.push(b),Xi.add(new pt(b,y))}Xi.visible=!1,U.add(Xi);const $a=new _a,a0=[],Jr=new Map,va=(y,v)=>{const b=Jr.get(v);b?b.push(y):Jr.set(v,[y])},Tl=[],$r=new Map,ec=[],Qi=[],Ii=[],Gn=[],tc=new Ye({color:13617078,roughness:.86,transparent:!0,opacity:.96}),qn=new Ye({color:7828331,roughness:.74,transparent:!0,opacity:.96}),Cl=new Ye({color:3423042,roughness:.46,metalness:.12,transparent:!0,opacity:.98}),_l=new Ye({color:6581616,roughness:.72,transparent:!0,opacity:.78}),ci=new Ye({color:9410197,roughness:.82,transparent:!0,opacity:.94}),Ji=new Ye({color:6716805,roughness:.36,metalness:.08,transparent:!0,opacity:.85}),Ua=(y,v,b,z,D,L,E,j=0)=>{const Y=new Ar(z*O,D*O,L*O);Y.rotateY(-j),Y.translate(y,v,b),va(Y,E)},ds=(y,v,b,z)=>{const D=Fn([y])[0],L=Fn([v])[0];if(!D||!L)return;const E=Math.hypot(L.x-D.x,L.z-D.z)*Pe;if(E<1.2)return;const j=Math.atan2(L.z-D.z,L.x-D.x),Y=(D.x+L.x)*.5,ne=(D.z+L.z)*.5,te=Mt(Y,ne)+b*O*.5;Ua(Y,te,ne,E,b,.28,z,j),Ua(Y,te+b*O*.53,ne,E+.1,.12,.44,qn,j)},ac=y=>y?.toLowerCase().includes("curtain")?7046542:y?.toLowerCase().includes("stone")?11642255:y?.toLowerCase().includes("brick")?11109747:8752269;for(const y of ka){for(const v of y.compounds??[]){const b=v.boundary??v.wallRing;if(!b||b.length<3)continue;for(let D=0;D<b.length;D+=1)ds(b[D],b[(D+1)%b.length],v.fenceHeightMetres??1.8,tc);const z=v.gatePoints??[];if(z.length>=2){const D=Fn([z[0]])[0],L=Fn([z[1]])[0],E=(D.x+L.x)*.5,j=(D.z+L.z)*.5,Y=Math.atan2(L.z-D.z,L.x-D.x),ne=-Math.sin(Y),te=Math.cos(Y),me=Mt(E,j);Ua(E-ne*5.5*O,me+2.1*O,j-te*5.5*O,.9,4.2,.9,qn,Y),Ua(E+ne*5.5*O,me+2.1*O,j+te*5.5*O,.9,4.2,.9,qn,Y),Ua(E,me+3.4*O,j,12.4,1.15,.45,Cl,Y),Ua(E,me+1*O,j,.22,1.8,.18,Cl,Y)}for(const D of v.paths??[]){const L=hs(Fn(D),(v.roadWidthMetres??5)+.3,.12);L&&va(L,pa.paving)}}for(const v of y.buildings??[]){if(v.footprint.length<3)continue;const b=Fn(v.footprint),z=Mt(b.reduce((ye,he)=>ye+he.x,0)/b.length,b.reduce((ye,he)=>ye+he.z,0)/b.length),D=v,L=Number(D.heightMetres??D.height_metres??3.2),E=Number(v.floors??Math.max(1,Math.round(L/3.2))),j=Number(v.podiumHeightMetres??0);if(j>0){const ye=hs([...b,b[0]],.24,0);if(ye){const he=ye.attributes.position;for(let be=0;be<he.count;be+=1)he.setY(be,z+j*O);he.needsUpdate=!0,va(ye,ci)}}const Y=ac(v.facade);let ne=$r.get(Y);ne||(ne=new Ye({color:Y,roughness:.56,metalness:v.facade?.toLowerCase().includes("curtain")?.18:.02,transparent:!0,opacity:.9}),$r.set(Y,ne),Tl.push(ne));const te=v.balconyBays&&v.balconyBays>0?2:4;for(let ye=1;ye<E;ye+=te){const he=hs([...b,b[0]],v.balconyBays&&v.balconyBays>0?.34:.16,0);if(!he)continue;const be=he.attributes.position,Ee=z+ye*3.2*O;for(let Se=0;Se<be.count;Se+=1)be.setY(Se,Ee);be.needsUpdate=!0,va(he,v.balconyBays&&v.balconyBays>0?_l:ne)}const me=Math.max(1,Math.min(8,Number(v.entranceCount??1)));for(let ye=0;ye<me;ye+=1){const he=b[ye%b.length],be=b[(ye+1)%b.length],Ee=(ye+.5)/me,Se=Ve.lerp(he.x,be.x,Ee),Ue=Ve.lerp(he.z,be.z,Ee),Le=be.x-he.x,Ae=be.z-he.z;Ua(Se,z+1.15*O,Ue,1.4,2.3,.08,Ji,Math.atan2(Ae,Le))}}for(const v of y.junctions??[])for(const b of v.signalHeads??[]){const z=b.position;if(!z)continue;const D=(z.x_km-it)*st,L=(z.y_km-it)*st,E=Mt(D,L),j=Number(b.heightMetres??5.8);Ua(D,E+j*O*.5,L,.12,j,.12,Cl);const Y=String(b.aspects?.[0]??"red").toLowerCase(),ne=new Ye({color:Y.includes("green")?5821053:Y.includes("yellow")?15188815:14110029,emissive:Y.includes("green")?1457444:Y.includes("yellow")?4799250:4592403,emissiveIntensity:.55,roughness:.3,transparent:!0,opacity:.98});Qi.push(ne);const te=new Gs(.22*O,8,6);te.translate(D,E+(j-.45)*O,L),ec.push(te),$a.add(new pt(te,ne))}}for(const[y,v]of Jr){if(v.length===0)continue;const b=v.length===1?v[0]:Nn(v,!1);if(!b){for(const z of v)z.dispose();continue}if(b!==v[0])for(const z of v)z.dispose();b.computeBoundingSphere(),a0.push(b),$a.add(new pt(b,y))}const $i=new Ye({color:11777193,roughness:.85});Gn.push($i);const Go=[4157496,5013312,3629871,5671493],nc=new Map,n0=y=>{let v=nc.get(y);return v||(v=new Ye({color:y,roughness:.95,flatShading:!0}),nc.set(y,v),Gn.push(v)),v},ms=new Map,Bu=(y,v,b,z,D,L)=>{let E=L>>>0;const j=()=>(E=E*1664525+1013904223>>>0,E/4294967296),Y=new Gs(z,7,5);Y.scale(1,D,1),Y.translate(y,v+z*D*.62,b);const ne=n0(Go[Math.floor(j()*Go.length)]),te=ms.get(ne);te?te.push(Y):ms.set(ne,[Y])},o0=(()=>{const v=new Uint8Array(16384);let b=9137;const z=()=>(b=b*1664525+1013904223>>>0,b/4294967296),D=Array.from({length:10},()=>({x:6+z()*52,w:2.1+z()*2.5,lean:(z()-.5)*20,tint:z()}));for(let E=0;E<64;E++)for(let j=0;j<64;j++){const Y=1-E/64;let ne=-1;for(const me of D){const ye=me.x+me.lean*(1-Y);if(Math.abs(j-ye)<me.w*(.3+Y*.7)){ne=me.tint;break}}const te=(E*64+j)*4;v[te]=58+ne*34,v[te+1]=ne<0?0:124+ne*58-Y*22,v[te+2]=48+ne*26,v[te+3]=ne<0?0:255}const L=new fa(v,64,64);return L.magFilter=la,L.minFilter=al,L.generateMipmaps=!0,L.colorSpace=So,L.needsUpdate=!0,new Ye({map:L,alphaTest:.5,side:ct,roughness:.95,vertexColors:!0})})();Gn.push(o0);{const y=[],v=[],b=[],z=[];let D=0,L=4200,E=5151;const j=()=>(E=E*1664525+1013904223>>>0,E/4294967296),Y=(ne,te,me,ye)=>{if(L--<=0)return;const he=.75+j()*.45;for(let be=0;be<2;be++){const Ee=j()*Math.PI+be*Math.PI/2,Se=Math.cos(Ee)*ye/2,Ue=Math.sin(Ee)*ye/2;y.push(ne-Se,me,te-Ue,ne+Se,me,te+Ue,ne+Se,me+ye*.95,te+Ue,ne-Se,me+ye*.95,te-Ue),v.push(0,0,1,0,1,1,0,1);for(let Le=0;Le<4;Le++)b.push(he,he*(.95+j()*.12),he*.88);z.push(D,D+1,D+2,D,D+2,D+3),D+=4}};for(const ne of o.modernCities??[]){for(const Se of ne.parcels??[]){if(!String(Se.useType??"").toLowerCase().includes("park")||!Se.ring||Se.ring.length<3)continue;let Le=1/0,Ae=-1/0,ke=1/0,qe=-1/0;for(const Fe of Se.ring)Le=Math.min(Le,Fe.x_km),Ae=Math.max(Ae,Fe.x_km),ke=Math.min(ke,Fe.y_km),qe=Math.max(qe,Fe.y_km);const Oe=Math.min(24,Math.max(3,Math.round((Ae-Le)*(qe-ke)*1e6*.03)));for(let Fe=0;Fe<Oe*3&&Fe<90;Fe++){const Ge=Le+j()*(Ae-Le),De=ke+j()*(qe-ke),He=(Ge-it)*st,Ke=(De-it)*st;Y(He,Ke,Mt(He,Ke)+.06*O,.55*O*(.75+j()*.5)*12.5)}}for(const Se of ne.hdRoads??[]){const Ue=Number(Se.medianMetres??0);if(Ue<1.5||Se.bridge||Number(Se.layer??0)!==0)continue;const Le=Se.centreline;if(!Le||Le.length<2)continue;const Ae=Le.map(qe=>({x:(qe.x_km-it)*st,z:(qe.y_km-it)*st})),ke=Se.class==="expressway";for(let qe=0;qe+1<Ae.length;qe++){const Oe=Ae[qe],Fe=Ae[qe+1],Ge=Math.hypot(Fe.x-Oe.x,Fe.z-Oe.z)*Pe,De=Math.max(1,Math.round(Ge/2.2));for(let He=0;He<De;He++){const Ke=(He+.5)/De,Be=Oe.x+(Fe.x-Oe.x)*Ke,Xe=Oe.z+(Fe.z-Oe.z)*Ke,rt=Mt(Be,Xe)+.1*O,$e=(j()-.5)*Math.max(.3,Ue*.42)*O,je=-((Fe.z-Oe.z)/Math.max(1e-6,Math.hypot(Fe.x-Oe.x,Fe.z-Oe.z))),Ze=(Fe.x-Oe.x)/Math.max(1e-6,Math.hypot(Fe.x-Oe.x,Fe.z-Oe.z));if(ke){const ot=new Ar(Ge/De*O*1.05,.8*O,.35*O);ot.rotateY(-Math.atan2(Fe.z-Oe.z,Fe.x-Oe.x)),ot.translate(Be,rt+.4*O,Xe);const Tt=ms.get($i);Tt?Tt.push(ot):ms.set($i,[ot]);continue}Bu(Be+je*$e,rt,Xe+Ze*$e,(.42+j()*.4)*O*12.5*.09+.42*O,.7+j()*.3,qe*31+He*7+(Se.id??0)*977>>>0),j()<.35&&Y(Be,Xe,rt,(.4+j()*.25)*O*12.5*.09+.3*O)}}}const te=[],me=[];for(const Se of ne.hdRoads??[]){if(Number(Se.layer??0)!==0||Se.class!=="arterial"&&Se.class!=="collector")continue;const Ue=Se.centreline;if(!Ue||Ue.length<2)continue;const Le=Ue.map(ke=>({x:(ke.x_km-it)*st,z:(ke.y_km-it)*st}));let Ae=0;for(let ke=0;ke+1<Le.length;ke++){const qe=Le[ke],Oe=Le[ke+1],Fe=Math.hypot(Oe.x-qe.x,Oe.z-qe.z),Ge=(Oe.x-qe.x)/Math.max(1e-9,Fe),De=(Oe.z-qe.z)/Math.max(1e-9,Fe);let He=Ae;for(;He<Fe;){const Ke=Math.round((He+Ae)/(35*O))%2===0?1:-1,Be=qe.x+Ge*He,Xe=qe.z+De*He,rt=Math.atan2(-De,Ge),$e=-De*Ke,je=Ge*Ke,Ze=Be+$e*(Se.widthMetres*.5+4.5)*O,ot=Xe+je*(Se.widthMetres*.5+4.5)*O,Tt=Mt(Ze,ot),Lt=new Er(.06*O,.1*O,7.6*O,6);Lt.translate(Ze,Tt+3.8*O,ot),te.push(Lt);const Rt=2.4*O,Gt=new Ar(Rt,.09*O,.09*O);Gt.translate(Rt/2,0,0),Gt.rotateY(rt),Gt.translate(Ze,Tt+7.45*O,ot),te.push(Gt);const Qt=new Ar(.3*O,.13*O,.8*O);Qt.translate(Rt-.25*O,-.14*O,0),Qt.rotateY(rt),Qt.translate(Ze,Tt+7.45*O,ot),me.push(Qt),He+=35*O}Ae+=Fe}}const ye=new Ye({color:4871520,roughness:.6}),he=new Ye({color:14213864,roughness:.4});if(Gn.push(ye,he),te.length){const Se=Nn(te,!1);for(const Ue of te)Ue.dispose();if(Se){Se.computeBoundingSphere();const Ue=new pt(Se,ye);Ue.castShadow=!0,$a.add(Ue)}}if(me.length){const Se=Nn(me,!1);for(const Ue of me)Ue.dispose();Se&&(Se.computeBoundingSphere(),$a.add(new pt(Se,he)))}const be=new Ye({color:10134696,roughness:.5,metalness:.35});Gn.push(be);const Ee=[];for(const Se of ne.hdRoads??[]){if(Number(Se.layer??0)!==0||Se.class!=="arterial"||Se.bridge)continue;const Ue=Se.centreline;if(!Ue||Ue.length<2)continue;const Le=Ue.map(Ae=>({x:(Ae.x_km-it)*st,z:(Ae.y_km-it)*st}));for(const Ae of[-1,1])for(let ke=0;ke+1<Le.length;ke++){const qe=Le[ke],Oe=Le[ke+1],Fe=Math.hypot(Oe.x-qe.x,Oe.z-qe.z)*Pe,Ge=Math.max(1,Math.round(Fe/3)),De=-((Oe.z-qe.z)/Math.max(1e-9,Math.hypot(Oe.x-qe.x,Oe.z-qe.z))),He=(Oe.x-qe.x)/Math.max(1e-9,Math.hypot(Oe.x-qe.x,Oe.z-qe.z)),Ke=Ae*(Se.widthMetres*.5+3.6)*O;for(let Be=0;Be<Ge;Be++){const Xe=Be/Ge,rt=(Be+1)/Ge,$e=qe.x+(Oe.x-qe.x)*Xe+De*Ke,je=qe.z+(Oe.z-qe.z)*Xe+He*Ke,Ze=qe.x+(Oe.x-qe.x)*rt+De*Ke,ot=qe.z+(Oe.z-qe.z)*rt+He*Ke,Tt=Mt(($e+Ze)/2,(je+ot)/2),Lt=Math.atan2(-(ot-je),Ze-$e),Rt=Math.max(.05,Math.hypot(Ze-$e,ot-je));for(const Qt of[.52,1.02]){const ya=new Ar(Rt,.05*O,.045*O);ya.rotateY(-Lt),ya.translate(($e+Ze)/2,Tt+Qt*O,(je+ot)/2),Ee.push(ya)}const Gt=new Er(.022*O,.022*O,1.06*O,5);Gt.translate(Ze,Tt+.53*O,ot),Ee.push(Gt)}}}if(Ee.length){const Se=Nn(Ee,!1);for(const Ue of Ee)Ue.dispose();if(Se){Se.computeBoundingSphere();const Ue=new pt(Se,be);Ue.castShadow=!0,$a.add(Ue)}}}for(const[ne,te]of ms){if(te.length===0)continue;const me=te.length===1?te[0]:Nn(te,!1);if(!me)continue;me.computeBoundingSphere();const ye=new pt(me,ne);$a.add(ye)}if(D>0){const ne=new It;ne.setAttribute("position",new ft(y,3)),ne.setAttribute("uv",new ft(v,2)),ne.setAttribute("color",new ft(b,3)),ne.setIndex(z),ne.computeVertexNormals(),$a.add(new pt(ne,o0))}}$a.visible=!1,U.add($a);const l0=[6259528,7311698,5141312,8231509,9084254].map(y=>new Zt(y));let er=null;const s0=()=>{if(er)return er;const y=64,v=new Uint8Array(y*y*4);let b=9931;const z=()=>(b=b*1664525+1013904223>>>0,b/4294967296);for(let L=0;L<26;L++){const E=6+z()*52,j=6+z()*52,Y=z()*Math.PI,ne=5.5+z()*5.5,te=2.4+z()*2.4,me=Math.cos(Y),ye=Math.sin(Y),he=.72+z()*.36;for(let be=0;be<y;be++)for(let Ee=0;Ee<y;Ee++){const Se=Ee+.5-E,Ue=be+.5-j,Le=Se*me+Ue*ye,Ae=-Se*ye+Ue*me;if(Le*Le/(ne*ne)+Ae*Ae/(te*te)>1)continue;const ke=(be*y+Ee)*4,qe=226*he;v[ke]=qe,v[ke+1]=qe,v[ke+2]=qe,v[ke+3]=255}}const D=new fa(v,y,y);return D.colorSpace=So,D.magFilter=la,D.minFilter=al,D.generateMipmaps=!0,D.needsUpdate=!0,er=D,D},tr=new Ye({map:s0(),color:16777215,vertexColors:!0,roughness:.88,side:ct,alphaTest:.45}),en=new Ye({color:7035205,roughness:.95});Gn.push(tr,en);const ar=(y,v,b,z,D)=>{let L=D>>>0;const E=()=>(L=L*1664525+1013904223>>>0,L/4294967296),j=z*(.36+E()*.12),Y=[],ne=[],te=[];let me=0;const ye=(He,Ke,Be,Xe,rt,$e,je,Ze)=>{const ot=Xe-He,Tt=rt-Ke,Lt=$e-Be,Rt=Math.hypot(ot,Tt,Lt)||.01,Gt=ot/Rt,Qt=Tt/Rt,ya=Lt/Rt;let sn=-Qt,xt=Gt,Sa=0;const Qa=Math.hypot(sn,xt,Sa);Qa<.05?(sn=1,xt=0,Sa=0):(sn/=Qa,xt/=Qa,Sa/=Qa);const Zo=Qt*Sa-ya*xt,vo=ya*sn-Gt*Sa,Ts=Gt*xt-Qt*sn,ja=5;for(let rn=0;rn<ja;rn++){const Cs=rn/ja*Math.PI*2,xa=(rn+1)/ja*Math.PI*2,qt=(zn,xi,Zn)=>{const wi=Math.cos(Zn)*xi,mr=Math.sin(Zn)*xi;return[He+Gt*Rt*zn+sn*wi+Zo*mr,Ke+Qt*Rt*zn+xt*wi+vo*mr,Be+ya*Rt*zn+Sa*wi+Ts*mr,Math.cos(Zn)*sn+Math.sin(Zn)*Zo,Math.cos(Zn)*xt+Math.sin(Zn)*vo,Math.cos(Zn)*Sa+Math.sin(Zn)*Ts]},go=qt(0,je,Cs),Xn=qt(0,je,xa),yo=qt(1,Ze,Cs),Qo=qt(1,Ze,xa);Y.push(go[0],go[1],go[2],yo[0],yo[1],yo[2],Qo[0],Qo[1],Qo[2],Xn[0],Xn[1],Xn[2]),ne.push(go[3],go[4],go[5],yo[3],yo[4],yo[5],Qo[3],Qo[4],Qo[5],Xn[3],Xn[4],Xn[5]),te.push(me,me+1,me+2,me,me+2,me+3),me+=4}};ye(0,0,0,0,j,0,.07+z*.032,.05+z*.016);const he=8+Math.floor(E()*4),be=[],Ee=[],Se=[],Ue=[],Le=[];let Ae=0;const ke=(He,Ke,Be,Xe,rt,$e,je,Ze,ot)=>{let Tt=-$e,Lt=0,Rt=Xe;const Gt=Math.hypot(Tt,Lt,Rt);Gt<.05?(Tt=1,Lt=0,Rt=0):(Tt/=Gt,Lt/=Gt,Rt/=Gt);const Qt=rt*Rt-$e*Lt,ya=$e*Tt-Xe*Rt,sn=Xe*Lt-rt*Tt,xt=je*.5,Sa=Xe*.4,Qa=rt*.4+1,Zo=$e*.4,vo=Math.hypot(Sa,Qa,Zo)||1,Ts=[[-1,-1],[1,-1],[1,1],[-1,1]];for(const[ja,rn]of Ts)be.push(He+(Tt*ja+Qt*rn)*xt,Ke+(Lt*ja+ya*rn)*xt,Be+(Rt*ja+sn*rn)*xt),Le.push(Sa/vo,Qa/vo,Zo/vo);Ue.push(0,0,1,0,1,1,0,1);for(let ja=0;ja<4;ja++)Ee.push(Ze.r*ot,Ze.g*ot,Ze.b*ot);Se.push(Ae,Ae+1,Ae+2,Ae,Ae+2,Ae+3),Ae+=4};for(let He=0;He<he;He++){const Ke=(He+E()*.7)*(Math.PI*2/he),Be=j*(.55+He/he*.6),Xe=z*(.16+E()*.16),rt=Math.cos(Ke)*Xe,$e=Math.sin(Ke)*Xe,je=Be+z*(.18+E()*.24);ye(0,Be,0,rt,je,$e,.05,.02);const Ze=z*(.16+E()*.08),ot=22+Math.floor(E()*10),Tt=l0[Math.floor(E()*l0.length)];for(let Lt=0;Lt<ot;Lt++){const Rt=E()*Math.PI*2,Gt=Math.acos(2*E()-1),Qt=Ze*(.35+E()*.75),ya=rt+Math.sin(Gt)*Math.cos(Rt)*Qt,sn=je+Math.cos(Gt)*Qt*.85,xt=$e+Math.sin(Gt)*Math.sin(Rt)*Qt,Sa=E()*Math.PI*2,Qa=Math.acos(2*E()-1),Zo=Math.sin(Qa)*Math.cos(Sa),vo=Math.cos(Qa)*.6,Ts=Math.sin(Qa)*Math.sin(Sa),ja=z*(.15+E()*.13),rn=Math.max(0,Math.min(1,(sn-.4*z)/(z*.9))),Cs=(.74+E()*.36)*(.78+.3*rn);ke(ya,sn,xt,Zo,vo,Ts,ja,Tt,Cs)}}const qe=O,Oe=new It,Fe=[];for(const He of Y)Fe.push(He*qe);Oe.setAttribute("position",new ft(Fe,3)),Oe.setAttribute("normal",new ft(ne,3)),Oe.setAttribute("uv",new ft(new Array(me*2).fill(0),2)),Oe.setIndex(te),Oe.translate(y,v,b);const Ge=new It,De=[];for(const He of be)De.push(He*qe);return Ge.setAttribute("position",new ft(De,3)),Ge.setAttribute("color",new ft(Ee,3)),Ge.setAttribute("normal",new ft(Le,3)),Ge.setAttribute("uv",new ft(Ue,2)),Ge.setIndex(Se),Ge.translate(y,v,b),{wood:Oe,leaves:Ge}},Oa=ka.flatMap(y=>y.trees??[]);{const v=Math.max(1,Math.ceil(Oa.length/1200)),b=[],z=[];let D=0;for(let j=0;j<Oa.length;j+=v){const Y=Oa[j],ne=Y?.point??{x_km:Y?.x_km??0,y_km:Y?.y_km??0},te=(ne.x_km-it)*st,me=(ne.y_km-it)*st,ye=Mt(te,me),he=Ve.clamp(Number(Y?.heightMetres??10),4,26),be=ar(te,ye,me,he,(Y?.id??D)*7919+17>>>0);b.push(be.wood),z.push(be.leaves),D+=1}const L=b.length?Nn(b,!1):null;for(const j of b)j.dispose();const E=z.length?Nn(z,!1):null;for(const j of z)j.dispose();if(L){L.computeBoundingSphere();const j=new pt(L,en);j.castShadow=!0,$a.add(j),Ii.push(L)}if(E){E.computeBoundingSphere();const j=new pt(E,tr);$a.add(j),Ii.push(E)}}const lo=new _a,nr=[],ui=[],ps={residential:new Ye({color:9410192,roughness:1,transparent:!0,opacity:.34,depthWrite:!1}),commercial:new Ye({color:10917753,roughness:.98,transparent:!0,opacity:.38,depthWrite:!1}),green:new Ye({color:6193500,roughness:1,transparent:!0,opacity:.42,depthWrite:!1})},fi={residential:[],commercial:[],green:[]},Vn=new _a,zl=[],hi=[],qo=new Ye({color:2977402,roughness:.2,metalness:.08,transparent:!0,opacity:.84,depthWrite:!1});for(const y of ka){for(const b of y.parcels??[]){if(!b.boundary||b.boundary.length<3)continue;const z=b.boundary.map(te=>({x:(te.x_km-it)*st,z:(te.y_km-it)*st})),D=new w1;D.moveTo(z[0].x,-z[0].z);for(const te of z.slice(1))D.lineTo(te.x,-te.z);D.closePath();const L=new Dg(D);L.rotateX(-Math.PI/2);const E=z.reduce((te,me)=>te+me.x,0)/z.length,j=z.reduce((te,me)=>te+me.z,0)/z.length;L.translate(0,Mt(E,j)+.032*O,0);const Y=(("landUse"in b?b.landUse:"land_use"in b?b.land_use:void 0)??"residential").toLowerCase(),ne=Y.includes("park")||Y.includes("green")||Y.includes("water")?"green":Y.includes("commercial")||Y.includes("office")||Y.includes("mixed")?"commercial":"residential";fi[ne].push(L),nr.push(L)}const v=y.river;if(v){const b=("centerline_km"in v?v.centerline_km:void 0)??("path_km"in v?v.path_km:void 0),D=("centerline"in v?v.centerline:void 0)??("path"in v?v.path:void 0)??b?.map(([E,j])=>({x_km:E,y_km:j}))??[],L=("widthMetres"in v?v.widthMetres:void 0)??("width_metres"in v?v.width_metres:void 0)??24;for(let E=0;E+1<D.length;E+=1){const j=Wi(D[E],D[E+1],L,.055);zl.push(j),hi.push(j)}}}for(const y of Object.keys(fi)){const v=fi[y];if(v.length===0)continue;const b=Nn(v,!1);b&&(b.computeBoundingSphere(),lo.add(new pt(b,ps[y])),ui.push(b))}const or=zl.length>0?Nn(zl,!1):null;or&&(or.computeBoundingSphere(),Vn.add(new pt(or,qo)),zl.length=0,zl.push(or));for(const y of nr)y.dispose();for(const y of hi)y.dispose();lo.visible=!1,Vn.visible=!1,U.add(lo,Vn);const Ba=oi();for(const y of o.modernCities??[]){const v=y.nodes??[];if(v.length===0)continue;let b=1/0,z=-1/0,D=1/0,L=-1/0;for(const Ue of v)b=Math.min(b,Ue.point.x_km),z=Math.max(z,Ue.point.x_km),D=Math.min(D,Ue.point.y_km),L=Math.max(L,Ue.point.y_km);const E=.12;b-=E,z+=E,D-=E,L+=E;const j=18,Y=(b-it)*st,ne=(z-it)*st,te=(D-it)*st,me=(L-it)*st,ye=[],he=[],be=[];for(let Ue=0;Ue<=j;Ue+=1)for(let Le=0;Le<=j;Le+=1){const Ae=Y+(ne-Y)*Le/j,ke=te+(me-te)*Ue/j;ye.push(Ae,Mt(Ae,ke)+.02*O,ke),he.push(Ae/O/4,ke/O/4)}for(let Ue=0;Ue<j;Ue+=1)for(let Le=0;Le<j;Le+=1){const Ae=Ue*(j+1)+Le;be.push(Ae,Ae+j+1,Ae+1,Ae+1,Ae+j+1,Ae+j+2)}const Ee=new It;Ee.setAttribute("position",new ft(ye,3)),Ee.setAttribute("uv",new ft(he,2)),Ee.setIndex(be),Ee.computeVertexNormals();const Se=new pt(Ee,Ba);Se.renderOrder=-1,dl.add(Se)}const vs=new _a,tn=new _a,lr=new Ye({color:5066316,roughness:.93,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),gs=new Ye({color:7955527,roughness:1,polygonOffset:!0,polygonOffsetFactor:-2,polygonOffsetUnits:-2}),Kn=new Dn({color:15197400,polygonOffset:!0,polygonOffsetFactor:-4,polygonOffsetUnits:-4}),so=new Dn({color:14200114,polygonOffset:!0,polygonOffsetFactor:-4,polygonOffsetUnits:-4}),sr=[],ir=o.worldSizeKm*.5,i0=y=>{if(y.length<2)return[];let v=y.map(([D,L])=>[D,L]);for(let D=0;D<2;D+=1){const L=[v[0]];for(let E=0;E+1<v.length;E+=1){const j=v[E],Y=v[E+1];L.push([j[0]*.75+Y[0]*.25,j[1]*.75+Y[1]*.25]),L.push([j[0]*.25+Y[0]*.75,j[1]*.25+Y[1]*.75])}L.push(v[v.length-1]),v=L}const b=[];let z=0;for(let D=0;D+1<v.length;D+=1){const L=v[D],E=v[D+1],j=Math.hypot(E[0]-L[0],E[1]-L[1])*1e3,Y=Math.max(1,Math.ceil(j/80));for(let ne=D===0?0:1;ne<=Y;ne+=1){const te=ne/Y,me=Ve.lerp(L[0],E[0],te),ye=Ve.lerp(L[1],E[1],te),he=(me-ir)*1e3*O,be=(ye-ir)*1e3*O;if(b.length>0){const Ee=b[b.length-1];z+=Math.hypot(he-Ee.x,be-Ee.z)*Pe}b.push({x:he,z:be,y:Mt(he,be)+.06*O,distanceMetres:z})}}return b.map((D,L)=>{const E=b[Math.max(0,L-1)],j=b[Math.min(b.length-1,L+1)],Y=Math.max(1e-9,Math.hypot(j.x-E.x,j.z-E.z));return{...D,nx:-(j.z-E.z)/Y,nz:(j.x-E.x)/Y}})},ga=(y,v,b=0)=>{const z=[],D=[],L=v*O*.5,E=b*O;for(const Y of y){const ne=Y.x+Y.nx*E,te=Y.z+Y.nz*E;z.push(ne+Y.nx*L,Y.y,te+Y.nz*L,ne-Y.nx*L,Y.y,te-Y.nz*L)}for(let Y=0;Y+1<y.length;Y+=1){const ne=Y*2;D.push(ne,ne+2,ne+1,ne+1,ne+2,ne+3)}const j=new It;return j.setAttribute("position",new ft(z,3)),j.setIndex(D),j.computeVertexNormals(),sr.push(j),j},Vo=(y,v,b)=>{const z=[],D=[],L=v*O*.5;for(let j=0;j+1<y.length;j+=1){const Y=y[j],ne=y[j+1],te=ne.distanceMetres-Y.distanceMetres;let me=Y.distanceMetres;for(;me<ne.distanceMetres;){const ye=me%9,he=ye<3?me:me+(9-ye),be=Math.min(ne.distanceMetres,he+3);if(he>=ne.distanceMetres||be<=he)break;const Ee=(he-Y.distanceMetres)/Math.max(te,1e-6),Se=(be-Y.distanceMetres)/Math.max(te,1e-6),Ue=z.length/3;for(const Le of[Ee,Se]){const Ae=Ve.lerp(Y.nx,ne.nx,Le),ke=Ve.lerp(Y.nz,ne.nz,Le),qe=Math.max(1e-9,Math.hypot(Ae,ke)),Oe=Ae/qe,Fe=ke/qe,Ge=Ve.lerp(Y.x,ne.x,Le)+Oe*b*O,De=Ve.lerp(Y.z,ne.z,Le)+Fe*b*O,He=Ve.lerp(Y.y,ne.y,Le)+.018*O;z.push(Ge+Oe*L,He,De+Fe*L,Ge-Oe*L,He,De-Fe*L)}D.push(Ue,Ue+2,Ue+1,Ue+1,Ue+2,Ue+3),me=be+6}}const E=new It;return E.setAttribute("position",new ft(z,3)),E.setIndex(D),E.computeVertexNormals(),sr.push(E),E};for(const y of o.roads??[]){const v=i0(y.pathKm);if(v.length<2)continue;const b=new pt(ga(v,y.profile.carriagewayWidthMetres),y.profile.paved?lr:gs);b.renderOrder=3,vs.add(b);const z=(D,L)=>{const E=new pt(D,L);E.renderOrder=4,tn.add(E)};y.class==="motorway"?(z(Vo(v,.15,-y.profile.carriagewayWidthMetres*.25),Kn),z(Vo(v,.15,y.profile.carriagewayWidthMetres*.25),Kn),z(ga(v,.14,-.16),so),z(ga(v,.14,.16),so)):y.class==="arterial"?(z(ga(v,.12,-.14),so),z(ga(v,.12,.14),so)):y.class==="collector"&&z(Vo(v,.12,0),so)}vs.visible=!1,tn.visible=!1,U.add(vs,tn);const ys=(y,v,b)=>new Ye({color:y,roughness:.92,polygonOffset:!0,polygonOffsetFactor:v,polygonOffsetUnits:v,...b}),xs={metresToScene:O,worldSizeKm:o.worldSizeKm,terrainHeightAt:Mt,layer:ys},Ko=b5(o.railways??[],xs);Ko.group.visible=!1,U.add(Ko.group);const Ha=w5(o.railLines??[],xs);U.add(Ha.group);const La=o.airport?T5(o.airport,xs):null;La&&U.add(La.group);const Po=Mt(oe.target.x,oe.target.z);oe.target.y=Po+Ft,G instanceof $o&&(G.position.y=Math.max(G.position.y,oe.target.y+Nt)),oe.update();let an=Number.POSITIVE_INFINITY,nn=Number.POSITIVE_INFINITY,bn=0;const oc=new Co,rr=new iu,cr=new lt,lc=new lt,ia=new Zt,Ht=new lt(0,1,0),vt=y=>y<=420?8:y<=900?12:y<=1800?20:y<=3600?36:80,Yo=(y,v)=>{const b=Ve.clamp(y/O*.72*v,180,8e3),z=vt(b),D=Math.hypot(oe.target.x-an,oe.target.z-nn)/O,L=Math.abs(b-bn)/Math.max(1,bn);if(D<z*5&&L<.14)return;an=oe.target.x,nn=oe.target.z,bn=b;const E=(an+1.6)/O,j=(nn+1.6)/O,Y=Math.floor((E-b)/z),ne=Math.ceil((E+b)/z),te=Math.floor((j-b)/z),me=Math.ceil((j+b)/z);let ye=0;const he=Ps[b<=320?0:1],be=new Array(he.length).fill(0);for(let Ee=te;Ee<=me&&ye<Wa;Ee+=1)for(let Se=Y;Se<=ne&&ye<Wa;Se+=1){const Ue=dt(Se,Ee,31),Le=dt(Se,Ee,32),Ae=(Se+.08+Ue*.84)*z,ke=(Ee+.08+Le*.84)*z,qe=Ae-E,Oe=ke-j;if(qe*qe+Oe*Oe>b*b)continue;const Fe=-1.6+Ae*O,Ge=-1.6+ke*O;if(Fe<=-1.6||Fe>=1.6||Ge<=-1.6||Ge>=1.6)continue;const De=Un(vn,Fe,Ge)/255;if(Un(Ia,Fe,Ge)/255>.12)continue;const Ke=Ve.clamp((De-.18)*1.45,0,.96);if(dt(Se,Ee,33)>Ke)continue;const Be=Math.max(0,Un(J,Fe,Ge)-o.stats.minElevation)*O,Xe=.68+dt(Se,Ee,34)*.76,rt=.72+dt(Se,Ee,35)*.58;rr.setFromAxisAngle(Ht,dt(Se,Ee,36)*Math.PI*2);const $e=Math.floor(dt(Se,Ee,39)*he.length)%he.length;if(be[$e]>=Wa)continue;lc.set(Fe,Be,Ge),cr.set(rt,Xe,rt),oc.compose(lc,rr,cr),he[$e].setMatrixAt(be[$e],oc);const je=.8+dt(Se,Ee,38)*.36,Ze=(dt(Se,Ee,37)-.5)*.1;ia.setRGB(je*(1+Ze),je,je*(1-Ze)),he[$e].setColorAt(be[$e],ia),be[$e]+=1,ye+=1}for(const Ee of ul)Ee.count=0;he.forEach((Ee,Se)=>{Ee.count=be[Se],Ee.instanceMatrix.needsUpdate=!0,Ee.instanceColor&&(Ee.instanceColor.needsUpdate=!0),Ee.count>0&&Ee.computeBoundingSphere()})},Xa=new kg(14869734,10130050,.4);U.add(Xa);const di=Ve.degToRad(l.sunAzimuth),Mn=Ve.degToRad(l.sunElevation),St=new y1(16773590,1.95);St.color.r*=we[0]/ge,St.color.g*=we[1]/ge,St.color.b*=we[2]/ge,St.position.set(Math.sin(di)*Math.cos(Mn)*5,Math.sin(Mn)*5,Math.cos(di)*Math.cos(Mn)*5),St.castShadow=!0;const sc=Number(new URLSearchParams(window.location.search).get("shadowmap"))||(aa?1024:4096);St.shadow.mapSize.set(sc,sc),St.shadow.camera.near=.001,St.shadow.camera.far=12,St.shadow.bias=-3e-4,St.shadow.normalBias=.5*O,U.add(St),U.add(St.target);const io=Ve.degToRad(l.windDirection),ro=Ci(np(o.waterHeightDataBase64),o.meshSize),Sn=Ci(Kl(o.waterMaskBase64),o.waterDataSize),Al=Ci(Kl(o.waterKindBase64),o.waterDataSize),ur=Ci(new Int8Array(Kl(o.flowDirectionBase64).buffer),o.waterDataSize,2),co=Ci(Kl(o.flowStrengthBase64),o.waterDataSize),on=g5(O,{hash01:dt,forest:(y,v)=>Un(vn,y,v),excluded:(y,v)=>Un(Ia,y,v),water:(y,v)=>{const b=o.waterDataSize,z=Ve.clamp(Math.round((y+1.6)/3.2*(b-1)),0,b-1),D=Ve.clamp(Math.round((v+1.6)/3.2*(b-1)),0,b-1);return Sn[D*b+z]},height:(y,v)=>{const b=o.meshSize-1,z=Ve.clamp((y+1.6)/3.2*b,0,b-1e-6),D=Ve.clamp((v+1.6)/3.2*b,0,b-1e-6),L=Math.floor(z),E=Math.floor(D),j=z-L,Y=D-E,ne=(be,Ee)=>Math.max(0,J[Ee*o.meshSize+be]-o.stats.minElevation)*O,te=ne(L,E),me=ne(L,E+1),ye=ne(L+1,E+1),he=ne(L+1,E);return j+Y<=1?te+j*(he-te)+Y*(me-te):ye+(1-j)*(me-ye)+(1-Y)*(he-ye)},elevation01:(y,v)=>(Un(J,y,v)-o.stats.minElevation)/Math.max(1,o.stats.maxElevation-o.stats.minElevation)});U.add(on.group),r==="3d"&&U.add(l5({heights:J,meshSize:o.meshSize,minElevation:o.stats.minElevation,metresToScene:O,map:Yt,waterHeights:ro,waterMask:Sn,waterMaskSize:o.waterDataSize}));const Pn=new Bc(3.2,3.2,o.meshSize-1,o.meshSize-1),Yn=Pn.attributes.position;for(let y=0;y<ro.length;y+=1){const v=Math.max(0,ro[y]-o.stats.minElevation);Yn.setZ(y,v*O+O*.18)}Yn.needsUpdate=!0;const El=new Uint8Array(Sn.length*4),Rl=new Uint8Array(Sn.length*4),Tn=o.waterDataSize,fr=.18*255,uo=new Uint8Array(Sn),ra=new Uint8Array(Al),mi=new Uint8Array(co);for(let y=0;y<Tn;y+=1)for(let v=0;v<Tn;v+=1){const b=y*Tn+v;if(Sn[b]===0||Al[b]>fr)continue;const z=Math.round(Math.min(255,Sn[b]*1.5)*.9),D=Math.max(co[b],96);for(const[L,E]of[[1,0],[-1,0],[0,1],[0,-1]]){const j=v+L,Y=y+E;if(j<0||Y<0||j>=Tn||Y>=Tn)continue;const ne=Y*Tn+j;Sn[ne]===0&&z>uo[ne]&&(uo[ne]=z,ra[ne]=Al[b],mi[ne]=D)}}for(let y=0;y<Sn.length;y+=1)El[y*4]=uo[y],El[y*4+1]=ra[y],El[y*4+2]=Math.max(mi[y],co[y]),El[y*4+3]=255,Rl[y*4]=Math.round((ur[y*2]/127*.5+.5)*255),Rl[y*4+1]=Math.round((ur[y*2+1]/127*.5+.5)*255),Rl[y*4+2]=128,Rl[y*4+3]=255;const fo=new fa(El,o.waterDataSize,o.waterDataSize,$n);fo.minFilter=la,fo.magFilter=la,fo.wrapS=Ti,fo.wrapT=Ti,fo.flipY=!0,fo.needsUpdate=!0;const Wo=new fa(Rl,o.waterDataSize,o.waterDataSize,$n);Wo.minFilter=la,Wo.magFilter=la,Wo.wrapS=Ti,Wo.wrapT=Ti,Wo.flipY=!0,Wo.needsUpdate=!0;const ic={...ll.clone(Ug.fog),uTime:{value:0},uMetresToScene:{value:O},uWind:{value:new zt(Math.cos(io),Math.sin(io))},uSunDirection:{value:St.position.clone().normalize()},uHydrologyMap:{value:fo},uFlowMap:{value:Wo}},r0=new Pa({vertexShader:_5,fragmentShader:z5,uniforms:ic,fog:!0,transparent:!0,depthWrite:!1,polygonOffset:!0,polygonOffsetFactor:-1,polygonOffsetUnits:-2,side:ct}),Kt=new pt(Pn,r0);Kt.rotation.x=-Math.PI/2,Kt.renderOrder=2,U.add(Kt);const pi=new zt(Math.cos(io),Math.sin(io)),Wn={uTime:{value:0},uCoverage:{value:l.cloudCoverage/100},uWind:{value:pi.clone().multiplyScalar(l.cloudSpeed*6e-4)},uSunColor:{value:new Zt(1,.91,.76)},uShadowMode:{value:0}},ws=new Pa({vertexShader:op,fragmentShader:lp,uniforms:Wn,transparent:!0,depthWrite:!1,side:ct}),hr=new Bc(4.4,4.4,1,1),ho=new pt(hr,ws);ho.rotation.x=-Math.PI/2;const Nl=o.stats.maxElevation-o.stats.minElevation;ho.position.y=(Nl+1800)*O,U.add(ho);{const y=new Set((new URLSearchParams(window.location.search).get("hide")??"").split(","));y.has("clouds")&&(ho.visible=!1),y.has("water")&&(Kt.visible=!1),y.has("forest")&&(Da.visible=!1)}const Hu=new URLSearchParams(window.location.search).get("hide")?.split(",").includes("forest")??!1,vi={uTime:{value:0},uCoverage:{value:l.cloudCoverage/100},uWind:{value:pi.clone().multiplyScalar(l.cloudSpeed*6e-4)},uSunColor:{value:new Zt(1,1,1)},uShadowMode:{value:1}},c0=new Pa({vertexShader:op,fragmentShader:lp,uniforms:vi,transparent:!0,depthWrite:!1,side:ct,blending:Og}),rc=new Bc(3.28,3.28,1,1),Cn=new pt(rc,c0);Cn.rotation.x=-Math.PI/2,Cn.position.y=(Nl+35)*O,Cn.renderOrder=1,Cn.visible=r==="satellite",U.add(Cn);const dr=()=>{const y=Math.max(1,K.clientWidth),v=Math.max(1,K.clientHeight),b=y/v;G instanceof $o?G.aspect=b:(G.left=-1.7*b,G.right=1.7*b,G.top=1.7,G.bottom=-1.7),G.updateProjectionMatrix(),F.setSize(y,v,!1),V&&"setSize"in V&&V.setSize(y,v)},gi=new ResizeObserver(dr);gi.observe(K),dr();const u0=new Bg,mo=new lt,Za=new lt,Dl=new lt,bs=new lt().crossVectors(new lt(0,1,0),Z).normalize(),cc=new lt().crossVectors(Z,bs).normalize();let _n=0,Xo=0,po=0;const ln={applied:!1,settled:0,frames:0,reported:!1,startedAt:performance.now()};let uc=0,Ms=0,Ss=!1;const yi=()=>{const y=u0.getDelta();_n+=y;const v=Math.sin(_n*.035+l.seed*.0017),b=.82+Math.sin(_n*.21+l.seed*.013)*.18,z=Ve.clamp(l.cloudCoverage/100+v*.11,0,1);Wn.uTime.value+=y,Wn.uCoverage.value=z,Wn.uWind.value.copy(pi).multiplyScalar(l.cloudSpeed*6e-4*b),vi.uTime.value=Wn.uTime.value,vi.uCoverage.value=z,vi.uWind.value.copy(Wn.uWind.value),ic.uTime.value+=y;const D=di+_n*.0015,L=Mn+Math.sin(_n*.012)*.035;St.position.set(Math.sin(D)*Math.cos(L)*5,Math.sin(L)*5,Math.cos(D)*Math.cos(L)*5),St.intensity=3.4*ge*(1-z*.28),ic.uSunDirection.value.copy(Z);const E=A.current,j=T.current;if(j&&j.nonce!==po){po=j.nonce,nt?.active&&Bo(!1);const De=(j.xKm-it)*st,He=(j.yKm-it)*st,Ke=Mt(De,He);if(oe.target.set(De,Ke+Ft,He),G instanceof $o){const Be=j.spanKm*1e3*O/(2*Math.tan(Ve.degToRad(G.fov*.5))),Xe=G.position.clone().sub(oe.target);if(Xe.lengthSq()<1e-9&&Xe.set(.7,.55,.7),j.pitchDeg!==void 0){const rt=Ve.degToRad(Math.min(89.5,j.pitchDeg)),$e=Ve.degToRad(j.azimuthDeg??200);Xe.set(Math.cos(rt)*Math.sin($e),Math.sin(rt),Math.cos(rt)*Math.cos($e))}G.position.copy(oe.target).add(Xe.setLength(Be))}else{const Be=G;Be.zoom=3.4/Math.max(1e-6,j.spanKm*1e3*O),Be.updateProjectionMatrix(),G.position.set(De,Ke+5.2,He+.001)}}if(E?.camera&&!ln.applied&&G instanceof $o){ln.applied=!0;const De=Be=>{const Xe=E.camera;let rt,$e;if(Xe.city!==void 0){const je=o.modernCities?.[Xe.city]?.frame;if(!je)return null;const Ze=Be[0]*O,ot=Be[2]*O,Tt=Math.cos(je.rotationRadians),Lt=Math.sin(je.rotationRadians);rt=(je.origin.x_km-it)*st+Ze*Tt-ot*Lt,$e=(je.origin.y_km-it)*st+Ze*Lt+ot*Tt}else rt=(Be[0]-it)*st,$e=(Be[2]-it)*st;return new lt(rt,Mt(rt,$e)+Be[1]*O,$e)},He=De(E.camera.pos),Ke=De(E.camera.target);if(He&&Ke){G.up.set(0,1,0),G.position.copy(He),G.lookAt(Ke),G.updateMatrixWorld(!0),oe.target.copy(Ke);const Be=E.camera.mode??"orbit";Be==="orbit"?oe.update():nt&&(nt.active&&nt.exit(),nt.enter(Be==="walk"),oe.enabled=!1,ls())}}if(nt?.active?(nt.update(y),oe.target.copy(nt.groundTarget)):oe.update(),G instanceof $o){oe.target.x=Ve.clamp(oe.target.x,-Ie+tt,Ie-tt),oe.target.z=Ve.clamp(oe.target.z,-Ie+tt,Ie-tt),oe.target.y=Mt(oe.target.x,oe.target.z)+Ft;const De=Mt(G.position.x,G.position.z);G.position.y<De+Nt&&(G.position.y=De+Nt);const He=G.position.distanceTo(oe.target),Ke=Ve.clamp(He/300,.05*O,40*O),Be=30;(Math.abs(G.near-Ke)/Ke>.12||Math.abs(G.far-Be)/Be>.12)&&(G.near=Ke,G.far=Be,G.updateProjectionMatrix())}else oe.target.x=Ve.clamp(oe.target.x,-Ie+tt,Ie-tt),oe.target.z=Ve.clamp(oe.target.z,-Ie+tt,Ie-tt),oe.target.y=Mt(oe.target.x,oe.target.z)+Ft;const Y=G instanceof vh?3.4/G.zoom:G.position.distanceTo(oe.target)*Math.tan(Ve.degToRad(G.fov*.5))*2;G.getWorldDirection(mo);const ne=G instanceof $o?Math.min(3,1/Math.max(.34,Math.abs(mo.y))):1,te=Y*Pe,me=Ve.clamp((Math.log10(Math.max(te,1))-Math.log10(2e3))/(Math.log10(2e4)-Math.log10(2e3)),0,1);oe.zoomSpeed=1+me*2;const ye=xn(te,3200,7200),he=Zc(te,3800,19e3),be=ye>.001;for(const De of ul)De.visible=be&&!Hu;Et(Eo,ye),Da.visible=he>.001,Et(yn,he),be&&Yo(Y,ne);const Ee=Y/O*ne,Se=xn(Ee,380,2450),Ue=xn(Ee,230,760),Le=Se>.001,Ae=Ue>.001;On.visible=Le,Et(Di,Se);for(const De of Hi)De.visible=Ae;for(const De of[Do,Ui,fl,Zs,Ro,No,hl])Et(De,Ue);(Le||Ae)&&Li(Y,ne),on.setFade(xn(Ee,320,780)),on.update(Y,ne,oe.target.x,oe.target.z);const ke=xn(te,7500,18e3),qe=xn(te,1800,5500),Oe=xn(te,9e3,26e3),Fe=xn(te,650,7500);vs.visible=ke>.001,Ko.group.visible=ke>.001,Ha.group.visible=ke<.999,tn.visible=qe>.001,li.visible=!1,Xi.visible=!1,dl.visible=!1,$a.visible=!1,Pr.visible=!1,lo.visible=!1,Vn.visible=!1,Et(lr,ke),Et(gs,ke),Et(Kn,qe),Et(so,qe);for(const De of Js)Et(De,Oe);Et(Ba,Oe),Et(Kr,Oe);for(const De of Object.values(Zr))Et(De,Fe);Et(Qr,Fe);for(const De of Object.values(pa))Et(De,Fe);Et(tc,Oe),Et(qn,Oe),Et(Cl,Oe),Et(_l,Oe),Et(ci,Oe),Et(Ji,Oe);for(const De of Tl)Et(De,Oe);for(const De of Qi)Et(De,Oe);for(const De of Gn)Et(De,Oe);for(const De of Object.values(ps))Et(De,Oe);if(Et(qo,Oe),S.uTime.value=_n,ue?.update(_n),wn?.update(_n,G.position.distanceTo(oe.target)*Pe,G)&&(F.shadowMap.needsUpdate=!0),pe){const De=te<1500&&!Ne;pe.enabled!==De&&(pe.enabled=De)}if(_u(y),Ru(),Du(_n),Dr){if(wn){const De=wn.lodStats();qa("lodInstances",De.instances),qa("lodHiddenStatics",De.hiddenStatics)}qa("viewSpanM",te),qa("roam",nt?.active?nt.walking?2:1:0),qa("camXm",G.position.x*Pe),qa("camYm",G.position.y*Pe),qa("camZm",G.position.z*Pe),qa("groundYm",Mt(G.position.x,G.position.z)*Pe)}Q.position.copy(G.position),Q.scale.setScalar(Math.max(1e-6,G.far*.9/9e3)),S.uCameraHeight.value=Math.max(2,G.position.y*Pe),S.uCoverage.value=z;let Ge=Ve.clamp(Y*.85,40*O,1.5);if(Za.copy(oe.target),nt?.active){const De=Math.max(0,G.position.y-Mt(G.position.x,G.position.z)),He=Math.max(90*O,De*3);He<Ge&&(Ge=He,Dl.set(mo.x,0,mo.z),Dl.lengthSq()>1e-8&&Dl.normalize(),Za.copy(G.position).addScaledVector(Dl,Ge*.62),Za.y=Mt(Za.x,Za.z))}{const De=Ge*2/St.shadow.mapSize.x,He=Za.dot(Z),Ke=Math.round(Za.dot(bs)/De)*De,Be=Math.round(Za.dot(cc)/De)*De;Za.set(0,0,0).addScaledVector(Z,He).addScaledVector(bs,Ke).addScaledVector(cc,Be),St.shadow.normalBias=Math.max(.03*O,De*2.2)}if(St.target.position.copy(Za),St.position.copy(Za).addScaledVector(Z,Ge*2.5+.05),St.shadow.camera.left=-Ge,St.shadow.camera.right=Ge,St.shadow.camera.top=Ge,St.shadow.camera.bottom=-Ge,St.shadow.camera.near=1e-4,St.shadow.camera.far=Ge*6+.2,St.shadow.bias=-(Ge*2/St.shadow.mapSize.x)*2/St.shadow.camera.far,St.shadow.camera.updateProjectionMatrix(),pn.uSunViewDir.value.copy(Z).transformDirection(G.matrixWorldInverse),ee&&k){const De=(G.position.y-Mt(G.position.x,G.position.z))*Pe,He=ee.update(!!nt?.active&&De<60&&r==="3d",G.position,18*O,.3*O,30,_n),Ke=He??k;U.environment!==Ke&&(U.environment=Ke,U.environmentIntensity=He?1.15:1,Xa.intensity=He?.3:.4)}if(uc+=1,uc%(nt?.active?10:45)===1&&(F.shadowMap.needsUpdate=!0),P.begin(),V&&r==="3d"?V.render():F.render(U,G),P.end(),!Ss&&(Ms+=1)>=3&&(Ss=!0,xh()),E&&!ln.reported){ln.frames+=1;const De=T.current?.nonce??0,He=(E.camera?ln.applied:!0)&&po===De,Ke=ni.reduce((rt,$e)=>rt+($e?$e.pending():0),0);ln.settled=He&&Ke===0?ln.settled+1:0;const Be=(performance.now()-ln.startedAt)/1e3,Xe=Be>(E.tileTimeoutSec??240);if(ln.settled>=(E.settleFrames??6)||Xe&&He){ln.reported=!0;const rt=ni.map($e=>$e?$e.summary():null);R.current?.({frames:ln.frames,waitedSec:Number(Be.toFixed(1)),timedOut:Xe,camera:{xm:G.position.x*Pe,ym:G.position.y*Pe,zm:G.position.z*Pe,groundYm:Mt(G.position.x,G.position.z)*Pe,roam:nt?.active?nt.walking?"walk":"fly":"orbit"},streamers:rt,wholeCityMounted:!!wn,render:{calls:F.info.render.calls,triangles:F.info.render.triangles,geometries:F.info.memory.geometries,textures:F.info.memory.textures},webgl2:F.capabilities.isWebGL2,composer:!!(V&&r==="3d")})}}if(Dr){const De=window;De.__SNAP__&&(De.__SNAPSHOT__=F.domElement.toDataURL("image/png"),De.__SNAP__=!1)}Xo=requestAnimationFrame(yi)};return yi(),()=>{cancelAnimationFrame(Xo),gi.disconnect(),d.current=null,P.dispose(),window.removeEventListener("keydown",Fr),nt?.dispose(),Ja?.root.remove(),Qc(),ti?.dispose(),wn?.dispose();for(const y of jo)y&&(U.remove(y.mesh),y.water&&U.remove(y.water),y.dispose());oe.dispose(),Yt.dispose(),_e.dispose(),Aa.dispose(),on.dispose(),na.dispose();for(const y of cl)y.dispose();Or.dispose(),Di.dispose(),Bn.dispose(),oa.dispose(),ma.dispose(),to.dispose(),Xs.dispose(),Kc.dispose(),ki.dispose(),Ui.dispose(),fl.dispose(),Zs.dispose(),Ro.dispose(),No.dispose(),hl.dispose(),Do.dispose();for(const y of ml)y.dispose();for(const y of Yc)y.dispose();for(const y of Js)y.dispose();for(const y of Yr.values())y.map?.dispose(),y.normalMap?.dispose(),y.dispose();for(const y of pl)y.dispose();fs.dispose(),Kr.dispose();for(const y of ii)y.dispose();for(const y of Object.values(pa))y.dispose();for(const y of a0)y.dispose();for(const y of Tl)y.dispose();for(const y of ec)y.dispose();for(const y of Qi)y.dispose();for(const y of Ii)y.dispose();for(const y of Gn)y.dispose();tc.dispose(),qn.dispose(),Cl.dispose(),_l.dispose(),ci.dispose(),Ji.dispose();for(const y of Ki)y.dispose();Qr.dispose();for(const y of Object.values(Zr))y.dispose();for(const y of e0)y.dispose();for(const y of ui)y.dispose();for(const y of Object.values(ps))y.dispose();for(const y of zl)y.dispose();qo.dispose();for(const y of sr)y.dispose();for(const y of[Ko,Ha,La])if(y){for(const v of y.geometries)v.dispose();for(const v of y.materials)v.dispose();U.remove(y.group)}lr.dispose(),gs.dispose(),Kn.dispose(),so.dispose(),Pn.dispose(),r0.dispose(),fo.dispose(),Wo.dispose(),hr.dispose(),ws.dispose(),rc.dispose(),c0.dispose(),ee?.dispose(),F.dispose(),aa&&F.forceContextLoss(),F.domElement.remove()}};return mt.useEffect(()=>{const K=m.current;if(K){C(null),gp();try{return H(K)}catch(U){console.error("[terrain3d] mount failed",U),K.replaceChildren(),xh(),C(U instanceof Error?U.message:String(U));return}}},[o,r,l.cloudCoverage,l.cloudSpeed,l.haze,l.sunAzimuth,l.sunElevation,l.windDirection]),re.jsx("div",{ref:m,className:"terrain-3d",children:B?re.jsxs("div",{className:"mount-error",role:"alert",children:[re.jsx("strong",{children:"3D 视图无法在此设备上启动"}),re.jsx("span",{children:B}),re.jsx("small",{children:"可切换到「动态卫星」或分析图层查看同一片地貌，或在设置中降低网格后重新生成。"})]}):null})}function E5(o,l=!1){return window.__TAURI_INTERNALS__.transformCallback(o,l)}async function Oh(o,l={},r){return window.__TAURI_INTERNALS__.invoke(o,l,r)}const R5=Object.freeze(Object.defineProperty({__proto__:null,invoke:Oh,transformCallback:E5},Symbol.toStringTag,{value:"Module"})),Ch=()=>typeof window>"u"?{}:window;async function N5(){if(Ch().__terrainLabAutomation)return Ch().__terrainLabAutomation??null;try{return await Oh("automation_spec")??null}catch(o){return typeof window<"u"&&"__TAURI_INTERNALS__"in window&&console.error("[automation]",o),null}}function Zl(o,l={}){const r=Ch().__terrainLabEvent;if(r){r(o,l);return}Oh("automation_event",{event:o,detail:l}).catch(()=>{})}function D5(){for(const o of["log","info","warn","error"]){const l=console[o].bind(console);console[o]=(...r)=>{l(...r),Zl("console",{level:o,text:r.map(c=>c instanceof Error?c.stack??c.message:typeof c=="string"?c:JSON.stringify(c)).join(" ").slice(0,2e3)})}}window.addEventListener("error",o=>Zl("console",{level:"uncaught",text:`${o.message} @ ${o.filename}:${o.lineno}`})),window.addEventListener("unhandledrejection",o=>Zl("console",{level:"unhandledrejection",text:String(o.reason)}))}const qs={arid:{name:"干旱高山",description:"裸岩、冲积扇与干涸河谷",rainfall:425,evaporation:975,windSpeed:11,windDirection:35},temperate:{name:"温带湿润",description:"森林、活跃河网与冲积平原",rainfall:1275,evaporation:600,windSpeed:7,windDirection:225},glacial:{name:"高寒冰川",description:"冰斗、雪线与冰川侵蚀谷",rainfall:750,evaporation:250,windSpeed:13,windDirection:285}},ph={mountainRange:{name:"山脉",description:"造山带与深切河谷"},hills:{name:"丘陵",description:"低缓山丘与谷地"},plains:{name:"平原",description:"低起伏与大型河网"},plateau:{name:"高原",description:"台地、陡崖与峡谷"},coastal:{name:"海岸",description:"大陆边缘与海湾"},archipelago:{name:"群岛",description:"岛链、海峡与浅海"}},k5={seed:284735,preset:"temperate",landform:"mountainRange",gridSize:aa?256:512,worldSizeKm:80,rainfall:qs.temperate.rainfall,evaporation:qs.temperate.evaporation,windSpeed:qs.temperate.windSpeed,windDirection:qs.temperate.windDirection,sunAzimuth:220,sunElevation:31,haze:2.5,cloudCoverage:35,cloudSpeed:24},lu={discharge:"汇流量",lake:"湖泊",wetland:"湿地",floodplain:"洪泛区",basin:"流域",riverOrder:"河流级序",geology:"岩性分区",soilDepth:"土壤深度",landCover:"地表覆盖",travelCost:"通行成本",hazard:"灾害风险",settlementSuitability:"聚落适宜度",agriculturalSuitability:"农业适宜度",urbanLand:"城市占地",cultivatedLand:"耕作带",infrastructure:"聚落与道路"};function el({label:o,value:l,min:r,max:c,step:i=1,unit:f="",onChange:h}){return re.jsxs("label",{className:"control",children:[re.jsxs("span",{children:[re.jsx("b",{children:o}),re.jsxs("output",{children:[l,f]})]}),re.jsx("input",{type:"range",value:l,min:r,max:c,step:i,onChange:g=>h(Number(g.target.value))})]})}function U5(){const[o,l]=mt.useState(k5),[r,c]=mt.useState(null),[i,f]=mt.useState(!1),[h,g]=mt.useState("等待生成"),[m,d]=mt.useState(0),[x,M]=mt.useState(4096),[T,A]=mt.useState(null),[R,B]=mt.useState("3d"),[C,H]=mt.useState("discharge"),[K,U]=mt.useState(null),[_,I]=mt.useState(null),Z=mt.useRef(null),[le,X]=mt.useState(null),[se,we]=mt.useState(null),[ge,G]=mt.useState(0),F=mt.useRef(!1);mt.useEffect(()=>{N5().then(J=>{J&&(D5(),we(J),J.config&&l(_e=>({..._e,...J.config})),G(1))})},[]),mt.useEffect(()=>{let J;return dp("terrain-progress",_e=>{g(_e.payload.stage),d(_e.payload.progress)}).then(_e=>{J=_e}),()=>J?.()},[]),mt.useEffect(()=>{const J=_e=>{!r||_e.ctrlKey||_e.altKey||_e.metaKey||_e.target?.matches("input, select, textarea")||(_e.key.toLowerCase()==="s"&&B("satellite"),_e.key==="3"&&B("3d"))};return window.addEventListener("keydown",J),()=>window.removeEventListener("keydown",J)},[r]);const ie=mt.useMemo(()=>(o.worldSizeKm*1e3/o.gridSize).toFixed(1),[o.gridSize,o.worldSizeKm]),$=mt.useMemo(()=>{if(!r?.modernCities?.length)return[];const J=r.modernCities.map((_e,Je)=>{let O=1/0,Pe=-1/0,Ie=1/0,tt=-1/0;for(const Yt of _e.nodes)O=Math.min(O,Yt.point.x_km),Pe=Math.max(Pe,Yt.point.x_km),Ie=Math.min(Ie,Yt.point.y_km),tt=Math.max(tt,Yt.point.y_km);const Nt=(O+Pe)/2,Ft=(Ie+tt)/2;return{label:Je===0?"中心城（县城）":Je<4?`镇区 ${Je}`:`村庄 ${Je-3}`,xKm:Nt,yKm:Ft,spanKm:Je===0?6:Je<4?3.5:1.6}});return[{label:"全域视角",xKm:o.worldSizeKm/2,yKm:o.worldSizeKm/2,spanKm:o.worldSizeKm*.55},...J]},[r,o.worldSizeKm]),[N,P]=mt.useState(null),[k,ee]=mt.useState(!1);mt.useEffect(()=>{if(_===null){P(null);return}let J=!1;return ee(!0),xp(_).catch(()=>e2(_)).then(_e=>{J||P(_e)}).catch(_e=>{J||A(_e.message)}).finally(()=>{J||ee(!1)}),()=>{J=!0}},[_]);const Me=(J,_e)=>{U({xKm:J.xKm,yKm:J.yKm,spanKm:J.spanKm,nonce:Date.now()}),_e>=1&&B("3d")},S=(J,_e)=>{l(Je=>({...Je,[J]:_e}))},Q=J=>{const _e=qs[J];l(Je=>({...Je,preset:J,rainfall:_e.rainfall,evaporation:_e.evaporation,windSpeed:_e.windSpeed,windDirection:_e.windDirection}))},ve=J=>S("landform",J),V=()=>S("seed",Math.floor(Math.random()*2147483647)),pe=async()=>{f(!0),A(null),d(.02),g(aa?`初始化计算核心 · ${ru}`:"初始化原生计算核心");try{const J=await Ig(o);c(J),B("3d"),I(null),d(1),g(`完成 · ${(J.elapsedMs/1e3).toFixed(2)} 秒${aa?` · ${ru}`:""}`),X(null)}catch(J){A(String(J)),g("生成失败")}finally{f(!1)}};mt.useEffect(()=>{ge===1&&(G(2),Zl("generating",{config:o}),pe().then(()=>Zl("generated",{})))},[ge]),mt.useEffect(()=>{if(!se||!r||F.current)return;F.current=!0,se.view==="satellite"?B("satellite"):se.view==="analysis"&&(H(se.layer??"discharge"),B("analysis"));const J=se.focus;if(J){const _e=$[J.option];_e&&U({xKm:_e.xKm,yKm:_e.yKm,spanKm:J.spanKm??_e.spanKm,nonce:Date.now(),pitchDeg:J.pitchDeg,azimuthDeg:J.azimuthDeg})}(se.view==="satellite"||se.view==="analysis")&&window.setTimeout(()=>Zl("ready",{view:se.view}),4e3)},[se,r]),mt.useEffect(()=>{T&&se&&Zl("error",{error:T})},[T,se]);const ue=async()=>{const J=await yh({title:"导出卫星正射影像",defaultPath:`wind-water-${o.preset}-${o.seed}-${x}.png`,filters:[{name:"PNG image",extensions:["png"]}]});if(J){f(!0),A(null),g(`渲染 ${x} × ${x} 影像`),d(.02);try{await t2(o,J,x),d(1),g(`已导出 · ${J}`)}catch(_e){A(String(_e)),g("导出失败")}finally{f(!1)}}},Ce=async()=>{const J=await yh({title:"保存地貌项目",defaultPath:`wind-water-${o.seed}.fwl`,filters:[{name:"Wind & Water project",extensions:["fwl"]}]});if(J)try{await a2({schemaVersion:1,name:`Terrain ${o.seed}`,config:o},J),g(`项目已保存 · ${J}`),A(null)}catch(_e){A(String(_e))}},Ne=async()=>{const J=await pp({title:"打开地貌项目",multiple:!1,directory:!1,filters:[{name:"Wind & Water project",extensions:["fwl"]}]});if(J)try{const _e=await n2(J);l(_e.config),c(null),g(`已打开 · ${_e.name}`),A(null)}catch(_e){A(String(_e))}},oe=async()=>{Z.current&&(document.fullscreenElement?await document.exitFullscreen():await Z.current.requestFullscreen())};return re.jsxs("main",{className:"app-shell",children:[re.jsxs("header",{className:"topbar",children:[re.jsxs("div",{className:"brand",children:[re.jsx("span",{className:"brand-mark",children:"W/W"}),re.jsxs("div",{children:[re.jsx("h1",{children:"风与水"}),re.jsx("p",{children:"无限地貌实验室 · Desktop Alpha"})]})]}),re.jsxs("div",{className:"top-actions",children:[re.jsxs("span",{className:"engine-badge",children:[re.jsx("i",{})," ",$l()?"RUST NATIVE":"RUST · WASM"]}),re.jsx("button",{className:"ghost",onClick:Ne,disabled:i,children:"打开"}),re.jsx("button",{className:"ghost",onClick:Ce,disabled:i,children:"保存"}),re.jsx("button",{className:"ghost",onClick:V,disabled:i,children:"随机种子"}),re.jsx("button",{className:"sheet-toggle",onClick:()=>X(le==="left"?null:"left"),children:"参数"}),re.jsx("button",{className:"sheet-toggle",onClick:()=>X(le==="right"?null:"right"),children:"光照"}),re.jsx("button",{className:"primary",onClick:pe,disabled:i,children:i?"计算中…":"生成地貌"})]})]}),re.jsxs("section",{className:"workspace",children:[re.jsxs("aside",{className:`sidebar left-panel${le==="left"?" sheet-open":""}`,children:[re.jsxs("section",{children:[re.jsxs("div",{className:"section-title",children:[re.jsx("span",{children:"地貌骨架"}),re.jsx("em",{children:"01"})]}),re.jsx("div",{className:"landform-grid",children:Object.keys(ph).map(J=>re.jsxs("button",{className:`landform-card ${o.landform===J?"active":""}`,onClick:()=>ve(J),disabled:i,children:[re.jsx("strong",{children:ph[J].name}),re.jsx("small",{children:ph[J].description})]},J))})]}),re.jsxs("section",{children:[re.jsxs("div",{className:"section-title",children:[re.jsx("span",{children:"气候环境"}),re.jsx("em",{children:"02"})]}),re.jsx("div",{className:"preset-grid",children:Object.keys(qs).map(J=>re.jsxs("button",{className:`preset-card ${o.preset===J?"active":""}`,onClick:()=>Q(J),disabled:i,children:[re.jsx("strong",{children:qs[J].name}),re.jsx("small",{children:qs[J].description})]},J))})]}),re.jsxs("section",{children:[re.jsxs("div",{className:"section-title",children:[re.jsx("span",{children:"计算域"}),re.jsx("em",{children:"03"})]}),re.jsxs("label",{className:"select-control",children:[re.jsx("span",{children:"模拟网格"}),re.jsxs("select",{value:o.gridSize,onChange:J=>S("gridSize",Number(J.target.value)),disabled:i,children:[aa?re.jsx("option",{value:256,children:"256² · 手机"}):null,re.jsx("option",{value:512,children:"512² · 快速"}),aa?null:re.jsxs(re.Fragment,{children:[re.jsx("option",{value:1024,children:"1024² · 精细"}),re.jsx("option",{value:2048,children:"2048² · 专业"})]})]})]}),re.jsx(el,{label:"区域尺度",value:o.worldSizeKm,min:20,max:160,step:10,unit:" km",onChange:J=>S("worldSizeKm",J)}),re.jsxs("div",{className:"domain-note",children:[re.jsx("span",{children:"物理分辨率"}),re.jsxs("b",{children:[ie," m/cell"]})]}),re.jsxs("label",{className:"seed-input",children:[re.jsx("span",{children:"随机种子"}),re.jsx("input",{type:"number",value:o.seed,onChange:J=>S("seed",Number(J.target.value)),disabled:i})]})]}),re.jsxs("section",{children:[re.jsxs("div",{className:"section-title",children:[re.jsx("span",{children:"水文与气候"}),re.jsx("em",{children:"04"})]}),re.jsx(el,{label:"年降水",value:o.rainfall,min:100,max:2e3,step:25,unit:" mm",onChange:J=>S("rainfall",J)}),re.jsx(el,{label:"蒸散量",value:o.evaporation,min:0,max:1600,step:25,unit:" mm",onChange:J=>S("evaporation",J)}),re.jsx(el,{label:"风速",value:o.windSpeed,min:0,max:30,step:.5,unit:" m/s",onChange:J=>S("windSpeed",J)}),re.jsx(el,{label:"风向",value:o.windDirection,min:0,max:359,unit:"°",onChange:J=>S("windDirection",J)})]})]}),re.jsxs("section",{className:"viewport",ref:Z,children:[r?R==="analysis"?re.jsx("img",{className:"terrain-preview",src:r.analysisPreviews[C],alt:`${lu[C]}分析图层`}):re.jsx(A5,{result:r,config:o,cameraMode:R,cityFocus:K,cityScene:N,shot:se&&(!se.focus||K)?se:null,onShotReady:J=>{Zl("ready",J),se?.exitAfterReadyMs!==void 0&&window.setTimeout(()=>Zl("exit"),se.exitAfterReadyMs)},onNeedCity:J=>{I(J===null?null:_e=>_e===J?_e:J)}}):re.jsxs("div",{className:"empty-state",children:[re.jsx("div",{className:"contour-art"}),re.jsx("span",{children:"80 × 80 KM SYNTHETIC EARTH"}),re.jsx("h2",{children:"创造一片不存在，却足够真实的土地"}),re.jsx("p",{children:"原生 Rust 地貌演化 · 河网推演 · 多尺度卫星辐射合成"}),re.jsx("button",{className:"primary large",onClick:pe,children:"生成第一片地貌"})]}),r&&re.jsxs("div",{className:"view-switcher",children:[re.jsxs(re.Fragment,{children:[re.jsx("button",{className:R==="3d"?"active":"",onClick:()=>B("3d"),children:"3D 地形"}),re.jsx("button",{className:R==="satellite"?"active":"",onClick:()=>B("satellite"),children:"动态卫星"}),re.jsxs("select",{className:R==="analysis"?"active":"",value:C,onChange:J=>{H(J.target.value),B("analysis")},children:[re.jsx("option",{disabled:!0,children:"分析图层"}),Object.keys(lu).map(J=>re.jsx("option",{value:J,children:lu[J]},J))]}),re.jsxs("select",{value:"",className:"city-jump",onChange:J=>{const _e=Number(J.target.value),Je=$[_e];Je&&Me(Je,_e)},children:[re.jsx("option",{disabled:!0,value:"",children:"跳转城市"}),$.map((J,_e)=>re.jsx("option",{value:_e,children:J.label},J.label))]})]}),re.jsx("button",{onClick:oe,children:"全屏"})]}),aa&&re.jsxs("div",{className:"light-notice",children:[ru,Fs?" · 安全模式":""]}),re.jsxs("div",{className:"viewport-meta",children:[re.jsx("span",{children:R==="3d"?"3D TERRAIN · VERTICAL 1:1 · LIVE WEATHER":R==="analysis"?`SEMANTIC LAYER · ${lu[C]}`:"LIVE ORTHOGRAPHIC · SAME WORLD · SAME TIME"}),re.jsx("span",{children:k?"正在加载城市细节…":N&&_!==null?`${$[_+1]?.label??"城市"} · 米制细节已载入`:`${o.worldSizeKm} KM / ${o.gridSize} PX`})]}),i&&re.jsxs("div",{className:"progress-overlay",children:[re.jsxs("div",{children:[re.jsx("span",{children:h}),re.jsxs("b",{children:[Math.round(m*100),"%"]})]}),re.jsx("progress",{value:m,max:1})]})]}),re.jsxs("aside",{className:`sidebar right-panel${le==="right"?" sheet-open":""}`,children:[re.jsxs("section",{children:[re.jsxs("div",{className:"section-title",children:[re.jsx("span",{children:"光照与大气"}),re.jsx("em",{children:"04"})]}),re.jsx(el,{label:"太阳方位",value:o.sunAzimuth,min:0,max:359,unit:"°",onChange:J=>S("sunAzimuth",J)}),re.jsx(el,{label:"太阳高度",value:o.sunElevation,min:5,max:80,unit:"°",onChange:J=>S("sunElevation",J)}),re.jsx(el,{label:"大气霾度",value:o.haze,min:0,max:10,step:.5,onChange:J=>S("haze",J)}),re.jsx(el,{label:"云层覆盖",value:o.cloudCoverage,min:0,max:100,unit:"%",onChange:J=>S("cloudCoverage",J)}),re.jsx(el,{label:"云层速度",value:o.cloudSpeed,min:0,max:90,onChange:J=>S("cloudSpeed",J)})]}),re.jsxs("section",{children:[re.jsxs("div",{className:"section-title",children:[re.jsx("span",{children:"区域诊断"}),re.jsx("em",{children:"05"})]}),r?re.jsxs("div",{className:"stats-grid",children:[re.jsxs("div",{children:[re.jsx("span",{children:"高程范围"}),re.jsxs("b",{children:[r.stats.minElevation.toFixed(0),"–",r.stats.maxElevation.toFixed(0)," m"]})]}),re.jsxs("div",{children:[re.jsx("span",{children:"平均高程"}),re.jsxs("b",{children:[r.stats.meanElevation.toFixed(0)," m"]})]}),re.jsxs("div",{children:[re.jsx("span",{children:"平均坡度"}),re.jsxs("b",{children:[r.stats.meanSlope.toFixed(1),"°"]})]}),re.jsxs("div",{children:[re.jsx("span",{children:"森林覆盖"}),re.jsxs("b",{children:[(r.stats.forestCoverage*100).toFixed(1),"%"]})]}),re.jsxs("div",{children:[re.jsx("span",{children:"水体覆盖"}),re.jsxs("b",{children:[(r.stats.waterCoverage*100).toFixed(1),"%"]})]}),re.jsxs("div",{children:[re.jsx("span",{children:"积雪覆盖"}),re.jsxs("b",{children:[(r.stats.snowCoverage*100).toFixed(1),"%"]})]}),re.jsxs("div",{children:[re.jsx("span",{children:"候选聚落"}),re.jsx("b",{children:r.infrastructureSummary.settlements})]}),re.jsxs("div",{children:[re.jsx("span",{children:"道路连接"}),re.jsx("b",{children:r.infrastructureSummary.roads})]}),re.jsxs("div",{children:[re.jsx("span",{children:"桥梁 / 隧道"}),re.jsxs("b",{children:[r.infrastructureSummary.bridges," / ",r.infrastructureSummary.tunnels]})]})]}):re.jsx("p",{className:"muted-copy",children:"生成后显示地形、水文与地表覆盖统计。"})]}),re.jsxs("section",{className:"export-section",children:[re.jsxs("div",{className:"section-title",children:[re.jsx("span",{children:"卫星影像导出"}),re.jsx("em",{children:"06"})]}),re.jsxs("label",{className:"select-control",children:[re.jsx("span",{children:"输出尺寸"}),re.jsxs("select",{value:x,onChange:J=>M(Number(J.target.value)),disabled:i,children:[re.jsx("option",{value:2048,children:"2048 × 2048"}),re.jsx("option",{value:4096,children:"4096 × 4096"}),re.jsx("option",{value:8192,children:"8192 × 8192"})]})]}),re.jsxs("button",{className:"export-button",onClick:ue,disabled:i||!r,children:[re.jsx("span",{children:"导出 PNG"}),re.jsx("small",{children:"正射自然色 · 无水印"})]}),re.jsx("p",{className:"fine-print",children:"采用原生多线程重采样与亚像素材质细节，高分辨率导出不会阻塞界面。"})]})]})]}),re.jsxs("footer",{className:"statusbar",children:[re.jsxs("span",{className:T?"error":"",children:[T??h,Fs?" · 上次 3D 视图被系统终止，已启用安全模式（?safe=0 恢复）":""]}),re.jsx("span",{children:"CORE 0.1 · LOCAL ONLY · NO TELEMETRY"})]})]})}Yg.createRoot(document.getElementById("root")).render(re.jsx(mt.StrictMode,{children:re.jsx(U5,{})}));export{Oh as i,E5 as t};
