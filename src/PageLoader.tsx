import { useEventHandler } from "@/lib/events";
import { EmitMsg } from "@/lib/msg_bindings";
import { Loader2 } from "lucide-react";
import { ReactNode, useState } from "react";

const START_LOADING_ON_EVENTS = ["game_will_restart"] satisfies readonly EmitMsg[];

const STOP_LOADING_ON_EVENTS = ["game_did_restart"] satisfies readonly EmitMsg[];

export function PageLoader(props: { children: ReactNode }) {
  const [isLoading, setIsLoading] = useState(false);

  START_LOADING_ON_EVENTS.forEach((evt) => {
    useEventHandler(evt, () => setIsLoading(true));
  });

  STOP_LOADING_ON_EVENTS.forEach((evt) => {
    useEventHandler(evt, () => setIsLoading(false));
  });

  return isLoading ? (
    <div className="w-screen h-screen flex items-center justify-center gap-2">
      <Loader2 className="animate-spin" />
      <p>Loading...</p>
    </div>
  ) : (
    props.children
  );
}
