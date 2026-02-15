<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import * as Card from "$lib/components/ui/card";
    import * as Select from "$lib/components/ui/select";
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Label } from "$lib/components/ui/label";
    import { Textarea } from "$lib/components/ui/textarea";
    import { Checkbox } from "$lib/components/ui/checkbox";
    import {
        Avatar,
        AvatarFallback,
        AvatarImage,
    } from "$lib/components/ui/avatar";
    import {
        Save,
        X,
        Upload,
        Plus,
        Trash2,
        User,
        Phone,
        Briefcase,
        FileText,
        House,
    } from "lucide-svelte";
    import { EmployeeStatus, type Employee } from "$lib/types/employee";

    interface Props {
        onCancel: () => void;
        onSave: (employee: Employee) => void;
        initialData?: Employee | null;
    }

    let { onCancel, onSave, initialData = null }: Props = $props();

    // Form state
    let formData = $state<Employee>({
        name: initialData?.name || "",
        fatherName: initialData?.fatherName || "",
        spouseName: initialData?.spouseName || "",
        currentPlace: initialData?.currentPlace || "",
        currentPost: initialData?.currentPost || "",
        currentAddress: initialData?.currentAddress || "",
        phoneNumbers: initialData?.phoneNumbers
            ? JSON.parse(initialData.phoneNumbers)
            : [""],
        permanentSameAsCurrent:
            initialData?.permanentSameAsCurrent === 1 ? true : false,
        permanentPlace: initialData?.permanentPlace || "",
        permanentPost: initialData?.permanentPost || "",
        permanentAddress: initialData?.permanentAddress || "",
        emergencyContactName: initialData?.emergencyContactName || "",
        emergencyContactRelation: initialData?.emergencyContactRelation || "",
        emergencyContactPhone: initialData?.emergencyContactPhone || "",
        policeStation: initialData?.policeStation || "",
        experience: initialData?.experience || "",
        jobPost: initialData?.jobPost || "",
        employmentStatus:
            initialData?.employmentStatus || EmployeeStatus.APPLIED,
        joiningDate: initialData?.joiningDate || "",
        exitDate: initialData?.exitDate || "",
        essid: initialData?.essid || "",
        photoPath: initialData?.photoPath || "",
        dateOfBirth: initialData?.dateOfBirth || "",
        uan: initialData?.uan || "",
        esiip: initialData?.esiip || "",
    });

    let errors = $state<Record<string, string>>({});
    let saving = $state(false);
    let photoPreview = $state(initialData?.photoPath || "");

    // Employment status options
    const employmentStatusOptions = [
        { value: "applied", label: "Applied" },
        { value: "current", label: "Current Employee" },
        { value: "past", label: "Past Employee" },
    ];

    // Add phone number field
    function addPhoneNumber() {
        formData.phoneNumbers = [...formData.phoneNumbers, ""];
    }

    // Remove phone number field
    function removePhoneNumber(index: number) {
        formData.phoneNumbers = formData.phoneNumbers.filter(
            (_, i) => i !== index,
        );
    }

    // Handle photo upload
    async function handlePhotoUpload() {
        try {
            const selected = await open({
                multiple: false,
                filters: [
                    {
                        name: "Image",
                        extensions: ["png", "jpg", "jpeg", "gif", "webp"],
                    },
                ],
            });

            if (selected) {
                formData.photoPath = selected as string;
                photoPreview = selected as string;
            }
        } catch (error) {
            console.error("Error selecting photo:", error);
        }
    }

    // Copy current address to permanent
    function copyCurrentToPermanent() {
        formData.permanentPlace = formData.currentPlace;
        formData.permanentPost = formData.currentPost;
        formData.permanentAddress = formData.currentAddress;
    }

    // Watch for permanent address checkbox
    $effect(() => {
        if (formData.permanentSameAsCurrent) {
            copyCurrentToPermanent();
        }
    });

    // Validation
    function validate(): boolean {
        errors = {};

        if (!formData.name.trim()) {
            errors.name = "Name is required";
        }

        if (!formData.dateOfBirth) {
            errors.dateOfBirth = "Date of birth is required";
        }

        // Validate phone numbers
        const validPhones = formData.phoneNumbers.filter(
            (p: String) => p.trim() !== "",
        );
        if (validPhones.length === 0) {
            errors.phoneNumbers = "At least one phone number is required";
        } else {
            // Validate phone number format (10 digits)
            const invalidPhones = validPhones.filter(
                (p) => !/^\d{10}$/.test(p.trim()),
            );
            if (invalidPhones.length > 0) {
                errors.phoneNumbers = "Phone numbers must be 10 digits";
            }
        }

        // Validate UAN if provided (12 digits)
        if (formData.uan && !/^\d{12}$/.test(formData.uan)) {
            errors.uan = "UAN must be 12 digits";
        }

        // Validate ESI IP if provided (10 digits)
        if (formData.esiip && !/^\d{10}$/.test(formData.esiip)) {
            errors.esiip = "ESI IP must be 10 digits";
        }

        // Validate joining date for current/past employees
        if (
            (formData.employmentStatus === "current" ||
                formData.employmentStatus === "past") &&
            !formData.joiningDate
        ) {
            errors.joiningDate =
                "Joining date is required for current/past employees";
        }

        // Validate exit date for past employees
        if (formData.employmentStatus === "past" && !formData.exitDate) {
            errors.exitDate = "Exit date is required for past employees";
        }

        return Object.keys(errors).length === 0;
    }

    // Handle form submission
    async function handleSubmit() {
        if (!validate()) {
            return;
        }

        saving = true;

        try {
            // Filter out empty phone numbers
            const validPhones = formData.phoneNumbers.filter(
                (p) => p.trim() !== "",
            );

            const employeeData = {
                ...formData,
                phoneNumbers: JSON.stringify(validPhones),
                permanentSameAsCurrent: formData.permanentSameAsCurrent ? 1 : 0,
            };

            let result;
            if (initialData?.id) {
                // Update existing employee
                result = await invoke<Employee>("update_employee", {
                    id: initialData.id,
                    employee: employeeData,
                });
            } else {
                // Create new employee
                result = await invoke<Employee>("create_employee", {
                    employee: employeeData,
                });
            }

            onSave(result);
        } catch (error) {
            console.error("Error saving employee:", error);
            errors.submit = error as string;
        } finally {
            saving = false;
        }
    }

    function getInitials(name: string): string {
        return name
            .split(" ")
            .map((n) => n[0])
            .join("")
            .toUpperCase()
            .slice(0, 2);
    }
</script>

<div class="max-w-6xl mx-auto p-6">
    <Card.Root>
        <Card.Header>
            <div class="flex items-center justify-between">
                <div>
                    <Card.Title class="text-2xl">
                        {initialData ? "Edit Employee" : "Add New Employee"}
                    </Card.Title>
                    <Card.Description>
                        {initialData
                            ? "Update employee information"
                            : "Fill in the details to add a new employee"}
                    </Card.Description>
                </div>
                <Button variant="ghost" size="icon" onclick={onCancel}>
                    <X class="h-4 w-4" />
                </Button>
            </div>
        </Card.Header>

        <Card.Content>
            <form
                onsubmit={(e) => {
                    e.preventDefault();
                    handleSubmit();
                }}
                class="space-y-8"
            >
                <!-- Photo Upload -->
                <div class="flex items-center gap-6">
                    <Avatar class="h-24 w-24">
                        {#if photoPreview}
                            <AvatarImage
                                src={photoPreview}
                                alt="Employee photo"
                            />
                        {/if}
                        <AvatarFallback class="text-2xl">
                            {#if formData.name}
                                {getInitials(formData.name)}
                            {:else}
                                <User class="h-8 w-8" />
                            {/if}
                        </AvatarFallback>
                    </Avatar>
                    <div>
                        <Button
                            type="button"
                            variant="outline"
                            onclick={handlePhotoUpload}
                        >
                            <Upload class="h-4 w-4 mr-2" />
                            Upload Photo
                        </Button>
                        <p class="text-sm text-gray-500 mt-2">
                            Recommended: Square image, max 2MB
                        </p>
                    </div>
                </div>

                <!-- Personal Information -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 mb-4">
                        <User class="h-5 w-5 text-gray-600" />
                        <h3 class="text-lg font-semibold">
                            Personal Information
                        </h3>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="space-y-2">
                            <Label for="name"
                                >Full Name <span class="text-red-500">*</span
                                ></Label
                            >
                            <Input
                                id="name"
                                bind:value={formData.name}
                                placeholder="Enter full name"
                                class={errors.name ? "border-red-500" : ""}
                            />
                            {#if errors.name}
                                <p class="text-sm text-red-500">
                                    {errors.name}
                                </p>
                            {/if}
                        </div>

                        <div class="space-y-2">
                            <Label for="dob"
                                >Date of Birth <span class="text-red-500"
                                    >*</span
                                ></Label
                            >
                            <Input
                                id="dob"
                                type="date"
                                bind:value={formData.dateOfBirth}
                                class={errors.dateOfBirth
                                    ? "border-red-500"
                                    : ""}
                            />
                            {#if errors.dateOfBirth}
                                <p class="text-sm text-red-500">
                                    {errors.dateOfBirth}
                                </p>
                            {/if}
                        </div>

                        <div class="space-y-2">
                            <Label for="fatherName">Father's Name</Label>
                            <Input
                                id="fatherName"
                                bind:value={formData.fatherName}
                                placeholder="Enter father's name"
                            />
                        </div>

                        <div class="space-y-2">
                            <Label for="spouseName">Spouse Name</Label>
                            <Input
                                id="spouseName"
                                bind:value={formData.spouseName}
                                placeholder="Enter spouse name"
                            />
                        </div>
                    </div>
                </div>

                <!-- Contact Information -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 mb-4">
                        <Phone class="h-5 w-5 text-gray-600" />
                        <h3 class="text-lg font-semibold">
                            Contact Information
                        </h3>
                    </div>

                    <div class="space-y-3">
                        <Label
                            >Phone Numbers <span class="text-red-500">*</span
                            ></Label
                        >
                        {#each formData.phoneNumbers as phone, index}
                            <div class="flex gap-2">
                                <Input
                                    bind:value={formData.phoneNumbers[index]}
                                    placeholder="10 digit phone number"
                                    maxlength={10}
                                    class="flex-1"
                                />
                                {#if formData.phoneNumbers.length > 1}
                                    <Button
                                        type="button"
                                        variant="outline"
                                        size="icon"
                                        onclick={() => removePhoneNumber(index)}
                                    >
                                        <Trash2 class="h-4 w-4" />
                                    </Button>
                                {/if}
                            </div>
                        {/each}
                        <Button
                            type="button"
                            variant="outline"
                            size="sm"
                            onclick={addPhoneNumber}
                        >
                            <Plus class="h-4 w-4 mr-2" />
                            Add Phone Number
                        </Button>
                        {#if errors.phoneNumbers}
                            <p class="text-sm text-red-500">
                                {errors.phoneNumbers}
                            </p>
                        {/if}
                    </div>
                </div>

                <!-- Current Address -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 mb-4">
                        <House class="h-5 w-5 text-gray-600" />
                        <h3 class="text-lg font-semibold">Current Address</h3>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="space-y-2">
                            <Label for="currentPlace">Place</Label>
                            <Input
                                id="currentPlace"
                                bind:value={formData.currentPlace}
                                placeholder="City/Village name"
                            />
                        </div>

                        <div class="space-y-2">
                            <Label for="currentPost">Post Office</Label>
                            <Input
                                id="currentPost"
                                bind:value={formData.currentPost}
                                placeholder="Post office name"
                            />
                        </div>

                        <div class="space-y-2 md:col-span-2">
                            <Label for="currentAddress">Full Address</Label>
                            <Textarea
                                id="currentAddress"
                                bind:value={formData.currentAddress}
                                placeholder="House number, street, landmark"
                                rows={3}
                            />
                        </div>
                    </div>
                </div>

                <!-- Permanent Address -->
                <div class="space-y-4">
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-2">
                            <House class="h-5 w-5 text-gray-600" />
                            <h3 class="text-lg font-semibold">
                                Permanent Address
                            </h3>
                        </div>
                        <div class="flex items-center space-x-2">
                            <Checkbox
                                id="sameAddress"
                                bind:checked={formData.permanentSameAsCurrent}
                            />
                            <Label
                                for="sameAddress"
                                class="font-normal cursor-pointer"
                            >
                                Same as current address
                            </Label>
                        </div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="space-y-2">
                            <Label for="permanentPlace">Place</Label>
                            <Input
                                id="permanentPlace"
                                bind:value={formData.permanentPlace}
                                placeholder="City/Village name"
                                disabled={formData.permanentSameAsCurrent}
                            />
                        </div>

                        <div class="space-y-2">
                            <Label for="permanentPost">Post Office</Label>
                            <Input
                                id="permanentPost"
                                bind:value={formData.permanentPost}
                                placeholder="Post office name"
                                disabled={formData.permanentSameAsCurrent}
                            />
                        </div>

                        <div class="space-y-2 md:col-span-2">
                            <Label for="permanentAddress">Full Address</Label>
                            <Textarea
                                id="permanentAddress"
                                bind:value={formData.permanentAddress}
                                placeholder="House number, street, landmark"
                                rows={3}
                                disabled={formData.permanentSameAsCurrent}
                            />
                        </div>
                    </div>
                </div>

                <!-- Emergency Contact -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 mb-4">
                        <Phone class="h-5 w-5 text-gray-600" />
                        <h3 class="text-lg font-semibold">Emergency Contact</h3>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="space-y-2">
                            <Label for="emergencyName">Contact Name</Label>
                            <Input
                                id="emergencyName"
                                bind:value={formData.emergencyContactName}
                                placeholder="Name of contact person"
                            />
                        </div>

                        <div class="space-y-2">
                            <Label for="emergencyRelation">Relation</Label>
                            <Input
                                id="emergencyRelation"
                                bind:value={formData.emergencyContactRelation}
                                placeholder="Father/Mother/Spouse/etc"
                            />
                        </div>

                        <div class="space-y-2">
                            <Label for="emergencyPhone">Phone Number</Label>
                            <Input
                                id="emergencyPhone"
                                bind:value={formData.emergencyContactPhone}
                                placeholder="10 digit phone number"
                                maxlength={10}
                            />
                        </div>
                    </div>
                </div>

                <!-- Employment Details -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 mb-4">
                        <Briefcase class="h-5 w-5 text-gray-600" />
                        <h3 class="text-lg font-semibold">
                            Employment Details
                        </h3>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="space-y-2">
                            <Label for="employmentStatus"
                                >Employment Status</Label
                            >
                            <Select.Root
                                selected={employmentStatusOptions.find(
                                    (o) =>
                                        o.value === formData.employmentStatus,
                                )}
                                onSelectedChange={(v) => {
                                    if (v)
                                        formData.employmentStatus = v.value as
                                            | "applied"
                                            | "current"
                                            | "past";
                                }}
                            >
                                <Select.Trigger>
                                    <Select.Value placeholder="Select status" />
                                </Select.Trigger>
                                <Select.Content>
                                    {#each employmentStatusOptions as option}
                                        <Select.Item value={option.value}
                                            >{option.label}</Select.Item
                                        >
                                    {/each}
                                </Select.Content>
                            </Select.Root>
                        </div>

                        <div class="space-y-2">
                            <Label for="jobPost">Job Position/Designation</Label
                            >
                            <Input
                                id="jobPost"
                                bind:value={formData.jobPost}
                                placeholder="e.g., Manager, Developer, Technician"
                            />
                        </div>

                        <div class="space-y-2">
                            <Label for="joiningDate">
                                Joining Date
                                {#if formData.employmentStatus === "current" || formData.employmentStatus === "past"}
                                    <span class="text-red-500">*</span>
                                {/if}
                            </Label>
                            <Input
                                id="joiningDate"
                                type="date"
                                bind:value={formData.joiningDate}
                                class={errors.joiningDate
                                    ? "border-red-500"
                                    : ""}
                            />
                            {#if errors.joiningDate}
                                <p class="text-sm text-red-500">
                                    {errors.joiningDate}
                                </p>
                            {/if}
                        </div>

                        <div class="space-y-2">
                            <Label for="exitDate">
                                Exit Date
                                {#if formData.employmentStatus === "past"}
                                    <span class="text-red-500">*</span>
                                {/if}
                            </Label>
                            <Input
                                id="exitDate"
                                type="date"
                                bind:value={formData.exitDate}
                                disabled={formData.employmentStatus !== "past"}
                                class={errors.exitDate ? "border-red-500" : ""}
                            />
                            {#if errors.exitDate}
                                <p class="text-sm text-red-500">
                                    {errors.exitDate}
                                </p>
                            {/if}
                        </div>

                        <div class="space-y-2 md:col-span-2">
                            <Label for="experience">Previous Experience</Label>
                            <Textarea
                                id="experience"
                                bind:value={formData.experience}
                                placeholder="List previous work experience, roles, companies"
                                rows={3}
                            />
                        </div>
                    </div>
                </div>

                <!-- Official Details -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 mb-4">
                        <FileText class="h-5 w-5 text-gray-600" />
                        <h3 class="text-lg font-semibold">Official Details</h3>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="space-y-2">
                            <Label for="essid">Employee/ESSID Number</Label>
                            <Input
                                id="essid"
                                bind:value={formData.essid}
                                placeholder="Employee identification number"
                            />
                        </div>

                        <div class="space-y-2">
                            <Label for="policeStation">Police Station</Label>
                            <Input
                                id="policeStation"
                                bind:value={formData.policeStation}
                                placeholder="Nearest police station"
                            />
                        </div>

                        <div class="space-y-2">
                            <Label for="uan">UAN Number (12 digits)</Label>
                            <Input
                                id="uan"
                                bind:value={formData.uan}
                                placeholder="Universal Account Number"
                                maxlength={12}
                                class={errors.uan ? "border-red-500" : ""}
                            />
                            {#if errors.uan}
                                <p class="text-sm text-red-500">{errors.uan}</p>
                            {/if}
                        </div>

                        <div class="space-y-2">
                            <Label for="esiip">ESI IP Number (10 digits)</Label>
                            <Input
                                id="esiip"
                                bind:value={formData.esiip}
                                placeholder="Employee State Insurance IP"
                                maxlength={10}
                                class={errors.esiip ? "border-red-500" : ""}
                            />
                            {#if errors.esiip}
                                <p class="text-sm text-red-500">
                                    {errors.esiip}
                                </p>
                            {/if}
                        </div>
                    </div>
                </div>

                <!-- Error Message -->
                {#if errors.submit}
                    <div class="bg-red-50 border border-red-200 rounded-lg p-4">
                        <p class="text-sm text-red-600">{errors.submit}</p>
                    </div>
                {/if}

                <!-- Action Buttons -->
                <div class="flex justify-end gap-3 pt-6 border-t">
                    <Button
                        type="button"
                        variant="outline"
                        onclick={onCancel}
                        disabled={saving}
                    >
                        Cancel
                    </Button>
                    <Button type="submit" disabled={saving}>
                        {#if saving}
                            <div
                                class="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"
                            ></div>
                            Saving...
                        {:else}
                            <Save class="h-4 w-4 mr-2" />
                            {initialData ? "Update Employee" : "Save Employee"}
                        {/if}
                    </Button>
                </div>
            </form>
        </Card.Content>
    </Card.Root>
</div>
