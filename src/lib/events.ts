import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { EmitMsg } from "@/lib/msg_bindings";

export function useEventHandler(evtId: EmitMsg, handler: () => void) {
  useEffect(() => {
    const unlisten = listen(evtId, () => {
      handler();
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);
}
