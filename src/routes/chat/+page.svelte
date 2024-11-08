<script lang="ts">
  // shadcn components
  import * as Card from "$lib/components/ui/card";
  import * as Drawer from "$lib/components/ui/drawer";
  import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
  import * as Select from "$lib/components/ui/select";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button/index.js";
  import { buttonVariants } from "$lib/components/ui/button";
  import {
    Settings,
    Text,
    Link,
    Youtube,
    MessageCircleQuestion,
  } from "lucide-svelte/icons";
  // lib components
  import ModelParameters from "$lib/components/cards/model-parameters.svelte";
  import PatternSearchBox from "$lib/components/search-box/patterns.svelte";
  import ContextSearchBox from "$lib/components/search-box/contexts.svelte";
  import ModelsSearchBox from "$lib/components/search-box/models.svelte";
  import InputTypeSearchBox from "$lib/components/search-box/input-type.svelte";

  // tauri
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { create, BaseDirectory } from "@tauri-apps/plugin-fs";

  // lib
  import { runFabric } from "$lib/utils/fabric";

  // Define the interface

  // Define message type
  interface Message {
    role: "user" | "assistant";
    content: string;
    flag?: string;
  }

  // State management
  let messages = $state<Message[]>([
    { role: "assistant", content: "Hello! How can I help you today?" },
  ]);
  let inputMessage = $state("");
  let showSettings = $state(false);
  let currentPattern = $state("");
  let currentContext = $state("");
  let currentModel = $state("");
  let currentFlag = $state("");


  async function handleCopy(content: string) {
    try {
      console.log("Copying to clipboard:", content);
      await writeText(content);
    } catch (error) {
      console.error("Failed to copy to clipboard:", error);
    }
  }

  async function handleDownload(content: string) {
    try {
      // Generate timestamp for unique filename
      const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
      const filename = `fabric-chat-${timestamp}.txt`;

      // Create and write to file in Downloads directory
      const file = await create(filename, { baseDir: BaseDirectory.Download });
      await file.write(new TextEncoder().encode(content));
      await file.close();
    } catch (error) {
      console.error("Failed to download message:", error);
    }
  }

  // Update message display to show input type
  function getMessageClass(message: any) {
    const baseClass =
      message.role === "user"
        ? "ml-auto bg-primary/10 max-w-[80%]"
        : "mr-auto bg-muted max-w-[80%]";
    return baseClass;
  }

  // Update handleSend to properly manage message state
  async function handleSend(): Promise<void> {
    console.log({
      flag: currentFlag,
      message: inputMessage,
      pattern: currentPattern,
      context: currentContext,
      model: currentModel,
    });

    if (!inputMessage.trim()) {
      return;
    }

    // Add user message
    messages = [
      ...messages,
      {
      role: "user",
      content: inputMessage,
      flag: currentFlag,
    },
  ];

  const result = await runFabric(
    currentFlag,
    inputMessage,
    currentModel,
    currentPattern,
    currentContext,
  );

  // Add assistant response
  messages = [
    ...messages,
    {
      role: "assistant",
      content: result,
    },
  ];

  // Clear input after sending
  inputMessage = "";
}
</script>

<div class="container mx-auto max-w-4xl p-4 space-y-4">
  <!-- Message Log -->
  <div class="space-y-4 h-[60vh] overflow-y-auto p-4 rounded-lg border">
    {#each messages as message}
      <ContextMenu.Root>
        <ContextMenu.Trigger>
          <Card.Root class={getMessageClass(message)}>
            <Card.Content class="p-3">
              {#if message.role === "user" && message.type}
                <div
                  class="flex items-center gap-2 mb-1 text-xs text-muted-foreground"
                >
                  {#if inputTypes.find((t) => t.value === message.type)?.icon}
                    {@const Icon = inputTypes.find(
                      (t) => t.value === message.type,
                    )?.icon}
                    <Icon class="size-3" />
                  {/if}
                  <span
                    >{inputTypes.find((t) => t.value === message.type)
                      ?.label}</span
                  >
                </div>
              {/if}
              <p class="text-sm">{message.content}</p>
            </Card.Content>
          </Card.Root>
        </ContextMenu.Trigger>
        <ContextMenu.Content>
          <ContextMenu.Item onclick={() => handleCopy(message.content)}>
            Copy
          </ContextMenu.Item>
          <ContextMenu.Item onclick={() => handleDownload(message.content)}>
            Download
          </ContextMenu.Item>
        </ContextMenu.Content>
      </ContextMenu.Root>
    {/each}
  </div>

  <!-- Input Area -->
  <div class="flex flex-col gap-2">
    <!-- First row: Input type, pattern search, and settings -->
    <div class="flex gap-2 w-full">
      <div class="flex-1">
        <PatternSearchBox
          onPatternSelected={(pattern:string) => (currentPattern = pattern)}
        />
      </div>
      <div class="flex-1">
        <ContextSearchBox
          onContextSelected={(context:string) => (currentContext = context)}
        />
      </div>
      <div class="flex-1">
        <ModelsSearchBox onModelSelected={(model:string) => (currentModel = model)} />
      </div>
      <!-- settings drawer  -->
      <Drawer.Root bind:open={showSettings}>
        <Drawer.Trigger
          class={buttonVariants({ variant: "outline", size: "icon" })}
        >
          <Settings class="size-4" />
          <span class="sr-only">Model settings</span>
        </Drawer.Trigger>
        <Drawer.Content>
          <div class="mx-auto w-full max-w-sm">
            <Drawer.Header>
              <Drawer.Title>Model Parameters</Drawer.Title>
              <Drawer.Description>
                Adjust the AI model's behavior parameters.
              </Drawer.Description>
            </Drawer.Header>
            <ModelParameters />
            <Drawer.Footer>
              <Drawer.Close class={buttonVariants({ variant: "outline" })}>
                Close
              </Drawer.Close>
            </Drawer.Footer>
          </div>
        </Drawer.Content>
      </Drawer.Root>
    </div>
    <!-- Second row: Input and send button -->

    <div class="flex gap-2">
      <!-- input type selector -->
      <!-- <InputType onInputTypeSelected={handleTypeChange} /> -->
      <InputTypeSearchBox onInputTypeSelected={(flag: string) => (currentFlag = flag)} />

      <!-- input -->
      <Input
        class="flex-1"
        placeholder="Type your message..."
        bind:value={inputMessage}
      />

      <!-- send button -->
      <Button onclick={handleSend}>Send</Button>
    </div>
  </div>
</div>
