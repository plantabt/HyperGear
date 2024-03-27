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
	hljs.registerLanguage('xml', xml); // for HTML
	hljs.registerLanguage('css', css);
	hljs.registerLanguage('javascript', javascript);
	hljs.registerLanguage('typescript', typescript);
	storeHighlightJs.set(hljs);

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	import { onMount } from 'svelte';

	onMount(async () => {
		//await invoke("show",{visible:true});
		appWindow.show();
	});

	import   {close,windowMinimize,userSecret}from 'svelte-awesome/icons';
    import { emit, listen } from '@tauri-apps/api/event';

	
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
			if(event.payload["event"]=="call_js_function"){
				console.log(event.payload["data"]);
				console.log('Message from Rust:1');
				console.log('Message from Rust:1');
				modalConfirm(modalStore);
			}
			if(event.payload["event"]=="MessageBox"){
				let data = event.payload["data"];
				MessageBox(modalStore,data.title,data.info,data.type)
			}
		});
	}
	init_event();

	function myJavaScriptFunction(message: string) {
		console.log('Message from Rust:', message);
		// 进行其他操作
	}
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

