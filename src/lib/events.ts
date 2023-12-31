import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

type OakenEvent = "game_did_restart" | "week_did_advance";

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
