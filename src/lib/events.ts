import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

type OakenEvent = "config_did_load";

export function useEventHandler(evtId: OakenEvent, handler: () => void) {
  useEffect(() => {
    const unlisten = listen(evtId, () => {
      handler();
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);
}
