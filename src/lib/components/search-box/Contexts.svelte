<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import Check from "lucide-svelte/icons/check";
	import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
	import * as Command from "$lib/components/ui/command";
	import * as Popover from "$lib/components/ui/popover";
	import { Button } from "$lib/components/ui/button";
	import { cn } from "$lib/utils";
	import { tick } from "svelte";
	import { onMount } from "svelte";

	// Make onContextsListed optional with a default no-op function
	// let { onContextsListed = () => {} } = $props<{
	// 	onContextsListed?: (contexts: string) => void;
	// }>();

	let { onContextSelected = () => {} } = $props<{
		onContextSelected?: (context: string) => void;
	}>();

	interface Context {
		value: string;
		label: string;
	}

	let contexts = $state<Context[]>([]);
	let open = $state(false);
	let value = $state("");
	let search = $state("");
	let triggerRef = $state<HTMLButtonElement>(null!);
	let defaultContext = $state<string | null>(null);

	const selectedContext = $derived(
		contexts.find((c) => c.value === value)?.label ??
			defaultContext ??
			"Select a context...",
	);

	async function getContexts() {
		try {
			const result = await invoke("list_contexts");
			const contextArray = (result as string).split("\n").filter(Boolean);
			contexts = contextArray.map((context) => ({
				value: context,
				label: context,
			}));
			// onContextsListed(result as string);
		} catch (error) {
			console.error("Failed to list contexts:", error);
		}
	}

	async function getDefaultContext() {
		try {
			const defaultValue = await invoke("get_secret", {
				key: "CURRENT_CONTEXT",
			});
			defaultContext = defaultValue as string;
			if (defaultContext) {
				// value = defaultContext;
				onContextSelected(defaultContext);
			}
		} catch (error) {
			console.error("Failed to get default context:", error);
		}
	}

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef?.focus();
		});
	}

	onMount(() => {
		getContexts();
		getDefaultContext();
	});
</script>

<div class="flex flex-col gap-4">
	<Popover.Root bind:open>
		<Popover.Trigger bind:ref={triggerRef}>
			<Button
				variant="outline"
				role="combobox"
				aria-expanded={open}
				class="w-[300px] justify-between"
			>
				{selectedContext}
				<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
			</Button>
		</Popover.Trigger>
		<Popover.Content class="w-[300px] p-0">
			<Command.Root>
				<Command.Input
					placeholder="Search context..."
					bind:value={search}
				/>
				{#key search}
					<Command.List>
						<Command.Empty>No context found.</Command.Empty>
						<Command.Group>
							{#each contexts.filter((c) => c.label
									.toLowerCase()
									.includes(search.toLowerCase())) as context}
								<Command.Item
									value={context.value}
									onSelect={() => {
										// value = context.value;
										onContextSelected(context.value);
										closeAndFocusTrigger();
									}}
								>
									<Check
										class={cn(
											"mr-2 h-4 w-4",
											value !== context.value &&
												"text-transparent",
										)}
									/>
									{context.label}
								</Command.Item>
							{/each}
						</Command.Group>
					</Command.List>
				{/key}
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
</div>
