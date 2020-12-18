!function(t){function e(e){for(var s,r,o=e[0],l=e[1],u=e[2],p=0,v=[];p<o.length;p++)r=o[p],Object.prototype.hasOwnProperty.call(i,r)&&i[r]&&v.push(i[r][0]),i[r]=0;for(s in l)Object.prototype.hasOwnProperty.call(l,s)&&(t[s]=l[s]);for(d&&d(e);v.length;)v.shift()();return n.push.apply(n,u||[]),a()}function a(){for(var t,e=0;e<n.length;e++){for(var a=n[e],s=!0,o=1;o<a.length;o++){var l=a[o];0!==i[l]&&(s=!1)}s&&(n.splice(e--,1),t=r(r.s=a[0]))}return t}var s={},i={0:0},n=[];function r(e){if(s[e])return s[e].exports;var a=s[e]={i:e,l:!1,exports:{}};return t[e].call(a.exports,a,a.exports,r),a.l=!0,a.exports}r.m=t,r.c=s,r.d=function(t,e,a){r.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:a})},r.r=function(t){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},r.t=function(t,e){if(1&e&&(t=r(t)),8&e)return t;if(4&e&&"object"==typeof t&&t&&t.__esModule)return t;var a=Object.create(null);if(r.r(a),Object.defineProperty(a,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var s in t)r.d(a,s,function(e){return t[e]}.bind(null,s));return a},r.n=function(t){var e=t&&t.__esModule?function(){return t.default}:function(){return t};return r.d(e,"a",e),e},r.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},r.p="";var o=window.webpackJsonp=window.webpackJsonp||[],l=o.push.bind(o);o.push=e,o=o.slice();for(var u=0;u<o.length;u++)e(o[u]);var d=l;n.push([435,1]),a()}({435:function(t,e,a){"use strict";a.r(e);a(202),a(431);var s=a(2),i=a(199),n=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("v-app",[a("v-navigation-drawer",{attrs:{app:""},model:{value:t.menu_visible,callback:function(e){t.menu_visible=e},expression:"menu_visible"}},[a("v-list",{attrs:{nav:"",dense:""}},t._l(t.days,(function(e,s){var i=e[0],n=e[1],r=e[2];return a("v-list-item",{key:"day"+s,attrs:{"two-line":"",href:"day"+i+".html"}},[a("v-list-item-action",{staticClass:"mr-2"},[i===t.current_day?a("v-icon",{attrs:{title:"Selected day"}},[t._v(t._s(t.mdiArrowRightBold))]):t._e()],1),t._v(" "),a("v-list-item-content",[a("v-list-item-title",{staticClass:"aoc-em"},[t._v("Day "+t._s(i))]),t._v(" "),a("v-list-item-subtitle",[t._v(t._s(n))])],1),t._v(" "),r?a("v-list-item-action",[a("v-icon",{attrs:{title:"Visualization"}},[t._v(t._s(t.mdiEye))])],1):t._e()],1)})),1)],1),t._v(" "),a("v-app-bar",{attrs:{app:""}},[a("v-app-bar-nav-icon",{on:{click:function(e){e.stopPropagation(),t.menu_visible=!t.menu_visible}}}),t._v(" "),a("v-toolbar-title",[t._v("Day "+t._s(t.current_day)+": "),a("a",{attrs:{href:"https://adventofcode.com/2020/day/"+t.current_day}},[t._v(t._s(t.days[t.current_day-1][1]))])])],1),t._v(" "),a("v-main",[a("v-skeleton-loader",{directives:[{name:"show",rawName:"v-show",value:t.loading,expression:"loading"}],attrs:{type:"article@3"}}),t._v(" "),a("v-container",{directives:[{name:"show",rawName:"v-show",value:!t.loading,expression:"!loading"}],attrs:{fluid:""}},[a("v-textarea",{attrs:{outlined:"",label:"Input"},model:{value:t.input,callback:function(e){t.input=e},expression:"input"}}),t._v(" "),a("v-btn",{staticClass:"ma-3",attrs:{outlined:"",disabled:t.in_progress,id:"run"},on:{click:function(e){return t._run()}}},[t._v("Run")]),t._v(" "),a("div",{staticClass:"d-flex"},[a("v-textarea",{attrs:{outlined:"",label:"Part 1"},model:{value:t.output1,callback:function(e){t.output1=e},expression:"output1"}}),t._v(" "),a("v-textarea",{attrs:{outlined:"",label:"Part 2"},model:{value:t.output2,callback:function(e){t.output2=e},expression:"output2"}})],1),t._v(" "),a("v-card",{directives:[{name:"show",rawName:"v-show",value:t.visual_supported,expression:"visual_supported"}]},[a("v-card-title",[t._v("Visualization")]),t._v(" "),a("v-card-text",[a("v-checkbox",{attrs:{label:"Enabled"},model:{value:t.visual_enabled,callback:function(e){t.visual_enabled=e},expression:"visual_enabled"}}),t._v(" "),a("v-slider",{attrs:{min:"0",max:"100",label:"Speed"},model:{value:t.visual_speed,callback:function(e){t.visual_speed=e},expression:"visual_speed"}}),t._v(" "),a("canvas",{ref:"canvas",attrs:{id:"canvas",oncontextmenu:"event.preventDefault()"}}),a("br"),t._v(" "),a("v-btn",{staticClass:"ma-3",attrs:{outlined:"",id:"stop",disabled:!t.in_progress},on:{click:function(e){return t._stop()}}},[t._v("Stop")]),t._v(" "),a("div",[t._v("\n\t\t\t\t\t\tIf you see visualization glitches, try "),a("a",{attrs:{href:"?renderer=software"}},[t._v("this")]),t._v(" and contact me.\n\t\t\t\t\t")])],1)],1),t._v("\n\t\t\tSource code is "),a("a",{attrs:{href:"https://github.com/DarthGandalf/advent-of-code/tree/master/2020"}},[t._v("over here")]),t._v(".\n\t\t")],1)],1)],1)};n._withStripped=!0;var r=a(156),o={props:{days:Array,current_day:Number},data:()=>({menu_visible:null,loading:!0,input:"",output1:"",output2:"",visual_supported:!1,visual_enabled:!0,visual_speed:90,in_progress:!1,should_stop:!1,visual_cb:null,visual_ptr:null,mdiEye:r.b,mdiArrowRightBold:r.a}),mounted(){new ResizeObserver(t=>{for(let e of t){const{width:t,height:a}=e.contentRect;this.visual_cb&&this.visual_cb(this.visual_ptr,t,a)}}).observe(this.$refs.canvas)},methods:{_run(){this.visual_supported&&this.visual_enabled&&this.$vuetify.goTo("#canvas")},_stop(){this.should_stop=!0},getInput(){return this.input},setInput(t){this.input=t},setOutput1(t){this.output1=t},setOutput2(t){this.output2=t},supportVisual(t,e){this.visual_supported=!0,this.visual_cb=t,this.visual_ptr=e},visualEnabled(){return this.visual_enabled&&!this.should_stop},getVisualSpeed(){return this.visual_speed},setVisualSpeed(t){this.visual_speed=t},finishVisual(){this.$vuetify.goTo(0)},setInProgress(t){this.in_progress=t,t&&(this.should_stop=!1)},setLoaded(){this.loading=!1}}},l=a(196),u=a(197),d=a.n(u),p=a(455),v=a(453),c=a(449),h=a(200),_=a(154),f=a(91),b=a(452),m=a(454),y=a(153),g=a(155),w=a(92),V=a(152),x=a(48),S=a(456),O=a(450),k=a(457),C=a(458),I=a(451),P=a(198),$=Object(l.a)(o,n,[],!1,null,null,null);d()($,{VApp:p.a,VAppBar:v.a,VAppBarNavIcon:c.a,VBtn:h.a,VCard:_.a,VCardText:f.a,VCardTitle:f.b,VCheckbox:b.a,VContainer:m.a,VIcon:y.a,VList:g.a,VListItem:w.a,VListItemAction:V.a,VListItemContent:x.a,VListItemSubtitle:x.b,VListItemTitle:x.c,VMain:S.a,VNavigationDrawer:O.a,VSkeletonLoader:k.a,VSlider:C.a,VTextarea:I.a,VToolbarTitle:P.a}),$.options.__file="web/App.vue";var T=$.exports;const R=[[1,"Report Repair",0],[2,"Password Philosophy",0],[3,"Toboggan Trajectory",1],[4,"Passport Processing",0],[5,"Binary Boarding",0],[6,"Custom Customs",0],[7,"Handy Haversacks",0],[8,"Handheld Halting",0],[9,"Encoding Error",0],[10,"Adapter Array",0],[11,"Seating System",0],[12,"Rain Risk",1],[13,"Shuttle Search",0],[14,"Docking Data",0],[15,"Rambunctious Recitation",0],[16,"Ticket Translation",0],[17,"Conway Cubes",0],[18,"Operation Order",0]];s.a.use(i.a),window.aocvue=new s.a({vuetify:new i.a({theme:{dark:!0,themes:{dark:{primary:"#ffff66"}}},icons:{iconfont:"mdiSvg"}}),el:"#main",render:t=>t(T,{props:{days:R,current_day:window.AOCCurrentDay}}),methods:{getInput(){return this.$children[0].getInput()},setInput(t){this.$children[0].setInput(t)},setOutput1(t){this.$children[0].setOutput1(t)},setOutput2(t){this.$children[0].setOutput2(t)},supportVisual(t,e){this.$children[0].supportVisual(t,e)},visualEnabled(){return this.$children[0].visualEnabled()},getVisualSpeed(){return this.$children[0].getVisualSpeed()},setVisualSpeed(t){this.$children[0].setVisualSpeed(t)},finishVisual(){this.$children[0].finishVisual()},setInProgress(t){this.$children[0].setInProgress(t)},setLoaded(){this.$children[0].setLoaded()}}}),window.AOCRender=new URL(document.location).searchParams.get("renderer")||""}});
//# sourceMappingURL=main.js.map