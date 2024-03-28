<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->
<script lang="ts">
	//import { MessageBox } from '$lib/utilis';
	import {CDelegate} from '$lib/delegate'
	import {globalVar} from '$lib/global'
	
	import { faker } from '@faker-js/faker';
	import { ListBox, ListBoxItem, Table, type PaginationSettings, type TableSource, tableMapperValues,  ProgressRadial, RangeSlider, SlideToggle, type ModalSettings, initializeStores, getModalStore, ProgressBar} from '@skeletonlabs/skeleton';
	import { Paginator } from '@skeletonlabs/skeleton';
    import { invoke } from '@tauri-apps/api';
    import { appWindow } from '@tauri-apps/api/window';
	import Icon from 'svelte-awesome';
	import   {userSecret}from 'svelte-awesome/icons';
    import { emit } from '@tauri-apps/api/event';
    import { getVersion } from '@tauri-apps/api/app';
	import { registerAll, unregister, unregisterAll } from '@tauri-apps/api/globalShortcut';

    let g_version:string="";
	let toggle_speed=true;
	let g_speed: number = 0; // %
	let max:number =100;
	let g_exe_name="";
	let speadtxt=0;
	


	async function init(){

		g_version = await CDelegate.GetVersion();
	}
	



	async function injection(e:Event){
		if(g_exe_name==""){
			return;
		}

		if($globalVar.exe_info[0].ismapped==false){
			$globalVar.exe_info[0].ismapped = await CDelegate.CreateSharedMem(g_exe_name,4);
		}

		await CDelegate.WriteSpeed(g_exe_name,$globalVar.exe_info[0].speed);

		let dllpath = await CDelegate.GetCurrentDir()+"\\gear.dll";
		if(await CDelegate.Inject(g_exe_name,dllpath)!=0){
			CDelegate.MessageBox("Good Job.","Speedhacking injected!");
		}

	}

	async function changeSpeed(speed:number){
		if($globalVar.exe_info[0].ismapped==true){
			if(!toggle_speed){
				return;
			}
			await CDelegate.WriteSpeed(g_exe_name,speed);
		}
	}
	


	function apply(e:Event){
		g_speed=speadtxt*10;
	}
	
	async function toggle(e:Event){
		toggle_speed =!toggle_speed;
		if(!toggle_speed){
			await CDelegate.WriteSpeed(g_exe_name,1.0);
		}else{
			await CDelegate.WriteSpeed(g_exe_name,$globalVar.exe_info[0].speed);
		}
		
		console.log(toggle_speed);
	}
	
$: {

		if(g_speed<=10.0){
			g_speed=10.0;
		}

		let cal_speed=parseFloat((g_speed/10.0).toFixed(2));
		changeSpeed(cal_speed);
		$globalVar.exe_info[0].speed=cal_speed;
		speadtxt=cal_speed;
	
}

	init();
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div data-tauri-drag-region class="container h-full mx-auto flex justify-center items-center"  >
	
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div data-tauri-drag-region class="space-y-6 text-center flex flex-col items-center " >
	
		<h2 data-tauri-drag-region class="h2 mt-[-20px] mb-[60px]" >{g_version}</h2>
		
		<div class="input-group grid-cols-[auto_1fr_auto] h-[32px] text-[12px]" >
			<div class="input-group-shim h-[30px] text-[12px]" >PROCESS</div>
			<input type="search" class="h-[30px] text-[12px]" placeholder="Input name of exe..." bind:value={g_exe_name}/>
			<button class="variant-filled h-[30px] text-[12px]" on:click={injection}><Icon data={userSecret}/> (INJECT)</button>
		</div>
		<br/>
		<ProgressRadial value={g_speed} class="h-[120px] w-[120px]">{(g_speed/10).toFixed(2)}</ProgressRadial>
		<!--<ProgressBar label="Progress Bar" value={value} max={max} />-->
		<div class="space-y-1 text-center flex flex-col items-center " >
		<div class="flex justify-between items-center">
			<input class="input h-[24px] text-[12px] w-[120px] mr-[20px]" type="text" bind:value={speadtxt} />
			<button type="button" class="btn variant-filled h-[24px] w-[50px] text-[12px]" on:click={apply}>APPLY</button>
		</div>
		<RangeSlider name="range-slider" bind:value={g_speed} max={max} step={0.001} class="w-[198px]" >
			<!--<div class="flex justify-between items-center">
				<div class="font-bold">[0 to 10] times:</div>
				<div class="text-xs">{(g_speed/10).toFixed(2)}</div>
				
			</div>-->
		</RangeSlider>
	</div>
	<br/>
		<SlideToggle name="slider-label" size="sm" class="text-[16px] font-bold" active="bg-primary-500" on:click={toggle} checked>SpeedHacking</SlideToggle><!--checked-->

	</div>
</div>

<style lang="postcss">

</style>
