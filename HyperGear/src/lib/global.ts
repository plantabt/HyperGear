import { writable } from 'svelte/store';
export const globalVar = writable({
    exe_info:[{
        exepath:"",
        exename:"",
        speed:0,
        ismapped:false,
    }],
  });

