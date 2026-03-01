<script lang="ts">
    import {
        X,
        Search,
        MapPin,
        Funnel,
        CircleX,
        Calendar,
    } from "lucide-svelte";
    import * as Card from "$lib/components/ui/card";
    import { Input } from "$lib/components/ui/input";
    import * as Select from "$lib/components/ui/select";
    import { EmployeeStatus, type FilterOptions } from "$lib/types/employee";

    interface Props {
        filters: FilterOptions;
    }

    let { filters = $bindable() }: Props = $props();

    function handleReset() {
        filters = {};
    }

    function clearFilter(key: keyof FilterOptions) {
        filters[key] = undefined;
    }

    const employmentStatusOptions = [
        { value: "", label: "All Statuses" },
        { value: EmployeeStatus.APPLIED, label: "Applied" },
        { value: EmployeeStatus.CURRENT, label: "Current" },
        { value: EmployeeStatus.PAST, label: "Past" },
    ];

    const hasActiveFilters = $derived(
        filters.query ||
            filters.employmentStatus ||
            filters.post ||
            filters.joiningDateFrom ||
            filters.joiningDateTo ||
            filters.exitDateFrom ||
            filters.exitDateTo,
    );
</script>

<Card.Root class="border-0 shadow-none bg-muted/30 rounded-lg">
    <Card.Content class="p-3">
        <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-1.5 text-muted-foreground">
                <Funnel class="h-3.5 w-3.5" />
                <span class="text-xs font-medium uppercase tracking-wide"
                    >Filters</span
                >
            </div>
            {#if hasActiveFilters}
                <button
                    onclick={handleReset}
                    class="text-xs text-muted-foreground hover:text-destructive flex items-center gap-1"
                >
                    <CircleX class="h-3 w-3" />
                    Clear all
                </button>
            {/if}
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            <div class="relative">
                <Search
                    class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground"
                />
                <Input
                    type="text"
                    bind:value={filters.query}
                    placeholder="Search by name or ESSID..."
                    class="h-9 pl-8 pr-8 text-sm bg-background"
                />
                {#if filters.query}
                    <button
                        onclick={() => clearFilter("query")}
                        class="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                    >
                        <X class="h-3.5 w-3.5" />
                    </button>
                {/if}
            </div>

            <Select.Root
                type="single"
                name="employmentStatus"
                bind:value={filters.employmentStatus}
            >
                <Select.Trigger class="h-9 w-full text-sm bg-background">
                    <span class="truncate">
                        {employmentStatusOptions.find(
                            (o) => o.value === filters.employmentStatus,
                        )?.label || "All Statuses"}
                    </span>
                </Select.Trigger>
                <Select.Content>
                    {#each employmentStatusOptions as option}
                        <Select.Item value={option.value} class="text-sm">
                            {option.label}
                        </Select.Item>
                    {/each}
                </Select.Content>
            </Select.Root>

            <div class="relative">
                <MapPin
                    class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground"
                />
                <Input
                    type="text"
                    bind:value={filters.post}
                    placeholder="Filter by location..."
                    class="h-9 pl-8 pr-8 text-sm bg-background"
                />
                {#if filters.post}
                    <button
                        onclick={() => clearFilter("post")}
                        class="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                    >
                        <X class="h-3.5 w-3.5" />
                    </button>
                {/if}
            </div>

            <div>
                <div
                    class="flex items-center h-9 rounded-md border border-input bg-background px-2 focus-within:ring-1 focus-within:ring-ring"
                >
                    <Calendar
                        class="mr-1.5 h-3.5 w-3.5 text-muted-foreground shrink-0"
                    />
                    <span
                        class="text-[11px] font-medium text-muted-foreground mr-2 shrink-0"
                        >Joined</span
                    >
                    <input
                        type="text"
                        onfocus={(e) => (e.currentTarget.type = "date")}
                        onblur={(e) => (e.currentTarget.type = "text")}
                        bind:value={filters.joiningDateFrom}
                        placeholder="From"
                        class="w-full min-w-0 bg-transparent text-sm outline-none placeholder:text-muted-foreground/50"
                    />
                    <span class="text-muted-foreground/30 mx-1">—</span>
                    <input
                        type="text"
                        onfocus={(e) => (e.currentTarget.type = "date")}
                        onblur={(e) => (e.currentTarget.type = "text")}
                        bind:value={filters.joiningDateTo}
                        placeholder="To"
                        class="w-full min-w-0 bg-transparent text-sm outline-none placeholder:text-muted-foreground/50"
                    />
                </div>
            </div>

            <div>
                <div
                    class="flex items-center h-9 rounded-md border border-input bg-background px-2 focus-within:ring-1 focus-within:ring-ring"
                >
                    <Calendar
                        class="mr-1.5 h-3.5 w-3.5 text-muted-foreground shrink-0"
                    />
                    <span
                        class="text-[11px] font-medium text-muted-foreground mr-2 shrink-0"
                        >Exited</span
                    >
                    <input
                        type="text"
                        onfocus={(e) => (e.currentTarget.type = "date")}
                        onblur={(e) => (e.currentTarget.type = "text")}
                        bind:value={filters.exitDateFrom}
                        placeholder="From"
                        class="w-full min-w-0 bg-transparent text-sm outline-none placeholder:text-muted-foreground/50"
                    />
                    <span class="text-muted-foreground/30 mx-1">—</span>
                    <input
                        type="text"
                        onfocus={(e) => (e.currentTarget.type = "date")}
                        onblur={(e) => (e.currentTarget.type = "text")}
                        bind:value={filters.exitDateTo}
                        placeholder="To"
                        class="w-full min-w-0 bg-transparent text-sm outline-none placeholder:text-muted-foreground/50"
                    />
                </div>
            </div>
        </div>

        {#if hasActiveFilters}
            <div
                class="flex flex-wrap items-center gap-1.5 mt-3 pt-3 border-t border-border/40"
            >
                <span class="text-xs text-muted-foreground mr-1"
                    >Active filters:</span
                >

                {#if filters.query}
                    <span
                        class="inline-flex items-center gap-1 px-2 py-0.5 bg-secondary/50 rounded-md text-xs"
                    >
                        Search: "{filters.query}"
                        <button
                            onclick={() => clearFilter("query")}
                            class="hover:text-destructive"
                        >
                            <X class="h-3 w-3" />
                        </button>
                    </span>
                {/if}

                {#if filters.employmentStatus}
                    <span
                        class="inline-flex items-center gap-1 px-2 py-0.5 bg-secondary/50 rounded-md text-xs"
                    >
                        Status: {employmentStatusOptions.find(
                            (o) => o.value === filters.employmentStatus,
                        )?.label}
                        <button
                            onclick={() => clearFilter("employmentStatus")}
                            class="hover:text-destructive"
                        >
                            <X class="h-3 w-3" />
                        </button>
                    </span>
                {/if}

                {#if filters.post}
                    <span
                        class="inline-flex items-center gap-1 px-2 py-0.5 bg-secondary/50 rounded-md text-xs"
                    >
                        Location: {filters.post}
                        <button
                            onclick={() => clearFilter("post")}
                            class="hover:text-destructive"
                        >
                            <X class="h-3 w-3" />
                        </button>
                    </span>
                {/if}

                {#if filters.joiningDateFrom || filters.joiningDateTo}
                    <span
                        class="inline-flex items-center gap-1 px-2 py-0.5 bg-secondary/50 rounded-md text-xs"
                    >
                        Joined: {filters.joiningDateFrom || "Any"} — {filters.joiningDateTo ||
                            "Any"}
                        <button
                            onclick={() => {
                                clearFilter("joiningDateFrom");
                                clearFilter("joiningDateTo");
                            }}
                            class="hover:text-destructive"
                        >
                            <X class="h-3 w-3" />
                        </button>
                    </span>
                {/if}

                {#if filters.exitDateFrom || filters.exitDateTo}
                    <span
                        class="inline-flex items-center gap-1 px-2 py-0.5 bg-secondary/50 rounded-md text-xs"
                    >
                        Exited: {filters.exitDateFrom || "Any"} — {filters.exitDateTo ||
                            "Any"}
                        <button
                            onclick={() => {
                                clearFilter("exitDateFrom");
                                clearFilter("exitDateTo");
                            }}
                            class="hover:text-destructive"
                        >
                            <X class="h-3 w-3" />
                        </button>
                    </span>
                {/if}
            </div>
        {/if}
    </Card.Content>
</Card.Root>
