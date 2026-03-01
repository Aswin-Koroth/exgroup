<script lang="ts">
    import {
        Trash2,
        EyeIcon,
        UserPlus,
        Download,
        SquarePen,
        RefreshCw,
        ChevronLeftIcon,
        ChevronRightIcon,
    } from "lucide-svelte";
    import type {
        Employee,
        FilterOptions,
        EmployeeListResponse,
    } from "$lib/types/employee";
    import { toast } from "svelte-sonner";
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "$lib/components/ui/button";
    import DeleteDialog from "$lib/components/dialogs/DeleteDialog.svelte";
    import EmployeeForm from "$lib/components/employee/EmployeeForm.svelte";
    import EmployeeTable from "$lib/components/employee/EmployeeTable.svelte";
    import EmployeeFilters from "$lib/components/employee/EmployeeFilters.svelte";
    import { save } from "@tauri-apps/plugin-dialog";

    const ITEMS_PER_PAGE = 10;

    let employeeListResponse = $state<EmployeeListResponse>({
        employees: [],
        totalCount: 0,
    });
    let filteredEmployees = $state<Employee[]>([]);
    let loading = $state(false);
    let showForm = $state(false);
    let editingEmployee = $state<Employee | null>(null);
    let filters = $state<FilterOptions>({});
    let deleteDialogOpen = $state(false);
    let employeeToDelete = $state<number | null>(null);

    // Pagination
    let currentPage = $state(1);
    let totalPages = $state(1);

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

    $effect(() => {
        loadEmployees();
    });

    function handleDeleteClick(id: number) {
        employeeToDelete = id;
        deleteDialogOpen = true;
    }

    async function confirmDelete() {
        if (!employeeToDelete) return;

        try {
            await invoke("delete_employee", {
                id: employeeToDelete,
            });
            toast.success("Employee deleted successfully");
            loadEmployees();
        } catch (error) {
            console.error("Error deleting employee:", error);
        } finally {
            deleteDialogOpen = false;
            employeeToDelete = null;
        }
    }
    async function loadEmployees() {
        loading = true;
        try {
            employeeListResponse = await invoke<EmployeeListResponse>(
                "get_all_employees",
                {
                    page: currentPage,
                    limit: ITEMS_PER_PAGE,
                    filter: filters,
                },
            );
            filteredEmployees = employeeListResponse.employees;
            totalPages = Math.ceil(
                employeeListResponse.totalCount / ITEMS_PER_PAGE,
            );
        } catch (error) {
            console.error("Error loading employees:", error);
        } finally {
            loading = false;
        }
    }

    function handleView(id: number) {
        const employee = filteredEmployees.find((e) => e.id === id);
        if (employee) {
            editingEmployee = employee;
            showForm = true;
        }
    }

    function handleEdit(id: number) {
        const employee = filteredEmployees.find((e) => e.id === id);
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
        loadEmployees();
    }

    function handleFormSave(_: Employee) {
        showForm = false;
        editingEmployee = null;
        loadEmployees();
    }

    async function handleExport() {
        const filePath = await save({
            title: "Export Employees",
            defaultPath: "employees.csv",
            filters: [{ name: "CSV", extensions: ["csv"] }],
        });

        if (!filePath) return;

        try {
            await invoke("export_employees_csv", { exportPath: filePath });
            alert(`Exported successfully to:\n${filePath}`);
        } catch (error) {
            alert(`Export failed: ${error}`);
        }
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
                {@render EmployeeLoading()}
            {:else if filteredEmployees.length === 0}
                {@render NoEmployeesFound()}
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

<DeleteDialog bind:deleteDialogOpen onConfirmDelete={confirmDelete} />

<!-- Snippets -->
{#snippet actions(id: number)}
    <div class="flex gap-2">
        <Button variant="outline" size="sm" onclick={() => handleView(id)}>
            <EyeIcon class="h-4 w-4" />
        </Button>
        <Button variant="outline" size="sm" onclick={() => handleEdit?.(id)}>
            <SquarePen class="h-4 w-4" />
        </Button>
        <Button
            variant="destructive"
            size="sm"
            onclick={() => handleDeleteClick?.(id)}
        >
            <Trash2 class="h-4 w-4" />
        </Button>
    </div>
{/snippet}

{#snippet pagination()}
    <div class="flex justify-between items-center p-4">
        <div class="text-sm text-gray-500">
            Showing {(currentPage - 1) * ITEMS_PER_PAGE + 1} - {Math.min(
                currentPage * ITEMS_PER_PAGE,
                employeeListResponse.totalCount,
            )} of {employeeListResponse.totalCount}
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

{#snippet EmployeeLoading()}
    <div class="p-13 text-center">
        <RefreshCw class="h-9 w-8 animate-spin mx-auto mb-4 text-gray-400" />
        <p class="text-gray-501">Loading employees...</p>
    </div>
{/snippet}

{#snippet NoEmployeesFound()}
    <div class="p-12 text-center">
        <p class="text-gray-500 text-lg">No employees found</p>
        <p class="text-gray-400 text-sm mt-2">
            Try adjusting your filters or add a new employee
        </p>
    </div>
{/snippet}
