import { Command } from "@tauri-apps/plugin-shell";
import { invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";

// base fabric settings

export async function runFabric(flag: string, message: string, model: string, pattern: string, context?: string) {
  try {
    // Fix the order in the log string to match the actual command structure
    console.log("pattern", pattern);
    console.log("Running fabric command:", `fabric ${flag} ${message} | fabric --pattern ${pattern} --model ${model}${context && context.trim() ? ` --context ${context}` : ""}`);

    // Second command: fabric with pattern and model
    const result = await Command.create("fabric", [
      flag,
      message,
      "|",
      "fabric",
      "--pattern",
      pattern,
      "--model",
      model,
      ...(context && context.trim() ? ["--context", context] : []),
    ]).execute();

    return result.stdout;
  } catch (error) {
    console.error("Error running fabric command:", error);
    return `Error: ${error instanceof Error ? error.message : String(error)}`;
  }
}

// JinaAI functions

export async function scrapeUrl(urlToScrape: string) {
  try {
    console.log("Scraping URL:", urlToScrape);
    const result = await Command.create("fabric", [
      "-q",
      urlToScrape,
    ]).execute();
    console.log("Scrape command result:", result);
    // alert(`URL scraped successfully. Output: ${result.stdout}`);
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
    console.log("Searching question:", questionToSearch);
    const result = await Command.create("fabric", [
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
    // TODO: Configure current pattern to be in store
    // const selectedPattern = "summarize";
    if (!selectedPattern) {
      return "Please select a pattern first.";
    }

    console.log(
      `Scraping ${flag === "-u" ? "URL" : "question"} and running pattern:`,
      input,
      selectedPattern
    );

    const result = await Command.create("fabric", [
      flag,
      input,
      "|",
      "fabric",
      "--pattern",
      selectedPattern as string,
    ]).execute();

    console.log("Execution result:", result);
    return result.stdout;
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
