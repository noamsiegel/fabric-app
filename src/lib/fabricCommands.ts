import { Command } from "@tauri-apps/plugin-shell";
import { invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";

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

export async function clipboardContentsAndRunPattern(): Promise<string> {
  try {
    await invoke("set_is_running", { value: true });
    const selectedPattern = await invoke("get_selected_pattern");
    if (!selectedPattern) {
      return "Please select a pattern first.";
    }

    const currentPlatform = await platform();
    let clipboardCommand: string;

    switch (currentPlatform) {
      case "windows":
        clipboardCommand = 'powershell.exe -command "Get-Clipboard"';
        break;
      case "macos":
        clipboardCommand = "pbpaste";
        break;
      case "linux":
        clipboardCommand = "xclip -selection clipboard -o";
        break;
      default:
        throw new Error(`Unsupported platform: ${currentPlatform}`);
    }

    console.log("Running pattern with clipboard contents:", selectedPattern);

    const result = await Command.create("sh", [
      "-c",
      `${clipboardCommand} | fabric --pattern "${selectedPattern}"`,
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

async function runFabricCommand(
  input: string,
  flag: "-u" | "-q"
): Promise<string> {
  try {
    await invoke("set_is_running", { value: true });
    const selectedPattern = await invoke("get_selected_pattern");
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
