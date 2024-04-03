<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->
<script lang="ts">
	//import { MessageBox } from '$lib/utilis';
	import {CDelegate} from '$lib/delegate'
	import {globalVar} from '$lib/global'
	// Types
	import type { AutocompleteOption, PopupSettings } from '@skeletonlabs/skeleton';
	import { ProgressRadial, RangeSlider, SlideToggle,Autocomplete, popup} from '@skeletonlabs/skeleton';
	import Icon from 'svelte-awesome';
	import   {userSecret}from 'svelte-awesome/icons';


    let g_version:string="";
	let toggle_speed=true;
	let g_speed: number = 0; // %
	let max:number =100;
	let g_exe_name="";
	let speadtxt=0;

	let popupSettings: PopupSettings = {
		event: 'click',
		target: 'popupAutocomplete',
		placement: 'bottom',
	};
	let g_process_selected: string = '';
	
	type FlavorOption = AutocompleteOption<string, { healthy: boolean }>;
	let flavorOptions: FlavorOption[] = [];


	function onPopupDemoSelect(event: CustomEvent<FlavorOption>): void {
		g_process_selected = event.detail.label;
	}
	async function update_process_list(){
		let process_list = await CDelegate.GetProcessList();
		flavorOptions=null;
		flavorOptions = [];
		for(let i in process_list){
			flavorOptions.push({label: process_list[i]["name"] + " - [ " + process_list[i]["pid"] + " ]", value: process_list[i]["pid"], keywords: 'processlist', meta: { healthy: false }});
		}
	}
	function onInputProcname(e:Event){

		update_process_list();
	}

	function updatelist(e:Event){
		update_process_list();
	}
	async function init(){

		g_version = await CDelegate.GetVersion();
	}
	



	async function injection(e:Event){
		let split_pname = g_process_selected.split("-");
		if(split_pname.length<2){
			CDelegate.MessageBox("Inject error","Process name error,format is:  procname - [ pid ] ");
			return;
		}
		g_exe_name = split_pname[0].trim();
		let pid = parseInt(split_pname[1].match(/[0-9]{1,10}/)[0]);
		console.log(pid);
		if(g_exe_name==""){
			CDelegate.MessageBox("Inject error","Input a process first!");
			return;
		}

		if($globalVar.exe_info[0].ismapped==false){
			$globalVar.exe_info[0].ismapped = await CDelegate.CreateSharedMem(g_exe_name,4);
		}

		await CDelegate.WriteSpeed(g_exe_name,$globalVar.exe_info[0].speed);

		let dllpath = await CDelegate.GetCurrentDir()+"\\gear.dll";
		if(await CDelegate.InjectByPid(pid,dllpath)!=0){
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
<div  class="container h-full mx-auto flex justify-center items-center"  >
	
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div class="space-y-6 text-center flex flex-col items-center " >
	
		<h2 data-tauri-drag-region class="h2 h-10 -m-24 mb-2 w-[800px] pt-8 pb-14" >{g_version}</h2>
		<!--
		<input class="input autocomplete" type="search" name="autocomplete-search" bind:value={g_process_selected} placeholder="Search..." use:popup={popupSettings} />
		<div data-popup="popupAutocomplete">
			<Autocomplete bind:input={g_process_selected} options={flavorOptions} on:selection={onPopupDemoSelect}		/>
		</div>-->
		<div class="flex">
			<div class="flex items-center justify-center bg-gray-500  h-7 w-16 text-xs  rounded-l-3xl" >PROCESS</div>
			<div class="flex-1 text-token space-y-1 z-50 h-7 ">
				<input
					class="input text-xs rounded-none h-7 border border-gray-600"
					type="search"
					name="autocomplete-search"
					bind:value={g_process_selected}
					placeholder="Search Process..."
					use:popup={popupSettings}
					on:click={updatelist}
					on:input={onInputProcname}
					autocomplete="off"
				/>
				<div data-popup="popupAutocomplete" class="card max-w-xs max-h-32 p-2 overflow-y-auto text-xs rounded-md " tabindex="-1">
					<Autocomplete bind:input={g_process_selected} options={flavorOptions} on:selection={onPopupDemoSelect}/>
				</div>
			</div>
			<button class="flex items-center justify-center variant-filled h-7 w-20 text-xs rounded-r-3xl" on:click={injection}><Icon data={userSecret}/> (INJECT)</button>
		</div>
		
		<!--<input type="search" class="h-[30px] text-[12px]" placeholder="Input name of exe..." bind:value={g_exe_name}/>-->
		<!--<div class="input-group grid-cols-[auto_1fr_auto] h-8 text-xs" >
			<div class="input-group-shim h-[30px] text-xs" >PROCESS</div>
			
			<div class="text-token space-y-1 z-50">
				<input
					class="autocomplete text-xs"
					type="search"
					name="autocomplete-search"
					bind:value={g_process_selected}
					placeholder="Search..."
					use:popup={popupSettings}
				/>
				<div data-popup="popupAutocomplete" class="card max-w-xs max-h-32 p-2 overflow-y-auto text-xs" tabindex="-1">
					<Autocomplete bind:input={g_process_selected} options={flavorOptions} on:selection={onPopupDemoSelect} />
				</div>
			</div>
			<button class="variant-filled h-[30px] text-xs" on:click={injection}><Icon data={userSecret}/> (INJECT)</button>
			</div>
		</div>-->
		<br/>
		<ProgressRadial value={g_speed} class=" h-[110px] w-[110px]">{(g_speed/10).toFixed(2)}</ProgressRadial>
		<!--<ProgressBar label="Progress Bar" value={value} max={max} />-->
		<div class="space-y-1 text-center flex flex-col items-center " >
		<div class="flex justify-between items-center">
			<input class="input h-6 text-xs w-32 mr-5" type="text" bind:value={speadtxt} />
			<button type="button" class="btn variant-filled h-6 w-12 text-xs" on:click={apply}>APPLY</button>
		</div>
		<RangeSlider name="range-slider" bind:value={g_speed} max={max} step={0.001} class=" w-48" >
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
