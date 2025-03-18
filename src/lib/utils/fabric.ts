import { Command } from "@tauri-apps/plugin-shell";
import { invoke } from "@tauri-apps/api/core";
import { platform } from '@tauri-apps/plugin-os';

// base fabric settings
export async function runFabric(flag: string, message: string, model: string, pattern: string, context?: string) {
  try {
    console.log(`Running fabric with model: ${model}, pattern: ${pattern}`);
    
    // First try to use our Rust backend function that has better error handling
    try {
      console.log("Invoking run_fabric_command with:", { input: message, flag });
      const result = await invoke("run_fabric_command", {
        input: message,
        flag: flag,
      });
      return result as string;
    } catch (e) {
      console.warn("Rust backend call failed, falling back to direct command", e);
      
      // Check if we're on Windows for correct command execution
      const isWindows = (await platform()) === 'win32';
      const fabricCmd = isWindows ? "fabric.exe" : "fabric";
      
      // Fallback to direct command execution
      // Build command arguments array
      const args = [
        flag,
        message,
        "--pattern",
        pattern,
        "--model",
        model,
      ];

      if (context && context.trim()) {
        args.push("--context", context);
      }

      console.log(`Executing direct command: ${fabricCmd} ${args.join(' ')}`);
      
      // Execute command using 'fabric'
      const result = await Command.create(fabricCmd, args).execute();
      console.log("Command result:", result);
      return result.stdout;
    }
  } catch (error) {
    console.error("Error running fabric command:", error);
    return `Error: ${error instanceof Error ? error.message : String(error)}`;
  }
}

// JinaAI functions

export async function scrapeUrl(urlToScrape: string) {
  try {
    const isWindows = (await platform()) === 'win32';
    const fabricCmd = isWindows ? "fabric.exe" : "fabric";
    
    console.log("Scraping URL:", urlToScrape);
    const result = await Command.create(fabricCmd, [
      "-q",
      urlToScrape,
    ]).execute();
    console.log("Scrape command result:", result);
  } catch (error) {
    console.error("Error scraping URL:", error);
    if (error instanceof Error) {
      alert(`Error scraping URL: ${error.message}`);
    } else {
      alert(`An unexpected error occurred: ${String(error)}`);
    }
  }
}

export async function searchQuestion(questionToSearch: string) {
  try {
    await invoke("set_is_running", { value: true });
    
    const isWindows = (await platform()) === 'win32';
    const fabricCmd = isWindows ? "fabric.exe" : "fabric";
    
    console.log("Searching question:", questionToSearch);
    const result = await Command.create(fabricCmd, [
      "-q",
      questionToSearch,
    ]).execute();
    console.log("Search question command result:", result);
    alert(`Question searched successfully. Output: ${result.stdout}`);
  } catch (error) {
    console.error("Error searching question:", error);
    if (error instanceof Error) {
      alert(`Error searching question: ${error.message}`);
    } else {
      alert(`An unexpected error occurred: ${String(error)}`);
    }
  } finally {
    await invoke("set_is_running", { value: false });
  }
}

async function runFabricCommand(
  input: string,
  flag: "-u" | "-q"
): Promise<string> {
  try {
    await invoke("set_is_running", { value: true });
    const selectedPattern = await invoke("get_secret", { key: "DEFAULT_PATTERN" });
    if (!selectedPattern) {
      return "Please select a pattern first.";
    }

    console.log(
      `Scraping ${flag === "-u" ? "URL" : "question"} and running pattern:`,
      input,
      selectedPattern
    );

    // Try using our Rust backend first as it has better error handling
    try {
      const result = await invoke("run_fabric_command", {
        input,
        flag,
      });
      return result as string;
    } catch (e) {
      // If Rust backend fails, try direct command
      console.warn("Rust backend failed, trying direct command:", e);
      
      const isWindows = (await platform()) === 'win32';
      const fabricCmd = isWindows ? "fabric.exe" : "fabric";
      
      const result = await Command.create(fabricCmd, [
        flag,
        input,
        "|",
        fabricCmd,
        "--pattern",
        selectedPattern as string,
      ]).execute();

      console.log("Direct command result:", result);
      return result.stdout;
    }
  } catch (error) {
    console.error("Error:", error);
    return `Error: ${error instanceof Error ? error.message : String(error)}`;
  } finally {
    await invoke("set_is_running", { value: false });
  }
}

export async function scrapeUrlAndRunPattern(
  urlToScrape: string
): Promise<string> {
  return runFabricCommand(urlToScrape, "-u");
}

export async function scrapeQuestionAndRunPattern(
  questionToScrape: string
): Promise<string> {
  return runFabricCommand(questionToScrape, "-q");
}
