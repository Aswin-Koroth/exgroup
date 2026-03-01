<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import * as Card from "$lib/components/ui/card";
    import { Button } from "$lib/components/ui/button";
    import { Database, Download } from "lucide-svelte";
    import { open } from "@tauri-apps/plugin-dialog";

    interface DbInfo {
        path: string;
        version: number;
        employeeCount: number;
    }

    let dbInfo = $state<DbInfo | null>(null);
    let loading = $state(false);
    let backupStatus = $state("");

    async function loadDbInfo() {
        loading = true;
        try {
            dbInfo = await invoke<DbInfo>("get_db_info");
        } catch (error) {
            console.error("Error loading DB info:", error);
        } finally {
            loading = false;
        }
    }

    async function createBackup() {
        const selectedDir = await open({
            directory: true,
            multiple: false,
            title: "Choose Backup Destination",
        });

        if (!selectedDir) return;

        backupStatus = "Creating backup...";
        try {
            const backupPath = await invoke<string>("create_database_backup", {
                backupDir: selectedDir,
            });
            backupStatus = `Backup created: ${backupPath}`;
        } catch (error) {
            backupStatus = `Error: ${error}`;
        }
    }

    $effect(() => {
        loadDbInfo();
    });
</script>

<div class="container mx-auto p-6">
    <Card.Root>
        <Card.Header>
            <div class="flex items-center gap-2">
                <Database class="h-5 w-5" />
                <Card.Title>Database Information</Card.Title>
            </div>
        </Card.Header>
        <Card.Content class="space-y-4">
            {#if loading}
                <p>Loading...</p>
            {:else if dbInfo}
                <div class="space-y-3">
                    <div class="flex items-center justify-between">
                        <span class="text-sm font-medium">Database Path:</span>
                        <span class="text-sm text-gray-600 font-mono"
                            >{dbInfo.path}</span
                        >
                    </div>
                    <div class="flex items-center justify-between">
                        <span class="text-sm font-medium">Schema Version:</span>
                        <span class="text-sm text-gray-600"
                            >v{dbInfo.version}</span
                        >
                    </div>
                    <div class="flex items-center justify-between">
                        <span class="text-sm font-medium">Total Employees:</span
                        >
                        <span class="text-sm text-gray-600"
                            >{dbInfo.employeeCount}</span
                        >
                    </div>
                </div>

                <div class="pt-4 border-t">
                    <Button onclick={createBackup} class="w-full">
                        <Download class="h-4 w-4 mr-2" />
                        Create Backup
                    </Button>
                    {#if backupStatus}
                        <p class="text-sm mt-2 text-center text-gray-600">
                            {backupStatus}
                        </p>
                    {/if}
                </div>
            {/if}
        </Card.Content>
    </Card.Root>
</div>
