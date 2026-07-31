import{p as h,f as $,n as _,s as o,a as u,b as y,c as B,d as g,t as b}from"./iframe-CMUQcgBj.js";import{c as S,i as k,d as L}from"./create-runtime-stories-CxX1yIVv.js";import{B as c}from"./Button-Dat8UTEz.js";import{C as f}from"./Card-6yPI_lkc.js";import{H as w}from"./hexagon-filled-qMh4ESLA.js";import"./preload-helper-Dp1pzeXC.js";import"./slot-B1isFuVn.js";import"./attributes-CXXSZYqR.js";import"./misc-BMKWk-bU.js";import"./Icon-mzNJGl0N.js";const i=(n,s=_)=>{f(n,{height:"100%",children:(t,l)=>{c(t,g(s,{children:(e,m)=>{var d=b("Click me");u(e,d)},$$slots:{default:!0}}))},$$slots:{default:!0}})},P=(n,s=_)=>{f(n,{height:"100%",children:(t,l)=>{c(t,g(s,{children:(e,m)=>{w(e,{size:150})},$$slots:{default:!0}}))},$$slots:{default:!0}})},I={title:"UI/Button",tags:["autodocs"],component:c},{Story:a}=L();var x=B("<!> <!> <!> <!> <!> <!>",1);function v(n,s){h(s,!1),k();var t=x(),l=$(t);a(l,{name:"Primary",args:{variant:"primary",fontSize:"medium",ariaLabel:""},get template(){return i},parameters:{__svelteCsf:{rawCode:`<Card height="100%">
    <Button {...args}>
        Click me
    </Button>
</Card>`}}});var e=o(l,2);a(e,{name:"Secondary",args:{variant:"secondary",fontSize:"medium",ariaLabel:""},get template(){return i},parameters:{__svelteCsf:{rawCode:`<Card height="100%">
    <Button {...args}>
        Click me
    </Button>
</Card>`}}});var m=o(e,2);a(m,{name:"Transparent",args:{variant:"transparent",fontSize:"medium",ariaLabel:""},get template(){return i},parameters:{__svelteCsf:{rawCode:`<Card height="100%">
    <Button {...args}>
        Click me
    </Button>
</Card>`}}});var d=o(m,2);a(d,{name:"Disabled",args:{variant:"primary",fontSize:"medium",disabled:!0,ariaLabel:""},get template(){return i},parameters:{__svelteCsf:{rawCode:`<Card height="100%">
    <Button {...args}>
        Click me
    </Button>
</Card>`}}});var p=o(d,2);a(p,{name:"Anchor Link",args:{variant:"primary",fontSize:"medium",href:"https:www.example.com",ariaLabel:""},get template(){return i},parameters:{__svelteCsf:{rawCode:`<Card height="100%">
    <Button {...args}>
        Click me
    </Button>
</Card>`}}});var C=o(p,2);a(C,{name:"Icon Only",args:{variant:"transparent",iconOnly:!0,ariaLabel:"Test Icon"},get template(){return P},parameters:{__svelteCsf:{rawCode:`<Card height="100%">
    <Button {...args}>
        <IconHexagonFilled size={150}/> 
    </Button>
</Card>`}}}),u(n,t),y()}v.__docgen={data:[],name:"Button.stories.svelte"};const r=S(v,I),U=["Primary","Secondary","Transparent","Disabled","AnchorLink","IconOnly"],j={...r.Primary,tags:["svelte-csf-v5"]},q={...r.Secondary,tags:["svelte-csf-v5"]},G={...r.Transparent,tags:["svelte-csf-v5"]},J={...r.Disabled,tags:["svelte-csf-v5"]},K={...r.AnchorLink,tags:["svelte-csf-v5"]},N={...r.IconOnly,tags:["svelte-csf-v5"]};export{K as AnchorLink,J as Disabled,N as IconOnly,j as Primary,q as Secondary,G as Transparent,U as __namedExportsOrder,I as default};
