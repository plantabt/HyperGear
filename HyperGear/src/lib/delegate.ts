import type { Unit } from "@faker-js/faker";
import { invoke } from "@tauri-apps/api";

export class CDelegate{

    public static async ReadSpeed(exename:string):Promise<any>{
        return await invoke("read_shared_mem",{memname:exename});
    }
    public static async WriteSpeed(exename:string,speed:number){
        await invoke("write_shared_mem",{memname:exename,speed:speed});
    }
    public static async OpenSharedMem(exename:string):Promise<boolean>{
        return await invoke("open_shared_mem",{memname:exename});
    }
    public static async IsSharedMem(exename:any):Promise<boolean>{
        return await invoke("is_shared_mem",{memname:exename});
    }
    public static async CreateSharedMem(exename:string,memsize:number):Promise<boolean>{
        return await invoke("create_shared_mem",{memname:exename,memsize:memsize});
    }
    public static async GetVersion():Promise<string>{
        return await invoke("get_appname",{}) + " " + await invoke("get_version",{});
    }
    public static async Inject(exename:string,dllpath:string){
        return await invoke("inject",{processName:exename,dllpath:dllpath});
    }
    public static async InjectByPid(pid:number,dllpath:string){
        return await invoke("inject_by_pid",{pid:pid,dllpath:dllpath});
    }
    public static async GetCurrentDir():Promise<string>{
        return await invoke("get_current_path",{});
    }
    public static async MessageBox(title:String,info:String){
        return await invoke("message_box",{title:title,info:info,msgtype:""});
    }
    public static async GetProcessList():Promise<any>{
        let process_list =  await invoke("get_process_list",{});
        return process_list["process"];
    }
};
