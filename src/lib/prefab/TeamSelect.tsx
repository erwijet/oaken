import { useQuery } from "@tanstack/react-query";
import { useEventHandler } from "../events";
import { api } from "../rpc";

import {
  ComponentIcon,
  MedalIcon,
  UserRoundCog, Users, UsersRoundIcon
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
import { HTMLAttributes, useMemo } from "react";
import { useNavigate } from "react-router";
import { arr } from '@tsly/arr';
import { maybe } from "@tsly/maybe";
import { League } from "@/bindings";

export const TeamSelect = (props: HTMLAttributes<HTMLDivElement>) => {
  const { data: teams, refetch } = useQuery({
    queryKey: ["getTeamInfos"],
    queryFn: () => api.query(["getTeamInfos"]),
  });

  const hierarchy = useMemo(() => {
    function byId<T extends { id: number }>(a: T, b: T) { return a.id == b.id }

    return maybe(teams)?.take(teams => {
      const leagues = arr(teams.map(team => team.league)).dedup(byId).take();
      const tiers = arr(teams.map(team => team.tier)).dedup(byId).take();

      return leagues.map(league => ({
        ...league,
        tiers: tiers.filter(tier => tier.leagueId == league.id).map(tier => ({
          ...tier,
          teams: teams.filter(team => team.tier.id == tier.id)
        }))
      }))
    }) ?? [];
  }, [teams]);

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
          {hierarchy.map(league =>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>
                <ComponentIcon className="mr-2 h-4 w-4" />
                <span>{league.name}</span>
                </DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  {league.tiers.map(tier =>
                    <DropdownMenuSub>
                      <DropdownMenuSubTrigger>
                        <MedalIcon className="mr-2 h-4 w-4" />
                        <span>{tier.name}</span>
                      </DropdownMenuSubTrigger>
                      <DropdownMenuSubContent>
                        {tier.teams.map(team =>
                          <DropdownMenuItem onClick={() => nav('/team/' + team.id)}>
                            <UserRoundCog className="mr-2 h-4 w-4" />
                            <span>{team.name}</span>
                          </DropdownMenuItem>
                        )}
                      </DropdownMenuSubContent>
                    </DropdownMenuSub>
                  )}
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>
          )}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
};
