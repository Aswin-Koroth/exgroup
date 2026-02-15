<script lang="ts">
    import "../app.css";
    import { cn } from "$lib/utils";
    import { Toaster } from "$lib/components/ui/sonner";
    import { isSidebarOpen } from "$lib/stores/navigation";
    import Sidebar from "$lib/components/sidebar/Sidebar.svelte";

    interface Props {
        children?: any;
    }

    let { children }: Props = $props();
</script>

<Toaster position="bottom-right" />

<div class="min-h-screen bg-gray-50">
    <Sidebar />

    <main
        class={cn("transition-all duration-300 ease-in-out", {
            "ml-64": $isSidebarOpen,
            "ml-16": !$isSidebarOpen,
        })}
    >
        {@render children?.()}
    </main>
</div>
