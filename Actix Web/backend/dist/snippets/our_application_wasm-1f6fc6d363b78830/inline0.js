export function __cargo_web_snippet_035ceffc8e6602f54ba4ae8307df0330b7c9616e(Module) { console.log("Heelo");const evtSource=new EventSource("/here");evtSource.onmessage=(event)=>{console.log(event.data);} }