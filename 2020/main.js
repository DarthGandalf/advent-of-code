!function(t){function e(e){for(var n,s,o=e[0],u=e[1],l=e[2],p=0,v=[];p<o.length;p++)s=o[p],Object.prototype.hasOwnProperty.call(i,s)&&i[s]&&v.push(i[s][0]),i[s]=0;for(n in u)Object.prototype.hasOwnProperty.call(u,n)&&(t[n]=u[n]);for(d&&d(e);v.length;)v.shift()();return r.push.apply(r,l||[]),a()}function a(){for(var t,e=0;e<r.length;e++){for(var a=r[e],n=!0,o=1;o<a.length;o++){var u=a[o];0!==i[u]&&(n=!1)}n&&(r.splice(e--,1),t=s(s.s=a[0]))}return t}var n={},i={0:0},r=[];function s(e){if(n[e])return n[e].exports;var a=n[e]={i:e,l:!1,exports:{}};return t[e].call(a.exports,a,a.exports,s),a.l=!0,a.exports}s.m=t,s.c=n,s.d=function(t,e,a){s.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:a})},s.r=function(t){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},s.t=function(t,e){if(1&e&&(t=s(t)),8&e)return t;if(4&e&&"object"==typeof t&&t&&t.__esModule)return t;var a=Object.create(null);if(s.r(a),Object.defineProperty(a,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var n in t)s.d(a,n,function(e){return t[e]}.bind(null,n));return a},s.n=function(t){var e=t&&t.__esModule?function(){return t.default}:function(){return t};return s.d(e,"a",e),e},s.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},s.p="";var o=window.webpackJsonp=window.webpackJsonp||[],u=o.push.bind(o);o.push=e,o=o.slice();for(var l=0;l<o.length;l++)e(o[l]);var d=u;r.push([435,1]),a()}({435:function(t,e,a){"use strict";a.r(e);a(202),a(431);var n=a(2),i=a(199),r=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("v-app",[a("v-navigation-drawer",{attrs:{app:""},model:{value:t.menu_visible,callback:function(e){t.menu_visible=e},expression:"menu_visible"}},[a("v-list",{attrs:{nav:"",dense:""}},t._l(t.days,(function(e,n){var i=e[0],r=e[1],s=e[2];return a("v-list-item",{key:"day"+n,attrs:{"two-line":"",href:"day"+i+".html"}},[a("v-list-item-action",{staticClass:"mr-2"},[i===t.current_day?a("v-icon",{attrs:{title:"Selected day"}},[t._v(t._s(t.mdiArrowRightBold))]):t._e()],1),t._v(" "),a("v-list-item-content",[a("v-list-item-title",{staticClass:"aoc-em"},[t._v("Day "+t._s(i))]),t._v(" "),a("v-list-item-subtitle",[t._v(t._s(r))])],1),t._v(" "),s?a("v-list-item-action",[a("v-icon",{attrs:{title:"Visualization"}},[t._v(t._s(t.mdiEye))])],1):t._e()],1)})),1)],1),t._v(" "),a("v-app-bar",{attrs:{app:""}},[a("v-app-bar-nav-icon",{on:{click:function(e){e.stopPropagation(),t.menu_visible=!t.menu_visible}}}),t._v(" "),a("v-toolbar-title",[t._v("Day "+t._s(t.current_day)+": "),a("a",{attrs:{href:"https://adventofcode.com/2020/day/"+t.current_day}},[t._v(t._s(t.days[t.current_day-1][1]))])])],1),t._v(" "),a("v-main",[a("v-skeleton-loader",{directives:[{name:"show",rawName:"v-show",value:t.loading,expression:"loading"}],attrs:{type:"article@3"}}),t._v(" "),a("v-container",{directives:[{name:"show",rawName:"v-show",value:!t.loading,expression:"!loading"}],attrs:{fluid:""}},[a("v-textarea",{attrs:{outlined:"",label:"Input"},model:{value:t.input,callback:function(e){t.input=e},expression:"input"}}),t._v(" "),a("v-btn",{staticClass:"ma-3",attrs:{outlined:"",disabled:t.in_progress,id:"run"},on:{click:function(e){return t._run()}}},[t._v("Run")]),t._v(" "),a("div",{staticClass:"d-flex"},[a("v-textarea",{attrs:{outlined:"",label:"Part 1"},model:{value:t.output1,callback:function(e){t.output1=e},expression:"output1"}}),t._v(" "),a("v-textarea",{attrs:{outlined:"",label:"Part 2"},model:{value:t.output2,callback:function(e){t.output2=e},expression:"output2"}})],1),t._v(" "),a("v-card",{directives:[{name:"show",rawName:"v-show",value:t.visual_supported,expression:"visual_supported"}]},[a("v-card-title",[t._v("Visualization")]),t._v(" "),a("v-card-text",[a("v-checkbox",{attrs:{label:"Enabled"},on:{click:function(e){return t._unvis()}},model:{value:t.visual_enabled,callback:function(e){t.visual_enabled=e},expression:"visual_enabled"}}),t._v(" "),a("v-slider",{attrs:{min:"0",max:"100",label:"Speed"},model:{value:t.visual_speed,callback:function(e){t.visual_speed=e},expression:"visual_speed"}}),t._v(" "),a("canvas",{attrs:{id:"canvas",oncontextmenu:"event.preventDefault()"}})],1)],1),t._v("\n\t\t\tSource code is "),a("a",{attrs:{href:"https://github.com/DarthGandalf/advent-of-code/tree/master/2020"}},[t._v("over here")]),t._v(".\n\t\t")],1)],1)],1)};r._withStripped=!0;var s=a(156),o={props:{days:Array,current_day:Number},data:()=>({menu_visible:null,loading:!0,input:"",output1:"",output2:"",visual_supported:!1,visual_enabled:!0,visual_speed:90,in_progress:!1,mdiEye:s.b,mdiArrowRightBold:s.a}),methods:{_run(){this.visual_supported&&this.visual_enabled&&this.$vuetify.goTo("#canvas")},_unvis(){this.visual_enabled||this.$vuetify.goTo(0)},getInput(){return this.input},setInput(t){this.input=t},setOutput1(t){this.output1=t},setOutput2(t){this.output2=t},supportVisual(){this.visual_supported=!0},visualEnabled(){return this.visual_enabled},visualSpeed(){return this.visual_speed},setInProgress(t){this.in_progress=t},setLoaded(){this.loading=!1}}},u=a(196),l=a(197),d=a.n(l),p=a(455),v=a(453),c=a(449),_=a(200),h=a(154),f=a(91),b=a(452),m=a(454),y=a(153),g=a(155),w=a(92),V=a(152),x=a(48),k=a(456),O=a(450),S=a(457),I=a(458),P=a(451),C=a(198),T=Object(u.a)(o,r,[],!1,null,null,null);d()(T,{VApp:p.a,VAppBar:v.a,VAppBarNavIcon:c.a,VBtn:_.a,VCard:h.a,VCardText:f.a,VCardTitle:f.b,VCheckbox:b.a,VContainer:m.a,VIcon:y.a,VList:g.a,VListItem:w.a,VListItemAction:V.a,VListItemContent:x.a,VListItemSubtitle:x.b,VListItemTitle:x.c,VMain:k.a,VNavigationDrawer:O.a,VSkeletonLoader:S.a,VSlider:I.a,VTextarea:P.a,VToolbarTitle:C.a}),T.options.__file="web/App.vue";var $=T.exports;const j=[[1,"Report Repair",0],[2,"Password Philosophy",0],[3,"Toboggan Trajectory",1],[4,"Passport Processing",0],[5,"Binary Boarding",0],[6,"Custom Customs",0],[7,"Handy Haversacks",0],[8,"Handheld Halting",0],[9,"Encoding Error",0],[10,"Adapter Array",0],[11,"Seating System",0],[12,"Rain Risk",1]];n.a.use(i.a),window.aocvue=new n.a({vuetify:new i.a({theme:{dark:!0,themes:{dark:{primary:"#ffff66"}}},icons:{iconfont:"mdiSvg"}}),el:"#main",render:t=>t($,{props:{days:j,current_day:window.AOCCurrentDay}}),methods:{getInput(){return this.$children[0].getInput()},setInput(t){this.$children[0].setInput(t)},setOutput1(t){this.$children[0].setOutput1(t)},setOutput2(t){this.$children[0].setOutput2(t)},supportVisual(){this.$children[0].supportVisual()},visualEnabled(){return this.$children[0].visualEnabled()},visualSpeed(){return this.$children[0].visualSpeed()},setInProgress(t){this.$children[0].setInProgress(t)},setLoaded(){this.$children[0].setLoaded()}}})}});
//# sourceMappingURL=main.js.map