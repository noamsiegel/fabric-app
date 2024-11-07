<script lang="ts">
	import Check from "lucide-svelte/icons/check";
	import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
	import * as Command from "$lib/components/ui/command";
	import * as Popover from "$lib/components/ui/popover";
	import { Button } from "$lib/components/ui/button";
	import { cn } from "$lib/utils";
	import { tick } from "svelte";
	import {
		Text,
		Link,
		Youtube,
		MessageCircleQuestion,
	} from "lucide-svelte/icons";

	interface InputType {
		value: string;
		label: string;
		icon: any;
		flag: string;
	}

	const inputTypes: InputType[] = [
		{ value: "text", label: "Text", icon: Text, flag: "paste" },
		{ value: "url", label: "URL", icon: Link, flag: "-u" },
		{ value: "youtube", label: "YouTube", icon: Youtube, flag: "-y" },
		{
			value: "question",
			label: "Question",
			icon: MessageCircleQuestion,
			flag: "-q",
		},
	];

	let { onInputTypeSelected = (type: string) => {} } = $props<{
		onInputTypeSelected?: (type: string) => void;
	}>();

	// Use state for open/value/search
	let open = $state(false);
	let value = $state("");
	let search = $state("");
	let triggerRef = $state<HTMLButtonElement>(null!);

	// Derive selected value
	const selectedValue = $derived(
		inputTypes.find((t) => t.value === value)?.label ?? "Input type",
	);

	// Helper function to close popover and focus trigger
	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef?.focus();
		});
	}
</script>

<div class="flex flex-col gap-4">
	<Popover.Root bind:open>
		<Popover.Trigger bind:ref={triggerRef}>
			<Button
				variant="outline"
				role="combobox"
				aria-expanded={open}
				class="w-[200px] justify-between"
			>
				{#if value && inputTypes.find((t) => t.value === value)?.icon}
					{@const IconComponent = inputTypes.find(
						(t) => t.value === value,
					)?.icon}
					<IconComponent class="mr-2 h-4 w-4" />
				{/if}
				{selectedValue}
				<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
			</Button>
		</Popover.Trigger>
		<Popover.Content class="w-[200px] p-0">
			<Command.Root>
				<Command.Input
					placeholder="Search input type..."
					bind:value={search}
				/>
				{#key search}
					<Command.List>
						<Command.Empty>No input type found.</Command.Empty>
						<Command.Group>
							{#each inputTypes.filter((t) => t.label
									.toLowerCase()
									.includes(search.toLowerCase())) as type}
								<Command.Item
									value={type.value}
									onSelect={() => {
										value = type.value;
										onInputTypeSelected(type.value);
										closeAndFocusTrigger();
									}}
								>
									<Check
										class={cn(
											"mr-2 h-4 w-4",
											value !== type.value &&
												"text-transparent",
										)}
									/>
									{@const IconComponent = type.icon}
									<IconComponent class="mr-2 h-4 w-4" />
									{type.label}
								</Command.Item>
							{/each}
						</Command.Group>
					</Command.List>
				{/key}
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
</div>
