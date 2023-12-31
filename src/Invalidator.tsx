import { useQueryClient } from "@tanstack/react-query";
import { ReactNode } from "react";
import { useEventHandler } from "./lib/events";

/**
 * Invalidates ALL queries whenever a "game_did_restart" message is recieved
 */
export function Invalidator(props: { children: ReactNode }) {
  const client = useQueryClient();

  useEventHandler("game_did_restart", () => {
    client.invalidateQueries();
  });

  useEventHandler("week_did_advance", () => {
    client.invalidateQueries();
  });

  return props.children;
}
