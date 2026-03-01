<script lang="ts">
    import { Users, Database, PanelLeftCloseIcon } from "lucide-svelte";
    import { page } from "$app/state";
    import { cn } from "tailwind-variants";
    import { goto } from "$app/navigation";
    import logo from "$lib/assets/icon.png";
    import { isSidebarOpen } from "$lib/stores/navigation";

    interface NavItem {
        id: string;
        label: string;
        icon: any;
        link: string;
    }

    const navItems: NavItem[] = [
        { id: "employees", label: "Employees", icon: Users, link: "/" },
        {
            id: "database",
            label: "Database",
            icon: Database,
            link: "/database",
        },
    ];

    let activeRoute = $derived(page.url.pathname);

    function toggleSidebar() {
        $isSidebarOpen = !$isSidebarOpen;
    }
</script>

<aside
    class={cn(
        "fixed select-none left-0 top-0 group/sidebar h-screen bg-linear-to-b from-gray-900 to-gray-800 text-white transition-all duration-300 ease-in-out shadow-2xl z-50",
        {
            "w-64": $isSidebarOpen,
            "w-16": !$isSidebarOpen,
        },
    )}
>
    {@render header()}

    <nav class="mt-6 px-2">
        <ul class="space-y-1">
            {#each navItems as item}
                {@render navItem(item, activeRoute === item.link)}
            {/each}
        </ul>
    </nav>
    {@render version()}
</aside>

{#snippet header()}
    <div
        class="h-16 flex items-center justify-between px-3 border-b border-gray-700 overflow-hidden"
    >
        <div class="flex items-center gap-2 min-w-0">
            <div
                class="bg-white p-1 rounded-md flex items-center shrink-0"
                style={$isSidebarOpen ? "display:flex" : "display:none"}
            >
                <img src={logo} alt="ExGroup Logo" class="size-7" />
            </div>
            <span
                class="font-bold text-lg whitespace-nowrap overflow-hidden transition-all duration-300 ease-in-out"
                style={$isSidebarOpen
                    ? "max-width: 200px; opacity: 1;"
                    : "max-width: 0px; opacity: 0;"}
            >
                ExGroup
            </span>
        </div>

        <button
            class="hover:bg-gray-700 hover:text-white p-2 rounded-md shrink-0 transition-transform duration-300"
            onclick={toggleSidebar}
        >
            <div
                class="transition-transform duration-300"
                style={$isSidebarOpen ? "rotate: 0deg" : "rotate: 180deg"}
            >
                <PanelLeftCloseIcon class="size-5" />
            </div>
        </button>
    </div>
{/snippet}

{#snippet navItem(item: NavItem, isActive: boolean)}
    <li>
        <button
            onclick={() => goto(item.link)}
            class={cn(
                "w-full flex items-center gap-3 p-3 rounded-md transition-all duration-200 group relative",
                {
                    "bg-blue-600 text-white shadow-lg": isActive,
                    "text-gray-300 hover:bg-gray-700 hover:text-white":
                        !isActive,
                },
            )}
        >
            <item.icon
                class={cn("h-5 w-5 shrink-0", {
                    "text-white": isActive,
                    "text-gray-400 group-hover:text-white": !isActive,
                })}
            />

            {#if $isSidebarOpen}
                <span class="text-sm font-medium truncate">{item.label}</span>
            {/if}

            {#if !$isSidebarOpen}
                <div
                    class="absolute left-full ml-2 px-3 py-2 bg-gray-900 text-white text-sm rounded-lg shadow-xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 whitespace-nowrap z-50"
                >
                    {item.label}
                    <div
                        class="absolute left-0 top-1/2 -translate-x-1 -translate-y-1/2 w-2 h-2 bg-gray-900 rotate-45"
                    ></div>
                </div>
            {/if}
        </button>
    </li>
{/snippet}

{#snippet version()}
    <div class="absolute bottom-4 left-0 right-0 px-3 select-none">
        {#if $isSidebarOpen}
            <p class="text-xs text-gray-500 text-center">v1.0.0</p>
        {:else}
            <p class="text-xs text-gray-500 text-center">v1</p>
        {/if}
    </div>
{/snippet}
