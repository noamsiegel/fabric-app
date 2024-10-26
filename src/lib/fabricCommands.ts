import { Command } from "@tauri-apps/plugin-shell";
import { invoke } from "@tauri-apps/api/core";

export async function runPattern() {
  try {
    const selectedPattern = await invoke("get_selected_pattern");
    if (!selectedPattern) {
      alert("Please select a pattern first.");
      return;
    }

    console.log("Running pattern:", selectedPattern);
    const result = await Command.create("fabric", [
      "--pattern",
      selectedPattern as string,
    ]).execute();

    console.log("Test command result:", result);
    alert(`Command executed successfully. Output: ${result.stdout}`);
  } catch (error) {
    console.error("Error running test command:", error);
    if (error instanceof Error) {
      alert(`Error running command: ${error.message}`);
    } else {
      alert(`An unexpected error occurred: ${String(error)}`);
    }
  }
}

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
  }
}

export async function scrapeUrlAndRunPattern(
  urlToScrape: string
): Promise<string> {
  try {
    const selectedPattern = await invoke("get_selected_pattern");
    if (!selectedPattern) {
      return "Please select a pattern first.";
    }

    console.log(
      "Scraping URL and running pattern:",
      urlToScrape,
      selectedPattern
    );

    const result = await Command.create("fabric", [
      "-u",
      urlToScrape,
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
  }
}

export async function scrapeQuestionAndRunPattern(
  questionToScrape: string
): Promise<string> {
  try {
    const selectedPattern = await invoke("get_selected_pattern");
    if (!selectedPattern) {
      return "Please select a pattern first.";
    }

    console.log(
      "Scraping question and running pattern:",
      questionToScrape,
      selectedPattern
    );

    const result = await Command.create("fabric", [
      "-q",
      questionToScrape,
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
  }
}
