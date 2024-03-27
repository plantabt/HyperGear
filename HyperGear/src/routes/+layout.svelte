<script lang="ts">
	import '../app.postcss';

	import { invoke } from "@tauri-apps/api/tauri";
	import { appWindow } from "@tauri-apps/api/window";
	
	import { AppShell, AppBar, type ModalSettings, getModalStore, Modal, initializeStores, } from '@skeletonlabs/skeleton';
	import { FileButton, CodeBlock } from '@skeletonlabs/skeleton';
	// Highlight JS
	import hljs from 'highlight.js/lib/core';
	import 'highlight.js/styles/github-dark.css';
	import { storeHighlightJs } from '@skeletonlabs/skeleton';
	import xml from 'highlight.js/lib/languages/xml'; // for HTML
	import css from 'highlight.js/lib/languages/css';
	import javascript from 'highlight.js/lib/languages/javascript';
	import typescript from 'highlight.js/lib/languages/typescript';
	import Icon from 'svelte-awesome';

	import   {close,windowMinimize,userSecret}from 'svelte-awesome/icons';
    import { emit, listen } from '@tauri-apps/api/event';
	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';


	import { onMount } from 'svelte';
    import { registerAll, unregister } from '@tauri-apps/api/globalShortcut';

	let bankeys=['Ctrl+Shift+J','Ctrl+Shift+I','Ctrl+U','Ctrl+J','Ctrl+P','Ctrl+F','Ctrl+G'];
	let bankeys_js=[123, 114,116,118];//123=f12,114=f3,116=f5,118=f7

	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	hljs.registerLanguage('xml', xml); // for HTML
	hljs.registerLanguage('css', css);
	hljs.registerLanguage('javascript', javascript);
	hljs.registerLanguage('typescript', typescript);
	storeHighlightJs.set(hljs);


	onMount(() => {
		reghotkeys();
		window.onkeydown = function (e) {
			var keyCode = e.keyCode || e.which || e.charCode;
			if (bankeys_js.indexOf(keyCode) > -1) {
				e.preventDefault();
			}
		}
		appWindow.show();
	});
	
	async function reghotkeys(){
		await registerAll(bankeys, (shortcut) => {
			//console.log(`Shortcut ${shortcut} triggered`);
		});
	}
	async function unreghotkeys(){
		for(let i in bankeys){
			//console.log(bankeys[i]);
			await unregister(bankeys[i]);
		}

	}
	async function setOnfocusEvent(){
		await appWindow.onFocusChanged(({ payload: focused }) => {
			if(focused){
				reghotkeys();
			}else{
				unreghotkeys();
			}
		});
	}
	async function init(){
		
		setOnfocusEvent();
	}
	init();


	
	initializeStores();
	const modalStore = getModalStore();

	function modalConfirm(mode_store:any): void {
    const modal: ModalSettings = {
        type: 'confirm',
        title: 'Exit HyperGear?',
        body: 'Are you sure you wish to terminate?',
        response: (r: boolean) => {
            if(r){
                appWindow.close();
            }
        }
    };
    modalStore.trigger(modal);
}

function MessageBox(mode_store:any,_title,_info,_type): void {
	const modal: ModalSettings = {
		type:_type,
		title: _title,
		body: _info,
		response: (r: boolean) => {
			if(r){
				//appWindow.close();
			}
		}
	};
	modalStore.trigger(modal);
}    
	/*
	'alert'
	 	'confirm'
		'component'
		 'prompt'
	 * @param message
	 */

	async function onClose(event: MouseEvent) :Promise<void>{
		//await invoke("exit");
		modalConfirm(modalStore);
		//appWindow.close();
	}
	async function onMinimize(event: MouseEvent) :Promise<void>{
		//await invoke("exit");
		
		appWindow.minimize();
	}
	let unlisten;
	async function init_event(){
		unlisten = await listen("front-event",(event)=>{
			if(event.payload["event"]=="MessageBox"){
				let data = event.payload["data"];
				MessageBox(modalStore,data.title,data.info,data.type)
			}
		});
	}
	init_event();



</script>

<!-- App Shell -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div style="position:absolute; left:95%;top:4px;" on:mousedown={onClose}><Icon data={close}  class="titlebar-button"/></div>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div style="position:absolute;left:90%;top:2px;" on:mousedown={onMinimize}><Icon data={windowMinimize} style="width:12px" class="titlebar-button"/></div>
<Modal class=""/>
<AppShell>
	<slot>
	</slot>
</AppShell>

