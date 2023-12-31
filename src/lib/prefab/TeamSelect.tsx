import { useQuery } from "@tanstack/react-query";
import { useEventHandler } from "../events";
import { api } from "../rpc";

import {
    UserRoundCog, Users
} from "lucide-react";

import { Button } from "@/lib/ui/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuPortal,
    DropdownMenuSub,
    DropdownMenuSubContent,
    DropdownMenuSubTrigger,
    DropdownMenuTrigger
} from "@/lib/ui/dropdown-menu";
import { HTMLAttributes } from "react";
import { useNavigate } from "react-router";

export const TeamSelect = (props: HTMLAttributes<HTMLDivElement>) => {
  const { data: teams, refetch } = useQuery({
    queryKey: ["getTeams"],
    queryFn: () => api.query(["getTeams"]),
  });

  useEventHandler("game_did_restart", () => refetch());
  const nav = useNavigate();

  return (
    <div {...props}>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" className="w-fit">
            Teams
            <Users className="w-4 h-4 ml-2" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="ml-4 w-56">
          {teams?.map((team) => (
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>{team.name}</DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <DropdownMenuItem onClick={() => nav('/team/' + team.id)}>
                    <UserRoundCog className="mr-2 h-4 w-4" />
                    <span>View Team</span>
                  </DropdownMenuItem>
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};
