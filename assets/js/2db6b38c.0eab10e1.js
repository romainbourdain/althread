"use strict";(self.webpackChunkalthread=self.webpackChunkalthread||[]).push([[471],{6137:(e,s,n)=>{n.r(s),n.d(s,{assets:()=>o,contentTitle:()=>t,default:()=>u,frontMatter:()=>i,metadata:()=>c,toc:()=>l});var r=n(6070),a=n(5710);const i={sidebar_position:3},t="Variables partag\xe9es",c={id:"guide/process/global",title:"Variables partag\xe9es",description:"Jusqu'\xe0 pr\xe9sent, les variables d\xe9clar\xe9es dans un processus sont locales \xe0 ce processus. Cela signifie qu'un processus ne peut pas acc\xe9der aux variables des autres processus :",source:"@site/docs/guide/process/global.md",sourceDirName:"guide/process",slug:"/guide/process/global",permalink:"/althread/docs/guide/process/global",draft:!1,unlisted:!1,editUrl:"https://github.com/romainbourdain/althread/tree/main/doc/docs/guide/process/global.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"guideSidebar",previous:{title:"Arguments (non impl\xe9ment\xe9)",permalink:"/althread/docs/guide/process/arguments"},next:{title:"Lire dans un canal (non impl\xe9ment\xe9)",permalink:"/althread/docs/guide/channels/read"}},o={},l=[{value:"D\xe9claration de variables partag\xe9es",id:"d\xe9claration-de-variables-partag\xe9es",level:2},{value:"Ex\xe9cution de processus avec des variables partag\xe9es",id:"ex\xe9cution-de-processus-avec-des-variables-partag\xe9es",level:2}];function d(e){const s={admonition:"admonition",code:"code",h1:"h1",h2:"h2",header:"header",p:"p",pre:"pre",...(0,a.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(s.header,{children:(0,r.jsx)(s.h1,{id:"variables-partag\xe9es",children:"Variables partag\xe9es"})}),"\n",(0,r.jsx)(s.p,{children:"Jusqu'\xe0 pr\xe9sent, les variables d\xe9clar\xe9es dans un processus sont locales \xe0 ce processus. Cela signifie qu'un processus ne peut pas acc\xe9der aux variables des autres processus :"}),"\n",(0,r.jsx)(s.pre,{children:(0,r.jsx)(s.code,{children:"process Process1() {\n    print(x); // x n'existe pas dans ce processus\n}\n\nmain {\n    let x = 0;\n    run Process1();\n}\n"})}),"\n",(0,r.jsx)(s.admonition,{type:"danger",children:(0,r.jsxs)(s.p,{children:["Le code ci-dessus renverra une erreur : le processus ",(0,r.jsx)(s.code,{children:"Process1"})," ne peut pas acc\xe9der \xe0 la variable ",(0,r.jsx)(s.code,{children:"x"})," d\xe9clar\xe9e dans le processus principal."]})}),"\n",(0,r.jsx)(s.h2,{id:"d\xe9claration-de-variables-partag\xe9es",children:"D\xe9claration de variables partag\xe9es"}),"\n",(0,r.jsx)(s.p,{children:"Pour permettre \xe0 plusieurs processus d'acc\xe9der \xe0 une m\xeame variable, vous devez la d\xe9clarer comme une variable partag\xe9e. Une variable partag\xe9e est une variable qui peut \xeatre lue et modifi\xe9e par plusieurs processus. Voici comment d\xe9clarer une variable partag\xe9e :"}),"\n",(0,r.jsx)(s.pre,{children:(0,r.jsx)(s.code,{children:"shared {\n    let x: int;\n    let y = false;\n    const a = 42;\n}\n"})}),"\n",(0,r.jsx)(s.admonition,{type:"tip",children:(0,r.jsxs)(s.p,{children:["Les d\xe9claration du block ",(0,r.jsx)(s.code,{children:"shared"})," fonctionnent comme les d\xe9clarations classiques : elles peuvent \xeatre constantes ou mutables, avoir n'importe quel type et l'on peut leur assigner une valeur"]})}),"\n",(0,r.jsx)(s.admonition,{type:"warning",children:(0,r.jsxs)(s.p,{children:["Il n'est possible de faire que des d\xe9clarations dans le block ",(0,r.jsx)(s.code,{children:"shared"}),"."]})}),"\n",(0,r.jsx)(s.h2,{id:"ex\xe9cution-de-processus-avec-des-variables-partag\xe9es",children:"Ex\xe9cution de processus avec des variables partag\xe9es"}),"\n",(0,r.jsxs)(s.p,{children:["Lors de l'ex\xe9cution, le block ",(0,r.jsx)(s.code,{children:"shared"})," est ex\xe9cut\xe9 d'une traite avant les processus. Les variables partag\xe9es sont ainsi accessibles et modifiables par tous les processus."]}),"\n",(0,r.jsx)(s.pre,{children:(0,r.jsx)(s.code,{children:"shared {\n    let x : int;\n}\n\nprocess Process1() {\n    x++;\n    wait(x == 2);\n}\n\nmain {\n    run Process1();\n    run Process1();\n}\n"})}),"\n",(0,r.jsx)(s.admonition,{type:"note",children:(0,r.jsxs)(s.p,{children:["Dans cet exemple, les deux processus ",(0,r.jsx)(s.code,{children:"Process1"})," incr\xe9mentent la variable ",(0,r.jsx)(s.code,{children:"x"})," de 1. Le premier processus attend ensuite que ",(0,r.jsx)(s.code,{children:"x"})," soit \xe9gal \xe0 2 avant de continuer."]})})]})}function u(e={}){const{wrapper:s}={...(0,a.R)(),...e.components};return s?(0,r.jsx)(s,{...e,children:(0,r.jsx)(d,{...e})}):d(e)}},5710:(e,s,n)=>{n.d(s,{R:()=>t,x:()=>c});var r=n(758);const a={},i=r.createContext(a);function t(e){const s=r.useContext(i);return r.useMemo((function(){return"function"==typeof e?e(s):{...s,...e}}),[s,e])}function c(e){let s;return s=e.disableParentContext?"function"==typeof e.components?e.components(a):e.components||a:t(e.components),r.createElement(i.Provider,{value:s},e.children)}}}]);