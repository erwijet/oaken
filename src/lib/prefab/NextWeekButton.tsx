import { useEventHandler } from "@/lib/events";
import { api } from "@/lib/rpc";
import { Button } from "@/lib/ui/button";
import { cn } from "@/lib/utils";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { ChevronRight, Loader2 } from "lucide-react";
import { useState } from "react";

export function NextWeekButton() {
  const [isLoading, setIsLoading] = useState(false);

  const { data: game } = useQuery({
    queryKey: ["getGameState"],
    queryFn: () => api.query(["getGameState"]),
  });

  useEventHandler("week_did_advance", () => setIsLoading(false));

  return (
    <Button
    disabled={isLoading}
      onClick={() => {
        setIsLoading(true);
        api
          .mutation(["advanceWeek"])
          .then(() => setIsLoading(false));
      }}
    >
      Next Week ({(game?.wk_no ?? 0) + 1})
      {isLoading ? (
        <Loader2 className={cn("animate-spin h-4 w-4 ml-2", !isLoading && "invisible")} />
      ) : (
        <ChevronRight className={"w-4 h-4 ml-2"} />
      )}
    </Button>
  );
}
