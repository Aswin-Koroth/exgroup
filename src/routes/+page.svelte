<script lang="ts">
    import {
        EyeIcon,
        UserPlus,
        Download,
        SquarePen,
        RefreshCw,
        ChevronRightIcon,
        ChevronLeftIcon,
    } from "lucide-svelte";
    import { invoke } from "@tauri-apps/api/core";
    import EmployeeForm from "$lib/components/employee/EmployeeForm.svelte";
    import { Button } from "$lib/components/ui/button";
    import EmployeeFilters from "$lib/components/employee/EmployeeFilters.svelte";
    import EmployeeTable from "$lib/components/employee/EmployeeTable.svelte";
    import type { Employee, FilterOptions } from "$lib/types/employee";

    const itemsPerPage = 1;
    let employees = $state<Employee[]>([]);
    let filteredEmployees = $state<Employee[]>([]);
    let loading = $state(false);
    let showForm = $state(false);
    let editingEmployee = $state<Employee | null>(null);

    let currentPage = $state(1);
    let totalPages = $state(2);

    let nextPage = () => {
        if (currentPage < totalPages) {
            currentPage++;
        }
    };
    let previousPage = () => {
        if (currentPage > 1) {
            currentPage--;
        }
    };
    let filters = $state<FilterOptions>({});

    $effect(() => {
        loadEmployees();
    });

    async function loadEmployees() {
        loading = true;
        try {
            employees = await invoke<Employee[]>("get_all_employees", {
                page: currentPage,
                limit: itemsPerPage,
                filter: filters,
            });
            filteredEmployees = employees;
        } catch (error) {
            console.error("Error loading employees:", error);
        } finally {
            loading = false;
        }
    }

    function handleView(id: number) {
        const employee = employees.find((e) => e.id === id);
        if (employee) {
            editingEmployee = employee;
            showForm = true;
        }
    }

    function handleEdit(id: number) {
        const employee = employees.find((e) => e.id === id);
        if (employee) {
            editingEmployee = employee;
            showForm = true;
        }
    }

    function handleAddNew() {
        editingEmployee = null;
        showForm = true;
    }

    function handleFormCancel() {
        showForm = false;
        editingEmployee = null;
    }

    function handleFormSave(employee: Employee) {
        showForm = false;
        editingEmployee = null;
        loadEmployees();
    }

    function handleExport() {
        console.log("Export data");
    }
</script>

{#if showForm}
    <EmployeeForm
        onCancel={handleFormCancel}
        onSave={handleFormSave}
        initialData={editingEmployee}
    />
{:else}
    <div class="container mx-auto p-6 space-y-6">
        <!-- Header -->
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-3xl font-bold tracking-tight">
                    Employee Management
                </h1>
            </div>
            <div class="flex gap-2">
                <Button
                    variant="outline"
                    onclick={loadEmployees}
                    disabled={loading}
                >
                    <RefreshCw
                        class="h-4 w-4 mr-2 {loading ? 'animate-spin' : ''}"
                    />
                    Refresh
                </Button>
                <Button variant="outline" onclick={handleExport}>
                    <Download class="h-4 w-4 mr-2" />
                    Export
                </Button>
                <Button onclick={handleAddNew}>
                    <UserPlus class="h-4 w-4 mr-2" />
                    Add New
                </Button>
            </div>
        </div>

        <EmployeeFilters bind:filters />

        <div class="p-0">
            {#if loading}
                <div class="p-12 text-center">
                    <RefreshCw
                        class="h-8 w-8 animate-spin mx-auto mb-4 text-gray-400"
                    />
                    <p class="text-gray-500">Loading employees...</p>
                </div>
            {:else if filteredEmployees.length === 0}
                <div class="p-12 text-center">
                    <p class="text-gray-500 text-lg">No employees found</p>
                    <p class="text-gray-400 text-sm mt-2">
                        Try adjusting your filters or add a new employee
                    </p>
                </div>
            {:else}
                <div class="overflow-x-auto">
                    <EmployeeTable
                        employees={filteredEmployees}
                        {actions}
                        {pagination}
                    />
                </div>
            {/if}
        </div>
    </div>
{/if}

{#snippet actions(id: number)}
    <div class="flex gap-2">
        <Button variant="ghost" size="sm" onclick={() => handleView(id)}>
            <EyeIcon class="h-4 w-4" />
        </Button>
        <Button variant="ghost" size="sm" onclick={() => handleEdit?.(id)}>
            <SquarePen class="h-4 w-4" />
        </Button>
    </div>
{/snippet}

{#snippet pagination()}
    <div class="flex justify-between items-center p-4">
        <div class="text-sm text-gray-500">
            Showing {(currentPage - 1) * itemsPerPage + 1} - {Math.min(
                currentPage * itemsPerPage,
                filteredEmployees.length,
            )} of {filteredEmployees.length}
        </div>
        <div class="flex gap-2">
            <div class="flex items-center gap-2">
                <span class="text-sm text-gray-500">Go to page:</span>
                <input
                    type="number"
                    min="1"
                    max={totalPages}
                    value={currentPage}
                    oninput={(e) => {
                        const page = parseInt(e.currentTarget.value);
                        if (page >= 1 && page <= totalPages) {
                            currentPage = page;
                        }
                    }}
                    class="w-16 px-2 py-1 text-sm border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
            </div>

            <div class="flex gap-2">
                <Button
                    variant="ghost"
                    size="sm"
                    disabled={currentPage === 1}
                    onclick={previousPage}
                >
                    <ChevronLeftIcon class="h-4 w-4" />
                </Button>

                <div class="flex items-center gap-2 px-3">
                    <span class="text-sm font-medium">{currentPage}</span>
                    <span class="text-sm text-gray-400">/</span>
                    <span class="text-sm text-gray-500">{totalPages}</span>
                </div>

                <Button
                    variant="ghost"
                    size="sm"
                    disabled={currentPage >= totalPages}
                    onclick={nextPage}
                >
                    <ChevronRightIcon class="h-4 w-4" />
                </Button>
            </div>
        </div>
    </div>
{/snippet}
